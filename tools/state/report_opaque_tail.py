#!/usr/bin/env python3
from __future__ import annotations

import argparse
import csv
import json
import re
import time
from pathlib import Path
from typing import Any

CONST_RE = re.compile(r"pub const (CODE_(?:SM|PM)_[A-Z0-9_]+): u32 = (\d+);")
OPAQUE_ARRAY_RE = re.compile(r"pub const OPAQUE_SERVER_CONTROL_CODES: \[u32; \d+\] = \[(.*?)\];", re.S)
ARRAY_SYMBOL_RE = re.compile(r"(CODE_(?:SM|PM)_[A-Z0-9_]+)")


def parse_protocol_constants(protocol_lib: Path) -> dict[str, int]:
    source = protocol_lib.read_text(encoding="utf-8")
    return {name: int(code) for name, code in CONST_RE.findall(source)}


def parse_opaque_symbols(protocol_lib: Path) -> list[str]:
    source = protocol_lib.read_text(encoding="utf-8")
    match = OPAQUE_ARRAY_RE.search(source)
    if match is None:
        raise ValueError("OPAQUE_SERVER_CONTROL_CODES array not found")
    return ARRAY_SYMBOL_RE.findall(match.group(1))


def read_message_map(path: Path) -> dict[int, dict[str, str]]:
    rows: dict[int, dict[str, str]] = {}
    with path.open("r", encoding="utf-8") as fh:
        for row in csv.DictReader(fh):
            code = row.get("code", "")
            if code.isdigit():
                rows[int(code)] = row
    return rows


def build_report(repo_root: Path, protocol_lib: Path, message_map_path: Path) -> dict[str, Any]:
    constants = parse_protocol_constants(protocol_lib)
    opaque_symbols = parse_opaque_symbols(protocol_lib)
    mapped_rows = read_message_map(message_map_path)

    entries: list[dict[str, Any]] = []
    for symbol in opaque_symbols:
        code = constants.get(symbol)
        if code is None:
            continue
        row = mapped_rows.get(code, {})
        entries.append(
            {
                "symbol": symbol,
                "code": code,
                "message": row.get("name", symbol.replace("CODE_", "")),
                "confidence": row.get("confidence", ""),
                "source": row.get("source", ""),
                "notes": row.get("notes", ""),
            }
        )

    static_evidence_count = sum(1 for entry in entries if entry["source"].startswith("evidence/"))
    runtime_evidence_count = sum(1 for entry in entries if entry["source"].startswith("captures/"))

    batch_1_codes = {41, 61, 67, 70}
    batch_2_codes = {71, 73, 82, 93, 102}
    batch_3_codes = {114, 115, 116, 138, 141, 142}

    def classify_batch(code: int) -> str:
        if code in batch_1_codes:
            return "S6-Batch-1"
        if code in batch_2_codes:
            return "S6-Batch-2"
        if code in batch_3_codes:
            return "S6-Batch-3"
        return "S6-Unclassified"

    for entry in entries:
        entry["recommended_batch"] = classify_batch(int(entry["code"]))

    return {
        "generated_unix_secs": int(time.time()),
        "stage": "S6 opaque-tail baseline",
        "opaque_tail_count": len(entries),
        "summary": {
            "runtime_evidence_count": runtime_evidence_count,
            "static_evidence_count": static_evidence_count,
            "target": "Promote opaque tail to typed runtime-backed payloads without parity regressions.",
        },
        "recommended_batches": {
            "S6-Batch-1": sorted(batch_1_codes),
            "S6-Batch-2": sorted(batch_2_codes),
            "S6-Batch-3": sorted(batch_3_codes),
        },
        "entries": sorted(entries, key=lambda row: int(row["code"])),
        "inputs": {
            "protocol_lib": str(protocol_lib.relative_to(repo_root)),
            "message_map": str(message_map_path.relative_to(repo_root)),
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate opaque-tail baseline report for S6")
    parser.add_argument("--protocol-lib", default="rust/protocol/src/lib.rs")
    parser.add_argument("--message-map", default="analysis/ghidra/maps/message_map.csv")
    parser.add_argument("--out", default="docs/state/opaque-tail-report.json")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parents[2]
    protocol_lib = (repo_root / args.protocol_lib).resolve()
    message_map = (repo_root / args.message_map).resolve()
    out = (repo_root / args.out).resolve()

    report = build_report(repo_root, protocol_lib, message_map)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
