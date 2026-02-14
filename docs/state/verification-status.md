# Verification Status

## Objective

Ensure evidence traceability and semantic protocol parity baseline while expanding Stage 4F (global/admin/distributed-control) mapping coverage.

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

## Stage 4F Coverage Status

S4F 8-message mapping contract set is present in:

- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`

Messages:

- `SM_COMMAND`
- `SM_ADMIN_MESSAGE`
- `SM_GLOBAL_USER_LIST`
- `SM_SEND_DISTRIBUTIONS`
- `SM_NOTE_PARENT`
- `SM_CHILD_PARENT_MAP`
- `SM_DNET_MESSAGE`
- `SM_DNET_RESET`

Confidence distribution for the S4F contract set:

- `high=8`
- `medium=0`
- `low=0`

Protocol matrix status:

- Tracked message names from static string tables: `131`
- Implemented + mapped: `67`
- Mapped not implemented: `8`
- Missing: `55`
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
- Stage 4F is mapping-first and currently static-evidence-driven via jump-table extraction (`evidence/reverse/message_codes_jump_table.md`).

## Residual Risk

- `SM_GET_USER_PRIVILEGES_STATUS` remains `medium` from S4C because code `122` is deprecated in public specs and behavior can vary by server implementation.
- `SM_PEER_MESSAGE` compatibility alias `292` is implemented as decode-only fallback and still needs corroboration from authenticated server runtime.
- S4F currently adds mapped-not-implemented rows; typed codec/core/CLI support for this domain remains a follow-up stage objective.
- Several protocol names in string tables remain unmapped (`55` missing in matrix); S4G should prioritize parent/distributed tuning and global control continuation.
- `PM_SHARED_FILES_IN_FOLDER` response payload is still represented as `directory + compressed bytes`; deep decompression schema remains a follow-up parser task.
