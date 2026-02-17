# PR 0030 - S9P I3 Runtime/Format Hardening

## Summary

This iteration executes I3 (`I3-T01..I3-T06`) under S9P with focus on deterministic runtime instrumentation and static/runtime synthesis updates for persistence-critical reverse work.

Primary outcomes:

1. Corrected arm64 absolute hook offsets in `frida/hooks/soulseek_io_trace.js` from static `nm` evidence.
2. Added deterministic Frida process selection by executable path (`--process-path-contains`) to avoid same-name attach ambiguity.
3. Hardened Frida teardown behavior for expected target-process exit (`script is destroyed` no longer causes false run failure).
4. Produced deterministic runtime I/O captures with non-trivial persistence signal (`writestring`, `saveData`, `DataSaver`).
5. Updated S9P architecture/format artifacts, capability/stage state, roadmap, and evidence ledger.

## Dependency Graph (I3)

- `I3-T01 -> I3-T02`
- `I3-T01 -> I3-T03`
- `I3-T02 -> I3-T04`
- `I3-T03 -> I3-T04`
- `I3-T02 -> I3-T05`
- `I3-T04 -> I3-T06`
- `I3-T05 -> I3-T06`

## Key Files

- Runtime hooks/tooling:
  - `frida/hooks/soulseek_io_trace.js`
  - `tools/runtime/frida_capture.py`
  - `tools/runtime/capture_harness.py`
  - `scripts/capture_golden.sh`
  - `tests/runtime/test_frida_capture.py`
- Static/runtime synthesis artifacts:
  - `analysis/re/official_architecture_map.json`
  - `analysis/re/official_file_format_map.json`
  - `evidence/ui_audit/decomp/runtime_hook_offsets_arm64.txt`
  - `docs/re/static/file-format-map.md`
- State/KB sync:
  - `TODO-CODEX.md`
  - `analysis/state/capability_registry.json`
  - `analysis/state/stage_registry.json`
  - `docs/state/roadmap.md`
  - `docs/state/verification-status.md`
  - `docs/state/project-status.md`
  - `docs/state/decompilation-status.md`
  - `docs/verification/evidence-ledger.md`

## Runtime Evidence Added

- Deterministic capture with attach disambiguation:
  - `captures/raw/20260217T010817Z-i3-t04-io-runtime-r5/manifest.raw.json` (local raw)
  - `captures/redacted/20260217T010817Z-i3-t04-io-runtime-r5/manifest.redacted.json`
  - `captures/redacted/20260217T010817Z-i3-t04-io-runtime-r5/io-events.redacted.jsonl`
- Observed persistence-relevant events:
  - `writestring` (high-volume writer path)
  - `mainwindow_time_to_save_data_enter`
  - `mainwindow_save_data_enter` / `mainwindow_save_data_leave`
  - `datasaver_save_enter`
  - `datasaver_save_to_file_enter`

## Validation

Executed:

1. `python3 scripts/kb_validate.py` ✅
2. `python3 -m unittest discover -s tests/state` ✅
3. `python3 -m unittest discover -s tests/runtime -p 'test_frida_capture.py'` ✅
4. `python3 -m unittest discover -s tests/runtime -p 'test_redaction.py'` ✅
5. `python3 -m zensical build -f zensical.toml` ✅
6. `scripts/run_diff_verify.sh` ❌ (pre-existing fixture mismatch in transfer fixtures)
7. `scripts/run_regression.sh` ❌ (fails at diff-verify stage with same pre-existing fixture mismatch)

Known pre-existing failing fixture set:

- `captures/fixtures/peer_transfer_request.hex`
- `captures/fixtures/peer_transfer_response.hex`

## Mandatory Review Loop - Round 1

### blockchain_protocol_engineer pass

- Focus: transfer semantics implications of runtime evidence and parity impact.
- Findings:
  1. Deterministic attach path was required to avoid wrong-process traces; fixed via `--process-path-contains`.
  2. Runtime signal now confirms transfer-store write path execution, reducing uncertainty for `FMT-TRANSFER-STATE`.
- Action: accepted and implemented in this PR.

### code_simplifier pass

- Focus: keep runtime selector/harness changes minimal and testable.
- Findings:
  1. Process-selection logic should be factored into testable helpers.
  2. Add unit tests for candidate ordering, path filter behavior, and fallback.
- Action: accepted and implemented (`candidate_pids`, `attach_target`, `tests/runtime/test_frida_capture.py`).

### web3_security_review_expert pass

- Focus: instrumentation safety and data handling.
- Findings:
  1. Redacted artifact policy preserved; no plaintext credentials were introduced.
  2. Runtime capture teardown should not mark false failure when target exits normally.
- Action: accepted and implemented (`script is destroyed` handling).

## Mandatory Review Loop - Round 2

### blockchain_protocol_engineer pass

- Re-check after fixes: no additional protocol-critical issues found.

### code_simplifier pass

- Re-check after helper extraction/tests: no additional simplification blockers found.

### web3_security_review_expert pass

- Re-check after teardown hardening: no additional security regressions found.

## Residual Risks

1. QSettings and QDataStream symbol hooks are still unresolved on the active specimen.
2. Transfer fixture mismatch in diff-verify remains unresolved and blocks full green closure gates.
3. Flim E2E payload completion (`bytes_written > 0`) remains open and deferred to downstream S9P tasks.
