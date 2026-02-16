#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SERVER="${NSS_TEST_SERVER:-server.slsknet.org:2416}"
USERNAME="${1:-${NSS_TEST_USERNAME:-}}"
PASSWORD="${2:-${NSS_TEST_PASSWORD:-}}"
QUERY="${NSS_TEST_QUERY:-aphex twin flim}"
SEARCH_MODE="${NSS_TEST_SEARCH_MODE:-distributed}"
STRICT_TRACK="${NSS_TEST_STRICT_TRACK:-flim}"
RESULT_INDEX="${NSS_TEST_RESULT_INDEX:-0}"
FILE_INDEX="${NSS_TEST_FILE_INDEX:-0}"
OUTPUT_PATH="${NSS_TEST_OUTPUT_PATH:-$ROOT_DIR/tmp/live-downloads/flim-$(date +%Y%m%d%H%M%S).bin}"

if [[ -z "$USERNAME" || -z "$PASSWORD" ]]; then
  echo "usage: $0 <username> <password>" >&2
  echo "or set NSS_TEST_USERNAME/NSS_TEST_PASSWORD in env." >&2
  exit 1
fi

mkdir -p "$(dirname "$OUTPUT_PATH")"

export NSS_LIVE_TEST=1
export NSS_TEST_SERVER="$SERVER"
export NSS_TEST_USERNAME="$USERNAME"
export NSS_TEST_PASSWORD="$PASSWORD"
export NSS_TEST_QUERY="$QUERY"
export NSS_TEST_SEARCH_MODE="$SEARCH_MODE"
export NSS_TEST_STRICT_TRACK="$STRICT_TRACK"
export NSS_TEST_RESULT_INDEX="$RESULT_INDEX"
export NSS_TEST_FILE_INDEX="$FILE_INDEX"
export NSS_TEST_OUTPUT_PATH="$OUTPUT_PATH"

echo "Running live test against ${NSS_TEST_SERVER} as user ${NSS_TEST_USERNAME}"
echo "Query: ${NSS_TEST_QUERY} | mode=${NSS_TEST_SEARCH_MODE} strict_track=${NSS_TEST_STRICT_TRACK} | result_index=${NSS_TEST_RESULT_INDEX} file_index=${NSS_TEST_FILE_INDEX}"

cd "$ROOT_DIR/rust"
cargo test -q -p soul-cli --test live_login_download_flim -- --ignored --nocapture

echo "Live test passed. Download saved to ${NSS_TEST_OUTPUT_PATH}"
