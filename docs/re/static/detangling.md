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

### `MainWindow::onAddSharedFolder()` -> `ui_add_shared_folder`
- Binary: `SoulseekQt`
- Address: `0x1000300cc`
- Confidence: `high`
- Status: `approved`
- Evidence:
  - `xref`: `evidence/reverse/ui_handler_symbols_nm.txt` (Symbolized UI action handler address.)
  - `string`: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt` (Includes `Add Shared Folder` UI surface labels.)

### `MainWindow::onSetFolderPermissions()` -> `ui_set_folder_permissions`
- Binary: `SoulseekQt`
- Address: `0x100030128`
- Confidence: `high`
- Status: `approved`
- Evidence:
  - `xref`: `evidence/reverse/ui_handler_symbols_nm.txt` (Symbolized UI action handler address.)
  - `string`: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt` (Includes `Set Folder Permissions` UI labels.)

### `MainWindow::onEnableListeningPortsClicked(bool)` -> `ui_toggle_listening_ports`
- Binary: `SoulseekQt`
- Address: `0x10003f6a0`
- Confidence: `high`
- Status: `approved`
- Evidence:
  - `xref`: `evidence/reverse/ui_handler_symbols_nm.txt` (UI control handler with stable symbol and address.)
  - `xref`: `evidence/ui_audit/decomp/mainwindow_methods.txt` (Method inventory confirms handler presence.)

### `MainWindow::onAutoDisableListeningPortsClicked(bool)` -> `ui_toggle_auto_disable_listening_ports`
- Binary: `SoulseekQt`
- Address: `0x10003fb90`
- Confidence: `high`
- Status: `approved`
- Evidence:
  - `xref`: `evidence/reverse/ui_handler_symbols_nm.txt` (UI control handler with stable symbol and address.)
  - `string`: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt` (Auto-disable controls and labels are present.)

### `MainWindow::minimizeToTrayOnCloseClicked(bool)` -> `ui_toggle_minimize_to_tray_on_close`
- Binary: `SoulseekQt`
- Address: `0x10003f1d0`
- Confidence: `high`
- Status: `approved`
- Evidence:
  - `xref`: `evidence/reverse/ui_handler_symbols_nm.txt` (UI handler used by tray/minimize preference.)
  - `string`: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt` (Tray and minimize labels are present.)

### `MainWindow::onUseOldStyleSearchResultsCheckboxToggled(bool)` -> `ui_toggle_old_style_search_results`
- Binary: `SoulseekQt`
- Address: `0x100036eac`
- Confidence: `high`
- Status: `approved`
- Evidence:
  - `xref`: `evidence/reverse/ui_handler_symbols_nm.txt` (UI handler address from demangled symbol extraction.)
  - `string`: `evidence/ui_audit/ui_strings_feature_candidates_normalized.txt` (Old-style search results option labels are present.)

### `Server::EnterRoom(QString, bool)` -> `server_enter_room`
- Binary: `SoulseekQt`
- Address: `0x10006d75c`
- Confidence: `high`
- Status: `approved`
- Evidence:
  - `xref`: `evidence/reverse/ui_handler_symbols_nm.txt` (Server method address and symbol extracted from app binary.)
  - `xref`: `evidence/ui_audit/decomp/server_methods.txt` (Method inventory confirms room-entry server path.)

### `Server::SendPrivateChat(QString, QString)` -> `server_send_private_chat`
- Binary: `SoulseekQt`
- Address: `0x10006f128`
- Confidence: `high`
- Status: `approved`
- Evidence:
  - `xref`: `evidence/reverse/ui_handler_symbols_nm.txt` (Server method address and symbol extracted from app binary.)
  - `xref`: `evidence/ui_audit/decomp/server_methods.txt` (Method inventory confirms private-chat transmit path.)

## Approved Data Labels

No approved data labels yet.

## Review Queue

Review queue is empty.

## Protocol Coverage (Stage 2)

- Message rows: `131`
- Confidence split: `high=130` `medium=1` `low=0`
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
  - `server` `SM_GET_SIMILAR_TERMS` code `50` confidence `high`
  - `server` `SM_GET_RECOMMENDATIONS` code `54` confidence `high`
  - `server` `SM_GET_MY_RECOMMENDATIONS` code `55` confidence `high`
  - `server` `SM_GET_GLOBAL_RECOMMENDATIONS` code `56` confidence `high`
  - `server` `SM_GET_USER_RECOMMENDATIONS` code `57` confidence `high`
  - `server` `SM_PRIVILEGED_LIST` code `69` confidence `high`
  - `server` `SM_GET_RECOMMENDED_USERS` code `110` confidence `high`
  - `server` `SM_GET_TERM_RECOMMENDATIONS` code `111` confidence `high`
  - `server` `SM_GET_RECOMMENDATION_USERS` code `112` confidence `high`
  - `server` `SM_ROOM_MEMBERS` code `133` confidence `high`
  - `server` `SM_ROOM_OPERATORS` code `148` confidence `high`
