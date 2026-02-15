from __future__ import annotations

import argparse
import json
import os
import struct
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.runtime.slsk_runtime import frame, now_iso, s, send_server_sequence, sm_login, u32


CODE_SM_JOIN_ROOM = 14
CODE_SM_LEAVE_ROOM = 15
CODE_SM_GET_ROOM_TICKER = 113
CODE_SM_UPLOAD_SPEED = 121
CODE_SM_GET_USER_PRIVILEGES_STATUS = 122


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
        "notes": (
            "Stage 5A runtime capture for typed hardening: "
            "parent/distributed control observations + upload speed + user privileges status."
        ),
    }
    (run_dir / "manifest.raw.json").write_text(
        json.dumps(manifest, indent=2, ensure_ascii=True) + "\n",
        encoding="utf-8",
    )
    return run_dir


def main() -> int:
    parser = argparse.ArgumentParser(
        description=(
            "Generate Stage 5A runtime capture for typed hardening "
            "(parent/distributed control + upload speed + user privileges status)."
        )
    )
    parser.add_argument("--server", default="")
    parser.add_argument("--username", default="")
    parser.add_argument("--password", default="")
    parser.add_argument("--client-version", type=int, default=160)
    parser.add_argument("--minor-version", type=int, default=1)
    parser.add_argument("--target-user", default="")
    parser.add_argument("--room", default="nicotine")
    parser.add_argument("--upload-speed", type=int, default=128_000)
    parser.add_argument("--run-id", default="login-parent-distributed-control")
    args = parser.parse_args()

    load_dotenv_local(REPO_ROOT)
    server = require(args.server, "NSS_TEST_SERVER")
    username = require(args.username, "NSS_TEST_USERNAME")
    password = require(args.password, "NSS_TEST_PASSWORD")
    target_user = args.target_user or username

    login_frame = sm_login(username, password, args.client_version, args.minor_version)
    frames = send_server_sequence(
        server,
        [
            login_frame,
            frame(CODE_SM_UPLOAD_SPEED, u32(args.upload_speed)),
            frame(CODE_SM_GET_USER_PRIVILEGES_STATUS, s(target_user)),
            frame(CODE_SM_JOIN_ROOM, s(args.room)),
            frame(CODE_SM_GET_ROOM_TICKER, s(args.room)),
            frame(CODE_SM_LEAVE_ROOM, s(args.room)),
        ],
    )

    events = [
        {
            "ts": now_iso(),
            "event": "s5a.typed-runtime-hardening.capture",
            "server": server,
            "target_user": target_user,
            "room": args.room,
            "upload_speed": args.upload_speed,
            "code_counts": count_codes(frames),
        }
    ]
    run_dir = write_run(
        repo_root=REPO_ROOT,
        run_id=args.run_id,
        scenario="login-parent-distributed-control",
        source_type="runtime_socket_server_authenticated",
        server=server,
        frames=frames,
        frida_events=events,
    )

    print(
        json.dumps(
            {
                "run_id": args.run_id,
                "run_dir": str(run_dir),
                "frame_count": len(frames),
                "code_counts": count_codes(frames),
            },
            indent=2,
            ensure_ascii=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
