# Message Schema

- Generated: `2026-02-14T04:58:01+00:00`
- Framing: `<u32 frame_len_le><u32 message_code_le><payload>`
- Framing confidence: `medium`
- Coverage contract: `high >= 18` `medium <= 7` `low <= 0`
- Current coverage: `high=18` `medium=7` `low=0`

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
- Confidence: `medium`
- Payload fields:
  - `token`: `u32`
  - `username`: `string`
  - `result_count`: `u32`
- Evidence:
  - `string`: `evidence/reverse/peer_messagecodetostring_otool.txt` (Literal dispatch identified; complex payload list normalization pending.)

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
- Confidence: `medium`
- Payload fields:
  - `username`: `string`
  - `virtual_path`: `string`
  - `place`: `u32`
- Evidence:
  - `string`: `evidence/reverse/peer_messagecodetostring_otool.txt` (Literal dispatch identified; queue position payload pending runtime confirmation.)

### `peer` `PM_UPLOAD_FAILED` (code `46`)
- Confidence: `medium`
- Payload fields:
  - `username`: `string`
  - `virtual_path`: `string`
  - `reason`: `string`
- Evidence:
  - `string`: `evidence/reverse/peer_messagecodetostring_otool.txt` (Literal dispatch identified; runtime reasons pending.)
  - `ghidra_decompile`: `evidence/reverse/disasm/upload_write_socket.txt` (Upload send path emits failure branch when transfer cannot continue.)

### `peer` `PM_UPLOAD_DENIED` (code `50`)
- Confidence: `medium`
- Payload fields:
  - `username`: `string`
  - `virtual_path`: `string`
  - `reason`: `string`
- Evidence:
  - `string`: `evidence/reverse/peer_messagecodetostring_otool.txt` (Literal dispatch identified; runtime reasons pending.)
  - `ghidra_decompile`: `evidence/reverse/disasm/upload_write_socket.txt` (Upload send path emits deny branch for rejected requests.)

### `server` `SM_LOGIN` (code `1`)
- Confidence: `high`
- Payload fields:
  - `username`: `string`
  - `password_md5`: `string`
  - `client_version`: `u32`
  - `minor_version`: `u32`
- Evidence:
  - `string`: `evidence/reverse/server_messagecodetostring_otool.txt` (Observed literal in Server::MessageCodeToString dispatch.)

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
- Confidence: `medium`
- Payload fields:
  - `username`: `string`
  - `search_text`: `string`
- Evidence:
  - `string`: `evidence/reverse/server_messagecodetostring_otool.txt` (Literal dispatch identified; payload details pending runtime confirmation.)

### `server` `SM_EXACT_FILE_SEARCH` (code `65`)
- Confidence: `medium`
- Payload fields:
  - `virtual_path`: `string`
- Evidence:
  - `string`: `evidence/reverse/message_name_strings.txt` (String present in binary; payload shape pending runtime confirmation.)

### `server` `SM_SEARCH_ROOM` (code `120`)
- Confidence: `medium`
- Payload fields:
  - `room`: `string`
  - `search_text`: `string`
- Evidence:
  - `string`: `evidence/reverse/message_name_strings.txt` (String present in binary; payload shape pending runtime confirmation.)

### `server` `SM_UPLOAD_SPEED` (code `121`)
- Confidence: `high`
- Payload fields:
  - `bytes_per_sec`: `u32`
- Evidence:
  - `string`: `evidence/reverse/message_name_strings.txt` (String present and mirrored by upload code paths.)

