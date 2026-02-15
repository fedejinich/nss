#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path
from typing import Any

ABS_WIN_RE = re.compile(r"^[A-Za-z]:\\")


def is_absolute_path_string(value: str) -> bool:
    return value.startswith("/") or bool(ABS_WIN_RE.match(value))


def to_path_ref(value: str, *, repo_root: Path) -> str:
    candidate = Path(value)
    if not is_absolute_path_string(value):
        return value

    resolved = candidate.resolve()
    try:
        return resolved.relative_to(repo_root.resolve()).as_posix()
    except ValueError:
        digest = hashlib.sha256(str(resolved).encode("utf-8")).hexdigest()[:12]
        return f"<external:path:{digest}>"


def is_path_like_key(key: str) -> bool:
    lowered = key.lower()
    return any(
        token in lowered
        for token in ("path", "source", "dir", "manifest", "events", "frames", "artifact")
    )


def normalize_obj(obj: Any, *, repo_root: Path, key: str = "") -> tuple[Any, int]:
    changes = 0
    if isinstance(obj, dict):
        normalized: dict[str, Any] = {}
        for child_key, child_value in obj.items():
            next_key = str(child_key)
            child_norm, child_changes = normalize_obj(
                child_value, repo_root=repo_root, key=next_key
            )
            normalized[next_key] = child_norm
            changes += child_changes
        return normalized, changes

    if isinstance(obj, list):
        normalized_list = []
        for item in obj:
            item_norm, item_changes = normalize_obj(item, repo_root=repo_root, key=key)
            normalized_list.append(item_norm)
            changes += item_changes
        return normalized_list, changes

    if isinstance(obj, str):
        if is_path_like_key(key) or obj.startswith(str(repo_root.resolve())):
            normalized = to_path_ref(obj, repo_root=repo_root)
            if normalized != obj:
                return normalized, 1
        return obj, 0

    return obj, 0


def normalize_file(path: Path, *, repo_root: Path) -> int:
    payload = json.loads(path.read_text(encoding="utf-8"))
    normalized, changes = normalize_obj(payload, repo_root=repo_root)
    if changes > 0:
        path.write_text(json.dumps(normalized, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")
    return changes


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Sanitize path-like metadata fields in captures/redacted manifests/summaries"
    )
    parser.add_argument("--root", default="captures/redacted")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parents[2]
    root = (repo_root / args.root).resolve()
    if not root.exists():
        raise SystemExit(f"redacted root not found: {root}")

    run_dirs = [p for p in sorted(root.iterdir()) if p.is_dir()]
    changed_files = 0
    changed_fields = 0

    for run_dir in run_dirs:
        for file_name in ("manifest.redacted.json", "redaction-summary.json"):
            file_path = run_dir / file_name
            if not file_path.exists():
                continue
            changes = normalize_file(file_path, repo_root=repo_root)
            if changes > 0:
                changed_files += 1
                changed_fields += changes

    print(
        json.dumps(
            {
                "runs_scanned": len(run_dirs),
                "changed_files": changed_files,
                "changed_fields": changed_fields,
            },
            ensure_ascii=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
