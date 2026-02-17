# Protocol Backlog (Post S4L)

## Objective

Maintain full protocol coverage (`implemented+mapped=131`) while executing S9P transfer/protocol parity closure against SoulseekQt runtime behavior, reverse-mapping architecture and persistence-critical file formats, and preserving runtime-complete evidence (`verified_runtime=131`, `verified_static=0`) plus semantic-depth closure (no unresolved `raw_tail/raw_payload` schema fields).

## Stage S9P Note (Protocol Parity Program)

- S9P is now the active stage, and S9B/S9C are dependency-paused until S9P closure.
- Explicit analysis tracks:
  - static: architecture/dispatch/state-machine + persistence format surface recovery,
  - runtime: official protocol and file-I/O capture corpus,
  - synthesis: parity matrix + architecture/format replicability classification,
  - patch: transfer behavior convergence with regression coverage.
- Mandatory S9P transfer scenarios:
  - `TC-FLIM-001`
  - `TC-TRANSFER-REQ-002`
  - `TC-QUEUE-REJECT-003`
  - `TC-PFLOWS-004`
  - `TC-TIMEOUT-005`
  - `TC-DIFF-006`
  - `TC-REG-007`
- Mandatory S9P format scenarios:
  - `TC-FMT-001` (QSettings roundtrip)
  - `TC-FMT-002` (transfer-state roundtrip semantics)
  - `TC-FMT-003` (hotlist import parse parity)
  - `TC-FMT-004` (I/O payload redaction policy compliance)
  - `TC-FMT-005` (format-map schema validator)

## Stage 7A Note (Strict Runtime Closure)

- Stage 7A introduced no new protocol codes/messages.
- Stage 7A closed runtime provenance for the prior static-only set (`30` rows) and promoted all rows to `verified_runtime`.
- New runtime closure artifacts:
  - `analysis/state/runtime_coverage_registry.json`
  - `docs/state/runtime-coverage.json`
  - `docs/state/runtime-coverage.md`
  - `captures/redacted/login-static-server-runtime`
  - `captures/redacted/peer-static-runtime`

## Stage 7B Note (Semantic-Tail Closure)

- Stage 7B introduced no new protocol codes/messages.
- Stage 7B replaced residual placeholder tails with explicit extension semantics in schema/protocol:
  - `raw_tail` -> `extension_reserved_bytes`
- Runtime-tail evidence scenario:
  - `captures/redacted/login-partial-tail-runtime`

## Stage 7C Note (Core Transfer Orchestration)

- Stage 7C introduced no new protocol codes/messages.
- Stage 7C added higher-level core/CLI orchestration surfaces:
  - `SessionClient::search_select_and_download(...)`
  - `soul-cli session download-auto ...`
- Runtime orchestration scenario:
  - `captures/redacted/login-search-download-auto`

## Stage 6A Note (State UX + Dashboard Tooling)

- Stage 6A introduced no protocol message additions or removals.
- Stage 6A added dashboard and codebase-visualizer observability tooling:
  - `analysis/state/stage_registry.json`
  - `docs/state/project-dashboard-data.json`
  - `docs/state/codebase-graph.json`
  - `docs/pr/index.md`

## Stage 6B Note (S5A Closure Hardening Audit)

- Stage 6B introduced no protocol message additions or removals.
- Stage 6B added executable closure verification for S5A objectives:
  - `tools/state/verify_s5a_closure.py`
  - `docs/state/s5a-closure-audit.json`
  - `docs/state/s5a-closure-audit.md`
- Stage 6B also added regression coverage:
  - `tests/state/test_s5a_closure_audit.py`

## Stage 6C Note (Opaque-Tail Baseline + Batch Plan)

- Stage 6C introduced no protocol message additions or removals.
- Stage 6C added executable opaque-tail inventory and S6 batch plan artifacts:
  - `tools/state/report_opaque_tail.py`
  - `docs/state/opaque-tail-report.json`
  - `docs/state/opaque-tail-plan.md`
- Stage 6C also added regression coverage:
  - `tests/state/test_opaque_tail_report.py`

## Stage 6D Note (Opaque-Tail Typed Batches Execution)

- Stage 6D promoted the full S6 batch closure from generic opaque handling to typed payload coverage:
  - Batch 1: `41`, `61`, `67`, `70`
  - Batch 2: `71`, `73`, `82`, `93`, `102`
  - Batch 3: `114`, `115`, `116`, `138`, `141`, `142`
- Stage 6D added runtime capture tooling and artifacts:
  - `tools/runtime/generate_stage6_typed_batches_captures.py`
  - `captures/redacted/login-s6-batch1-control`
  - `captures/redacted/login-s6-batch2-control`
  - `captures/redacted/login-s6-batch3-control`
- Generic opaque closure baseline is now:
  - `OPAQUE_SERVER_CONTROL_CODES = []`
  - `docs/state/opaque-tail-report.json` -> `opaque_tail_count=0`

## Stage 5B Note (UI/Feature Audit)

- Stage 5B added no new protocol codes/messages.
- Stage 5B produced UI-to-protocol bridge evidence in:
  - `docs/state/soulseek-feature-inventory.md`
  - `evidence/reverse/ui_handler_symbols_nm.txt`
  - `evidence/ui_audit/decomp/mainwindow_methods.txt`
  - `evidence/ui_audit/decomp/server_methods.txt`
  - `evidence/ui_audit/decomp/peer_methods.txt`
  - `evidence/ui_audit/decomp/transfer_methods.txt`

## Completed in S3B (Rooms + Presence Batch)

- `SM_ROOM_LIST`
- `SM_JOIN_ROOM`
- `SM_LEAVE_ROOM`
- `SM_USER_JOINED_ROOM`
- `SM_USER_LEFT_ROOM`
- `SM_ROOM_MEMBERS`
- `SM_ROOM_OPERATORS`
- `SM_SAY_CHATROOM`

## Completed in S4A (Recommendations + Discovery Batch)

- `SM_GET_RECOMMENDATIONS`
- `SM_GET_MY_RECOMMENDATIONS`
- `SM_GET_GLOBAL_RECOMMENDATIONS`
- `SM_GET_USER_RECOMMENDATIONS`
- `SM_GET_SIMILAR_TERMS`

## Completed in S4B (Peer Advanced + Room Moderation Batch)

- `PM_USER_INFO_REQUEST`
- `PM_USER_INFO_REPLY`
- `PM_EXACT_FILE_SEARCH_REQUEST`
- `PM_INDIRECT_FILE_SEARCH_REQUEST`
- `PM_UPLOAD_PLACE_IN_LINE_REQUEST`
- `SM_ADD_ROOM_MEMBER`
- `SM_REMOVE_ROOM_MEMBER`
- `SM_ADD_ROOM_OPERATOR`
- `SM_REMOVE_ROOM_OPERATOR`

## Completed in S4C (Privileges/Social + Peer Folder Batch)

- `SM_IGNORE_USER`
- `SM_UNIGNORE_USER`
- `SM_GET_OWN_PRIVILEGES_STATUS`
- `SM_GET_USER_PRIVILEGES_STATUS`
- `SM_GIVE_PRIVILEGE`
- `SM_INFORM_USER_OF_PRIVILEGES`
- `SM_INFORM_USER_OF_PRIVILEGES_ACK`
- `PM_GET_SHARED_FILES_IN_FOLDER`
- `PM_SHARED_FILES_IN_FOLDER`

Status: completed in S4C with runtime captures (`login-privileges-social`, `peer-folder-local`), protocol implementation, and semantic differential verification.

## Completed in S4D (Privilege/Messaging Gaps + Peer Legacy Cleanup)

- `SM_BAN_USER`
- `SM_PRIVILEGED_LIST`
- `SM_GET_RECOMMENDATION_USERS`
- `SM_GET_RECOMMENDED_USERS`
- `SM_GET_TERM_RECOMMENDATIONS`
- `PM_INVITE_USER_TO_ROOM`
- `PM_CANCELLED_QUEUED_TRANSFER`
- `PM_QUEUED_DOWNLOADS`
- `PM_MOVE_DOWNLOAD_TO_TOP`
- runtime promotions:
  - `PM_EXACT_FILE_SEARCH_REQUEST` (`medium -> high`)
  - `PM_INDIRECT_FILE_SEARCH_REQUEST` (`medium -> high`)

Status: completed in S4D with runtime captures (`login-privilege-messaging`, `peer-legacy-local`), jump-table static extraction (`SM_BAN_USER`), protocol implementation, and semantic verification updates.

## Completed in S4E (Private Messaging + User-State Domain)

- `SM_MESSAGE_USER`
- `SM_MESSAGE_ACKED`
- `SM_GET_USER_STATUS`
- `SM_GET_USER_STATS`
- `SM_GET_PEER_ADDRESS`
- `SM_CONNECT_TO_PEER`
- `SM_MESSAGE_USERS`
- `SM_PEER_MESSAGE`

Status: completed in S4E with runtime captures (`login-private-message`, `login-user-state`, `login-peer-address-connect`, `login-message-users`, `login-peer-message`), protocol implementation, and semantic verification updates.

## Completed in S4F (Global/Admin/Distributed Control Mapping Batch)

- `SM_COMMAND`
- `SM_ADMIN_MESSAGE`
- `SM_GLOBAL_USER_LIST`
- `SM_SEND_DISTRIBUTIONS`
- `SM_NOTE_PARENT`
- `SM_CHILD_PARENT_MAP`
- `SM_DNET_MESSAGE`
- `SM_DNET_RESET`

Status: completed in S4F with authoritative static mapping evidence from jump-table extraction and synchronized schema/docs/matrix artifacts.

## Completed in S4G (Parent/Distributed Tuning Mapping Batch)

- `SM_SET_PARENT_MIN_SPEED`
- `SM_SET_PARENT_SPEED_CONNECTION_RATIO`
- `SM_SET_PARENT_INACTIVITY_BEFORE_DISCONNECT`
- `SM_SET_SERVER_INACTIVITY_BEFORE_DISCONNECT`
- `SM_NODES_IN_CACHE_BEFORE_DISCONNECT`
- `SM_SET_SECONDS_BEFORE_PING_CHILDREN`
- `SM_CAN_PARENT`
- `SM_POSSIBLE_PARENTS`

Status: completed in S4G with authoritative static mapping evidence from jump-table extraction and synchronized schema/docs/matrix artifacts.

## Completed in S4H (Global Room/System Control Mapping Batch)

- `SM_ADD_CHATROOM`
- `SM_SET_STATUS`
- `SM_HEARTBEAT`
- `SM_RELOGGED`
- `SM_USER_LIST`
- `SM_ROOM_ADDED`
- `SM_ROOM_REMOVED`
- `SM_CONNECT_TO_CLIENT`

Status: completed in S4H with authoritative static mapping evidence from jump-table extraction and synchronized schema/docs/matrix artifacts.

## Completed in S4I (Ticker/Term Control Mapping Batch)

- `SM_ADD_LIKE_TERM`
- `SM_REMOVE_LIKE_TERM`
- `SM_GET_ROOM_TICKER`
- `SM_ROOM_TICKER_USER_ADDED`
- `SM_ROOM_TICKER_USER_REMOVED`
- `SM_SET_TICKER`
- `SM_ADD_HATE_TERM`
- `SM_REMOVE_HATE_TERM`

Status: completed in S4I with authoritative static mapping evidence from jump-table extraction and synchronized schema/docs/matrix artifacts.

## Completed in S4J (Private Room Ownership/Membership Mapping Batch)

- `SM_REMOVE_OWN_ROOM_MEMBERSHIP`
- `SM_GIVE_UP_ROOM`
- `SM_TRANSFER_ROOM_OWNERSHIP`
- `SM_ADD_ROOM_MEMBERSHIP`
- `SM_REMOVE_ROOM_MEMBERSHIP`
- `SM_ENABLE_PRIVATE_ROOM_ADD`
- `SM_CHANGE_PASSWORD`
- `SM_ADD_ROOM_OPERATORSHIP`

Status: completed in S4J with authoritative static mapping evidence from jump-table extraction and synchronized schema/docs/matrix artifacts.

## Completed in S4K (Missing-Code Closure + Global/Distributed Tail + Peer Control)

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

Status: completed in S4K with authoritative static mapping evidence from jump-table extraction plus protocol codec coverage in `rust/protocol`. Matrix `missing` bucket is now `0`.

## Completed in S4L (Mapped-not-implemented Closure)

- Promoted all remaining mapped-only rows (`40`) to implemented coverage in `rust/protocol`.
- Added `OpaqueServerControlPayload` decode/encode handling for unresolved server control payload shapes while preserving code-level traceability.
- Matrix reached full coverage baseline:
  - `implemented+mapped=131`
  - `mapped_not_implemented=0`
  - `missing=0`

## Completed in S5A (Typed Runtime Hardening Wave 1)

- Runtime-promoted typed control messages:
  - `SM_SET_PARENT_MIN_SPEED` (`83`)
  - `SM_SET_PARENT_SPEED_CONNECTION_RATIO` (`84`)
  - `SM_GET_ROOM_TICKER` (`113`)
  - `SM_UPLOAD_SPEED` (`121`)
  - `SM_GET_USER_PRIVILEGES_STATUS` (`122`, promoted `medium -> high`)
- Runtime scenario added:
  - `login-parent-distributed-control`
- Parser hardening:
  - `PM_SHARED_FILES_IN_FOLDER` now supports decompression-aware parsing with zlib safety limit and typed listing format classification.
- SDK/CLI hardening:
  - `SessionClient::request_room_ticker(...)`
  - `SessionClient::set_upload_speed(...)`
  - `soul-cli room ticker --room ...`
- Coverage baseline preserved:
  - `implemented+mapped=131`
  - `mapped_not_implemented=0`
  - `missing=0`

## Completed in S5C (Typed Runtime Hardening Wave 2)

- Runtime-promoted typed control messages:
  - `SM_ADD_CHATROOM` (`10`)
  - `SM_ADD_LIKE_TERM` (`51`)
  - `SM_REMOVE_LIKE_TERM` (`52`)
- Runtime scenario added:
  - `login-room-term-control`
- SDK/CLI additions:
  - `SessionClient::add_chatroom(...)`
  - `SessionClient::add_like_term(...)`
  - `SessionClient::remove_like_term(...)`
  - `soul-cli room add --room ...`
  - `soul-cli discover add-like-term --term ...`
  - `soul-cli discover remove-like-term --term ...`
- Coverage baseline preserved:
  - `implemented+mapped=131`
  - `mapped_not_implemented=0`
  - `missing=0`

## Completed in S5D-S5H (Typed Runtime Hardening Waves 3-7)

- S5D (`global/discovery control`):
  - `SM_JOIN_GLOBAL_ROOM` (`150`)
  - `SM_LEAVE_GLOBAL_ROOM` (`151`)
  - `SM_SAY_GLOBAL_ROOM` (`152`)
  - `SM_SEARCH_CORRELATIONS` (`153`)
  - runtime scenario: `login-global-room-control`
- S5E (`parent/disconnect control`):
  - `SM_SET_PARENT_INACTIVITY_BEFORE_DISCONNECT` (`86`)
  - `SM_SET_SERVER_INACTIVITY_BEFORE_DISCONNECT` (`87`)
  - `SM_NODES_IN_CACHE_BEFORE_DISCONNECT` (`88`)
  - `SM_SET_SECONDS_BEFORE_PING_CHILDREN` (`90`)
  - `SM_CAN_PARENT` (`100`)
  - runtime scenario: `login-parent-disconnect-control`
- S5F (`private-room membership/ownership control`):
  - `SM_REMOVE_OWN_ROOM_MEMBERSHIP` (`136`)
  - `SM_GIVE_UP_ROOM` (`137`)
  - `SM_ADD_ROOM_MEMBERSHIP` (`139`)
  - `SM_REMOVE_ROOM_MEMBERSHIP` (`140`)
  - `SM_ADD_ROOM_OPERATORSHIP` (`145`)
  - runtime scenario: `login-private-room-membership-control`
- S5G (`text-control payloads`):
  - `SM_COMMAND` (`58`)
  - `SM_ROOM_ADDED` (`62`)
  - `SM_ROOM_REMOVED` (`63`)
  - `SM_ADMIN_MESSAGE` (`66`)
  - `SM_ADD_HATE_TERM` (`117`)
  - `SM_REMOVE_HATE_TERM` (`118`)
  - runtime scenario: `login-text-control`
- S5H (`system-control payloads`):
  - `SM_SET_STATUS` (`28`)
  - `SM_HEARTBEAT` (`32`)
  - `SM_DNET_RESET` (`130`)
  - runtime scenario: `login-system-control`

Status:

- `23` message families promoted from opaque to typed runtime-backed payloads.
- `OPAQUE_SERVER_CONTROL_CODES` reduced from `34` to `15`.
- Full matrix baseline preserved:
  - `implemented+mapped=131`
  - `mapped_not_implemented=0`
  - `missing=0`

## Stage 6E Completion Note (Dedicated Legacy Opaque Reduction)

- Stage 6E added authenticated runtime runs for dedicated legacy control families:
  - `captures/redacted/login-legacy-room-operatorship-control`
  - `captures/redacted/login-legacy-distributed-control`
- Stage 6E promoted these dedicated legacy variants from opaque to typed payload handling:
  - `SM_REMOVE_ROOM_OPERATORSHIP` (`146`)
  - `SM_REMOVE_OWN_ROOM_OPERATORSHIP` (`147`)
  - `SM_DNET_LEVEL` (`126`)
  - `SM_DNET_GROUP_LEADER` (`127`)
  - `SM_DNET_CHILD_DEPTH` (`129`)

## Stage 6F Completion Note (Residual Semantic Closure)

- Stage 6F added dedicated authenticated runtime residual probes:
  - `captures/redacted/login-legacy-residual-control`
- Stage 6F promoted final dedicated residual variants to typed payload handling:
  - `SM_DNET_DELIVERY_REPORT` (`128`)
  - `SM_FLOOD` (`131`)
- Dedicated legacy residual ambiguity is now closed for the S6 scope.

## Execution Strategy

1. Preserve the full-coverage matrix baseline (`131/131`) on every stage.
2. Replace opaque decoding with typed schemas only when runtime/static evidence is sufficient.
3. Regenerate schema/docs from authoritative maps and protocol constants:
   - `scripts/derive_message_schema.sh`
   - `python3 scripts/kb_sync_docs.py`
4. Extend SDK/CLI/verify for newly typed message families once protocol decode is stable.
5. Keep regression green (`scripts/run_regression.sh`) before stage closure.

## Remaining Backlog After S7

Protocol mapping and implementation backlog is closed.

Remaining large-scale work is productization and hardening:

1. Stage 8B completion hardening:
   - operational soak checks for TUI flow handling
   - tighter recovery behavior around transient disconnects
2. Stage 8C release hardening:
   - config and log redaction review
   - packaging and operator lifecycle runbooks
   - final closure gate execution and release checklist
