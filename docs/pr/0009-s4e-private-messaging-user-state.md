# PR 0009 - S4E: private messaging and user-state protocol batch

## Branch

- `codex/s4e-private-messaging-user-state`

## Objective

Close Stage 4E with:

1. Typed protocol coverage for private messaging and user-state server flows.
2. Runtime-authenticated evidence for S4E scenarios with redacted artifacts.
3. SDK+CLI operations for messaging, status/stats, and peer address/connect requests.
4. Semantic differential verification extension for S4E payloads.
5. Knowledge-base and status synchronization, including protocol matrix refresh.

## Scope

### Protocol constants, payloads, and directional decode

- Added/confirmed constants in `rust/protocol/src/lib.rs`:
  - `CODE_SM_MESSAGE_USERS = 149`
  - `CODE_SM_PEER_MESSAGE = 68`
  - `CODE_SM_PEER_MESSAGE_ALT = 292` (decode compatibility alias)
- Added S4E typed payloads:
  - `PeerAddressResponsePayload`
  - `UserStatusResponsePayload`
  - `UserStatsResponsePayload`
  - `MessageUserIncomingPayload`
  - `MessageUsersPayload`
  - `PeerMessagePayload`
- Extended `ServerMessage` with additive variants for directional request/response and incoming private events.
- Updated `encode_server_message` and `decode_server_message` for overlapping server message codes:
  - `3`, `7`, `18`, `22`, `36`
- Added builder helpers:
  - `build_message_user_request(...)`
  - `build_message_users_request(...)`
  - `build_get_user_status_request(...)`
  - `build_get_user_stats_request(...)`
  - `build_get_peer_address_request(...)`
  - `build_connect_to_peer_request(...)`

### Core session operations

- Extended `rust/core/src/lib.rs` `SessionClient` with typed S4E operations:
  - `send_private_message(...)`
  - `wait_message_ack(...)`
  - `get_user_status(...)`
  - `get_user_stats(...)`
  - `get_peer_address(...)`
  - `connect_to_peer(...)`
  - `send_message_users(...)`
  - `collect_private_events(...)`
- Added `PrivateEvent` model for inbound private message + ack consumption.

### CLI surface

- Extended `rust/cli/src/main.rs` session commands:
  - `session message --target-user <name> --message <text> [--wait-ack]`
  - `session message-users --targets a,b --message <text>`
  - `session status --target-user <name>`
  - `session stats --target-user <name>`
  - `session peer-address --target-user <name>`
  - `session connect-peer --target-user <name> --token <u32> --connection-type <str>`
  - `session watch-private --timeout-secs <n>`
- Default command output stays summary-oriented; sensitive content logging remains constrained.

### Runtime captures and evidence

- Added generator:
  - `tools/runtime/generate_stage4e_private_userstate_captures.py`
- Added Stage 4E redacted runs:
  - `captures/redacted/login-private-message`
  - `captures/redacted/login-user-state`
  - `captures/redacted/login-peer-address-connect`
  - `captures/redacted/login-message-users`
  - `captures/redacted/login-peer-message`
- Extended required runs in `scripts/run_diff_verify.sh` with all five S4E scenarios.

### Mapping/schema/KB sync

- Updated authoritative map:
  - `analysis/ghidra/maps/message_map.csv`
- Updated protocol schema generation input:
  - `tools/protocol/derive_schema.py`
- Regenerated:
  - `analysis/protocol/message_schema.json`
  - `docs/re/static/message-schema.md`
  - `docs/re/static/detangling.md`
  - `docs/verification/evidence-ledger.md`
  - `docs/state/protocol-matrix.md`
- Updated stage/status docs:
  - `docs/state/project-status.md`
  - `docs/state/verification-status.md`
  - `docs/state/protocol-backlog.md`
  - `docs/state/roadmap.md`
  - `docs/state/decompilation-status.md`
  - `TODO-CODEX.md`

### Governance update

- Updated `AGENTS.md` with mandatory two-round `@codex review` loop process for each stage PR.

## S4E Contract Outcome

Required 8-message pack:

1. `SM_MESSAGE_USER`
2. `SM_MESSAGE_ACKED`
3. `SM_GET_USER_STATUS`
4. `SM_GET_USER_STATS`
5. `SM_GET_PEER_ADDRESS`
6. `SM_CONNECT_TO_PEER`
7. `SM_MESSAGE_USERS`
8. `SM_PEER_MESSAGE`

Confidence outcome for contract set:

- `high=8`
- `medium=0`
- `low=0`

## Validation

```bash
python3 scripts/kb_validate.py
scripts/run_diff_verify.sh
scripts/run_regression.sh
```

All checks passed on this branch snapshot.

## Runtime Verification Snapshot

- Official server: `server.slsknet.org:2242`
- Auth tuple: `160/1`
- Runtime scenarios:
  - `login-private-message` (code `22` + code `23` path)
  - `login-user-state` (code `7` + code `36` request/response)
  - `login-peer-address-connect` (code `3` + code `18` request/response)
  - `login-message-users` (code `149`)
  - `login-peer-message` (deterministic local runtime covering code `68` and alias `292`)

## Retrospective

### Was there a more maintainable approach?

Yes. Directional decoding for overlapping server codes (`3`, `7`, `18`, `22`, `36`) is more maintainable than relying on a single ambiguous decode path.

### What was reused to avoid double writing?

1. Existing runtime redaction pipeline (`raw -> redacted`) and verification artifact format.
2. Existing schema/docs regeneration flow (`scripts/derive_message_schema.sh`, `scripts/kb_sync_docs.py`, protocol matrix generator).
3. Existing semantic verifier architecture with typed decode and unknown-message fallback hashing.

### What was removed or avoided to reduce maintenance surface?

1. Avoided stage-specific verifier wrappers by extending `scripts/run_diff_verify.sh`.
2. Avoided CLI-local protocol parsing by centralizing decode/encode in `rust/protocol`.
3. Avoided introducing new credential paths; reused existing `.env.local` runtime discipline.
