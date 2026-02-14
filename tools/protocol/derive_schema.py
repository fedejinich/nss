from __future__ import annotations

import argparse
import csv
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


KNOWN_PAYLOADS: dict[tuple[str, str], list[dict[str, str]]] = {
    ("server", "SM_LOGIN"): [
        {"name": "username", "type": "string"},
        {"name": "password_md5", "type": "string"},
        {"name": "client_version", "type": "u32"},
        {"name": "minor_version", "type": "u32"},
    ],
    ("server", "SM_FILE_SEARCH"): [
        {"name": "search_token", "type": "u32"},
        {"name": "search_text", "type": "string"},
    ],
    ("peer", "PM_TRANSFER_REQUEST"): [
        {"name": "direction", "type": "u32"},
        {"name": "token", "type": "u32"},
        {"name": "virtual_path", "type": "string"},
        {"name": "file_size", "type": "u64"},
    ],
    ("peer", "PM_TRANSFER_RESPONSE"): [
        {"name": "token", "type": "u32"},
        {"name": "allowed", "type": "bool_u32"},
        {"name": "queue_or_reason", "type": "string"},
    ],
}


KNOWN_CODES: dict[tuple[str, str], int] = {
    ("server", "SM_LOGIN"): 1,
    ("server", "SM_FILE_SEARCH"): 26,
    ("peer", "PM_TRANSFER_REQUEST"): 40,
    ("peer", "PM_TRANSFER_RESPONSE"): 41,
}


def now_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def _read_message_map(path: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    if not path.exists():
        return rows

    with path.open("r", encoding="utf-8") as fh:
        reader = csv.DictReader(fh)
        for row in reader:
            scope = (row.get("scope") or "").strip()
            name = (row.get("name") or "").strip()
            if not scope or not name:
                continue
            code_raw = (row.get("code") or "").strip()
            code = int(code_raw) if code_raw.isdigit() else KNOWN_CODES.get((scope, name))
            rows.append(
                {
                    "scope": scope,
                    "name": name,
                    "code": code,
                    "source": (row.get("source") or "").strip(),
                    "status": (row.get("status") or "").strip(),
                    "notes": (row.get("notes") or "").strip(),
                }
            )
    return rows


def _entry(row: dict[str, Any]) -> dict[str, Any]:
    scope = row["scope"]
    name = row["name"]
    code = row.get("code")

    evidence = []
    if row.get("source"):
        source = row["source"]
        ev_kind = "ghidra_decompile" if "/disasm/" in source or source.endswith(".asm") else "string"
        evidence.append(
            {
                "kind": ev_kind,
                "source": source,
                "note": row.get("notes", "").strip() or "Message mapping source",
            }
        )

    if (scope, name) == ("server", "SM_FILE_SEARCH"):
        evidence.append(
            {
                "kind": "ghidra_decompile",
                "source": "evidence/reverse/disasm/server_file_search.txt",
                "note": "Function writes constant 0x1a before serializing search payload.",
            }
        )
    if (scope, name) in {("peer", "PM_TRANSFER_REQUEST"), ("peer", "PM_TRANSFER_RESPONSE")}:
        evidence.append(
            {
                "kind": "ghidra_decompile",
                "source": "evidence/reverse/disasm/transfer_on_file_request.txt",
                "note": "Transfer queue dispatcher handles peer transfer negotiation path.",
            }
        )

    confidence = "high" if code is not None else "medium"
    if name in {"SM_FILE_SEARCH", "PM_TRANSFER_REQUEST", "PM_TRANSFER_RESPONSE"}:
        confidence = "high"

    return {
        "scope": scope,
        "code": code,
        "name": name,
        "payload": KNOWN_PAYLOADS.get((scope, name), []),
        "confidence": confidence,
        "evidence": evidence,
    }


def build_schema(message_rows: list[dict[str, Any]]) -> dict[str, Any]:
    dedup: dict[tuple[str, str], dict[str, Any]] = {}
    for row in message_rows:
        key = (row["scope"], row["name"])
        dedup[key] = _entry(row)

    for key, code in KNOWN_CODES.items():
        if key in dedup:
            if dedup[key]["code"] is None:
                dedup[key]["code"] = code
            continue
        scope, name = key
        dedup[key] = {
            "scope": scope,
            "code": code,
            "name": name,
            "payload": KNOWN_PAYLOADS.get(key, []),
            "confidence": "medium",
            "evidence": [
                {
                    "kind": "manual_note",
                    "source": "docs/re/static/search-download-flow.md",
                    "note": "Bootstrap default from static flow extraction; verify with runtime capture.",
                }
            ],
        }

    entries = sorted(
        dedup.values(),
        key=lambda row: (row["scope"], row["code"] if row["code"] is not None else 10**9, row["name"]),
    )

    return {
        "version": 1,
        "generated_at": now_iso(),
        "framing": {
            "transport": "tcp",
            "layout": "<u32 frame_len_le><u32 message_code_le><payload>",
            "confidence": "medium",
            "evidence": [
                {
                    "kind": "ghidra_decompile",
                    "source": "evidence/reverse/disasm/server_send_message.txt",
                    "note": "Server send path serializes integer fields through MemStream before socket write.",
                },
                {
                    "kind": "ghidra_decompile",
                    "source": "evidence/reverse/disasm/peer_send_message.txt",
                    "note": "Peer send path mirrors frame serialization through MemStream.",
                },
            ],
        },
        "messages": entries,
    }


def write_markdown(schema: dict[str, Any], out_path: Path) -> None:
    lines: list[str] = []
    lines.append("# Message Schema")
    lines.append("")
    lines.append(f"- Generated: `{schema['generated_at']}`")
    lines.append(f"- Framing: `{schema['framing']['layout']}`")
    lines.append(f"- Framing confidence: `{schema['framing']['confidence']}`")
    lines.append("")
    lines.append("## Messages")
    lines.append("")

    for entry in schema["messages"]:
        code = entry["code"] if entry["code"] is not None else "unknown"
        lines.append(f"### `{entry['scope']}` `{entry['name']}` (code `{code}`)")
        lines.append(f"- Confidence: `{entry['confidence']}`")
        if entry["payload"]:
            lines.append("- Payload fields:")
            for field in entry["payload"]:
                lines.append(f"  - `{field['name']}`: `{field['type']}`")
        else:
            lines.append("- Payload fields: pending derivation")
        lines.append("- Evidence:")
        for ev in entry["evidence"]:
            lines.append(f"  - `{ev['kind']}`: `{ev['source']}` ({ev.get('note', '').strip()})")
        lines.append("")

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description="Derive protocol message schema from KB evidence")
    parser.add_argument("--message-map", default="analysis/ghidra/maps/message_map.csv")
    parser.add_argument("--out-json", default="analysis/protocol/message_schema.json")
    parser.add_argument("--out-md", default="docs/re/static/message-schema.md")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parent.parent.parent
    message_rows = _read_message_map(repo_root / args.message_map)
    schema = build_schema(message_rows)

    out_json = repo_root / args.out_json
    out_json.parent.mkdir(parents=True, exist_ok=True)
    out_json.write_text(json.dumps(schema, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")

    write_markdown(schema, repo_root / args.out_md)

    print(json.dumps({"messages": len(schema["messages"]), "out_json": str(out_json)}, ensure_ascii=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
