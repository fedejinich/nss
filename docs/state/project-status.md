# Project Status

## Date

- 2026-02-14

## Current Phase

- Stage 2 complete: Core P2P MVP with KB-first contract over 25 core protocol messages.
- Stage 2R complete: runtime capture refresh and confidence promotion (`medium -> high`) for all 25 core messages.
- Stage 3A complete: authenticated login against official server (`160/1`) and semantic differential verification as default.
- Stage 3B complete: Rooms+Presence protocol batch with runtime evidence and CLI support.
- Stage 4A complete on this branch: Recommendations/Discovery batch with runtime-authenticated evidence and CLI support.
- Product direction remains unchanged: SDK+CLI first, custom evolvable app (not a 1:1 official client clone).

## Stage 4A Completion

1. Added runtime-authenticated S4A scenarios:
   - `login-recommendations`
   - `login-user-recommendations`
   - `login-similar-terms`
2. Expanded protocol mapping from 33 to 38 total rows (`+5` discovery messages):
   - `SM_GET_SIMILAR_TERMS`
   - `SM_GET_RECOMMENDATIONS`
   - `SM_GET_MY_RECOMMENDATIONS`
   - `SM_GET_GLOBAL_RECOMMENDATIONS`
   - `SM_GET_USER_RECOMMENDATIONS`
3. Implemented typed discovery payload handling in `rust/protocol` for request/response flows.
4. Extended `SessionClient` discovery operations in `rust/core`:
   - `get_recommendations(...)`
   - `get_my_recommendations(...)`
   - `get_global_recommendations(...)`
   - `get_user_recommendations(...)`
   - `get_similar_terms(...)`
5. Added `soul-cli discover` command group in `rust/cli`:
   - `discover recommendations`
   - `discover mine`
   - `discover global`
   - `discover user --target-user <name>`
   - `discover similar-terms --term <term>`
6. Extended semantic verification coverage and required capture runs for S4A.
7. Validation gates are green (`kb_validate`, `run_diff_verify`, `run_regression`).

## Stage 3B Completion (Reference)

1. Published visual roadmap for Zensical (`docs/state/roadmap.md`) with timeline and dependency graph.
2. Added 8-message rooms/presence protocol batch with runtime evidence.
3. Implemented room operations and room command group (`soul-cli room ...`).
4. Added mandatory S3B runtime redacted runs.

## Core Artifacts

- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`
- `docs/re/static/message-schema.md`
- `docs/verification/evidence-ledger.md`
- `docs/state/roadmap.md`
- `captures/redacted/login-recommendations/manifest.redacted.json`
- `captures/redacted/login-user-recommendations/manifest.redacted.json`
- `captures/redacted/login-similar-terms/manifest.redacted.json`
- `tools/runtime/generate_stage4a_discovery_captures.py`

## Operational Notes

- Stage 2 core contract remains intact (`25/25`, `high=25`, `medium=0`, `low=0`).
- Total mapped protocol rows now: `38`.
- Runtime credentials remain local-only in `.env.local` and are never committed.
