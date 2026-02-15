# PR 0028 - S9A TUI-First Simplification and Persistence Hardening

## Summary

This stage executes S9A in the locked order:

1. TUI-first simplification and hardening now.
2. SwiftUI macOS GUI MVP next (S9B).
3. Next.js web GUI MVP after Swift (S9C).

S9A focuses on a minimal, operator-friendly terminal surface with mandatory login and persistent local state.

## Scope

In scope for this PR:

1. Refactor `soul-tui` into focused modules (`app/ui/state/storage`).
2. Enforce login-first workflow (`LoginModal` gate) before search/download.
3. Simplify layout and apply retro orange palette.
4. Add downloads panel show/hide and clear-history semantics.
5. Persist credentials, UI preferences, and downloads history locally.
6. Convert persisted `in_progress` downloads to `interrupted` on startup.
7. Update docs/state artifacts to reflect S9A -> S9B -> S9C execution plan.

Out of scope in this PR:

1. SwiftUI GUI implementation.
2. Next.js GUI implementation.
3. Figma-driven GUI artifacts (pending S9B/S9C kickoff inputs).

## Key Changes

### TUI code

1. Added state model:
   - `rust/tui/src/state.rs`
2. Added persistence service:
   - `rust/tui/src/storage.rs`
3. Added app state machine and action handling:
   - `rust/tui/src/app.rs`
4. Added rendering/event loop module:
   - `rust/tui/src/ui.rs`
5. Reduced bootstrap entrypoint:
   - `rust/tui/src/main.rs`
6. Added dependencies:
   - `serde`, `serde_json`, `directories` (`rust/tui/Cargo.toml`)

### Behavior changes

1. Mandatory login modal is shown first.
2. Main screen is blocked until login success.
3. Startup auto-login attempts persisted credentials when present.
4. Main layout:
   - results in main body,
   - query control at bottom,
   - optional downloads side panel.
5. Downloads panel controls:
   - `t` toggles visibility,
   - `c` clears history only (files on disk stay untouched).
6. Persisted startup recovery:
   - any `in_progress` entry is moved to `interrupted`.

### Documentation and state governance

1. Updated runbook:
   - `docs/runbooks/tui-core-transfer.md`
2. Updated quick-start README:
   - `README.md`
3. Added optional TUI defaults to:
   - `.env.example`
4. Updated stage/capability planning artifacts:
   - `analysis/state/stage_registry.json`
   - `analysis/state/capability_registry.json`
   - `docs/state/roadmap.md`
   - `docs/state/project-status.md`
   - `docs/state/verification-status.md`
   - `TODO-CODEX.md`
5. Dashboard route-safety improvement for evidence links:
   - `docs/state/project-dashboard.html`

## Validation

Commands run for S9A:

```bash
cd rust
cargo test -p soul-tui
cd ..
scripts/sync_state_dashboards.sh
python3 scripts/kb_validate.py
scripts/run_regression.sh
./.venv-tools/bin/zensical build -f zensical.toml
```

## Review Loop Notes

Round 1:

1. `blockchain_protocol_engineer`
2. `code_simplifier`
3. `web3_security_review_expert`

Actions and outcome:

1. Protocol/flow review confirmed no protocol-surface drift in this stage (TUI remains on top of `soul-core`).
2. Simplification review confirmed module split is materially clearer than prior single-file implementation.
3. Security review confirmed:
   - no plaintext password emission in logs,
   - persisted state uses restrictive permissions (`0600` target),
   - clear-history action only mutates local history state.

Round 2:

1. `blockchain_protocol_engineer`
2. `code_simplifier`
3. `web3_security_review_expert`

Actions and outcome:

1. Re-checked after dashboard/state sync and gate reruns.
2. No additional actionable findings were raised.

## Retrospective

### Maintainability

1. Splitting TUI into four focused modules reduces coupling and localizes concerns.
2. Persistence/recovery logic is isolated in `storage.rs`/`state.rs`.
3. UI rendering code no longer mixes protocol/session orchestration details.

### Reuse and no double writing

1. TUI operations continue to use `soul-core` APIs directly, avoiding protocol duplication.
2. Stage/capability dashboards remain generated from canonical registries.
3. Existing runbooks and registry generators were reused instead of introducing parallel artifacts.

### Surface reduction

1. Login-first modal removes invalid pre-auth actions from normal operator paths.
2. Downloads clear action only affects history state, avoiding accidental data loss.
3. Route-safe dashboard links reduce broken navigation paths.
