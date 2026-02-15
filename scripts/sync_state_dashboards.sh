#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

python3 "${ROOT_DIR}/tools/protocol/generate_protocol_matrix.py"
python3 "${ROOT_DIR}/tools/state/generate_dashboard_data.py"
python3 "${ROOT_DIR}/tools/state/generate_codebase_graph.py"
python3 "${ROOT_DIR}/tools/state/verify_s5a_closure.py"
python3 "${ROOT_DIR}/tools/state/report_opaque_tail.py"
python3 "${ROOT_DIR}/tools/docs/generate_pr_index.py"

echo "State dashboards and indexes synchronized."
