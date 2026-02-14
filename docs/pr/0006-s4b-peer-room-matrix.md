# PR 0006 - S4B: peer advanced + room moderation + protocol matrix

## Branch

- `codex/s4b-peer-advanced-room-moderation`

## Objective

Close Stage 4B with:

1. Peer advanced and room moderation protocol batch mapped and implemented.
2. Runtime evidence for S4B scenarios (official room moderation attempts + local deterministic peer advanced run).
3. Protocol matrix published for full coverage visibility (`implemented/mapped/missing`).
4. Semantic differential verification and regression coverage extended for S4B.
5. KB/docs/state synchronized for stage closure.

## Scope

### Protocol matrix

- Added generator:
  - `tools/protocol/generate_protocol_matrix.py`
- Published canonical matrix:
  - `docs/state/protocol-matrix.md`
- Added regression test:
  - `tests/protocol/test_protocol_matrix.py`

### Runtime capture and evidence

- Added generator:
  - `tools/runtime/generate_stage4b_peer_room_captures.py`
- Added redacted runs:
  - `captures/redacted/login-room-moderation`
  - `captures/redacted/peer-advanced-local`
- Extended required differential runs in `scripts/run_diff_verify.sh`.

### Mapping and schema

- Expanded `analysis/ghidra/maps/message_map.csv` with S4B batch:
  - `SM_ADD_ROOM_MEMBER`
  - `SM_REMOVE_ROOM_MEMBER`
  - `SM_ADD_ROOM_OPERATOR`
  - `SM_REMOVE_ROOM_OPERATOR`
  - `PM_USER_INFO_REQUEST`
  - `PM_USER_INFO_REPLY`
  - `PM_EXACT_FILE_SEARCH_REQUEST`
  - `PM_INDIRECT_FILE_SEARCH_REQUEST`
  - `PM_UPLOAD_PLACE_IN_LINE_REQUEST`
- Regenerated:
  - `analysis/protocol/message_schema.json`
  - `docs/re/static/message-schema.md`
  - `docs/re/static/detangling.md`
  - `docs/verification/evidence-ledger.md`

### Rust implementation

- `rust/protocol`:
  - Added S4B constants, payloads, enum variants, encode/decode logic, and builders.
  - Added tolerant parsing for peer legacy search requests and user-info optional fields.
- `rust/core`:
  - Added room moderation methods:
    - `add_room_member(...)`
    - `remove_room_member(...)`
    - `add_room_operator(...)`
    - `remove_room_operator(...)`
- `rust/cli`:
  - Added room moderation commands:
    - `room add-member --room <name> --target-user <name>`
    - `room remove-member --room <name> --target-user <name>`
    - `room add-operator --room <name> --target-user <name>`
    - `room remove-operator --room <name> --target-user <name>`

### Tests and contracts

- Added Stage 4B contract test:
  - `tests/protocol/test_stage4b_peer_room_contract.py`
- Extended Rust protocol/core tests for S4B message and room moderation flows.

## S4B Contract Outcome

Required 9-message pack:

1. `SM_ADD_ROOM_MEMBER`
2. `SM_REMOVE_ROOM_MEMBER`
3. `SM_ADD_ROOM_OPERATOR`
4. `SM_REMOVE_ROOM_OPERATOR`
5. `PM_USER_INFO_REQUEST`
6. `PM_USER_INFO_REPLY`
7. `PM_EXACT_FILE_SEARCH_REQUEST`
8. `PM_INDIRECT_FILE_SEARCH_REQUEST`
9. `PM_UPLOAD_PLACE_IN_LINE_REQUEST`

Confidence outcome:

- `high=7`
- `medium=2`
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
- S4B runtime scenarios:
  - `login-room-moderation` (authenticated server traffic including outbound codes 134/135/143/144)
  - `peer-advanced-local` (deterministic peer frames for codes 15/16/47/49/51)

## Retrospective

### Was there a more maintainable approach?

Yes. Making the protocol matrix generated from canonical inputs prevents manual drift and reduces repeated bookkeeping when new message batches are added.

### What was reused to avoid double writing?

1. Existing redaction and semantic diff pipeline for new S4B captures.
2. Existing schema/doc generation flow (`derive_schema.py` + `kb_sync_docs.py`).
3. Existing CLI session auth flow and room command structure for moderation operations.

### What was removed or avoided to reduce maintenance surface?

1. Avoided hand-maintained protocol coverage tables by introducing generated `protocol-matrix.md`.
2. Avoided S4B-specific diff scripts by extending `scripts/run_diff_verify.sh` required runs.
3. Avoided ad-hoc payload parsing in CLI by keeping all new decoding logic in `rust/protocol`.
