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


def main() -> int:
    parser = argparse.ArgumentParser(description="Capture Frida hook events from SoulseekQt")
    parser.add_argument("--process", default="SoulseekQt", help="Process name or PID")
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

    session = device.attach(target)
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

        script.unload()
        session.detach()

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
