#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT_DIR}/rust"
cargo run -q -p soul-cli -- verify-fixtures \
  --fixtures-dir "${ROOT_DIR}/captures/fixtures" \
  --report "${ROOT_DIR}/captures/fixtures/verify-report.json"
