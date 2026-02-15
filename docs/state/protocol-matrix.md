# Protocol Message Matrix

This matrix tracks protocol coverage from authoritative artifacts.

## Snapshot

- Generated at: `2026-02-15T06:33:27+00:00`
- Total messages tracked: `131`
- Server messages: `106`
- Peer messages: `25`
- Implemented + mapped: `131`
- Mapped not implemented: `0`
- Implemented not mapped: `0`
- Missing: `0`

Status legend:

- `implemented_mapped`: present in authoritative map and implemented in `rust/protocol`.
- `mapped_not_implemented`: mapped with evidence but not yet implemented in `rust/protocol`.
- `implemented_not_mapped`: implemented in `rust/protocol` but absent from authoritative map.
- `missing`: known from static string tables but not mapped/implemented yet.

## Matrix

| scope | code | message | status | confidence | purpose summary | evidence |
|---|---:|---|---|---|---|---|
| peer | 1 | `PM_SAY` | `implemented_mapped` | high | Peer MessageCodeToString jump-table extraction resolves code 1 to PM_SAY (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| peer | 4 | `PM_GET_SHARED_FILE_LIST` | `implemented_mapped` | high | Observed literal in PeerMessenger::MessageCodeToString dispatch. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer | 5 | `PM_SHARED_FILE_LIST` | `implemented_mapped` | high | Observed literal in PeerMessenger::MessageCodeToString dispatch. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer | 8 | `PM_FILE_SEARCH_REQUEST` | `implemented_mapped` | high | Observed literal in PeerMessenger::MessageCodeToString dispatch. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer | 9 | `PM_FILE_SEARCH_RESULT` | `implemented_mapped` | high | Observed inbound runtime frame in login-search-download scenario (code 9) with token+user+result_count. | `captures/redacted/login-search-download/official_frames.hex` |
| peer | 10 | `PM_INVITE_USER_TO_ROOM` | `implemented_mapped` | high | Deterministic peer-local runtime flow emits invite-to-room frame (code 10) with room payload. | `captures/redacted/peer-legacy-local/official_frames.hex` |
| peer | 14 | `PM_CANCELLED_QUEUED_TRANSFER` | `implemented_mapped` | high | Deterministic peer-local runtime flow emits cancelled-queued-transfer frame (code 14) with virtual path payload. | `captures/redacted/peer-legacy-local/official_frames.hex` |
| peer | 15 | `PM_USER_INFO_REQUEST` | `implemented_mapped` | high | Peer code 15 UserInfoRequest with empty payload (code resolved from SLSK protocol spec); symbol confirmed in peer message code table. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer | 16 | `PM_USER_INFO_REPLY` | `implemented_mapped` | high | Peer code 16 UserInfoReply with description/picture/uploads/queue/slots fields (code resolved from SLSK protocol spec); symbol confirmed in peer message code table. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer | 33 | `PM_SEND_CONNECT_TOKEN` | `implemented_mapped` | high | Peer MessageCodeToString jump-table extraction resolves code 33 to PM_SEND_CONNECT_TOKEN (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| peer | 34 | `PM_MOVE_DOWNLOAD_TO_TOP` | `implemented_mapped` | high | Deterministic peer-local runtime flow emits move-download-to-top frame (code 34) with virtual path payload. | `captures/redacted/peer-legacy-local/official_frames.hex` |
| peer | 36 | `PM_GET_SHARED_FILES_IN_FOLDER` | `implemented_mapped` | high | Deterministic peer-local runtime flow sends request for shared files in folder (code 36) with directory payload. | `captures/redacted/peer-folder-local/official_frames.hex` |
| peer | 37 | `PM_SHARED_FILES_IN_FOLDER` | `implemented_mapped` | high | Deterministic peer-local runtime flow emits shared-files-in-folder response (code 37); protocol parser now performs zlib decompression-aware decoding. | `captures/redacted/peer-folder-local/official_frames.hex` |
| peer | 40 | `PM_TRANSFER_REQUEST` | `implemented_mapped` | high | Transfer queue dispatcher handles transfer request negotiation. | `evidence/reverse/disasm/transfer_on_file_request.txt` |
| peer | 41 | `PM_TRANSFER_RESPONSE` | `implemented_mapped` | high | Transfer queue dispatcher handles transfer response negotiation. | `evidence/reverse/disasm/transfer_on_file_request.txt` |
| peer | 42 | `PM_PLACEHOLD_UPLOAD` | `implemented_mapped` | high | Peer MessageCodeToString jump-table extraction resolves code 42 to PM_PLACEHOLD_UPLOAD (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| peer | 43 | `PM_QUEUE_UPLOAD` | `implemented_mapped` | high | Observed literal in PeerMessenger::MessageCodeToString dispatch. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer | 44 | `PM_UPLOAD_PLACE_IN_LINE` | `implemented_mapped` | high | Observed inbound runtime frame in upload-deny scenario (code 44) with queue place payload. | `captures/redacted/upload-deny/official_frames.hex` |
| peer | 46 | `PM_UPLOAD_FAILED` | `implemented_mapped` | high | Observed inbound runtime frame in upload-deny scenario (code 46) with failure reason payload. | `captures/redacted/upload-deny/official_frames.hex` |
| peer | 47 | `PM_EXACT_FILE_SEARCH_REQUEST` | `implemented_mapped` | high | Deterministic peer-local runtime flow emits exact-file-search frame (code 47) with token+query payload. | `captures/redacted/peer-legacy-local/official_frames.hex` |
| peer | 48 | `PM_QUEUED_DOWNLOADS` | `implemented_mapped` | high | Deterministic peer-local runtime flow emits queued-downloads frame (code 48) with list payload. | `captures/redacted/peer-legacy-local/official_frames.hex` |
| peer | 49 | `PM_INDIRECT_FILE_SEARCH_REQUEST` | `implemented_mapped` | high | Deterministic peer-local runtime flow emits indirect-file-search frame (code 49) with query payload. | `captures/redacted/peer-legacy-local/official_frames.hex` |
| peer | 50 | `PM_UPLOAD_DENIED` | `implemented_mapped` | high | Observed inbound runtime frame in upload-deny scenario (code 50) with deny reason payload. | `captures/redacted/upload-deny/official_frames.hex` |
| peer | 51 | `PM_UPLOAD_PLACE_IN_LINE_REQUEST` | `implemented_mapped` | high | Peer code 51 PlaceInQueueRequest with filename/path payload (code resolved from SLSK protocol spec); symbol confirmed in peer message code table. | `evidence/reverse/peer_messagecodetostring_otool.txt` |
| peer | 52 | `PM_NOTHING` | `implemented_mapped` | high | Peer MessageCodeToString jump-table extraction resolves code 52 to PM_NOTHING (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 1 | `SM_LOGIN` | `implemented_mapped` | high | Observed authenticated runtime login request+response (code 1) with success payload on tuple 160/1. | `captures/redacted/login-only/official_frames.hex` |
| server | 2 | `SM_SET_WAIT_PORT` | `implemented_mapped` | high | Observed literal in Server::MessageCodeToString dispatch. | `evidence/reverse/server_messagecodetostring_otool.txt` |
| server | 3 | `SM_GET_PEER_ADDRESS` | `implemented_mapped` | high | Authenticated runtime flow includes GetPeerAddress request/response (code 3) with username and endpoint payload fields. | `captures/redacted/login-peer-address-connect/official_frames.hex` |
| server | 5 | `SM_ADD_USER` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 5 to SM_ADD_USER (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 6 | `SM_REMOVE_USER` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 6 to SM_REMOVE_USER (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 7 | `SM_GET_USER_STATUS` | `implemented_mapped` | high | Authenticated runtime flow includes user-status request/response (code 7) with status and privilege fields. | `captures/redacted/login-user-state/official_frames.hex` |
| server | 10 | `SM_ADD_CHATROOM` | `implemented_mapped` | high | Authenticated runtime flow sends add-chatroom control frame (code 10) with room string payload. | `captures/redacted/login-room-term-control/official_frames.hex` |
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
| server | 28 | `SM_SET_STATUS` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 28 to SM_SET_STATUS (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 32 | `SM_HEARTBEAT` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 32 to SM_HEARTBEAT (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 33 | `SM_SEND_CONNECT_TOKEN` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 33 to SM_SEND_CONNECT_TOKEN (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 34 | `SM_DOWNLOAD_SPEED` | `implemented_mapped` | high | Observed literal in Server::MessageCodeToString dispatch. | `evidence/reverse/server_messagecodetostring_otool.txt` |
| server | 35 | `SM_SHARED_FOLDERS_FILES` | `implemented_mapped` | high | Observed literal in Server::MessageCodeToString dispatch. | `evidence/reverse/server_messagecodetostring_otool.txt` |
| server | 36 | `SM_GET_USER_STATS` | `implemented_mapped` | high | Authenticated runtime flow includes user-stats request/response (code 36) with stats summary fields. | `captures/redacted/login-user-state/official_frames.hex` |
| server | 41 | `SM_RELOGGED` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 41 to SM_RELOGGED (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 42 | `SM_SEARCH_USER_FILES` | `implemented_mapped` | high | Observed outbound runtime frame in login-search scenario (code 42) with user+query payload. | `captures/redacted/login-search/official_frames.hex` |
| server | 50 | `SM_GET_SIMILAR_TERMS` | `implemented_mapped` | high | Observed authenticated runtime request/response for similar-terms discovery flow (code 50). | `captures/redacted/login-similar-terms/official_frames.hex` |
| server | 51 | `SM_ADD_LIKE_TERM` | `implemented_mapped` | high | Authenticated runtime flow sends add-like-term control frame (code 51) with term string payload. | `captures/redacted/login-room-term-control/official_frames.hex` |
| server | 52 | `SM_REMOVE_LIKE_TERM` | `implemented_mapped` | high | Authenticated runtime flow sends remove-like-term control frame (code 52) with term string payload. | `captures/redacted/login-room-term-control/official_frames.hex` |
| server | 54 | `SM_GET_RECOMMENDATIONS` | `implemented_mapped` | high | Observed authenticated runtime recommendations flow (code 54) including outbound request and inbound summary payload. | `captures/redacted/login-recommendations/official_frames.hex` |
| server | 55 | `SM_GET_MY_RECOMMENDATIONS` | `implemented_mapped` | high | Observed outbound authenticated runtime request for my-recommendations (code 55) in recommendation batch. | `captures/redacted/login-recommendations/official_frames.hex` |
| server | 56 | `SM_GET_GLOBAL_RECOMMENDATIONS` | `implemented_mapped` | high | Observed authenticated runtime global-recommendations flow (code 56) with populated inbound payload. | `captures/redacted/login-recommendations/official_frames.hex` |
| server | 57 | `SM_GET_USER_RECOMMENDATIONS` | `implemented_mapped` | high | Observed authenticated runtime user-recommendations flow (code 57) with user request and reply payload. | `captures/redacted/login-user-recommendations/official_frames.hex` |
| server | 58 | `SM_COMMAND` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 58 to SM_COMMAND (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 59 | `SM_PLACE_IN_LINE` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 59 to SM_PLACE_IN_LINE (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 60 | `SM_PLACE_IN_LINE_RESPONSE` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 60 to SM_PLACE_IN_LINE_RESPONSE (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 61 | `SM_USER_LIST` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 61 to SM_USER_LIST (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 62 | `SM_ROOM_ADDED` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 62 to SM_ROOM_ADDED (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 63 | `SM_ROOM_REMOVED` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 63 to SM_ROOM_REMOVED (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 64 | `SM_ROOM_LIST` | `implemented_mapped` | high | Observed runtime room list request/response flow with authenticated session. | `captures/redacted/login-room-list/official_frames.hex` |
| server | 65 | `SM_EXACT_FILE_SEARCH` | `implemented_mapped` | high | Observed outbound runtime frame in login-search scenario (code 65) with exact virtual path payload. | `captures/redacted/login-search/official_frames.hex` |
| server | 66 | `SM_ADMIN_MESSAGE` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 66 to SM_ADMIN_MESSAGE (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 67 | `SM_GLOBAL_USER_LIST` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 67 to SM_GLOBAL_USER_LIST (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 68 | `SM_PEER_MESSAGE` | `implemented_mapped` | high | Deterministic runtime-local legacy tunneled-message capture covers code 68 request/response payload variants; alt compatibility alias 292 observed in same run. | `captures/redacted/login-peer-message/official_frames.hex` |
| server | 69 | `SM_PRIVILEGED_LIST` | `implemented_mapped` | high | Authenticated runtime flow includes privileged-list request and response frames (code 69) with user-list style payloads. | `captures/redacted/login-privilege-messaging/official_frames.hex` |
| server | 70 | `SM_CONNECT_TO_CLIENT` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 70 to SM_CONNECT_TO_CLIENT (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 71 | `SM_SEND_DISTRIBUTIONS` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 71 to SM_SEND_DISTRIBUTIONS (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 73 | `SM_NOTE_PARENT` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 73 to SM_NOTE_PARENT (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 82 | `SM_CHILD_PARENT_MAP` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 82 to SM_CHILD_PARENT_MAP (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 83 | `SM_SET_PARENT_MIN_SPEED` | `implemented_mapped` | high | Authenticated runtime flow observes server-issued parent min speed tuning frame (code 83) with typed `u32` payload. | `captures/redacted/login-parent-distributed-control/official_frames.hex` |
| server | 84 | `SM_SET_PARENT_SPEED_CONNECTION_RATIO` | `implemented_mapped` | high | Authenticated runtime flow observes server-issued parent speed ratio tuning frame (code 84) with typed `u32` payload. | `captures/redacted/login-parent-distributed-control/official_frames.hex` |
| server | 86 | `SM_SET_PARENT_INACTIVITY_BEFORE_DISCONNECT` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 86 to SM_SET_PARENT_INACTIVITY_BEFORE_DISCONNECT (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 87 | `SM_SET_SERVER_INACTIVITY_BEFORE_DISCONNECT` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 87 to SM_SET_SERVER_INACTIVITY_BEFORE_DISCONNECT (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 88 | `SM_NODES_IN_CACHE_BEFORE_DISCONNECT` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 88 to SM_NODES_IN_CACHE_BEFORE_DISCONNECT (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 90 | `SM_SET_SECONDS_BEFORE_PING_CHILDREN` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 90 to SM_SET_SECONDS_BEFORE_PING_CHILDREN (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 91 | `SM_ADD_PRIVILEGED_USER` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 91 to SM_ADD_PRIVILEGED_USER (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 92 | `SM_GET_OWN_PRIVILEGES_STATUS` | `implemented_mapped` | high | Authenticated runtime flow sends own-privileges status request (code 92); spec defines CheckPrivileges response with remaining seconds. | `captures/redacted/login-privileges-social/official_frames.hex` |
| server | 93 | `SM_DNET_MESSAGE` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 93 to SM_DNET_MESSAGE (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 100 | `SM_CAN_PARENT` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 100 to SM_CAN_PARENT (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 102 | `SM_POSSIBLE_PARENTS` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 102 to SM_POSSIBLE_PARENTS (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 103 | `SM_LOW_PRIORITY_FILE_SEARCH` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 103 to SM_LOW_PRIORITY_FILE_SEARCH (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 104 | `SM_WISHLIST_WAIT` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 104 to SM_WISHLIST_WAIT (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 110 | `SM_GET_RECOMMENDED_USERS` | `implemented_mapped` | high | Authenticated runtime flow includes code 110 request/response for similar users with scored user entries. | `captures/redacted/login-privilege-messaging/official_frames.hex` |
| server | 111 | `SM_GET_TERM_RECOMMENDATIONS` | `implemented_mapped` | high | Authenticated runtime flow includes code 111 request with term payload and recommendation-entry response. | `captures/redacted/login-privilege-messaging/official_frames.hex` |
| server | 112 | `SM_GET_RECOMMENDATION_USERS` | `implemented_mapped` | high | Authenticated runtime flow includes code 112 request with term payload and scored-user response entries. | `captures/redacted/login-privilege-messaging/official_frames.hex` |
| server | 113 | `SM_GET_ROOM_TICKER` | `implemented_mapped` | high | Authenticated runtime flow captures request+response multiplexing on code 113 and validates typed room ticker payload decoding. | `captures/redacted/login-parent-distributed-control/official_frames.hex` |
| server | 114 | `SM_ROOM_TICKER_USER_ADDED` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 114 to SM_ROOM_TICKER_USER_ADDED (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 115 | `SM_ROOM_TICKER_USER_REMOVED` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 115 to SM_ROOM_TICKER_USER_REMOVED (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 116 | `SM_SET_TICKER` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 116 to SM_SET_TICKER (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 117 | `SM_ADD_HATE_TERM` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 117 to SM_ADD_HATE_TERM (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 118 | `SM_REMOVE_HATE_TERM` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 118 to SM_REMOVE_HATE_TERM (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 120 | `SM_SEARCH_ROOM` | `implemented_mapped` | high | Observed outbound runtime frame in login-search scenario (code 120) with room+query payload. | `captures/redacted/login-search/official_frames.hex` |
| server | 121 | `SM_UPLOAD_SPEED` | `implemented_mapped` | high | Authenticated runtime flow sends upload-speed control frame (code 121) with typed `bytes_per_sec` payload. | `captures/redacted/login-parent-distributed-control/official_frames.hex` |
| server | 122 | `SM_GET_USER_PRIVILEGES_STATUS` | `implemented_mapped` | high | Authenticated runtime flow captures both request and response on code 122 with typed `username + privileged` semantics. | `captures/redacted/login-parent-distributed-control/official_frames.hex` |
| server | 123 | `SM_GIVE_PRIVILEGE` | `implemented_mapped` | high | Authenticated runtime flow sends give-privilege request (code 123) with username+days payload. | `captures/redacted/login-privileges-social/official_frames.hex` |
| server | 124 | `SM_INFORM_USER_OF_PRIVILEGES` | `implemented_mapped` | high | Authenticated runtime flow sends notify-privileges request (code 124) with token+username payload. | `captures/redacted/login-privileges-social/official_frames.hex` |
| server | 125 | `SM_INFORM_USER_OF_PRIVILEGES_ACK` | `implemented_mapped` | high | Authenticated runtime flow sends notify-privileges ack (code 125) with token payload. | `captures/redacted/login-privileges-social/official_frames.hex` |
| server | 126 | `SM_DNET_LEVEL` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 126 to SM_DNET_LEVEL (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 127 | `SM_DNET_GROUP_LEADER` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 127 to SM_DNET_GROUP_LEADER (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 128 | `SM_DNET_DELIVERY_REPORT` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 128 to SM_DNET_DELIVERY_REPORT (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 129 | `SM_DNET_CHILD_DEPTH` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 129 to SM_DNET_CHILD_DEPTH (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 130 | `SM_DNET_RESET` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 130 to SM_DNET_RESET (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 131 | `SM_FLOOD` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 131 to SM_FLOOD (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 132 | `SM_BAN_USER` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 132 to SM_BAN_USER (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 133 | `SM_ROOM_MEMBERS` | `implemented_mapped` | high | Observed runtime room-members request flow in authenticated room session. | `captures/redacted/login-join-room-presence/official_frames.hex` |
| server | 134 | `SM_ADD_ROOM_MEMBER` | `implemented_mapped` | high | Server code 134 AddUserToPrivileged with room+username payload (code resolved from SLSK protocol spec) and symbol confirmed in binary string table. | `evidence/reverse/message_name_strings.txt` |
| server | 135 | `SM_REMOVE_ROOM_MEMBER` | `implemented_mapped` | high | Server code 135 RemoveUserFromPrivileged with room+username payload (code resolved from SLSK protocol spec) and symbol confirmed in binary string table. | `evidence/reverse/message_name_strings.txt` |
| server | 136 | `SM_REMOVE_OWN_ROOM_MEMBERSHIP` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 136 to SM_REMOVE_OWN_ROOM_MEMBERSHIP (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 137 | `SM_GIVE_UP_ROOM` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 137 to SM_GIVE_UP_ROOM (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 138 | `SM_TRANSFER_ROOM_OWNERSHIP` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 138 to SM_TRANSFER_ROOM_OWNERSHIP (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 139 | `SM_ADD_ROOM_MEMBERSHIP` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 139 to SM_ADD_ROOM_MEMBERSHIP (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 140 | `SM_REMOVE_ROOM_MEMBERSHIP` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 140 to SM_REMOVE_ROOM_MEMBERSHIP (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 141 | `SM_ENABLE_PRIVATE_ROOM_ADD` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 141 to SM_ENABLE_PRIVATE_ROOM_ADD (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 142 | `SM_CHANGE_PASSWORD` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 142 to SM_CHANGE_PASSWORD (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 143 | `SM_ADD_ROOM_OPERATOR` | `implemented_mapped` | high | Server code 143 AddOperatorToPrivileged with room+username payload (code resolved from SLSK protocol spec) and symbol confirmed in binary string table. | `evidence/reverse/message_name_strings.txt` |
| server | 144 | `SM_REMOVE_ROOM_OPERATOR` | `implemented_mapped` | high | Server code 144 RemoveOperatorFromPrivileged with room+username payload (code resolved from SLSK protocol spec) and symbol confirmed in binary string table. | `evidence/reverse/message_name_strings.txt` |
| server | 145 | `SM_ADD_ROOM_OPERATORSHIP` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 145 to SM_ADD_ROOM_OPERATORSHIP (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 146 | `SM_REMOVE_ROOM_OPERATORSHIP` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 146 to SM_REMOVE_ROOM_OPERATORSHIP (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 147 | `SM_REMOVE_OWN_ROOM_OPERATORSHIP` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 147 to SM_REMOVE_OWN_ROOM_OPERATORSHIP (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 148 | `SM_ROOM_OPERATORS` | `implemented_mapped` | high | Observed runtime room-operators request flow in authenticated room session. | `captures/redacted/login-join-room-presence/official_frames.hex` |
| server | 149 | `SM_MESSAGE_USERS` | `implemented_mapped` | high | Authenticated runtime flow sends message-users payload (code 149) with user list and message body. | `captures/redacted/login-message-users/official_frames.hex` |
| server | 150 | `SM_JOIN_GLOBAL_ROOM` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 150 to SM_JOIN_GLOBAL_ROOM (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 151 | `SM_LEAVE_GLOBAL_ROOM` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 151 to SM_LEAVE_GLOBAL_ROOM (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 152 | `SM_SAY_GLOBAL_ROOM` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 152 to SM_SAY_GLOBAL_ROOM (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 153 | `SM_SEARCH_CORRELATIONS` | `implemented_mapped` | high | Server MessageCodeToString jump-table extraction resolves code 153 to SM_SEARCH_CORRELATIONS (x86_64 binary disassembly). | `evidence/reverse/message_codes_jump_table.md` |
| server | 292 | `SM_PEER_MESSAGE_ALT` | `implemented_mapped` | high | Observed runtime alias code 292 for peer message tunneling in login-peer-message flow. | `captures/redacted/login-peer-message/official_frames.hex` |

## Regeneration

```bash
python3 tools/protocol/generate_protocol_matrix.py
```
