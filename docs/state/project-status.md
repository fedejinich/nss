# Project Status

## Date

- 2026-02-14

## Current Phase

- Stage 2 complete: Core P2P MVP with KB-first contract over 25 core protocol messages.
- Stage 2R complete: runtime capture refresh and confidence promotion (`medium -> high`) for all 25 core messages.
- Stage 3A complete: authenticated login against official server (`160/1`) and semantic differential verification as default.
- Stage 3B complete: Rooms+Presence protocol batch with runtime evidence and CLI support.
- Stage 4A complete: Recommendations/Discovery batch with runtime-authenticated evidence and CLI support.
- Stage 4B complete: Peer advanced + room moderation batch with protocol matrix publication and runtime evidence.
- Product direction remains unchanged: SDK+CLI first, custom evolvable app (not a 1:1 official client clone).

## Stage 4B Completion

1. Added protocol matrix generation and published `docs/state/protocol-matrix.md` to track mapped/implemented/missing protocol coverage with purpose summaries.
2. Expanded protocol mapping from 38 to 47 rows (`+9` S4B messages):
   - `SM_ADD_ROOM_MEMBER`
   - `SM_REMOVE_ROOM_MEMBER`
   - `SM_ADD_ROOM_OPERATOR`
   - `SM_REMOVE_ROOM_OPERATOR`
   - `PM_USER_INFO_REQUEST`
   - `PM_USER_INFO_REPLY`
   - `PM_EXACT_FILE_SEARCH_REQUEST`
   - `PM_INDIRECT_FILE_SEARCH_REQUEST`
   - `PM_UPLOAD_PLACE_IN_LINE_REQUEST`
3. Added S4B runtime scenarios and redacted artifacts:
   - `login-room-moderation`
   - `peer-advanced-local`
4. Extended `rust/protocol` with S4B constants, payloads, codecs, and builders for room moderation and peer advanced messages.
5. Extended `rust/core` room operations with explicit moderation methods:
   - `add_room_member(...)`
   - `remove_room_member(...)`
   - `add_room_operator(...)`
   - `remove_room_operator(...)`
6. Extended `rust/cli` room command group:
   - `room add-member --room <name> --target-user <name>`
   - `room remove-member --room <name> --target-user <name>`
   - `room add-operator --room <name> --target-user <name>`
   - `room remove-operator --room <name> --target-user <name>`
7. Extended semantic verification required-run set to include S4B scenarios and kept all validation gates green.

## Stage 3B Completion (Reference)

1. Published visual roadmap for Zensical (`docs/state/roadmap.md`) with timeline and dependency graph.
2. Added 8-message rooms/presence protocol batch with runtime evidence.
3. Implemented room operations and room command group (`soul-cli room ...`).
4. Added mandatory S3B runtime redacted runs.

## Core Artifacts

- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`
- `docs/state/protocol-matrix.md`
- `docs/re/static/message-schema.md`
- `docs/verification/evidence-ledger.md`
- `docs/state/roadmap.md`
- `captures/redacted/login-room-moderation/manifest.redacted.json`
- `captures/redacted/peer-advanced-local/manifest.redacted.json`
- `tools/runtime/generate_stage4b_peer_room_captures.py`
- `tools/protocol/generate_protocol_matrix.py`

## Operational Notes

- Stage 2 core contract remains intact (`25/25`, `high=25`, `medium=0`, `low=0`).
- Total mapped protocol rows now: `47`.
- Runtime credentials remain local-only in `.env.local` and are never committed.
