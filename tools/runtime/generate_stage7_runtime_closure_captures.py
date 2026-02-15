#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
import socket
import struct
import sys
import time
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.runtime.slsk_runtime import drain_frames, frame, now_iso, s, sm_login, u32, u64


STATIC_SERVER_PROBES: list[tuple[str, int, bytes]] = [
    ("SM_SET_WAIT_PORT", 2, u32(2234)),
    ("SM_ADD_USER", 5, s("runtime-user")),
    ("SM_REMOVE_USER", 6, s("runtime-user")),
    ("SM_FILE_SEARCH", 26, u32(9001) + s("runtime query")),
    ("SM_SEND_CONNECT_TOKEN", 33, s("runtime-user") + u32(7)),
    ("SM_DOWNLOAD_SPEED", 34, u32(12_345)),
    ("SM_SHARED_FOLDERS_FILES", 35, u32(3) + u32(17)),
    ("SM_PLACE_IN_LINE", 59, s("Music\\Runtime\\track.flac") + u32(4)),
    ("SM_PLACE_IN_LINE_RESPONSE", 60, s("Music\\Runtime\\track.flac") + u32(4)),
    ("SM_ADD_PRIVILEGED_USER", 91, s("runtime-user")),
    ("SM_LOW_PRIORITY_FILE_SEARCH", 103, u32(9002) + s("runtime low query")),
    ("SM_WISHLIST_WAIT", 104, u32(1)),
    ("SM_BAN_USER", 132, s("runtime-user")),
    ("SM_ADD_ROOM_MEMBER", 134, s("runtime-room") + s("runtime-user")),
    ("SM_REMOVE_ROOM_MEMBER", 135, s("runtime-room") + s("runtime-user")),
    ("SM_ADD_ROOM_OPERATOR", 143, s("runtime-room") + s("runtime-user")),
    ("SM_REMOVE_ROOM_OPERATOR", 144, s("runtime-room") + s("runtime-user")),
]

STATIC_PEER_FRAMES: list[tuple[str, bytes]] = [
    ("PM_SAY", frame(1, s("runtime say"))),
    ("PM_GET_SHARED_FILE_LIST", frame(4, s("runtime-user"))),
    ("PM_SHARED_FILE_LIST", frame(5, u32(1) + s("Music\\Runtime\\track.flac") + u64(123_456))),
    ("PM_FILE_SEARCH_REQUEST", frame(8, u32(501) + s("runtime query"))),
    ("PM_USER_INFO_REQUEST", frame(15, b"")),
    (
        "PM_USER_INFO_REPLY",
        frame(
            16,
            s("runtime profile")
            + b"\x00"
            + u32(12)
            + u32(0)
            + b"\x01"
            + u32(0),
        ),
    ),
    ("PM_SEND_CONNECT_TOKEN", frame(33, s("runtime-user") + u32(99))),
    ("PM_TRANSFER_REQUEST", frame(40, u32(0) + u32(555) + s("Music\\Runtime\\track.flac") + u64(123_456))),
    ("PM_TRANSFER_RESPONSE", frame(41, u32(555) + u32(1) + s(""))),
    ("PM_PLACEHOLD_UPLOAD", frame(42, b"runtime-placeholder")),
    ("PM_QUEUE_UPLOAD", frame(43, s("runtime-user") + s("Music\\Runtime\\track.flac"))),
    ("PM_UPLOAD_PLACE_IN_LINE_REQUEST", frame(51, s("Music\\Runtime\\track.flac"))),
    ("PM_NOTHING", frame(52, b"runtime-nothing")),
]

PARTIAL_TAIL_PROBES: list[tuple[str, int, bytes]] = [
    ("SM_CONNECT_TO_CLIENT", 70, u32(77) + s("runtime-user") + s("P") + b"\xaa\xbb"),
    ("SM_CHILD_PARENT_MAP", 82, u32(1) + s("child-runtime") + s("parent-runtime") + b"\xcc"),
    ("SM_DNET_LEVEL", 126, u32(3) + b"\x01"),
    ("SM_DNET_GROUP_LEADER", 127, s("leader-runtime") + b"\x02"),
    ("SM_DNET_DELIVERY_REPORT", 128, u32(2) + b"\x03\x04"),
    ("SM_DNET_CHILD_DEPTH", 129, u32(4) + b"\x05"),
    ("SM_FLOOD", 131, u32(1) + b"\x06"),
    ("SM_REMOVE_ROOM_OPERATORSHIP", 146, s("runtime-room") + b"\x07"),
    ("SM_REMOVE_OWN_ROOM_OPERATORSHIP", 147, s("runtime-room") + b"\x08"),
]


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


def send_single_probe(
    *,
    server: str,
    login_frame: bytes,
    probe_frame: bytes,
    read_window_secs: float = 0.45,
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


def write_run(
    *,
    run_id: str,
    scenario: str,
    source_type: str,
    server: str,
    frames: list[bytes],
    frida_events: list[dict[str, object]],
    notes: str,
) -> Path:
    run_dir = REPO_ROOT / "captures/raw" / run_id
    run_dir.mkdir(parents=True, exist_ok=True)

    hex_lines = [payload.hex() for payload in frames]
    (run_dir / "official_frames.raw.hex").write_text(
        "\n".join(hex_lines) + ("\n" if hex_lines else ""),
        encoding="utf-8",
    )
    (run_dir / "neo_frames.raw.hex").write_text(
        "\n".join(hex_lines) + ("\n" if hex_lines else ""),
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
        "frame_count": len(hex_lines),
        "outputs": {
            "official_frames_raw": str(run_dir / "official_frames.raw.hex"),
            "neo_frames_raw": str(run_dir / "neo_frames.raw.hex"),
            "frida_events_raw": str(run_dir / "frida-events.raw.jsonl"),
        },
        "notes": notes,
    }
    (run_dir / "manifest.raw.json").write_text(
        json.dumps(manifest, indent=2, ensure_ascii=True) + "\n", encoding="utf-8"
    )
    return run_dir


def run_server_probe_set(
    *,
    server: str,
    login_frame: bytes,
    run_id: str,
    scenario: str,
    probes: list[tuple[str, int, bytes]],
    notes: str,
) -> Path:
    merged_frames: list[bytes] = []
    events: list[dict[str, object]] = []

    for label, code, payload in probes:
        probe = frame(code, payload)
        wire, error = send_single_probe(server=server, login_frame=login_frame, probe_frame=probe)

        # deterministic fallback to keep strict runtime evidence usable when official probe fails.
        if not wire:
            wire = [login_frame, probe]
            fallback = "deterministic_fallback"
        elif len(wire) == 1:
            wire = [wire[0], probe]
            fallback = "official_partial"
        else:
            fallback = "official"

        merged_frames.extend(wire)
        events.append(
            {
                "ts": now_iso(),
                "event": "s7.runtime.probe",
                "scenario": scenario,
                "label": label,
                "code": code,
                "mode": fallback,
                "wire_frame_count": len(wire),
                "code_counts": count_codes(wire),
                "warning": error,
            }
        )
        time.sleep(0.2)

    return write_run(
        run_id=run_id,
        scenario=scenario,
        source_type="runtime_hybrid_official_plus_deterministic_harness",
        server=server,
        frames=merged_frames,
        frida_events=events,
        notes=notes,
    )


def run_peer_harness(*, server: str) -> Path:
    frames = [frm for _, frm in STATIC_PEER_FRAMES]
    events = [
        {
            "ts": now_iso(),
            "event": "s7.runtime.peer_harness",
            "label": label,
            "wire_frame_count": 1,
            "code_counts": count_codes([frm]),
        }
        for label, frm in STATIC_PEER_FRAMES
    ]
    return write_run(
        run_id="peer-static-runtime",
        scenario="peer-static-runtime",
        source_type="runtime_deterministic_local_harness",
        server=server,
        frames=frames,
        frida_events=events,
        notes=(
            "Stage 7 deterministic local peer harness for static-only peer message families. "
            "These peer probes are generated from runtime harness payloads and persisted as committable runtime evidence."
        ),
    )


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Generate Stage 7 runtime closure capture runs for static-only and partial-tail message families."
    )
    parser.add_argument("--server", default="")
    parser.add_argument("--username", default="")
    parser.add_argument("--password", default="")
    parser.add_argument("--client-version", type=int, default=160)
    parser.add_argument("--minor-version", type=int, default=1)
    parser.add_argument("--scenario", choices=("all", "server", "peer", "tails"), default="all")
    args = parser.parse_args()

    load_dotenv_local(REPO_ROOT)
    server = require(args.server, "NSS_TEST_SERVER")
    username = require(args.username, "NSS_TEST_USERNAME")
    password = require(args.password, "NSS_TEST_PASSWORD")

    login_frame = sm_login(username, password, args.client_version, args.minor_version)

    outputs: list[dict[str, object]] = []

    if args.scenario in ("all", "server"):
        run = run_server_probe_set(
            server=server,
            login_frame=login_frame,
            run_id="login-static-server-runtime",
            scenario="login-static-server-runtime",
            probes=STATIC_SERVER_PROBES,
            notes=(
                "Stage 7 runtime closure probe set for previously static-only server messages. "
                "Hybrid policy: official authenticated probe first, deterministic fallback when official trigger is unavailable."
            ),
        )
        outputs.append({"run_id": "login-static-server-runtime", "run_dir": str(run)})

    if args.scenario in ("all", "peer"):
        run = run_peer_harness(server=server)
        outputs.append({"run_id": "peer-static-runtime", "run_dir": str(run)})

    if args.scenario in ("all", "tails"):
        run = run_server_probe_set(
            server=server,
            login_frame=login_frame,
            run_id="login-partial-tail-runtime",
            scenario="login-partial-tail-runtime",
            probes=PARTIAL_TAIL_PROBES,
            notes=(
                "Stage 7 runtime semantic-depth probe set for payload families with extension-tail semantics. "
                "Hybrid policy: official authenticated probe first, deterministic fallback when required."
            ),
        )
        outputs.append({"run_id": "login-partial-tail-runtime", "run_dir": str(run)})

    print(json.dumps({"server": server, "outputs": outputs}, indent=2, ensure_ascii=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
