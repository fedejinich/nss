# Verification Status

## Objective

Ensure evidence traceability and semantic protocol parity for Stage 4D (privilege/messaging gaps + peer legacy cleanup) while preserving all previous stage guarantees.

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

## Stage 4D Coverage Status

S4D 11-message contract set (9 new + 2 promoted) is present in:

- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`

Messages:

- `SM_BAN_USER`
- `SM_PRIVILEGED_LIST`
- `SM_GET_RECOMMENDED_USERS`
- `SM_GET_TERM_RECOMMENDATIONS`
- `SM_GET_RECOMMENDATION_USERS`
- `PM_INVITE_USER_TO_ROOM`
- `PM_CANCELLED_QUEUED_TRANSFER`
- `PM_MOVE_DOWNLOAD_TO_TOP`
- `PM_QUEUED_DOWNLOADS`
- `PM_EXACT_FILE_SEARCH_REQUEST` (promoted)
- `PM_INDIRECT_FILE_SEARCH_REQUEST` (promoted)

Confidence distribution for the S4D contract set:

- `high=11`
- `medium=0`
- `low=0`

Protocol matrix status:

- Tracked message names from static string tables: `130`
- Implemented + mapped: `65`
- Missing: `65`
- Matrix source: `docs/state/protocol-matrix.md`

## Runtime Evidence Snapshot

- Official server: `server.slsknet.org:2242`
- Auth tuple used: `160/1`
- S4D runtime redacted runs:
  - `captures/redacted/login-privilege-messaging`
  - `captures/redacted/peer-legacy-local`
- Privilege/messaging runtime scenario includes authenticated request/response frames for codes `69`, `110`, `111`, `112`, and outbound `132`.
- Peer-legacy deterministic runtime scenario includes frames `10`, `14`, `34`, `48`, `47`, `49`.
- Authoritative static mapping for `SM_BAN_USER` is backed by jump-table extraction:
  - `evidence/reverse/message_codes_jump_table.md`

## Residual Risk

- `SM_GET_USER_PRIVILEGES_STATUS` remains `medium` from S4C because code `122` is deprecated in public specs and behavior can vary by server implementation.
- Several protocol names in string tables remain unmapped (`65` missing in matrix); S4E should prioritize private messaging and user-state domains.
- `PM_SHARED_FILES_IN_FOLDER` response payload is still represented as `directory + compressed bytes`; deep decompression schema remains a follow-up parser task.
