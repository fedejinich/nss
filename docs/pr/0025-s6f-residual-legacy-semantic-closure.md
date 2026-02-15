# PR 0025 - S6F: dedicated residual legacy semantic closure

## Branch

- `codex/s6f-residual-legacy-semantic-closure`

## Objective

Close the final dedicated residual payload ambiguities by promoting:

1. `SM_DNET_DELIVERY_REPORT` (`128`)
2. `SM_FLOOD` (`131`)

from dedicated opaque handling to typed payload handling with runtime/static evidence, while keeping semantic parity gates green and protocol matrix closure stable.

## Scope

1. Add dedicated runtime probe captures for residual semantics.
2. Promote protocol decode/encode paths for codes `128` and `131` to typed payloads.
3. Update message maps/schema/docs and backlog status.
4. Run full gates and mandatory two-round local review loop:
   - `blockchain_protocol_engineer`
   - `code_simplifier`
   - `web3_security_review_expert`

## Runtime Artifacts

1. `captures/redacted/login-legacy-residual-control`

## Validation

```bash
python3 scripts/kb_validate.py
scripts/run_diff_verify.sh
scripts/run_regression.sh
./.venv-tools/bin/zensical build -f zensical.toml
```

Observed result:

1. `python3 scripts/kb_validate.py`: pass.
2. `scripts/run_diff_verify.sh`: pass in semantic mode for all required runs, including:
   - `login-legacy-residual-control`
3. `scripts/run_regression.sh`: pass.
4. `./.venv-tools/bin/zensical build -f zensical.toml`: pass.

## Mandatory Blocking Review Loops

1. Round 1:
   - `blockchain_protocol_engineer`: accepted typed closure shape for `128/131` with conservative `optional_u32 + raw_tail` layout and runtime evidence.
   - `code_simplifier`: shared the `optional_u32 + raw_tail` parser helper to avoid duplicated parsing logic in `rust/protocol`.
   - `web3_security_review_expert`: validated local-only credential handling and redacted-only committed runtime artifacts.
2. Round 2:
   - `blockchain_protocol_engineer`: no additional protocol-shape corrections after schema/docs synchronization.
   - `code_simplifier`: no further simplification needed after helper refactor and builder consistency pass.
   - `web3_security_review_expert`: no additional findings after full gate rerun.

## Retrospective

1. Was there a more maintainable approach?
   - Yes. Promoting residual payloads through one shared parse strategy (`optional_u32 + raw_tail`) kept behavior explicit without inventing speculative field schemas.
2. What did we reuse to avoid double writing?
   - Reused Stage 6 runtime probe pattern and redaction pipeline (`raw -> redacted -> semantic verify`) instead of creating a parallel capture workflow.
   - Reused schema/doc generation pipeline (`derive_message_schema.sh`, `kb_sync_docs.py`, `sync_state_dashboards.sh`) for canonical updates.
3. What did we remove to reduce maintenance surface?
   - Removed final dedicated opaque decode branches for `128/131` in favor of typed variants.
   - Removed “residual unresolved” status from roadmap/backlog/verification docs for S6 scope.
