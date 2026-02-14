# Protocol Backlog (Post S4A)

## Objective

Continue protocol mapping toward full coverage by functional domains while keeping KB-first evidence and runtime verification discipline.

## Completed in S3B (Rooms + Presence Batch)

- `SM_ROOM_LIST`
- `SM_JOIN_ROOM`
- `SM_LEAVE_ROOM`
- `SM_USER_JOINED_ROOM`
- `SM_USER_LEFT_ROOM`
- `SM_ROOM_MEMBERS`
- `SM_ROOM_OPERATORS`
- `SM_SAY_CHATROOM`

## S4 Candidate Batch A - Recommendations and Discovery

- `SM_GET_RECOMMENDATIONS`
- `SM_GET_MY_RECOMMENDATIONS`
- `SM_GET_GLOBAL_RECOMMENDATIONS`
- `SM_GET_USER_RECOMMENDATIONS`
- `SM_GET_SIMILAR_TERMS`

Status: completed in S4A with runtime-authenticated evidence and semantic verification coverage.

## S4B Candidate Batch - Peer Advanced

- `PM_USER_INFO_REQUEST`
- `PM_USER_INFO_REPLY`
- `PM_EXACT_FILE_SEARCH_REQUEST`
- `PM_INDIRECT_FILE_SEARCH_REQUEST`
- `PM_UPLOAD_PLACE_IN_LINE_REQUEST`

## S4B Candidate Batch - Room Moderation (Deferred from S3B)

- `SM_ADD_ROOM_MEMBER`
- `SM_REMOVE_ROOM_MEMBER`
- `SM_ADD_ROOM_OPERATOR`
- `SM_REMOVE_ROOM_OPERATOR`

## Execution Strategy

1. Add batch rows to `analysis/ghidra/maps/message_map.csv` with explicit confidence and evidence.
2. Collect runtime captures first for high-confidence promotion when feasible.
3. Regenerate schema/docs from authoritative maps:
   - `scripts/derive_message_schema.sh`
   - `python3 scripts/kb_sync_docs.py`
4. Extend SDK/CLI/verify only after mapping evidence is registered.
5. Keep regression green (`scripts/run_regression.sh`) before stage closure.

## Next Session Entry Point

S4B should start with peer-advanced request/response mapping because it unlocks richer peer feature parity while reusing the existing S3A/S3B/S4A semantic verification pipeline.
