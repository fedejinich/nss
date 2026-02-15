# PR 0023 - S6D: opaque-tail typed batches (S6-Batch-1/2/3)

## Branch

- `codex/s6d-opaque-tail-typed-batches`

## Objective

Execute S6 typed promotion for the full opaque-tail closure by implementing S6-Batch-1/2/3 with runtime-backed evidence and updated KB/state artifacts.

## Scope

1. Replace remaining `OPAQUE_SERVER_CONTROL_CODES` handling with typed payload variants.
2. Add runtime capture generation for S6 batch families and commit redacted artifacts.
3. Update canonical protocol maps/schema/evidence with payload fields and runtime links.
4. Regenerate protocol/state dashboards and closure reports.
5. Add/adjust regression tests for typed branches and opaque-tail closure.

## Outcome

1. Added typed server payloads and codec coverage for all S6 batches:
   1. `S6-Batch-1`: `41, 61, 67, 70`
   2. `S6-Batch-2`: `71, 73, 82, 93, 102`
   3. `S6-Batch-3`: `114, 115, 116, 138, 141, 142`
2. Closed generic opaque server-tail handling:
   1. `OPAQUE_SERVER_CONTROL_CODES = []`
   2. `docs/state/opaque-tail-report.json` now reports `opaque_tail_count=0`.
3. Added runtime capture generation script:
   1. `tools/runtime/generate_stage6_typed_batches_captures.py`
4. Added redacted runtime runs:
   1. `captures/redacted/login-s6-batch1-control/`
   2. `captures/redacted/login-s6-batch2-control/`
   3. `captures/redacted/login-s6-batch3-control/`
5. Updated KB and protocol artifacts:
   1. `analysis/ghidra/maps/message_map.csv`
   2. `analysis/protocol/message_schema.json`
   3. `docs/verification/evidence-ledger.md`
   4. `docs/re/static/detangling.md`
   5. state/roadmap/status/backlog/dashboard artifacts.
6. Fixed Zensical route navigation to avoid `.md` 404 links:
   1. route-first nav in `zensical.toml`
   2. route-first links in `docs/index.md` and `docs/state/roadmap.md`
   3. route-first links in generated `docs/pr/index.md`.

## Validation

```bash
python3 scripts/kb_validate.py
scripts/run_diff_verify.sh
scripts/run_regression.sh
./.venv-tools/bin/zensical build -f zensical.toml
```

Observed result:

1. Passed.

## Local Review Loops

1. Round 1 security pass:
   - Completed.
   - Checked for plaintext credential leakage, debug leftovers, and unsafe logging on new runtime/protocol paths.
   - Confirmed no secrets are written into committed redacted artifacts.
2. Round 1 code-simplifier pass:
   - Completed.
   - Reduced nested conditionals in typed parser paths and kept behavior unchanged.
   - Added clippy-clean simplifications in `rust/protocol/src/lib.rs`.
3. Round 2 security + simplifier pass:
   - Completed.
   - Re-ran static checks and strict clippy on protocol crate:
     - `cargo clippy -p protocol --manifest-path rust/Cargo.toml -- -D warnings` (pass).

## Retrospective

1. More maintainable approach:
   1. Typed payload structs replaced generic opaque tails for S6 control families.
   2. Runtime-backed captures are now part of the promotion path, reducing schema ambiguity.
2. Reuse to avoid double writing:
   1. Reused existing schema derivation and KB sync scripts.
   2. Reused diff-verify pipeline by adding S6 runs to required run set.
   3. Reused PR index generation workflow while switching links to route-safe output.
3. Surface reduction:
   1. Removed generic opaque-tail fallback for server control codes.
   2. Kept only explicit typed decode/encode paths for S6 promoted codes.
