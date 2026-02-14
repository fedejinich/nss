from __future__ import annotations

import argparse
import json
import re
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Iterable


SYMBOL_RE = re.compile(r"^(?P<addr>[0-9a-fA-F]+|0x[0-9a-fA-F]+)\s+\([^\)]+\)\s+external\s+(?P<name>.+)$")


@dataclass(frozen=True)
class FlowNodeSpec:
    node_id: str
    symbol: str
    stage: str
    description: str


@dataclass(frozen=True)
class FlowEdgeSpec:
    from_id: str
    to_id: str
    reason: str


FLOW_NODES: list[FlowNodeSpec] = [
    FlowNodeSpec(
        "server_prepare_search",
        "Server::PrepareSearch(QString)",
        "server_tx",
        "Normaliza texto de busqueda antes de serializar.",
    ),
    FlowNodeSpec(
        "server_file_search",
        "Server::FileSearch(QString, QString)",
        "server_tx",
        "Construye y envia mensaje de busqueda al servidor.",
    ),
    FlowNodeSpec(
        "server_send_message",
        "Server::SendMessage(MemStream&, bool)",
        "server_tx",
        "Serializa MemStream en socket de servidor.",
    ),
    FlowNodeSpec(
        "server_handle_message",
        "Server::HandleMessage(int, MemStream&)",
        "server_rx",
        "Despacha mensajes entrantes del servidor.",
    ),
    FlowNodeSpec(
        "peer_queue_download",
        "PeerMessenger::QueueDownload(QString, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>)",
        "peer_tx",
        "Encapsula solicitud inicial de descarga a peer.",
    ),
    FlowNodeSpec(
        "peer_send_message",
        "PeerMessenger::SendMessage(QTcpSocket*, MemStream&, bool)",
        "peer_tx",
        "Serializa mensajes sobre socket peer.",
    ),
    FlowNodeSpec(
        "peer_handle_message",
        "PeerMessenger::HandleMessage(QTcpSocket*, MemStream)",
        "peer_rx",
        "Despacha mensajes entrantes peer a manejadores de transferencia.",
    ),
    FlowNodeSpec(
        "transfer_on_queue_download",
        "TransferQueueManager::OnQueueDownloadRequested(QString, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, long long)",
        "transfer_ctrl",
        "Materializa entrada de cola local y dispara conexion de archivo.",
    ),
    FlowNodeSpec(
        "transfer_on_file_request",
        "TransferQueueManager::OnFileTransferRequest(QString, int, unsigned int, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, long long)",
        "transfer_ctrl",
        "Negocia request/respuesta de transferencia con peer.",
    ),
    FlowNodeSpec(
        "download_read_socket",
        "DownloadTask::readSocket()",
        "download_rx",
        "Consume bytes del socket para escribir archivo destino.",
    ),
    FlowNodeSpec(
        "upload_write_socket",
        "UploadTask::writeToSocket()",
        "upload_tx",
        "Publica bytes locales al peer remoto.",
    ),
]


FLOW_EDGES: list[FlowEdgeSpec] = [
    FlowEdgeSpec("server_file_search", "server_prepare_search", "file_search prepara terminos antes de armar frame"),
    FlowEdgeSpec("server_file_search", "server_send_message", "file_search termina en send_message"),
    FlowEdgeSpec("server_send_message", "server_handle_message", "request/response via socket servidor"),
    FlowEdgeSpec("peer_queue_download", "transfer_on_queue_download", "queue_download dispara alta en cola"),
    FlowEdgeSpec("transfer_on_queue_download", "peer_send_message", "cola de descarga emite request al peer"),
    FlowEdgeSpec("peer_send_message", "peer_handle_message", "response peer vuelve por handle_message"),
    FlowEdgeSpec("peer_handle_message", "transfer_on_file_request", "dispatch de transfer_request"),
    FlowEdgeSpec("transfer_on_file_request", "download_read_socket", "request aceptado inicia descarga"),
    FlowEdgeSpec("transfer_on_file_request", "upload_write_socket", "request aceptado puede iniciar upload"),
]


def _load_symbols(path: Path) -> dict[str, str]:
    symbols: dict[str, str] = {}
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line:
            continue
        match = SYMBOL_RE.match(line)
        if not match:
            continue
        addr = match.group("addr")
        if not addr.startswith("0x"):
            addr = "0x" + addr
        symbols[match.group("name")] = addr
    return symbols


def _detangling_lines(
    *,
    binary: str,
    architecture: str,
    nodes: list[dict[str, str]],
    edges: Iterable[FlowEdgeSpec],
    symbol_source: str,
    strings_source: str,
) -> list[str]:
    lines: list[str] = []
    lines.append("# Search/Download Flow")
    lines.append("")
    lines.append(f"- Binary: `{binary}`")
    lines.append(f"- Architecture: `{architecture}`")
    lines.append(f"- Symbols source: `{symbol_source}`")
    lines.append(f"- Strings source: `{strings_source}`")
    lines.append("")
    lines.append("## Nodes")
    lines.append("")
    for node in nodes:
        lines.append(f"### `{node['node_id']}`")
        lines.append(f"- Symbol: `{node['symbol']}`")
        lines.append(f"- Address: `{node['address']}`")
        lines.append(f"- Stage: `{node['stage']}`")
        lines.append(f"- Description: {node['description']}")
        disasm = node.get("disassembly")
        if disasm:
            lines.append(f"- Disassembly: `{disasm}`")
        lines.append("")

    lines.append("## Edges")
    lines.append("")
    for edge in edges:
        lines.append(f"- `{edge.from_id}` -> `{edge.to_id}`: {edge.reason}")
    lines.append("")
    return lines


def main() -> int:
    parser = argparse.ArgumentParser(description="Build static search/download flow map from extracted symbols")
    parser.add_argument("--symbols", required=True, help="Path to filtered demangled nm symbols")
    parser.add_argument("--binary", default="SoulseekQt")
    parser.add_argument("--architecture", default="arm64")
    parser.add_argument("--strings-source", default="evidence/reverse/search_download_strings.txt")
    parser.add_argument("--out-json", required=True)
    parser.add_argument("--out-markdown", required=True)
    parser.add_argument("--disasm-dir", required=True)
    parser.add_argument("--repo-root", default=".")
    args = parser.parse_args()

    symbols_path = Path(args.symbols)
    symbols = _load_symbols(symbols_path)
    disasm_dir = Path(args.disasm_dir)
    repo_root = Path(args.repo_root).resolve()

    nodes: list[dict[str, str]] = []
    for spec in FLOW_NODES:
        filename = spec.node_id + ".txt"
        disasm_path = disasm_dir / filename
        disasm_link = ""
        if disasm_path.exists():
            try:
                disasm_link = str(disasm_path.resolve().relative_to(repo_root))
            except ValueError:
                disasm_link = str(disasm_path.resolve())

        node = {
            "node_id": spec.node_id,
            "symbol": spec.symbol,
            "address": symbols.get(spec.symbol, "missing"),
            "stage": spec.stage,
            "description": spec.description,
            "disassembly": disasm_link,
        }
        nodes.append(node)

    out_json = Path(args.out_json)
    out_json.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "generated_at": datetime.now(timezone.utc).replace(microsecond=0).isoformat(),
        "binary": args.binary,
        "architecture": args.architecture,
        "sources": {
            "symbols": str(symbols_path),
            "strings": args.strings_source,
        },
        "nodes": nodes,
        "edges": [edge.__dict__ for edge in FLOW_EDGES],
    }
    out_json.write_text(json.dumps(payload, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")

    out_md = Path(args.out_markdown)
    out_md.parent.mkdir(parents=True, exist_ok=True)
    out_md.write_text(
        "\n".join(
            _detangling_lines(
                binary=args.binary,
                architecture=args.architecture,
                nodes=nodes,
                edges=FLOW_EDGES,
                symbol_source=str(symbols_path),
                strings_source=args.strings_source,
            )
        )
        + "\n",
        encoding="utf-8",
    )

    print(json.dumps({"nodes": len(nodes), "edges": len(FLOW_EDGES), "output": str(out_json)}, ensure_ascii=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
