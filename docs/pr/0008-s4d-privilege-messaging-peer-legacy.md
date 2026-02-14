# PR 0008 - S4D: privilege/messaging gaps + peer legacy cleanup

## Branch

- `codex/s4d-privilege-messaging-peer-legacy`

## Objective

Close Stage 4D with:

1. Authoritative mapping and implementation of the S4D 9-message batch.
2. Runtime promotion of 2 legacy peer-search messages from medium to high confidence.
3. Runtime evidence refresh for S4D scenarios.
4. Semantic differential verification required-run expansion.
5. KB/docs/state synchronization for stage closure.

## Scope

### Authoritative code resolution

- Added deterministic extractor:
  - `tools/re/extract_message_codes.py`
- Added static extraction artifacts:
  - `evidence/reverse/message_codes_jump_table.json`
  - `evidence/reverse/message_codes_jump_table.md`
- Resolved key codes including:
  - `SM_BAN_USER = 132`
  - `SM_PRIVILEGED_LIST = 69`
  - `SM_GET_RECOMMENDED_USERS = 110`
  - `SM_GET_TERM_RECOMMENDATIONS = 111`
  - `SM_GET_RECOMMENDATION_USERS = 112`
  - `PM_INVITE_USER_TO_ROOM = 10`
  - `PM_CANCELLED_QUEUED_TRANSFER = 14`
  - `PM_MOVE_DOWNLOAD_TO_TOP = 34`
  - `PM_QUEUED_DOWNLOADS = 48`

### Runtime capture and evidence

- Added generator:
  - `tools/runtime/generate_stage4d_privilege_legacy_captures.py`
- Added redacted runs:
  - `captures/redacted/login-privilege-messaging`
  - `captures/redacted/peer-legacy-local`
- Extended required differential runs in `scripts/run_diff_verify.sh`:
  - `login-privilege-messaging`
  - `peer-legacy-local`

### Mapping and schema

- Expanded `analysis/ghidra/maps/message_map.csv` with S4D batch:
  - `SM_BAN_USER`
  - `SM_PRIVILEGED_LIST`
  - `SM_GET_RECOMMENDED_USERS`
  - `SM_GET_TERM_RECOMMENDATIONS`
  - `SM_GET_RECOMMENDATION_USERS`
  - `PM_INVITE_USER_TO_ROOM`
  - `PM_CANCELLED_QUEUED_TRANSFER`
  - `PM_MOVE_DOWNLOAD_TO_TOP`
  - `PM_QUEUED_DOWNLOADS`
- Promoted runtime confidence:
  - `PM_EXACT_FILE_SEARCH_REQUEST` (`medium -> high`)
  - `PM_INDIRECT_FILE_SEARCH_REQUEST` (`medium -> high`)
- Regenerated:
  - `analysis/protocol/message_schema.json`
  - `docs/re/static/message-schema.md`
  - `docs/re/static/detangling.md`
  - `docs/verification/evidence-ledger.md`
  - `docs/state/protocol-matrix.md`

### Rust implementation

- `rust/protocol`:
  - Added S4D constants, payloads, enum variants, encode/decode logic, and builders for privilege/messaging + peer-legacy messages.
- `rust/core`:
  - Added S4D operations:
    - `ban_user(...)`
    - `get_privileged_list(...)`
    - `get_recommended_users(...)`
    - `get_term_recommendations(...)`
    - `get_recommendation_users(...)`
- `rust/cli`:
  - Added session commands:
    - `session ban-user --target-user <name>`
    - `session privileged-list`
  - Added discover commands:
    - `discover recommended-users`
    - `discover term-recommendations --term <value>`
    - `discover recommendation-users --term <value>`

### Tests and contracts

- Added Stage 4D contract test:
  - `tests/protocol/test_stage4d_privilege_legacy_contract.py`
- Extended Rust protocol/core tests for S4D message builders, decode roundtrip, and session operations.

## S4D Contract Outcome

Required 9-message pack:

1. `SM_BAN_USER`
2. `SM_PRIVILEGED_LIST`
3. `SM_GET_RECOMMENDED_USERS`
4. `SM_GET_TERM_RECOMMENDATIONS`
5. `SM_GET_RECOMMENDATION_USERS`
6. `PM_INVITE_USER_TO_ROOM`
7. `PM_CANCELLED_QUEUED_TRANSFER`
8. `PM_MOVE_DOWNLOAD_TO_TOP`
9. `PM_QUEUED_DOWNLOADS`

Runtime-promotion carryover:

1. `PM_EXACT_FILE_SEARCH_REQUEST`
2. `PM_INDIRECT_FILE_SEARCH_REQUEST`

Confidence outcome for contract set:

- `high=11`
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
- S4D runtime scenarios:
  - `login-privilege-messaging` (authenticated request/response traffic covering `69`, `110`, `111`, `112` plus outbound `132`)
  - `peer-legacy-local` (deterministic peer frames covering `10`, `14`, `34`, `48`, `47`, `49`)

## Retrospective

### Was there a more maintainable approach?

Yes. Resolving unknown codes with a reusable jump-table extractor is more maintainable than ad-hoc/manual disassembly notes and prevents repeated one-off reverse work.

### What was reused to avoid double writing?

1. Existing redaction + semantic diff pipeline for new S4D captures.
2. Existing schema/doc generation flow (`scripts/derive_message_schema.sh`, `scripts/kb_sync_docs.py`, protocol matrix generator).
3. Existing authenticated session wiring in `rust/core`/`rust/cli` for incremental command extension.

### What was removed or avoided to reduce maintenance surface?

1. Avoided custom stage-specific verification scripts by extending `scripts/run_diff_verify.sh`.
2. Avoided CLI-local decoders by centralizing protocol decode/encode in `rust/protocol`.
3. Avoided speculative confidence promotion; every promotion is tied to static/runtime evidence in canonical artifacts.
