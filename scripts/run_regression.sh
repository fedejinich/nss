#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

python3 -m unittest discover -s "${ROOT_DIR}/tests" -p 'test_*.py' -v
python3 "${ROOT_DIR}/scripts/kb_validate.py"

cd "${ROOT_DIR}/rust"
cargo test

cd "${ROOT_DIR}"
./scripts/run_diff_verify.sh

if [ -x "${ROOT_DIR}/.venv-tools/bin/zensical" ]; then
  "${ROOT_DIR}/.venv-tools/bin/zensical" build -f "${ROOT_DIR}/zensical.toml" >/dev/null
fi
