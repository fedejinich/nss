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
- Product direction remains unchanged: SDK+CLI first, custom evolvable app (not a 1:1 official client clone).

## Stage 4C Completion

1. Expanded protocol mapping from 47 to 56 rows (`+9` S4C messages):
   - `SM_IGNORE_USER`
   - `SM_UNIGNORE_USER`
   - `SM_GET_OWN_PRIVILEGES_STATUS`
   - `SM_GET_USER_PRIVILEGES_STATUS`
   - `SM_GIVE_PRIVILEGE`
   - `SM_INFORM_USER_OF_PRIVILEGES`
   - `SM_INFORM_USER_OF_PRIVILEGES_ACK`
   - `PM_GET_SHARED_FILES_IN_FOLDER`
   - `PM_SHARED_FILES_IN_FOLDER`
2. Added S4C runtime scenarios and redacted artifacts:
   - `login-privileges-social`
   - `peer-folder-local`
3. Extended `rust/protocol` with S4C constants, payloads, codecs, and builders for privileges/social and peer-folder messages.
4. Extended `rust/core` with social-control and privileges operations:
   - `ignore_user(...)`
   - `unignore_user(...)`
   - `get_own_privileges_status(...)`
   - `get_user_privileges_status(...)`
   - `give_privilege(...)`
   - `inform_user_of_privileges(...)`
   - `inform_user_of_privileges_ack(...)`
5. Extended `rust/cli` session commands:
   - `session ignore-user --target-user <name>`
   - `session unignore-user --target-user <name>`
   - `session own-privileges`
   - `session user-privileges --target-user <name>`
   - `session give-privilege --target-user <name> --days <n>`
   - `session inform-privileges --token <n> --target-user <name>`
   - `session inform-privileges-ack --token <n>`
6. Added peer-folder frame helper command:
   - `build-peer-folder-request --directory <path>`
7. Extended differential verification required-run set to include S4C scenarios and kept all validation gates green.

## Core Artifacts

- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`
- `docs/state/protocol-matrix.md`
- `docs/re/static/message-schema.md`
- `docs/verification/evidence-ledger.md`
- `docs/state/roadmap.md`
- `captures/redacted/login-privileges-social/manifest.redacted.json`
- `captures/redacted/peer-folder-local/manifest.redacted.json`
- `tools/runtime/generate_stage4c_privileges_social_captures.py`

## Operational Notes

- Stage 2 core contract remains intact (`25/25`, `high=25`, `medium=0`, `low=0`).
- Total mapped protocol rows: `56`.
- Protocol matrix snapshot: tracked `130`, implemented+mapped `56`, missing `74`.
- Runtime credentials remain local-only in `.env.local` and are never committed.
- `SM_BAN_USER` remains deferred until authoritative code/evidence is resolved.
