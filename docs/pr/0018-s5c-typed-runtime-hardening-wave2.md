# PR 0018 - S5C: typed runtime hardening wave 2 (room/term control)

## Branch

- `codex/s5c-typed-runtime-hardening-wave2`

## Objective

Close S5C by replacing selected opaque server-control handling with typed payloads backed by authenticated runtime evidence, while preserving full protocol coverage.

## Scope

1. Promote runtime evidence for:
   - `SM_ADD_CHATROOM` (`10`)
   - `SM_ADD_LIKE_TERM` (`51`)
   - `SM_REMOVE_LIKE_TERM` (`52`)
2. Add runtime capture scenario:
   - `login-room-term-control`
3. Extend protocol typing and builders for the selected S5C control messages.
4. Extend core/CLI operations for room/term control.
5. Extend contract and semantic verifier coverage.
6. Regenerate schema/matrix/docs and sync KB/state artifacts.

## Outcome

1. `rust/protocol` now includes typed handling for S5C controls:
   - `AddChatRoomPayload`
   - `ServerMessage::AddLikeTerm`
   - `ServerMessage::RemoveLikeTerm`
2. `OPAQUE_SERVER_CONTROL_CODES` excludes S5C codes (`10`, `51`, `52`).
3. Added protocol builders:
   - `build_add_chatroom_request(...)`
   - `build_add_like_term_request(...)`
   - `build_remove_like_term_request(...)`
4. Added core APIs:
   - `SessionClient::add_chatroom(...)`
   - `SessionClient::add_like_term(...)`
   - `SessionClient::remove_like_term(...)`
5. Added CLI commands:
   - `soul-cli room add --room ...`
   - `soul-cli discover add-like-term --term ...`
   - `soul-cli discover remove-like-term --term ...`
6. Added runtime evidence artifacts:
   - `tools/runtime/generate_stage5c_room_term_control_captures.py`
   - `captures/redacted/login-room-term-control/*`
7. Added/updated coverage:
   - `tests/protocol/test_stage5c_room_term_control_contract.py`
   - core operation test for S5C codes in `rust/core`
   - semantic verifier regression for like-term diff path in `rust/verify`
8. Updated required capture verification runs with:
   - `login-room-term-control`

## Validation

```bash
python3 scripts/kb_validate.py
scripts/run_diff_verify.sh
scripts/run_regression.sh
./.venv-tools/bin/zensical build -f zensical.toml
```

Observed result:

- All commands completed successfully.
- Semantic capture verification now includes `login-room-term-control`.

## Local Review Loops (No `@codex review`)

1. Round 1 security pass:
   - reviewed new runtime capture tooling and ensured redaction-first commit flow is preserved.
   - verified no plaintext credentials are persisted in committed capture artifacts.
2. Round 1 code-simplifier pass:
   - reviewed new typed-control paths to avoid duplicate encode/decode branches.
   - kept payload reuse via `SimilarTermsRequestPayload` for term-control messages.
3. Round 2 security + simplifier pass:
   - re-reviewed touched protocol/core/cli/verify paths after regression fixes.
   - confirmed no behavior drift with full regression and semantic diff gates.

## Retrospective

1. More maintainable approach:
   - promoted only runtime-backed opaque controls to typed variants; unresolved families remain explicitly opaque.
2. Reuse to avoid double-writing:
   - reused existing runtime capture/redaction pipeline and schema/matrix generation scripts.
3. Surface reduction:
   - removed S5C codes from opaque control handling instead of adding parallel fallback paths.
