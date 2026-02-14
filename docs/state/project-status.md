# Project Status

## Date

- 2026-02-14

## Current Phase

- Stage 2 complete: Core P2P MVP with KB-first contract over 25 core protocol messages.
- Stage 2R complete: runtime capture refresh and confidence promotion (`medium -> high`) for all 25 core messages.
- Stage 3A complete: authenticated login against official server (`160/1`) and semantic differential verification as default.
- Stage 3B complete on this branch: Rooms+Presence 8-message protocol batch, runtime captures, SDK/CLI support, and semantic verification coverage.
- Product direction remains: evolvable custom app (SDK+CLI first), not a 1:1 clone of the official client.

## Stage 3B Completion

1. Published visual roadmap page for Zensical (`docs/state/roadmap.md`) with timeline and dependency graph.
2. Added runtime-authenticated S3B scenarios:
   - `login-room-list`
   - `login-join-room-presence`
   - `login-leave-room`
3. Expanded protocol mapping from 25 to 33 messages (`+8` rooms/presence messages):
   - `SM_ROOM_LIST`
   - `SM_JOIN_ROOM`
   - `SM_LEAVE_ROOM`
   - `SM_USER_JOINED_ROOM`
   - `SM_USER_LEFT_ROOM`
   - `SM_ROOM_MEMBERS`
   - `SM_ROOM_OPERATORS`
   - `SM_SAY_CHATROOM`
4. Implemented typed room/presence protocol handling in `rust/protocol` and stateful room operations in `rust/core`.
5. Added `soul-cli room` command group in `rust/cli`:
   - `room list`
   - `room join`
   - `room leave`
   - `room members`
   - `room watch`
6. Extended semantic verification coverage for room/presence messages through typed decoding.
7. Validation gates are green (`kb_validate`, `run_diff_verify`, `run_regression`).

## Stage 3A Completion (Reference)

1. Auth runtime tuple accepted by official server: `160/1` (`client_version/minor_version`).
2. Login request migrated to `username + password + client_version + md5hash + minor_version`.
3. Session auth is stateful: `LoggedIn` only after valid `LoginResponsePayload::Success`.
4. CLI/runtime moved to plain `--password`; `--password-md5` is explicitly deprecated.
5. Semantic differential verification is default in `scripts/run_diff_verify.sh`.

## Core Artifacts

- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`
- `docs/re/static/message-schema.md`
- `docs/verification/evidence-ledger.md`
- `docs/state/roadmap.md`
- `captures/redacted/login-room-list/manifest.redacted.json`
- `captures/redacted/login-join-room-presence/manifest.redacted.json`
- `captures/redacted/login-leave-room/manifest.redacted.json`
- `tools/runtime/generate_stage3b_room_captures.py`

## Operational Notes

- Repository-level protocol confidence now reflects 33 mapped rows, all `high` in the current map.
- Stage 2 core coverage contract remains intact (`25/25`, `high=25`, `medium=0`, `low=0`).
- Runtime credentials remain local-only in `.env.local` and are never committed.
