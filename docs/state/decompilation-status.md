# Decompilation Status

## Objective

Map the Soulseek protocol incrementally and with traceable evidence to enable an evolvable custom client.

## Stage 2 Coverage

- Core target: `25` messages (server + peer).
- Current Stage 2 core coverage: `25/25` in `analysis/ghidra/maps/message_map.csv`.
- Confidence contract: `high >= 18`, `medium <= 7`, `low = 0`.
- Stage 2 confidence: `high=25`, `medium=0`, `low=0`.

## Stage 3B Addendum

- Total mapped rows now: `33` (`25` core + `8` rooms/presence messages).
- S3B batch confidence: `high=8`, `medium=0`, `low=0`.

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
- Scenario wrappers: `scripts/capture_session.sh`, `scripts/capture_golden.sh`
- S3B room capture generator: `tools/runtime/generate_stage3b_room_captures.py`
- Redacted run storage: `captures/redacted/*`

## Next Reverse Focus

- Execute S4 batches (recommendations/discovery + peer advanced + room moderation) with runtime-first evidence promotion.
