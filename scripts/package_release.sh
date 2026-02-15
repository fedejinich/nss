#!/usr/bin/env bash
set -euo pipefail
export LC_ALL=C

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUST_DIR="${ROOT_DIR}/rust"

REF="$(git -C "${ROOT_DIR}" rev-parse --short HEAD)"
TS="$(date -u +%Y%m%d-%H%M%S)"
VERSION="${1:-v1-${TS}-${REF}}"
OUT_DIR="${ROOT_DIR}/dist/releases/${VERSION}"
BIN_DIR="${OUT_DIR}/bin"

mkdir -p "${BIN_DIR}"

echo "[package_release] building release binaries..."
(
  cd "${RUST_DIR}"
  cargo build --release -p soul-cli -p soul-tui
)

cp "${RUST_DIR}/target/release/soul-cli" "${BIN_DIR}/"
cp "${RUST_DIR}/target/release/soul-tui" "${BIN_DIR}/"

cat > "${OUT_DIR}/README.txt" <<EOF
NeoSoulSeek Release Bundle
version: ${VERSION}
git_ref: ${REF}
generated_utc: ${TS}

Included binaries:
- bin/soul-cli
- bin/soul-tui

Validation:
- sha256 checksums in SHA256SUMS.txt
EOF

(
  cd "${OUT_DIR}"
  shasum -a 256 bin/soul-cli bin/soul-tui > SHA256SUMS.txt
)

echo "[package_release] bundle created at ${OUT_DIR}"
