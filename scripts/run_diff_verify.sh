#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RED_DIR="${ROOT_DIR}/captures/redacted"
REQUIRED_RUNS=(
  "login-only"
  "login-search"
  "login-search-download"
  "upload-deny"
  "upload-accept"
)

cd "${ROOT_DIR}/rust"
cargo run -q -p soul-cli -- verify fixtures \
  --fixtures-dir "${ROOT_DIR}/captures/fixtures" \
  --report "${ROOT_DIR}/captures/fixtures/verify-report.json"

for run_id in "${REQUIRED_RUNS[@]}"; do
  if [ ! -d "${RED_DIR}/${run_id}" ]; then
    echo "missing redacted run: ${RED_DIR}/${run_id}" >&2
    exit 1
  fi
  cargo run -q -p soul-cli -- verify captures --run "${run_id}" --base-dir "${RED_DIR}"
done
