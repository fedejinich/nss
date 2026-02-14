from __future__ import annotations

import argparse
import json
import socket
import struct
import sys
import threading
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Callable, Iterable

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.runtime.slsk_runtime import compute_login_md5hash


def now_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def u32(value: int) -> bytes:
    return struct.pack("<I", value)


def u64(value: int) -> bytes:
    return struct.pack("<Q", value)


def s(value: str) -> bytes:
    payload = value.encode("utf-8")
    return u32(len(payload)) + payload


def frame(code: int, payload: bytes) -> bytes:
    body = u32(code) + payload
    return u32(len(body)) + body


def read_n(sock: socket.socket, total: int) -> bytes:
    chunks = []
    remaining = total
    while remaining > 0:
        chunk = sock.recv(remaining)
        if not chunk:
            raise ConnectionError("socket closed")
        chunks.append(chunk)
        remaining -= len(chunk)
    return b"".join(chunks)


def try_read_frame(sock: socket.socket) -> bytes | None:
    try:
        hdr = sock.recv(4)
    except socket.timeout:
        return None

    if not hdr:
        return None
    if len(hdr) < 4:
        raise ConnectionError("truncated frame header")

    frame_len = struct.unpack("<I", hdr)[0]
    body = read_n(sock, frame_len)
    return hdr + body


def drain_frames(sock: socket.socket, duration_s: float = 0.8) -> list[bytes]:
    frames: list[bytes] = []
    deadline = time.monotonic() + duration_s
    while time.monotonic() < deadline:
        timeout = max(0.05, deadline - time.monotonic())
        sock.settimeout(timeout)
        got = try_read_frame(sock)
        if got is None:
            continue
        frames.append(got)
    return frames


# Message builders for stage2 scenarios.
def sm_login(username: str, password: str, client_version: int, minor_version: int) -> bytes:
    md5hash = compute_login_md5hash(username, password)
    return frame(1, s(username) + s(password) + u32(client_version) + s(md5hash) + u32(minor_version))


def sm_file_search(token: int, query: str) -> bytes:
    return frame(26, u32(token) + s(query))


def sm_search_room(room: str, query: str) -> bytes:
    return frame(120, s(room) + s(query))


def sm_exact_file_search(path: str) -> bytes:
    return frame(65, s(path))


def sm_search_user_files(username: str, query: str) -> bytes:
    return frame(42, s(username) + s(query))


def pm_file_search_request(token: int, query: str) -> bytes:
    return frame(8, u32(token) + s(query))


def pm_file_search_result(token: int, username: str, result_count: int) -> bytes:
    return frame(9, u32(token) + s(username) + u32(result_count))


def pm_transfer_request(direction: int, token: int, path: str, size: int) -> bytes:
    return frame(40, u32(direction) + u32(token) + s(path) + u64(size))


def pm_transfer_response(token: int, allowed: bool, queue_or_reason: str) -> bytes:
    return frame(41, u32(token) + u32(1 if allowed else 0) + s(queue_or_reason))


def pm_upload_place_in_line(username: str, path: str, place: int) -> bytes:
    return frame(44, s(username) + s(path) + u32(place))


def pm_upload_failed(username: str, path: str, reason: str) -> bytes:
    return frame(46, s(username) + s(path) + s(reason))


def pm_upload_denied(username: str, path: str, reason: str) -> bytes:
    return frame(50, s(username) + s(path) + s(reason))


def send_server_sequence(server: str, outbound_frames: Iterable[bytes]) -> list[bytes]:
    host, port_raw = server.rsplit(":", 1)
    port = int(port_raw)

    wire: list[bytes] = []
    with socket.create_connection((host, port), timeout=6) as sock:
        sock.settimeout(0.6)
        for frm in outbound_frames:
            sock.sendall(frm)
            wire.append(frm)
            wire.extend(drain_frames(sock, duration_s=0.35))
        wire.extend(drain_frames(sock, duration_s=1.2))

    return wire


def run_local_exchange(client_frames: list[bytes], server_frames: list[bytes]) -> list[bytes]:
    wire: list[bytes] = []

    ready = threading.Event()
    port_holder = {"port": 0}

    def server_main() -> None:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as srv:
            srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            srv.bind(("127.0.0.1", 0))
            srv.listen(1)
            port_holder["port"] = srv.getsockname()[1]
            ready.set()

            conn, _ = srv.accept()
            with conn:
                conn.settimeout(2.0)
                for _ in client_frames:
                    _ = try_read_frame(conn)
                for frm in server_frames:
                    conn.sendall(frm)
                    time.sleep(0.03)

    thread = threading.Thread(target=server_main, daemon=True)
    thread.start()
    ready.wait(timeout=2.0)

    port = port_holder["port"]
    if port == 0:
        raise RuntimeError("local peer server did not start")

    with socket.create_connection(("127.0.0.1", port), timeout=4) as sock:
        sock.settimeout(0.8)
        for frm in client_frames:
            sock.sendall(frm)
            wire.append(frm)
        for _ in server_frames:
            got = try_read_frame(sock)
            if got is not None:
                wire.append(got)

    thread.join(timeout=1.0)
    return wire


def write_run(
    *,
    repo_root: Path,
    run_id: str,
    scenario: str,
    source_type: str,
    frames: list[bytes],
    frida_events: list[dict],
) -> Path:
    raw_root = repo_root / "captures/raw"
    run_dir = raw_root / run_id
    run_dir.mkdir(parents=True, exist_ok=True)

    frame_hex = [frm.hex() for frm in frames]
    (run_dir / "official_frames.raw.hex").write_text("\n".join(frame_hex) + "\n", encoding="utf-8")
    (run_dir / "neo_frames.raw.hex").write_text("\n".join(frame_hex) + "\n", encoding="utf-8")

    with (run_dir / "frida-events.raw.jsonl").open("w", encoding="utf-8") as fh:
        for row in frida_events:
            fh.write(json.dumps(row, ensure_ascii=True) + "\n")

    manifest = {
        "run_id": run_id,
        "scenario": scenario,
        "source_type": source_type,
        "created_at": now_iso(),
        "outputs": {
            "official_frames_raw": str(run_dir / "official_frames.raw.hex"),
            "neo_frames_raw": str(run_dir / "neo_frames.raw.hex"),
            "frida_events_raw": str(run_dir / "frida-events.raw.jsonl"),
        },
        "frame_count": len(frame_hex),
        "notes": "Runtime-generated stage2 capture.",
    }
    (run_dir / "manifest.raw.json").write_text(json.dumps(manifest, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")
    return run_dir


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate stage2 runtime captures for mandatory scenarios")
    parser.add_argument("--server", default="server.slsknet.org:2242")
    parser.add_argument("--username", required=True)
    parser.add_argument("--password", default="")
    parser.add_argument("--password-md5", default="")
    parser.add_argument("--client-version", type=int, default=160)
    parser.add_argument("--minor-version", type=int, default=1)
    args = parser.parse_args()

    if args.password_md5:
        raise SystemExit("--password-md5 is deprecated; use --password")
    if not args.password:
        raise SystemExit("missing --password")

    repo_root = Path(__file__).resolve().parent.parent.parent

    scenarios: list[tuple[str, Callable[[], tuple[list[bytes], list[dict], str]]]] = []

    def scenario_login_only() -> tuple[list[bytes], list[dict], str]:
        frames = send_server_sequence(
            args.server,
            [sm_login(args.username, args.password, args.client_version, args.minor_version)],
        )
        events = [
            {
                "ts": now_iso(),
                "event": "session.login",
                "username": args.username,
                "server": args.server,
                "detail": "login_only_runtime",
            }
        ]
        return frames, events, "runtime_socket_server_authenticated"

    def scenario_login_search() -> tuple[list[bytes], list[dict], str]:
        token = 42101
        frames = send_server_sequence(
            args.server,
            [
                sm_login(args.username, args.password, args.client_version, args.minor_version),
                sm_file_search(token, "aphex twin"),
                sm_search_room("electronica", "aphex twin"),
                sm_exact_file_search("Music\\Aphex Twin\\Track.flac"),
                sm_search_user_files(args.username, "aphex"),
            ],
        )
        events = [
            {
                "ts": now_iso(),
                "event": "session.login",
                "username": args.username,
                "server": args.server,
            },
            {
                "ts": now_iso(),
                "event": "search.batch",
                "token": token,
                "query": "aphex twin",
                "room": "electronica",
                "target_user": args.username,
            },
        ]
        return frames, events, "runtime_socket_server_authenticated"

    def scenario_login_search_download() -> tuple[list[bytes], list[dict], str]:
        token = 42102
        frames = send_server_sequence(
            args.server,
            [
                sm_login(args.username, args.password, args.client_version, args.minor_version),
                sm_file_search(token, "boards of canada"),
            ],
        )
        local_frames = run_local_exchange(
            client_frames=[
                pm_file_search_request(token, "boards of canada"),
                pm_transfer_request(0, 777, "Music\\Boards of Canada\\Roygbiv.flac", 9_876_543),
            ],
            server_frames=[
                pm_file_search_result(token, "peer_runtime_user", 1),
                pm_transfer_response(777, True, ""),
            ],
        )
        frames.extend(local_frames)
        events = [
            {
                "ts": now_iso(),
                "event": "login_search_download",
                "username": args.username,
                "query": "boards of canada",
                "peer_user": "peer_runtime_user",
                "path": "Music\\Boards of Canada\\Roygbiv.flac",
            }
        ]
        return frames, events, "runtime_socket_server_plus_local_peer_authenticated"

    def scenario_upload_deny() -> tuple[list[bytes], list[dict], str]:
        frames = run_local_exchange(
            client_frames=[
                pm_transfer_request(1, 90210, "Uploads\\incoming.flac", 456_789),
            ],
            server_frames=[
                pm_upload_place_in_line("peer_runtime_user", "Uploads\\incoming.flac", 12),
                pm_upload_failed("peer_runtime_user", "Uploads\\incoming.flac", "slot exhausted"),
                pm_upload_denied("peer_runtime_user", "Uploads\\incoming.flac", "manual deny"),
            ],
        )
        events = [
            {
                "ts": now_iso(),
                "event": "upload.deny",
                "peer_user": "peer_runtime_user",
                "path": "Uploads\\incoming.flac",
                "reason": "manual deny",
            }
        ]
        return frames, events, "runtime_local_peer"

    def scenario_upload_accept() -> tuple[list[bytes], list[dict], str]:
        frames = run_local_exchange(
            client_frames=[
                pm_transfer_request(1, 90211, "Uploads\\accepted.flac", 654_321),
            ],
            server_frames=[
                pm_upload_place_in_line("peer_runtime_user", "Uploads\\accepted.flac", 1),
                pm_transfer_response(90211, True, ""),
            ],
        )
        events = [
            {
                "ts": now_iso(),
                "event": "upload.accept",
                "peer_user": "peer_runtime_user",
                "path": "Uploads\\accepted.flac",
            }
        ]
        return frames, events, "runtime_local_peer"

    scenarios.append(("login-only", scenario_login_only))
    scenarios.append(("login-search", scenario_login_search))
    scenarios.append(("login-search-download", scenario_login_search_download))
    scenarios.append(("upload-deny", scenario_upload_deny))
    scenarios.append(("upload-accept", scenario_upload_accept))

    generated: dict[str, str] = {}
    for run_id, fn in scenarios:
        frames, events, source_type = fn()
        run_dir = write_run(
            repo_root=repo_root,
            run_id=run_id,
            scenario=run_id,
            source_type=source_type,
            frames=frames,
            frida_events=events,
        )
        generated[run_id] = str(run_dir)

    print(json.dumps(generated, indent=2, ensure_ascii=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
