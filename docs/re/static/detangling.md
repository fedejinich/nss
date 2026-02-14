# Detangling Notes

This page tracks approved mappings and pending review candidates for SoulseekQt reverse engineering.

## Approved Function Renames

### `Server::SendMessage(MemStream&, bool)` -> `server_send_message`
- Binary: `SoulseekQt`
- Address: `0x100054dac`
- Confidence: `high`
- Status: `approved`
- Evidence:
  - `xref`: `evidence/reverse/search_download_symbols_nm.txt` (Symbolized server transmit entrypoint.)
  - `ghidra_decompile`: `evidence/reverse/disasm/server_send_message.txt` (MemStream payload is serialized and flushed to socket.)

### `Server::FileSearch(QString, QString)` -> `server_file_search`
- Binary: `SoulseekQt`
- Address: `0x100060fa0`
- Confidence: `high`
- Status: `approved`
- Evidence:
  - `xref`: `evidence/reverse/search_download_symbols_nm.txt` (Demangled symbol at fixed text address.)
  - `ghidra_decompile`: `evidence/reverse/disasm/server_file_search.txt` (Calls PrepareSearch and SendMessage with SM_FILE_SEARCH code path.)

### `FUN_10006c590` -> `server_message_code_to_string`
- Binary: `SoulseekQt`
- Address: `0x10006c590`
- Confidence: `high`
- Status: `approved`
- Evidence:
  - `ghidra_decompile`: `evidence/reverse/server_messagecodetostring_otool.txt` (Jump table maps integer server message codes to SM_* literals.)
  - `string`: `evidence/reverse/message_name_strings.txt` (SM_* literals present in binary strings.)

### `PeerMessenger::QueueDownload(QString, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>)` -> `peer_queue_download`
- Binary: `SoulseekQt`
- Address: `0x1000a4474`
- Confidence: `high`
- Status: `approved`
- Evidence:
  - `xref`: `evidence/reverse/search_download_symbols_nm.txt` (Peer download enqueue symbol found in text section.)
  - `ghidra_decompile`: `evidence/reverse/disasm/peer_queue_download.txt` (Queue download request preparation before peer send.)

### `TransferQueueManager::OnFileTransferRequest(QString, int, unsigned int, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, long long)` -> `transfer_queue_on_file_transfer_request`
- Binary: `SoulseekQt`
- Address: `0x1000d7114`
- Confidence: `high`
- Status: `approved`
- Evidence:
  - `xref`: `evidence/reverse/search_download_symbols_nm.txt` (Transfer negotiation dispatcher symbol.)
  - `ghidra_decompile`: `evidence/reverse/disasm/transfer_on_file_request.txt` (Handles transfer request/response state and task dispatch.)

## Approved Data Labels

No approved data labels yet.

## Review Queue

Review queue is empty.

## Protocol Coverage (Stage 2)

- Message rows: `33`
- Confidence split: `high=33` `medium=0` `low=0`
- Latest mapped messages:
  - `server` `SM_LOGIN` code `1` confidence `high`
  - `server` `SM_SET_WAIT_PORT` code `2` confidence `high`
  - `server` `SM_GET_PEER_ADDRESS` code `3` confidence `high`
  - `server` `SM_CONNECT_TO_PEER` code `18` confidence `high`
  - `server` `SM_SAY_CHATROOM` code `13` confidence `high`
  - `server` `SM_JOIN_ROOM` code `14` confidence `high`
  - `server` `SM_LEAVE_ROOM` code `15` confidence `high`
  - `server` `SM_USER_JOINED_ROOM` code `16` confidence `high`
  - `server` `SM_USER_LEFT_ROOM` code `17` confidence `high`
  - `server` `SM_FILE_SEARCH` code `26` confidence `high`
  - `server` `SM_ROOM_LIST` code `64` confidence `high`
  - `server` `SM_SEARCH_ROOM` code `120` confidence `high`
  - `server` `SM_EXACT_FILE_SEARCH` code `65` confidence `high`
  - `server` `SM_SEARCH_USER_FILES` code `42` confidence `high`
  - `server` `SM_ROOM_MEMBERS` code `133` confidence `high`
  - `server` `SM_ROOM_OPERATORS` code `148` confidence `high`
  - `server` `SM_MESSAGE_USER` code `22` confidence `high`
  - `server` `SM_MESSAGE_ACKED` code `23` confidence `high`
  - `server` `SM_GET_USER_STATS` code `36` confidence `high`
  - `server` `SM_GET_USER_STATUS` code `7` confidence `high`
  - `server` `SM_SHARED_FOLDERS_FILES` code `35` confidence `high`
  - `server` `SM_DOWNLOAD_SPEED` code `34` confidence `high`
  - `server` `SM_UPLOAD_SPEED` code `121` confidence `high`
  - `peer` `PM_GET_SHARED_FILE_LIST` code `4` confidence `high`
  - `peer` `PM_SHARED_FILE_LIST` code `5` confidence `high`
