# Protocol Message Matrix

This matrix tracks protocol coverage from authoritative artifacts.

## Snapshot

- Generated at: `2026-02-14T15:40:25+00:00`
- Total messages tracked: `130`
- Server messages: `105`
- Peer messages: `25`
- Implemented + mapped: `47`
- Mapped not implemented: `0`
- Implemented not mapped: `0`
- Missing: `83`

Status legend:

- `implemented_mapped`: present in authoritative map and implemented in `rust/protocol`.
- `mapped_not_implemented`: mapped with evidence but not yet implemented in `rust/protocol`.
- `implemented_not_mapped`: implemented in `rust/protocol` but absent from authoritative map.
- `missing`: known from static string tables but not mapped/implemented yet.

## Matrix

| scope | code | message | status | confidence | purpose summary | evidence |
|---|---:|---|---|---|---|---|
| peer | 4 | `PM_GET_SHARED_FILE_LIST` | `implemented_mapped` | high | Observed literal in PeerMessenger::MessageCodeToString dispatch. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer | 5 | `PM_SHARED_FILE_LIST` | `implemented_mapped` | high | Observed literal in PeerMessenger::MessageCodeToString dispatch. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer | 8 | `PM_FILE_SEARCH_REQUEST` | `implemented_mapped` | high | Observed literal in PeerMessenger::MessageCodeToString dispatch. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer | 9 | `PM_FILE_SEARCH_RESULT` | `implemented_mapped` | high | Observed inbound runtime frame in login-search-download scenario (code 9) with token+user+result_count. | `captures/redacted/login-search-download/official_frames.hex` |
| peer | 15 | `PM_USER_INFO_REQUEST` | `implemented_mapped` | high | Peer code 15 UserInfoRequest with empty payload (code resolved from SLSK protocol spec); symbol confirmed in peer message code table. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer | 16 | `PM_USER_INFO_REPLY` | `implemented_mapped` | high | Peer code 16 UserInfoReply with description/picture/uploads/queue/slots fields (code resolved from SLSK protocol spec); symbol confirmed in peer message code table. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer | 40 | `PM_TRANSFER_REQUEST` | `implemented_mapped` | high | Transfer queue dispatcher handles transfer request negotiation. | `evidence/reverse/disasm/transfer_on_file_request.txt` |
| peer | 41 | `PM_TRANSFER_RESPONSE` | `implemented_mapped` | high | Transfer queue dispatcher handles transfer response negotiation. | `evidence/reverse/disasm/transfer_on_file_request.txt` |
| peer | 43 | `PM_QUEUE_UPLOAD` | `implemented_mapped` | high | Observed literal in PeerMessenger::MessageCodeToString dispatch. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer | 44 | `PM_UPLOAD_PLACE_IN_LINE` | `implemented_mapped` | high | Observed inbound runtime frame in upload-deny scenario (code 44) with queue place payload. | `captures/redacted/upload-deny/official_frames.hex` |
| peer | 46 | `PM_UPLOAD_FAILED` | `implemented_mapped` | high | Observed inbound runtime frame in upload-deny scenario (code 46) with failure reason payload. | `captures/redacted/upload-deny/official_frames.hex` |
| peer | 47 | `PM_EXACT_FILE_SEARCH_REQUEST` | `implemented_mapped` | medium | Peer code list maps 47 to ExactFileSearchRequest (code resolved from SLSK protocol spec); payload shape treated as legacy optional-token+query until runtime evidence is captured. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer | 49 | `PM_INDIRECT_FILE_SEARCH_REQUEST` | `implemented_mapped` | medium | Peer code list maps 49 to IndirectFileSearchRequest (code resolved from SLSK protocol spec); payload shape treated as legacy optional-token+query until runtime evidence is captured. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer | 50 | `PM_UPLOAD_DENIED` | `implemented_mapped` | high | Observed inbound runtime frame in upload-deny scenario (code 50) with deny reason payload. | `captures/redacted/upload-deny/official_frames.hex` |
| peer | 51 | `PM_UPLOAD_PLACE_IN_LINE_REQUEST` | `implemented_mapped` | high | Peer code 51 PlaceInQueueRequest with filename/path payload (code resolved from SLSK protocol spec); symbol confirmed in peer message code table. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer |  | `PM_CANCELLED_QUEUED_TRANSFER` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| peer |  | `PM_GET_SHARED_FILES_IN_FOLDER` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| peer |  | `PM_INVITE_USER_TO_ROOM` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| peer |  | `PM_MOVE_DOWNLOAD_TO_TOP` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| peer |  | `PM_NOTHING` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| peer |  | `PM_PLACEHOLD_UPLOAD` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| peer |  | `PM_QUEUED_DOWNLOADS` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| peer |  | `PM_SAY` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| peer |  | `PM_SEND_CONNECT_TOKEN` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| peer |  | `PM_SHARED_FILES_IN_FOLDER` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server | 1 | `SM_LOGIN` | `implemented_mapped` | high | Observed authenticated runtime login request+response (code 1) with success payload on tuple 160/1. | `captures/redacted/login-only/official_frames.hex` |
| server | 2 | `SM_SET_WAIT_PORT` | `implemented_mapped` | high | Observed literal in Server::MessageCodeToString dispatch. | `evidence/reverse/server_messagecodetostring_otool.txt` |
| server | 3 | `SM_GET_PEER_ADDRESS` | `implemented_mapped` | high | Observed literal in Server::MessageCodeToString dispatch. | `evidence/reverse/server_messagecodetostring_otool.txt` |
| server | 7 | `SM_GET_USER_STATUS` | `implemented_mapped` | high | Observed literal in Server::MessageCodeToString dispatch. | `evidence/reverse/server_messagecodetostring_otool.txt` |
| server | 13 | `SM_SAY_CHATROOM` | `implemented_mapped` | high | Observed runtime room message flow: outbound SAY request and inbound room chat event in authenticated room session. | `captures/redacted/login-join-room-presence/official_frames.hex` |
| server | 14 | `SM_JOIN_ROOM` | `implemented_mapped` | high | Observed runtime join-room request and server join payload during authenticated room session. | `captures/redacted/login-join-room-presence/official_frames.hex` |
| server | 15 | `SM_LEAVE_ROOM` | `implemented_mapped` | high | Observed runtime leave-room request and server leave acknowledgement payload. | `captures/redacted/login-leave-room/official_frames.hex` |
| server | 16 | `SM_USER_JOINED_ROOM` | `implemented_mapped` | high | Observed runtime user-joined room presence events while watching authenticated room session. | `captures/redacted/login-join-room-presence/official_frames.hex` |
| server | 17 | `SM_USER_LEFT_ROOM` | `implemented_mapped` | high | Observed runtime user-left room presence events while watching authenticated room session. | `captures/redacted/login-join-room-presence/official_frames.hex` |
| server | 18 | `SM_CONNECT_TO_PEER` | `implemented_mapped` | high | Observed literal in Server::MessageCodeToString dispatch and peer connect path. | `evidence/reverse/server_messagecodetostring_otool.txt` |
| server | 22 | `SM_MESSAGE_USER` | `implemented_mapped` | high | Observed literal in Server::MessageCodeToString dispatch. | `evidence/reverse/server_messagecodetostring_otool.txt` |
| server | 23 | `SM_MESSAGE_ACKED` | `implemented_mapped` | high | Observed literal in Server::MessageCodeToString dispatch. | `evidence/reverse/server_messagecodetostring_otool.txt` |
| server | 26 | `SM_FILE_SEARCH` | `implemented_mapped` | high | FileSearch writes constant 0x1a before Server::SendMessage. | `evidence/reverse/disasm/server_file_search.txt` |
| server | 34 | `SM_DOWNLOAD_SPEED` | `implemented_mapped` | high | Observed literal in Server::MessageCodeToString dispatch. | `evidence/reverse/server_messagecodetostring_otool.txt` |
| server | 35 | `SM_SHARED_FOLDERS_FILES` | `implemented_mapped` | high | Observed literal in Server::MessageCodeToString dispatch. | `evidence/reverse/server_messagecodetostring_otool.txt` |
| server | 36 | `SM_GET_USER_STATS` | `implemented_mapped` | high | Observed literal in Server::MessageCodeToString dispatch. | `evidence/reverse/server_messagecodetostring_otool.txt` |
| server | 42 | `SM_SEARCH_USER_FILES` | `implemented_mapped` | high | Observed outbound runtime frame in login-search scenario (code 42) with user+query payload. | `captures/redacted/login-search/official_frames.hex` |
| server | 50 | `SM_GET_SIMILAR_TERMS` | `implemented_mapped` | high | Observed authenticated runtime request/response for similar-terms discovery flow (code 50). | `captures/redacted/login-similar-terms/official_frames.hex` |
| server | 54 | `SM_GET_RECOMMENDATIONS` | `implemented_mapped` | high | Observed authenticated runtime recommendations flow (code 54) including outbound request and inbound summary payload. | `captures/redacted/login-recommendations/official_frames.hex` |
| server | 55 | `SM_GET_MY_RECOMMENDATIONS` | `implemented_mapped` | high | Observed outbound authenticated runtime request for my-recommendations (code 55) in recommendation batch. | `captures/redacted/login-recommendations/official_frames.hex` |
| server | 56 | `SM_GET_GLOBAL_RECOMMENDATIONS` | `implemented_mapped` | high | Observed authenticated runtime global-recommendations flow (code 56) with populated inbound payload. | `captures/redacted/login-recommendations/official_frames.hex` |
| server | 57 | `SM_GET_USER_RECOMMENDATIONS` | `implemented_mapped` | high | Observed authenticated runtime user-recommendations flow (code 57) with user request and reply payload. | `captures/redacted/login-user-recommendations/official_frames.hex` |
| server | 64 | `SM_ROOM_LIST` | `implemented_mapped` | high | Observed runtime room list request/response flow with authenticated session. | `captures/redacted/login-room-list/official_frames.hex` |
| server | 65 | `SM_EXACT_FILE_SEARCH` | `implemented_mapped` | high | Observed outbound runtime frame in login-search scenario (code 65) with exact virtual path payload. | `captures/redacted/login-search/official_frames.hex` |
| server | 120 | `SM_SEARCH_ROOM` | `implemented_mapped` | high | Observed outbound runtime frame in login-search scenario (code 120) with room+query payload. | `captures/redacted/login-search/official_frames.hex` |
| server | 121 | `SM_UPLOAD_SPEED` | `implemented_mapped` | high | String present and mirrored by upload code paths. | `evidence/reverse/message_name_strings.txt` |
| server | 133 | `SM_ROOM_MEMBERS` | `implemented_mapped` | high | Observed runtime room-members request flow in authenticated room session. | `captures/redacted/login-join-room-presence/official_frames.hex` |
| server | 134 | `SM_ADD_ROOM_MEMBER` | `implemented_mapped` | high | Server code 134 AddUserToPrivileged with room+username payload (code resolved from SLSK protocol spec) and symbol confirmed in binary string table. | `evidence/reverse/message_name_strings.txt` |
| server | 135 | `SM_REMOVE_ROOM_MEMBER` | `implemented_mapped` | high | Server code 135 RemoveUserFromPrivileged with room+username payload (code resolved from SLSK protocol spec) and symbol confirmed in binary string table. | `evidence/reverse/message_name_strings.txt` |
| server | 143 | `SM_ADD_ROOM_OPERATOR` | `implemented_mapped` | high | Server code 143 AddOperatorToPrivileged with room+username payload (code resolved from SLSK protocol spec) and symbol confirmed in binary string table. | `evidence/reverse/message_name_strings.txt` |
| server | 144 | `SM_REMOVE_ROOM_OPERATOR` | `implemented_mapped` | high | Server code 144 RemoveOperatorFromPrivileged with room+username payload (code resolved from SLSK protocol spec) and symbol confirmed in binary string table. | `evidence/reverse/message_name_strings.txt` |
| server | 148 | `SM_ROOM_OPERATORS` | `implemented_mapped` | high | Observed runtime room-operators request flow in authenticated room session. | `captures/redacted/login-join-room-presence/official_frames.hex` |
| server |  | `SM_ADD_CHATROOM` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ADD_HATE_TERM` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ADD_LIKE_TERM` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ADD_PRIVILEGED_USER` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ADD_ROOM_MEMBERSHIP` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ADD_ROOM_OPERATORSHIP` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ADD_USER` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ADMIN_MESSAGE` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_BAN_USER` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_CAN_PARENT` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_CHANGE_PASSWORD` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_CHILD_PARENT_MAP` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_COMMAND` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_CONNECT_TO_CLIENT` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_DNET_CHILD_DEPTH` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_DNET_DELIVERY_REPORT` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_DNET_GROUP_LEADER` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_DNET_LEVEL` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_DNET_MESSAGE` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_DNET_RESET` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ENABLE_PRIVATE_ROOM_ADD` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_FLOOD` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_GET_OWN_PRIVILEGES_STATUS` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_GET_RECOMMENDATION_USERS` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_GET_RECOMMENDED_USERS` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_GET_ROOM_TICKER` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_GET_TERM_RECOMMENDATIONS` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_GET_USER_PRIVILEGES_STATUS` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_GIVE_PRIVILEGE` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_GIVE_UP_ROOM` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_GLOBAL_USER_LIST` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_HEARTBEAT` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_IGNORE_USER` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_INFORM_USER_OF_PRIVILEGES` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_INFORM_USER_OF_PRIVILEGES_ACK` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_JOIN_GLOBAL_ROOM` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_LEAVE_GLOBAL_ROOM` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_LOW_PRIORITY_FILE_SEARCH` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_MESSAGE_USERS` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_NODES_IN_CACHE_BEFORE_DISCONNECT` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_NOTE_PARENT` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_PEER_MESSAGE` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_PLACE_IN_LINE` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_PLACE_IN_LINE_RESPONSE` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_POSSIBLE_PARENTS` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_PRIVILEGED_LIST` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_RELOGGED` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_REMOVE_HATE_TERM` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_REMOVE_LIKE_TERM` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_REMOVE_OWN_ROOM_MEMBERSHIP` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_REMOVE_OWN_ROOM_OPERATORSHIP` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_REMOVE_ROOM_MEMBERSHIP` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_REMOVE_ROOM_OPERATORSHIP` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_REMOVE_USER` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ROOM_ADDED` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ROOM_REMOVED` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ROOM_TICKER_USER_ADDED` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ROOM_TICKER_USER_REMOVED` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_SAY_GLOBAL_ROOM` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_SEARCH_CORRELATIONS` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_SEND_CONNECT_TOKEN` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_SEND_DISTRIBUTIONS` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_SET_PARENT_INACTIVITY_BEFORE_DISCONNECT` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_SET_PARENT_MIN_SPEED` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_SET_PARENT_SPEED_CONNECTION_RATIO` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_SET_SECONDS_BEFORE_PING_CHILDREN` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_SET_SERVER_INACTIVITY_BEFORE_DISCONNECT` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_SET_STATUS` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_SET_TICKER` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_TRANSFER_ROOM_OWNERSHIP` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_UNIGNORE_USER` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_USER_LIST` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_WISHLIST_WAIT` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |

## Regeneration

```bash
python3 tools/protocol/generate_protocol_matrix.py
```
