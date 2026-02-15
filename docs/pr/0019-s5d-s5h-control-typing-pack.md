# PR 0019 - S5D-S5H: multi-wave control typing pack (runtime-backed)

## Branch

- `codex/s5d-s5h-control-typing-pack`

## Objective

Execute five consecutive typed-hardening waves (S5D, S5E, S5F, S5G, S5H) to replace opaque server-control payload handling with typed payloads backed by authenticated runtime evidence, while preserving full matrix closure (`implemented+mapped=131`).

## Scope

1. S5D global/discovery control typing:
   - `SM_JOIN_GLOBAL_ROOM` (`150`)
   - `SM_LEAVE_GLOBAL_ROOM` (`151`)
   - `SM_SAY_GLOBAL_ROOM` (`152`)
   - `SM_SEARCH_CORRELATIONS` (`153`)
2. S5E parent/disconnect control typing:
   - `SM_SET_PARENT_INACTIVITY_BEFORE_DISCONNECT` (`86`)
   - `SM_SET_SERVER_INACTIVITY_BEFORE_DISCONNECT` (`87`)
   - `SM_NODES_IN_CACHE_BEFORE_DISCONNECT` (`88`)
   - `SM_SET_SECONDS_BEFORE_PING_CHILDREN` (`90`)
   - `SM_CAN_PARENT` (`100`)
3. S5F private-room membership/ownership control typing:
   - `SM_REMOVE_OWN_ROOM_MEMBERSHIP` (`136`)
   - `SM_GIVE_UP_ROOM` (`137`)
   - `SM_ADD_ROOM_MEMBERSHIP` (`139`)
   - `SM_REMOVE_ROOM_MEMBERSHIP` (`140`)
   - `SM_ADD_ROOM_OPERATORSHIP` (`145`)
4. S5G text-control typing:
   - `SM_COMMAND` (`58`)
   - `SM_ROOM_ADDED` (`62`)
   - `SM_ROOM_REMOVED` (`63`)
   - `SM_ADMIN_MESSAGE` (`66`)
   - `SM_ADD_HATE_TERM` (`117`)
   - `SM_REMOVE_HATE_TERM` (`118`)
5. S5H system-control typing:
   - `SM_SET_STATUS` (`28`)
   - `SM_HEARTBEAT` (`32`)
   - `SM_DNET_RESET` (`130`)

## Outcome

1. `rust/protocol` now decodes/encodes these 23 control families as typed payloads instead of generic opaque control bytes.
2. Added typed builders for all newly promoted S5D-S5H messages.
3. Reduced `OPAQUE_SERVER_CONTROL_CODES` generic closure set from `34` to `15`.
4. Added runtime capture generators and committed redacted artifacts:
   - `tools/runtime/generate_stage5d_global_control_captures.py`
   - `tools/runtime/generate_stage5e_parent_disconnect_control_captures.py`
   - `tools/runtime/generate_stage5f_private_room_membership_control_captures.py`
   - `tools/runtime/generate_stage5g_text_control_captures.py`
   - `tools/runtime/generate_stage5h_system_control_captures.py`
   - `captures/redacted/login-global-room-control/*`
   - `captures/redacted/login-parent-disconnect-control/*`
   - `captures/redacted/login-private-room-membership-control/*`
   - `captures/redacted/login-text-control/*`
   - `captures/redacted/login-system-control/*`
5. Added protocol contract tests:
   - `tests/protocol/test_stage5d_s5h_typed_control_contract.py`
   - protocol unit additions in `rust/protocol/src/lib.rs` for builder-code coverage and typed decode coverage.
6. Updated required semantic capture verification runs with all five new scenarios.
7. Regenerated authoritative KB artifacts and matrix/schema docs from the updated maps and typed payload definitions.

## Validation

```bash
python3 scripts/kb_validate.py
scripts/run_diff_verify.sh
scripts/run_regression.sh
./.venv-tools/bin/zensical build -f zensical.toml
```

Observed result:

- All commands completed successfully.
- `scripts/run_diff_verify.sh` now covers 29 required runtime scenarios including all S5D-S5H runs.
- Protocol matrix remains fully closed:
  - `implemented+mapped=131`
  - `mapped_not_implemented=0`
  - `missing=0`

## Local Review Loops (No `@codex review`)

1. Round 1 security pass:
   - reviewed runtime generators and redaction pipeline usage.
   - ensured all committed capture artifacts are redacted and no plaintext credentials are persisted.
   - verified typed decode paths keep safe bounds and explicit bool/u32 parsing behavior.
2. Round 1 code-simplifier pass:
   - consolidated repeated payload-shape handling by introducing shared typed payload structs and builders.
   - removed unnecessary opaque-control dependence for promoted codes.
3. Round 2 security + simplifier pass:
   - re-reviewed changed protocol, schema, map, and run wiring after full-regression pass.
   - confirmed no drift in semantic parity checks and no regression in existing staged contracts.

## Retrospective

1. More maintainable approach:
   - promoted entire coherent control families (by domain) rather than isolated codes, reducing mixed typed/opaque drift.
2. Reuse to avoid double-writing:
   - reused the existing runtime capture + redaction + diff verification pipeline for all five waves.
   - reused existing payload struct patterns (`room`, `term`, `u32 control`) across families.
3. Surface reduction:
   - shrank generic opaque closure handling (`34 -> 15`) and moved promoted codes into explicit typed variants/builders.
