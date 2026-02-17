# Decompilation Status

## Objective

Map SoulseekQt protocol, architecture, and persistence-critical file formats with traceable static/runtime evidence to drive S9P protocol parity and subsystem replicability classification.

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
- `analysis/re/official_architecture_map.json`
- `analysis/re/official_file_format_map.json`
- `analysis/re/protocol_parity_matrix.json`

## Runtime Evidence Paths

- Capture harness: `tools/runtime/capture_harness.py`
- Redaction tool: `tools/runtime/redact_capture_run.py`
- Runtime I/O hooks: `frida/hooks/soulseek_io_trace.js`
- Stage 3B capture generator: `tools/runtime/generate_stage3b_room_captures.py`
- Stage 4A capture generator: `tools/runtime/generate_stage4a_discovery_captures.py`
- Stage 4B capture generator: `tools/runtime/generate_stage4b_peer_room_captures.py`
- Stage 4C capture generator: `tools/runtime/generate_stage4c_privileges_social_captures.py`
- Stage 4D capture generator: `tools/runtime/generate_stage4d_privilege_legacy_captures.py`
- Stage 4E capture generator: `tools/runtime/generate_stage4e_private_userstate_captures.py`
- Redacted run storage: `captures/redacted/*`
- Stage 5B UI static extraction artifacts: `evidence/ui_audit/*` + `docs/state/soulseek-feature-inventory.md`
- Redacted runtime I/O events (v3 track): `captures/redacted/*/io-events.redacted.jsonl`

## S9P Static Baseline Snapshot

- Date: `2026-02-16`
- Commands:
  - `scripts/ghidra_pipeline.sh`
  - `scripts/extract_search_download_flow.sh`
- Output artifacts:
  - `analysis/binaries/SoulseekQt`
  - `analysis/ghidra/project/soulseekqt`
  - `evidence/reverse/search_download_symbols_nm.txt`
  - `evidence/reverse/search_download_strings.txt`
  - `analysis/re/flow_graph.json`
  - `docs/re/static/search-download-flow.md`
- Result: static baseline completed for `S9P-T04S`.

## S9P Runtime Baseline Snapshot

- Date: `2026-02-17`
- Commands:
  - `scripts/capture_golden.sh` with `OFFICIAL_RUNNER=1`
  - `tools/runtime/official_runner.py`
- Output artifacts:
  - `captures/raw/20260216T235428Z-s9p-v3-t05-runner-both-debug-r2`
  - `captures/raw/20260216T235612Z-s9p-v3-t05-runner-both-debug-r3`
  - `captures/raw/20260217T000258Z-s9p-v3-t04f-startup-io-r4`
  - `captures/raw/20260217T010817Z-i3-t04-io-runtime-r5`
  - `captures/redacted/20260216T235428Z-s9p-v3-t05-runner-both-debug-r2`
  - `captures/redacted/20260216T235612Z-s9p-v3-t05-runner-both-debug-r3`
  - `captures/redacted/20260217T000258Z-s9p-v3-t04f-startup-io-r4`
  - `captures/redacted/20260217T010817Z-i3-t04-io-runtime-r5`
- Result:
  - tooling/runtime baseline is reproducible under debug-specimen instrumentation,
  - arm64 hook offsets were reconciled from static `nm` evidence,
  - Frida attach selection is now path-disambiguated (`--process-path-contains`) to avoid same-name process ambiguity,
  - runtime traces include high-signal transfer-store persistence events (`writestring`, `mainwindow_save_data_enter`, `datasaver_save_enter`, `datasaver_save_to_file_enter`),
  - QSettings/QDataStream hooks remain a targeted runtime gap.

## Next Reverse Focus

1. Runtime deepening:
   - expand official scenario corpus (`login/search/download`, transfer edge paths) using `tools/runtime/official_runner.py`,
   - continue refining QSettings/QDataStream symbol hooks and trigger flows to close remaining settings/export-import runtime gaps.
2. Execute synthesis block:
   - reconcile static intent vs runtime ordering/content and file-I/O behavior,
   - publish protocol/transfer parity deltas and architecture/format replicability matrix.
3. Execute patch blocks:
   - implement minimal transfer parity fixes with regression tests,
   - iterate until `Flim` E2E reaches `bytes_written > 0` or classify hardwall with evidence.
