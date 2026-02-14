#!/usr/bin/env python3
from __future__ import annotations

import argparse
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
if str(REPO_ROOT) not in sys.path:
    sys.path.insert(0, str(REPO_ROOT))

from tools.kb.docs_sync import sync_docs


def main() -> int:
    parser = argparse.ArgumentParser(description="Sync markdown docs from authoritative KB maps")
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--name-map", default="analysis/ghidra/maps/name_map.json")
    parser.add_argument("--data-map", default="analysis/ghidra/maps/data_map.json")
    parser.add_argument("--review-queue", default="analysis/ghidra/queue/review_queue.jsonl")
    parser.add_argument("--message-map", default="analysis/ghidra/maps/message_map.csv")
    parser.add_argument("--detangling-doc", default="docs/re/static/detangling.md")
    parser.add_argument("--ledger-doc", default="docs/verification/evidence-ledger.md")
    args = parser.parse_args()

    repo_root = Path(args.repo_root).resolve()
    sync_docs(
        name_map_path=repo_root / args.name_map,
        data_map_path=repo_root / args.data_map,
        review_queue_path=repo_root / args.review_queue,
        message_map_path=repo_root / args.message_map,
        detangling_doc_path=repo_root / args.detangling_doc,
        ledger_doc_path=repo_root / args.ledger_doc,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
