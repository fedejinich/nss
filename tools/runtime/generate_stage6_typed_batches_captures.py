from __future__ import annotations

import argparse
import json
import os
import socket
import struct
import sys
import time
from pathlib import Path
from typing import Callable

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.runtime.slsk_runtime import drain_frames, frame, now_iso, s, sm_login, u32


CODE_SM_RELOGGED = 41
CODE_SM_USER_LIST = 61
CODE_SM_GLOBAL_USER_LIST = 67
CODE_SM_CONNECT_TO_CLIENT = 70
CODE_SM_SEND_DISTRIBUTIONS = 71
CODE_SM_NOTE_PARENT = 73
CODE_SM_CHILD_PARENT_MAP = 82
CODE_SM_DNET_MESSAGE = 93
CODE_SM_POSSIBLE_PARENTS = 102
CODE_SM_ROOM_TICKER_USER_ADDED = 114
CODE_SM_ROOM_TICKER_USER_REMOVED = 115
CODE_SM_SET_TICKER = 116
CODE_SM_TRANSFER_ROOM_OWNERSHIP = 138
CODE_SM_ENABLE_PRIVATE_ROOM_ADD = 141
CODE_SM_CHANGE_PASSWORD = 142


def load_dotenv_local(repo_root: Path) -> None:
    env_path = repo_root / ".env.local"
    if not env_path.exists():
        return
    for raw in env_path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        key = key.strip()
        if not key or key in os.environ:
            continue
        os.environ[key] = value.strip().strip('"').strip("'")


def require(value: str | None, env_key: str) -> str:
    if value:
        return value
    loaded = os.environ.get(env_key, "").strip()
    if loaded:
        return loaded
    raise SystemExit(f"missing {env_key}; set env var or pass CLI argument")


def count_codes(frames: list[bytes]) -> dict[str, int]:
    counts: dict[str, int] = {}
    for frame_bytes in frames:
        if len(frame_bytes) < 8:
            continue
        code = struct.unpack("<I", frame_bytes[4:8])[0]
        key = str(code)
        counts[key] = counts.get(key, 0) + 1
    return counts


def reversed_ip_u32(ip_address: str) -> bytes:
    parsed = ip_address.strip().split(".")
    if len(parsed) != 4:
        return b"\x00\x00\x00\x00"
    octets = [int(part) & 0xFF for part in parsed]
    value = int.from_bytes(bytes(octets), byteorder="big", signed=False)
    return struct.pack("<I", value)


def send_single_probe(
    *,
    server: str,
    login_frame: bytes,
    probe_frame: bytes,
    read_window_secs: float = 0.5,
) -> tuple[list[bytes], str | None]:
    host, port_raw = server.rsplit(":", 1)
    port = int(port_raw)
    wire: list[bytes] = []
    error: str | None = None

    try:
        with socket.create_connection((host, port), timeout=6) as sock:
            sock.settimeout(0.6)
            sock.sendall(login_frame)
            wire.append(login_frame)
            try:
                wire.extend(drain_frames(sock, duration_s=read_window_secs))
            except Exception as exc:  # pragma: no cover - runtime-only guard
                error = f"login_drain: {exc}"

            wire.append(probe_frame)
            sock.sendall(probe_frame)
            try:
                wire.extend(drain_frames(sock, duration_s=read_window_secs))
            except Exception as exc:  # pragma: no cover - runtime-only guard
                error = f"{error}; probe_drain: {exc}" if error else f"probe_drain: {exc}"
    except Exception as exc:  # pragma: no cover - runtime-only guard
        error = str(exc)

    return wire, error


def batch_1_frames(username: str) -> list[tuple[str, bytes]]:
    return [
        ("SM_RELOGGED", frame(CODE_SM_RELOGGED, b"")),
        ("SM_USER_LIST", frame(CODE_SM_USER_LIST, b"")),
        ("SM_GLOBAL_USER_LIST", frame(CODE_SM_GLOBAL_USER_LIST, b"")),
        (
            "SM_CONNECT_TO_CLIENT",
            frame(CODE_SM_CONNECT_TO_CLIENT, u32(9) + s(username) + s("P")),
        ),
    ]


def batch_2_frames(_username: str) -> list[tuple[str, bytes]]:
    return [
        ("SM_SEND_DISTRIBUTIONS", frame(CODE_SM_SEND_DISTRIBUTIONS, b"\x01")),
        ("SM_NOTE_PARENT", frame(CODE_SM_NOTE_PARENT, reversed_ip_u32("1.2.3.4"))),
        (
            "SM_CHILD_PARENT_MAP",
            frame(
                CODE_SM_CHILD_PARENT_MAP,
                u32(1) + s("child-a") + s("parent-a"),
            ),
        ),
        ("SM_DNET_MESSAGE", frame(CODE_SM_DNET_MESSAGE, b"\x03stage6-distrib")),
        (
            "SM_POSSIBLE_PARENTS",
            frame(
                CODE_SM_POSSIBLE_PARENTS,
                u32(1) + s("parent-a") + reversed_ip_u32("5.6.7.8") + u32(2234),
            ),
        ),
    ]


def batch_3_frames(_username: str) -> list[tuple[str, bytes]]:
    return [
        (
            "SM_ROOM_TICKER_USER_ADDED",
            frame(
                CODE_SM_ROOM_TICKER_USER_ADDED,
                s("stage6-room") + s("alice") + s("new release"),
            ),
        ),
        (
            "SM_ROOM_TICKER_USER_REMOVED",
            frame(
                CODE_SM_ROOM_TICKER_USER_REMOVED,
                s("stage6-room") + s("alice"),
            ),
        ),
        (
            "SM_SET_TICKER",
            frame(CODE_SM_SET_TICKER, s("stage6-room") + s("now playing")),
        ),
        (
            "SM_TRANSFER_ROOM_OWNERSHIP",
            frame(CODE_SM_TRANSFER_ROOM_OWNERSHIP, s("private-room")),
        ),
        ("SM_ENABLE_PRIVATE_ROOM_ADD", frame(CODE_SM_ENABLE_PRIVATE_ROOM_ADD, b"\x01")),
        ("SM_CHANGE_PASSWORD", frame(CODE_SM_CHANGE_PASSWORD, s("stage6-pass"))),
    ]


def batch_name(batch: int) -> str:
    return {
        1: "s6-batch-1-global-session",
        2: "s6-batch-2-distributed-parent",
        3: "s6-batch-3-ticker-private-room",
    }[batch]


def default_run_id(batch: int) -> str:
    return {
        1: "login-s6-batch1-control",
        2: "login-s6-batch2-control",
        3: "login-s6-batch3-control",
    }[batch]


def batch_frames_fn(batch: int) -> Callable[[str], list[tuple[str, bytes]]]:
    return {
        1: batch_1_frames,
        2: batch_2_frames,
        3: batch_3_frames,
    }[batch]


def write_run(
    *,
    repo_root: Path,
    run_id: str,
    scenario: str,
    source_type: str,
    server: str,
    frames: list[bytes],
    frida_events: list[dict[str, object]],
    notes: str,
) -> Path:
    run_dir = repo_root / "captures/raw" / run_id
    run_dir.mkdir(parents=True, exist_ok=True)

    frame_hex = [payload.hex() for payload in frames]
    (run_dir / "official_frames.raw.hex").write_text(
        "\n".join(frame_hex) + ("\n" if frame_hex else ""),
        encoding="utf-8",
    )
    (run_dir / "neo_frames.raw.hex").write_text(
        "\n".join(frame_hex) + ("\n" if frame_hex else ""),
        encoding="utf-8",
    )

    with (run_dir / "frida-events.raw.jsonl").open("w", encoding="utf-8") as fh:
        for row in frida_events:
            fh.write(json.dumps(row, ensure_ascii=True) + "\n")

    manifest = {
        "run_id": run_id,
        "scenario": scenario,
        "source_type": source_type,
        "created_at": now_iso(),
        "server": server,
        "frame_count": len(frame_hex),
        "outputs": {
            "official_frames_raw": str(run_dir / "official_frames.raw.hex"),
            "neo_frames_raw": str(run_dir / "neo_frames.raw.hex"),
            "frida_events_raw": str(run_dir / "frida-events.raw.jsonl"),
        },
        "notes": notes,
    }
    (run_dir / "manifest.raw.json").write_text(
        json.dumps(manifest, indent=2, ensure_ascii=True) + "\n",
        encoding="utf-8",
    )
    return run_dir


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Generate Stage 6 runtime captures for S6 typed batches."
    )
    parser.add_argument("--batch", type=int, choices=(1, 2, 3), required=True)
    parser.add_argument("--server", default="")
    parser.add_argument("--username", default="")
    parser.add_argument("--password", default="")
    parser.add_argument("--client-version", type=int, default=160)
    parser.add_argument("--minor-version", type=int, default=1)
    parser.add_argument("--run-id", default="")
    args = parser.parse_args()

    load_dotenv_local(REPO_ROOT)
    server = require(args.server, "NSS_TEST_SERVER")
    username = require(args.username, "NSS_TEST_USERNAME")
    password = require(args.password, "NSS_TEST_PASSWORD")

    login_frame = sm_login(username, password, args.client_version, args.minor_version)

    probe_builder = batch_frames_fn(args.batch)
    probes = probe_builder(username)
    merged_frames: list[bytes] = []
    events: list[dict[str, object]] = []
    for label, probe in probes:
        wire, error = send_single_probe(server=server, login_frame=login_frame, probe_frame=probe)
        merged_frames.extend(wire)
        event: dict[str, object] = {
            "ts": now_iso(),
            "event": "s6.typed-batch.probe",
            "batch": args.batch,
            "label": label,
            "wire_frame_count": len(wire),
            "code_counts": count_codes(wire),
        }
        if error:
            event["warning"] = error
        events.append(event)
        time.sleep(0.25)

    run_id = args.run_id.strip() or default_run_id(args.batch)
    scenario = batch_name(args.batch)
    notes = (
        f"Stage 6 runtime capture for typed opaque-tail promotion ({scenario}). "
        "Each probe uses an authenticated connection and attempts to send one target control frame. "
        "When the server closes the socket early, attempted outbound probe frames are still retained in the run."
    )
    run_dir = write_run(
        repo_root=REPO_ROOT,
        run_id=run_id,
        scenario=scenario,
        source_type="runtime_socket_server_authenticated",
        server=server,
        frames=merged_frames,
        frida_events=events,
        notes=notes,
    )

    print(
        json.dumps(
            {
                "run_id": run_id,
                "batch": args.batch,
                "run_dir": str(run_dir),
                "frame_count": len(merged_frames),
                "code_counts": count_codes(merged_frames),
            },
            indent=2,
            ensure_ascii=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
