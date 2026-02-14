# Search/Download Flow

- Binary: `SoulseekQt`
- Architecture: `arm64`
- Symbols source: `/Users/void_rsk/Projects/soul-dec/evidence/reverse/search_download_symbols_nm.txt`
- Strings source: `evidence/reverse/search_download_strings.txt`

## Nodes

### `server_prepare_search`
- Symbol: `Server::PrepareSearch(QString)`
- Address: `0x00000001000615c0`
- Stage: `server_tx`
- Description: Normalizes search text before serialization.
- Disassembly: `evidence/reverse/disasm/server_prepare_search.txt`

### `server_file_search`
- Symbol: `Server::FileSearch(QString, QString)`
- Address: `0x0000000100060fa0`
- Stage: `server_tx`
- Description: Builds and sends search message to the server.
- Disassembly: `evidence/reverse/disasm/server_file_search.txt`

### `server_send_message`
- Symbol: `Server::SendMessage(MemStream&, bool)`
- Address: `0x0000000100054dac`
- Stage: `server_tx`
- Description: Serializes MemStream into the server socket.
- Disassembly: `evidence/reverse/disasm/server_send_message.txt`

### `server_handle_message`
- Symbol: `Server::HandleMessage(int, MemStream&)`
- Address: `0x000000010005521c`
- Stage: `server_rx`
- Description: Dispatches incoming server messages.
- Disassembly: `evidence/reverse/disasm/server_handle_message.txt`

### `peer_queue_download`
- Symbol: `PeerMessenger::QueueDownload(QString, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>)`
- Address: `0x00000001000a4474`
- Stage: `peer_tx`
- Description: Builds the initial download request sent to a peer.
- Disassembly: `evidence/reverse/disasm/peer_queue_download.txt`

### `peer_send_message`
- Symbol: `PeerMessenger::SendMessage(QTcpSocket*, MemStream&, bool)`
- Address: `0x00000001000a0328`
- Stage: `peer_tx`
- Description: Serializes messages on the peer socket.
- Disassembly: `evidence/reverse/disasm/peer_send_message.txt`

### `peer_handle_message`
- Symbol: `PeerMessenger::HandleMessage(QTcpSocket*, MemStream)`
- Address: `0x00000001000964ec`
- Stage: `peer_rx`
- Description: Dispatches incoming peer messages to transfer handlers.
- Disassembly: `evidence/reverse/disasm/peer_handle_message.txt`

### `transfer_on_queue_download`
- Symbol: `TransferQueueManager::OnQueueDownloadRequested(QString, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, long long)`
- Address: `0x00000001000f0478`
- Stage: `transfer_ctrl`
- Description: Creates local queue entry and starts file connection flow.
- Disassembly: `evidence/reverse/disasm/transfer_on_queue_download.txt`

### `transfer_on_file_request`
- Symbol: `TransferQueueManager::OnFileTransferRequest(QString, int, unsigned int, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, long long)`
- Address: `0x00000001000d7114`
- Stage: `transfer_ctrl`
- Description: Negotiates transfer request/response with peer.
- Disassembly: `evidence/reverse/disasm/transfer_on_file_request.txt`

### `download_read_socket`
- Symbol: `DownloadTask::readSocket()`
- Address: `0x00000001000eb9fc`
- Stage: `download_rx`
- Description: Consumes socket bytes and writes destination file.
- Disassembly: `evidence/reverse/disasm/download_read_socket.txt`

### `upload_write_socket`
- Symbol: `UploadTask::writeToSocket()`
- Address: `0x00000001000ea3cc`
- Stage: `upload_tx`
- Description: Streams local bytes to the remote peer.
- Disassembly: `evidence/reverse/disasm/upload_write_socket.txt`

## Edges

- `server_file_search` -> `server_prepare_search`: file_search prepares terms before frame build
- `server_file_search` -> `server_send_message`: file_search ends in send_message
- `server_send_message` -> `server_handle_message`: request/response over server socket
- `peer_queue_download` -> `transfer_on_queue_download`: queue_download creates transfer queue entry
- `transfer_on_queue_download` -> `peer_send_message`: download queue emits request to peer
- `peer_send_message` -> `peer_handle_message`: peer response returns through handle_message
- `peer_handle_message` -> `transfer_on_file_request`: transfer_request dispatch
- `transfer_on_file_request` -> `download_read_socket`: accepted request starts download
- `transfer_on_file_request` -> `upload_write_socket`: accepted request may start upload

