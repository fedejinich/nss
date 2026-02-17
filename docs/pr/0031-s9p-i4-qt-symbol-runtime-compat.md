# PR 0031 - S9P I4 Qt Symbol Runtime Compatibility

## Summary

This iteration executes I4 (`I4-T01..I4-T06`) as a follow-up to I3, focused on restoring deterministic Qt symbol-hook behavior under the active macOS debug specimen.

Primary outcomes:

1. Added Frida export-lookup compatibility fallback (`findExportByName` + `getExportByName`) to prevent runtime API-version crashes.
2. Added safe module-symbol enumeration fallback and null-address filtering (`0x0`) to avoid invalid hook attaches.
3. Updated Qt symbol candidates to include underscore-prefixed arm64 variants (`_ZN...` and `__ZN...`) for `QSettings`, `QDataStream`, and `QFile::open` hooks.
4. Produced deterministic runtime captures confirming symbol-hook registration and runtime persistence signal (`qfile_open`, `writestring`) during official-runner login/search/download flow.
5. Added regression tests for hook-script symbol/fallback invariants and synchronized S9P planning/state artifacts.

## Dependency Graph (I4)

- `I4-T01 -> I4-T02`
- `I4-T01 -> I4-T03`
- `I4-T02 -> I4-T04`
- `I4-T03 -> I4-T04`
- `I4-T02 -> I4-T05`
- `I4-T04 -> I4-T06`
- `I4-T05 -> I4-T06`

## Key Files

- Runtime hooks/tooling:
  - `frida/hooks/soulseek_io_trace.js`
  - `tests/runtime/test_io_hook_script.py`
- Runtime evidence:
  - `captures/raw/20260217T024407Z-i4-t04-io-qt-symbol-r2/manifest.raw.json` (local raw)
  - `captures/raw/20260217T024407Z-i4-t04-io-qt-symbol-r2/io-events.raw.jsonl` (local raw)
  - `captures/raw/20260217T024511Z-i4-t04-io-qt-symbol-r3/manifest.raw.json` (local raw)
  - `captures/raw/20260217T024511Z-i4-t04-io-qt-symbol-r3/io-events.raw.jsonl` (local raw)
  - `captures/redacted/20260217T024407Z-i4-t04-io-qt-symbol-r2/redaction-summary.json`
  - `captures/redacted/20260217T024511Z-i4-t04-io-qt-symbol-r3/redaction-summary.json`
- State/KB sync:
  - `TODO-CODEX.md`
  - `analysis/state/capability_registry.json`
  - `analysis/state/stage_registry.json`
  - `docs/state/roadmap.md`
  - `docs/state/verification-status.md`
  - `docs/state/project-status.md`
  - `docs/state/decompilation-status.md`
  - `docs/verification/evidence-ledger.md`
  - generated dashboards from `scripts/sync_state_dashboards.sh`

## Runtime Evidence Highlights

- Run `20260217T024407Z-i4-t04-io-qt-symbol-r2`:
  - resolves prior script crash by using export/symbol fallback,
  - confirms `hook_registered` for `qsettings_*_symbol`, `qdatastream_*_symbol`, and `qfile_open_symbol`.
- Run `20260217T024511Z-i4-t04-io-qt-symbol-r3` (official-runner flow):
  - emits runtime `qfile_open` events,
  - emits high-volume `writestring` persistence events,
  - retains transfer-store save-path signal (`mainwindow_save_data_*`, `datasaver_save_*`).

## Validation

Executed:

1. `python3 -m unittest discover -s tests/runtime -p 'test_io_hook_script.py'` ✅
2. `python3 -m unittest discover -s tests/runtime -p 'test_frida_capture.py'` ✅
3. `python3 -m unittest discover -s tests/runtime -p 'test_redaction.py'` ✅
4. `python3 scripts/kb_validate.py` ✅
5. `python3 -m unittest discover -s tests/state` ✅
6. `python3 -m zensical build -f zensical.toml` ✅
7. `scripts/run_diff_verify.sh` ❌ (pre-existing fixture mismatch in transfer fixtures)
8. `scripts/run_regression.sh` ❌ (fails at diff-verify stage with the same pre-existing fixture mismatch)

Known pre-existing failing fixture set:

- `captures/fixtures/peer_transfer_request.hex`
- `captures/fixtures/peer_transfer_response.hex`

## Mandatory Review Loop - Round 1

### blockchain_protocol_engineer pass

- Focus: parity implications of Qt hook recovery and runtime evidence quality.
- Findings:
  1. Symbol-resolution failure mode is now removed, so runtime instrumentation reliability improved without altering protocol behavior.
  2. Evidence should explicitly separate "hook registered" from "hook invoked" for QSettings/QDataStream.
- Action: accepted and reflected in state/docs notes (invocation depth remains open gap).

### code_simplifier pass

- Focus: maintainability and resilience of hook resolver code.
- Findings:
  1. Shared helper functions are preferred over repeated direct API calls.
  2. Regression test should pin symbol variants and fallback strategy to prevent future regression.
- Action: accepted and implemented (`safeFindExportByName`, `enumerateModuleSymbols`, `test_io_hook_script.py`).

### web3_security_review_expert pass

- Focus: runtime instrumentation safety and sensitive-data handling.
- Findings:
  1. Null-pointer hook targets must be rejected to avoid unsafe attach attempts.
  2. Redaction workflow remains in place and no secret material was added to tracked artifacts.
- Action: accepted and implemented (`isNullAddress` guard + existing redaction pipeline preserved).

## Mandatory Review Loop - Round 2

### blockchain_protocol_engineer pass

- Re-check after updates: no additional protocol-critical regressions found.

### code_simplifier pass

- Re-check after helper consolidation/tests: no additional simplification blockers found.

### web3_security_review_expert pass

- Re-check after null-address guard: no additional security regressions found.

## Residual Risks

1. Deterministic scenarios still under-trigger `QSettings/QDataStream` invocation depth even though symbol hooks now resolve/register.
2. `run_diff_verify` fixture mismatch (`peer_transfer_request/response`) remains unresolved and blocks fully green regression closure.
3. Flim E2E payload completion (`bytes_written > 0`) remains open under downstream S9P transfer-parity tasks.
