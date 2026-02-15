#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import time
from collections import Counter
from pathlib import Path


def load_json(path: Path) -> dict[str, object]:
    return json.loads(path.read_text(encoding="utf-8"))


def validate_stage_registry(stages: list[dict[str, object]]) -> None:
    required = {
        "id",
        "title",
        "status",
        "owner_area",
        "depends_on",
        "evidence",
        "next_gate",
        "notes",
    }
    seen_ids: set[str] = set()

    for row in stages:
        missing = sorted(required - set(row))
        if missing:
            raise ValueError(f"stage entry missing fields: {missing}")

        stage_id = str(row["id"])
        if stage_id in seen_ids:
            raise ValueError(f"duplicate stage id: {stage_id}")
        seen_ids.add(stage_id)

        status = str(row["status"])
        if status not in {"done", "in_progress", "planned"}:
            raise ValueError(f"invalid stage status for {stage_id}: {status}")

        depends_on = row["depends_on"]
        if not isinstance(depends_on, list):
            raise ValueError(f"depends_on must be a list for stage {stage_id}")

    for row in stages:
        stage_id = str(row["id"])
        for dep in row["depends_on"]:  # type: ignore[index]
            if str(dep) not in seen_ids:
                raise ValueError(f"unknown dependency {dep} in stage {stage_id}")


def build_dashboard_data(
    *,
    stages_payload: dict[str, object],
    matrix_payload: dict[str, object],
    runtime_coverage_payload: dict[str, object],
    capability_matrix_payload: dict[str, object],
) -> dict[str, object]:
    stages = stages_payload.get("stages", [])
    if not isinstance(stages, list):
        raise ValueError("stage registry must contain a list under 'stages'")

    stage_rows: list[dict[str, object]] = [dict(row) for row in stages if isinstance(row, dict)]
    validate_stage_registry(stage_rows)

    counts = Counter(str(row["status"]) for row in stage_rows)
    dependencies: list[dict[str, str]] = []
    for row in stage_rows:
        stage_id = str(row["id"])
        for dep in row["depends_on"]:  # type: ignore[index]
            dependencies.append({"from": str(dep), "to": stage_id})

    current_focus = next((row for row in stage_rows if row["status"] == "in_progress"), None)
    if current_focus is None:
        current_focus = next((row for row in stage_rows if row["status"] == "planned"), None)

    snapshot = matrix_payload.get("snapshot", {})
    matrix_snapshot = snapshot if isinstance(snapshot, dict) else {}
    runtime_summary = runtime_coverage_payload.get("summary", {})
    runtime_summary = runtime_summary if isinstance(runtime_summary, dict) else {}
    capability_summary = capability_matrix_payload.get("summary", {})
    capability_summary = capability_summary if isinstance(capability_summary, dict) else {}
    final_gates = capability_matrix_payload.get("final_gates", [])
    final_gate_rows = [row for row in final_gates if isinstance(row, dict)]
    final_gate_pass = sum(1 for row in final_gate_rows if row.get("status") == "pass")
    final_gate_blocked = sum(1 for row in final_gate_rows if row.get("status") != "pass")

    return {
        "generated_unix_secs": int(time.time()),
        "stage_summary": {
            "total": len(stage_rows),
            "done": counts.get("done", 0),
            "in_progress": counts.get("in_progress", 0),
            "planned": counts.get("planned", 0),
        },
        "protocol_summary": {
            "total_messages": int(matrix_snapshot.get("total_messages", 0) or 0),
            "implemented_mapped": int(matrix_snapshot.get("implemented_mapped", 0) or 0),
            "mapped_not_implemented": int(matrix_snapshot.get("mapped_not_implemented", 0) or 0),
            "implemented_not_mapped": int(matrix_snapshot.get("implemented_not_mapped", 0) or 0),
            "missing": int(matrix_snapshot.get("missing", 0) or 0),
            "server_messages": int(matrix_snapshot.get("server_messages", 0) or 0),
            "peer_messages": int(matrix_snapshot.get("peer_messages", 0) or 0),
        },
        "runtime_summary": {
            "verified_runtime": int(runtime_summary.get("verified_runtime", 0) or 0),
            "verified_static": int(runtime_summary.get("verified_static", 0) or 0),
            "runtime_gap": int(runtime_summary.get("runtime_gap", 0) or 0),
            "semantic_tail_gaps": int(
                runtime_summary.get("messages_with_unresolved_semantic_tail", 0) or 0
            ),
        },
        "capability_summary": {
            "total_capabilities": int(capability_summary.get("total_capabilities", 0) or 0),
            "required_for_final": int(capability_summary.get("required_for_final", 0) or 0),
            "required_done": int(capability_summary.get("required_done", 0) or 0),
            "required_pending": int(capability_summary.get("required_pending", 0) or 0),
            "final_gate_total": len(final_gate_rows),
            "final_gate_pass": final_gate_pass,
            "final_gate_blocked": final_gate_blocked,
        },
        "current_focus": current_focus,
        "stages": stage_rows,
        "dependencies": dependencies,
        "source_artifacts": {
            "stage_registry": "analysis/state/stage_registry.json",
            "protocol_matrix_json": "docs/state/protocol-matrix.json",
            "runtime_coverage_json": "docs/state/runtime-coverage.json",
            "capability_matrix_json": "docs/state/capability-matrix.json",
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate dashboard data JSON from stage and matrix artifacts")
    parser.add_argument("--stage-registry", default="analysis/state/stage_registry.json")
    parser.add_argument("--protocol-matrix-json", default="docs/state/protocol-matrix.json")
    parser.add_argument("--runtime-coverage-json", default="docs/state/runtime-coverage.json")
    parser.add_argument("--capability-matrix-json", default="docs/state/capability-matrix.json")
    parser.add_argument("--out", default="docs/state/project-dashboard-data.json")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parents[2]
    stage_registry_path = (repo_root / args.stage_registry).resolve()
    protocol_matrix_json_path = (repo_root / args.protocol_matrix_json).resolve()
    runtime_coverage_json_path = (repo_root / args.runtime_coverage_json).resolve()
    capability_matrix_json_path = (repo_root / args.capability_matrix_json).resolve()
    out_path = (repo_root / args.out).resolve()

    data = build_dashboard_data(
        stages_payload=load_json(stage_registry_path),
        matrix_payload=load_json(protocol_matrix_json_path),
        runtime_coverage_payload=load_json(runtime_coverage_json_path),
        capability_matrix_payload=load_json(capability_matrix_json_path),
    )
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
