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
- Description: Normaliza texto de busqueda antes de serializar.
- Disassembly: `evidence/reverse/disasm/server_prepare_search.txt`

### `server_file_search`
- Symbol: `Server::FileSearch(QString, QString)`
- Address: `0x0000000100060fa0`
- Stage: `server_tx`
- Description: Construye y envia mensaje de busqueda al servidor.
- Disassembly: `evidence/reverse/disasm/server_file_search.txt`

### `server_send_message`
- Symbol: `Server::SendMessage(MemStream&, bool)`
- Address: `0x0000000100054dac`
- Stage: `server_tx`
- Description: Serializa MemStream en socket de servidor.
- Disassembly: `evidence/reverse/disasm/server_send_message.txt`

### `server_handle_message`
- Symbol: `Server::HandleMessage(int, MemStream&)`
- Address: `0x000000010005521c`
- Stage: `server_rx`
- Description: Despacha mensajes entrantes del servidor.
- Disassembly: `evidence/reverse/disasm/server_handle_message.txt`

### `peer_queue_download`
- Symbol: `PeerMessenger::QueueDownload(QString, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>)`
- Address: `0x00000001000a4474`
- Stage: `peer_tx`
- Description: Encapsula solicitud inicial de descarga a peer.
- Disassembly: `evidence/reverse/disasm/peer_queue_download.txt`

### `peer_send_message`
- Symbol: `PeerMessenger::SendMessage(QTcpSocket*, MemStream&, bool)`
- Address: `0x00000001000a0328`
- Stage: `peer_tx`
- Description: Serializa mensajes sobre socket peer.
- Disassembly: `evidence/reverse/disasm/peer_send_message.txt`

### `peer_handle_message`
- Symbol: `PeerMessenger::HandleMessage(QTcpSocket*, MemStream)`
- Address: `0x00000001000964ec`
- Stage: `peer_rx`
- Description: Despacha mensajes entrantes peer a manejadores de transferencia.
- Disassembly: `evidence/reverse/disasm/peer_handle_message.txt`

### `transfer_on_queue_download`
- Symbol: `TransferQueueManager::OnQueueDownloadRequested(QString, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, long long)`
- Address: `0x00000001000f0478`
- Stage: `transfer_ctrl`
- Description: Materializa entrada de cola local y dispara conexion de archivo.
- Disassembly: `evidence/reverse/disasm/transfer_on_queue_download.txt`

### `transfer_on_file_request`
- Symbol: `TransferQueueManager::OnFileTransferRequest(QString, int, unsigned int, std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char>>, long long)`
- Address: `0x00000001000d7114`
- Stage: `transfer_ctrl`
- Description: Negocia request/respuesta de transferencia con peer.
- Disassembly: `evidence/reverse/disasm/transfer_on_file_request.txt`

### `download_read_socket`
- Symbol: `DownloadTask::readSocket()`
- Address: `0x00000001000eb9fc`
- Stage: `download_rx`
- Description: Consume bytes del socket para escribir archivo destino.
- Disassembly: `evidence/reverse/disasm/download_read_socket.txt`

### `upload_write_socket`
- Symbol: `UploadTask::writeToSocket()`
- Address: `0x00000001000ea3cc`
- Stage: `upload_tx`
- Description: Publica bytes locales al peer remoto.
- Disassembly: `evidence/reverse/disasm/upload_write_socket.txt`

## Edges

- `server_file_search` -> `server_prepare_search`: file_search prepara terminos antes de armar frame
- `server_file_search` -> `server_send_message`: file_search termina en send_message
- `server_send_message` -> `server_handle_message`: request/response via socket servidor
- `peer_queue_download` -> `transfer_on_queue_download`: queue_download dispara alta en cola
- `transfer_on_queue_download` -> `peer_send_message`: cola de descarga emite request al peer
- `peer_send_message` -> `peer_handle_message`: response peer vuelve por handle_message
- `peer_handle_message` -> `transfer_on_file_request`: dispatch de transfer_request
- `transfer_on_file_request` -> `download_read_socket`: request aceptado inicia descarga
- `transfer_on_file_request` -> `upload_write_socket`: request aceptado puede iniciar upload

