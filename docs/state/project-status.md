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
- Product direction remains unchanged: SDK+CLI first, custom evolvable app (not a 1:1 official client clone).

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
- `tools/runtime/generate_stage4e_private_userstate_captures.py`
- `tools/runtime/generate_stage5c_room_term_control_captures.py`
- `docs/pr/0009-s4e-private-messaging-user-state.md`
- `docs/pr/0010-s4f-global-admin-distributed-map.md`
- `docs/pr/0011-s4g-parent-distributed-tuning-map.md`
- `docs/pr/0012-s4h-global-system-control-map.md`
- `docs/pr/0013-s4i-ticker-term-control-map.md`
- `docs/pr/0014-s4j-private-room-ownership-map.md`
- `docs/pr/0015-s4k-missing-code-closure-protocol-implementation.md`
- `docs/pr/0016-s4l-mapped-not-implemented-closure.md`
- `docs/pr/0018-s5c-typed-runtime-hardening-wave2.md`

## Operational Notes

- Stage 2 core contract remains intact (`25/25`, `high=25`, `medium=0`, `low=0`).
- Total mapped protocol rows: `131`.
- Protocol matrix snapshot: tracked `131`, implemented+mapped `131`, mapped-not-implemented `0`, missing `0`.
- Runtime credentials remain local-only in `.env.local` and are never committed.
