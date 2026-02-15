# Verification Status

## Objective

Ensure evidence traceability and semantic protocol parity baseline while expanding Stage 4K (missing-code closure + global/distributed peer-control) implementation coverage.

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

## Stage 4K Coverage Status

S4K closure set is present in:

- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`

Messages:

- `SM_ADD_USER`
- `SM_REMOVE_USER`
- `SM_SEND_CONNECT_TOKEN`
- `SM_PLACE_IN_LINE`
- `SM_PLACE_IN_LINE_RESPONSE`
- `SM_ADD_PRIVILEGED_USER`
- `SM_LOW_PRIORITY_FILE_SEARCH`
- `SM_WISHLIST_WAIT`
- `SM_DNET_LEVEL`
- `SM_DNET_GROUP_LEADER`
- `SM_DNET_DELIVERY_REPORT`
- `SM_DNET_CHILD_DEPTH`
- `SM_FLOOD`
- `SM_REMOVE_ROOM_OPERATORSHIP`
- `SM_REMOVE_OWN_ROOM_OPERATORSHIP`
- `SM_JOIN_GLOBAL_ROOM`
- `SM_LEAVE_GLOBAL_ROOM`
- `SM_SAY_GLOBAL_ROOM`
- `SM_SEARCH_CORRELATIONS`
- `SM_PEER_MESSAGE_ALT`
- `PM_SAY`
- `PM_SEND_CONNECT_TOKEN`
- `PM_PLACEHOLD_UPLOAD`
- `PM_NOTHING`

Confidence distribution for the S4K contract set:

- `high=24`
- `medium=0`
- `low=0`

Protocol matrix status:

- Tracked message names from static string tables: `131`
- Implemented + mapped: `91`
- Mapped not implemented: `40`
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
- Stage 4F/S4G/S4H/S4I/S4J are mapping-first and static-evidence-driven via jump-table extraction (`evidence/reverse/message_codes_jump_table.md`).
- Stage 4K closes all prior `missing` names by adding jump-table-backed mappings and protocol codec support.

## Residual Risk

- `SM_GET_USER_PRIVILEGES_STATUS` remains `medium` from S4C because code `122` is deprecated in public specs and behavior can vary by server implementation.
- `SM_PEER_MESSAGE` compatibility alias `292` is implemented as decode-only fallback and still needs corroboration from authenticated server runtime.
- Remaining risk concentration shifted from mapping gaps to typed behavior depth: `mapped_not_implemented=40` still requires staged protocol/core/CLI promotion.
- S4K introduced `OpaquePayload` handling for uncertain runtime-shape messages; runtime captures are still needed to replace opaque decoding with fully typed payload schemas.
- `PM_SHARED_FILES_IN_FOLDER` response payload is still represented as `directory + compressed bytes`; deep decompression schema remains a follow-up parser task.
