# Verification Status

## Objective

Ensure evidence traceability and semantic protocol parity baseline while preserving Stage 4L full-coverage closure, Stage 5A/S5C typed hardening waves, Stage 5B UI/feature research verification, Stage 5D-S5H multi-wave opaque-to-typed runtime promotion, Stage 6A dashboard-state observability artifacts, Stage 6B executable closure gates, Stage 6C opaque-tail baseline observability, Stage 6D typed-batch opaque-tail closure, Stage 6E dedicated legacy opaque reduction, Stage 6F dedicated residual semantic closure, Stage 7 runtime/semantic strict-closure gates, Stage 8 dashboard/TUI rollout gates, and Stage 9 TUI-first simplification gates.

## Validation Gates

### KB validation

```bash
python3 scripts/kb_validate.py
```

Checks:

- Name/data maps contain valid evidence.
- `message_map.csv` has valid source links and confidence fields.
- `message_schema.json` has valid evidence links and schema integrity.

### Differential verification

```bash
scripts/run_diff_verify.sh
```

Runs:

1. Fixture parity (`captures/fixtures/*`).
2. Runtime redacted capture parity for mandatory scenarios:
   - `login-only`
   - `login-search`
   - `login-search-download`
   - `upload-deny`
   - `upload-accept`
   - `login-room-list`
   - `login-join-room-presence`
   - `login-leave-room`
   - `login-recommendations`
   - `login-user-recommendations`
   - `login-similar-terms`
   - `login-room-moderation`
   - `peer-advanced-local`
   - `login-privileges-social`
   - `peer-folder-local`
   - `login-privilege-messaging`
   - `peer-legacy-local`
   - `login-private-message`
   - `login-user-state`
   - `login-peer-address-connect`
   - `login-message-users`
   - `login-peer-message`
   - `login-parent-distributed-control`
   - `login-room-term-control`
   - `login-global-room-control`
   - `login-parent-disconnect-control`
   - `login-private-room-membership-control`
   - `login-text-control`
   - `login-system-control`
   - `login-s6-batch1-control`
   - `login-s6-batch2-control`
   - `login-s6-batch3-control`
   - `login-legacy-room-operatorship-control`
   - `login-legacy-distributed-control`
   - `login-legacy-residual-control`
   - `login-static-server-runtime`
   - `peer-static-runtime`
   - `login-partial-tail-runtime`
   - `login-search-download-auto`
3. Default mode is semantic (`VERIFY_MODE=semantic`) with bytes mode compatibility.

### Full regression

```bash
scripts/run_regression.sh
```

Includes:

1. Python unit tests (`tests/kb`, `tests/protocol`, `tests/runtime`).
2. Rust unit/integration tests (`cargo test`).
3. KB validation gate.
4. Differential verification gate.
5. Zensical build check (if available).

### Release hardening audit

```bash
python3 tools/state/verify_release_hardening.py
```

Checks:

1. No absolute-path metadata in committed redacted captures.
2. Required release runbooks and packaging script exist.
3. Final closure checklist has no unchecked items.
4. Release-hardening capabilities are marked done with no blockers.

## Stage 4L Coverage Status

S4L closure set is present in:

- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`

Messages:

- all prior `mapped_not_implemented` rows (`40`) were promoted to `implemented_mapped` via protocol constants plus decode/encode support.

Confidence distribution for the S4L closure set:

- `high=40`
- `medium=0`
- `low=0`

Protocol matrix status:

- Tracked message names from static string tables: `131`
- Implemented + mapped: `131`
- Mapped not implemented: `0`
- Missing: `0`
- Matrix source: `docs/state/protocol-matrix.md`

## Runtime Evidence Snapshot

- Official server: `server.slsknet.org:2242`
- Auth tuple used: `160/1`
- S4E runtime redacted runs:
  - `captures/redacted/login-private-message`
  - `captures/redacted/login-user-state`
  - `captures/redacted/login-peer-address-connect`
  - `captures/redacted/login-message-users`
  - `captures/redacted/login-peer-message`
- Private messaging runtime scenarios include code `22` and `23` paths with directional payload decoding.
- User-state runtime scenario includes code `7` and `36` request/response payloads.
- Peer-address/connect scenario includes code `3` and `18` request/response payloads.
- Message-users scenario includes code `149`.
- Peer-message deterministic scenario includes code `68` plus compatibility alias `292`.
- Stage 5A authenticated runtime scenario `login-parent-distributed-control` provides runtime evidence for codes `83`, `84`, `113`, `121`, and `122`.
- Stage 5C authenticated runtime scenario `login-room-term-control` provides runtime evidence for codes `10`, `51`, and `52`.
- Stage 5D authenticated runtime scenario `login-global-room-control` provides runtime evidence for codes `150`, `151`, `152`, and `153`.
- Stage 5E authenticated runtime scenario `login-parent-disconnect-control` provides runtime evidence for codes `86`, `87`, `88`, `90`, and `100`.
- Stage 5F authenticated runtime scenario `login-private-room-membership-control` provides runtime evidence for codes `136`, `137`, `139`, `140`, and `145`.
- Stage 5G authenticated runtime scenario `login-text-control` provides runtime evidence for codes `58`, `62`, `63`, `66`, `117`, and `118`.
- Stage 5H authenticated runtime scenario `login-system-control` provides runtime evidence for codes `28`, `32`, and `130`.
- Stage 6E authenticated runtime scenario `login-legacy-room-operatorship-control` provides runtime evidence for codes `146` and `147`.
- Stage 6E authenticated runtime scenario `login-legacy-distributed-control` provides runtime evidence for codes `126`, `127`, `128`, `129`, and `131`.
- Stage 7A authenticated runtime closure scenarios:
  - `login-static-server-runtime`
  - `peer-static-runtime`
- Stage 7B authenticated runtime semantic-tail scenario:
  - `login-partial-tail-runtime`
- Stage 7C authenticated orchestration runtime scenario:
  - `login-search-download-auto`
- `SM_GET_USER_PRIVILEGES_STATUS` was promoted to `high` with authenticated request/response runtime evidence.
- `PM_SHARED_FILES_IN_FOLDER` parser now has decompression-aware coverage with zlib safety limits and typed listing classification.
- Stage 4F/S4G/S4H/S4I/S4J are mapping-first and static-evidence-driven via jump-table extraction (`evidence/reverse/message_codes_jump_table.md`).
- Stage 4K closes all prior `missing` names by adding jump-table-backed mappings and protocol codec support.
- Stage 4L closes the prior `mapped_not_implemented` bucket using `OpaqueServerControlPayload` coverage for unresolved runtime-shape control messages.

## Stage 6A State Artifact Verification

- Stage-state source of truth:
  - `analysis/state/stage_registry.json`
- Generated state artifacts:
  - `docs/state/project-dashboard-data.json`
  - `docs/state/codebase-graph.json`
  - `docs/pr/index.md`
- Regeneration command:
  - `scripts/sync_state_dashboards.sh`
- Validation coverage:
  - `tests/state/test_stage_registry.py`
  - `tests/state/test_dashboard_generators.py`
  - `tests/docs/test_pr_index.py`
- `run_regression.sh` includes all above tests and remains green.

## Stage 6B S5A Closure Verification

- Closure verifier:
  - `tools/state/verify_s5a_closure.py`
- Generated report:
  - `docs/state/s5a-closure-audit.json`
  - `docs/state/s5a-closure-audit.md`
- Enforced objectives:
  1. Opaque -> typed runtime evidence closure for S5A target message set.
  2. Runtime capture closure for `login-parent-distributed-control` and `login-global-room-control`.
  3. Decompression-aware parser and guard-test closure for `PM_SHARED_FILES_IN_FOLDER`.
  4. Residual hypothesis closure for `SM_GET_USER_PRIVILEGES_STATUS` and `SM_UPLOAD_SPEED`.
- Regression coverage:
  - `tests/state/test_s5a_closure_audit.py`
- Workflow integration:
  - `scripts/sync_state_dashboards.sh` now regenerates closure report together with dashboard artifacts.

## Stage 6C Opaque-Tail Baseline Verification

- Opaque-tail report generator:
  - `tools/state/report_opaque_tail.py`
- Generated report:
  - `docs/state/opaque-tail-report.json`
- Published execution plan:
  - `docs/state/opaque-tail-plan.md`
- Regression coverage:
  - `tests/state/test_opaque_tail_report.py`
- Workflow integration:
  - `scripts/sync_state_dashboards.sh` now regenerates opaque-tail report in the same run.
- Current baseline:
  - `opaque_tail_count=0` (`OPAQUE_SERVER_CONTROL_CODES`)
  - generic opaque-tail closure completed by S6D typed batches.

## Stage 6D Typed-Batch Closure Verification

- Runtime capture generator:
  - `tools/runtime/generate_stage6_typed_batches_captures.py`
- Redacted runtime runs:
  - `captures/redacted/login-s6-batch1-control`
  - `captures/redacted/login-s6-batch2-control`
  - `captures/redacted/login-s6-batch3-control`
- Typed promotion scope:
  - `41`, `61`, `67`, `70`, `71`, `73`, `82`, `93`, `102`, `114`, `115`, `116`, `138`, `141`, `142`
- Batch-3 caveat:
  - server-side connection resets occurred after login; attempted outbound probe frames are retained and explicitly marked in run metadata.

## Stage 6E Dedicated Legacy Opaque Reduction Verification

- Runtime capture generator:
  - `tools/runtime/generate_stage6e_legacy_control_captures.py`
- Redacted runtime runs:
  - `captures/redacted/login-legacy-room-operatorship-control`
  - `captures/redacted/login-legacy-distributed-control`
- Typed promotion scope:
  - `SM_REMOVE_ROOM_OPERATORSHIP` (`146`)
  - `SM_REMOVE_OWN_ROOM_OPERATORSHIP` (`147`)
  - `SM_DNET_LEVEL` (`126`)
  - `SM_DNET_GROUP_LEADER` (`127`)
  - `SM_DNET_CHILD_DEPTH` (`129`)

## Stage 6F Dedicated Residual Semantic Closure Verification

- Runtime capture generator:
  - `tools/runtime/generate_stage6f_residual_captures.py`
- Redacted runtime runs:
  - `captures/redacted/login-legacy-residual-control`
- Typed closure scope:
  - `SM_DNET_DELIVERY_REPORT` (`128`) -> `report: optional_u32`, `extension_reserved_bytes`
  - `SM_FLOOD` (`131`) -> `flood_code: optional_u32`, `extension_reserved_bytes`
- Static reinforcement:
  - `evidence/ui_audit/decomp/server_methods.txt` (`Server::DNetDeliveryReport(int)`)

## Stage 7 Runtime/Semantic Closure Verification

- Runtime closure contract:
  - `tests/protocol/test_stage7_runtime_semantic_contract.py`
- Runtime coverage artifact:
  - `docs/state/runtime-coverage.json`
  - `docs/state/runtime-coverage.md`
- Current closure snapshot:
  - `verified_runtime=131`
  - `verified_static=0`
  - unresolved `raw_tail/raw_payload` schema fields = `0`

## Stage 8 Dashboard and TUI Verification

- Capability artifacts:
  - `docs/state/capability-matrix.json`
  - `docs/state/capability-matrix.md`
  - `docs/state/capability-dashboard.html`
- TUI implementation and runbook:
  - `rust/tui/src/main.rs`
  - `docs/runbooks/tui-core-transfer.md`
- TUI baseline now includes runtime query edit controls for minimal operator search/download flow.

## Stage 8C Release Hardening Verification

- Release hardening audit:
  - `tools/state/verify_release_hardening.py`
  - `docs/state/release-hardening-audit.json`
  - `docs/state/release-hardening-audit.md`
- Packaging workflow:
  - `scripts/package_release.sh`
  - `docs/runbooks/release-packaging.md`
- Failure recovery workflow:
  - `docs/runbooks/failure-recovery.md`
- Final closure checklist:
  - `docs/state/final-closure-checklist.md`
- Redaction metadata policy:
  - `docs/verification/capture-redaction-policy.md` (absolute metadata paths are forbidden in committed redacted artifacts)

## Stage 9A TUI-First Simplification Verification

- Module split and bootstrap:
  - `rust/tui/src/main.rs`
  - `rust/tui/src/app.rs`
  - `rust/tui/src/ui.rs`
  - `rust/tui/src/state.rs`
  - `rust/tui/src/storage.rs`
- Validation scope:
  - mandatory login gate before any search/download action
  - orange-dominant retro UI palette and simplified layout
  - downloads panel `show/hide` and `clear history` semantics
  - persisted local state with file permission target `0600`
  - startup recovery (`in_progress` -> `interrupted`)
  - explicit typed auth error for server-close-before-response login failures
  - in-TUI diagnostics wizard (`g`) for server format, DNS, TCP connect, and login probe checks
- Tests:
  - `cargo test -p soul-tui`
  - persistence/recovery and login-gating assertions in `rust/tui` unit tests

## Stage 5B Verification Status

- Feature inventory artifact: `docs/state/soulseek-feature-inventory.md`
- Baseline evidence bundle: `evidence/ui_audit/*`
- Structured external-source snapshots:
  - `evidence/ui_audit/external/changelog_structured.json`
  - `evidence/ui_audit/external/news_structured.json`
  - `evidence/ui_audit/external/forum_topics_structured.json`
- Static handler/protocol bridge evidence:
  - `evidence/reverse/ui_handler_symbols_nm.txt`
  - `evidence/ui_audit/decomp/mainwindow_methods.txt`
  - `evidence/ui_audit/decomp/server_methods.txt`
  - `evidence/ui_audit/decomp/peer_methods.txt`
  - `evidence/ui_audit/decomp/transfer_methods.txt`
- Pass-2 closure:
  - `verified_pass2=41`
  - `gap_found=1`

## Residual Risk

- `SM_PEER_MESSAGE` compatibility alias `292` is implemented as decode-only fallback and still needs corroboration from authenticated server runtime.
- Generic opaque server-control closure is now `0`; dedicated legacy residual semantic closure is complete for S6.
- Stage 5B still has one UI-runtime visibility gap: live menu tree extraction requires macOS assistive-access permission (captured denial: `evidence/ui_audit/ui_menu_bar_items.err`).
