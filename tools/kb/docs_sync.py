from __future__ import annotations

import json
from pathlib import Path
from typing import Any


def _load_json(path: Path) -> dict[str, Any]:
    if not path.exists():
        return {"entries": []}
    return json.loads(path.read_text(encoding="utf-8"))


def _read_jsonl(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    rows: list[dict[str, Any]] = []
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if line:
            rows.append(json.loads(line))
    return rows


def _link(source: str) -> str:
    if source.startswith("http://") or source.startswith("https://"):
        return f"[{source}]({source})"
    return f"`{source}`"


def sync_docs(
    *,
    name_map_path: Path,
    data_map_path: Path,
    review_queue_path: Path,
    detangling_doc_path: Path,
    ledger_doc_path: Path,
) -> None:
    name_entries = _load_json(name_map_path).get("entries", [])
    data_entries = _load_json(data_map_path).get("entries", [])
    review_entries = _read_jsonl(review_queue_path)

    detangling_lines: list[str] = []
    detangling_lines.append("# Detangling Notes")
    detangling_lines.append("")
    detangling_lines.append("This page tracks approved mappings and pending review candidates for SoulseekQt reverse engineering.")
    detangling_lines.append("")
    detangling_lines.append("## Approved Function Renames")
    detangling_lines.append("")

    if not name_entries:
        detangling_lines.append("No approved function renames yet.")
    else:
        for entry in name_entries:
            detangling_lines.append(f"### `{entry['original_name']}` -> `{entry['new_name']}`")
            detangling_lines.append(f"- Binary: `{entry['binary']}`")
            detangling_lines.append(f"- Address: `{entry['address']}`")
            detangling_lines.append(f"- Confidence: `{entry['confidence']}`")
            detangling_lines.append(f"- Status: `{entry['status']}`")
            detangling_lines.append("- Evidence:")
            for ev in entry.get("evidence", []):
                source = str(ev.get("source", ""))
                kind = str(ev.get("kind", ""))
                note = str(ev.get("note", "")).strip()
                bullet = f"  - `{kind}`: {_link(source)}"
                if note:
                    bullet += f" ({note})"
                detangling_lines.append(bullet)
            detangling_lines.append("")

    detangling_lines.append("## Approved Data Labels")
    detangling_lines.append("")
    if not data_entries:
        detangling_lines.append("No approved data labels yet.")
    else:
        for entry in data_entries:
            detangling_lines.append(f"- `{entry['address']}` -> `{entry['new_label']}` ({entry['data_type']}, confidence `{entry['confidence']}`)")
    detangling_lines.append("")

    detangling_lines.append("## Review Queue")
    detangling_lines.append("")
    if not review_entries:
        detangling_lines.append("Review queue is empty.")
    else:
        for row in review_entries[-50:]:
            status = row.get("status", "unknown")
            reason = row.get("reason", "")
            candidate = row.get("candidate", {})
            name = candidate.get("new_name") or candidate.get("new_label") or "unnamed"
            detangling_lines.append(f"- `{status}` `{name}` reason: `{reason}`")

    detangling_doc_path.parent.mkdir(parents=True, exist_ok=True)
    detangling_doc_path.write_text("\n".join(detangling_lines) + "\n", encoding="utf-8")

    ledger_lines: list[str] = []
    ledger_lines.append("# Evidence Ledger")
    ledger_lines.append("")
    ledger_lines.append("Project-level evidence summaries and provenance tracking.")
    ledger_lines.append("")
    ledger_lines.append("## Totals")
    ledger_lines.append("")
    ledger_lines.append(f"- Approved function renames: `{len(name_entries)}`")
    ledger_lines.append(f"- Approved data labels: `{len(data_entries)}`")
    ledger_lines.append(f"- Review queue entries: `{len(review_entries)}`")
    ledger_lines.append("")

    ledger_lines.append("## Latest Evidence Sources")
    ledger_lines.append("")
    latest_sources: list[str] = []
    for entry in (name_entries + data_entries)[-20:]:
        for ev in entry.get("evidence", []):
            src = str(ev.get("source", "")).strip()
            if src:
                latest_sources.append(src)

    if not latest_sources:
        ledger_lines.append("No evidence sources registered yet.")
    else:
        deduped: list[str] = []
        for src in latest_sources:
            if src not in deduped:
                deduped.append(src)
        for src in deduped[:30]:
            ledger_lines.append(f"- {_link(src)}")

    ledger_doc_path.parent.mkdir(parents=True, exist_ok=True)
    ledger_doc_path.write_text("\n".join(ledger_lines) + "\n", encoding="utf-8")
