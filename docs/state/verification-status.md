# Verification Status

## Objective

Ensure evidence traceability and semantic protocol parity baseline while preserving Stage 4L full-coverage closure, Stage 5A typed runtime hardening wave 1, Stage 5B UI/feature research verification, and Stage 5C typed runtime hardening wave 2.

## Validation Gates

### KB validation

```bash
python3 scripts/kb_validate.py
```

Checks:

- Name/data maps contain valid evidence.
- `message_map.csv` has valid source links and confidence fields.
- `message_schema.json` has valid evidence links and schema integrity.

### Differential verification

```bash
scripts/run_diff_verify.sh
```

Runs:

1. Fixture parity (`captures/fixtures/*`).
2. Runtime redacted capture parity for mandatory scenarios:
   - `login-only`
   - `login-search`
   - `login-search-download`
   - `upload-deny`
   - `upload-accept`
   - `login-room-list`
   - `login-join-room-presence`
   - `login-leave-room`
   - `login-recommendations`
   - `login-user-recommendations`
   - `login-similar-terms`
   - `login-room-moderation`
   - `peer-advanced-local`
   - `login-privileges-social`
   - `peer-folder-local`
   - `login-privilege-messaging`
   - `peer-legacy-local`
   - `login-private-message`
   - `login-user-state`
   - `login-peer-address-connect`
   - `login-message-users`
   - `login-peer-message`
   - `login-parent-distributed-control`
   - `login-room-term-control`
3. Default mode is semantic (`VERIFY_MODE=semantic`) with bytes mode compatibility.

### Full regression

```bash
scripts/run_regression.sh
```

Includes:

1. Python unit tests (`tests/kb`, `tests/protocol`, `tests/runtime`).
2. Rust unit/integration tests (`cargo test`).
3. KB validation gate.
4. Differential verification gate.
5. Zensical build check (if available).

## Stage 4L Coverage Status

S4L closure set is present in:

- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`

Messages:

- all prior `mapped_not_implemented` rows (`40`) were promoted to `implemented_mapped` via protocol constants plus decode/encode support.

Confidence distribution for the S4L closure set:

- `high=40`
- `medium=0`
- `low=0`

Protocol matrix status:

- Tracked message names from static string tables: `131`
- Implemented + mapped: `131`
- Mapped not implemented: `0`
- Missing: `0`
- Matrix source: `docs/state/protocol-matrix.md`

## Runtime Evidence Snapshot

- Official server: `server.slsknet.org:2242`
- Auth tuple used: `160/1`
- S4E runtime redacted runs:
  - `captures/redacted/login-private-message`
  - `captures/redacted/login-user-state`
  - `captures/redacted/login-peer-address-connect`
  - `captures/redacted/login-message-users`
  - `captures/redacted/login-peer-message`
- Private messaging runtime scenarios include code `22` and `23` paths with directional payload decoding.
- User-state runtime scenario includes code `7` and `36` request/response payloads.
- Peer-address/connect scenario includes code `3` and `18` request/response payloads.
- Message-users scenario includes code `149`.
- Peer-message deterministic scenario includes code `68` plus compatibility alias `292`.
- Stage 5A authenticated runtime scenario `login-parent-distributed-control` provides runtime evidence for codes `83`, `84`, `113`, `121`, and `122`.
- Stage 5C authenticated runtime scenario `login-room-term-control` provides runtime evidence for codes `10`, `51`, and `52`.
- `SM_GET_USER_PRIVILEGES_STATUS` was promoted to `high` with authenticated request/response runtime evidence.
- `PM_SHARED_FILES_IN_FOLDER` parser now has decompression-aware coverage with zlib safety limits and typed listing classification.
- Stage 4F/S4G/S4H/S4I/S4J are mapping-first and static-evidence-driven via jump-table extraction (`evidence/reverse/message_codes_jump_table.md`).
- Stage 4K closes all prior `missing` names by adding jump-table-backed mappings and protocol codec support.
- Stage 4L closes the prior `mapped_not_implemented` bucket using `OpaqueServerControlPayload` coverage for unresolved runtime-shape control messages.

## Stage 5B Verification Status

- Feature inventory artifact: `docs/state/soulseek-feature-inventory.md`
- Baseline evidence bundle: `evidence/ui_audit/*`
- Structured external-source snapshots:
  - `evidence/ui_audit/external/changelog_structured.json`
  - `evidence/ui_audit/external/news_structured.json`
  - `evidence/ui_audit/external/forum_topics_structured.json`
- Static handler/protocol bridge evidence:
  - `evidence/reverse/ui_handler_symbols_nm.txt`
  - `evidence/ui_audit/decomp/mainwindow_methods.txt`
  - `evidence/ui_audit/decomp/server_methods.txt`
  - `evidence/ui_audit/decomp/peer_methods.txt`
  - `evidence/ui_audit/decomp/transfer_methods.txt`
- Pass-2 closure:
  - `verified_pass2=41`
  - `gap_found=1`

## Residual Risk

- `SM_PEER_MESSAGE` compatibility alias `292` is implemented as decode-only fallback and still needs corroboration from authenticated server runtime.
- Stage 5A and Stage 5C hardened key opaque-control subsets (`83`, `84`, `113`, `10`, `51`, `52`), but additional global/distributed control codes still rely on opaque payload handling until runtime evidence allows typed promotion.
- Stage 5B still has one UI-runtime visibility gap: live menu tree extraction requires macOS assistive-access permission (captured denial: `evidence/ui_audit/ui_menu_bar_items.err`).
