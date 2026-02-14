from __future__ import annotations

import json
from pathlib import Path
from typing import Any
from urllib.parse import urlparse


REQUIRED_NAME_FIELDS = [
    "binary",
    "address",
    "original_name",
    "new_name",
    "kind",
    "confidence",
    "evidence",
    "status",
]

REQUIRED_DATA_FIELDS = [
    "binary",
    "address",
    "new_label",
    "data_type",
    "confidence",
    "evidence",
    "status",
]


def _is_url(value: str) -> bool:
    parsed = urlparse(value)
    return parsed.scheme in {"http", "https"} and bool(parsed.netloc)


def _load(path: Path) -> dict[str, Any]:
    if not path.exists():
        return {"entries": []}
    return json.loads(path.read_text(encoding="utf-8"))


def _validate_entry(entry: dict[str, Any], required_fields: list[str], repo_root: Path) -> list[str]:
    errors: list[str] = []
    for field in required_fields:
        if entry.get(field) in (None, "", []):
            errors.append(f"missing_field:{field}")

    evidence = entry.get("evidence", [])
    if not isinstance(evidence, list) or not evidence:
        errors.append("missing_evidence")
        return errors

    for idx, ev in enumerate(evidence):
        source = str(ev.get("source", "")).strip()
        kind = str(ev.get("kind", "")).strip()
        if not kind:
            errors.append(f"evidence[{idx}].missing_kind")
        if not source:
            errors.append(f"evidence[{idx}].missing_source")
            continue
        if _is_url(source):
            continue

        path = Path(source)
        if not path.is_absolute():
            path = repo_root / path
        if not path.exists():
            errors.append(f"evidence[{idx}].source_not_found:{source}")

    return errors


def validate_maps(*, repo_root: Path, name_map_path: Path, data_map_path: Path) -> dict[str, list[str]]:
    errors: dict[str, list[str]] = {"name_map": [], "data_map": []}

    name_map = _load(name_map_path)
    for i, entry in enumerate(name_map.get("entries", [])):
        for err in _validate_entry(entry, REQUIRED_NAME_FIELDS, repo_root):
            errors["name_map"].append(f"entry[{i}]:{err}")

    data_map = _load(data_map_path)
    for i, entry in enumerate(data_map.get("entries", [])):
        for err in _validate_entry(entry, REQUIRED_DATA_FIELDS, repo_root):
            errors["data_map"].append(f"entry[{i}]:{err}")

    return errors
