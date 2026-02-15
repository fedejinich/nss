# Project Status

## Date

- 2026-02-15

## Current Phase

- Stage 2 complete: Core P2P MVP with KB-first contract over 25 core protocol messages.
- Stage 2R complete: runtime capture refresh and confidence promotion (`medium -> high`) for all 25 core messages.
- Stage 3A complete: authenticated login against official server (`160/1`) and semantic differential verification as default.
- Stage 3B complete: Rooms+Presence protocol batch with runtime evidence and CLI support.
- Stage 4A complete: Recommendations/Discovery batch with runtime-authenticated evidence and CLI support.
- Stage 4B complete: Peer advanced + room moderation batch with protocol matrix publication and runtime evidence.
- Stage 4C complete: Privileges/social control + peer-folder batch with runtime evidence and CLI support.
- Stage 4D complete: privilege/messaging gaps + peer legacy cleanup with runtime captures, protocol promotion, and CLI support.
- Stage 4E complete: private messaging + user-state batch with runtime evidence, typed SDK/CLI operations, and semantic verification updates.
- Stage 4F complete: global/admin/distributed-control mapping batch with authoritative jump-table evidence (mapping-first expansion).
- Stage 4G complete: parent/distributed tuning mapping continuation with authoritative jump-table evidence (mapping-first expansion).
- Stage 4H complete: global room/system control mapping continuation with authoritative jump-table evidence (mapping-first expansion).
- Stage 4I complete: ticker/term control mapping continuation with authoritative jump-table evidence (mapping-first expansion).
- Stage 4J complete: private-room ownership/membership mapping continuation with authoritative jump-table evidence (mapping-first expansion).
- Stage 4K complete: missing-code closure and protocol implementation for global/distributed tail + peer control with authoritative jump-table evidence.
- Stage 4L complete: mapped-not-implemented closure to full matrix implementation coverage (`implemented+mapped=131`).
- Stage 5A complete: typed runtime hardening wave 1 for parent/distributed control and folder decompression parsing with authenticated evidence.
- Stage 5B complete: exhaustive SoulseekQt UI + functionality research pass with pass-2 verification and decompilation addendum artifacts.
- Stage 5C complete: typed runtime hardening wave 2 for room/term control messages with authenticated runtime evidence and SDK/CLI coverage.
- Stage 5D complete: typed runtime hardening wave 3 for global control (`150/151/152/153`) with authenticated runtime evidence.
- Stage 5E complete: typed runtime hardening wave 4 for parent/disconnect control (`86/87/88/90/100`) with authenticated runtime evidence.
- Stage 5F complete: typed runtime hardening wave 5 for private-room membership/ownership control (`136/137/139/140/145`) with authenticated runtime evidence.
- Stage 5G complete: typed runtime hardening wave 6 for text-control payloads (`58/62/63/66/117/118`) with authenticated runtime evidence.
- Stage 5H complete: typed runtime hardening wave 7 for system-control payloads (`28/32/130`) with authenticated runtime evidence.
- Stage 6A complete: dashboard-first KB UX with visual project dashboard, interactive codebase visualizer, and collapsed PR catalog navigation.
- Stage 6B complete: executable closure audit for S5A hardening objectives with regression enforcement and state-sync integration.
- Stage 6C complete: executable opaque-tail baseline report and batch plan to drive S6 typed-promotion work.
- Stage 6D complete: executed S6 typed batches (S6-Batch-1/2/3) and closed generic opaque-tail control coverage to zero.
- Stage 6E complete: reduced dedicated legacy opaque variants with runtime-backed typing for room-operatorship and distributed legacy control families.
- Stage 6F complete: closed final dedicated residual semantics (`SM_DNET_DELIVERY_REPORT`, `SM_FLOOD`) with runtime-backed typed payload promotion.
- Stage 7R complete: roadmap/state rebaseline for strict runtime-complete closure before TUI expansion.
- Stage 7A complete: strict runtime closure achieved (`verified_runtime=131`, `verified_static=0`) with hybrid runtime evidence.
- Stage 7B complete: semantic-tail closure achieved (`raw_tail/raw_payload` unresolved fields reduced to `0`).
- Stage 7C complete: core orchestration flow added with `search_select_and_download(...)` and CLI `session download-auto`.
- Stage 8A complete: capability registry, capability matrix, and critical-path dashboard were added and wired into state sync.
- Stage 8B complete: minimal TUI v1 is operational for login, query edit, search, selection, and download orchestration.
- Stage 8C complete: capability-first release hardening closure is complete (`redaction`, `packaging`, `recovery runbooks`, `closure checklist/audit`).
- Stage 9A complete: TUI-first simplification and persistence hardening landed (`login modal gate`, `retro-orange UX`, `downloads history toggle/clear`, `startup recovery`).
- Stage 9B planned: SwiftUI macOS GUI MVP on top of `soul-cli` JSON mode.
- Stage 9C planned: Next.js web GUI MVP on top of `soul-cli` JSON mode.
- Product direction remains unchanged: SDK+CLI first, custom evolvable app (not a 1:1 official client clone).

## Active Execution Plan (Capability-First)

Current long-session objective:

1. Start S9B SwiftUI macOS GUI MVP on top of the closed S9A TUI baseline.
2. Keep S9C Next.js web MVP queued with the same semantics and shared CLI JSON contract.

Current capability chain:

1. `CAP-TUI-S9A-SIMPLIFIED`
2. `CAP-CLI-JSON-MVP`
3. `CAP-SWIFT-GUI-MVP`
4. `CAP-NEXT-GUI-MVP`

Execution rule:

1. Dashboard/KB plan updates are published first.
2. Then code/runtime/doc implementation proceeds.
3. Final capability promotion to done happens only after closure gates pass.

## Stage 7A Completion

1. Added runtime-coverage registry and generator:
   - `analysis/state/runtime_coverage_registry.json`
   - `tools/state/generate_runtime_coverage.py`
2. Published runtime coverage artifacts:
   - `docs/state/runtime-coverage.json`
   - `docs/state/runtime-coverage.md`
3. Generated runtime-evidence runs for static-only closure:
   - `captures/redacted/login-static-server-runtime`
   - `captures/redacted/peer-static-runtime`
4. Promoted remaining static rows in `analysis/ghidra/maps/message_map.csv` to runtime-verified.
5. Added stage gate regression contract:
   - `tests/protocol/test_stage7_runtime_semantic_contract.py`

## Stage 7B Completion

1. Captured targeted residual-tail runtime evidence:
   - `captures/redacted/login-partial-tail-runtime`
2. Replaced unresolved schema placeholders (`raw_tail`) with explicit `extension_reserved_bytes` semantics.
3. Updated protocol derivation logic:
   - `tools/protocol/derive_schema.py`
4. Regenerated canonical schema/docs:
   - `analysis/protocol/message_schema.json`
   - `docs/re/static/message-schema.md`

## Stage 7C Completion

1. Added core orchestration API:
   - `SessionClient::search_select_and_download(...)`
   - `SearchSelectDownloadRequest`
   - `SearchSelectDownloadResult`
   - `SearchSelectDownloadError`
2. Added CLI operation:
   - `soul-cli session download-auto ...`
3. Added runtime scenario and differential verify wiring:
   - `captures/redacted/login-search-download-auto`
   - `scripts/run_diff_verify.sh`
4. Updated operator documentation:
   - `docs/runbooks/cli-download-example.md`

## Stage 8A Completion

1. Added capability registry and generators:
   - `analysis/state/capability_registry.json`
   - `tools/state/generate_capability_matrix.py`
2. Published capability artifacts:
   - `docs/state/capability-matrix.json`
   - `docs/state/capability-matrix.md`
   - `docs/state/capability-dashboard.html`
3. Extended dashboard data pipeline:
   - `tools/state/generate_dashboard_data.py`
   - `scripts/sync_state_dashboards.sh`
4. Updated navigation/start pages:
   - `zensical.toml`
   - `docs/index.md`

## Stage 8B Completion

1. Added `rust/tui` crate with `ratatui` + `crossterm` shell.
2. Implemented core transfer workflow controls in TUI:
   - login
   - query editing at runtime (`/`, `Enter`, `Esc`)
   - search
   - result selection
   - download selected item via core orchestration
   - transfer monitoring
   - upload accept/deny mode toggles
3. Added TUI runbook and unit tests:
   - `docs/runbooks/tui-core-transfer.md`
   - `rust/tui/src/main.rs` test module

## Stage 9A Completion Snapshot

1. Refactored TUI into focused modules:
   - `rust/tui/src/app.rs`
   - `rust/tui/src/ui.rs`
   - `rust/tui/src/state.rs`
   - `rust/tui/src/storage.rs`
2. Added mandatory login-first state machine (`LoginModal` -> `Main`) with blocked search/download before successful auth.
3. Added retro-orange presentation and simplified layout:
   - search results top
   - search input/footer bottom
   - downloads panel with show/hide toggle
4. Added persisted local state:
   - server/username/password
   - last query/output directory
   - downloads history + UI preferences
5. Added startup recovery behavior:
   - persisted `in_progress` entries are converted to `interrupted` on boot.
6. Added/extended tests for persistence, login gating, toggle/clear behavior, and recovery semantics.

## Stage 4L Completion

1. Closed all `40` remaining `mapped_not_implemented` rows by adding protocol constants and decode/encode support.
2. Introduced `OpaqueServerControlPayload { code, bytes }` and `is_opaque_server_control_code(...)` for maintainable handling of unresolved control-message payload shapes.
3. Added regression tests for closure-set decode/encode and opaque control builder validation.
4. Regenerated matrix/schema/docs to reach the full-coverage baseline:
   - `implemented+mapped=131`
   - `mapped_not_implemented=0`
   - `missing=0`

## Stage 5B Completion

1. Published complete feature inventory document with explicit UI fields (`ui_id`, location, trigger, preconditions, expected behavior, alternate/error states, functional linkages):
   - `docs/state/soulseek-feature-inventory.md`
2. Added two-pass coverage closure:
   - pass-1 mapped entries: `42`
   - pass-2 revisited entries: `42`
   - gap log: `1` (`assistive access denied for live menu extraction`)
3. Added static decompilation evidence artifacts to support feature-to-protocol tracing:
   - `evidence/reverse/ui_handler_symbols_nm.txt`
   - `evidence/ui_audit/decomp/mainwindow_methods.txt`
   - `evidence/ui_audit/decomp/server_methods.txt`
   - `evidence/ui_audit/decomp/peer_methods.txt`
   - `evidence/ui_audit/decomp/transfer_methods.txt`

## Stage 5C Completion

1. Promoted three room/term control messages to runtime-backed evidence:
   - `SM_ADD_CHATROOM` (`10`)
   - `SM_ADD_LIKE_TERM` (`51`)
   - `SM_REMOVE_LIKE_TERM` (`52`)
2. Added authenticated runtime scenario:
   - `captures/redacted/login-room-term-control`
3. Replaced opaque handling with typed protocol variants and builders:
   - `AddChatRoomPayload`
   - `ServerMessage::AddLikeTerm`
   - `ServerMessage::RemoveLikeTerm`
   - `build_add_chatroom_request(...)`
   - `build_add_like_term_request(...)`
   - `build_remove_like_term_request(...)`
4. Extended SDK and CLI control operations:
   - `SessionClient::add_chatroom(...)`
   - `SessionClient::add_like_term(...)`
   - `SessionClient::remove_like_term(...)`
   - `soul-cli room add --room ...`
   - `soul-cli discover add-like-term --term ...`
   - `soul-cli discover remove-like-term --term ...`
5. Extended verification and contract coverage:
   - `tests/protocol/test_stage5c_room_term_control_contract.py`
   - semantic verifier regression for like-term diffs in `rust/verify`
   - required semantic run list includes `login-room-term-control`
6. Preserved full matrix baseline:
   - `implemented+mapped=131`
   - `mapped_not_implemented=0`
   - `missing=0`

## Stage 5D-5H Completion

1. Promoted `23` previously opaque control-message families to typed payloads in `rust/protocol`:
   - S5D: `150`, `151`, `152`, `153`
   - S5E: `86`, `87`, `88`, `90`, `100`
   - S5F: `136`, `137`, `139`, `140`, `145`
   - S5G: `58`, `62`, `63`, `66`, `117`, `118`
   - S5H: `28`, `32`, `130`
2. Added runtime-authenticated redacted scenarios:
   - `captures/redacted/login-global-room-control`
   - `captures/redacted/login-parent-disconnect-control`
   - `captures/redacted/login-private-room-membership-control`
   - `captures/redacted/login-text-control`
   - `captures/redacted/login-system-control`
3. Reduced generic opaque server-control closure set from `34` to `15` codes in `OPAQUE_SERVER_CONTROL_CODES`.
4. Extended typed builder surface for all S5D-S5H messages and added protocol regression tests:
   - `s5d_s5h_request_builders_emit_expected_codes`
   - `s5d_s5h_control_messages_decode_typed_payloads`
   - `tests/protocol/test_stage5d_s5h_typed_control_contract.py`
5. Regenerated authoritative artifacts and kept matrix closure unchanged:
   - `implemented+mapped=131`
   - `mapped_not_implemented=0`
   - `missing=0`

## Stage 6A Completion

1. Added canonical stage-state registry artifact:
   - `analysis/state/stage_registry.json`
2. Added dashboard and state data generation artifacts:
   - `docs/state/project-dashboard.html`
   - `docs/state/project-dashboard-data.json`
3. Added codebase topology visualizer artifacts:
   - `docs/state/codebase-visualizer.md`
   - `docs/state/codebase-visualizer.html`
   - `docs/state/codebase-graph.json`
4. Added collapsed PR catalog generation and output:
   - `tools/docs/generate_pr_index.py`
   - `docs/pr/index.md`
5. Added synchronized regeneration workflow command:
   - `scripts/sync_state_dashboards.sh`
6. Curated Zensical nav for high-signal state surfaces:
   - dashboard, roadmap, protocol matrix, visualizer, verification status, and PR catalog.

## Stage 6B Completion

1. Added executable S5A closure verifier:
   - `tools/state/verify_s5a_closure.py`
2. Added generated closure artifact:
   - `docs/state/s5a-closure-audit.json`
3. Added closure audit documentation page:
   - `docs/state/s5a-closure-audit.md`
4. Added regression coverage:
   - `tests/state/test_s5a_closure_audit.py`
5. Wired closure verification into state sync workflow:
   - `scripts/sync_state_dashboards.sh`
6. Verified closure objectives are all green:
   - opaque -> typed runtime evidence for S5A set
   - runtime captures for parent/distributed and global/distributed control
   - decompression-aware parser coverage for `PM_SHARED_FILES_IN_FOLDER`
   - residual hypotheses closed for `SM_GET_USER_PRIVILEGES_STATUS` and `SM_UPLOAD_SPEED`

## Stage 6C Completion

1. Added executable opaque-tail baseline report generator:
   - `tools/state/report_opaque_tail.py`
2. Added generated opaque-tail inventory artifact:
   - `docs/state/opaque-tail-report.json`
3. Added S6 plan page for batch execution:
   - `docs/state/opaque-tail-plan.md`
4. Added regression coverage:
   - `tests/state/test_opaque_tail_report.py`
5. Wired opaque-tail report generation into sync workflow:
   - `scripts/sync_state_dashboards.sh`
6. Published execution batches for S6 implementation:
   - `S6-Batch-1`: `41`, `61`, `67`, `70`
   - `S6-Batch-2`: `71`, `73`, `82`, `93`, `102`
   - `S6-Batch-3`: `114`, `115`, `116`, `138`, `141`, `142`

## Stage 6D Completion

1. Promoted the full S6 batch closure from generic opaque control handling to typed payload variants in `rust/protocol`:
   - `41`, `61`, `67`, `70`, `71`, `73`, `82`, `93`, `102`, `114`, `115`, `116`, `138`, `141`, `142`
2. Added typed decode/encode coverage and builders for the promoted families, plus protocol regression tests:
   - `s6d_batch1_messages_decode_typed_payloads`
   - `s6d_batch2_messages_decode_typed_payloads`
   - `s6d_batch3_messages_decode_typed_payloads`
   - `s6d_opaque_server_control_tail_is_empty`
3. Added Stage 6 runtime capture tooling and artifacts:
   - `tools/runtime/generate_stage6_typed_batches_captures.py`
   - `captures/redacted/login-s6-batch1-control`
   - `captures/redacted/login-s6-batch2-control`
   - `captures/redacted/login-s6-batch3-control`
4. Closed generic opaque-tail baseline:
   - `OPAQUE_SERVER_CONTROL_CODES`: `0`
   - `docs/state/opaque-tail-report.json`: `opaque_tail_count=0`
5. Preserved full matrix baseline:
   - `implemented+mapped=131`
   - `mapped_not_implemented=0`
   - `missing=0`

## Stage 6E Completion

1. Added Stage 6E authenticated runtime capture generator and redacted runs:
   - `tools/runtime/generate_stage6e_legacy_control_captures.py`
   - `captures/redacted/login-legacy-room-operatorship-control`
   - `captures/redacted/login-legacy-distributed-control`
2. Promoted dedicated legacy payload branches from opaque to typed in `rust/protocol`:
   - `SM_REMOVE_ROOM_OPERATORSHIP` (`146`)
   - `SM_REMOVE_OWN_ROOM_OPERATORSHIP` (`147`)
   - `SM_DNET_LEVEL` (`126`)
   - `SM_DNET_GROUP_LEADER` (`127`)
   - `SM_DNET_CHILD_DEPTH` (`129`)
3. Added typed builders and decode regression coverage for promoted messages:
   - `build_remove_room_operatorship_request(...)`
   - `build_remove_own_room_operatorship_request(...)`
   - `build_dnet_level_request(...)`
   - `build_dnet_group_leader_request(...)`
   - `build_dnet_child_depth_request(...)`
   - `s6e_legacy_control_messages_decode_typed_payloads`
4. Dedicated residual semantics remained explicitly tracked for S6F closure:
   - `SM_DNET_DELIVERY_REPORT` (`128`)
   - `SM_FLOOD` (`131`)
5. Added protocol contract regression for S6E:
   - `tests/protocol/test_stage6e_legacy_opaque_reduction_contract.py`
6. Preserved full matrix baseline:
   - `implemented+mapped=131`
   - `mapped_not_implemented=0`
   - `missing=0`

## Stage 5A Completion

1. Promoted runtime-backed evidence and confidence closure for previously static/hypothesis rows:
   - `SM_SET_PARENT_MIN_SPEED` (`83`)
   - `SM_SET_PARENT_SPEED_CONNECTION_RATIO` (`84`)
   - `SM_GET_ROOM_TICKER` (`113`)
   - `SM_UPLOAD_SPEED` (`121`)
   - `SM_GET_USER_PRIVILEGES_STATUS` (`122`, `medium -> high`)
2. Added authenticated runtime capture scenario:
   - `captures/redacted/login-parent-distributed-control`
3. Replaced selected opaque protocol handling with typed payloads:
   - `ParentMinSpeedPayload`
   - `ParentSpeedConnectionRatioPayload`
   - `RoomTickerRequestPayload`
   - `RoomTickerPayload`
4. Added decompression-aware parser support for `PM_SHARED_FILES_IN_FOLDER`:
   - zlib decompression with safety limit guard
   - typed listing classification (`BinaryEntries`, `Utf8Lines`, `OpaqueBytes`)
5. Extended core/CLI/verify surfaces:
   - `SessionClient::request_room_ticker(...)`
   - `SessionClient::set_upload_speed(...)`
   - CLI `room ticker ...`
   - semantic verifier test coverage for room-ticker field diffs
6. Preserved full protocol matrix coverage while increasing semantic depth:
   - `implemented+mapped=131`
   - `mapped_not_implemented=0`
   - `missing=0`

## Stage 4K Completion

1. Closed the full `missing` bucket by mapping and implementing `24` unresolved names/codes (`23` previously missing + `SM_PEER_MESSAGE_ALT` map closure):
   - `SM_ADD_USER`
   - `SM_REMOVE_USER`
   - `SM_SEND_CONNECT_TOKEN`
   - `SM_PLACE_IN_LINE`
   - `SM_PLACE_IN_LINE_RESPONSE`
   - `SM_ADD_PRIVILEGED_USER`
   - `SM_LOW_PRIORITY_FILE_SEARCH`
   - `SM_WISHLIST_WAIT`
   - `SM_DNET_LEVEL`
   - `SM_DNET_GROUP_LEADER`
   - `SM_DNET_DELIVERY_REPORT`
   - `SM_DNET_CHILD_DEPTH`
   - `SM_FLOOD`
   - `SM_REMOVE_ROOM_OPERATORSHIP`
   - `SM_REMOVE_OWN_ROOM_OPERATORSHIP`
   - `SM_JOIN_GLOBAL_ROOM`
   - `SM_LEAVE_GLOBAL_ROOM`
   - `SM_SAY_GLOBAL_ROOM`
   - `SM_SEARCH_CORRELATIONS`
   - `SM_PEER_MESSAGE_ALT`
   - `PM_SAY`
   - `PM_SEND_CONNECT_TOKEN`
   - `PM_PLACEHOLD_UPLOAD`
   - `PM_NOTHING`
2. Extended `rust/protocol` with S4K constants and codec coverage:
   - typed `UserLookupPayload` support for `SM_ADD_USER`, `SM_REMOVE_USER`, and `SM_ADD_PRIVILEGED_USER`
   - typed `FileSearchPayload` support for `SM_LOW_PRIORITY_FILE_SEARCH`
   - `OpaquePayload`-based encode/decode support for unresolved runtime-shape messages
3. Regenerated authoritative schema/docs/matrix from updated maps and protocol constants.
4. Raised matrix coverage to zero-missing baseline while preserving explicit `mapped_not_implemented` separation for remaining typed follow-up work.

## Stage 4J Completion

1. Expanded protocol mapping from 99 to 107 rows (`+8` S4J messages):
   - `SM_REMOVE_OWN_ROOM_MEMBERSHIP`
   - `SM_GIVE_UP_ROOM`
   - `SM_TRANSFER_ROOM_OWNERSHIP`
   - `SM_ADD_ROOM_MEMBERSHIP`
   - `SM_REMOVE_ROOM_MEMBERSHIP`
   - `SM_ENABLE_PRIVATE_ROOM_ADD`
   - `SM_CHANGE_PASSWORD`
   - `SM_ADD_ROOM_OPERATORSHIP`
2. Regenerated canonical schema/docs/matrix from authoritative maps.
3. Preserved verification baselines while continuing mapping-first expansion.
4. Kept S4J rows explicitly as `mapped_not_implemented` pending typed implementation.

## Stage 4I Completion

1. Expanded protocol mapping from 91 to 99 rows (`+8` S4I messages):
   - `SM_ADD_LIKE_TERM`
   - `SM_REMOVE_LIKE_TERM`
   - `SM_GET_ROOM_TICKER`
   - `SM_ROOM_TICKER_USER_ADDED`
   - `SM_ROOM_TICKER_USER_REMOVED`
   - `SM_SET_TICKER`
   - `SM_ADD_HATE_TERM`
   - `SM_REMOVE_HATE_TERM`
2. Regenerated canonical schema/docs/matrix from authoritative maps.
3. Preserved verification baselines while continuing mapping-first expansion.
4. Kept S4I rows explicitly as `mapped_not_implemented` pending typed implementation.

## Stage 4H Completion

1. Expanded protocol mapping from 83 to 91 rows (`+8` S4H messages):
   - `SM_ADD_CHATROOM`
   - `SM_SET_STATUS`
   - `SM_HEARTBEAT`
   - `SM_RELOGGED`
   - `SM_USER_LIST`
   - `SM_ROOM_ADDED`
   - `SM_ROOM_REMOVED`
   - `SM_CONNECT_TO_CLIENT`
2. Regenerated canonical schema/docs/matrix from authoritative maps.
3. Preserved verification baselines while continuing mapping-first expansion.
4. Kept S4H rows explicitly as `mapped_not_implemented` pending typed implementation.

## Stage 4G Completion

1. Expanded protocol mapping from 75 to 83 rows (`+8` S4G messages) from jump-table evidence:
   - `SM_SET_PARENT_MIN_SPEED`
   - `SM_SET_PARENT_SPEED_CONNECTION_RATIO`
   - `SM_SET_PARENT_INACTIVITY_BEFORE_DISCONNECT`
   - `SM_SET_SERVER_INACTIVITY_BEFORE_DISCONNECT`
   - `SM_NODES_IN_CACHE_BEFORE_DISCONNECT`
   - `SM_SET_SECONDS_BEFORE_PING_CHILDREN`
   - `SM_CAN_PARENT`
   - `SM_POSSIBLE_PARENTS`
2. Regenerated canonical schema/docs/matrix from authoritative maps.
3. Preserved runtime+semantic validation baseline while expanding mapped coverage.
4. Kept S4G rows explicitly as `mapped_not_implemented` pending typed protocol/core/CLI implementation.

## Stage 4F Completion

1. Expanded protocol mapping from 67 to 75 rows (`+8` S4F messages) using authoritative jump-table evidence:
   - `SM_COMMAND`
   - `SM_ADMIN_MESSAGE`
   - `SM_GLOBAL_USER_LIST`
   - `SM_SEND_DISTRIBUTIONS`
   - `SM_NOTE_PARENT`
   - `SM_CHILD_PARENT_MAP`
   - `SM_DNET_MESSAGE`
   - `SM_DNET_RESET`
2. Regenerated canonical schema/docs/matrix from authoritative maps:
   - `analysis/protocol/message_schema.json`
   - `docs/re/static/message-schema.md`
   - `docs/re/static/detangling.md`
   - `docs/verification/evidence-ledger.md`
   - `docs/state/protocol-matrix.md`
3. Preserved runtime+semantic verification baseline while expanding mapping-only coverage.
4. Marked new S4F rows as `mapped_not_implemented` in protocol matrix to keep implementation state explicit.
5. Updated roadmap/backlog/status artifacts for next domain iteration planning.

## Stage 4E Completion

1. Expanded protocol mapping from 65 to 67 rows (`+2` new S4E messages):
   - `SM_MESSAGE_USERS`
   - `SM_PEER_MESSAGE`
2. Upgraded runtime evidence quality for six pre-existing S4E-domain messages:
   - `SM_MESSAGE_USER`
   - `SM_MESSAGE_ACKED`
   - `SM_GET_USER_STATUS`
   - `SM_GET_USER_STATS`
   - `SM_GET_PEER_ADDRESS`
   - `SM_CONNECT_TO_PEER`
3. Added S4E runtime scenarios and redacted artifacts:
   - `login-private-message`
   - `login-user-state`
   - `login-peer-address-connect`
   - `login-message-users`
   - `login-peer-message`
4. Extended `rust/protocol` with S4E constants, payload variants, directional decode logic, and builders:
   - `CODE_SM_MESSAGE_USERS` (`149`)
   - `CODE_SM_PEER_MESSAGE` (`68`)
   - `CODE_SM_PEER_MESSAGE_ALT` (`292`, decode compatibility alias)
5. Extended `rust/core` with S4E operations:
   - `send_private_message(...)`
   - `wait_message_ack(...)`
   - `get_user_status(...)`
   - `get_user_stats(...)`
   - `get_peer_address(...)`
   - `connect_to_peer(...)`
   - `send_message_users(...)`
   - `collect_private_events(...)`
6. Extended `rust/cli` session command surface:
   - `session message --target-user ... --message ... [--wait-ack]`
   - `session message-users --targets a,b --message ...`
   - `session status --target-user ...`
   - `session stats --target-user ...`
   - `session peer-address --target-user ...`
   - `session connect-peer --target-user ... --token ... --connection-type ...`
   - `session watch-private --timeout-secs ...`
7. Extended semantic differential verification coverage and required-run set with S4E scenarios.
8. Governance was updated in `AGENTS.md`: each stage PR now runs two local review loops (security pass + code-simplifier pass) without `@codex review` dependency.

## Core Artifacts

- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`
- `docs/state/protocol-matrix.md`
- `docs/re/static/message-schema.md`
- `docs/verification/evidence-ledger.md`
- `docs/state/roadmap.md`
- `captures/redacted/login-private-message/manifest.redacted.json`
- `captures/redacted/login-user-state/manifest.redacted.json`
- `captures/redacted/login-peer-address-connect/manifest.redacted.json`
- `captures/redacted/login-message-users/manifest.redacted.json`
- `captures/redacted/login-peer-message/manifest.redacted.json`
- `captures/redacted/login-room-term-control/manifest.redacted.json`
- `captures/redacted/login-global-room-control/manifest.redacted.json`
- `captures/redacted/login-parent-disconnect-control/manifest.redacted.json`
- `captures/redacted/login-private-room-membership-control/manifest.redacted.json`
- `captures/redacted/login-text-control/manifest.redacted.json`
- `captures/redacted/login-system-control/manifest.redacted.json`
- `tools/runtime/generate_stage4e_private_userstate_captures.py`
- `tools/runtime/generate_stage5c_room_term_control_captures.py`
- `tools/runtime/generate_stage5d_global_control_captures.py`
- `tools/runtime/generate_stage5e_parent_disconnect_control_captures.py`
- `tools/runtime/generate_stage5f_private_room_membership_control_captures.py`
- `tools/runtime/generate_stage5g_text_control_captures.py`
- `tools/runtime/generate_stage5h_system_control_captures.py`
- `docs/pr/0009-s4e-private-messaging-user-state.md`
- `docs/pr/0010-s4f-global-admin-distributed-map.md`
- `docs/pr/0011-s4g-parent-distributed-tuning-map.md`
- `docs/pr/0012-s4h-global-system-control-map.md`
- `docs/pr/0013-s4i-ticker-term-control-map.md`
- `docs/pr/0014-s4j-private-room-ownership-map.md`
- `docs/pr/0015-s4k-missing-code-closure-protocol-implementation.md`
- `docs/pr/0016-s4l-mapped-not-implemented-closure.md`
- `docs/pr/0018-s5c-typed-runtime-hardening-wave2.md`
- `docs/pr/0019-s5d-s5h-control-typing-pack.md`

## Operational Notes

- Stage 2 core contract remains intact (`25/25`, `high=25`, `medium=0`, `low=0`).
- Total mapped protocol rows: `131`.
- Protocol matrix snapshot: tracked `131`, implemented+mapped `131`, mapped-not-implemented `0`, missing `0`.
- Runtime credentials remain local-only in `.env.local` and are never committed.
## Stage 6F Completion

1. Added dedicated Stage 6F authenticated runtime capture generator and redacted run:
   - `tools/runtime/generate_stage6f_residual_captures.py`
   - `captures/redacted/login-legacy-residual-control`
2. Promoted remaining dedicated residual payload branches from opaque to typed in `rust/protocol`:
   - `SM_DNET_DELIVERY_REPORT` (`128`)
   - `SM_FLOOD` (`131`)
3. Added typed builders and decode regression coverage for residual-closure messages:
   - `build_dnet_delivery_report_request(...)`
   - `build_flood_request(...)`
   - `s6e_legacy_control_messages_decode_typed_payloads` (extended to cover `128/131`)
4. Added protocol contract regression for S6F:
   - `tests/protocol/test_stage6f_residual_semantic_closure_contract.py`
5. Preserved full matrix baseline:
   - `implemented+mapped=131`
   - `mapped_not_implemented=0`
   - `missing=0`
