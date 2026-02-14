#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SCENARIO="${SCENARIO:-login-search-download}"
DURATION="${DURATION:-120}"
PROCESS_NAME="${PROCESS_NAME:-SoulseekQt}"
IFACE="${IFACE:-}"
BPF_FILTER="${BPF_FILTER:-tcp}"
SKIP_PCAP="${SKIP_PCAP:-0}"
AUTO_REDACT="${AUTO_REDACT:-1}"

args=(
  "${ROOT_DIR}/tools/runtime/capture_harness.py"
  "--process" "${PROCESS_NAME}"
  "--duration" "${DURATION}"
  "--output-root" "captures/raw"
  "--label" "${SCENARIO}"
  "--bpf" "${BPF_FILTER}"
)

if [ -n "${IFACE}" ]; then
  args+=("--iface" "${IFACE}")
fi
if [ "${SKIP_PCAP}" = "1" ]; then
  args+=("--skip-pcap")
fi

echo "starting golden capture scenario: ${SCENARIO}"
result="$(${ROOT_DIR}/.venv-tools/bin/python "${args[@]}")"
echo "${result}"

if [ "${AUTO_REDACT}" = "1" ]; then
  run_dir="$(python3 - <<'PY' "${result}"
import json,sys
print(json.loads(sys.argv[1])["run_dir"])
PY
)"
  "${ROOT_DIR}/.venv-tools/bin/python" "${ROOT_DIR}/tools/runtime/redact_capture_run.py" --run-dir "${run_dir}"
fi
