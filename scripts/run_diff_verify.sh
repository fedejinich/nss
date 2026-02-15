#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RED_DIR="${ROOT_DIR}/captures/redacted"
VERIFY_MODE="${VERIFY_MODE:-semantic}"
REQUIRED_RUNS=(
  "login-only"
  "login-search"
  "login-search-download"
  "upload-deny"
  "upload-accept"
  "login-room-list"
  "login-join-room-presence"
  "login-leave-room"
  "login-recommendations"
  "login-user-recommendations"
  "login-similar-terms"
  "login-room-moderation"
  "peer-advanced-local"
  "login-privileges-social"
  "peer-folder-local"
  "login-privilege-messaging"
  "peer-legacy-local"
  "login-private-message"
  "login-user-state"
  "login-peer-address-connect"
  "login-message-users"
  "login-peer-message"
  "login-parent-distributed-control"
  "login-room-term-control"
  "login-global-room-control"
  "login-parent-disconnect-control"
  "login-private-room-membership-control"
  "login-text-control"
  "login-system-control"
  "login-s6-batch1-control"
  "login-s6-batch2-control"
  "login-s6-batch3-control"
  "login-legacy-room-operatorship-control"
  "login-legacy-distributed-control"
  "login-legacy-residual-control"
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
  cargo run -q -p soul-cli -- verify captures --run "${run_id}" --base-dir "${RED_DIR}" --mode "${VERIFY_MODE}"
done
