#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.kb.workflow import promote_candidates


def main() -> int:
    parser = argparse.ArgumentParser(description="Promote high-confidence candidates into authoritative maps")
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--name-map", default="analysis/ghidra/maps/name_map.json")
    parser.add_argument("--data-map", default="analysis/ghidra/maps/data_map.json")
    parser.add_argument("--name-candidates", default="analysis/ghidra/queue/name_candidates.jsonl")
    parser.add_argument("--data-candidates", default="analysis/ghidra/queue/data_candidates.jsonl")
    parser.add_argument("--review-queue", default="analysis/ghidra/queue/review_queue.jsonl")
    args = parser.parse_args()

    repo_root = Path(args.repo_root).resolve()
    result = promote_candidates(
        repo_root=repo_root,
        name_map_path=(repo_root / args.name_map),
        data_map_path=(repo_root / args.data_map),
        name_candidates_path=(repo_root / args.name_candidates),
        data_candidates_path=(repo_root / args.data_candidates),
        review_queue_path=(repo_root / args.review_queue),
    )

    print(
        json.dumps(
            {
                "name": result["name"].__dict__,
                "data": result["data"].__dict__,
            },
            indent=2,
            ensure_ascii=True,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
