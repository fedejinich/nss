#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

python3 -m unittest discover -s "${ROOT_DIR}/tests" -p 'test_*.py' -v

cd "${ROOT_DIR}/rust"
cargo test

cd "${ROOT_DIR}"
./scripts/run_diff_verify.sh
