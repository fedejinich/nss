# Protocol Backlog (Post S4C)

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

## Completed in S4A (Recommendations + Discovery Batch)

- `SM_GET_RECOMMENDATIONS`
- `SM_GET_MY_RECOMMENDATIONS`
- `SM_GET_GLOBAL_RECOMMENDATIONS`
- `SM_GET_USER_RECOMMENDATIONS`
- `SM_GET_SIMILAR_TERMS`

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

## Completed in S4C (Privileges/Social + Peer Folder Batch)

- `SM_IGNORE_USER`
- `SM_UNIGNORE_USER`
- `SM_GET_OWN_PRIVILEGES_STATUS`
- `SM_GET_USER_PRIVILEGES_STATUS`
- `SM_GIVE_PRIVILEGE`
- `SM_INFORM_USER_OF_PRIVILEGES`
- `SM_INFORM_USER_OF_PRIVILEGES_ACK`
- `PM_GET_SHARED_FILES_IN_FOLDER`
- `PM_SHARED_FILES_IN_FOLDER`

Status: completed in S4C with runtime captures (`login-privileges-social`, `peer-folder-local`), protocol implementation, and semantic differential verification.

## Remaining S4D Candidate Batch - Privilege/Messaging Gaps

- `SM_BAN_USER` (authoritative code unresolved)
- `SM_PRIVILEGED_LIST`
- `SM_GET_RECOMMENDATION_USERS`
- `SM_GET_RECOMMENDED_USERS`
- `SM_GET_TERM_RECOMMENDATIONS`

## Remaining S4D Candidate Batch - Peer Legacy/Search Cleanup

- `PM_INVITE_USER_TO_ROOM`
- `PM_CANCELLED_QUEUED_TRANSFER`
- `PM_QUEUED_DOWNLOADS`
- `PM_MOVE_DOWNLOAD_TO_TOP`
- Runtime promotion of medium-confidence:
  - `PM_EXACT_FILE_SEARCH_REQUEST`
  - `PM_INDIRECT_FILE_SEARCH_REQUEST`

## Execution Strategy

1. Add batch rows to `analysis/ghidra/maps/message_map.csv` with explicit confidence and evidence.
2. Collect runtime captures first for high-confidence promotion when feasible.
3. Regenerate schema/docs from authoritative maps:
   - `scripts/derive_message_schema.sh`
   - `python3 scripts/kb_sync_docs.py`
4. Extend SDK/CLI/verify only after mapping evidence is registered.
5. Keep regression green (`scripts/run_regression.sh`) before stage closure.

## Next Session Entry Point

Start S4D by resolving `SM_BAN_USER` code mapping and promoting the remaining medium-confidence peer legacy search messages with live runtime evidence.
