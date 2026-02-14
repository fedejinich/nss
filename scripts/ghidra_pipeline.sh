#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DMG_PATH="${DMG_PATH:-${ROOT_DIR}/SoulseekQt-2025-10-11.dmg}"
MOUNT_POINT="${MOUNT_POINT:-/tmp/soulseek_dmg_mount}"
APP_BIN_REL="SoulseekQt.app/Contents/MacOS/SoulseekQt"
EXTRACTED_BIN="${ROOT_DIR}/analysis/binaries/SoulseekQt"
PROJECT_DIR="${ROOT_DIR}/analysis/ghidra/project"
PROJECT_NAME="${PROJECT_NAME:-soulseekqt}"
PROGRAM_NAME="${PROGRAM_NAME:-SoulseekQt}"

if [ ! -f "${DMG_PATH}" ]; then
  echo "DMG not found: ${DMG_PATH}" >&2
  exit 1
fi

if ! command -v brew >/dev/null 2>&1; then
  echo "brew not found; cannot resolve ghidra installation path." >&2
  exit 1
fi

GHIDRA_PREFIX="$(brew --prefix ghidra 2>/dev/null || true)"
if [ -z "${GHIDRA_PREFIX}" ]; then
  echo "ghidra is not installed. install via: brew install ghidra" >&2
  exit 1
fi

GHIDRA_HEADLESS="${GHIDRA_PREFIX}/libexec/support/analyzeHeadless"
if [ ! -x "${GHIDRA_HEADLESS}" ]; then
  echo "analyzeHeadless not found at ${GHIDRA_HEADLESS}" >&2
  exit 1
fi

mkdir -p "${ROOT_DIR}/analysis/binaries" "${PROJECT_DIR}" "${ROOT_DIR}/analysis/ghidra/tmp"

hdiutil attach -readonly -nobrowse -mountpoint "${MOUNT_POINT}" "${DMG_PATH}" >/tmp/soulseek_attach_ghidra.log 2>&1
cp "${MOUNT_POINT}/${APP_BIN_REL}" "${EXTRACTED_BIN}"
hdiutil detach "${MOUNT_POINT}" >/tmp/soulseek_detach_ghidra.log 2>&1

"${GHIDRA_HEADLESS}" "${PROJECT_DIR}" "${PROJECT_NAME}" \
  -import "${EXTRACTED_BIN}" \
  -overwrite

echo "Ghidra headless import+analysis completed for ${PROGRAM_NAME}."
echo "Extracted binary: ${EXTRACTED_BIN}"
echo "Project temp dir: ${PROJECT_DIR}"
