from __future__ import annotations

import csv
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

REQUIRED_MESSAGE_FIELDS = [
    "scope",
    "code",
    "name",
    "confidence",
    "source",
    "status",
]

REQUIRED_SCHEMA_FIELDS = [
    "scope",
    "code",
    "name",
    "confidence",
    "evidence",
]

ALLOWED_CONFIDENCE = {"high", "medium", "low"}

CORE_MESSAGES = {
    ("server", "SM_LOGIN"),
    ("server", "SM_SET_WAIT_PORT"),
    ("server", "SM_GET_PEER_ADDRESS"),
    ("server", "SM_CONNECT_TO_PEER"),
    ("server", "SM_FILE_SEARCH"),
    ("server", "SM_SEARCH_ROOM"),
    ("server", "SM_EXACT_FILE_SEARCH"),
    ("server", "SM_SEARCH_USER_FILES"),
    ("server", "SM_MESSAGE_USER"),
    ("server", "SM_MESSAGE_ACKED"),
    ("server", "SM_GET_USER_STATS"),
    ("server", "SM_GET_USER_STATUS"),
    ("server", "SM_SHARED_FOLDERS_FILES"),
    ("server", "SM_DOWNLOAD_SPEED"),
    ("server", "SM_UPLOAD_SPEED"),
    ("peer", "PM_GET_SHARED_FILE_LIST"),
    ("peer", "PM_SHARED_FILE_LIST"),
    ("peer", "PM_FILE_SEARCH_REQUEST"),
    ("peer", "PM_FILE_SEARCH_RESULT"),
    ("peer", "PM_TRANSFER_REQUEST"),
    ("peer", "PM_TRANSFER_RESPONSE"),
    ("peer", "PM_QUEUE_UPLOAD"),
    ("peer", "PM_UPLOAD_PLACE_IN_LINE"),
    ("peer", "PM_UPLOAD_FAILED"),
    ("peer", "PM_UPLOAD_DENIED"),
}


def _is_url(value: str) -> bool:
    parsed = urlparse(value)
    return parsed.scheme in {"http", "https"} and bool(parsed.netloc)


def _load(path: Path) -> dict[str, Any]:
    if not path.exists():
        return {"entries": []}
    return json.loads(path.read_text(encoding="utf-8"))


def _validate_source(source: str, repo_root: Path) -> list[str]:
    errors: list[str] = []
    if not source:
        errors.append("missing_source")
        return errors

    if _is_url(source):
        return errors

    path = Path(source)
    if not path.is_absolute():
        path = repo_root / path
    if not path.exists():
        errors.append(f"source_not_found:{source}")

    return errors


def _validate_evidence(evidence: Any, repo_root: Path) -> list[str]:
    errors: list[str] = []

    if not isinstance(evidence, list) or not evidence:
        errors.append("missing_evidence")
        return errors

    for idx, ev in enumerate(evidence):
        if not isinstance(ev, dict):
            errors.append(f"evidence[{idx}].invalid_type")
            continue

        kind = str(ev.get("kind", "")).strip()
        if not kind:
            errors.append(f"evidence[{idx}].missing_kind")

        source = str(ev.get("source", "")).strip()
        for source_error in _validate_source(source, repo_root):
            errors.append(f"evidence[{idx}].{source_error}")

    return errors


def _validate_entry(entry: dict[str, Any], required_fields: list[str], repo_root: Path) -> list[str]:
    errors: list[str] = []
    for field in required_fields:
        if entry.get(field) in (None, "", []):
            errors.append(f"missing_field:{field}")

    confidence = str(entry.get("confidence", "")).strip().lower()
    if confidence and confidence not in ALLOWED_CONFIDENCE:
        errors.append(f"invalid_confidence:{confidence}")

    errors.extend(_validate_evidence(entry.get("evidence", []), repo_root))
    return errors


def _validate_message_map(repo_root: Path, message_map_path: Path) -> list[str]:
    errors: list[str] = []
    if not message_map_path.exists():
        return [f"missing_file:{message_map_path}"]

    with message_map_path.open("r", encoding="utf-8") as fh:
        rows = list(csv.DictReader(fh))

    seen: set[tuple[str, str]] = set()
    core_confidence = {"high": 0, "medium": 0, "low": 0}

    for idx, row in enumerate(rows):
        for field in REQUIRED_MESSAGE_FIELDS:
            if (row.get(field) or "").strip() == "":
                errors.append(f"entry[{idx}]:missing_field:{field}")

        scope = (row.get("scope") or "").strip()
        name = (row.get("name") or "").strip()
        key = (scope, name)
        if scope and name:
            if key in seen:
                errors.append(f"entry[{idx}]:duplicate:{scope}:{name}")
            seen.add(key)

        code_raw = (row.get("code") or "").strip()
        try:
            int(code_raw)
        except ValueError:
            errors.append(f"entry[{idx}]:invalid_code:{code_raw}")

        confidence = (row.get("confidence") or "").strip().lower()
        if confidence and confidence not in ALLOWED_CONFIDENCE:
            errors.append(f"entry[{idx}]:invalid_confidence:{confidence}")

        source = (row.get("source") or "").strip()
        for source_error in _validate_source(source, repo_root):
            errors.append(f"entry[{idx}]:{source_error}")

        if key in CORE_MESSAGES:
            if confidence in core_confidence:
                core_confidence[confidence] += 1
            if confidence == "low":
                errors.append(f"entry[{idx}]:low_confidence_not_allowed_for_core")

    missing_core = sorted(CORE_MESSAGES - seen)
    for scope, name in missing_core:
        errors.append(f"missing_core_message:{scope}:{name}")

    if len(seen & CORE_MESSAGES) != 25:
        errors.append(f"core_coverage_count_invalid:{len(seen & CORE_MESSAGES)}")

    if core_confidence["high"] < 18:
        errors.append(f"core_high_confidence_below_threshold:{core_confidence['high']}")
    if core_confidence["medium"] > 7:
        errors.append(f"core_medium_confidence_above_threshold:{core_confidence['medium']}")
    if core_confidence["low"] != 0:
        errors.append(f"core_low_confidence_must_be_zero:{core_confidence['low']}")

    return errors


def _validate_message_schema(repo_root: Path, message_schema_path: Path) -> list[str]:
    errors: list[str] = []
    if not message_schema_path.exists():
        return [f"missing_file:{message_schema_path}"]

    payload = json.loads(message_schema_path.read_text(encoding="utf-8"))
    messages = payload.get("messages", [])
    if not isinstance(messages, list):
        return ["invalid_schema:messages_not_list"]

    seen: set[tuple[str, str]] = set()
    core_confidence = {"high": 0, "medium": 0, "low": 0}

    for idx, entry in enumerate(messages):
        if not isinstance(entry, dict):
            errors.append(f"entry[{idx}]:invalid_type")
            continue

        for field in REQUIRED_SCHEMA_FIELDS:
            if entry.get(field) in (None, "", []):
                errors.append(f"entry[{idx}]:missing_field:{field}")

        scope = str(entry.get("scope", "")).strip()
        name = str(entry.get("name", "")).strip()
        key = (scope, name)

        if scope and name:
            if key in seen:
                errors.append(f"entry[{idx}]:duplicate:{scope}:{name}")
            seen.add(key)

        confidence = str(entry.get("confidence", "")).strip().lower()
        if confidence and confidence not in ALLOWED_CONFIDENCE:
            errors.append(f"entry[{idx}]:invalid_confidence:{confidence}")

        for ev_error in _validate_evidence(entry.get("evidence", []), repo_root):
            errors.append(f"entry[{idx}]:{ev_error}")

        if key in CORE_MESSAGES:
            if confidence in core_confidence:
                core_confidence[confidence] += 1
            if confidence == "low":
                errors.append(f"entry[{idx}]:low_confidence_not_allowed_for_core")

    missing_core = sorted(CORE_MESSAGES - seen)
    for scope, name in missing_core:
        errors.append(f"missing_core_message:{scope}:{name}")

    if len(seen & CORE_MESSAGES) != 25:
        errors.append(f"core_coverage_count_invalid:{len(seen & CORE_MESSAGES)}")

    if core_confidence["high"] < 18:
        errors.append(f"core_high_confidence_below_threshold:{core_confidence['high']}")
    if core_confidence["medium"] > 7:
        errors.append(f"core_medium_confidence_above_threshold:{core_confidence['medium']}")
    if core_confidence["low"] != 0:
        errors.append(f"core_low_confidence_must_be_zero:{core_confidence['low']}")

    return errors


def validate_maps(
    *,
    repo_root: Path,
    name_map_path: Path,
    data_map_path: Path,
    message_map_path: Path | None = None,
    message_schema_path: Path | None = None,
) -> dict[str, list[str]]:
    errors: dict[str, list[str]] = {"name_map": [], "data_map": [], "message_map": [], "message_schema": []}

    name_map = _load(name_map_path)
    for i, entry in enumerate(name_map.get("entries", [])):
        for err in _validate_entry(entry, REQUIRED_NAME_FIELDS, repo_root):
            errors["name_map"].append(f"entry[{i}]:{err}")

    data_map = _load(data_map_path)
    for i, entry in enumerate(data_map.get("entries", [])):
        for err in _validate_entry(entry, REQUIRED_DATA_FIELDS, repo_root):
            errors["data_map"].append(f"entry[{i}]:{err}")

    if message_map_path is not None:
        errors["message_map"] = _validate_message_map(repo_root, message_map_path)

    if message_schema_path is not None:
        errors["message_schema"] = _validate_message_schema(repo_root, message_schema_path)

    return errors
