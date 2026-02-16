# Evidence Ledger

Project-level evidence summaries and provenance tracking.

## Totals

- Approved function renames: `5`
- Approved data labels: `0`
- Review queue entries: `0`
- Protocol message rows: `131`

## Protocol Confidence

- `high`: `131`
- `medium`: `0`
- `low`: `0`

## Latest Evidence Sources

- `evidence/reverse/search_download_symbols_nm.txt`
- `evidence/reverse/disasm/server_send_message.txt`
- `evidence/reverse/disasm/server_file_search.txt`
- `evidence/reverse/server_messagecodetostring_otool.txt`
- `evidence/reverse/message_name_strings.txt`
- `evidence/reverse/disasm/peer_queue_download.txt`
- `evidence/reverse/disasm/transfer_on_file_request.txt`
- `captures/redacted/login-private-room-membership-control/official_frames.hex`
- `captures/redacted/login-static-server-runtime/official_frames.hex`
- `captures/redacted/login-legacy-distributed-control/official_frames.hex`
- `captures/redacted/login-legacy-residual-control/official_frames.hex`
- `captures/redacted/login-legacy-room-operatorship-control/official_frames.hex`
- `captures/redacted/login-global-room-control/official_frames.hex`
- `captures/redacted/login-peer-message/official_frames.hex`
- `captures/redacted/peer-static-runtime/official_frames.hex`

## S9A-NEXT Runtime Transfer Diagnostics (In Progress)

- Date: `2026-02-16`
- Environment:
  - Server: `server.slsknet.org:2416`
  - Account: local-only test credential (`fede_test1234`)
  - Query: `aphex twin flim`
- Commands executed:
  - `cargo run -q -p soul-cli -- session search ... --search-mode distributed`
  - `cargo run -q -p soul-cli -- session download-auto ... --search-mode distributed --strict-track flim`
  - `NSS_DEBUG_TRANSFER=1` and `NSS_QUEUE_WAIT_SECS=180` diagnostic runs
- Runtime findings:
  - Distributed search consistently resolves `Flim` candidates (multiple peers and paths).
  - `PM_TRANSFER_REQUEST` request path works and peers typically respond `Queued`.
  - Queue wait can emit follow-up peer frames (`code=40` grant observed in one run; `code=50`/`code=46` denial variants also observed).
  - Denial reason is consistently `File not shared.` across queue and transfer follow-ups for current candidate set.
  - No successful payload transfer yet (`bytes_written > 0` still pending).
- Current conclusion:
  - Search-path and candidate discovery are live.
  - Inbound/queue transfer contract is partially validated at runtime but still not closed for successful byte transfer under current public peer set.
