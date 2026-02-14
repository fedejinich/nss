from __future__ import annotations

import argparse
import json
import os
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


def launch(cmd: list[str], log_path: Path) -> subprocess.Popen[str]:
    log_path.parent.mkdir(parents=True, exist_ok=True)
    log = log_path.open("w", encoding="utf-8")
    return subprocess.Popen(cmd, stdout=log, stderr=subprocess.STDOUT, text=True)


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


def main() -> int:
    parser = argparse.ArgumentParser(description="Run synchronized Frida+PCAP capture session")
    parser.add_argument("--process", default="SoulseekQt")
    parser.add_argument("--duration", type=int, default=60)
    parser.add_argument("--output-root", default="captures/raw")
    parser.add_argument("--label", default="")
    parser.add_argument("--iface", default="")
    parser.add_argument("--bpf", default="tcp")
    parser.add_argument("--skip-pcap", action="store_true")
    parser.add_argument("--python-bin", default=".venv-tools/bin/python")
    parser.add_argument("--frida-script", default="frida/hooks/soulseek_trace.js")
    parser.add_argument("--manifest-name", default="manifest.raw.json")
    parser.add_argument("--frida-events-name", default="frida-events.raw.jsonl")
    parser.add_argument("--pcap-name", default="traffic.raw.pcap")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parent.parent.parent
    output_root = (repo_root / args.output_root).resolve()
    output_root.mkdir(parents=True, exist_ok=True)

    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    run_name = f"{stamp}-{args.label}" if args.label else stamp
    run_dir = output_root / run_name
    run_dir.mkdir(parents=True, exist_ok=True)

    frida_events = run_dir / args.frida_events_name
    frida_log = run_dir / "frida-capture.log"
    pcap_file = run_dir / args.pcap_name
    pcap_log = run_dir / "pcap.log"
    manifest_path = run_dir / args.manifest_name

    python_bin = (repo_root / args.python_bin).resolve()
    frida_capture = (repo_root / "tools/runtime/frida_capture.py").resolve()
    frida_script = (repo_root / args.frida_script).resolve()

    manifest: dict[str, Any] = {
        "created_at": now_iso(),
        "duration_s": args.duration,
        "process": args.process,
        "frida_script": str(frida_script),
        "pcap": {
            "enabled": not args.skip_pcap,
            "iface": args.iface or infer_default_iface(),
            "bpf": args.bpf,
        },
        "outputs": {
            "run_dir": str(run_dir),
            "frida_events": str(frida_events),
            "frida_log": str(frida_log),
            "pcap": str(pcap_file),
            "pcap_log": str(pcap_log),
        },
    }

    frida_cmd = [
        str(python_bin),
        str(frida_capture),
        "--process",
        str(args.process),
        "--script",
        str(frida_script),
        "--output",
        str(frida_events),
        "--duration",
        str(max(args.duration + 5, 10)),
    ]

    procs: list[tuple[str, subprocess.Popen[str]]] = []
    start = time.monotonic()
    manifest["started_at"] = now_iso()
    manifest["commands"] = {"frida": frida_cmd}

    try:
        frida_proc = launch(frida_cmd, frida_log)
        procs.append(("frida", frida_proc))

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
            pcap_proc = launch(pcap_cmd, pcap_log)
            procs.append(("pcap", pcap_proc))

            time.sleep(1.0)
            if pcap_proc.poll() is not None:
                manifest["pcap"]["startup_error"] = "tcpdump exited early"
                manifest["pcap"]["returncode"] = pcap_proc.returncode

        while (time.monotonic() - start) < args.duration:
            time.sleep(0.25)

    finally:
        process_results: dict[str, dict[str, Any]] = {}
        for name, proc in reversed(procs):
            rc = terminate(proc)
            process_results[name] = {"returncode": rc}

        manifest["ended_at"] = now_iso()
        manifest["elapsed_s"] = round(time.monotonic() - start, 3)
        manifest["processes"] = process_results
        manifest_path.write_text(json.dumps(manifest, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")

    print(json.dumps({"run_dir": str(run_dir), "manifest": str(manifest_path)}, ensure_ascii=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
