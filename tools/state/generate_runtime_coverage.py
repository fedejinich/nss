#!/usr/bin/env python3
from __future__ import annotations

import argparse
import csv
import json
import time
from collections import Counter
from pathlib import Path


def read_json(path: Path) -> dict[str, object]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_message_map(path: Path) -> list[dict[str, str]]:
    with path.open("r", encoding="utf-8") as fh:
        return list(csv.DictReader(fh))


def read_schema_messages(path: Path) -> list[dict[str, object]]:
    payload = read_json(path)
    if isinstance(payload, dict):
        messages = payload.get("messages", [])
        if isinstance(messages, list):
            return [row for row in messages if isinstance(row, dict)]
    return []


def build_runtime_coverage(
    *,
    message_rows: list[dict[str, str]],
    schema_messages: list[dict[str, object]],
    registry: dict[str, object],
) -> dict[str, object]:
    status_counts = Counter((row.get("status") or "unknown").strip() for row in message_rows)
    total_messages = len(message_rows)
    verified_runtime = status_counts.get("verified_runtime", 0)
    verified_static = status_counts.get("verified_static", 0)

    static_rows = sorted(
        [row for row in message_rows if (row.get("status") or "").strip() == "verified_static"],
        key=lambda row: ((row.get("scope") or ""), int(row.get("code") or 0), row.get("name") or ""),
    )

    unresolved_tail_entries: list[dict[str, object]] = []
    for message in schema_messages:
        fields = message.get("payload", [])
        if not isinstance(fields, list):
            continue
        unresolved_fields = []
        for field in fields:
            if not isinstance(field, dict):
                continue
            name = str(field.get("name") or "")
            if name in {"raw_tail", "raw_payload"}:
                unresolved_fields.append(name)
        if unresolved_fields:
            unresolved_tail_entries.append(
                {
                    "scope": message.get("scope"),
                    "code": message.get("code"),
                    "name": message.get("name"),
                    "unresolved_fields": unresolved_fields,
                }
            )

    registry_targets = registry.get("runtime_targets", [])
    target_rows: list[dict[str, object]] = []
    if isinstance(registry_targets, list):
        lookup = {
            (row.get("scope"), str(row.get("code")), row.get("name")): row
            for row in message_rows
        }
        for target in registry_targets:
            if not isinstance(target, dict):
                continue
            key = (
                target.get("scope"),
                str(target.get("code")),
                target.get("name"),
            )
            row = lookup.get(key)
            target_rows.append(
                {
                    "scope": target.get("scope"),
                    "code": target.get("code"),
                    "name": target.get("name"),
                    "scenario": target.get("scenario"),
                    "current_status": (row or {}).get("status", "missing"),
                    "source": (row or {}).get("source", ""),
                }
            )

    return {
        "generated_unix_secs": int(time.time()),
        "policy": registry.get("policy", {}),
        "summary": {
            "total_messages": total_messages,
            "verified_runtime": verified_runtime,
            "verified_static": verified_static,
            "runtime_coverage_percent": round((verified_runtime / total_messages) * 100, 2)
            if total_messages
            else 0.0,
            "runtime_gap": verified_static,
            "unresolved_semantic_tail_fields": sum(
                len(row["unresolved_fields"]) for row in unresolved_tail_entries
            ),
            "messages_with_unresolved_semantic_tail": len(unresolved_tail_entries),
        },
        "targets": {
            "runtime_targets": target_rows,
            "semantic_tail_targets": registry.get("semantic_tail_targets", []),
        },
        "gaps": {
            "static_only_messages": static_rows,
            "unresolved_tail_messages": unresolved_tail_entries,
        },
        "source_artifacts": {
            "message_map": "analysis/ghidra/maps/message_map.csv",
            "message_schema": "analysis/protocol/message_schema.json",
            "runtime_coverage_registry": "analysis/state/runtime_coverage_registry.json",
        },
    }


def render_runtime_coverage_markdown(payload: dict[str, object]) -> str:
    summary = payload.get("summary", {})
    gaps = payload.get("gaps", {})
    static_rows = gaps.get("static_only_messages", []) if isinstance(gaps, dict) else []
    unresolved_rows = gaps.get("unresolved_tail_messages", []) if isinstance(gaps, dict) else []
    targets = payload.get("targets", {})
    runtime_targets = targets.get("runtime_targets", []) if isinstance(targets, dict) else []

    lines = [
        "# Runtime Coverage",
        "",
        "This page tracks runtime-vs-static evidence closure and semantic-tail closure status.",
        "",
        "## Snapshot",
        "",
        f"- Total messages: `{summary.get('total_messages', 0)}`",
        f"- Verified runtime: `{summary.get('verified_runtime', 0)}`",
        f"- Verified static: `{summary.get('verified_static', 0)}`",
        f"- Runtime coverage: `{summary.get('runtime_coverage_percent', 0)}%`",
        f"- Runtime gap: `{summary.get('runtime_gap', 0)}`",
        f"- Messages with unresolved semantic tail fields: `{summary.get('messages_with_unresolved_semantic_tail', 0)}`",
        "",
        "## Runtime Target Progress",
        "",
        "| scope | code | message | scenario | current status | source |",
        "|---|---:|---|---|---|---|",
    ]

    if isinstance(runtime_targets, list) and runtime_targets:
        for row in runtime_targets:
            if not isinstance(row, dict):
                continue
            lines.append(
                "| {scope} | {code} | `{name}` | `{scenario}` | `{current_status}` | `{source}` |".format(
                    scope=row.get("scope", ""),
                    code=row.get("code", ""),
                    name=row.get("name", ""),
                    scenario=row.get("scenario", ""),
                    current_status=row.get("current_status", ""),
                    source=row.get("source", ""),
                )
            )
    else:
        lines.append("| - | - | - | - | - | - |")

    lines.extend(["", "## Static-Only Messages", ""])
    if isinstance(static_rows, list) and static_rows:
        lines.append("| scope | code | message | confidence | source |")
        lines.append("|---|---:|---|---|---|")
        for row in static_rows:
            if not isinstance(row, dict):
                continue
            lines.append(
                "| {scope} | {code} | `{name}` | {confidence} | `{source}` |".format(
                    scope=row.get("scope", ""),
                    code=row.get("code", ""),
                    name=row.get("name", ""),
                    confidence=row.get("confidence", ""),
                    source=row.get("source", ""),
                )
            )
    else:
        lines.append("No static-only messages remain.")

    lines.extend(["", "## Unresolved Semantic Tail Fields", ""])
    if isinstance(unresolved_rows, list) and unresolved_rows:
        lines.append("| scope | code | message | unresolved fields |")
        lines.append("|---|---:|---|---|")
        for row in unresolved_rows:
            if not isinstance(row, dict):
                continue
            unresolved = ", ".join(str(v) for v in row.get("unresolved_fields", []))
            lines.append(
                "| {scope} | {code} | `{name}` | `{fields}` |".format(
                    scope=row.get("scope", ""),
                    code=row.get("code", ""),
                    name=row.get("name", ""),
                    fields=unresolved,
                )
            )
    else:
        lines.append("No unresolved semantic tail fields remain.")

    lines.extend(
        [
            "",
            "## Regeneration",
            "",
            "```bash",
            "python3 tools/state/generate_runtime_coverage.py",
            "```",
        ]
    )
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate runtime coverage JSON/Markdown artifacts")
    parser.add_argument("--message-map", default="analysis/ghidra/maps/message_map.csv")
    parser.add_argument("--message-schema", default="analysis/protocol/message_schema.json")
    parser.add_argument(
        "--runtime-registry", default="analysis/state/runtime_coverage_registry.json"
    )
    parser.add_argument("--out-json", default="docs/state/runtime-coverage.json")
    parser.add_argument("--out-md", default="docs/state/runtime-coverage.md")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parents[2]
    message_rows = read_message_map((repo_root / args.message_map).resolve())
    schema_messages = read_schema_messages((repo_root / args.message_schema).resolve())
    registry = read_json((repo_root / args.runtime_registry).resolve())

    payload = build_runtime_coverage(
        message_rows=message_rows,
        schema_messages=schema_messages,
        registry=registry,
    )

    out_json = (repo_root / args.out_json).resolve()
    out_md = (repo_root / args.out_md).resolve()
    out_json.parent.mkdir(parents=True, exist_ok=True)
    out_md.parent.mkdir(parents=True, exist_ok=True)

    out_json.write_text(json.dumps(payload, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")
    out_md.write_text(render_runtime_coverage_markdown(payload), encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
