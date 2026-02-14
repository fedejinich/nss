# Decompilation Status

## Objective

Map the Soulseek protocol incrementally with traceable evidence to enable a custom evolvable client.

## Coverage Summary

- Stage 2 core contract: `25/25` core messages (`high=25`, `medium=0`, `low=0`).
- Stage 3B rooms/presence addendum: `+8` messages.
- Stage 4A discovery addendum: `+5` messages.
- Stage 4B peer advanced + room moderation addendum: `+9` messages.
- Stage 4C privileges/social + peer-folder addendum: `+9` messages.
- Total mapped protocol rows: `56`.

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

## Runtime Evidence Paths

- Capture harness: `tools/runtime/capture_harness.py`
- Redaction tool: `tools/runtime/redact_capture_run.py`
- Stage 3B capture generator: `tools/runtime/generate_stage3b_room_captures.py`
- Stage 4A capture generator: `tools/runtime/generate_stage4a_discovery_captures.py`
- Stage 4B capture generator: `tools/runtime/generate_stage4b_peer_room_captures.py`
- Stage 4C capture generator: `tools/runtime/generate_stage4c_privileges_social_captures.py`
- Redacted run storage: `captures/redacted/*`

## Next Reverse Focus

- Resolve authoritative mapping for `SM_BAN_USER`.
- Promote remaining medium-confidence legacy peer search messages with runtime evidence.
- Add decompression-aware parser coverage for `PM_SHARED_FILES_IN_FOLDER` compressed payload semantics.
