#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import struct
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable


@dataclass(frozen=True)
class FunctionConfig:
    scope: str
    symbol: str


FUNCTIONS = (
    FunctionConfig(scope="server", symbol="__ZN6Server19MessageCodeToStringEi"),
    FunctionConfig(scope="peer", symbol="__ZN13PeerMessenger19MessageCodeToStringEi"),
)

RE_CODE_LIMIT = re.compile(r"cmpl\s+\$0x([0-9a-fA-F]+),\s+%e[ad]x")
RE_TABLE_BASE = re.compile(r"^([0-9a-f]{16})\s+leaq\s+([+-]?0x[0-9a-fA-F]+)\(%rip\),\s+%r[ac]x")
RE_LINE_ADDR = re.compile(r"^([0-9a-f]{16})\s")
RE_LITERAL = re.compile(r"^([0-9a-f]{16})\s+leaq\s+.*literal pool for: \"([^\"]*)\"")
RE_TEXT_ADDR = re.compile(r"\baddr\s+0x([0-9a-fA-F]+)")
RE_TEXT_OFFSET = re.compile(r"\boffset\s+(\d+)")
RE_X64_SLICE = re.compile(r"architecture x86_64.*?offset (\d+)", re.S)


def run(cmd: list[str]) -> str:
    return subprocess.check_output(cmd, text=True)


def parse_x64_slice_offset(binary: Path) -> int:
    details = run(["lipo", "-detailed_info", str(binary)])
    match = RE_X64_SLICE.search(details)
    if not match:
        raise RuntimeError("x86_64 slice offset not found in lipo output")
    return int(match.group(1))


def parse_text_section(binary: Path) -> tuple[int, int]:
    load_commands = run(["otool", "-arch", "x86_64", "-l", str(binary)])
    lines = load_commands.splitlines()
    for idx, line in enumerate(lines):
        if "sectname __text" not in line:
            continue
        window = "\n".join(lines[idx : idx + 24])
        addr_match = RE_TEXT_ADDR.search(window)
        offset_match = RE_TEXT_OFFSET.search(window)
        if addr_match and offset_match:
            return int(addr_match.group(1), 16), int(offset_match.group(1))
    raise RuntimeError("__TEXT,__text section metadata not found")


def extract_function_lines(disasm: str, symbol: str) -> list[str]:
    rows: list[str] = []
    active = False
    for line in disasm.splitlines():
        if line.startswith(f"{symbol}:"):
            active = True
            rows.append(line)
            continue
        if not active:
            continue
        if line.startswith("__ZN") and not line.startswith(f"{symbol}:"):
            break
        rows.append(line)
    if not rows:
        raise RuntimeError(f"symbol not found in disassembly: {symbol}")
    return rows


def parse_code_limit(function_lines: Iterable[str]) -> int:
    for line in function_lines:
        match = RE_CODE_LIMIT.search(line)
        if match:
            return int(match.group(1), 16) + 1
    raise RuntimeError("code limit compare not found in function body")


def parse_table_base(function_lines: list[str]) -> int:
    for idx, line in enumerate(function_lines):
        match = RE_TABLE_BASE.search(line)
        if not match:
            continue
        displacement = int(match.group(2), 16)
        next_addr = None
        for look_ahead in function_lines[idx + 1 :]:
            addr_match = RE_LINE_ADDR.match(look_ahead)
            if addr_match:
                next_addr = int(addr_match.group(1), 16)
                break
        if next_addr is None:
            raise RuntimeError("failed to locate instruction after jump-table base LEA")
        return next_addr + displacement
    raise RuntimeError("jump-table base LEA not found in function body")


def parse_literal_targets(function_lines: Iterable[str]) -> dict[int, str]:
    targets: dict[int, str] = {}
    for line in function_lines:
        match = RE_LITERAL.search(line)
        if not match:
            continue
        targets[int(match.group(1), 16)] = match.group(2)
    return targets


def decode_jump_table(
    *,
    binary: Path,
    slice_offset: int,
    text_addr: int,
    text_offset: int,
    table_base: int,
    entry_count: int,
    literals: dict[int, str],
    scope: str,
) -> list[dict[str, object]]:
    payload = binary.read_bytes()
    table_offset = slice_offset + text_offset + (table_base - text_addr)
    entries: list[dict[str, object]] = []
    for index in range(entry_count):
        rel = struct.unpack_from("<i", payload, table_offset + index * 4)[0]
        target = table_base + rel
        entries.append(
            {
                "scope": scope,
                "code": index + 1,
                "target_va": f"0x{target:016x}",
                "name": literals.get(target, ""),
            }
        )
    return entries


def render_markdown(entries: list[dict[str, object]], binary: Path) -> str:
    server = [row for row in entries if row["scope"] == "server" and row["name"]]
    peer = [row for row in entries if row["scope"] == "peer" and row["name"]]

    lines = [
        "# Message Codes Extracted From Jump Tables",
        "",
        f"- Source binary: `{binary}`",
        "- Method: parse x86_64 jump tables from `Server::MessageCodeToString` and `PeerMessenger::MessageCodeToString`.",
        "- This file is generated by `tools/re/extract_message_codes.py`.",
        "",
        "## Server Codes",
        "",
        "| code | message | target_va |",
        "|---:|---|---|",
    ]

    for row in server:
        lines.append(f"| {row['code']} | `{row['name']}` | `{row['target_va']}` |")

    lines.extend(
        [
            "",
            "## Peer Codes",
            "",
            "| code | message | target_va |",
            "|---:|---|---|",
        ]
    )
    for row in peer:
        lines.append(f"| {row['code']} | `{row['name']}` | `{row['target_va']}` |")

    lines.append("")
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description="Extract Soulseek message-code mappings from jump tables")
    parser.add_argument(
        "--binary",
        default="/Applications/SoulseekQt.app/Contents/MacOS/SoulseekQt",
        help="Path to SoulseekQt universal binary",
    )
    parser.add_argument(
        "--out-json",
        default="evidence/reverse/message_codes_jump_table.json",
        help="Output JSON path",
    )
    parser.add_argument(
        "--out-md",
        default="evidence/reverse/message_codes_jump_table.md",
        help="Output Markdown path",
    )
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parent.parent.parent
    binary = Path(args.binary).expanduser().resolve()
    out_json = (repo_root / args.out_json).resolve()
    out_md = (repo_root / args.out_md).resolve()

    disasm = run(["otool", "-arch", "x86_64", "-tvV", str(binary)])
    slice_offset = parse_x64_slice_offset(binary)
    text_addr, text_offset = parse_text_section(binary)

    all_entries: list[dict[str, object]] = []
    for config in FUNCTIONS:
        function_lines = extract_function_lines(disasm, config.symbol)
        entry_count = parse_code_limit(function_lines)
        table_base = parse_table_base(function_lines)
        literals = parse_literal_targets(function_lines)
        entries = decode_jump_table(
            binary=binary,
            slice_offset=slice_offset,
            text_addr=text_addr,
            text_offset=text_offset,
            table_base=table_base,
            entry_count=entry_count,
            literals=literals,
            scope=config.scope,
        )
        all_entries.extend(entries)

    payload = {
        "binary": str(binary),
        "arch": "x86_64",
        "entries": all_entries,
    }
    out_json.parent.mkdir(parents=True, exist_ok=True)
    out_json.write_text(json.dumps(payload, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")

    out_md.parent.mkdir(parents=True, exist_ok=True)
    out_md.write_text(render_markdown(all_entries, binary), encoding="utf-8")
    print(json.dumps({"out_json": str(out_json), "out_md": str(out_md)}, ensure_ascii=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
