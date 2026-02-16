#!/usr/bin/env bash
set -euo pipefail

DEST_ROOT="${1:-$HOME/Projects/nss}"
DEFAULT_SIDE_TGZ="$DEST_ROOT/handoff/s9a-next-hardwall-20260216/nss-handoff-artifacts.tgz"
SIDE_TGZ="${2:-$DEFAULT_SIDE_TGZ}"

if [ ! -f "$SIDE_TGZ" ]; then
  echo "missing sidecar tgz: $SIDE_TGZ" >&2
  exit 1
fi

if [ ! -d "$DEST_ROOT/.git" ]; then
  git clone https://github.com/fedejinich/nss.git "$DEST_ROOT"
fi

cd "$DEST_ROOT"
git fetch --all --prune
git checkout main
git pull --ff-only

mkdir -p tmp/logs
mkdir -p /tmp/nss-handoff-artifacts
tar -xzf "$SIDE_TGZ" -C /tmp
cp /tmp/nss-handoff-artifacts/*.log tmp/logs/

if [ -f .env.local ]; then chmod 600 .env.local; fi

echo "== parity checks =="
git status -sb
rg -n "NSS_SEND_CONNECT_TOKEN_ON_OUTBOUND_FILE_INIT|NSS_OUTBOUND_FILE_VARIANT_ORDER|offset\+token" rust/core/src/lib.rs

cd rust
cargo test -q -p soul-core outbound_transfer_variants_continue_after_zero_byte_attempt -- --nocapture
cargo test -q -p soul-core file_transfer_offset_then_token_init_writes_expected_order -- --nocapture
cargo build -q -p soul-cli

cd "$DEST_ROOT"
echo
echo "Next commands (H9):"
echo "DURATION=120 SCENARIO=s9a-hardwall-official-search-r3 scripts/capture_golden.sh"
echo "scripts/run_diff_verify.sh"
