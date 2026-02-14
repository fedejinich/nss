# Protocol Backlog (Post S4B)

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

## Completed in S4B (Peer Advanced + Room Moderation Batch)

- `PM_USER_INFO_REQUEST`
- `PM_USER_INFO_REPLY`
- `PM_EXACT_FILE_SEARCH_REQUEST`
- `PM_INDIRECT_FILE_SEARCH_REQUEST`
- `PM_UPLOAD_PLACE_IN_LINE_REQUEST`
- `SM_ADD_ROOM_MEMBER`
- `SM_REMOVE_ROOM_MEMBER`
- `SM_ADD_ROOM_OPERATOR`
- `SM_REMOVE_ROOM_OPERATOR`

Status: completed in S4B with static/spec evidence, runtime captures (`login-room-moderation`, `peer-advanced-local`), protocol implementation, and semantic differential verification.

## S4C Candidate Batch - Privileges and Social Control

- `SM_BAN_USER`
- `SM_GET_USER_PRIVILEGES_STATUS`
- `SM_GET_OWN_PRIVILEGES_STATUS`
- `SM_GIVE_PRIVILEGE`
- `SM_INFORM_USER_OF_PRIVILEGES`
- `SM_INFORM_USER_OF_PRIVILEGES_ACK`
- `SM_IGNORE_USER`
- `SM_UNIGNORE_USER`

## S4C Candidate Batch - Peer Folder Domain

- `PM_GET_SHARED_FILES_IN_FOLDER`
- `PM_SHARED_FILES_IN_FOLDER`

## Execution Strategy

1. Add batch rows to `analysis/ghidra/maps/message_map.csv` with explicit confidence and evidence.
2. Collect runtime captures first for high-confidence promotion when feasible.
3. Regenerate schema/docs from authoritative maps:
   - `scripts/derive_message_schema.sh`
   - `python3 scripts/kb_sync_docs.py`
4. Extend SDK/CLI/verify only after mapping evidence is registered.
5. Keep regression green (`scripts/run_regression.sh`) before stage closure.

## Next Session Entry Point

Start S4C with privileges/social-control messages because they are high-impact for real account behavior and reuse the same authenticated runtime harness established in S3A/S4A/S4B.
