#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VENV_DIR="${ROOT_DIR}/.venv-tools"

if command -v brew >/dev/null 2>&1; then
  brew install ghidra
fi

"${VENV_DIR}/bin/python" -m pip install --upgrade frida frida-tools

if command -v ghidra >/dev/null 2>&1; then
  ghidra --help >/dev/null 2>&1 || true
fi

"${VENV_DIR}/bin/frida" --version
"${VENV_DIR}/bin/frida-trace" --version
if command -v tcpdump >/dev/null 2>&1; then
  tcpdump --version | head -n 1
fi
