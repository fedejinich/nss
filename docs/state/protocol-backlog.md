# Protocol Backlog (Post S3B)

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

## S4 Candidate Batch B - Peer Advanced

- `PM_USER_INFO_REQUEST`
- `PM_USER_INFO_REPLY`
- `PM_EXACT_FILE_SEARCH_REQUEST`
- `PM_INDIRECT_FILE_SEARCH_REQUEST`
- `PM_UPLOAD_PLACE_IN_LINE_REQUEST`

## S4 Candidate Batch C - Room Moderation (Deferred from S3B)

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
