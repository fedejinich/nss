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
- Product direction remains unchanged: SDK+CLI first, custom evolvable app (not a 1:1 official client clone).

## Stage 4D Completion

1. Expanded protocol mapping from 56 to 65 rows (`+9` new S4D messages):
   - `SM_BAN_USER`
   - `SM_PRIVILEGED_LIST`
   - `SM_GET_RECOMMENDED_USERS`
   - `SM_GET_TERM_RECOMMENDATIONS`
   - `SM_GET_RECOMMENDATION_USERS`
   - `PM_INVITE_USER_TO_ROOM`
   - `PM_CANCELLED_QUEUED_TRANSFER`
   - `PM_MOVE_DOWNLOAD_TO_TOP`
   - `PM_QUEUED_DOWNLOADS`
2. Promoted two legacy peer-search mappings from medium to high using runtime evidence:
   - `PM_EXACT_FILE_SEARCH_REQUEST`
   - `PM_INDIRECT_FILE_SEARCH_REQUEST`
3. Added S4D runtime scenarios and redacted artifacts:
   - `login-privilege-messaging`
   - `peer-legacy-local`
4. Added authoritative static extraction for unresolved code mapping:
   - `tools/re/extract_message_codes.py`
   - `evidence/reverse/message_codes_jump_table.json`
   - `evidence/reverse/message_codes_jump_table.md`
5. Extended `rust/protocol` with S4D constants, payloads, codecs, and builders for privilege-messaging and peer-legacy messages.
6. Extended `rust/core` with S4D operations:
   - `ban_user(...)`
   - `get_privileged_list(...)`
   - `get_recommended_users(...)`
   - `get_term_recommendations(...)`
   - `get_recommendation_users(...)`
7. Extended `rust/cli` command surface:
   - `session ban-user --target-user <name>`
   - `session privileged-list`
   - `discover recommended-users`
   - `discover term-recommendations --term <value>`
   - `discover recommendation-users --term <value>`
8. Extended differential verification required-run set with S4D scenarios and kept all validation gates green.

## Core Artifacts

- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`
- `docs/state/protocol-matrix.md`
- `docs/re/static/message-schema.md`
- `docs/verification/evidence-ledger.md`
- `docs/state/roadmap.md`
- `captures/redacted/login-privilege-messaging/manifest.redacted.json`
- `captures/redacted/peer-legacy-local/manifest.redacted.json`
- `tools/runtime/generate_stage4d_privilege_legacy_captures.py`
- `evidence/reverse/message_codes_jump_table.md`

## Operational Notes

- Stage 2 core contract remains intact (`25/25`, `high=25`, `medium=0`, `low=0`).
- Total mapped protocol rows: `65`.
- Protocol matrix snapshot: tracked `130`, implemented+mapped `65`, missing `65`.
- Runtime credentials remain local-only in `.env.local` and are never committed.
