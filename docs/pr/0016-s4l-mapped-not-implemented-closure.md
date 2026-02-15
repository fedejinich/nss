# PR 0016 - S4L: mapped-not-implemented closure to full protocol coverage

## Branch

- `codex/s4l-mapped-not-implemented-closure`

## Objective

Close Stage 4L by promoting every remaining `mapped_not_implemented` row to implementation coverage and reach full protocol matrix closure.

## Scope

1. Add constants for all remaining mapped-only server messages (`40`).
2. Add maintainable decode/encode coverage with `OpaqueServerControlPayload { code, bytes }`.
3. Add regression tests for closure-set roundtrip behavior and builder validation.
4. Regenerate authoritative schema/docs/matrix and sync state artifacts.
5. Update workflow governance to disable `@codex review` dependency and use local review passes.

Closure outcome target:

- `implemented+mapped=131`
- `mapped_not_implemented=0`
- `implemented_not_mapped=0`
- `missing=0`

## Outcome

1. `rust/protocol` now includes constants for all tracked protocol names from authoritative maps.
2. Added `is_opaque_server_control_code(...)`, `OPAQUE_SERVER_CONTROL_CODES`, and `build_opaque_server_control_request(...)`.
3. Added protocol tests:
   - `s4l_opaque_server_control_codes_roundtrip`
   - `opaque_server_control_builder_rejects_unknown_code`
4. Matrix reached full closure baseline:
   - tracked: `131`
   - implemented+mapped: `131`
   - mapped-not-implemented: `0`
   - implemented-not-mapped: `0`
   - missing: `0`

## Validation

```bash
cargo test -p protocol
python3 scripts/kb_validate.py
scripts/run_diff_verify.sh
scripts/run_regression.sh
./.venv-tools/bin/zensical build -f zensical.toml
```

## Local Review Loops (No `@codex review`)

1. Round 1 security pass:
   - checked touched protocol code for unsafe parsing assumptions, panic paths in production flow, and payload validation boundaries.
2. Round 1 code-simplifier pass:
   - reduced duplication by extracting `OPAQUE_SERVER_CONTROL_CODES` used by both production checks and tests.
3. Round 2 local review:
   - verified no semantic drift after simplification and re-ran protocol tests.

## Maintainability Notes

1. This stage intentionally uses opaque payload handling for control families lacking runtime-verified payload schemas.
2. Next stage should prioritize replacing opaque payloads with typed structures using runtime captures while preserving current matrix closure.
