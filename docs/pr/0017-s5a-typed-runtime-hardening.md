# PR 0017 - S5A: typed runtime hardening for opaque control payloads

## Branch

- `codex/s5a-typed-runtime-hardening`

## Objective

Close S5A by converting selected opaque/runtime-hypothesis message handling into typed payload coverage with authenticated runtime evidence, while keeping full protocol matrix closure.

## Scope

1. Promote runtime evidence for:
   - `SM_SET_PARENT_MIN_SPEED` (`83`)
   - `SM_SET_PARENT_SPEED_CONNECTION_RATIO` (`84`)
   - `SM_GET_ROOM_TICKER` (`113`)
   - `SM_UPLOAD_SPEED` (`121`)
   - `SM_GET_USER_PRIVILEGES_STATUS` (`122`)
2. Add runtime capture scenario:
   - `login-parent-distributed-control`
3. Extend protocol typing:
   - parent tuning payloads (`83`, `84`)
   - room ticker request/response multiplex (`113`)
4. Add decompression-aware parser for `PM_SHARED_FILES_IN_FOLDER` with safety bounds.
5. Extend core/CLI/verify surfaces and tests for new typed behavior.
6. Regenerate schema/matrix/docs and sync KB artifacts.

## Outcome

1. `rust/protocol` now includes typed variants for `83`, `84`, and `113`:
   - `ParentMinSpeedPayload`
   - `ParentSpeedConnectionRatioPayload`
   - `RoomTickerRequestPayload`
   - `RoomTickerPayload`
2. `OPAQUE_SERVER_CONTROL_CODES` excludes typed S5A control codes (`83`, `84`, `113`).
3. Added builder/API coverage:
   - `build_get_room_ticker_request(...)`
   - `build_upload_speed_request(...)`
4. Added decompression-aware parser for `PM_SHARED_FILES_IN_FOLDER`:
   - zlib decompression
   - hard safety limit (`16 MiB`)
   - format classification (`BinaryEntries`, `Utf8Lines`, `OpaqueBytes`)
5. `rust/core` additions:
   - `SessionClient::request_room_ticker(...)`
   - `SessionClient::set_upload_speed(...)`
   - `RoomEvent::TickerSnapshot(...)` integration in event collection
6. `rust/cli` additions:
   - `session upload-speed --bytes-per-sec ...`
   - `room ticker --room ... --timeout-secs ...`
   - room watch summary includes ticker snapshot counts
7. Runtime evidence/KB closure:
   - `captures/redacted/login-parent-distributed-control`
   - `SM_GET_USER_PRIVILEGES_STATUS` promoted to `high`
   - all protocol rows now `high=131`, `medium=0`, `low=0`

## Validation

```bash
cargo test --quiet
python3 scripts/kb_validate.py
scripts/run_diff_verify.sh
scripts/run_regression.sh
./.venv-tools/bin/zensical build -f zensical.toml
```

Observed result:

- All commands completed successfully.
- Semantic capture verification includes the new run `login-parent-distributed-control`.

## Local Review Loops (No `@codex review`)

1. Round 1 security pass:
   - reviewed new decompression parser and added explicit decompression-size bound enforcement.
   - added regression test for oversized decompressed payload rejection.
2. Round 1 code-simplifier pass:
   - reduced duplicated construction logic in decompression-aware parser by centralizing payload assembly.
3. Round 2 security + simplifier pass:
   - re-reviewed touched protocol/core/cli/verify paths after simplification.
   - confirmed no behavior drift with full Rust and regression gates.

## Retrospective

1. More maintainable approach:
   - payload typing was introduced only where runtime evidence exists, keeping unknown families explicit and isolated.
2. Reuse to avoid double-writing:
   - reused existing runtime capture/redaction pipeline and schema/doc generation scripts.
3. Surface reduction:
   - removed selected codes from opaque-control dispatch instead of adding parallel decode paths.
