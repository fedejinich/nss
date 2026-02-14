# Decompilation Status

## Objective

Mapear protocolo Soulseek de forma incremental y trazable para habilitar una app propia evolutiva.

## Stage 2 Coverage

- Target core: `25` mensajes (server + peer).
- Coverage actual: `25/25` en `analysis/ghidra/maps/message_map.csv`.
- Confidence contract: `high >= 18`, `medium <= 7`, `low = 0`.
- Confidence actual: `high=25`, `medium=0`, `low=0`.

## Static Evidence Sources

- `evidence/reverse/server_messagecodetostring_otool.txt`
- `evidence/reverse/peer_messagecodetostring_otool.txt`
- `evidence/reverse/message_name_strings.txt`
- `evidence/reverse/disasm/server_file_search.txt`
- `evidence/reverse/disasm/server_prepare_search.txt`
- `evidence/reverse/disasm/server_handle_message.txt`
- `evidence/reverse/disasm/transfer_on_file_request.txt`
- `evidence/reverse/disasm/transfer_on_queue_download.txt`
- `evidence/reverse/disasm/upload_write_socket.txt`

## Runtime Evidence Path

- Capture harness: `tools/runtime/capture_harness.py`
- Redaction tool: `tools/runtime/redact_capture_run.py`
- Scenario wrappers: `scripts/capture_session.sh`, `scripts/capture_golden.sh`
- Mandatory scenario runs stored in: `captures/redacted/*`

## Next Reverse Focus

- Resolver versión de login aceptada por servidor para capturar sesiones autenticadas completas y ampliar cobertura beyond-core.
