#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SCENARIO="${SCENARIO:-login-search-download}"
SCENARIO_ID="${SCENARIO_ID:-${SCENARIO}}"
DURATION="${DURATION:-120}"
PROCESS_NAME="${PROCESS_NAME:-SoulseekQt}"
PROCESS_PATH_CONTAINS="${PROCESS_PATH_CONTAINS:-}"
IFACE="${IFACE:-}"
BPF_FILTER="${BPF_FILTER:-tcp}"
SKIP_PCAP="${SKIP_PCAP:-0}"
AUTO_REDACT="${AUTO_REDACT:-1}"
HOOK_SET="${HOOK_SET:-protocol}"
FRIDA_SCRIPT="${FRIDA_SCRIPT:-}"
IO_FRIDA_SCRIPT="${IO_FRIDA_SCRIPT:-}"
LAUNCH_BINARY="${LAUNCH_BINARY:-}"
PROFILE_ROOT="${PROFILE_ROOT:-}"
NOTES="${NOTES:-}"
STARTUP_DELAY="${STARTUP_DELAY:-1.0}"
RUNNER_CMD="${RUNNER_CMD:-}"
RUNNER_CWD="${RUNNER_CWD:-${ROOT_DIR}}"
OFFICIAL_RUNNER="${OFFICIAL_RUNNER:-0}"
RUNNER_SCENARIO="${RUNNER_SCENARIO:-${SCENARIO_ID}}"
RUNNER_APP_BUNDLE="${RUNNER_APP_BUNDLE:-/Applications/SoulseekQt.app}"
RUNNER_APP_NAME="${RUNNER_APP_NAME:-SoulseekQt}"
RUNNER_PROCESS_NAME="${RUNNER_PROCESS_NAME:-${PROCESS_NAME}}"
RUNNER_QUERY="${RUNNER_QUERY:-aphex twin flim}"
RUNNER_INITIAL_WAIT="${RUNNER_INITIAL_WAIT:-2.0}"
RUNNER_STEP_WAIT="${RUNNER_STEP_WAIT:-0.8}"
RUNNER_MANUAL_WAIT="${RUNNER_MANUAL_WAIT:-8.0}"
RUNNER_REQUIRE_ACCESSIBILITY="${RUNNER_REQUIRE_ACCESSIBILITY:-0}"
RUNNER_SKIP_LAUNCH="${RUNNER_SKIP_LAUNCH:-0}"
RUNNER_SKIP_QUIT="${RUNNER_SKIP_QUIT:-0}"
RUNNER_OUTPUT="${RUNNER_OUTPUT:-tmp/runtime/official_runner_last.json}"

runner_pid=""
cleanup() {
  if [ -n "${runner_pid}" ] && kill -0 "${runner_pid}" >/dev/null 2>&1; then
    kill "${runner_pid}" >/dev/null 2>&1 || true
    wait "${runner_pid}" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

if [ -z "${RUNNER_CMD}" ] && [ "${OFFICIAL_RUNNER}" = "1" ]; then
  case "${RUNNER_SCENARIO}" in
    login-only|login-search|login-search-download) ;;
    *)
      RUNNER_SCENARIO="login-search-download"
      ;;
  esac

  runner_args=(
    "python3"
    "${ROOT_DIR}/tools/runtime/official_runner.py"
    "--scenario" "${RUNNER_SCENARIO}"
    "--app-bundle" "${RUNNER_APP_BUNDLE}"
    "--app-name" "${RUNNER_APP_NAME}"
    "--process-name" "${RUNNER_PROCESS_NAME}"
    "--query" "${RUNNER_QUERY}"
    "--initial-wait" "${RUNNER_INITIAL_WAIT}"
    "--between-step-wait" "${RUNNER_STEP_WAIT}"
    "--manual-wait" "${RUNNER_MANUAL_WAIT}"
    "--output" "${RUNNER_OUTPUT}"
    "--notes" "capture_golden:${SCENARIO_ID}"
  )

  if [ "${RUNNER_REQUIRE_ACCESSIBILITY}" = "1" ]; then
    runner_args+=("--require-accessibility")
  fi
  if [ "${RUNNER_SKIP_LAUNCH}" = "1" ]; then
    runner_args+=("--skip-launch")
  fi
  if [ "${RUNNER_SKIP_QUIT}" = "1" ]; then
    runner_args+=("--skip-quit")
  fi

  RUNNER_CMD="$(printf "%q " "${runner_args[@]}")"
fi

args=(
  "${ROOT_DIR}/tools/runtime/capture_harness.py"
  "--process" "${PROCESS_NAME}"
  "--duration" "${DURATION}"
  "--output-root" "captures/raw"
  "--label" "${SCENARIO}"
  "--scenario-id" "${SCENARIO_ID}"
  "--hook-set" "${HOOK_SET}"
  "--bpf" "${BPF_FILTER}"
  "--startup-delay" "${STARTUP_DELAY}"
)

if [ -n "${PROCESS_PATH_CONTAINS}" ]; then
  args+=("--process-path-contains" "${PROCESS_PATH_CONTAINS}")
fi
if [ -n "${IFACE}" ]; then
  args+=("--iface" "${IFACE}")
fi
if [ -n "${LAUNCH_BINARY}" ]; then
  args+=("--launch-binary" "${LAUNCH_BINARY}")
fi
if [ -n "${PROFILE_ROOT}" ]; then
  args+=("--profile-root" "${PROFILE_ROOT}")
fi
if [ -n "${NOTES}" ]; then
  args+=("--notes" "${NOTES}")
fi
if [ -n "${FRIDA_SCRIPT}" ]; then
  args+=("--frida-script" "${FRIDA_SCRIPT}")
fi
if [ -n "${IO_FRIDA_SCRIPT}" ]; then
  args+=("--io-frida-script" "${IO_FRIDA_SCRIPT}")
fi
if [ "${SKIP_PCAP}" = "1" ]; then
  args+=("--skip-pcap")
fi

echo "starting golden capture scenario: ${SCENARIO}"
if [ -n "${RUNNER_CMD}" ]; then
  echo "starting scenario runner command"
  (
    cd "${RUNNER_CWD}"
    bash -lc "${RUNNER_CMD}"
  ) &
  runner_pid="$!"
fi

result="$(${ROOT_DIR}/.venv-tools/bin/python "${args[@]}")"
echo "${result}"

if [ "${AUTO_REDACT}" = "1" ]; then
  run_dir="$(python3 - <<'PY' "${result}"
import json,sys
print(json.loads(sys.argv[1])["run_dir"])
PY
)"
  "${ROOT_DIR}/.venv-tools/bin/python" "${ROOT_DIR}/tools/runtime/redact_capture_run.py" --run-dir "${run_dir}"
fi
