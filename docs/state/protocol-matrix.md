# Protocol Message Matrix

This matrix tracks protocol coverage from authoritative artifacts.

## Snapshot

- Generated at: `2026-02-14T20:06:03+00:00`
- Total messages tracked: `131`
- Server messages: `106`
- Peer messages: `25`
- Implemented + mapped: `67`
- Mapped not implemented: `16`
- Implemented not mapped: `1`
- Missing: `47`

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
| peer | 10 | `PM_INVITE_USER_TO_ROOM` | `implemented_mapped` | high | Deterministic peer-local runtime flow emits invite-to-room frame (code 10) with room payload. | `captures/redacted/peer-legacy-local/official_frames.hex` |
| peer | 14 | `PM_CANCELLED_QUEUED_TRANSFER` | `implemented_mapped` | high | Deterministic peer-local runtime flow emits cancelled-queued-transfer frame (code 14) with virtual path payload. | `captures/redacted/peer-legacy-local/official_frames.hex` |
| peer | 15 | `PM_USER_INFO_REQUEST` | `implemented_mapped` | high | Peer code 15 UserInfoRequest with empty payload (code resolved from SLSK protocol spec); symbol confirmed in peer message code table. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer | 16 | `PM_USER_INFO_REPLY` | `implemented_mapped` | high | Peer code 16 UserInfoReply with description/picture/uploads/queue/slots fields (code resolved from SLSK protocol spec); symbol confirmed in peer message code table. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer | 34 | `PM_MOVE_DOWNLOAD_TO_TOP` | `implemented_mapped` | high | Deterministic peer-local runtime flow emits move-download-to-top frame (code 34) with virtual path payload. | `captures/redacted/peer-legacy-local/official_frames.hex` |
| peer | 36 | `PM_GET_SHARED_FILES_IN_FOLDER` | `implemented_mapped` | high | Deterministic peer-local runtime flow sends request for shared files in folder (code 36) with directory payload. | `captures/redacted/peer-folder-local/official_frames.hex` |
| peer | 37 | `PM_SHARED_FILES_IN_FOLDER` | `implemented_mapped` | high | Deterministic peer-local runtime flow emits shared-files-in-folder response (code 37) with directory plus compressed listing bytes. | `captures/redacted/peer-folder-local/official_frames.hex` |
| peer | 40 | `PM_TRANSFER_REQUEST` | `implemented_mapped` | high | Transfer queue dispatcher handles transfer request negotiation. | `evidence/reverse/disasm/transfer_on_file_request.txt` |
| peer | 41 | `PM_TRANSFER_RESPONSE` | `implemented_mapped` | high | Transfer queue dispatcher handles transfer response negotiation. | `evidence/reverse/disasm/transfer_on_file_request.txt` |
| peer | 43 | `PM_QUEUE_UPLOAD` | `implemented_mapped` | high | Observed literal in PeerMessenger::MessageCodeToString dispatch. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer | 44 | `PM_UPLOAD_PLACE_IN_LINE` | `implemented_mapped` | high | Observed inbound runtime frame in upload-deny scenario (code 44) with queue place payload. | `captures/redacted/upload-deny/official_frames.hex` |
| peer | 46 | `PM_UPLOAD_FAILED` | `implemented_mapped` | high | Observed inbound runtime frame in upload-deny scenario (code 46) with failure reason payload. | `captures/redacted/upload-deny/official_frames.hex` |
| peer | 47 | `PM_EXACT_FILE_SEARCH_REQUEST` | `implemented_mapped` | high | Deterministic peer-local runtime flow emits exact-file-search frame (code 47) with token+query payload. | `captures/redacted/peer-legacy-local/official_frames.hex` |
| peer | 48 | `PM_QUEUED_DOWNLOADS` | `implemented_mapped` | high | Deterministic peer-local runtime flow emits queued-downloads frame (code 48) with list payload. | `captures/redacted/peer-legacy-local/official_frames.hex` |
| peer | 49 | `PM_INDIRECT_FILE_SEARCH_REQUEST` | `implemented_mapped` | high | Deterministic peer-local runtime flow emits indirect-file-search frame (code 49) with query payload. | `captures/redacted/peer-legacy-local/official_frames.hex` |
| peer | 50 | `PM_UPLOAD_DENIED` | `implemented_mapped` | high | Observed inbound runtime frame in upload-deny scenario (code 50) with deny reason payload. | `captures/redacted/upload-deny/official_frames.hex` |
| peer | 51 | `PM_UPLOAD_PLACE_IN_LINE_REQUEST` | `implemented_mapped` | high | Peer code 51 PlaceInQueueRequest with filename/path payload (code resolved from SLSK protocol spec); symbol confirmed in peer message code table. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer |  | `PM_NOTHING` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| peer |  | `PM_PLACEHOLD_UPLOAD` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| peer |  | `PM_SAY` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| peer |  | `PM_SEND_CONNECT_TOKEN` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server | 1 | `SM_LOGIN` | `implemented_mapped` | high | Observed authenticated runtime login request+response (code 1) with success payload on tuple 160/1. | `captures/redacted/login-only/official_frames.hex` |
| server | 2 | `SM_SET_WAIT_PORT` | `implemented_mapped` | high | Observed literal in Server::MessageCodeToString dispatch. | `evidence/reverse/server_messagecodetostring_otool.txt` |
| server | 3 | `SM_GET_PEER_ADDRESS` | `implemented_mapped` | high | Authenticated runtime flow includes GetPeerAddress request/response (code 3) with username and endpoint payload fields. | `captures/redacted/login-peer-address-connect/official_frames.hex` |
| server | 7 | `SM_GET_USER_STATUS` | `implemented_mapped` | high | Authenticated runtime flow includes user-status request/response (code 7) with status and privilege fields. | `captures/redacted/login-user-state/official_frames.hex` |
| server | 11 | `SM_IGNORE_USER` | `implemented_mapped` | high | Authenticated runtime flow sends ignore-user request (code 11); mapping aligns with server MessageCodeToString and SLSK spec obsolete ignore operation. | `captures/redacted/login-privileges-social/official_frames.hex` |
| server | 12 | `SM_UNIGNORE_USER` | `implemented_mapped` | high | Authenticated runtime flow sends unignore-user request (code 12); mapping aligns with server MessageCodeToString and SLSK spec obsolete unignore operation. | `captures/redacted/login-privileges-social/official_frames.hex` |
| server | 13 | `SM_SAY_CHATROOM` | `implemented_mapped` | high | Observed runtime room message flow: outbound SAY request and inbound room chat event in authenticated room session. | `captures/redacted/login-join-room-presence/official_frames.hex` |
| server | 14 | `SM_JOIN_ROOM` | `implemented_mapped` | high | Observed runtime join-room request and server join payload during authenticated room session. | `captures/redacted/login-join-room-presence/official_frames.hex` |
| server | 15 | `SM_LEAVE_ROOM` | `implemented_mapped` | high | Observed runtime leave-room request and server leave acknowledgement payload. | `captures/redacted/login-leave-room/official_frames.hex` |
| server | 16 | `SM_USER_JOINED_ROOM` | `implemented_mapped` | high | Observed runtime user-joined room presence events while watching authenticated room session. | `captures/redacted/login-join-room-presence/official_frames.hex` |
| server | 17 | `SM_USER_LEFT_ROOM` | `implemented_mapped` | high | Observed runtime user-left room presence events while watching authenticated room session. | `captures/redacted/login-join-room-presence/official_frames.hex` |
| server | 18 | `SM_CONNECT_TO_PEER` | `implemented_mapped` | high | Authenticated runtime flow includes ConnectToPeer request and response payloads (code 18) with token | `captures/redacted/login-peer-address-connect/official_frames.hex` |
| server | 22 | `SM_MESSAGE_USER` | `implemented_mapped` | high | Authenticated runtime flow includes private-message send and inbound private-message frame (code 22) with directional payload variants. | `captures/redacted/login-private-message/official_frames.hex` |
| server | 23 | `SM_MESSAGE_ACKED` | `implemented_mapped` | high | Authenticated runtime flow includes private-message acknowledgement path (code 23) with message_id payload. | `captures/redacted/login-private-message/official_frames.hex` |
| server | 26 | `SM_FILE_SEARCH` | `implemented_mapped` | high | FileSearch writes constant 0x1a before Server::SendMessage. | `evidence/reverse/disasm/server_file_search.txt` |
| server | 34 | `SM_DOWNLOAD_SPEED` | `implemented_mapped` | high | Observed literal in Server::MessageCodeToString dispatch. | `evidence/reverse/server_messagecodetostring_otool.txt` |
| server | 35 | `SM_SHARED_FOLDERS_FILES` | `implemented_mapped` | high | Observed literal in Server::MessageCodeToString dispatch. | `evidence/reverse/server_messagecodetostring_otool.txt` |
| server | 36 | `SM_GET_USER_STATS` | `implemented_mapped` | high | Authenticated runtime flow includes user-stats request/response (code 36) with stats summary fields. | `captures/redacted/login-user-state/official_frames.hex` |
| server | 42 | `SM_SEARCH_USER_FILES` | `implemented_mapped` | high | Observed outbound runtime frame in login-search scenario (code 42) with user+query payload. | `captures/redacted/login-search/official_frames.hex` |
| server | 50 | `SM_GET_SIMILAR_TERMS` | `implemented_mapped` | high | Observed authenticated runtime request/response for similar-terms discovery flow (code 50). | `captures/redacted/login-similar-terms/official_frames.hex` |
| server | 54 | `SM_GET_RECOMMENDATIONS` | `implemented_mapped` | high | Observed authenticated runtime recommendations flow (code 54) including outbound request and inbound summary payload. | `captures/redacted/login-recommendations/official_frames.hex` |
| server | 55 | `SM_GET_MY_RECOMMENDATIONS` | `implemented_mapped` | high | Observed outbound authenticated runtime request for my-recommendations (code 55) in recommendation batch. | `captures/redacted/login-recommendations/official_frames.hex` |
| server | 56 | `SM_GET_GLOBAL_RECOMMENDATIONS` | `implemented_mapped` | high | Observed authenticated runtime global-recommendations flow (code 56) with populated inbound payload. | `captures/redacted/login-recommendations/official_frames.hex` |
| server | 57 | `SM_GET_USER_RECOMMENDATIONS` | `implemented_mapped` | high | Observed authenticated runtime user-recommendations flow (code 57) with user request and reply payload. | `captures/redacted/login-user-recommendations/official_frames.hex` |
| server | 64 | `SM_ROOM_LIST` | `implemented_mapped` | high | Observed runtime room list request/response flow with authenticated session. | `captures/redacted/login-room-list/official_frames.hex` |
| server | 65 | `SM_EXACT_FILE_SEARCH` | `implemented_mapped` | high | Observed outbound runtime frame in login-search scenario (code 65) with exact virtual path payload. | `captures/redacted/login-search/official_frames.hex` |
| server | 68 | `SM_PEER_MESSAGE` | `implemented_mapped` | high | Deterministic runtime-local legacy tunneled-message capture covers code 68 request/response payload variants; alt compatibility alias 292 observed in same run. | `captures/redacted/login-peer-message/official_frames.hex` |
| server | 69 | `SM_PRIVILEGED_LIST` | `implemented_mapped` | high | Authenticated runtime flow includes privileged-list request and response frames (code 69) with user-list style payloads. | `captures/redacted/login-privilege-messaging/official_frames.hex` |
| server | 92 | `SM_GET_OWN_PRIVILEGES_STATUS` | `implemented_mapped` | high | Authenticated runtime flow sends own-privileges status request (code 92); spec defines CheckPrivileges response with remaining seconds. | `captures/redacted/login-privileges-social/official_frames.hex` |
| server | 110 | `SM_GET_RECOMMENDED_USERS` | `implemented_mapped` | high | Authenticated runtime flow includes code 110 request/response for similar users with scored user entries. | `captures/redacted/login-privilege-messaging/official_frames.hex` |
| server | 111 | `SM_GET_TERM_RECOMMENDATIONS` | `implemented_mapped` | high | Authenticated runtime flow includes code 111 request with term payload and recommendation-entry response. | `captures/redacted/login-privilege-messaging/official_frames.hex` |
| server | 112 | `SM_GET_RECOMMENDATION_USERS` | `implemented_mapped` | high | Authenticated runtime flow includes code 112 request with term payload and scored-user response entries. | `captures/redacted/login-privilege-messaging/official_frames.hex` |
| server | 120 | `SM_SEARCH_ROOM` | `implemented_mapped` | high | Observed outbound runtime frame in login-search scenario (code 120) with room+query payload. | `captures/redacted/login-search/official_frames.hex` |
| server | 121 | `SM_UPLOAD_SPEED` | `implemented_mapped` | high | String present and mirrored by upload code paths. | `evidence/reverse/message_name_strings.txt` |
| server | 122 | `SM_GET_USER_PRIVILEGES_STATUS` | `implemented_mapped` | medium | Authenticated runtime flow sends user-privileges status request (code 122); response semantics are deprecated in spec and treated as username+privileged summary. | `captures/redacted/login-privileges-social/official_frames.hex` |
| server | 123 | `SM_GIVE_PRIVILEGE` | `implemented_mapped` | high | Authenticated runtime flow sends give-privilege request (code 123) with username+days payload. | `captures/redacted/login-privileges-social/official_frames.hex` |
| server | 124 | `SM_INFORM_USER_OF_PRIVILEGES` | `implemented_mapped` | high | Authenticated runtime flow sends notify-privileges request (code 124) with token+username payload. | `captures/redacted/login-privileges-social/official_frames.hex` |
| server | 125 | `SM_INFORM_USER_OF_PRIVILEGES_ACK` | `implemented_mapped` | high | Authenticated runtime flow sends notify-privileges ack (code 125) with token payload. | `captures/redacted/login-privileges-social/official_frames.hex` |
| server | 132 | `SM_BAN_USER` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 132 to SM_BAN_USER (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 133 | `SM_ROOM_MEMBERS` | `implemented_mapped` | high | Observed runtime room-members request flow in authenticated room session. | `captures/redacted/login-join-room-presence/official_frames.hex` |
| server | 134 | `SM_ADD_ROOM_MEMBER` | `implemented_mapped` | high | Server code 134 AddUserToPrivileged with room+username payload (code resolved from SLSK protocol spec) and symbol confirmed in binary string table. | `evidence/reverse/message_name_strings.txt` |
| server | 135 | `SM_REMOVE_ROOM_MEMBER` | `implemented_mapped` | high | Server code 135 RemoveUserFromPrivileged with room+username payload (code resolved from SLSK protocol spec) and symbol confirmed in binary string table. | `evidence/reverse/message_name_strings.txt` |
| server | 143 | `SM_ADD_ROOM_OPERATOR` | `implemented_mapped` | high | Server code 143 AddOperatorToPrivileged with room+username payload (code resolved from SLSK protocol spec) and symbol confirmed in binary string table. | `evidence/reverse/message_name_strings.txt` |
| server | 144 | `SM_REMOVE_ROOM_OPERATOR` | `implemented_mapped` | high | Server code 144 RemoveOperatorFromPrivileged with room+username payload (code resolved from SLSK protocol spec) and symbol confirmed in binary string table. | `evidence/reverse/message_name_strings.txt` |
| server | 148 | `SM_ROOM_OPERATORS` | `implemented_mapped` | high | Observed runtime room-operators request flow in authenticated room session. | `captures/redacted/login-join-room-presence/official_frames.hex` |
| server | 149 | `SM_MESSAGE_USERS` | `implemented_mapped` | high | Authenticated runtime flow sends message-users payload (code 149) with user list and message body. | `captures/redacted/login-message-users/official_frames.hex` |
| server | 58 | `SM_COMMAND` | `mapped_not_implemented` | high | Server MessageCodeToString jump-table extraction resolves code 58 to SM_COMMAND (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 66 | `SM_ADMIN_MESSAGE` | `mapped_not_implemented` | high | Server MessageCodeToString jump-table extraction resolves code 66 to SM_ADMIN_MESSAGE (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 67 | `SM_GLOBAL_USER_LIST` | `mapped_not_implemented` | high | Server MessageCodeToString jump-table extraction resolves code 67 to SM_GLOBAL_USER_LIST (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 71 | `SM_SEND_DISTRIBUTIONS` | `mapped_not_implemented` | high | Server MessageCodeToString jump-table extraction resolves code 71 to SM_SEND_DISTRIBUTIONS (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 73 | `SM_NOTE_PARENT` | `mapped_not_implemented` | high | Server MessageCodeToString jump-table extraction resolves code 73 to SM_NOTE_PARENT (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 82 | `SM_CHILD_PARENT_MAP` | `mapped_not_implemented` | high | Server MessageCodeToString jump-table extraction resolves code 82 to SM_CHILD_PARENT_MAP (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 83 | `SM_SET_PARENT_MIN_SPEED` | `mapped_not_implemented` | high | Server MessageCodeToString jump-table extraction resolves code 83 to SM_SET_PARENT_MIN_SPEED (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 84 | `SM_SET_PARENT_SPEED_CONNECTION_RATIO` | `mapped_not_implemented` | high | Server MessageCodeToString jump-table extraction resolves code 84 to SM_SET_PARENT_SPEED_CONNECTION_RATIO (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 86 | `SM_SET_PARENT_INACTIVITY_BEFORE_DISCONNECT` | `mapped_not_implemented` | high | Server MessageCodeToString jump-table extraction resolves code 86 to SM_SET_PARENT_INACTIVITY_BEFORE_DISCONNECT (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 87 | `SM_SET_SERVER_INACTIVITY_BEFORE_DISCONNECT` | `mapped_not_implemented` | high | Server MessageCodeToString jump-table extraction resolves code 87 to SM_SET_SERVER_INACTIVITY_BEFORE_DISCONNECT (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 88 | `SM_NODES_IN_CACHE_BEFORE_DISCONNECT` | `mapped_not_implemented` | high | Server MessageCodeToString jump-table extraction resolves code 88 to SM_NODES_IN_CACHE_BEFORE_DISCONNECT (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 90 | `SM_SET_SECONDS_BEFORE_PING_CHILDREN` | `mapped_not_implemented` | high | Server MessageCodeToString jump-table extraction resolves code 90 to SM_SET_SECONDS_BEFORE_PING_CHILDREN (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 93 | `SM_DNET_MESSAGE` | `mapped_not_implemented` | high | Server MessageCodeToString jump-table extraction resolves code 93 to SM_DNET_MESSAGE (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 100 | `SM_CAN_PARENT` | `mapped_not_implemented` | high | Server MessageCodeToString jump-table extraction resolves code 100 to SM_CAN_PARENT (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 102 | `SM_POSSIBLE_PARENTS` | `mapped_not_implemented` | high | Server MessageCodeToString jump-table extraction resolves code 102 to SM_POSSIBLE_PARENTS (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 130 | `SM_DNET_RESET` | `mapped_not_implemented` | high | Server MessageCodeToString jump-table extraction resolves code 130 to SM_DNET_RESET (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 292 | `SM_PEER_MESSAGE_ALT` | `implemented_not_mapped` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ADD_CHATROOM` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ADD_HATE_TERM` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ADD_LIKE_TERM` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ADD_PRIVILEGED_USER` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ADD_ROOM_MEMBERSHIP` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ADD_ROOM_OPERATORSHIP` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ADD_USER` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_CHANGE_PASSWORD` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_CONNECT_TO_CLIENT` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_DNET_CHILD_DEPTH` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_DNET_DELIVERY_REPORT` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_DNET_GROUP_LEADER` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_DNET_LEVEL` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_ENABLE_PRIVATE_ROOM_ADD` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_FLOOD` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_GET_ROOM_TICKER` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_GIVE_UP_ROOM` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_HEARTBEAT` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_JOIN_GLOBAL_ROOM` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_LEAVE_GLOBAL_ROOM` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_LOW_PRIORITY_FILE_SEARCH` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_PLACE_IN_LINE` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_PLACE_IN_LINE_RESPONSE` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
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
| server |  | `SM_SET_STATUS` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_SET_TICKER` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_TRANSFER_ROOM_OWNERSHIP` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_USER_LIST` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |
| server |  | `SM_WISHLIST_WAIT` | `missing` |  | Known message name from static string table; payload and behavior mapping pending. | `evidence/reverse/message_name_strings.txt` |

## Regeneration

```bash
python3 tools/protocol/generate_protocol_matrix.py
```
