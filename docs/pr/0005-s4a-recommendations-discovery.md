# PR 0005 - S4A: recommendations/discovery protocol batch

## Branch

- `codex/s4a-recommendations-discovery`

## Objective

Close Stage 4A with:

1. Runtime-authenticated recommendations/discovery evidence.
2. Protocol mapping expansion and typed codec coverage for the 5-message S4A contract.
3. SDK/CLI support for discovery operations.
4. Semantic differential verification coverage for the new batch.
5. KB/docs/state synchronization for stage closure.

## Scope

### Runtime evidence and captures

- Added reproducible generator:
  - `tools/runtime/generate_stage4a_discovery_captures.py`
- Generated and committed redacted runs:
  - `captures/redacted/login-recommendations`
  - `captures/redacted/login-user-recommendations`
  - `captures/redacted/login-similar-terms`
- Added S4A run requirements to `scripts/run_diff_verify.sh`.

### Protocol mapping and schema

- Updated `analysis/ghidra/maps/message_map.csv` with:
  - `SM_GET_SIMILAR_TERMS` (`50`)
  - `SM_GET_RECOMMENDATIONS` (`54`)
  - `SM_GET_MY_RECOMMENDATIONS` (`55`)
  - `SM_GET_GLOBAL_RECOMMENDATIONS` (`56`)
  - `SM_GET_USER_RECOMMENDATIONS` (`57`)
- Regenerated:
  - `analysis/protocol/message_schema.json`
  - `docs/re/static/message-schema.md`
  - `docs/verification/evidence-ledger.md`
  - `docs/re/static/detangling.md`

### Rust implementation

- `rust/protocol`:
  - Added discovery constants, payload structs, typed `ServerMessage` variants, request builders, and decode helpers.
- `rust/core`:
  - Added `SessionClient` operations for recommendations/discovery flows.
- `rust/cli`:
  - Added `discover` command group:
    - `discover recommendations`
    - `discover mine`
    - `discover global`
    - `discover user --target-user <name>`
    - `discover similar-terms --term <term>`
- `rust/verify`:
  - Extended semantic comparison coverage for discovery payloads.

### Tests and contracts

- Added protocol contract test:
  - `tests/protocol/test_stage4a_discovery_contract.py`
- Added/extended Rust unit tests for discovery codec/core/semantic behavior.

## S4A Contract Outcome

Required batch:

1. `SM_GET_SIMILAR_TERMS`
2. `SM_GET_RECOMMENDATIONS`
3. `SM_GET_MY_RECOMMENDATIONS`
4. `SM_GET_GLOBAL_RECOMMENDATIONS`
5. `SM_GET_USER_RECOMMENDATIONS`

Confidence outcome:

- `high=5`
- `medium=0`
- `low=0`

## Validation

```bash
python3 scripts/kb_validate.py
scripts/run_diff_verify.sh
scripts/run_regression.sh
```

All checks passed on this branch snapshot.

## Runtime Verification Snapshot

- Official server: `server.slsknet.org:2242`
- Auth tuple: `160/1`
- Runtime command validation succeeded:
  - `discover recommendations`
  - `discover mine`
  - `discover global`
  - `discover user --target-user <name>`
  - `discover similar-terms --term <term>`

## Retrospective

### Was there a more maintainable approach?

Yes. Extending the existing protocol/core/verify abstractions kept discovery support incremental and avoided parallel parsing stacks.

### What was reused to avoid double writing?

1. Existing framed message encode/decode path in `rust/protocol`.
2. Existing semantic verifier architecture in `rust/verify`.
3. Existing KB regeneration pipeline (`derive_schema.py`, `kb_sync_docs.py`) for canonical docs.
4. Existing runtime capture + redaction flow with a stage-specific generator wrapper.

### What was removed or avoided to reduce maintenance surface?

1. Avoided ad-hoc CLI-only payload parsing by centralizing typed parsing in protocol/core.
2. Avoided a separate verification script by extending the current required-run pipeline.
3. Avoided manual schema/doc edits by preserving `message_map.csv` as authoritative input.
