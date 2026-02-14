# Message Schema

- Generated: `2026-02-14T15:34:11+00:00`
- Framing: `<u32 frame_len_le><u32 message_code_le><payload>`
- Framing confidence: `medium`
- Coverage contract: `high >= 18` `medium <= 7` `low <= 0`
- Current coverage: `high=25` `medium=0` `low=0`

## Messages

### `peer` `PM_GET_SHARED_FILE_LIST` (code `4`)
- Confidence: `high`
- Payload fields:
  - `username`: `string`
- Evidence:
  - `string`: `evidence/reverse/peer_messagecodetostring_otool.txt` (Observed literal in PeerMessenger::MessageCodeToString dispatch.)

### `peer` `PM_SHARED_FILE_LIST` (code `5`)
- Confidence: `high`
- Payload fields:
  - `entries`: `array<shared_file_entry>`
  - `entry.virtual_path`: `string`
  - `entry.size`: `u64`
- Evidence:
  - `string`: `evidence/reverse/peer_messagecodetostring_otool.txt` (Observed literal in PeerMessenger::MessageCodeToString dispatch.)

### `peer` `PM_FILE_SEARCH_REQUEST` (code `8`)
- Confidence: `high`
- Payload fields:
  - `token`: `u32`
  - `query`: `string`
- Evidence:
  - `string`: `evidence/reverse/peer_messagecodetostring_otool.txt` (Observed literal in PeerMessenger::MessageCodeToString dispatch.)

### `peer` `PM_FILE_SEARCH_RESULT` (code `9`)
- Confidence: `high`
- Payload fields:
  - `token`: `u32`
  - `username`: `string`
  - `result_count`: `u32`
- Evidence:
  - `runtime_capture`: `captures/redacted/login-search-download/official_frames.hex` (Observed inbound runtime frame in login-search-download scenario (code 9) with token+user+result_count.)

### `peer` `PM_USER_INFO_REQUEST` (code `15`)
- Confidence: `high`
- Payload fields: pending derivation
- Evidence:
  - `manual_note`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Peer code 15 UserInfoRequest with empty payload; symbol confirmed in peer message code table.)
  - `string`: `evidence/reverse/peer_messagecodetostring_otool.txt` (Peer MessageCodeToString includes PM_USER_INFO_REQUEST.)
  - `spec`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Peer code 15 documents user info request as empty payload.)

### `peer` `PM_USER_INFO_REPLY` (code `16`)
- Confidence: `high`
- Payload fields:
  - `description`: `string`
  - `has_picture`: `bool_u8`
  - `picture`: `bytes_len_prefixed`
  - `total_uploads`: `u32`
  - `queue_size`: `u32`
  - `slots_free`: `bool_u8`
  - `upload_permissions`: `optional_u32`
- Evidence:
  - `manual_note`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Peer code 16 UserInfoReply with description/picture/uploads/queue/slots fields; symbol confirmed in peer message code table.)
  - `string`: `evidence/reverse/peer_messagecodetostring_otool.txt` (Peer MessageCodeToString includes PM_USER_INFO_REPLY.)
  - `spec`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Peer code 16 documents user info reply payload fields.)

### `peer` `PM_TRANSFER_REQUEST` (code `40`)
- Confidence: `high`
- Payload fields:
  - `direction`: `enum_u32`
  - `token`: `u32`
  - `virtual_path`: `string`
  - `file_size`: `u64`
- Evidence:
  - `ghidra_decompile`: `evidence/reverse/disasm/transfer_on_file_request.txt` (Transfer queue dispatcher handles transfer request negotiation.)
  - `ghidra_decompile`: `evidence/reverse/disasm/transfer_on_file_request.txt` (Transfer queue dispatcher handles transfer request negotiation path.)

### `peer` `PM_TRANSFER_RESPONSE` (code `41`)
- Confidence: `high`
- Payload fields:
  - `token`: `u32`
  - `allowed`: `bool_u32`
  - `queue_or_reason`: `string`
- Evidence:
  - `ghidra_decompile`: `evidence/reverse/disasm/transfer_on_file_request.txt` (Transfer queue dispatcher handles transfer response negotiation.)
  - `ghidra_decompile`: `evidence/reverse/disasm/transfer_on_file_request.txt` (Transfer queue dispatcher handles transfer response negotiation path.)

### `peer` `PM_QUEUE_UPLOAD` (code `43`)
- Confidence: `high`
- Payload fields:
  - `username`: `string`
  - `virtual_path`: `string`
- Evidence:
  - `string`: `evidence/reverse/peer_messagecodetostring_otool.txt` (Observed literal in PeerMessenger::MessageCodeToString dispatch.)
  - `ghidra_decompile`: `evidence/reverse/disasm/transfer_on_queue_download.txt` (Queue manager records upload queueing for pending peers.)

### `peer` `PM_UPLOAD_PLACE_IN_LINE` (code `44`)
- Confidence: `high`
- Payload fields:
  - `username`: `string`
  - `virtual_path`: `string`
  - `place`: `u32`
- Evidence:
  - `runtime_capture`: `captures/redacted/upload-deny/official_frames.hex` (Observed inbound runtime frame in upload-deny scenario (code 44) with queue place payload.)

### `peer` `PM_UPLOAD_FAILED` (code `46`)
- Confidence: `high`
- Payload fields:
  - `username`: `string`
  - `virtual_path`: `string`
  - `reason`: `string`
- Evidence:
  - `runtime_capture`: `captures/redacted/upload-deny/official_frames.hex` (Observed inbound runtime frame in upload-deny scenario (code 46) with failure reason payload.)
  - `ghidra_decompile`: `evidence/reverse/disasm/upload_write_socket.txt` (Upload send path emits failure branch when transfer cannot continue.)

### `peer` `PM_EXACT_FILE_SEARCH_REQUEST` (code `47`)
- Confidence: `medium`
- Payload fields:
  - `token`: `optional_u32`
  - `query`: `string`
- Evidence:
  - `manual_note`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Peer code list maps 47 to ExactFileSearchRequest; payload shape treated as legacy optional-token+query until runtime evidence is captured.)
  - `string`: `evidence/reverse/peer_messagecodetostring_otool.txt` (Peer MessageCodeToString includes PM_EXACT_FILE_SEARCH_REQUEST.)
  - `spec`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Peer code list includes code 47 for ExactFileSearchRequest (legacy/obsolete family).)

### `peer` `PM_INDIRECT_FILE_SEARCH_REQUEST` (code `49`)
- Confidence: `medium`
- Payload fields:
  - `token`: `optional_u32`
  - `query`: `string`
- Evidence:
  - `manual_note`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Peer code list maps 49 to IndirectFileSearchRequest; payload shape treated as legacy optional-token+query until runtime evidence is captured.)
  - `string`: `evidence/reverse/peer_messagecodetostring_otool.txt` (Peer MessageCodeToString includes PM_INDIRECT_FILE_SEARCH_REQUEST.)
  - `spec`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Peer code list includes code 49 for IndirectFileSearchRequest (legacy/obsolete family).)

### `peer` `PM_UPLOAD_DENIED` (code `50`)
- Confidence: `high`
- Payload fields:
  - `username`: `string`
  - `virtual_path`: `string`
  - `reason`: `string`
- Evidence:
  - `runtime_capture`: `captures/redacted/upload-deny/official_frames.hex` (Observed inbound runtime frame in upload-deny scenario (code 50) with deny reason payload.)
  - `ghidra_decompile`: `evidence/reverse/disasm/upload_write_socket.txt` (Upload send path emits deny branch for rejected requests.)

### `peer` `PM_UPLOAD_PLACE_IN_LINE_REQUEST` (code `51`)
- Confidence: `high`
- Payload fields:
  - `virtual_path`: `string`
- Evidence:
  - `manual_note`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Peer code 51 PlaceInQueueRequest with filename/path payload; symbol confirmed in peer message code table.)
  - `string`: `evidence/reverse/peer_messagecodetostring_otool.txt` (Peer MessageCodeToString includes PM_UPLOAD_PLACE_IN_LINE_REQUEST.)
  - `spec`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Peer code 51 documents place-in-line request carrying filename/path.)

### `server` `SM_LOGIN` (code `1`)
- Confidence: `high`
- Payload fields:
  - `username`: `string`
  - `password`: `string`
  - `client_version`: `u32`
  - `md5hash`: `string`
  - `minor_version`: `u32`
- Evidence:
  - `runtime_capture`: `captures/redacted/login-only/official_frames.hex` (Observed authenticated runtime login request+response (code 1) with success payload on tuple 160/1.)

### `server` `SM_SET_WAIT_PORT` (code `2`)
- Confidence: `high`
- Payload fields:
  - `listen_port`: `u32`
- Evidence:
  - `string`: `evidence/reverse/server_messagecodetostring_otool.txt` (Observed literal in Server::MessageCodeToString dispatch.)

### `server` `SM_GET_PEER_ADDRESS` (code `3`)
- Confidence: `high`
- Payload fields:
  - `username`: `string`
- Evidence:
  - `string`: `evidence/reverse/server_messagecodetostring_otool.txt` (Observed literal in Server::MessageCodeToString dispatch.)

### `server` `SM_GET_USER_STATUS` (code `7`)
- Confidence: `high`
- Payload fields:
  - `username`: `string`
- Evidence:
  - `string`: `evidence/reverse/server_messagecodetostring_otool.txt` (Observed literal in Server::MessageCodeToString dispatch.)

### `server` `SM_SAY_CHATROOM` (code `13`)
- Confidence: `high`
- Payload fields:
  - `room`: `string`
  - `username`: `optional_string`
  - `message`: `string`
- Evidence:
  - `runtime_capture`: `captures/redacted/login-join-room-presence/official_frames.hex` (Observed runtime room message flow: outbound SAY request and inbound room chat event in authenticated room session.)

### `server` `SM_JOIN_ROOM` (code `14`)
- Confidence: `high`
- Payload fields:
  - `room`: `string`
  - `users`: `array<string>`
- Evidence:
  - `runtime_capture`: `captures/redacted/login-join-room-presence/official_frames.hex` (Observed runtime join-room request and server join payload during authenticated room session.)

### `server` `SM_LEAVE_ROOM` (code `15`)
- Confidence: `high`
- Payload fields:
  - `room`: `string`
- Evidence:
  - `runtime_capture`: `captures/redacted/login-leave-room/official_frames.hex` (Observed runtime leave-room request and server leave acknowledgement payload.)

### `server` `SM_USER_JOINED_ROOM` (code `16`)
- Confidence: `high`
- Payload fields:
  - `room`: `string`
  - `username`: `string`
- Evidence:
  - `runtime_capture`: `captures/redacted/login-join-room-presence/official_frames.hex` (Observed runtime user-joined room presence events while watching authenticated room session.)

### `server` `SM_USER_LEFT_ROOM` (code `17`)
- Confidence: `high`
- Payload fields:
  - `room`: `string`
  - `username`: `string`
- Evidence:
  - `runtime_capture`: `captures/redacted/login-join-room-presence/official_frames.hex` (Observed runtime user-left room presence events while watching authenticated room session.)

### `server` `SM_CONNECT_TO_PEER` (code `18`)
- Confidence: `high`
- Payload fields:
  - `username`: `string`
  - `token`: `u32`
- Evidence:
  - `string`: `evidence/reverse/server_messagecodetostring_otool.txt` (Observed literal in Server::MessageCodeToString dispatch and peer connect path.)
  - `ghidra_decompile`: `evidence/reverse/disasm/server_handle_message.txt` (Server handler routes peer connect responses to transfer subsystem.)

### `server` `SM_MESSAGE_USER` (code `22`)
- Confidence: `high`
- Payload fields:
  - `username`: `string`
  - `message`: `string`
- Evidence:
  - `string`: `evidence/reverse/server_messagecodetostring_otool.txt` (Observed literal in Server::MessageCodeToString dispatch.)

### `server` `SM_MESSAGE_ACKED` (code `23`)
- Confidence: `high`
- Payload fields:
  - `message_id`: `u32`
- Evidence:
  - `string`: `evidence/reverse/server_messagecodetostring_otool.txt` (Observed literal in Server::MessageCodeToString dispatch.)

### `server` `SM_FILE_SEARCH` (code `26`)
- Confidence: `high`
- Payload fields:
  - `search_token`: `u32`
  - `search_text`: `string`
- Evidence:
  - `ghidra_decompile`: `evidence/reverse/disasm/server_file_search.txt` (FileSearch writes constant 0x1a before Server::SendMessage.)
  - `ghidra_decompile`: `evidence/reverse/disasm/server_file_search.txt` (Function writes constant 0x1a before serializing search payload.)
  - `ghidra_decompile`: `evidence/reverse/disasm/server_prepare_search.txt` (PrepareSearch normalizes and emits search tokens/strings.)

### `server` `SM_DOWNLOAD_SPEED` (code `34`)
- Confidence: `high`
- Payload fields:
  - `bytes_per_sec`: `u32`
- Evidence:
  - `string`: `evidence/reverse/server_messagecodetostring_otool.txt` (Observed literal in Server::MessageCodeToString dispatch.)

### `server` `SM_SHARED_FOLDERS_FILES` (code `35`)
- Confidence: `high`
- Payload fields:
  - `folder_count`: `u32`
  - `file_count`: `u32`
- Evidence:
  - `string`: `evidence/reverse/server_messagecodetostring_otool.txt` (Observed literal in Server::MessageCodeToString dispatch.)

### `server` `SM_GET_USER_STATS` (code `36`)
- Confidence: `high`
- Payload fields:
  - `username`: `string`
- Evidence:
  - `string`: `evidence/reverse/server_messagecodetostring_otool.txt` (Observed literal in Server::MessageCodeToString dispatch.)

### `server` `SM_SEARCH_USER_FILES` (code `42`)
- Confidence: `high`
- Payload fields:
  - `username`: `string`
  - `search_text`: `string`
- Evidence:
  - `runtime_capture`: `captures/redacted/login-search/official_frames.hex` (Observed outbound runtime frame in login-search scenario (code 42) with user+query payload.)

### `server` `SM_GET_SIMILAR_TERMS` (code `50`)
- Confidence: `high`
- Payload fields:
  - `term`: `string`
  - `recommendation_count`: `u32`
  - `recommendation.term`: `string`
  - `recommendation.score`: `i32`
- Evidence:
  - `runtime_capture`: `captures/redacted/login-similar-terms/official_frames.hex` (Observed authenticated runtime request/response for similar-terms discovery flow (code 50).)
  - `string`: `evidence/reverse/message_name_strings.txt` (Server string table includes SM_GET_SIMILAR_TERMS.)
  - `spec`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Protocol list documents similar recommendation request/response message family.)

### `server` `SM_GET_RECOMMENDATIONS` (code `54`)
- Confidence: `high`
- Payload fields:
  - `recommendation_count`: `u32`
  - `recommendation.term`: `string`
  - `recommendation.score`: `i32`
  - `unrecommendation_count`: `u32`
  - `unrecommendation.term`: `string`
  - `unrecommendation.score`: `i32`
- Evidence:
  - `runtime_capture`: `captures/redacted/login-recommendations/official_frames.hex` (Observed authenticated runtime recommendations flow (code 54) including outbound request and inbound summary payload.)
  - `string`: `evidence/reverse/server_messagecodetostring_otool.txt` (Server MessageCodeToString includes SM_GET_RECOMMENDATIONS.)
  - `spec`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Protocol list documents recommendation request/response code mapping.)

### `server` `SM_GET_MY_RECOMMENDATIONS` (code `55`)
- Confidence: `high`
- Payload fields:
  - `recommendation_count`: `u32`
  - `recommendation.term`: `string`
  - `recommendation.score`: `i32`
  - `unrecommendation_count`: `u32`
  - `unrecommendation.term`: `string`
  - `unrecommendation.score`: `i32`
- Evidence:
  - `runtime_capture`: `captures/redacted/login-recommendations/official_frames.hex` (Observed outbound authenticated runtime request for my-recommendations (code 55) in recommendation batch.)
  - `string`: `evidence/reverse/server_messagecodetostring_otool.txt` (Server MessageCodeToString includes SM_GET_MY_RECOMMENDATIONS.)
  - `spec`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Protocol list documents my-recommendations request code mapping.)

### `server` `SM_GET_GLOBAL_RECOMMENDATIONS` (code `56`)
- Confidence: `high`
- Payload fields:
  - `recommendation_count`: `u32`
  - `recommendation.term`: `string`
  - `recommendation.score`: `i32`
  - `unrecommendation_count`: `u32`
  - `unrecommendation.term`: `string`
  - `unrecommendation.score`: `i32`
- Evidence:
  - `runtime_capture`: `captures/redacted/login-recommendations/official_frames.hex` (Observed authenticated runtime global-recommendations flow (code 56) with populated inbound payload.)
  - `string`: `evidence/reverse/server_messagecodetostring_otool.txt` (Server MessageCodeToString includes SM_GET_GLOBAL_RECOMMENDATIONS.)
  - `spec`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Protocol list documents global recommendations message mapping.)

### `server` `SM_GET_USER_RECOMMENDATIONS` (code `57`)
- Confidence: `high`
- Payload fields:
  - `username`: `string`
  - `recommendation_count`: `u32`
  - `recommendation.term`: `string`
  - `recommendation.score`: `i32`
  - `unrecommendation_count`: `u32`
  - `unrecommendation.term`: `string`
  - `unrecommendation.score`: `i32`
- Evidence:
  - `runtime_capture`: `captures/redacted/login-user-recommendations/official_frames.hex` (Observed authenticated runtime user-recommendations flow (code 57) with user request and reply payload.)
  - `string`: `evidence/reverse/server_messagecodetostring_otool.txt` (Server MessageCodeToString includes SM_GET_USER_RECOMMENDATIONS.)
  - `spec`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Protocol list documents user recommendation/interests message mapping.)

### `server` `SM_ROOM_LIST` (code `64`)
- Confidence: `high`
- Payload fields:
  - `room_count`: `u32`
  - `rooms`: `array<string>`
- Evidence:
  - `runtime_capture`: `captures/redacted/login-room-list/official_frames.hex` (Observed runtime room list request/response flow with authenticated session.)

### `server` `SM_EXACT_FILE_SEARCH` (code `65`)
- Confidence: `high`
- Payload fields:
  - `virtual_path`: `string`
- Evidence:
  - `runtime_capture`: `captures/redacted/login-search/official_frames.hex` (Observed outbound runtime frame in login-search scenario (code 65) with exact virtual path payload.)

### `server` `SM_SEARCH_ROOM` (code `120`)
- Confidence: `high`
- Payload fields:
  - `room`: `string`
  - `search_text`: `string`
- Evidence:
  - `runtime_capture`: `captures/redacted/login-search/official_frames.hex` (Observed outbound runtime frame in login-search scenario (code 120) with room+query payload.)

### `server` `SM_UPLOAD_SPEED` (code `121`)
- Confidence: `high`
- Payload fields:
  - `bytes_per_sec`: `u32`
- Evidence:
  - `string`: `evidence/reverse/message_name_strings.txt` (String present and mirrored by upload code paths.)

### `server` `SM_ROOM_MEMBERS` (code `133`)
- Confidence: `high`
- Payload fields:
  - `room`: `string`
  - `users`: `array<string>`
- Evidence:
  - `runtime_capture`: `captures/redacted/login-join-room-presence/official_frames.hex` (Observed runtime room-members request flow in authenticated room session.)

### `server` `SM_ADD_ROOM_MEMBER` (code `134`)
- Confidence: `high`
- Payload fields:
  - `room`: `string`
  - `username`: `string`
- Evidence:
  - `manual_note`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Server code 134 AddUserToPrivileged with room+username payload; symbol confirmed in binary string table.)
  - `string`: `evidence/reverse/message_name_strings.txt` (Server string table includes SM_ADD_ROOM_MEMBER.)
  - `spec`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Server code 134 documents AddUserToPrivileged operation with room+username fields.)

### `server` `SM_REMOVE_ROOM_MEMBER` (code `135`)
- Confidence: `high`
- Payload fields:
  - `room`: `string`
  - `username`: `string`
- Evidence:
  - `manual_note`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Server code 135 RemoveUserFromPrivileged with room+username payload; symbol confirmed in binary string table.)
  - `string`: `evidence/reverse/message_name_strings.txt` (Server string table includes SM_REMOVE_ROOM_MEMBER.)
  - `spec`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Server code 135 documents RemoveUserFromPrivileged operation with room+username fields.)

### `server` `SM_ADD_ROOM_OPERATOR` (code `143`)
- Confidence: `high`
- Payload fields:
  - `room`: `string`
  - `username`: `string`
- Evidence:
  - `manual_note`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Server code 143 AddOperatorToPrivileged with room+username payload; symbol confirmed in binary string table.)
  - `string`: `evidence/reverse/message_name_strings.txt` (Server string table includes SM_ADD_ROOM_OPERATOR.)
  - `spec`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Server code 143 documents AddOperatorToPrivileged operation with room+username fields.)

### `server` `SM_REMOVE_ROOM_OPERATOR` (code `144`)
- Confidence: `high`
- Payload fields:
  - `room`: `string`
  - `username`: `string`
- Evidence:
  - `manual_note`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Server code 144 RemoveOperatorFromPrivileged with room+username payload; symbol confirmed in binary string table.)
  - `string`: `evidence/reverse/message_name_strings.txt` (Server string table includes SM_REMOVE_ROOM_OPERATOR.)
  - `spec`: `https://nicotine-plus.org/doc/SLSKPROTOCOL.html` (Server code 144 documents RemoveOperatorFromPrivileged operation with room+username fields.)

### `server` `SM_ROOM_OPERATORS` (code `148`)
- Confidence: `high`
- Payload fields:
  - `room`: `string`
  - `operators`: `array<string>`
- Evidence:
  - `runtime_capture`: `captures/redacted/login-join-room-presence/official_frames.hex` (Observed runtime room-operators request flow in authenticated room session.)

