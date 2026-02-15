#!/usr/bin/env python3
from __future__ import annotations

import argparse
import re
from dataclasses import dataclass
from pathlib import Path

PR_PATTERN = re.compile(r"^(\d{4})-(.+)\.md$")
STAGE_PATTERN = re.compile(r"-s(\d+[a-z]?)")


@dataclass(frozen=True)
class PrEntry:
    order: int
    filename: str
    title: str
    stage_group: str


def infer_stage_group(filename: str) -> str:
    lower = filename.lower()
    matches = STAGE_PATTERN.findall(lower)
    if matches:
        if len(matches) == 1:
            return f"S{matches[0].upper()}"
        start = matches[0].upper()
        end = matches[-1].upper()
        return f"S{start}-S{end}"

    if "kb" in lower or "parity" in lower:
        return "Foundation"

    return "Other"


def human_title(stem_tail: str) -> str:
    words = stem_tail.replace("-", " ").split()
    return " ".join(word.upper() if word.isupper() else word.capitalize() for word in words)


def stage_sort_key(stage: str) -> tuple[int, str]:
    if stage == "Foundation":
        return (0, stage)
    if stage == "Other":
        return (999, stage)

    match = re.match(r"^S(\d+)([A-Z]?)(?:-S(\d+)([A-Z]?))?$", stage)
    if not match:
        return (500, stage)

    major = int(match.group(1))
    letter = match.group(2) or ""
    major2 = int(match.group(3) or major)
    letter2 = match.group(4) or letter
    return (major * 100 + (ord(letter) - 64 if letter else 0), f"{major2:02d}{letter2}")


def collect_pr_entries(pr_dir: Path) -> list[PrEntry]:
    entries: list[PrEntry] = []
    for path in sorted(pr_dir.glob("*.md")):
        if path.name == "index.md":
            continue
        match = PR_PATTERN.match(path.name)
        if not match:
            continue

        order = int(match.group(1))
        tail = match.group(2)
        title = human_title(tail)
        entries.append(
            PrEntry(
                order=order,
                filename=path.name,
                title=title,
                stage_group=infer_stage_group(path.name),
            )
        )

    return entries


def render(entries: list[PrEntry]) -> str:
    grouped: dict[str, list[PrEntry]] = {}
    for entry in entries:
        grouped.setdefault(entry.stage_group, []).append(entry)

    lines: list[str] = []
    lines.append("# PR Catalog")
    lines.append("")
    lines.append("This page indexes all stage PR documents in collapsed groups to keep navigation compact.")
    lines.append("")
    lines.append(f"Total PR docs: `{len(entries)}`")
    lines.append("")

    for stage in sorted(grouped, key=stage_sort_key):
        rows = sorted(grouped[stage], key=lambda row: row.order)
        lines.append("<details>")
        lines.append(f"<summary>{stage} ({len(rows)} PRs)</summary>")
        lines.append("")
        for row in rows:
            route = f"{Path(row.filename).stem}/"
            lines.append(f"- [{row.filename} - {row.title}]({route})")
        lines.append("")
        lines.append("</details>")
        lines.append("")

    lines.append("## Regeneration")
    lines.append("")
    lines.append("```bash")
    lines.append("python3 tools/docs/generate_pr_index.py")
    lines.append("```")
    lines.append("")
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate collapsed PR catalog index")
    parser.add_argument("--pr-dir", default="docs/pr")
    parser.add_argument("--out", default="docs/pr/index.md")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parents[2]
    pr_dir = (repo_root / args.pr_dir).resolve()
    out_path = (repo_root / args.out).resolve()

    entries = collect_pr_entries(pr_dir)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(render(entries), encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
