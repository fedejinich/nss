#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUN_DIR="${RUN_DIR:-}"
OUT_ROOT="${OUT_ROOT:-captures/redacted}"
RUN_ID="${RUN_ID:-}"

if [ -z "${RUN_DIR}" ]; then
  echo "RUN_DIR is required" >&2
  echo "example: RUN_DIR=captures/raw/20260214T000000Z-login-only scripts/redact_capture_run.sh" >&2
  exit 1
fi

args=(
  "${ROOT_DIR}/tools/runtime/redact_capture_run.py"
  "--run-dir" "${RUN_DIR}"
  "--out-root" "${OUT_ROOT}"
)

if [ -n "${RUN_ID}" ]; then
  args+=("--run-id" "${RUN_ID}")
fi

"${ROOT_DIR}/.venv-tools/bin/python" "${args[@]}"
