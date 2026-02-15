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

from tools.runtime.slsk_runtime import drain_frames, frame, now_iso, s, sm_login, u32


CODE_SM_DNET_LEVEL = 126
CODE_SM_DNET_GROUP_LEADER = 127
CODE_SM_DNET_DELIVERY_REPORT = 128
CODE_SM_DNET_CHILD_DEPTH = 129
CODE_SM_FLOOD = 131
CODE_SM_REMOVE_ROOM_OPERATORSHIP = 146
CODE_SM_REMOVE_OWN_ROOM_OPERATORSHIP = 147


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


def room_operatorship_probes(room: str) -> list[tuple[str, bytes]]:
    return [
        ("SM_REMOVE_ROOM_OPERATORSHIP", frame(CODE_SM_REMOVE_ROOM_OPERATORSHIP, s(room))),
        (
            "SM_REMOVE_OWN_ROOM_OPERATORSHIP",
            frame(CODE_SM_REMOVE_OWN_ROOM_OPERATORSHIP, s(room)),
        ),
    ]


def distributed_legacy_probes(root_user: str) -> list[tuple[str, bytes]]:
    return [
        ("SM_DNET_LEVEL", frame(CODE_SM_DNET_LEVEL, u32(2))),
        ("SM_DNET_GROUP_LEADER", frame(CODE_SM_DNET_GROUP_LEADER, s(root_user))),
        ("SM_DNET_DELIVERY_REPORT", frame(CODE_SM_DNET_DELIVERY_REPORT, u32(1))),
        ("SM_DNET_CHILD_DEPTH", frame(CODE_SM_DNET_CHILD_DEPTH, u32(3))),
        ("SM_FLOOD", frame(CODE_SM_FLOOD, u32(1))),
    ]


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


def run_probe_set(
    *,
    server: str,
    login_frame: bytes,
    probes: list[tuple[str, bytes]],
    run_id: str,
    scenario: str,
    notes: str,
) -> Path:
    merged_frames: list[bytes] = []
    events: list[dict[str, object]] = []
    for label, probe in probes:
        wire, error = send_single_probe(server=server, login_frame=login_frame, probe_frame=probe)
        merged_frames.extend(wire)
        event: dict[str, object] = {
            "ts": now_iso(),
            "event": "s6e.legacy-control.probe",
            "scenario": scenario,
            "label": label,
            "wire_frame_count": len(wire),
            "code_counts": count_codes(wire),
        }
        if error:
            event["warning"] = error
        events.append(event)
        time.sleep(0.25)

    return write_run(
        repo_root=REPO_ROOT,
        run_id=run_id,
        scenario=scenario,
        source_type="runtime_socket_server_authenticated",
        server=server,
        frames=merged_frames,
        frida_events=events,
        notes=notes,
    )


def main() -> int:
    parser = argparse.ArgumentParser(
        description=(
            "Generate Stage 6E runtime captures for dedicated legacy opaque control families."
        )
    )
    parser.add_argument("--scenario", choices=("room", "distributed", "all"), default="all")
    parser.add_argument("--server", default="")
    parser.add_argument("--username", default="")
    parser.add_argument("--password", default="")
    parser.add_argument("--client-version", type=int, default=160)
    parser.add_argument("--minor-version", type=int, default=1)
    parser.add_argument("--room", default="private-room")
    parser.add_argument("--root-user", default="branch-root")
    args = parser.parse_args()

    load_dotenv_local(REPO_ROOT)
    server = require(args.server, "NSS_TEST_SERVER")
    username = require(args.username, "NSS_TEST_USERNAME")
    password = require(args.password, "NSS_TEST_PASSWORD")
    login_frame = sm_login(username, password, args.client_version, args.minor_version)

    outputs: list[dict[str, object]] = []
    if args.scenario in ("room", "all"):
        room_run = run_probe_set(
            server=server,
            login_frame=login_frame,
            probes=room_operatorship_probes(args.room),
            run_id="login-legacy-room-operatorship-control",
            scenario="login-legacy-room-operatorship-control",
            notes=(
                "Stage 6E runtime probe for room operatorship revocation controls "
                "(SM_REMOVE_ROOM_OPERATORSHIP, SM_REMOVE_OWN_ROOM_OPERATORSHIP)."
            ),
        )
        outputs.append(
            {
                "run_id": "login-legacy-room-operatorship-control",
                "run_dir": str(room_run),
            }
        )

    if args.scenario in ("distributed", "all"):
        distributed_run = run_probe_set(
            server=server,
            login_frame=login_frame,
            probes=distributed_legacy_probes(args.root_user),
            run_id="login-legacy-distributed-control",
            scenario="login-legacy-distributed-control",
            notes=(
                "Stage 6E runtime probe for distributed legacy controls "
                "(SM_DNET_LEVEL, SM_DNET_GROUP_LEADER, SM_DNET_DELIVERY_REPORT, "
                "SM_DNET_CHILD_DEPTH, SM_FLOOD)."
            ),
        )
        outputs.append(
            {
                "run_id": "login-legacy-distributed-control",
                "run_dir": str(distributed_run),
            }
        )

    print(json.dumps({"server": server, "outputs": outputs}, indent=2, ensure_ascii=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
