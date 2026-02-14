# Decompilation Status

## Objective

Map the Soulseek protocol incrementally with traceable evidence to enable a custom evolvable client.

## Coverage Summary

- Stage 2 core contract: `25/25` core messages (`high=25`, `medium=0`, `low=0`).
- Stage 3B rooms/presence addendum: `+8` messages.
- Stage 4A discovery addendum: `+5` messages.
- Stage 4B peer advanced + room moderation addendum: `+9` messages.
- Stage 4C privileges/social + peer-folder addendum: `+9` messages.
- Stage 4D privilege/messaging gaps + peer legacy addendum: `+9` new messages and `+2` confidence promotions.
- Total mapped protocol rows: `65`.

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
- `evidence/reverse/message_codes_jump_table.md`

## Runtime Evidence Paths

- Capture harness: `tools/runtime/capture_harness.py`
- Redaction tool: `tools/runtime/redact_capture_run.py`
- Stage 3B capture generator: `tools/runtime/generate_stage3b_room_captures.py`
- Stage 4A capture generator: `tools/runtime/generate_stage4a_discovery_captures.py`
- Stage 4B capture generator: `tools/runtime/generate_stage4b_peer_room_captures.py`
- Stage 4C capture generator: `tools/runtime/generate_stage4c_privileges_social_captures.py`
- Stage 4D capture generator: `tools/runtime/generate_stage4d_privilege_legacy_captures.py`
- Redacted run storage: `captures/redacted/*`

## Next Reverse Focus

- Map and validate private messaging runtime flows (`SM_MESSAGE_USER`, `SM_MESSAGE_ACKED`) with typed field coverage.
- Expand runtime evidence for user-state/status domains (`SM_GET_USER_STATUS`, `SM_GET_USER_STATS`, peer address/connect interactions).
- Add decompression-aware parser coverage for `PM_SHARED_FILES_IN_FOLDER` compressed payload semantics.
