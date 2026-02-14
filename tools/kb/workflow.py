from __future__ import annotations

import json
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any
from urllib.parse import urlparse


@dataclass
class PromoteResult:
    promoted: int = 0
    review_required: int = 0
    rejected: int = 0


def now_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def _load_json(path: Path) -> dict[str, Any]:
    if not path.exists():
        return {"version": 1, "generated_by": "NeoSoulSeek kb workflow", "entries": []}
    return json.loads(path.read_text(encoding="utf-8"))


def _write_json(path: Path, obj: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(obj, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")


def _read_jsonl(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    rows: list[dict[str, Any]] = []
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line:
            continue
        rows.append(json.loads(line))
    return rows


def _append_jsonl(path: Path, rows: list[dict[str, Any]]) -> None:
    if not rows:
        return
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("a", encoding="utf-8") as fh:
        for row in rows:
            fh.write(json.dumps(row, ensure_ascii=True) + "\n")


def _truncate_file(path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("", encoding="utf-8")


def _is_url(value: str) -> bool:
    parsed = urlparse(value)
    return parsed.scheme in {"http", "https"} and bool(parsed.netloc)


def _evidence_errors(evidence: list[dict[str, Any]], repo_root: Path) -> list[str]:
    errors: list[str] = []
    if not evidence:
        return ["missing_evidence"]

    for index, item in enumerate(evidence):
        source = str(item.get("source", "")).strip()
        kind = str(item.get("kind", "")).strip()
        if not kind:
            errors.append(f"evidence[{index}].missing_kind")
        if not source:
            errors.append(f"evidence[{index}].missing_source")
            continue
        if _is_url(source):
            continue

        source_path = Path(source)
        if not source_path.is_absolute():
            source_path = repo_root / source_path
        if not source_path.exists():
            errors.append(f"evidence[{index}].source_not_found:{source}")
    return errors


def _required_fields(candidate: dict[str, Any], entry_type: str) -> list[str]:
    common = ["binary", "address", "confidence", "evidence"]
    if entry_type == "name":
        required = common + ["original_name", "new_name", "kind"]
    else:
        required = common + ["new_label", "data_type"]

    missing: list[str] = []
    for field in required:
        value = candidate.get(field)
        if value in (None, "", []):
            missing.append(field)
    return missing


def _entry_key(entry: dict[str, Any], entry_type: str) -> str:
    if entry_type == "name":
        return f"{entry.get('binary')}::{entry.get('address')}::{entry.get('kind')}"
    return f"{entry.get('binary')}::{entry.get('address')}::{entry.get('new_label')}"


def _promote_single(
    candidate: dict[str, Any],
    entry_type: str,
    entries_by_key: dict[str, dict[str, Any]],
    review_rows: list[dict[str, Any]],
    repo_root: Path,
) -> str:
    missing = _required_fields(candidate, entry_type)
    ev_errors = _evidence_errors(candidate.get("evidence", []), repo_root)

    if missing or ev_errors:
        review_rows.append(
            {
                "time": now_iso(),
                "entry_type": entry_type,
                "status": "rejected",
                "reason": "validation_failed",
                "missing": missing,
                "evidence_errors": ev_errors,
                "candidate": candidate,
            }
        )
        return "rejected"

    confidence = str(candidate.get("confidence", "")).lower()
    if confidence != "high":
        review_rows.append(
            {
                "time": now_iso(),
                "entry_type": entry_type,
                "status": "review_required",
                "reason": "confidence_below_high",
                "candidate": candidate,
            }
        )
        return "review_required"

    promoted = dict(candidate)
    promoted["status"] = "approved"
    provenance = dict(promoted.get("provenance", {}))
    provenance.setdefault("analyst", "codex")
    provenance.setdefault("created_at", now_iso())
    provenance["promoted_at"] = now_iso()
    promoted["provenance"] = provenance

    entries_by_key[_entry_key(promoted, entry_type)] = promoted
    return "promoted"


def promote_candidates(
    *,
    repo_root: Path,
    name_map_path: Path,
    data_map_path: Path,
    name_candidates_path: Path,
    data_candidates_path: Path,
    review_queue_path: Path,
) -> dict[str, PromoteResult]:
    name_map = _load_json(name_map_path)
    data_map = _load_json(data_map_path)

    name_entries = { _entry_key(e, "name"): e for e in name_map.get("entries", []) }
    data_entries = { _entry_key(e, "data"): e for e in data_map.get("entries", []) }

    review_rows: list[dict[str, Any]] = []
    name_result = PromoteResult()
    data_result = PromoteResult()

    name_candidates = _read_jsonl(name_candidates_path)
    data_candidates = _read_jsonl(data_candidates_path)

    for candidate in name_candidates:
        outcome = _promote_single(candidate, "name", name_entries, review_rows, repo_root)
        if outcome == "promoted":
            name_result.promoted += 1
        elif outcome == "review_required":
            name_result.review_required += 1
        else:
            name_result.rejected += 1

    for candidate in data_candidates:
        outcome = _promote_single(candidate, "data", data_entries, review_rows, repo_root)
        if outcome == "promoted":
            data_result.promoted += 1
        elif outcome == "review_required":
            data_result.review_required += 1
        else:
            data_result.rejected += 1

    name_map["entries"] = sorted(name_entries.values(), key=lambda e: (str(e.get("binary")), str(e.get("address")), str(e.get("new_name"))))
    data_map["entries"] = sorted(data_entries.values(), key=lambda e: (str(e.get("binary")), str(e.get("address")), str(e.get("new_label"))))

    _write_json(name_map_path, name_map)
    _write_json(data_map_path, data_map)
    _append_jsonl(review_queue_path, review_rows)
    _truncate_file(name_candidates_path)
    _truncate_file(data_candidates_path)

    return {"name": name_result, "data": data_result}
