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

from tools.runtime.slsk_runtime import frame, now_iso, s, sm_login, try_read_frame, u32


CODE_SM_LOGIN = 1
CODE_SM_GET_PEER_ADDRESS = 3
CODE_SM_GET_USER_STATUS = 7
CODE_SM_CONNECT_TO_PEER = 18
CODE_SM_MESSAGE_USER = 22
CODE_SM_MESSAGE_ACKED = 23
CODE_SM_GET_USER_STATS = 36
CODE_SM_PEER_MESSAGE = 68
CODE_SM_MESSAGE_USERS = 149
CODE_SM_PEER_MESSAGE_ALT = 292


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


def read_for(sock: socket.socket, duration_s: float) -> list[bytes]:
    frames: list[bytes] = []
    deadline = time.monotonic() + duration_s
    while time.monotonic() < deadline:
        timeout = max(0.05, deadline - time.monotonic())
        sock.settimeout(timeout)
        try:
            got = try_read_frame(sock)
        except Exception:
            continue
        if got is None:
            continue
        frames.append(got)
    return frames


def send_server_sequence(
    server: str,
    outbound_frames: list[bytes],
    *,
    wait_after_each_s: float,
    final_wait_s: float,
) -> list[bytes]:
    host, port_raw = server.rsplit(":", 1)
    port = int(port_raw)

    wire: list[bytes] = []
    with socket.create_connection((host, port), timeout=6) as sock:
        sock.settimeout(0.8)
        for outbound in outbound_frames:
            sock.sendall(outbound)
            wire.append(outbound)
            wire.extend(read_for(sock, wait_after_each_s))
        wire.extend(read_for(sock, final_wait_s))
    return wire


def count_codes(frames: list[bytes]) -> dict[str, int]:
    counts: dict[str, int] = {}
    for frame_bytes in frames:
        if len(frame_bytes) < 8:
            continue
        code = struct.unpack("<I", frame_bytes[4:8])[0]
        key = str(code)
        counts[key] = counts.get(key, 0) + 1
    return counts


def write_run(
    *,
    repo_root: Path,
    run_id: str,
    scenario: str,
    source_type: str,
    server: str,
    frames: list[bytes],
    frida_events: list[dict[str, object]],
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
        "notes": "Stage 4E runtime capture generated for private messaging and user-state domain.",
    }
    (run_dir / "manifest.raw.json").write_text(
        json.dumps(manifest, indent=2, ensure_ascii=True) + "\n",
        encoding="utf-8",
    )
    return run_dir


def encode_message_users_payload(targets: list[str], message: str) -> bytes:
    payload = u32(len(targets))
    for target in targets:
        payload += s(target)
    payload += s(message)
    return payload


def encode_connect_to_peer_request(token: int, username: str, connection_type: str) -> bytes:
    return u32(token) + s(username) + s(connection_type)


def encode_peer_message_request(username: str, token: int, code: int, message: str) -> bytes:
    return s(username) + u32(token) + u32(code) + s(message)


def encode_peer_message_response(
    username: str,
    code: int,
    token: int,
    ip_address: str,
    port: int,
    message: str,
) -> bytes:
    ip_packed = struct.unpack("<I", socket.inet_aton(ip_address))[0]
    return s(username) + u32(code) + u32(token) + u32(ip_packed) + u32(port) + s(message)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Generate Stage 4E runtime captures (private messaging + user-state)"
    )
    parser.add_argument("--server", default="")
    parser.add_argument("--username", default="")
    parser.add_argument("--password", default="")
    parser.add_argument("--client-version", type=int, default=160)
    parser.add_argument("--minor-version", type=int, default=1)
    parser.add_argument("--target-user", default="")
    parser.add_argument("--message", default="neo s4e runtime probe")
    parser.add_argument("--token", type=int, default=77123)
    parser.add_argument("--connection-type", default="P")
    args = parser.parse_args()

    load_dotenv_local(REPO_ROOT)
    server = require(args.server, "NSS_TEST_SERVER")
    username = require(args.username, "NSS_TEST_USERNAME")
    password = require(args.password, "NSS_TEST_PASSWORD")
    target_user = args.target_user or username

    login_frame = sm_login(username, password, args.client_version, args.minor_version)

    def scenario_login_private_message() -> tuple[list[bytes], list[dict[str, object]]]:
        frames = send_server_sequence(
            server,
            [
                login_frame,
                frame(CODE_SM_MESSAGE_USER, s(target_user) + s(args.message)),
                frame(CODE_SM_MESSAGE_ACKED, u32(1)),
            ],
            wait_after_each_s=0.9,
            final_wait_s=1.6,
        )
        events = [
            {
                "ts": now_iso(),
                "event": "private.message.runtime",
                "server": server,
                "target_user": target_user,
                "message_len": len(args.message),
                "code_counts": count_codes(frames),
            }
        ]
        return frames, events

    def scenario_login_user_state() -> tuple[list[bytes], list[dict[str, object]]]:
        frames = send_server_sequence(
            server,
            [
                login_frame,
                frame(CODE_SM_GET_USER_STATUS, s(target_user)),
                frame(CODE_SM_GET_USER_STATS, s(target_user)),
            ],
            wait_after_each_s=0.9,
            final_wait_s=1.7,
        )
        events = [
            {
                "ts": now_iso(),
                "event": "user.state.runtime",
                "server": server,
                "target_user": target_user,
                "code_counts": count_codes(frames),
            }
        ]
        return frames, events

    def scenario_login_peer_address_connect() -> tuple[list[bytes], list[dict[str, object]]]:
        frames = send_server_sequence(
            server,
            [
                login_frame,
                frame(CODE_SM_GET_PEER_ADDRESS, s(target_user)),
                frame(
                    CODE_SM_CONNECT_TO_PEER,
                    encode_connect_to_peer_request(args.token, target_user, args.connection_type),
                ),
            ],
            wait_after_each_s=0.9,
            final_wait_s=1.9,
        )
        events = [
            {
                "ts": now_iso(),
                "event": "peer.address.connect.runtime",
                "server": server,
                "target_user": target_user,
                "token": args.token,
                "connection_type": args.connection_type,
                "code_counts": count_codes(frames),
            }
        ]
        return frames, events

    def scenario_login_message_users() -> tuple[list[bytes], list[dict[str, object]]]:
        frames = send_server_sequence(
            server,
            [
                login_frame,
                frame(
                    CODE_SM_MESSAGE_USERS,
                    encode_message_users_payload([target_user], args.message),
                ),
            ],
            wait_after_each_s=0.9,
            final_wait_s=1.4,
        )
        events = [
            {
                "ts": now_iso(),
                "event": "message.users.runtime",
                "server": server,
                "targets": [target_user],
                "message_len": len(args.message),
                "code_counts": count_codes(frames),
            }
        ]
        return frames, events

    def scenario_login_peer_message() -> tuple[list[bytes], list[dict[str, object]]]:
        frames = [
            frame(
                CODE_SM_PEER_MESSAGE,
                encode_peer_message_request(target_user, args.token, 40, "legacy-request"),
            ),
            frame(
                CODE_SM_PEER_MESSAGE,
                encode_peer_message_response(
                    target_user,
                    40,
                    args.token,
                    "127.0.0.1",
                    2242,
                    "legacy-response",
                ),
            ),
            frame(
                CODE_SM_PEER_MESSAGE_ALT,
                encode_peer_message_response(
                    target_user,
                    40,
                    args.token,
                    "127.0.0.1",
                    2242,
                    "legacy-response-alt",
                ),
            ),
        ]
        events = [
            {
                "ts": now_iso(),
                "event": "peer.message.local",
                "mode": "deterministic_local",
                "target_user": target_user,
                "token": args.token,
                "code_counts": count_codes(frames),
            }
        ]
        return frames, events

    scenarios: list[
        tuple[str, str, str, Callable[[], tuple[list[bytes], list[dict[str, object]]]]]
    ] = [
        (
            "login-private-message",
            "login-private-message",
            "runtime_socket_server_authenticated",
            scenario_login_private_message,
        ),
        (
            "login-user-state",
            "login-user-state",
            "runtime_socket_server_authenticated",
            scenario_login_user_state,
        ),
        (
            "login-peer-address-connect",
            "login-peer-address-connect",
            "runtime_socket_server_authenticated",
            scenario_login_peer_address_connect,
        ),
        (
            "login-message-users",
            "login-message-users",
            "runtime_socket_server_authenticated",
            scenario_login_message_users,
        ),
        (
            "login-peer-message",
            "login-peer-message",
            "runtime_peer_local_deterministic",
            scenario_login_peer_message,
        ),
    ]

    generated: dict[str, object] = {}
    for run_id, scenario, source_type, callback in scenarios:
        frames, events = callback()
        run_dir = write_run(
            repo_root=REPO_ROOT,
            run_id=run_id,
            scenario=scenario,
            source_type=source_type,
            server=server,
            frames=frames,
            frida_events=events,
        )
        generated[run_id] = {
            "run_dir": str(run_dir),
            "frame_count": len(frames),
            "code_counts": count_codes(frames),
        }

    print(json.dumps(generated, indent=2, ensure_ascii=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
