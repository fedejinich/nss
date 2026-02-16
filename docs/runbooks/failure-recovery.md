# Failure Recovery Runbook

This runbook documents operator recovery actions for the minimal CLI/TUI delivery surface.

## 1. Login/auth failure

Symptoms:

1. Login returns `INVALIDVERSION`/`INVALIDPASS`/`INVALIDUSERNAME`.
2. TUI remains disconnected after `l` login key.

Actions:

1. Validate local env values:
   - `NSS_TEST_SERVER`
   - `NSS_TEST_USERNAME`
   - `NSS_TEST_PASSWORD`
2. Probe login tuples:

```bash
cd rust
cargo run -q -p soul-cli -- session probe-login-version
```

3. Retry login with accepted tuple.

## 2. Search returns no rows

Symptoms:

1. Search command succeeds but returns zero summaries.
2. TUI search panel stays empty.

Actions:

1. Change query term.
2. Increase collection window:
   - higher `--search-timeout-secs`
   - higher `--max-messages`
3. Verify authenticated session is still active.

## 3. Download orchestration failure

Symptoms:

1. `session download-auto` fails with peer lookup/connect/transfer error.
2. TUI logs a failed transfer row.

Actions:

1. Retry with deterministic local peer override:
   - `--peer 127.0.0.1:2242`
   - `--skip-connect-probe`
2. Run manual deterministic path (`transfer serve-upload` + `transfer download`) to isolate transport vs orchestration.
3. Confirm writable output path.
4. Validate public inbound port reachability (Frida-free):
   - `./.venv-tools/bin/python tools/runtime/check_slsk_porttest.py 50036 50037 2242 --json`
5. If queue grants are observed but file payload stays at zero bytes, run outbound handshake diagnostics:
   - `NSS_DEBUG_TRANSFER=1`
   - `NSS_SEND_CONNECT_TOKEN_ON_OUTBOUND_FILE_INIT=1`
   - `NSS_OUTBOUND_FILE_VARIANT_ORDER=f_init_first`
   - increase `NSS_TRANSFER_FLOW_TIMEOUT_SECS` and `NSS_QUEUE_WAIT_SECS` for live runs.

## 4. Transfer interruption

Symptoms:

1. Partial file written.
2. Socket closes during transfer.

Actions:

1. Delete incomplete output file.
2. Re-run with deterministic local transfer flow.
3. Capture and compare frames with `scripts/run_diff_verify.sh` if regression is suspected.

## 5. TUI responsiveness issues

Symptoms:

1. Key inputs appear unresponsive.
2. Screen render appears stale.

Actions:

1. Exit with `q`.
2. Relaunch:

```bash
cd rust
cargo run -q -p soul-tui
```

3. If terminal state remains corrupted, run:

```bash
reset
```

## Escalation Checklist

1. Reproduce with deterministic local transfer mode first.
2. Save redacted runtime evidence under `captures/redacted/<run_id>`.
3. Record findings in:
   - `docs/verification/evidence-ledger.md`
   - `docs/state/verification-status.md`
