# Project Status

## Date

- 2026-02-14

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
- Product direction remains unchanged: SDK+CLI first, custom evolvable app (not a 1:1 official client clone).

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
8. Added governance rule in `AGENTS.md`: mandatory two-round `@codex review` loop for each stage PR before merge.

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
- `tools/runtime/generate_stage4e_private_userstate_captures.py`
- `docs/pr/0009-s4e-private-messaging-user-state.md`

## Operational Notes

- Stage 2 core contract remains intact (`25/25`, `high=25`, `medium=0`, `low=0`).
- Total mapped protocol rows: `67`.
- Protocol matrix snapshot: tracked `131`, implemented+mapped `67`, missing `63`.
- Runtime credentials remain local-only in `.env.local` and are never committed.
