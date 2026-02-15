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

from tools.runtime.slsk_runtime import drain_frames, frame, now_iso, sm_login, u32


CODE_SM_DNET_DELIVERY_REPORT = 128
CODE_SM_FLOOD = 131


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
            except Exception as exc:  # pragma: no cover - runtime guard
                error = f"login_drain: {exc}"

            wire.append(probe_frame)
            sock.sendall(probe_frame)
            try:
                wire.extend(drain_frames(sock, duration_s=read_window_secs))
            except Exception as exc:  # pragma: no cover - runtime guard
                error = f"{error}; probe_drain: {exc}" if error else f"probe_drain: {exc}"
    except Exception as exc:  # pragma: no cover - runtime guard
        error = str(exc)

    return wire, error


def residual_probes(delivery_values: list[int], flood_values: list[int]) -> list[tuple[str, bytes]]:
    probes: list[tuple[str, bytes]] = []
    for value in delivery_values:
        probes.append(
            (
                f"SM_DNET_DELIVERY_REPORT[{value}]",
                frame(CODE_SM_DNET_DELIVERY_REPORT, u32(value)),
            )
        )
    for value in flood_values:
        probes.append((f"SM_FLOOD[{value}]", frame(CODE_SM_FLOOD, u32(value))))
    return probes


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


def parse_u32_values(raw: str, *, default: list[int]) -> list[int]:
    tokenized = [item.strip() for item in raw.split(",") if item.strip()]
    if not tokenized:
        return default
    values: list[int] = []
    for token in tokenized:
        value = int(token, 10)
        if value < 0 or value > 0xFFFF_FFFF:
            raise ValueError(f"value out of u32 range: {value}")
        values.append(value)
    return values


def main() -> int:
    parser = argparse.ArgumentParser(
        description=(
            "Generate Stage 6F runtime captures for dedicated residual controls "
            "(SM_DNET_DELIVERY_REPORT and SM_FLOOD)."
        )
    )
    parser.add_argument("--server", default="")
    parser.add_argument("--username", default="")
    parser.add_argument("--password", default="")
    parser.add_argument("--client-version", type=int, default=160)
    parser.add_argument("--minor-version", type=int, default=1)
    parser.add_argument("--delivery-values", default="0,1,2")
    parser.add_argument("--flood-values", default="0,1,2")
    args = parser.parse_args()

    load_dotenv_local(REPO_ROOT)
    server = require(args.server, "NSS_TEST_SERVER")
    username = require(args.username, "NSS_TEST_USERNAME")
    password = require(args.password, "NSS_TEST_PASSWORD")

    delivery_values = parse_u32_values(args.delivery_values, default=[0, 1, 2])
    flood_values = parse_u32_values(args.flood_values, default=[0, 1, 2])

    login_frame = sm_login(username, password, args.client_version, args.minor_version)
    probes = residual_probes(delivery_values, flood_values)

    merged_frames: list[bytes] = []
    events: list[dict[str, object]] = []
    for label, probe in probes:
        wire, error = send_single_probe(server=server, login_frame=login_frame, probe_frame=probe)
        merged_frames.extend(wire)
        event: dict[str, object] = {
            "ts": now_iso(),
            "event": "s6f.legacy-residual.probe",
            "scenario": "login-legacy-residual-control",
            "label": label,
            "wire_frame_count": len(wire),
            "code_counts": count_codes(wire),
        }
        if error:
            event["warning"] = error
        events.append(event)
        time.sleep(0.25)

    run_dir = write_run(
        repo_root=REPO_ROOT,
        run_id="login-legacy-residual-control",
        scenario="login-legacy-residual-control",
        source_type="runtime_socket_server_authenticated",
        server=server,
        frames=merged_frames,
        frida_events=events,
        notes=(
            "Stage 6F runtime probe for dedicated residual controls "
            "(SM_DNET_DELIVERY_REPORT, SM_FLOOD) using multi-value u32 payloads."
        ),
    )

    print(
        json.dumps(
            {
                "server": server,
                "run_id": "login-legacy-residual-control",
                "run_dir": str(run_dir),
                "delivery_values": delivery_values,
                "flood_values": flood_values,
            },
            indent=2,
            ensure_ascii=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
