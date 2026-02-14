# PR 0004 - S3B: visual roadmap + rooms/presence protocol batch

## Branch

- `codex/s3a-auth-login-semantic-parity`

## Objective

Close Stage 3B with:

1. Visual roadmap published in Zensical for `S2 -> S2R -> S3A -> S3B -> S4 preview`.
2. Rooms/presence 8-message batch mapped with runtime evidence.
3. SDK/CLI support for room operations and room event collection.
4. Semantic differential verification updated and green with S3B runtime runs.
5. KB/docs/status fully synchronized.

## Scope

### Roadmap and status

- Added `docs/state/roadmap.md` with:
  - Mermaid timeline/gantt
  - Mermaid dependency graph
  - Stage status matrix
- Linked roadmap from `docs/index.md`.
- Updated status docs:
  - `docs/state/project-status.md`
  - `docs/state/verification-status.md`
  - `docs/state/protocol-backlog.md`

### Runtime capture and evidence

- Added reproducible generator:
  - `tools/runtime/generate_stage3b_room_captures.py`
- Generated raw + redacted S3B runs:
  - `captures/raw/login-room-list`
  - `captures/raw/login-join-room-presence`
  - `captures/raw/login-leave-room`
  - `captures/redacted/login-room-list`
  - `captures/redacted/login-join-room-presence`
  - `captures/redacted/login-leave-room`
- Added semantic verification reports for each redacted run.

### Protocol and SDK/CLI

- `rust/protocol`:
  - Added room/presence constants and payload types.
  - Extended `ServerMessage` with 8 new variants.
  - Added room/presence parsing helpers and request builders.
- `rust/core`:
  - Added room operations:
    - `list_rooms(...)`
    - `join_room(...)`
    - `leave_room(...)`
    - `collect_room_events(...)`
  - Added `RoomEvent` typed model.
- `rust/cli`:
  - Added room command group:
    - `soul-cli room list`
    - `soul-cli room join --room <name>`
    - `soul-cli room leave --room <name>`
    - `soul-cli room members --room <name>`
    - `soul-cli room watch --room <name> --timeout-secs <n>`
  - Summary output by default; `--verbose` prints decoded payload details.
- `rust/verify`:
  - Semantic comparison now covers S3B messages through typed protocol decoding.

### KB/canonical artifacts

- Updated `analysis/ghidra/maps/message_map.csv` with the S3B 8-message pack.
- Regenerated:
  - `analysis/protocol/message_schema.json`
  - `docs/re/static/message-schema.md`
  - `docs/verification/evidence-ledger.md`
- Added S3B contract tests:
  - `tests/protocol/test_stage3b_rooms_contract.py`

## S3B 8-Message Contract Status

- `SM_ROOM_LIST`
- `SM_JOIN_ROOM`
- `SM_LEAVE_ROOM`
- `SM_USER_JOINED_ROOM`
- `SM_USER_LEFT_ROOM`
- `SM_ROOM_MEMBERS`
- `SM_ROOM_OPERATORS`
- `SM_SAY_CHATROOM`

Batch confidence outcome:

- `high=8`
- `medium=0`
- `low=0`

## Validation

```bash
python3 scripts/kb_validate.py
scripts/run_diff_verify.sh
scripts/run_regression.sh
```

All checks passed for this branch snapshot.

## Runtime Evidence Snapshot

- Authenticated room flow captured against official server (`160/1` auth tuple).
- Verified runtime command execution in summary mode:
  - `room list`
  - `room join`
  - `room members`
  - `room watch`
  - `room leave`

## Retrospective

### Was there a more maintainable approach?

Yes. Reusing shared runtime socket helpers and extending existing protocol abstractions was significantly more maintainable than introducing parallel room-only parsing stacks.

### What was reused to avoid double writing?

1. Existing framing/reader/writer abstractions in `rust/protocol` for new room payloads.
2. Existing semantic verifier pipeline in `rust/verify`, extended by typed decode rather than custom per-run compare logic.
3. Existing redaction and KB sync tooling (`redact_capture_run.py`, `kb_sync_docs.py`, `derive_schema.py`) instead of manual artifact editing.

### What was removed to reduce maintenance surface?

1. Removed the need for ad-hoc room decoding in CLI by centralizing decoding in `rust/protocol` and event mapping in `rust/core`.
2. Avoided introducing a separate room-specific verification script by extending `scripts/run_diff_verify.sh` required runs.
3. Avoided manual schema/doc drift by keeping `message_map.csv` as source of truth and regenerating downstream artifacts.
