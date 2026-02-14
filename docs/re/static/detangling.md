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
