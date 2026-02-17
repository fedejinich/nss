from __future__ import annotations

import argparse
import json
import subprocess
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


SCENARIOS = {
    "login-only",
    "login-search",
    "login-search-download",
}


def now_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def apple_quote(value: str) -> str:
    return value.replace("\\", "\\\\").replace('"', '\\"')


def run_cmd(cmd: list[str], *, timeout: float = 20.0, stdin_text: str | None = None) -> dict[str, Any]:
    proc = subprocess.run(
        cmd,
        input=stdin_text,
        capture_output=True,
        text=True,
        timeout=timeout,
        check=False,
    )
    return {
        "cmd": cmd,
        "returncode": proc.returncode,
        "stdout": (proc.stdout or "").strip(),
        "stderr": (proc.stderr or "").strip(),
    }


def run_osascript(script: str, *, timeout: float = 20.0) -> dict[str, Any]:
    return run_cmd(["osascript", "-"], timeout=timeout, stdin_text=script)


def check_ui_scripting_enabled() -> tuple[bool, dict[str, Any]]:
    result = run_osascript('tell application "System Events" to return UI elements enabled', timeout=8.0)
    if result["returncode"] != 0:
        return False, result
    enabled = result["stdout"].strip().lower() == "true"
    return enabled, result


def script_activate_application(app_name: str) -> str:
    safe_name = apple_quote(app_name)
    return f"""
tell application "{safe_name}" to activate
return "activated"
"""


def script_focus_process(process_name: str) -> str:
    safe_name = apple_quote(process_name)
    return f"""
tell application "System Events"
  if exists process "{safe_name}" then
    tell process "{safe_name}"
      set frontmost to true
      return "focused"
    end tell
  end if
end tell
return "process-not-found"
"""


def script_login_hint(process_name: str) -> str:
    safe_name = apple_quote(process_name)
    return f"""
tell application "System Events"
  tell process "{safe_name}"
    set frontmost to true
    delay 0.15
    keystroke "l" using {{command down}}
  end tell
end tell
return "login-hotkey-sent"
"""


def script_search(process_name: str, query: str) -> str:
    safe_name = apple_quote(process_name)
    safe_query = apple_quote(query)
    return f"""
tell application "System Events"
  tell process "{safe_name}"
    set frontmost to true
    delay 0.15
    keystroke "f" using {{command down}}
    delay 0.15
    keystroke "a" using {{command down}}
    keystroke "{safe_query}"
    key code 36
  end tell
end tell
return "search-submitted"
"""


def script_download_first_result(process_name: str) -> str:
    safe_name = apple_quote(process_name)
    return f"""
tell application "System Events"
  tell process "{safe_name}"
    set frontmost to true
    delay 0.25
    key code 125
    delay 0.10
    key code 36
  end tell
end tell
return "download-hotkey-sent"
"""


def script_quit_application(process_name: str) -> str:
    safe_name = apple_quote(process_name)
    return f"""
tell application "System Events"
  tell process "{safe_name}"
    set frontmost to true
    delay 0.15
    keystroke "q" using {{command down}}
  end tell
end tell
return "quit-hotkey-sent"
"""


def append_step(
    payload: dict[str, Any],
    *,
    id_: str,
    action: str,
    result: dict[str, Any],
    strict: bool = False,
) -> bool:
    step = {
        "id": id_,
        "action": action,
        "returncode": result["returncode"],
        "stdout": result["stdout"],
        "stderr": result["stderr"],
    }
    payload.setdefault("steps", []).append(step)
    ok = result["returncode"] == 0
    if not ok:
        payload.setdefault("blockers", []).append(
            {
                "code": f"{id_}-failed",
                "detail": result["stderr"] or result["stdout"] or "step failed",
                "at": now_iso(),
            }
        )
    return ok or not strict


def build_scenario_steps(scenario: str, process_name: str, query: str, *, include_quit: bool) -> list[tuple[str, str]]:
    if scenario == "login-only":
        steps = [
            ("focus", script_focus_process(process_name)),
            ("login_hotkey", script_login_hint(process_name)),
        ]
    elif scenario == "login-search":
        steps = [
            ("focus", script_focus_process(process_name)),
            ("login_hotkey", script_login_hint(process_name)),
            ("search_submit", script_search(process_name, query)),
        ]
    else:
        steps = [
            ("focus", script_focus_process(process_name)),
            ("login_hotkey", script_login_hint(process_name)),
            ("search_submit", script_search(process_name, query)),
            ("download_first_result", script_download_first_result(process_name)),
        ]
    if include_quit:
        steps.append(("quit_hotkey", script_quit_application(process_name)))
    return steps


def main() -> int:
    parser = argparse.ArgumentParser(description="Official SoulseekQt UI scenario runner (osascript)")
    parser.add_argument("--scenario", default="login-search-download", choices=sorted(SCENARIOS))
    parser.add_argument("--app-bundle", default="/Applications/SoulseekQt.app")
    parser.add_argument("--app-name", default="SoulseekQt")
    parser.add_argument("--process-name", default="SoulseekQt")
    parser.add_argument("--query", default="aphex twin flim")
    parser.add_argument("--initial-wait", type=float, default=2.0)
    parser.add_argument("--between-step-wait", type=float, default=0.8)
    parser.add_argument("--manual-wait", type=float, default=8.0)
    parser.add_argument("--skip-launch", action="store_true")
    parser.add_argument("--skip-quit", action="store_true")
    parser.add_argument("--require-accessibility", action="store_true")
    parser.add_argument("--notes", default="")
    parser.add_argument("--output", default="")
    args = parser.parse_args()

    output_path = Path(args.output).resolve() if args.output else None
    if output_path is not None:
        output_path.parent.mkdir(parents=True, exist_ok=True)

    payload: dict[str, Any] = {
        "created_at": now_iso(),
        "scenario": args.scenario,
        "app_bundle": args.app_bundle,
        "app_name": args.app_name,
        "process_name": args.process_name,
        "query": args.query,
        "notes": args.notes,
        "steps": [],
        "blockers": [],
    }

    if not args.skip_launch:
        launch_result = run_cmd(["open", "-na", args.app_bundle], timeout=15.0)
        append_step(payload, id_="launch", action="open -na", result=launch_result, strict=False)

    if args.initial_wait > 0:
        time.sleep(args.initial_wait)

    if not args.skip_launch:
        activate_result = run_osascript(script_activate_application(args.app_name), timeout=8.0)
        append_step(payload, id_="activate", action="activate app", result=activate_result, strict=False)

    ui_enabled, ui_result = check_ui_scripting_enabled()
    payload["ui_scripting"] = {
        "enabled": ui_enabled,
        "check_stdout": ui_result["stdout"],
        "check_stderr": ui_result["stderr"],
        "check_returncode": ui_result["returncode"],
    }

    if not ui_enabled:
        payload["blockers"].append(
            {
                "code": "ui-scripting-disabled",
                "detail": "System Events accessibility is disabled or unavailable for this process",
                "at": now_iso(),
            }
        )
        if args.require_accessibility:
            payload["status"] = "blocked"
            payload["ended_at"] = now_iso()
            if output_path is not None:
                output_path.write_text(json.dumps(payload, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")
            print(json.dumps(payload, ensure_ascii=True))
            return 2

        payload["status"] = "manual_fallback"
        if args.manual_wait > 0:
            time.sleep(args.manual_wait)
        payload["ended_at"] = now_iso()
        if output_path is not None:
            output_path.write_text(json.dumps(payload, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")
        print(json.dumps(payload, ensure_ascii=True))
        return 0

    for step_id, script in build_scenario_steps(
        args.scenario,
        args.process_name,
        args.query,
        include_quit=not args.skip_quit,
    ):
        result = run_osascript(script, timeout=12.0)
        append_step(payload, id_=step_id, action="osascript", result=result, strict=False)
        if args.between_step_wait > 0:
            time.sleep(args.between_step_wait)

    payload["status"] = "completed" if not payload["blockers"] else "completed_with_blockers"
    payload["ended_at"] = now_iso()

    if output_path is not None:
        output_path.write_text(json.dumps(payload, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")
    print(json.dumps(payload, ensure_ascii=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
