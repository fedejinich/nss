# Message Schema

- Generated: `2026-02-14T03:58:52+00:00`
- Framing: `<u32 frame_len_le><u32 message_code_le><payload>`
- Framing confidence: `medium`

## Messages

### `peer` `PM_TRANSFER_REQUEST` (code `40`)
- Confidence: `high`
- Payload fields:
  - `direction`: `u32`
  - `token`: `u32`
  - `virtual_path`: `string`
  - `file_size`: `u64`
- Evidence:
  - `string`: `evidence/reverse/peer_messagecodetostring_otool.txt` (Observed literal in Peer MessageCodeToString dispatch.)
  - `ghidra_decompile`: `evidence/reverse/disasm/transfer_on_file_request.txt` (Transfer queue dispatcher handles peer transfer negotiation path.)

### `peer` `PM_TRANSFER_RESPONSE` (code `41`)
- Confidence: `high`
- Payload fields:
  - `token`: `u32`
  - `allowed`: `bool_u32`
  - `queue_or_reason`: `string`
- Evidence:
  - `string`: `evidence/reverse/peer_messagecodetostring_otool.txt` (Observed literal in Peer MessageCodeToString dispatch.)
  - `ghidra_decompile`: `evidence/reverse/disasm/transfer_on_file_request.txt` (Transfer queue dispatcher handles peer transfer negotiation path.)

### `server` `SM_LOGIN` (code `1`)
- Confidence: `high`
- Payload fields:
  - `username`: `string`
  - `password_md5`: `string`
  - `client_version`: `u32`
  - `minor_version`: `u32`
- Evidence:
  - `string`: `evidence/reverse/server_messagecodetostring_otool.txt` (Observed literal in MessageCodeToString dispatch.)

### `server` `SM_SET_WAIT_PORT` (code `2`)
- Confidence: `high`
- Payload fields: pending derivation
- Evidence:
  - `string`: `evidence/reverse/server_messagecodetostring_otool.txt` (Observed literal in MessageCodeToString dispatch.)

### `server` `SM_GET_PEER_ADDRESS` (code `3`)
- Confidence: `high`
- Payload fields: pending derivation
- Evidence:
  - `string`: `evidence/reverse/server_messagecodetostring_otool.txt` (Observed literal in MessageCodeToString dispatch.)

### `server` `SM_FILE_SEARCH` (code `26`)
- Confidence: `high`
- Payload fields:
  - `search_token`: `u32`
  - `search_text`: `string`
- Evidence:
  - `ghidra_decompile`: `evidence/reverse/disasm/server_file_search.txt` (FileSearch writes constant 0x1a before Server::SendMessage.)
  - `ghidra_decompile`: `evidence/reverse/disasm/server_file_search.txt` (Function writes constant 0x1a before serializing search payload.)

