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
- Stage 4E private messaging + user-state addendum: `+2` new messages with runtime validation upgrades for six existing mappings.
- Stage 4F global/admin/distributed mapping addendum: `+8` mapped messages from jump-table evidence.
- Stage 4G parent/distributed tuning mapping addendum: `+8` mapped messages from jump-table evidence.
- Stage 4H global room/system control mapping addendum: `+8` mapped messages from jump-table evidence.
- Stage 4I ticker/term-control mapping addendum: `+8` mapped messages from jump-table evidence.
- Stage 4J private-room ownership/membership mapping addendum: `+8` mapped messages from jump-table evidence.
- Stage 4K missing-code closure addendum: `+24` mapped+implemented messages from jump-table evidence and protocol codec extension.
- Stage 4L mapped-not-implemented closure addendum: `+40` protocol implementations to reach full matrix coverage.
- Stage 5B UI/feature audit addendum: symbolized `MainWindow/Server/Peer/Transfer` method inventory plus UI handler anchors for feature-to-protocol mapping.
- Total mapped protocol rows: `131`.
- Total implemented+mapped rows: `131`.

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
- `evidence/reverse/ui_handler_symbols_nm.txt`
- `evidence/ui_audit/decomp/mainwindow_methods.txt`
- `evidence/ui_audit/decomp/server_methods.txt`
- `evidence/ui_audit/decomp/peer_methods.txt`
- `evidence/ui_audit/decomp/transfer_methods.txt`

## Runtime Evidence Paths

- Capture harness: `tools/runtime/capture_harness.py`
- Redaction tool: `tools/runtime/redact_capture_run.py`
- Stage 3B capture generator: `tools/runtime/generate_stage3b_room_captures.py`
- Stage 4A capture generator: `tools/runtime/generate_stage4a_discovery_captures.py`
- Stage 4B capture generator: `tools/runtime/generate_stage4b_peer_room_captures.py`
- Stage 4C capture generator: `tools/runtime/generate_stage4c_privileges_social_captures.py`
- Stage 4D capture generator: `tools/runtime/generate_stage4d_privilege_legacy_captures.py`
- Stage 4E capture generator: `tools/runtime/generate_stage4e_private_userstate_captures.py`
- Redacted run storage: `captures/redacted/*`
- Stage 5B UI static extraction artifacts: `evidence/ui_audit/*` + `docs/state/soulseek-feature-inventory.md`

## Next Reverse Focus

- Replace `OpaqueServerControlPayload` and `OpaquePayload` control branches with typed payload schemas where runtime evidence is available.
- Expand runtime capture coverage for distributed/global control families that are currently static-only.
- Add decompression-aware parser coverage for `PM_SHARED_FILES_IN_FOLDER` compressed payload semantics.
- Close the remaining Stage 5B UI introspection gap by rerunning menu-tree extraction with macOS assistive access enabled.
