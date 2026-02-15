#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import time
from collections import Counter
from pathlib import Path


def read_json(path: Path) -> dict[str, object]:
    return json.loads(path.read_text(encoding="utf-8"))


def normalize_capabilities(payload: dict[str, object]) -> list[dict[str, object]]:
    caps = payload.get("capabilities", [])
    if not isinstance(caps, list):
        return []
    rows = [row for row in caps if isinstance(row, dict)]
    return rows


def dependency_depth(cap_map: dict[str, dict[str, object]], cap_id: str, memo: dict[str, int]) -> int:
    if cap_id in memo:
        return memo[cap_id]
    cap = cap_map.get(cap_id)
    if not cap:
        memo[cap_id] = 0
        return 0

    deps = cap.get("depends_on", [])
    if not isinstance(deps, list) or not deps:
        memo[cap_id] = 0
        return 0

    depth = 1 + max(dependency_depth(cap_map, str(dep), memo) for dep in deps)
    memo[cap_id] = depth
    return depth


def build_capability_matrix(
    *,
    capability_registry: dict[str, object],
    protocol_matrix: dict[str, object],
    runtime_coverage: dict[str, object],
) -> dict[str, object]:
    caps = normalize_capabilities(capability_registry)
    cap_map = {str(cap.get("id")): cap for cap in caps}

    status_counts = Counter(str(cap.get("status", "unknown")) for cap in caps)
    required_caps = [cap for cap in caps if bool(cap.get("required_for_final", False))]
    required_done = [cap for cap in required_caps if str(cap.get("status")) == "done"]
    required_pending = [cap for cap in required_caps if str(cap.get("status")) != "done"]

    depth_memo: dict[str, int] = {}
    required_pending_sorted = sorted(
        required_pending,
        key=lambda cap: (
            -dependency_depth(cap_map, str(cap.get("id")), depth_memo),
            str(cap.get("id")),
        ),
    )

    gates_payload = capability_registry.get("gates", [])
    gates = [row for row in gates_payload if isinstance(row, dict)] if isinstance(gates_payload, list) else []
    final_gates: list[dict[str, object]] = []
    for gate in gates:
        linked = gate.get("linked_capabilities", [])
        linked_ids = [str(item) for item in linked] if isinstance(linked, list) else []
        linked_caps = [cap_map[item] for item in linked_ids if item in cap_map]
        gate_pass = bool(linked_caps) and all(
            str(cap.get("status")) == "done" and not list(cap.get("blockers") or [])
            for cap in linked_caps
        )
        blockers: list[str] = []
        for cap in linked_caps:
            if str(cap.get("status")) != "done":
                blockers.append(f"{cap.get('id')} status={cap.get('status')}")
            for blocker in cap.get("blockers", []) or []:
                blockers.append(f"{cap.get('id')}: {blocker}")
        final_gates.append(
            {
                "id": gate.get("id"),
                "title": gate.get("title"),
                "required": gate.get("required"),
                "status": "pass" if gate_pass else "blocked",
                "linked_capabilities": linked_ids,
                "blockers": blockers,
            }
        )

    protocol_snapshot = protocol_matrix.get("snapshot", {})
    runtime_summary = runtime_coverage.get("summary", {})

    return {
        "generated_unix_secs": int(time.time()),
        "summary": {
            "total_capabilities": len(caps),
            "done": status_counts.get("done", 0),
            "in_progress": status_counts.get("in_progress", 0),
            "planned": status_counts.get("planned", 0),
            "required_for_final": len(required_caps),
            "required_done": len(required_done),
            "required_pending": len(required_pending),
            "protocol_messages": int(protocol_snapshot.get("total_messages", 0) or 0),
            "protocol_implemented_mapped": int(protocol_snapshot.get("implemented_mapped", 0) or 0),
            "runtime_verified": int(runtime_summary.get("verified_runtime", 0) or 0),
            "runtime_static": int(runtime_summary.get("verified_static", 0) or 0),
            "semantic_tail_gaps": int(
                runtime_summary.get("messages_with_unresolved_semantic_tail", 0) or 0
            ),
        },
        "final_gates": final_gates,
        "critical_path": [
            {
                "id": cap.get("id"),
                "title": cap.get("title"),
                "status": cap.get("status"),
                "depends_on": cap.get("depends_on", []),
                "blockers": cap.get("blockers", []),
            }
            for cap in required_pending_sorted
        ],
        "capabilities": caps,
        "source_artifacts": {
            "capability_registry": "analysis/state/capability_registry.json",
            "protocol_matrix_json": "docs/state/protocol-matrix.json",
            "runtime_coverage_json": "docs/state/runtime-coverage.json",
        },
    }


def render_markdown(payload: dict[str, object]) -> str:
    summary = payload.get("summary", {})
    capabilities = payload.get("capabilities", [])
    final_gates = payload.get("final_gates", [])
    critical_path = payload.get("critical_path", [])

    lines = [
        "# Capability Matrix",
        "",
        "This matrix tracks delivery capabilities, final gates, and critical-path blockers.",
        "",
        "## Snapshot",
        "",
        f"- Total capabilities: `{summary.get('total_capabilities', 0)}`",
        f"- Done: `{summary.get('done', 0)}`",
        f"- In progress: `{summary.get('in_progress', 0)}`",
        f"- Planned: `{summary.get('planned', 0)}`",
        f"- Required for final: `{summary.get('required_for_final', 0)}`",
        f"- Required done: `{summary.get('required_done', 0)}`",
        f"- Required pending: `{summary.get('required_pending', 0)}`",
        f"- Runtime verified/static: `{summary.get('runtime_verified', 0)}/{summary.get('runtime_static', 0)}`",
        f"- Semantic-tail gaps: `{summary.get('semantic_tail_gaps', 0)}`",
        "",
        "## Final Gates",
        "",
        "| gate | status | required | blockers |",
        "|---|---|---|---|",
    ]

    if isinstance(final_gates, list) and final_gates:
        for gate in final_gates:
            if not isinstance(gate, dict):
                continue
            blockers = gate.get("blockers", [])
            blocker_text = "; ".join(str(item) for item in blockers) if blockers else "-"
            lines.append(
                "| `{id}` | `{status}` | {required} | {blockers} |".format(
                    id=gate.get("id", ""),
                    status=gate.get("status", ""),
                    required=str(gate.get("required", "")).replace("|", "\\|"),
                    blockers=blocker_text.replace("|", "\\|"),
                )
            )
    else:
        lines.append("| - | - | - | - |")

    lines.extend(["", "## Capability Table", "", "| id | title | domain | status | required final | depends_on | blockers | evidence |", "|---|---|---|---|---|---|---|---|"])

    if isinstance(capabilities, list) and capabilities:
        for cap in capabilities:
            if not isinstance(cap, dict):
                continue
            depends = ", ".join(str(item) for item in cap.get("depends_on", []) or [])
            blockers = "; ".join(str(item) for item in cap.get("blockers", []) or [])
            evidence = ", ".join(str(item) for item in cap.get("evidence", []) or [])
            lines.append(
                "| `{id}` | {title} | `{domain}` | `{status}` | {required} | {depends} | {blockers} | {evidence} |".format(
                    id=cap.get("id", ""),
                    title=str(cap.get("title", "")).replace("|", "\\|"),
                    domain=cap.get("domain", ""),
                    status=cap.get("status", ""),
                    required="yes" if cap.get("required_for_final") else "no",
                    depends=depends.replace("|", "\\|") or "-",
                    blockers=blockers.replace("|", "\\|") or "-",
                    evidence=evidence.replace("|", "\\|") or "-",
                )
            )
    else:
        lines.append("| - | - | - | - | - | - | - | - |")

    lines.extend(["", "## Critical Path", ""])
    if isinstance(critical_path, list) and critical_path:
        for item in critical_path:
            if not isinstance(item, dict):
                continue
            blockers = ", ".join(str(v) for v in item.get("blockers", []) or [])
            deps = ", ".join(str(v) for v in item.get("depends_on", []) or [])
            lines.append(
                f"- `{item.get('id')}` `{item.get('status')}` deps=`{deps or '-'}' blockers=`{blockers or '-'}`"
            )
    else:
        lines.append("No pending required capabilities.")

    lines.extend(
        [
            "",
            "## Regeneration",
            "",
            "```bash",
            "python3 tools/state/generate_capability_matrix.py",
            "```",
        ]
    )

    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate capability matrix JSON/Markdown artifacts")
    parser.add_argument("--capability-registry", default="analysis/state/capability_registry.json")
    parser.add_argument("--protocol-matrix-json", default="docs/state/protocol-matrix.json")
    parser.add_argument("--runtime-coverage-json", default="docs/state/runtime-coverage.json")
    parser.add_argument("--out-json", default="docs/state/capability-matrix.json")
    parser.add_argument("--out-md", default="docs/state/capability-matrix.md")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parents[2]
    capability_registry = read_json((repo_root / args.capability_registry).resolve())
    protocol_matrix = read_json((repo_root / args.protocol_matrix_json).resolve())
    runtime_coverage = read_json((repo_root / args.runtime_coverage_json).resolve())

    payload = build_capability_matrix(
        capability_registry=capability_registry,
        protocol_matrix=protocol_matrix,
        runtime_coverage=runtime_coverage,
    )

    out_json = (repo_root / args.out_json).resolve()
    out_md = (repo_root / args.out_md).resolve()
    out_json.parent.mkdir(parents=True, exist_ok=True)
    out_md.parent.mkdir(parents=True, exist_ok=True)

    out_json.write_text(json.dumps(payload, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")
    out_md.write_text(render_markdown(payload), encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
