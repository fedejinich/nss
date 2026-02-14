# Verification Status

## Objective

Ensure evidence traceability and semantic protocol parity for Stage 4B (Peer advanced + room moderation batch) while preserving all previous stage guarantees.

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

## Stage 4B Coverage Status

S4B 9-message batch is present in:

- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`

Messages:

- `SM_ADD_ROOM_MEMBER`
- `SM_REMOVE_ROOM_MEMBER`
- `SM_ADD_ROOM_OPERATOR`
- `SM_REMOVE_ROOM_OPERATOR`
- `PM_USER_INFO_REQUEST`
- `PM_USER_INFO_REPLY`
- `PM_EXACT_FILE_SEARCH_REQUEST`
- `PM_INDIRECT_FILE_SEARCH_REQUEST`
- `PM_UPLOAD_PLACE_IN_LINE_REQUEST`

Confidence distribution for S4B batch:

- `high=7`
- `medium=2`
- `low=0`

Protocol matrix status:

- Tracked message names from static string tables: `130`
- Implemented + mapped: `47`
- Missing: `83`
- Matrix source: `docs/state/protocol-matrix.md`

## Runtime Evidence Snapshot

- Official server: `server.slsknet.org:2242`
- Auth tuple used: `160/1`
- S4B runtime redacted runs:
  - `captures/redacted/login-room-moderation`
  - `captures/redacted/peer-advanced-local`
- Room moderation runtime scenario includes authenticated outbound frames for codes `134`, `135`, `143`, `144`.
- Peer advanced deterministic runtime scenario includes peer frames `15`, `16`, `47`, `49`, `51`.

## Residual Risk

- `PM_EXACT_FILE_SEARCH_REQUEST` and `PM_INDIRECT_FILE_SEARCH_REQUEST` remain `medium` confidence until live runtime captures are available from peers emitting those legacy flows.
- `PM_USER_INFO_REPLY` optional tail fields can vary by client version; parser intentionally tolerates optional trailing fields for compatibility.
