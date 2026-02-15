#!/usr/bin/env python3
from __future__ import annotations

import argparse
import csv
import json
import time
from pathlib import Path
from typing import Any

REQUIRED_TYPED_RUNTIME_MESSAGES = [
    "SM_SET_PARENT_MIN_SPEED",
    "SM_SET_PARENT_SPEED_CONNECTION_RATIO",
    "SM_GET_ROOM_TICKER",
    "SM_UPLOAD_SPEED",
    "SM_GET_USER_PRIVILEGES_STATUS",
]

REQUIRED_GLOBAL_DISTRIBUTED_RUNS = [
    "login-parent-distributed-control",
    "login-global-room-control",
]

REQUIRED_HYPOTHESIS_CLOSURE_MESSAGES = [
    "SM_GET_USER_PRIVILEGES_STATUS",
    "SM_UPLOAD_SPEED",
]

PM_SHARED_FILES_IN_FOLDER_NAME = "PM_SHARED_FILES_IN_FOLDER"


def read_message_map(path: Path) -> dict[str, dict[str, str]]:
    rows: dict[str, dict[str, str]] = {}
    with path.open("r", encoding="utf-8") as fh:
        for row in csv.DictReader(fh):
            rows[row["name"]] = row
    return rows


def check_typed_runtime_messages(rows: dict[str, dict[str, str]], repo_root: Path) -> dict[str, Any]:
    details: list[dict[str, Any]] = []
    ok = True

    for name in REQUIRED_TYPED_RUNTIME_MESSAGES:
        row = rows.get(name)
        if row is None:
            ok = False
            details.append({"message": name, "ok": False, "reason": "missing from message_map"})
            continue

        confidence_ok = row.get("confidence", "").strip().lower() == "high"
        source = row.get("source", "").strip()
        runtime_source_ok = source.startswith("captures/redacted/")
        source_file_ok = (repo_root / source).exists()
        row_ok = confidence_ok and runtime_source_ok and source_file_ok
        ok = ok and row_ok
        details.append(
            {
                "message": name,
                "ok": row_ok,
                "confidence": row.get("confidence", ""),
                "source": source,
                "checks": {
                    "confidence_high": confidence_ok,
                    "runtime_source": runtime_source_ok,
                    "source_file_exists": source_file_ok,
                },
            }
        )

    return {"ok": ok, "details": details}


def check_global_distributed_runtime_captures(repo_root: Path) -> dict[str, Any]:
    details: list[dict[str, Any]] = []
    ok = True

    for run_id in REQUIRED_GLOBAL_DISTRIBUTED_RUNS:
        run_dir = repo_root / "captures" / "redacted" / run_id
        required_files = [
            "manifest.redacted.json",
            "official_frames.hex",
            "neo_frames.hex",
            "verify-captures-report.json",
        ]
        files_ok = all((run_dir / rel).exists() for rel in required_files)

        source_type_ok = False
        manifest_path = run_dir / "manifest.redacted.json"
        if manifest_path.exists():
            manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
            source_type = str(manifest.get("source_type", ""))
            source_type_ok = source_type.startswith("runtime")
        else:
            source_type = ""

        run_ok = run_dir.exists() and files_ok and source_type_ok
        ok = ok and run_ok
        details.append(
            {
                "run_id": run_id,
                "ok": run_ok,
                "checks": {
                    "run_dir_exists": run_dir.exists(),
                    "required_files_exist": files_ok,
                    "source_type_runtime": source_type_ok,
                },
                "source_type": source_type,
            }
        )

    return {"ok": ok, "details": details}


def check_pm_shared_files_decompression(protocol_lib: Path, rows: dict[str, dict[str, str]], repo_root: Path) -> dict[str, Any]:
    source = protocol_lib.read_text(encoding="utf-8")

    checks = {
        "has_decode_helper": "parse_shared_files_in_folder_payload_decompressed" in source,
        "has_decompress_function": "decompress_shared_files_in_folder_listing" in source,
        "has_zlib_context": "decompress zlib listing" in source,
        "has_binary_entries_test": "shared_files_in_folder_decompression_parser_supports_binary_entries" in source,
        "has_utf8_entries_test": "shared_files_in_folder_decompression_parser_supports_utf8_lines" in source,
        "has_oversized_guard_test": "shared_files_in_folder_decompression_parser_rejects_oversized_listing" in source,
    }

    row = rows.get(PM_SHARED_FILES_IN_FOLDER_NAME)
    row_ok = False
    row_summary: dict[str, Any] = {"message": PM_SHARED_FILES_IN_FOLDER_NAME, "ok": False}
    if row is not None:
        source_path = row.get("source", "").strip()
        row_checks = {
            "confidence_high": row.get("confidence", "").strip().lower() == "high",
            "runtime_source": source_path.startswith("captures/redacted/"),
            "source_file_exists": (repo_root / source_path).exists(),
        }
        row_ok = all(row_checks.values())
        row_summary = {
            "message": PM_SHARED_FILES_IN_FOLDER_NAME,
            "ok": row_ok,
            "confidence": row.get("confidence", ""),
            "source": source_path,
            "checks": row_checks,
        }

    ok = all(checks.values()) and row_ok
    return {"ok": ok, "checks": checks, "message_row": row_summary}


def check_residual_hypotheses(rows: dict[str, dict[str, str]], repo_root: Path) -> dict[str, Any]:
    details: list[dict[str, Any]] = []
    ok = True

    for name in REQUIRED_HYPOTHESIS_CLOSURE_MESSAGES:
        row = rows.get(name)
        if row is None:
            ok = False
            details.append({"message": name, "ok": False, "reason": "missing from message_map"})
            continue

        source = row.get("source", "").strip()
        notes = row.get("notes", "")
        checks = {
            "confidence_high": row.get("confidence", "").strip().lower() == "high",
            "runtime_parent_distributed_source": source.startswith("captures/redacted/login-parent-distributed-control/"),
            "source_file_exists": (repo_root / source).exists(),
            "notes_non_empty": bool(notes.strip()),
        }
        row_ok = all(checks.values())
        ok = ok and row_ok
        details.append(
            {
                "message": name,
                "ok": row_ok,
                "confidence": row.get("confidence", ""),
                "source": source,
                "checks": checks,
            }
        )

    return {"ok": ok, "details": details}


def build_report(repo_root: Path, message_map_path: Path, protocol_lib_path: Path) -> dict[str, Any]:
    rows = read_message_map(message_map_path)

    objective_1 = check_typed_runtime_messages(rows, repo_root)
    objective_2 = check_global_distributed_runtime_captures(repo_root)
    objective_3 = check_pm_shared_files_decompression(protocol_lib_path, rows, repo_root)
    objective_4 = check_residual_hypotheses(rows, repo_root)

    objectives = {
        "opaque_to_typed_runtime_evidence": objective_1,
        "global_distributed_runtime_captures": objective_2,
        "pm_shared_files_decompression_parser": objective_3,
        "residual_hypotheses_closed": objective_4,
    }

    all_ok = all(section.get("ok", False) for section in objectives.values())

    return {
        "generated_unix_secs": int(time.time()),
        "stage": "S5A closure verification",
        "overall_ok": all_ok,
        "objectives": objectives,
        "inputs": {
            "message_map": str(message_map_path.relative_to(repo_root)),
            "protocol_lib": str(protocol_lib_path.relative_to(repo_root)),
            "capture_base": "captures/redacted",
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description="Verify S5A closure requirements and emit JSON report")
    parser.add_argument("--message-map", default="analysis/ghidra/maps/message_map.csv")
    parser.add_argument("--protocol-lib", default="rust/protocol/src/lib.rs")
    parser.add_argument("--out", default="docs/state/s5a-closure-audit.json")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parents[2]
    message_map_path = (repo_root / args.message_map).resolve()
    protocol_lib_path = (repo_root / args.protocol_lib).resolve()
    out_path = (repo_root / args.out).resolve()

    report = build_report(repo_root, message_map_path, protocol_lib_path)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")

    if not report["overall_ok"]:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
