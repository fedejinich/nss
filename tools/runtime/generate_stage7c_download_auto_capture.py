#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.runtime.slsk_runtime import frame, now_iso, s, sm_login, u32, u64


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


def login_success_payload(ip_address: str = "127.0.0.1") -> bytes:
    octets = [int(p) & 0xFF for p in ip_address.split(".")]
    ip_raw = int.from_bytes(bytes(octets), byteorder="little", signed=False)
    return b"\x01" + s("welcome") + u32(ip_raw) + s("0" * 32) + b"\x00"


def search_summary_payload(username: str, token: int, path: str, size: int) -> bytes:
    return (
        s(username)
        + u32(token)
        + u32(1)
        + s(path)
        + u64(size)
        + s("flac")
        + u32(0)
        + u32(1)
        + u32(5000)
        + u32(0)
    )


def transfer_request_payload(token: int, path: str, size: int) -> bytes:
    return u32(0) + u32(token) + s(path) + u64(size)


def transfer_response_payload(token: int, allowed: bool = True, reason: str = "") -> bytes:
    return u32(token) + u32(1 if allowed else 0) + s(reason)


def write_run(*, run_id: str, server: str, frames: list[bytes], notes: str) -> Path:
    run_dir = REPO_ROOT / "captures/raw" / run_id
    run_dir.mkdir(parents=True, exist_ok=True)

    lines = [frm.hex() for frm in frames]
    (run_dir / "official_frames.raw.hex").write_text(
        "\n".join(lines) + ("\n" if lines else ""),
        encoding="utf-8",
    )
    (run_dir / "neo_frames.raw.hex").write_text(
        "\n".join(lines) + ("\n" if lines else ""),
        encoding="utf-8",
    )

    events = [
        {
            "ts": now_iso(),
            "event": "s7c.download-auto.capture",
            "frame_count": len(frames),
            "scenario": "login-search-download-auto",
        }
    ]
    with (run_dir / "frida-events.raw.jsonl").open("w", encoding="utf-8") as fh:
        for row in events:
            fh.write(json.dumps(row, ensure_ascii=True) + "\n")

    manifest = {
        "run_id": run_id,
        "scenario": "login-search-download-auto",
        "source_type": "runtime_deterministic_local_harness",
        "created_at": now_iso(),
        "server": server,
        "frame_count": len(lines),
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
        description="Generate deterministic runtime capture for session download-auto flow."
    )
    parser.add_argument("--server", default="")
    parser.add_argument("--username", default="")
    parser.add_argument("--password", default="")
    parser.add_argument("--client-version", type=int, default=160)
    parser.add_argument("--minor-version", type=int, default=1)
    parser.add_argument("--search-token", type=int, default=9901)
    parser.add_argument("--transfer-token", type=int, default=555)
    parser.add_argument("--virtual-path", default="Music\\Runtime\\auto-track.flac")
    parser.add_argument("--size", type=int, default=123456)
    args = parser.parse_args()

    load_dotenv_local(REPO_ROOT)
    server = require(args.server, "NSS_TEST_SERVER")
    username = require(args.username, "NSS_TEST_USERNAME")
    password = require(args.password, "NSS_TEST_PASSWORD")

    frames = [
        sm_login(username, password, args.client_version, args.minor_version),
        frame(1, login_success_payload()),
        frame(26, u32(args.search_token) + s("runtime auto download")),
        frame(64, search_summary_payload("runtime-peer", args.search_token, args.virtual_path, args.size)),
        frame(40, transfer_request_payload(args.transfer_token, args.virtual_path, args.size)),
        frame(41, transfer_response_payload(args.transfer_token, True, "")),
    ]

    run_dir = write_run(
        run_id="login-search-download-auto",
        server=server,
        frames=frames,
        notes=(
            "Stage 7C deterministic runtime harness for search-select-download orchestration. "
            "Captures login, search request/summary, and transfer request/response sequence used by session download-auto."
        ),
    )

    print(json.dumps({"run_id": "login-search-download-auto", "run_dir": str(run_dir)}, indent=2, ensure_ascii=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
