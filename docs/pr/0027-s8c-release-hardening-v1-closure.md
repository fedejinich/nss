# PR 0027 - S8C Release Hardening and v1 Closure Gates

## Summary

This stage executes release hardening for the minimal TUI baseline and drives the project to the final v1 closure gate.

Current stage state:

- `S8C` status: `done`
- title: `Release hardening and v1 closure gates`
- next gate: `post-v1 expansion roadmap`

## Scope

1. Harden redacted capture metadata to avoid absolute-path leakage in committed artifacts.
2. Publish reproducible packaging workflow for CLI + TUI artifacts.
3. Publish operational recovery runbook for core auth/search/download/TUI failures.
4. Publish closure checklist and executable release hardening audit.
5. Keep dashboard, capability matrix, and stage registry synchronized with current execution state.

## Implemented

1. Redaction metadata hardening:
   - `tools/runtime/redact_capture_run.py`
   - `tools/runtime/sanitize_redacted_metadata.py`
   - sanitized committed `captures/redacted/*` metadata fields.
2. Packaging workflow:
   - `scripts/package_release.sh`
   - `docs/runbooks/release-packaging.md`
3. Recovery runbook:
   - `docs/runbooks/failure-recovery.md`
4. Closure and audit:
   - `docs/state/final-closure-checklist.md`
   - `tools/state/verify_release_hardening.py`
   - `docs/state/release-hardening-audit.json`
   - `docs/state/release-hardening-audit.md`
   - `tests/state/test_release_hardening_audit.py`
5. TUI minimal usability increment in this stage:
   - runtime query edit flow (`/`, `Enter`, `Esc`, `Backspace`) in `rust/tui/src/main.rs`.

## Review/Workflow Rules Applied

1. Branch started from updated `main` under `codex/` prefix.
2. Capability-first planning synchronized in:
   - `analysis/state/stage_registry.json`
   - `analysis/state/capability_registry.json`
   - `TODO-CODEX.md`
3. Canonical workflow and mandatory review-loop governance updated in:
   - `AGENTS.md`

## Validation

Executed during this stage:

```bash
bash scripts/sync_state_dashboards.sh
bash scripts/run_regression.sh
```

Result:

1. Regression suite green (`44` Python tests, Rust tests, semantic diff runs).
2. State dashboards regenerated and consistent with `S8C done`.
3. Release-hardening audit supports stage-aware behavior:
   - strict closure when `S8C=done`
   - operational pass mode when `S8C=in_progress`

## Closure Status

1. `CAP-RELEASE-HARDENING` is promoted to `done`.
2. Final checklist is fully checked.
3. Strict release-hardening closure audit is green with `S8C=done`.
4. Stage is merge-ready and mapped to post-v1 expansion roadmap.

## Retrospective (Maintainability and Reuse)

1. Better maintainability:
   - stage-aware auditing removed false-negative gate failures during active execution.
2. Reuse over duplication:
   - existing `sync_state_dashboards.sh` pipeline remains the single regeneration path.
3. Surface reduction:
   - one closure audit script and one checklist are reused by dashboard, tests, and stage docs.
