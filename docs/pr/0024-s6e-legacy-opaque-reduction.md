# PR 0024 - S6E: legacy dedicated opaque variant reduction

## Branch

- `codex/s6e-legacy-opaque-reduction`

## Objective

Reduce remaining dedicated legacy opaque server payload variants with runtime-backed typing while keeping protocol closure and semantic parity gates green.

## Scope

1. Resolve and capture payload evidence for:
   - `SM_REMOVE_ROOM_OPERATORSHIP (146)`
   - `SM_REMOVE_OWN_ROOM_OPERATORSHIP (147)`
   - `SM_DNET_LEVEL (126)`
   - `SM_DNET_GROUP_LEADER (127)`
   - `SM_DNET_CHILD_DEPTH (129)`
2. Keep explicit residual tracking for unresolved variants when evidence is insufficient:
   - `SM_DNET_DELIVERY_REPORT (128)`
   - `SM_FLOOD (131)`
3. Promote typed protocol handling in `rust/protocol` for evidence-backed variants.
4. Refresh schema/maps/state docs and regenerate dashboard artifacts.
5. Run mandatory blocking review loops:
   - `blockchain_protocol_engineer`
   - `code_simplifier`
   - `web3_security_review_expert`

## Runtime Artifacts

1. `captures/redacted/login-legacy-room-operatorship-control`
2. `captures/redacted/login-legacy-distributed-control`

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
   - `login-legacy-room-operatorship-control`
   - `login-legacy-distributed-control`
3. `scripts/run_regression.sh`: pass.
4. `./.venv-tools/bin/zensical build -f zensical.toml`: pass.

## Mandatory Blocking Review Loops

1. Round 1:
   - `blockchain_protocol_engineer`: accepted promotion of `126/127/129/146/147` to typed payloads and explicit residual handling of `128/131`.
   - `code_simplifier`: kept parsing/encoding centralized in `rust/protocol/src/lib.rs`, added dedicated builders, and avoided duplicate ad-hoc parse paths.
   - `web3_security_review_expert`: confirmed no plaintext credential logging was introduced; runtime artifacts remain redacted-only in repo.
2. Round 2:
   - `blockchain_protocol_engineer`: no additional protocol-shape corrections required.
   - `code_simplifier`: applied dashboard route hardening to absolute site paths and added regression coverage for the route behavior.
   - `web3_security_review_expert`: no further findings after final gate run.

## Retrospective

1. Was there a more maintainable approach?
   - Yes. Using typed payload structs with `raw_tail` preserved backward compatibility while allowing progressive field-accuracy upgrades without duplicate message variants.
2. What did we reuse to avoid double writing?
   - Reused existing schema/doc sync pipeline (`derive_schema.py`, `kb_sync_docs.py`, `sync_state_dashboards.sh`) rather than hand-editing generated artifacts.
   - Reused semantic diff harness and only extended required run lists.
3. What did we remove to reduce maintenance surface?
   - Removed dedicated S6E targets from generic opaque handling by promoting proven payload families to typed decode/encode paths.
   - Reduced dashboard link fragility by switching evidence routing to absolute site paths and adding a regression test.
