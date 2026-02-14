#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN_PATH="${BIN_PATH:-${ROOT_DIR}/analysis/binaries/SoulseekQt}"
ARCH="${ARCH:-arm64}"

SYMBOLS_OUT="${ROOT_DIR}/evidence/reverse/search_download_symbols_nm.txt"
STRINGS_OUT="${ROOT_DIR}/evidence/reverse/search_download_strings.txt"
DISASM_DIR="${ROOT_DIR}/evidence/reverse/disasm"
FLOW_JSON="${ROOT_DIR}/analysis/re/flow_graph.json"
FLOW_MD="${ROOT_DIR}/docs/re/static/search-download-flow.md"

if [ ! -f "${BIN_PATH}" ]; then
  echo "missing binary: ${BIN_PATH}" >&2
  echo "run scripts/ghidra_pipeline.sh first to extract analysis/binaries/SoulseekQt" >&2
  exit 1
fi

if ! command -v lldb >/dev/null 2>&1; then
  echo "lldb not found" >&2
  exit 1
fi

mkdir -p "${DISASM_DIR}" "${ROOT_DIR}/analysis/re" "${ROOT_DIR}/evidence/reverse"

nm -nm "${BIN_PATH}" | c++filt | \
  rg 'Server::(SendMessage\(|HandleMessage\(|FileSearch\(|PrepareSearch\()|PeerMessenger::(HandleMessage\(|SendMessage\(|QueueDownload\(|AcceptTransferRequest\(|RequestUpload\(|MessageCodeToString\()|TransferQueueManager::(OnFileTransferRequest\(|OnQueueDownloadRequested\()|DownloadTask::readSocket\(|UploadTask::writeToSocket\(' \
  > "${SYMBOLS_OUT}"

strings -a "${BIN_PATH}" | \
  rg 'SM_|PM_|search_request|search_request_text|queue_download|transfer|download|upload|FileSearch|QueueDownload|TRANSFER_REQUEST|TRANSFER_RESPONSE' \
  > "${STRINGS_OUT}"

symbol_specs=(
  "server_prepare_search|Server::PrepareSearch(QString)"
  "server_file_search|Server::FileSearch(QString, QString)"
  "server_send_message|Server::SendMessage(MemStream&, bool)"
  "server_handle_message|Server::HandleMessage(int, MemStream&)"
  "peer_queue_download|PeerMessenger::QueueDownload(QString, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>)"
  "peer_send_message|PeerMessenger::SendMessage(QTcpSocket*, MemStream&, bool)"
  "peer_handle_message|PeerMessenger::HandleMessage(QTcpSocket*, MemStream)"
  "transfer_on_queue_download|TransferQueueManager::OnQueueDownloadRequested(QString, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, long long)"
  "transfer_on_file_request|TransferQueueManager::OnFileTransferRequest(QString, int, unsigned int, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, long long)"
  "download_read_socket|DownloadTask::readSocket()"
  "upload_write_socket|UploadTask::writeToSocket()"
)

for spec in "${symbol_specs[@]}"; do
  node_id="${spec%%|*}"
  symbol_name="${spec#*|}"
  out_file="${DISASM_DIR}/${node_id}.txt"
  symbol_addr="$(grep -F " ${symbol_name}" "${SYMBOLS_OUT}" | head -n 1 | awk '{print $1}' || true)"
  if [ -n "${symbol_addr}" ] && [[ "${symbol_addr}" != 0x* ]]; then
    symbol_addr="0x${symbol_addr}"
  fi
  if ! lldb -b -Q \
    -o "target create -a ${ARCH} ${BIN_PATH}" \
    -o "disassemble -n '${symbol_name}'" \
    -o "quit" > "${out_file}" 2>&1; then
    : # fallback below
  fi
  if rg -q "no function was specified|Unable to find symbol|Not disassembling a range" "${out_file}" 2>/dev/null && [ -n "${symbol_addr}" ]; then
    if ! lldb -b -Q \
      -o "target create -a ${ARCH} ${BIN_PATH}" \
      -o "disassemble --start-address ${symbol_addr} --count 240" \
      -o "quit" > "${out_file}" 2>&1; then
      echo "warning: failed to disassemble ${symbol_name} (${symbol_addr})" >&2
    fi
  elif rg -q "no function was specified|Unable to find symbol|Not disassembling a range" "${out_file}" 2>/dev/null; then
    echo "warning: failed to resolve ${symbol_name} and no address fallback found" >&2
  fi
  if [ ! -s "${out_file}" ] && [ -n "${symbol_addr}" ]; then
    lldb -b -Q \
      -o "target create -a ${ARCH} ${BIN_PATH}" \
      -o "disassemble --start-address ${symbol_addr} --count 240" \
      -o "quit" > "${out_file}" 2>&1 || true
  fi
done

python3 "${ROOT_DIR}/tools/re/flow_extract.py" \
  --symbols "${SYMBOLS_OUT}" \
  --repo-root "${ROOT_DIR}" \
  --binary "SoulseekQt" \
  --architecture "${ARCH}" \
  --strings-source "evidence/reverse/search_download_strings.txt" \
  --disasm-dir "${DISASM_DIR}" \
  --out-json "${FLOW_JSON}" \
  --out-markdown "${FLOW_MD}"

echo "flow extraction complete"
echo "- symbols: ${SYMBOLS_OUT}"
echo "- strings: ${STRINGS_OUT}"
echo "- graph: ${FLOW_JSON}"
echo "- doc: ${FLOW_MD}"
