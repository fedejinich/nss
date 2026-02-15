# Protocol Backlog (Post S4K)

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

## Completed in S4D (Privilege/Messaging Gaps + Peer Legacy Cleanup)

- `SM_BAN_USER`
- `SM_PRIVILEGED_LIST`
- `SM_GET_RECOMMENDATION_USERS`
- `SM_GET_RECOMMENDED_USERS`
- `SM_GET_TERM_RECOMMENDATIONS`
- `PM_INVITE_USER_TO_ROOM`
- `PM_CANCELLED_QUEUED_TRANSFER`
- `PM_QUEUED_DOWNLOADS`
- `PM_MOVE_DOWNLOAD_TO_TOP`
- runtime promotions:
  - `PM_EXACT_FILE_SEARCH_REQUEST` (`medium -> high`)
  - `PM_INDIRECT_FILE_SEARCH_REQUEST` (`medium -> high`)

Status: completed in S4D with runtime captures (`login-privilege-messaging`, `peer-legacy-local`), jump-table static extraction (`SM_BAN_USER`), protocol implementation, and semantic verification updates.

## Completed in S4E (Private Messaging + User-State Domain)

- `SM_MESSAGE_USER`
- `SM_MESSAGE_ACKED`
- `SM_GET_USER_STATUS`
- `SM_GET_USER_STATS`
- `SM_GET_PEER_ADDRESS`
- `SM_CONNECT_TO_PEER`
- `SM_MESSAGE_USERS`
- `SM_PEER_MESSAGE`

Status: completed in S4E with runtime captures (`login-private-message`, `login-user-state`, `login-peer-address-connect`, `login-message-users`, `login-peer-message`), protocol implementation, and semantic verification updates.

## Completed in S4F (Global/Admin/Distributed Control Mapping Batch)

- `SM_COMMAND`
- `SM_ADMIN_MESSAGE`
- `SM_GLOBAL_USER_LIST`
- `SM_SEND_DISTRIBUTIONS`
- `SM_NOTE_PARENT`
- `SM_CHILD_PARENT_MAP`
- `SM_DNET_MESSAGE`
- `SM_DNET_RESET`

Status: completed in S4F with authoritative static mapping evidence from jump-table extraction and synchronized schema/docs/matrix artifacts.

## Completed in S4G (Parent/Distributed Tuning Mapping Batch)

- `SM_SET_PARENT_MIN_SPEED`
- `SM_SET_PARENT_SPEED_CONNECTION_RATIO`
- `SM_SET_PARENT_INACTIVITY_BEFORE_DISCONNECT`
- `SM_SET_SERVER_INACTIVITY_BEFORE_DISCONNECT`
- `SM_NODES_IN_CACHE_BEFORE_DISCONNECT`
- `SM_SET_SECONDS_BEFORE_PING_CHILDREN`
- `SM_CAN_PARENT`
- `SM_POSSIBLE_PARENTS`

Status: completed in S4G with authoritative static mapping evidence from jump-table extraction and synchronized schema/docs/matrix artifacts.

## Completed in S4H (Global Room/System Control Mapping Batch)

- `SM_ADD_CHATROOM`
- `SM_SET_STATUS`
- `SM_HEARTBEAT`
- `SM_RELOGGED`
- `SM_USER_LIST`
- `SM_ROOM_ADDED`
- `SM_ROOM_REMOVED`
- `SM_CONNECT_TO_CLIENT`

Status: completed in S4H with authoritative static mapping evidence from jump-table extraction and synchronized schema/docs/matrix artifacts.

## Completed in S4I (Ticker/Term Control Mapping Batch)

- `SM_ADD_LIKE_TERM`
- `SM_REMOVE_LIKE_TERM`
- `SM_GET_ROOM_TICKER`
- `SM_ROOM_TICKER_USER_ADDED`
- `SM_ROOM_TICKER_USER_REMOVED`
- `SM_SET_TICKER`
- `SM_ADD_HATE_TERM`
- `SM_REMOVE_HATE_TERM`

Status: completed in S4I with authoritative static mapping evidence from jump-table extraction and synchronized schema/docs/matrix artifacts.

## Completed in S4J (Private Room Ownership/Membership Mapping Batch)

- `SM_REMOVE_OWN_ROOM_MEMBERSHIP`
- `SM_GIVE_UP_ROOM`
- `SM_TRANSFER_ROOM_OWNERSHIP`
- `SM_ADD_ROOM_MEMBERSHIP`
- `SM_REMOVE_ROOM_MEMBERSHIP`
- `SM_ENABLE_PRIVATE_ROOM_ADD`
- `SM_CHANGE_PASSWORD`
- `SM_ADD_ROOM_OPERATORSHIP`

Status: completed in S4J with authoritative static mapping evidence from jump-table extraction and synchronized schema/docs/matrix artifacts.

## Completed in S4K (Missing-Code Closure + Global/Distributed Tail + Peer Control)

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

Status: completed in S4K with authoritative static mapping evidence from jump-table extraction plus protocol codec coverage in `rust/protocol`. Matrix `missing` bucket is now `0`.

## Next Candidate Stage (S4L) - Mapped-not-implemented Reduction Wave 1

Target messages for typed implementation promotion:

- `SM_COMMAND`
- `SM_ADMIN_MESSAGE`
- `SM_GLOBAL_USER_LIST`
- `SM_SEND_DISTRIBUTIONS`
- `SM_NOTE_PARENT`
- `SM_CHILD_PARENT_MAP`
- `SM_DNET_MESSAGE`
- `SM_DNET_RESET`
- `SM_SET_PARENT_MIN_SPEED`
- `SM_SET_PARENT_SPEED_CONNECTION_RATIO`
- `SM_SET_PARENT_INACTIVITY_BEFORE_DISCONNECT`
- `SM_SET_SERVER_INACTIVITY_BEFORE_DISCONNECT`
- `SM_NODES_IN_CACHE_BEFORE_DISCONNECT`
- `SM_SET_SECONDS_BEFORE_PING_CHILDREN`
- `SM_CAN_PARENT`
- `SM_POSSIBLE_PARENTS`
- carryover parser-depth follow-up:
  - `PM_SHARED_FILES_IN_FOLDER` compressed payload decomposition

## Execution Strategy

1. Prioritize conversion from `mapped_not_implemented` to `implemented_mapped` in `rust/protocol`.
2. Add typed payload decoding where payload shape is deterministic; keep opaque compatibility only when runtime evidence is still absent.
3. Regenerate schema/docs from authoritative maps and protocol constants:
   - `scripts/derive_message_schema.sh`
   - `python3 scripts/kb_sync_docs.py`
4. Extend SDK/CLI/verify for promoted message families once protocol decode is stable.
5. Keep regression green (`scripts/run_regression.sh`) before stage closure.

## Next Session Entry Point

Start S4L by reducing the `mapped_not_implemented` bucket (`40`) with typed protocol implementations for S4F/S4G distributed-control families.
