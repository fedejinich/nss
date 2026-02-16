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
  - Queries:
    - `aphex twin flim`
    - fallback: `smells like teen spirit`
- Commands executed:
  - `cargo run -q -p soul-cli -- session search ... --search-mode distributed`
  - `cargo run -q -p soul-cli -- session download-auto ... --search-mode distributed --strict-track flim`
  - multi-candidate diagnostic runs with:
    - `NSS_DEBUG_TRANSFER=1`
    - `NSS_MAX_CANDIDATE_ATTEMPTS=10`
    - `NSS_SKIP_TRANSFER_REQUEST_FLOW=1`
    - `NSS_QUEUE_WAIT_SECS=30`
    - `NSS_TRANSFER_FLOW_TIMEOUT_SECS=60`
    - `NSS_DIRECT_TRANSFER_FLOW_TIMEOUT_SECS=60`
  - username+path queue variant diagnostic:
    - `NSS_QUEUE_UPLOAD_INCLUDE_USERNAME=1`
- Runtime findings:
  - Distributed search consistently resolves many live peers for both `Flim` and popular fallback queries.
  - `PM_TRANSFER_REQUEST`/queue-grant frames are repeatedly observed with non-zero sizes (for example `~12MB`, `~15MB`, `~37MB`, `~84MB`).
  - Even after queue grants, payload transfer still fails (`inbound F timeout`, `peer returned zero bytes for transfer`, `direct flow read frame len / connection reset`).
  - Queue-upload target variants now exercise normalized/suffix paths first and emit explicit peer responses.
  - Most peers reject all queue variants as `File not shared.` (including basename/suffix attempts and username+path attempts).
  - No successful payload transfer yet (`bytes_written > 0` still pending).
- Alternate account/env comparison (`2026-02-16`):
  - Account/env: `.env.local` tuple (`server.slsknet.org:2242`, `nss_auto_*`).
  - Evidence logs:
    - `tmp/logs/altacct-flim-short-20260216040656.log`
    - `tmp/logs/altacct-teen-short-20260216041102.log`
    - `tmp/logs/altacct-flim-outbound-first-20260216043250.log`
    - `tmp/logs/altacct-teen-with-probe-clean-20260216044845.log`
  - Outcome: same hardwall signature reproduced under alternate account/server:
    - queue grants with non-zero sizes,
    - `peer returned zero bytes` and `inbound F timeout`,
    - queue-upload rejection branch (`code=46/50`) with `File not shared`.
- Probe path diagnostic (`2026-02-16`):
  - With `SoulseekQt` running concurrently, inbound wait-port bind failed (`Address already in use`) in probe-assisted runs.
  - After terminating `SoulseekQt`, bind conflict disappeared, but transfer still failed with the same queue-grant/zero-byte/denial pattern.
- Official handshake capture attempt (`2026-02-16`):
  - Raw run dirs:
    - `captures/raw/20260216T072154Z-s9a-hardwall-official-search`
    - `captures/raw/20260216T072406Z-s9a-hardwall-official-search-r2`
  - Blockers:
    - Frida attach/helper runtime failure (`frida-helper` launch/permission errors; no `frida-events.raw.jsonl` emitted).
    - Local tcpdump path remains permission-blocked on `/dev/bpf*`.
  - Tooling hardening applied:
    - `tools/runtime/capture_harness.py` now preserves venv interpreter path (no symlink resolve) to avoid launching with wrong Python environment.
- Baseline official-vs-neo transfer sequence comparison:
  - Source: `captures/redacted/login-search-download/official_frames.hex` and `captures/redacted/login-search-download/neo_frames.hex`
  - Observation: baseline successful run terminates transfer control path at `code=40` then `code=41` with no `code=46/50` denial branch.
  - Current live runs diverge into denial/timeout branches after queue grant.
- Handshake-variant probe (`2026-02-16`):
  - Change: runtime knob `NSS_SEND_CONNECT_TOKEN_ON_PEER_INIT=0` to skip `PM_SEND_CONNECT_TOKEN` on peer-init flows.
  - Evidence:
    - `tmp/logs/altacct-teen-no-connect-token-20260216102133.log`
  - Outcome:
    - transfer still fails with same terminal shape (`queue grant` -> `peer returned zero bytes` and/or `code=46/50 File not shared` + direct-flow EOF).
    - no successful payload transfer observed.
- Outbound file-transfer variant hardening (`2026-02-16`):
  - Code changes:
    - fixed outbound variant loop to reject zero-byte/invalid transfer content and continue probing additional init variants instead of returning early.
    - added queue-upload raw share-prefixed targets (for example `@@share\\...`) and slash variants (`@@share/...`) in target expansion.
    - added outbound init probes:
      - `offset+token` init ordering
      - optional connect-token on outbound file-init (`NSS_SEND_CONNECT_TOKEN_ON_OUTBOUND_FILE_INIT=1`)
      - outbound variant ordering control (`NSS_OUTBOUND_FILE_VARIANT_ORDER`).
  - Regression tests:
    - `cargo test -q -p soul-core outbound_transfer_variants_continue_after_zero_byte_attempt -- --nocapture`
    - `cargo test -q -p soul-core file_transfer_offset_then_token_init_writes_expected_order -- --nocapture`
  - Evidence logs:
    - `tmp/logs/download-teen-e2e-20260216104724.log`
    - `tmp/logs/search-teen-mp3-distributed-20260216112051.log`
    - `tmp/logs/download-teen-mp3-idx12-20260216112224.log`
    - `tmp/logs/download-teen-mp3-idx12-v2-20260216113205.log`
    - `tmp/logs/download-teen-mp3-idx12-finit-20260216114049.log`
    - `tmp/logs/download-teen-mp3-scan-idx0-20260216115000.log`
    - `tmp/logs/download-teen-mp3-scan-idx1-20260216115000.log`
    - `tmp/logs/download-teen-mp3-scan-idx2-20260216115000.log`
    - `tmp/logs/download-teen-mp3-scan-idx3-20260216115000.log`
    - `tmp/logs/download-teen-mp3-scan-idx4-20260216115000.log`
    - `tmp/logs/download-teen-mp3-scan-idx5-20260216115000.log`
  - Outcome:
    - diagnostic coverage improved (full variant traversal visible in logs),
    - queue grants now appear for additional path variants (including raw `@@share` paths),
    - no live payload completion yet (`SCAN_SUCCESS=0`, `bytes_written=0`).
- Frida-free inbound reachability check (`2026-02-16`):
  - Tool: `tools/runtime/check_slsk_porttest.py`
  - Command:
    - `./.venv-tools/bin/python tools/runtime/check_slsk_porttest.py 50036 50037 2242 --json`
  - Result:
    - `50036/tcp CLOSED`
    - `50037/tcp CLOSED`
    - `2242/tcp CLOSED`
  - Interpretation:
    - public inbound connectivity appears blocked in current network environment, consistent with repeated inbound wait-port and inbound F timeout failures.
- Community corroboration scan (Google Groups + Reddit):
  - Reports repeatedly associate "queued/stuck/no transfer" symptoms with:
    - closed/incorrect listen ports or NAT/firewall constraints,
    - VPN/proxy / ISP security filtering,
    - occasional client-version regressions fixed by updates.
  - Sources:
    - `https://groups.google.com/g/soulseek-discussion/c/5FXPNm1pdA0`
    - `https://groups.google.com/g/soulseek-discussion/c/BLIhl7P1cfA`
    - `https://www.reddit.com/r/Soulseek/comments/1ebd1il`
    - `https://www.reddit.com/r/Soulseek/comments/fyosas/new_to_this_my_downloads_getting_stuck_as_queued/`
- Current conclusion:
  - Search-path and candidate discovery are live.
  - Queue/control contracts are now strongly evidenced (grant + denial + timeout branches), but end-to-end transfer remains blocked by runtime hardwall under current public peer/account conditions.
