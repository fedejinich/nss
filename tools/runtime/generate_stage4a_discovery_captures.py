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

from tools.runtime.slsk_runtime import frame, now_iso, s, sm_login, try_read_frame


CODE_SM_GET_SIMILAR_TERMS = 50
CODE_SM_GET_RECOMMENDATIONS = 54
CODE_SM_GET_MY_RECOMMENDATIONS = 55
CODE_SM_GET_GLOBAL_RECOMMENDATIONS = 56
CODE_SM_GET_USER_RECOMMENDATIONS = 57


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
        "notes": "Runtime-authenticated recommendations/discovery capture generated for S4A.",
    }
    (run_dir / "manifest.raw.json").write_text(
        json.dumps(manifest, indent=2, ensure_ascii=True) + "\n",
        encoding="utf-8",
    )
    return run_dir


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Generate authenticated Stage 4A recommendations/discovery runtime captures"
    )
    parser.add_argument("--server", default="")
    parser.add_argument("--username", default="")
    parser.add_argument("--password", default="")
    parser.add_argument("--client-version", type=int, default=160)
    parser.add_argument("--minor-version", type=int, default=1)
    parser.add_argument("--similar-term", default="electronic")
    args = parser.parse_args()

    load_dotenv_local(REPO_ROOT)
    server = require(args.server, "NSS_TEST_SERVER")
    username = require(args.username, "NSS_TEST_USERNAME")
    password = require(args.password, "NSS_TEST_PASSWORD")

    login_frame = sm_login(username, password, args.client_version, args.minor_version)

    def scenario_login_recommendations() -> tuple[list[bytes], list[dict[str, object]]]:
        frames = send_server_sequence(
            server,
            [
                login_frame,
                frame(CODE_SM_GET_RECOMMENDATIONS, b""),
                frame(CODE_SM_GET_MY_RECOMMENDATIONS, b""),
                frame(CODE_SM_GET_GLOBAL_RECOMMENDATIONS, b""),
            ],
            wait_after_each_s=1.0,
            final_wait_s=2.2,
        )
        events = [
            {
                "ts": now_iso(),
                "event": "discover.recommendations",
                "server": server,
                "code_counts": count_codes(frames),
            }
        ]
        return frames, events

    def scenario_login_user_recommendations() -> tuple[list[bytes], list[dict[str, object]]]:
        frames = send_server_sequence(
            server,
            [
                login_frame,
                frame(CODE_SM_GET_USER_RECOMMENDATIONS, s(username)),
            ],
            wait_after_each_s=1.0,
            final_wait_s=2.0,
        )
        events = [
            {
                "ts": now_iso(),
                "event": "discover.user_recommendations",
                "server": server,
                "target_user": username,
                "code_counts": count_codes(frames),
            }
        ]
        return frames, events

    def scenario_login_similar_terms() -> tuple[list[bytes], list[dict[str, object]]]:
        frames = send_server_sequence(
            server,
            [
                login_frame,
                frame(CODE_SM_GET_SIMILAR_TERMS, s(args.similar_term)),
            ],
            wait_after_each_s=1.0,
            final_wait_s=1.5,
        )
        events = [
            {
                "ts": now_iso(),
                "event": "discover.similar_terms",
                "server": server,
                "term": args.similar_term,
                "code_counts": count_codes(frames),
            }
        ]
        return frames, events

    scenarios: list[tuple[str, str, Callable[[], tuple[list[bytes], list[dict[str, object]]]]]] = [
        ("login-recommendations", "login-recommendations", scenario_login_recommendations),
        (
            "login-user-recommendations",
            "login-user-recommendations",
            scenario_login_user_recommendations,
        ),
        ("login-similar-terms", "login-similar-terms", scenario_login_similar_terms),
    ]

    generated: dict[str, object] = {}
    for run_id, scenario, callback in scenarios:
        frames, events = callback()
        run_dir = write_run(
            repo_root=REPO_ROOT,
            run_id=run_id,
            scenario=scenario,
            source_type="runtime_socket_server_authenticated",
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
