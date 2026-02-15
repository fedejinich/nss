from __future__ import annotations

import argparse
import csv
import json
import re
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path


CONST_RE = re.compile(r"pub const CODE_(SM_[A-Z0-9_]+|PM_[A-Z0-9_]+): u32 = (\d+);")


def now_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def read_message_names(path: Path) -> list[str]:
    names = []
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line:
            continue
        if line.startswith("SM_") or line.startswith("PM_"):
            names.append(line)
    return names


def read_message_map(path: Path) -> dict[str, dict[str, str]]:
    rows: dict[str, dict[str, str]] = {}
    with path.open("r", encoding="utf-8") as fh:
        reader = csv.DictReader(fh)
        for row in reader:
            rows[row["name"]] = row
    return rows


def read_protocol_constants(path: Path) -> dict[str, int]:
    data = path.read_text(encoding="utf-8")
    constants: dict[str, int] = {}
    for name, code in CONST_RE.findall(data):
        constants[name] = int(code)
    return constants


def first_sentence(note: str) -> str:
    clean = " ".join(note.split())
    if not clean:
        return "Pending behavior mapping."
    if "." in clean:
        return clean.split(".", 1)[0].strip() + "."
    return clean


def build_rows(
    *,
    known_names: list[str],
    message_map: dict[str, dict[str, str]],
    protocol_constants: dict[str, int],
) -> list[dict[str, str]]:
    all_names = sorted(set(known_names) | set(message_map) | set(protocol_constants))
    rows: list[dict[str, str]] = []

    for name in all_names:
        mapped = message_map.get(name)
        implemented = name in protocol_constants
        scope = "server" if name.startswith("SM_") else "peer"
        code = ""
        confidence = ""
        evidence = "evidence/reverse/message_name_strings.txt"
        summary = "Known message name from static string table; payload and behavior mapping pending."
        status = "missing"

        if mapped:
            code = mapped.get("code", "")
            confidence = mapped.get("confidence", "")
            evidence = mapped.get("source", evidence)
            summary = first_sentence(mapped.get("notes", ""))
            status = "mapped_not_implemented"

        if implemented:
            code = str(protocol_constants[name])
            status = "implemented_not_mapped" if not mapped else "implemented_mapped"

        rows.append(
            {
                "scope": scope,
                "code": code,
                "name": name,
                "status": status,
                "confidence": confidence,
                "summary": summary,
                "evidence": evidence,
            }
        )

    rows.sort(
        key=lambda row: (
            row["scope"],
            {"implemented_mapped": 0, "mapped_not_implemented": 1, "implemented_not_mapped": 2, "missing": 3}.get(
                row["status"], 9
            ),
            int(row["code"]) if row["code"].isdigit() else 10_000_000,
            row["name"],
        )
    )
    return rows


def render_markdown(rows: list[dict[str, str]]) -> str:
    counts = Counter(row["status"] for row in rows)
    server_total = sum(1 for row in rows if row["scope"] == "server")
    peer_total = sum(1 for row in rows if row["scope"] == "peer")

    lines = [
        "# Protocol Message Matrix",
        "",
        "This matrix tracks protocol coverage from authoritative artifacts.",
        "",
        "## Snapshot",
        "",
        f"- Generated at: `{now_iso()}`",
        f"- Total messages tracked: `{len(rows)}`",
        f"- Server messages: `{server_total}`",
        f"- Peer messages: `{peer_total}`",
        f"- Implemented + mapped: `{counts.get('implemented_mapped', 0)}`",
        f"- Mapped not implemented: `{counts.get('mapped_not_implemented', 0)}`",
        f"- Implemented not mapped: `{counts.get('implemented_not_mapped', 0)}`",
        f"- Missing: `{counts.get('missing', 0)}`",
        "",
        "Status legend:",
        "",
        "- `implemented_mapped`: present in authoritative map and implemented in `rust/protocol`.",
        "- `mapped_not_implemented`: mapped with evidence but not yet implemented in `rust/protocol`.",
        "- `implemented_not_mapped`: implemented in `rust/protocol` but absent from authoritative map.",
        "- `missing`: known from static string tables but not mapped/implemented yet.",
        "",
        "## Matrix",
        "",
        "| scope | code | message | status | confidence | purpose summary | evidence |",
        "|---|---:|---|---|---|---|---|",
    ]

    for row in rows:
        lines.append(
            "| {scope} | {code} | `{name}` | `{status}` | {confidence} | {summary} | `{evidence}` |".format(
                scope=row["scope"],
                code=row["code"] or "",
                name=row["name"],
                status=row["status"],
                confidence=row["confidence"] or "",
                summary=row["summary"].replace("|", "\\|"),
                evidence=row["evidence"],
            )
        )

    lines.extend(
        [
            "",
            "## Regeneration",
            "",
            "```bash",
            "python3 tools/protocol/generate_protocol_matrix.py",
            "```",
        ]
    )
    return "\n".join(lines) + "\n"


def render_json(rows: list[dict[str, str]]) -> dict[str, object]:
    counts = Counter(row["status"] for row in rows)
    server_total = sum(1 for row in rows if row["scope"] == "server")
    peer_total = sum(1 for row in rows if row["scope"] == "peer")

    return {
        "generated_at": now_iso(),
        "snapshot": {
            "total_messages": len(rows),
            "server_messages": server_total,
            "peer_messages": peer_total,
            "implemented_mapped": counts.get("implemented_mapped", 0),
            "mapped_not_implemented": counts.get("mapped_not_implemented", 0),
            "implemented_not_mapped": counts.get("implemented_not_mapped", 0),
            "missing": counts.get("missing", 0),
        },
        "rows": [
            {
                "scope": row["scope"],
                "code": int(row["code"]) if row["code"].isdigit() else None,
                "message": row["name"],
                "status": row["status"],
                "confidence": row["confidence"] or None,
                "summary": row["summary"],
                "evidence": row["evidence"],
            }
            for row in rows
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate protocol coverage matrix markdown")
    parser.add_argument("--message-names", default="evidence/reverse/message_name_strings.txt")
    parser.add_argument("--message-map", default="analysis/ghidra/maps/message_map.csv")
    parser.add_argument("--protocol-lib", default="rust/protocol/src/lib.rs")
    parser.add_argument("--out", default="docs/state/protocol-matrix.md")
    parser.add_argument("--out-json", default="docs/state/protocol-matrix.json")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parent.parent.parent
    names_path = (repo_root / args.message_names).resolve()
    map_path = (repo_root / args.message_map).resolve()
    protocol_path = (repo_root / args.protocol_lib).resolve()
    out_path = (repo_root / args.out).resolve()
    out_json_path = (repo_root / args.out_json).resolve()

    rows = build_rows(
        known_names=read_message_names(names_path),
        message_map=read_message_map(map_path),
        protocol_constants=read_protocol_constants(protocol_path),
    )
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(render_markdown(rows), encoding="utf-8")
    out_json_path.parent.mkdir(parents=True, exist_ok=True)
    out_json_path.write_text(json.dumps(render_json(rows), indent=2) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
