#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DURATION="${DURATION:-60}"
PROCESS_NAME="${PROCESS_NAME:-SoulseekQt}"
LABEL="${LABEL:-session}"
IFACE="${IFACE:-}"
BPF_FILTER="${BPF_FILTER:-tcp}"
SKIP_PCAP="${SKIP_PCAP:-0}"

args=(
  "${ROOT_DIR}/tools/runtime/capture_harness.py"
  "--process" "${PROCESS_NAME}"
  "--duration" "${DURATION}"
  "--output-root" "captures/runs"
  "--label" "${LABEL}"
  "--bpf" "${BPF_FILTER}"
)

if [ -n "${IFACE}" ]; then
  args+=("--iface" "${IFACE}")
fi
if [ "${SKIP_PCAP}" = "1" ]; then
  args+=("--skip-pcap")
fi

"${ROOT_DIR}/.venv-tools/bin/python" "${args[@]}"
