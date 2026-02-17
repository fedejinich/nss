# Runtime Capture Runbook

## Goal

Collect synchronized runtime evidence for core flows and keep only redacted artifacts in git.

## Capture Model

- Raw local-only run: `captures/raw/<run_id>/`
- Redacted commit-safe run: `captures/redacted/<run_id>/`

Each redacted run should contain:

- `manifest.redacted.json`
- `frida-events.redacted.jsonl` (if available)
- `official_frames.hex`
- `neo_frames.hex`

## Prerequisites

1. SoulseekQt running locally.
2. Tooling initialized with `scripts/setup_toolchain.sh`.
3. Packet capture permission for `tcpdump` on macOS.

## Standard Session

```bash
scripts/capture_session.sh
```

Environment variables:

- `DURATION` (default `60`)
- `PROCESS_NAME` (default `SoulseekQt`)
- `IFACE` (optional, auto when empty)
- `BPF_FILTER` (default `tcp`)
- `STARTUP_DELAY` (default `1.0`, reduce for early-start hook capture)
- `PROCESS_PATH_CONTAINS` (optional executable path token to disambiguate same-name processes during Frida attach)
- `SKIP_PCAP=1` for Frida-only capture
- `AUTO_REDACT=0` to skip automatic redaction

## Scenario Session

```bash
SCENARIO=login-search-download DURATION=120 scripts/capture_golden.sh
```

Official runner mode (`osascript`-driven login/search/download actions):

```bash
OFFICIAL_RUNNER=1 \
RUNNER_SCENARIO=login-search-download \
RUNNER_QUERY="aphex twin flim" \
RUNNER_REQUIRE_ACCESSIBILITY=1 \
scripts/capture_golden.sh
```

Useful runner options:

- `RUNNER_SKIP_LAUNCH=1` when `LAUNCH_BINARY` is managed by the harness.
- `RUNNER_SKIP_QUIT=1` to avoid runner-sent `Cmd+Q`.
- `RUNNER_OUTPUT=tmp/runtime/official_runner_last.json` to persist runner step results.
- `RUNNER_INITIAL_WAIT` / `RUNNER_STEP_WAIT` to tune action pacing.
- `PROCESS_PATH_CONTAINS=/Users/void/Applications/SoulseekQt-Debug.app` to force Frida attach to the debug specimen when both signed and debug variants are present.

Mandatory scenarios for stage 2:

1. `login-only`
2. `login-search`
3. `login-search-download`
4. `upload-deny`
5. `upload-accept`

S9P runtime note:

- When Frida attach is blocked by hardened-runtime policy on the signed original app, use the instrumentation specimen path `/Users/void/Applications/SoulseekQt-Debug.app` and keep `/Applications/SoulseekQt.app` unchanged as source-of-truth.

## Runtime Scenario Generator (Stage 2R)

For deterministic runtime scenario generation (server socket + local peer flows):

```bash
python3 tools/runtime/generate_stage2_real_captures.py \
  --server server.slsknet.org:2242 \
  --username <user> \
  --password <plain_password> \
  --client-version 160 \
  --minor-version 1
```

Then redact:

```bash
for run in login-only login-search login-search-download upload-deny upload-accept; do
  ./.venv-tools/bin/python tools/runtime/redact_capture_run.py \
    --run-dir captures/raw/$run --run-id $run --out-root captures/redacted
done
```

## Manual Redaction

```bash
RUN_DIR=captures/raw/<run_id> scripts/redact_capture_run.sh
```

Optional:

- `RUN_ID=<new_id>` to rename output run
- `OUT_ROOT=captures/redacted` to change target root

## Verification

```bash
scripts/run_diff_verify.sh
```

This verifies:

1. Fixture parity in `captures/fixtures`.
2. Per-run parity for mandatory runs in `captures/redacted/*`.

Optional explicit mode:

```bash
VERIFY_MODE=bytes scripts/run_diff_verify.sh
```

## Policy

See `docs/verification/capture-redaction-policy.md`.
