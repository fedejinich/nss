#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.kb.validate import validate_maps


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate KB maps for required evidence and fields")
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--name-map", default="analysis/ghidra/maps/name_map.json")
    parser.add_argument("--data-map", default="analysis/ghidra/maps/data_map.json")
    args = parser.parse_args()

    repo_root = Path(args.repo_root).resolve()
    result = validate_maps(
        repo_root=repo_root,
        name_map_path=repo_root / args.name_map,
        data_map_path=repo_root / args.data_map,
    )

    print(json.dumps(result, indent=2, ensure_ascii=True))
    has_errors = any(result.values())
    return 1 if has_errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
