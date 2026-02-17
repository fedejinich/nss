from __future__ import annotations

import argparse
import json
import os
import shlex
import signal
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


def now_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def infer_default_iface() -> str:
    if sys.platform == "darwin":
        return "en0"
    return "any"


def launch(cmd: list[str], log_path: Path, env: dict[str, str] | None = None) -> subprocess.Popen[str]:
    log_path.parent.mkdir(parents=True, exist_ok=True)
    log = log_path.open("w", encoding="utf-8")
    return subprocess.Popen(cmd, stdout=log, stderr=subprocess.STDOUT, text=True, env=env)


def terminate(proc: subprocess.Popen[str], timeout: float = 5.0) -> int:
    if proc.poll() is not None:
        return proc.returncode
    proc.send_signal(signal.SIGTERM)
    try:
        proc.wait(timeout=timeout)
    except subprocess.TimeoutExpired:
        proc.kill()
        proc.wait(timeout=2.0)
    return proc.returncode


def add_blocker(manifest: dict[str, Any], *, code: str, detail: str) -> None:
    blockers = manifest.setdefault("blockers", [])
    blockers.append({"code": code, "detail": detail, "at": now_iso()})


def build_frida_cmd(
    *,
    python_bin: Path,
    frida_capture: Path,
    process: str,
    process_path_contains: str,
    script: Path,
    output: Path,
    duration: int,
) -> list[str]:
    cmd = [
        str(python_bin),
        str(frida_capture),
        "--process",
        process,
        "--script",
        str(script),
        "--output",
        str(output),
        "--duration",
        str(max(duration + 5, 10)),
    ]
    if process_path_contains:
        cmd.extend(["--process-path-contains", process_path_contains])
    return cmd


def profile_env(base_env: dict[str, str], profile_root: Path | None) -> dict[str, str]:
    if profile_root is None:
        return base_env

    profile_root.mkdir(parents=True, exist_ok=True)
    env = dict(base_env)
    env["HOME"] = str(profile_root)
    env["XDG_CONFIG_HOME"] = str(profile_root / ".config")
    env["XDG_DATA_HOME"] = str(profile_root / ".local" / "share")
    env["XDG_CACHE_HOME"] = str(profile_root / ".cache")
    return env


def main() -> int:
    parser = argparse.ArgumentParser(description="Run synchronized Frida+PCAP capture session")
    parser.add_argument("--process", default="SoulseekQt")
    parser.add_argument("--duration", type=int, default=60)
    parser.add_argument("--output-root", default="captures/raw")
    parser.add_argument("--label", default="")
    parser.add_argument("--scenario-id", default="")
    parser.add_argument("--process-path-contains", default="")
    parser.add_argument("--notes", default="")
    parser.add_argument("--iface", default="")
    parser.add_argument("--bpf", default="tcp")
    parser.add_argument("--skip-pcap", action="store_true")
    parser.add_argument("--python-bin", default=".venv-tools/bin/python")
    parser.add_argument("--frida-script", default="frida/hooks/soulseek_trace.js")
    parser.add_argument("--io-frida-script", default="frida/hooks/soulseek_io_trace.js")
    parser.add_argument("--hook-set", choices=("protocol", "io", "both"), default="protocol")
    parser.add_argument("--manifest-name", default="manifest.raw.json")
    parser.add_argument("--frida-events-name", default="frida-events.raw.jsonl")
    parser.add_argument("--io-events-name", default="io-events.raw.jsonl")
    parser.add_argument("--pcap-name", default="traffic.raw.pcap")
    parser.add_argument("--launch-binary", default="")
    parser.add_argument("--profile-root", default="")
    parser.add_argument("--startup-delay", type=float, default=1.0)
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parent.parent.parent
    output_root = (repo_root / args.output_root).resolve()
    output_root.mkdir(parents=True, exist_ok=True)

    scenario_id = args.scenario_id or args.label
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    run_label = args.label or args.scenario_id
    run_name = f"{stamp}-{run_label}" if run_label else stamp
    run_dir = output_root / run_name
    run_dir.mkdir(parents=True, exist_ok=True)

    frida_events = run_dir / args.frida_events_name
    io_events = run_dir / args.io_events_name
    frida_log = run_dir / "frida-capture.log"
    io_frida_log = run_dir / "io-frida-capture.log"
    pcap_file = run_dir / args.pcap_name
    pcap_log = run_dir / "pcap.log"
    launch_log = run_dir / "launch-binary.log"
    manifest_path = run_dir / args.manifest_name

    # Keep virtualenv interpreter path intact; resolving symlinks can escape venv site-packages.
    python_bin = repo_root / args.python_bin
    frida_capture = (repo_root / "tools/runtime/frida_capture.py").resolve()
    frida_script = (repo_root / args.frida_script).resolve()
    io_frida_script = (repo_root / args.io_frida_script).resolve()
    profile_root = (Path(args.profile_root).expanduser().resolve() if args.profile_root else None)

    manifest: dict[str, Any] = {
        "created_at": now_iso(),
        "duration_s": args.duration,
        "process": args.process,
        "process_path_contains": args.process_path_contains,
        "scenario": scenario_id,
        "label": args.label,
        "notes": args.notes,
        "hook_set": args.hook_set,
        "profile_root": str(profile_root) if profile_root else "",
        "frida_scripts": {
            "protocol": str(frida_script),
            "io": str(io_frida_script),
        },
        "pcap": {
            "enabled": not args.skip_pcap,
            "iface": args.iface or infer_default_iface(),
            "bpf": args.bpf,
        },
        "outputs": {
            "run_dir": str(run_dir),
            "frida_events": str(frida_events),
            "io_events": str(io_events),
            "frida_log": str(frida_log),
            "io_frida_log": str(io_frida_log),
            "pcap": str(pcap_file),
            "pcap_log": str(pcap_log),
            "launch_log": str(launch_log),
        },
        "commands": {},
        "blockers": [],
    }

    procs: list[tuple[str, subprocess.Popen[str]]] = []
    start = time.monotonic()
    manifest["started_at"] = now_iso()

    try:
        base_env = os.environ.copy()
        launch_env = profile_env(base_env, profile_root)

        if args.launch_binary:
            launch_cmd = shlex.split(args.launch_binary)
            if not launch_cmd:
                add_blocker(manifest, code="launch-empty-command", detail="launch-binary argument did not parse into a command")
            else:
                manifest["commands"]["launch"] = launch_cmd
                launch_proc = launch(launch_cmd, launch_log, env=launch_env)
                procs.append(("launch", launch_proc))
                time.sleep(max(args.startup_delay, 0.0))
                if launch_proc.poll() is not None:
                    add_blocker(manifest, code="launch-exited-early", detail=f"launch process exited early rc={launch_proc.returncode}")

        if args.hook_set in {"protocol", "both"}:
            if not frida_script.exists():
                add_blocker(manifest, code="missing-protocol-frida-script", detail=str(frida_script))
            else:
                frida_cmd = build_frida_cmd(
                    python_bin=python_bin,
                    frida_capture=frida_capture,
                    process=args.process,
                    process_path_contains=args.process_path_contains,
                    script=frida_script,
                    output=frida_events,
                    duration=args.duration,
                )
                manifest["commands"]["frida_protocol"] = frida_cmd
                frida_proc = launch(frida_cmd, frida_log, env=base_env)
                procs.append(("frida_protocol", frida_proc))
                time.sleep(0.2)

        if args.hook_set in {"io", "both"}:
            if not io_frida_script.exists():
                add_blocker(manifest, code="missing-io-frida-script", detail=str(io_frida_script))
            else:
                io_frida_cmd = build_frida_cmd(
                    python_bin=python_bin,
                    frida_capture=frida_capture,
                    process=args.process,
                    process_path_contains=args.process_path_contains,
                    script=io_frida_script,
                    output=io_events,
                    duration=args.duration,
                )
                manifest["commands"]["frida_io"] = io_frida_cmd
                io_proc = launch(io_frida_cmd, io_frida_log, env=base_env)
                procs.append(("frida_io", io_proc))
                time.sleep(0.2)

        if not args.skip_pcap:
            iface = args.iface or infer_default_iface()
            pcap_cmd = [
                "tcpdump",
                "-i",
                iface,
                "-w",
                str(pcap_file),
                args.bpf,
            ]
            manifest["commands"]["pcap"] = pcap_cmd
            pcap_proc = launch(pcap_cmd, pcap_log, env=base_env)
            procs.append(("pcap", pcap_proc))

            time.sleep(1.0)
            if pcap_proc.poll() is not None:
                manifest["pcap"]["startup_error"] = "tcpdump exited early"
                manifest["pcap"]["returncode"] = pcap_proc.returncode
                add_blocker(
                    manifest,
                    code="pcap-startup-failed",
                    detail=f"tcpdump exited early rc={pcap_proc.returncode}",
                )

        while (time.monotonic() - start) < args.duration:
            time.sleep(0.25)

    finally:
        process_results: dict[str, dict[str, Any]] = {}
        for name, proc in reversed(procs):
            rc = terminate(proc)
            process_results[name] = {"returncode": rc}
            if rc not in (0, -15):
                add_blocker(manifest, code=f"{name}-nonzero-exit", detail=f"{name} exited with rc={rc}")

        manifest["ended_at"] = now_iso()
        manifest["elapsed_s"] = round(time.monotonic() - start, 3)
        manifest["processes"] = process_results
        manifest_path.write_text(json.dumps(manifest, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")

    print(json.dumps({"run_dir": str(run_dir), "manifest": str(manifest_path)}, ensure_ascii=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
