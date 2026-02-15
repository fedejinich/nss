#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import time
from pathlib import Path
from typing import Any

ABS_WIN_RE = re.compile(r"^[A-Za-z]:\\")
CHECKED_BOX_RE = re.compile(r"- \[x\] ")
UNCHECKED_BOX_RE = re.compile(r"- \[ \] ")


def is_absolute_path_string(value: str) -> bool:
    return value.startswith("/") or bool(ABS_WIN_RE.match(value))


def find_abs_paths(obj: Any, *, path: str = "$") -> list[dict[str, str]]:
    findings: list[dict[str, str]] = []
    if isinstance(obj, dict):
        for key, value in obj.items():
            findings.extend(find_abs_paths(value, path=f"{path}.{key}"))
        return findings
    if isinstance(obj, list):
        for idx, value in enumerate(obj):
            findings.extend(find_abs_paths(value, path=f"{path}[{idx}]"))
        return findings
    if isinstance(obj, str) and is_absolute_path_string(obj):
        findings.append({"path": path, "value": obj})
    return findings


def check_redacted_metadata(repo_root: Path) -> dict[str, Any]:
    runs_root = repo_root / "captures" / "redacted"
    run_dirs = [p for p in sorted(runs_root.iterdir()) if p.is_dir()] if runs_root.exists() else []
    details: list[dict[str, Any]] = []
    ok = True

    for run_dir in run_dirs:
        run_detail: dict[str, Any] = {"run_id": run_dir.name, "files": []}
        for file_name in ("manifest.redacted.json", "redaction-summary.json"):
            file_path = run_dir / file_name
            if not file_path.exists():
                run_detail["files"].append({"file": file_name, "ok": True, "findings": []})
                continue
            payload = json.loads(file_path.read_text(encoding="utf-8"))
            findings = find_abs_paths(payload)
            file_ok = len(findings) == 0
            ok = ok and file_ok
            run_detail["files"].append({"file": file_name, "ok": file_ok, "findings": findings})
        details.append(run_detail)

    return {"ok": ok, "runs": len(run_dirs), "details": details}


def check_required_files(repo_root: Path) -> dict[str, Any]:
    required = [
        "scripts/package_release.sh",
        "docs/runbooks/release-packaging.md",
        "docs/runbooks/failure-recovery.md",
        "docs/state/final-closure-checklist.md",
    ]
    details = []
    ok = True
    for rel in required:
        path = repo_root / rel
        exists = path.exists()
        executable = path.is_file() and bool(path.stat().st_mode & 0o111) if rel.endswith(".sh") else True
        row_ok = exists and executable
        ok = ok and row_ok
        details.append(
            {
                "path": rel,
                "exists": exists,
                "executable": executable,
                "ok": row_ok,
            }
        )
    return {"ok": ok, "details": details}


def check_final_closure_checklist(repo_root: Path) -> dict[str, Any]:
    checklist_path = repo_root / "docs/state/final-closure-checklist.md"
    if not checklist_path.exists():
        return {"ok": False, "reason": "missing checklist file", "checked": 0, "unchecked": 0}

    text = checklist_path.read_text(encoding="utf-8")
    checked = len(CHECKED_BOX_RE.findall(text))
    unchecked = len(UNCHECKED_BOX_RE.findall(text))
    return {
        "ok": unchecked == 0 and checked > 0,
        "checked": checked,
        "unchecked": unchecked,
    }


def check_capability_state(repo_root: Path) -> dict[str, Any]:
    registry_path = repo_root / "analysis/state/capability_registry.json"
    if not registry_path.exists():
        return {"ok": False, "reason": "missing capability registry"}

    payload = json.loads(registry_path.read_text(encoding="utf-8"))
    capabilities = payload.get("capabilities", [])
    if not isinstance(capabilities, list):
        return {"ok": False, "reason": "invalid capabilities payload"}

    by_id = {
        str(cap.get("id")): cap
        for cap in capabilities
        if isinstance(cap, dict) and cap.get("id")
    }
    required_ids = [
        "CAP-REDACTION-HARDENING",
        "CAP-PACKAGING-RELEASE",
        "CAP-RECOVERY-RUNBOOKS",
        "CAP-CLOSURE-CHECKLIST",
        "CAP-RELEASE-HARDENING",
    ]

    details = []
    ok = True
    for cap_id in required_ids:
        cap = by_id.get(cap_id)
        if cap is None:
            ok = False
            details.append({"id": cap_id, "ok": False, "reason": "missing"})
            continue
        status_done = str(cap.get("status", "")) == "done"
        blockers = cap.get("blockers", [])
        blockers_empty = isinstance(blockers, list) and len(blockers) == 0
        row_ok = status_done and blockers_empty
        ok = ok and row_ok
        details.append(
            {
                "id": cap_id,
                "status": cap.get("status"),
                "blockers": blockers,
                "ok": row_ok,
            }
        )

    return {"ok": ok, "details": details}


def read_stage_status(repo_root: Path, stage_id: str = "S8C") -> dict[str, Any]:
    registry_path = repo_root / "analysis/state/stage_registry.json"
    if not registry_path.exists():
        return {"ok": False, "status": "unknown", "reason": "missing stage registry"}

    payload = json.loads(registry_path.read_text(encoding="utf-8"))
    stages = payload.get("stages", [])
    if not isinstance(stages, list):
        return {"ok": False, "status": "unknown", "reason": "invalid stages payload"}

    for row in stages:
        if not isinstance(row, dict):
            continue
        if str(row.get("id", "")) != stage_id:
            continue
        status = str(row.get("status", "unknown"))
        if status not in {"done", "in_progress", "planned"}:
            return {"ok": False, "status": status, "reason": f"invalid stage status {status}"}
        return {"ok": True, "status": status}

    return {"ok": False, "status": "unknown", "reason": f"stage {stage_id} not found"}


def build_report(repo_root: Path) -> dict[str, Any]:
    stage_state = read_stage_status(repo_root)
    checks = {
        "redacted_metadata_paths": check_redacted_metadata(repo_root),
        "required_files": check_required_files(repo_root),
        "final_closure_checklist": check_final_closure_checklist(repo_root),
        "capability_registry_state": check_capability_state(repo_root),
    }
    strict_closure_ok = all(section.get("ok", False) for section in checks.values())
    operational_ok = all(
        checks[name].get("ok", False)
        for name in ("redacted_metadata_paths", "required_files")
    )
    stage_status = str(stage_state.get("status", "unknown"))
    if stage_status == "done":
        overall_ok = strict_closure_ok
        mode = "strict_closure"
    elif stage_status == "in_progress":
        overall_ok = bool(stage_state.get("ok", False)) and operational_ok
        mode = "in_progress_operational"
    else:
        overall_ok = False
        mode = "invalid_stage_state"

    return {
        "generated_unix_secs": int(time.time()),
        "stage": "S8C release hardening closure",
        "stage_state": stage_state,
        "mode": mode,
        "overall_ok": overall_ok,
        "strict_closure_ok": strict_closure_ok,
        "operational_ok": operational_ok,
        "checks": checks,
        "inputs": {
            "captures_redacted": "captures/redacted",
            "capability_registry": "analysis/state/capability_registry.json",
            "stage_registry": "analysis/state/stage_registry.json",
            "closure_checklist": "docs/state/final-closure-checklist.md",
        },
    }


def to_markdown(report: dict[str, Any]) -> str:
    checks = report.get("checks", {})
    lines = [
        "# Release Hardening Audit",
        "",
        f"- Stage: `{report.get('stage', '')}`",
        f"- Stage status: `{report.get('stage_state', {}).get('status', 'unknown')}`",
        f"- Mode: `{report.get('mode', 'unknown')}`",
        f"- Operational OK: `{report.get('operational_ok', False)}`",
        f"- Strict closure OK: `{report.get('strict_closure_ok', False)}`",
        f"- Overall OK: `{report.get('overall_ok', False)}`",
        "",
        "## Checks",
        "",
    ]

    for name, payload in checks.items():
        if not isinstance(payload, dict):
            continue
        lines.append(f"### `{name}`")
        lines.append(f"- ok: `{payload.get('ok', False)}`")
        if "checked" in payload:
            lines.append(f"- checked boxes: `{payload.get('checked', 0)}`")
            lines.append(f"- unchecked boxes: `{payload.get('unchecked', 0)}`")
        if "runs" in payload:
            lines.append(f"- runs scanned: `{payload.get('runs', 0)}`")
        lines.append("")

    lines.extend(
        [
            "## Regeneration",
            "",
            "```bash",
            "python3 tools/state/verify_release_hardening.py",
            "```",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description="Verify S8C release hardening closure and emit audit artifacts")
    parser.add_argument("--out-json", default="docs/state/release-hardening-audit.json")
    parser.add_argument("--out-md", default="docs/state/release-hardening-audit.md")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parents[2]
    out_json = (repo_root / args.out_json).resolve()
    out_md = (repo_root / args.out_md).resolve()
    out_json.parent.mkdir(parents=True, exist_ok=True)
    out_md.parent.mkdir(parents=True, exist_ok=True)

    report = build_report(repo_root)
    out_json.write_text(json.dumps(report, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")
    out_md.write_text(to_markdown(report), encoding="utf-8")
    return 0 if report.get("overall_ok", False) else 1


if __name__ == "__main__":
    raise SystemExit(main())
