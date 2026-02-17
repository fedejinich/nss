from __future__ import annotations

import argparse
import json
import signal
import sys
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import frida


STOP = False


def now_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def on_signal(_signum: int, _frame: Any) -> None:
    global STOP
    STOP = True


def _proc_started_key(proc: Any) -> float:
    started = getattr(proc, "parameters", {}).get("started")
    if started is None:
        return 0.0
    if hasattr(started, "timestamp"):
        try:
            return float(started.timestamp())
        except Exception:
            return 0.0
    return 0.0


def _proc_path(proc: Any) -> str:
    parameters = getattr(proc, "parameters", {})
    path = parameters.get("path", "")
    return str(path or "")


def enumerate_processes_full(device: Any) -> list[Any]:
    try:
        return list(device.enumerate_processes(scope="full"))
    except TypeError:
        return list(device.enumerate_processes())


def candidate_pids(processes: list[Any], *, process_name: str, process_path_contains: str = "") -> list[int]:
    token = process_path_contains.strip().lower()
    selected: list[Any] = []
    for proc in processes:
        if getattr(proc, "name", "") != process_name:
            continue
        if token:
            if token not in _proc_path(proc).lower():
                continue
        selected.append(proc)

    selected.sort(key=lambda proc: (_proc_started_key(proc), getattr(proc, "pid", 0)), reverse=True)
    return [int(getattr(proc, "pid", 0)) for proc in selected if int(getattr(proc, "pid", 0)) > 0]


def attach_target(
    device: Any,
    target: int | str,
    *,
    process_path_contains: str = "",
    attach_timeout_s: float = 8.0,
) -> tuple[Any, int | str]:
    if isinstance(target, int):
        return device.attach(target), target

    token = process_path_contains.strip()
    deadline = time.monotonic() + max(attach_timeout_s, 0.0)
    while True:
        processes = enumerate_processes_full(device)
        pids = candidate_pids(
            processes,
            process_name=target,
            process_path_contains=token,
        )
        if pids:
            last_error: Exception | None = None
            for pid in pids:
                try:
                    return device.attach(pid), pid
                except Exception as exc:  # pragma: no cover - runtime process attach behavior
                    last_error = exc
            if last_error is not None:
                raise last_error

        if not token:
            return device.attach(target), target
        if time.monotonic() >= deadline:
            raise RuntimeError(
                f"no process candidate matched name='{target}' and path token '{token}' within {attach_timeout_s:.1f}s"
            )
        time.sleep(0.2)


def main() -> int:
    parser = argparse.ArgumentParser(description="Capture Frida hook events from SoulseekQt")
    parser.add_argument("--process", default="SoulseekQt", help="Process name or PID")
    parser.add_argument(
        "--process-path-contains",
        default="",
        help="Optional executable path substring to disambiguate same-name processes",
    )
    parser.add_argument("--attach-timeout", type=float, default=8.0)
    parser.add_argument("--script", default="frida/hooks/soulseek_trace.js")
    parser.add_argument("--output", required=True)
    parser.add_argument("--duration", type=float, default=0.0, help="seconds, 0 means run until signal")
    args = parser.parse_args()

    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)

    script_source = Path(args.script).read_text(encoding="utf-8")

    signal.signal(signal.SIGINT, on_signal)
    signal.signal(signal.SIGTERM, on_signal)

    device = frida.get_local_device()
    target: int | str
    try:
        target = int(args.process)
    except ValueError:
        target = args.process

    session, attached = attach_target(
        device,
        target,
        process_path_contains=args.process_path_contains,
        attach_timeout_s=args.attach_timeout,
    )
    print(f"attached process target={attached}", file=sys.stderr)
    script = session.create_script(script_source)

    with output_path.open("w", encoding="utf-8") as fh:

        def on_message(message: dict[str, Any], data: bytes | None) -> None:
            record: dict[str, Any] = {
                "host_time": now_iso(),
                "host_monotonic_ns": time.monotonic_ns(),
                "message": message,
            }
            if data is not None:
                record["data_len"] = len(data)
            fh.write(json.dumps(record, ensure_ascii=True) + "\n")
            fh.flush()

        script.on("message", on_message)
        script.load()

        start = time.monotonic()
        while not STOP:
            if args.duration > 0 and (time.monotonic() - start) >= args.duration:
                break
            time.sleep(0.1)

        try:
            script.unload()
        except frida.InvalidOperationError as exc:  # pragma: no cover - depends on target process lifetime
            if "script is destroyed" not in str(exc).lower():
                raise

        try:
            session.detach()
        except frida.InvalidOperationError as exc:  # pragma: no cover - depends on target process lifetime
            if "session is detached" not in str(exc).lower():
                raise

    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except frida.ProcessNotFoundError:
        print("process not found", file=sys.stderr)
        raise SystemExit(2)
    except Exception as exc:  # pragma: no cover - defensive runtime wrapper
        print(f"frida capture failed: {exc}", file=sys.stderr)
        raise SystemExit(1)
