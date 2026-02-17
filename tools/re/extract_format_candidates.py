from __future__ import annotations

import argparse
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


def now_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def read_indexed_lines(path: Path) -> list[tuple[Path, int, str]]:
    if not path.exists():
        return []
    lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    return [(path, idx, line) for idx, line in enumerate(lines, start=1)]


def select_hits(lines: list[tuple[Path, int, str]], tokens: list[str], repo_root: Path) -> list[dict[str, Any]]:
    if not tokens:
        return []

    lowered_tokens = [token.lower() for token in tokens]
    hits: list[dict[str, Any]] = []
    for source_path, lineno, line in lines:
        normalized = line.lower()
        if any(token in normalized for token in lowered_tokens):
            hits.append({"source": to_rel(source_path, repo_root), "line": lineno, "text": line.strip()})
    return hits


def confidence_from_counts(symbol_hits: int, method_hits: int, string_hits: int) -> str:
    total = symbol_hits + method_hits + string_hits
    kinds = sum(1 for value in (symbol_hits, method_hits, string_hits) if value > 0)
    if total >= 12 and kinds == 3:
        return "high"
    if total >= 5 and kinds >= 2:
        return "medium"
    return "low"


def trim_hits(hits: list[dict[str, Any]], limit: int = 8) -> list[dict[str, Any]]:
    if len(hits) <= limit:
        return hits
    return hits[:limit]


def to_rel(path: Path, root: Path) -> str:
    try:
        return path.resolve().relative_to(root.resolve()).as_posix()
    except ValueError:
        return str(path.resolve())


def write_json(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")


def write_markdown(path: Path, payload: dict[str, Any]) -> None:
    lines: list[str] = []
    lines.append("# Static Persistence Format Candidates")
    lines.append("")
    lines.append(f"- Generated at: `{payload['generated_at']}`")
    lines.append(f"- Candidate count: `{len(payload['candidates'])}`")
    lines.append("")
    lines.append("| ID | Confidence | Symbol Hits | Method Hits | String Hits |")
    lines.append("|---|---:|---:|---:|---:|")
    for candidate in payload["candidates"]:
        lines.append(
            f"| `{candidate['id']}` | `{candidate['confidence']}` | "
            f"{candidate['counts']['symbol_hits']} | {candidate['counts']['method_hits']} | {candidate['counts']['string_hits']} |"
        )
    lines.append("")

    for candidate in payload["candidates"]:
        lines.append(f"## {candidate['id']}")
        lines.append("")
        lines.append(f"- Title: {candidate['title']}")
        lines.append(f"- Confidence: `{candidate['confidence']}`")
        lines.append(f"- Notes: {candidate['notes']}")
        lines.append("- Evidence snippets:")
        for section in ("symbol_hits", "method_hits", "string_hits"):
            hits = candidate["evidence"][section]
            lines.append(f"  - {section}: `{len(hits)}`")
            for hit in hits:
                lines.append(f"    - `{hit['source']}:{hit['line']}` {hit['text']}")
        lines.append("")

    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description="Extract persistence format candidates from static evidence files")
    parser.add_argument("--nm", default="evidence/ui_audit/decomp/nm_demangled_full.txt")
    parser.add_argument("--mainwindow", default="evidence/ui_audit/decomp/mainwindow_methods.txt")
    parser.add_argument("--transfer", default="evidence/ui_audit/decomp/transfer_methods.txt")
    parser.add_argument("--strings", default="evidence/ui_audit/ui_strings_feature_candidates.txt")
    parser.add_argument("--output-json", default="analysis/re/format_candidates.json")
    parser.add_argument("--output-md", default="docs/re/static/file-format-candidates.md")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parents[2]
    nm_path = (repo_root / args.nm).resolve()
    mainwindow_path = (repo_root / args.mainwindow).resolve()
    transfer_path = (repo_root / args.transfer).resolve()
    strings_path = (repo_root / args.strings).resolve()
    output_json = (repo_root / args.output_json).resolve()
    output_md = (repo_root / args.output_md).resolve()

    nm_lines = read_indexed_lines(nm_path)
    method_lines = read_indexed_lines(mainwindow_path) + read_indexed_lines(transfer_path)
    string_lines = read_indexed_lines(strings_path)

    candidates = [
        {
            "id": "FMT-SETTINGS-QSETTINGS",
            "title": "QSettings-backed persistent options",
            "notes": "Key/value persistence for UI and transfer behavior toggles.",
            "symbol_tokens": ["QSettings::setValue", "QSettings::value", "QSettings::contains", "QSettings::remove"],
            "method_tokens": ["MainWindow::saveData", "MainWindow::onImportClientDataClicked", "MainWindow::onExportClientDataClicked"],
            "string_tokens": ["minimize_on_close", "transfer_queued", "showprivate", "max_results_per_search", "save data"],
        },
        {
            "id": "FMT-TRANSFER-STATE",
            "title": "Transfer queue and progress persistence",
            "notes": "Serialized transfer queue/in-progress/completed state and requeue semantics.",
            "symbol_tokens": ["WriteString(QFile&", "ReadString(QFile&", "QFileStreamer::ReadBuffer", "QFileStreamer::WriteBuffer"],
            "method_tokens": ["TransferQueueManager::OnDataLoaded", "TransferQueueManager::TransfersLoaded", "TransferQueueManager::requeueDownload"],
            "string_tokens": ["transfer_queued", "queued", "Data successfully imported"],
        },
        {
            "id": "FMT-USERLIST-IMPORT",
            "title": "User list import format",
            "notes": "Legacy hotlist import pathway from `hotlist.cfg`.",
            "symbol_tokens": ["MainWindow::importConfigurationData", "QFileDialog::getOpenFileName", "QFile::open"],
            "method_tokens": ["MainWindow::onImportUserListClicked", "MainWindow::importConfigurationData"],
            "string_tokens": ["hotlist.cfg", "import your user list", "importUserListButton"],
        },
        {
            "id": "FMT-CLIENT-DATA-EXPORT-IMPORT",
            "title": "Client data backup/restore format",
            "notes": "Export/import UI surface for complete client-data snapshot.",
            "symbol_tokens": ["MainWindow::importConfigurationData", "MainWindow::saveData", "QDataStream::writeBytes", "QDataStream::readRawData"],
            "method_tokens": ["MainWindow::onExportClientDataClicked", "MainWindow::onImportClientDataClicked"],
            "string_tokens": ["Client data exported", "Client data export failed", "Client data successfully imported", "Data import failed"],
        },
        {
            "id": "FMT-SHARE-SCAN-CACHE",
            "title": "Share scan and file-index cache",
            "notes": "Persisted share scan outcomes and refresh behavior linked to transfer availability.",
            "symbol_tokens": ["QFileInfo::absoluteFilePath", "QFile::exists", "QFile::remove"],
            "method_tokens": ["MainWindow::onSharedFilesRescanned", "MainWindow::onSharedFolderAdded", "TransferQueueManager::onUserShare"],
            "string_tokens": ["shared folder", "rescanned", "share"],
        },
        {
            "id": "FMT-SEARCH-HISTORY-PREFERENCES",
            "title": "Search history and preference persistence",
            "notes": "Search history controls and related UX preference keys.",
            "symbol_tokens": ["QSettings::setValue", "QSettings::value"],
            "method_tokens": ["MainWindow::onClearSearchHistoryClicked", "MainWindow::onShowPrivateSearchResultsClicked", "MainWindow::onUseOldStyleSearchResultsCheckboxToggled"],
            "string_tokens": ["clearsearchhistory", "show private search", "search history"],
        },
    ]

    output_candidates: list[dict[str, Any]] = []
    for candidate in candidates:
        symbol_hits = trim_hits(select_hits(nm_lines, candidate["symbol_tokens"], repo_root))
        method_hits = trim_hits(select_hits(method_lines, candidate["method_tokens"], repo_root))
        string_hits = trim_hits(select_hits(string_lines, candidate["string_tokens"], repo_root))

        confidence = confidence_from_counts(len(symbol_hits), len(method_hits), len(string_hits))
        output_candidates.append(
            {
                "id": candidate["id"],
                "title": candidate["title"],
                "notes": candidate["notes"],
                "confidence": confidence,
                "counts": {
                    "symbol_hits": len(symbol_hits),
                    "method_hits": len(method_hits),
                    "string_hits": len(string_hits),
                },
                "evidence": {
                    "symbol_hits": symbol_hits,
                    "method_hits": method_hits,
                    "string_hits": string_hits,
                },
            }
        )

    payload = {
        "generated_at": now_iso(),
        "inputs": {
            "nm": to_rel(nm_path, repo_root),
            "mainwindow": to_rel(mainwindow_path, repo_root),
            "transfer": to_rel(transfer_path, repo_root),
            "strings": to_rel(strings_path, repo_root),
        },
        "candidates": output_candidates,
    }

    write_json(output_json, payload)
    write_markdown(output_md, payload)
    print(json.dumps({"output_json": str(output_json), "output_md": str(output_md)}, ensure_ascii=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
