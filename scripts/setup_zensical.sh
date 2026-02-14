#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VENV_DIR="${ROOT_DIR}/.venv-tools"
PYTHON_BIN="${PYTHON_BIN:-python3}"

# Pin source to a known commit from github.com/zensical/zensical
ZENSICAL_GIT_REF="${ZENSICAL_GIT_REF:-d9462af}"
ZENSICAL_SRC="git+https://github.com/zensical/zensical@${ZENSICAL_GIT_REF}"
ZENSICAL_VERSION_FALLBACK="${ZENSICAL_VERSION_FALLBACK:-0.0.23}"

"${PYTHON_BIN}" -m venv "${VENV_DIR}"
"${VENV_DIR}/bin/python" -m pip install --upgrade pip setuptools wheel
"${VENV_DIR}/bin/python" -m pip install "${ZENSICAL_SRC}"

# The GitHub source can be missing bundled templates depending on upstream
# packaging state. If so, fall back to a pinned wheel release.
THEME_DIR="$("${VENV_DIR}/bin/python" - <<'PY'
import zensical.config as c
print(c.get_theme_dir())
PY
)"
if [ ! -d "${THEME_DIR}" ]; then
  echo "zensical templates missing from git build, applying pinned PyPI fallback ${ZENSICAL_VERSION_FALLBACK}" >&2
  "${VENV_DIR}/bin/python" -m pip install --force-reinstall "zensical==${ZENSICAL_VERSION_FALLBACK}"
fi

"${VENV_DIR}/bin/zensical" --version
