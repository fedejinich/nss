# PR 0002 - Rust protocol workspace + differential verification

## Branch

- `codex/kb-first-zensical`

## Commits

- `b194aef` feat: add rust protocol workspace and differential verifier

## Scope

- Rust workspace with crates: `protocol`, `core`, `cli`, `verify`.
- Framing/codec implementation and typed builders for login/search/transfer.
- Core client operations for login/search and single-file download path.
- Fixture-based differential verification and JSON report output.
- Unified regression runner and state/TODO closure docs.

## Key Files

- `rust/protocol/src/lib.rs`
- `rust/core/src/lib.rs`
- `rust/cli/src/main.rs`
- `rust/verify/src/lib.rs`
- `captures/fixtures/*.hex`
- `scripts/run_diff_verify.sh`
- `scripts/run_regression.sh`
- `TODO-CODEX.md`

## Validation

- `cd rust && cargo test`
- `scripts/run_diff_verify.sh`
- `scripts/run_regression.sh`
