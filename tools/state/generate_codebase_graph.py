#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import time
from pathlib import Path

ALLOWED_EXTENSIONS = {
    ".md",
    ".py",
    ".sh",
    ".toml",
    ".json",
    ".jsonl",
    ".csv",
    ".rs",
    ".txt",
    ".yml",
    ".yaml",
    ".html",
    ".js",
    ".css",
}

INCLUDE_TOP_LEVEL_DIRS = {
    "analysis",
    "docs",
    "evidence",
    "frida",
    "overrides",
    "rust",
    "scripts",
    "tests",
    "tools",
}

INCLUDE_ROOT_FILES = {
    "README.md",
    "AGENTS.md",
    "TODO-CODEX.md",
    "zensical.toml",
}

EXCLUDED_DIR_NAMES = {
    ".git",
    ".cache",
    ".venv-tools",
    "__pycache__",
    "site",
    "target",
    "captures",
}


def count_loc(path: Path) -> int:
    try:
        return len(path.read_text(encoding="utf-8", errors="ignore").splitlines())
    except OSError:
        return 0


def file_is_allowed(path: Path) -> bool:
    if path.suffix.lower() not in ALLOWED_EXTENSIONS:
        return False
    return True


def is_excluded(path: Path, repo_root: Path) -> bool:
    rel = path.relative_to(repo_root)
    for part in rel.parts:
        if part in EXCLUDED_DIR_NAMES:
            return True
    return False


def collect_files(repo_root: Path) -> list[Path]:
    files: list[Path] = []

    for name in sorted(INCLUDE_ROOT_FILES):
        candidate = repo_root / name
        if candidate.exists() and candidate.is_file():
            files.append(candidate)

    for top in sorted(INCLUDE_TOP_LEVEL_DIRS):
        top_dir = repo_root / top
        if not top_dir.exists() or not top_dir.is_dir():
            continue

        for candidate in top_dir.rglob("*"):
            if candidate.is_dir():
                continue
            if is_excluded(candidate, repo_root):
                continue
            if not file_is_allowed(candidate):
                continue
            files.append(candidate)

    files = sorted(set(files), key=lambda p: p.relative_to(repo_root).as_posix())
    return files


def parent_dir(path: str) -> str | None:
    if "/" not in path:
        return None
    return path.rsplit("/", 1)[0]


def build_graph(repo_root: Path, files: list[Path]) -> dict[str, object]:
    nodes_by_id: dict[str, dict[str, object]] = {}
    edges: set[tuple[str, str, str]] = set()

    root_id = "root:neosoulseek"
    nodes_by_id[root_id] = {
        "id": root_id,
        "label": "NeoSoulSeek",
        "kind": "root",
        "path": ".",
        "loc": 0,
        "size_bytes": 0,
        "extension": None,
        "domain": "root",
    }

    domain_ids: dict[str, str] = {}
    dir_ids: dict[str, str] = {}

    def ensure_domain(domain: str) -> str:
        if domain in domain_ids:
            return domain_ids[domain]
        domain_id = f"domain:{domain}"
        domain_ids[domain] = domain_id
        nodes_by_id[domain_id] = {
            "id": domain_id,
            "label": domain,
            "kind": "domain",
            "path": domain,
            "loc": 0,
            "size_bytes": 0,
            "extension": None,
            "domain": domain,
        }
        edges.add((root_id, domain_id, "contains"))
        return domain_id

    def ensure_dir(path_str: str, domain: str) -> str:
        if path_str in dir_ids:
            return dir_ids[path_str]

        dir_id = f"dir:{path_str}"
        dir_ids[path_str] = dir_id
        label = path_str.split("/")[-1]
        nodes_by_id[dir_id] = {
            "id": dir_id,
            "label": label,
            "kind": "dir",
            "path": path_str,
            "loc": 0,
            "size_bytes": 0,
            "extension": None,
            "domain": domain,
        }

        parent = parent_dir(path_str)
        if parent is None:
            edges.add((ensure_domain(domain), dir_id, "contains"))
        else:
            edges.add((ensure_dir(parent, domain), dir_id, "contains"))
        return dir_id

    for file_path in files:
        rel = file_path.relative_to(repo_root).as_posix()
        parts = rel.split("/")
        domain = parts[0] if len(parts) > 1 else "repo-root"

        domain_id = ensure_domain(domain)
        dir_path = parent_dir(rel)

        if dir_path is not None:
            parent_id = ensure_dir(dir_path, domain)
        else:
            parent_id = domain_id

        loc = count_loc(file_path)
        size_bytes = file_path.stat().st_size
        extension = file_path.suffix.lower() if file_path.suffix else None

        file_id = f"file:{rel}"
        nodes_by_id[file_id] = {
            "id": file_id,
            "label": parts[-1],
            "kind": "file",
            "path": rel,
            "loc": loc,
            "size_bytes": size_bytes,
            "extension": extension,
            "domain": domain,
        }
        edges.add((parent_id, file_id, "contains"))

        nodes_by_id[root_id]["loc"] = int(nodes_by_id[root_id]["loc"]) + loc
        nodes_by_id[root_id]["size_bytes"] = int(nodes_by_id[root_id]["size_bytes"]) + size_bytes
        nodes_by_id[domain_id]["loc"] = int(nodes_by_id[domain_id]["loc"]) + loc
        nodes_by_id[domain_id]["size_bytes"] = int(nodes_by_id[domain_id]["size_bytes"]) + size_bytes

        current_dir = dir_path
        while current_dir is not None:
            current_id = dir_ids[current_dir]
            nodes_by_id[current_id]["loc"] = int(nodes_by_id[current_id]["loc"]) + loc
            nodes_by_id[current_id]["size_bytes"] = int(nodes_by_id[current_id]["size_bytes"]) + size_bytes
            current_dir = parent_dir(current_dir)

    nodes = sorted(
        nodes_by_id.values(),
        key=lambda n: (
            {"root": 0, "domain": 1, "dir": 2, "file": 3}.get(str(n["kind"]), 9),
            str(n["path"]),
        ),
    )
    edge_rows = sorted(
        ({"source": s, "target": t, "kind": k} for s, t, k in edges),
        key=lambda row: (row["source"], row["target"], row["kind"]),
    )

    stats = {
        "domain_count": len(domain_ids),
        "dir_count": len(dir_ids),
        "file_count": sum(1 for node in nodes if node["kind"] == "file"),
        "node_count": len(nodes),
        "edge_count": len(edge_rows),
        "total_loc": int(nodes_by_id[root_id]["loc"]),
        "total_size_bytes": int(nodes_by_id[root_id]["size_bytes"]),
    }

    return {
        "generated_unix_secs": int(time.time()),
        "repo_root": str(repo_root),
        "scope": {
            "included_top_level_dirs": sorted(INCLUDE_TOP_LEVEL_DIRS),
            "included_root_files": sorted(INCLUDE_ROOT_FILES),
            "excluded_dir_names": sorted(EXCLUDED_DIR_NAMES),
            "allowed_extensions": sorted(ALLOWED_EXTENSIONS),
        },
        "stats": stats,
        "nodes": nodes,
        "edges": edge_rows,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate NeoSoulSeek codebase graph JSON")
    parser.add_argument("--out", default="docs/state/codebase-graph.json")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parents[2]
    out_path = (repo_root / args.out).resolve()

    files = collect_files(repo_root)
    graph = build_graph(repo_root, files)

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(graph, indent=2) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
