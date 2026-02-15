# PR 0029 - S9A Hotfix: Explicit Login Close Error and TUI Diagnostics Wizard

## Summary

This hotfix improves login troubleshooting for real operators:

1. Login now returns a typed, explicit error when the server closes before sending a login response frame.
2. The TUI now includes an in-app diagnostics wizard (`g`) to check server formatting, DNS/TCP connectivity, and login probe attempts.

## Scope

1. `rust/core` auth error handling improvement for close-before-response path.
2. `rust/tui` diagnostics wizard action and modal.
3. Regression tests for both behaviors.
4. Runbook/README and status docs updates.

## Changes

### Core auth error clarity

File:

- `rust/core/src/lib.rs`

Changes:

1. Added `AuthError::ServerClosedBeforeLoginResponse`.
2. `SessionClient::login(...)` now maps EOF/connection-reset before frame read to this typed error.
3. Added test:
   - `login_server_close_before_response_returns_typed_error`

Resulting user-facing error text:

- `server closed before login response (possible invalid account/registration/ban)`

### TUI diagnostics wizard

Files:

- `rust/tui/src/app.rs`
- `rust/tui/src/ui.rs`

Changes:

1. Added action:
   - `PendingAction::RunDiagnostics`
2. Added diagnostics state:
   - `diagnostics_visible`
   - `diagnostics_lines`
3. Added keybindings:
   - `g` open diagnostics wizard from login/main
   - `r` rerun diagnostics inside modal
   - `Esc`/`g` close diagnostics modal
4. Diagnostics checks:
   - server string parse (`host:port`)
   - DNS lookup
   - TCP connect
   - login version probe matrix (`probe_login_versions`)
5. Added TUI tests:
   - diagnostics open key behavior
   - diagnostics close behavior
   - server parser validation

### Documentation

Files:

- `README.md`
- `docs/runbooks/tui-core-transfer.md`
- `TODO-CODEX.md`
- `docs/state/project-status.md`
- `docs/state/verification-status.md`

## Validation

```bash
cd rust
cargo test -p soul-core login_server_close_before_response_returns_typed_error
cargo test -p soul-tui

cargo run -q -p soul-cli -- session probe-login-version \
  --server server.slsknet.org:2242 \
  --username doesnotexist_cx_12345 \
  --password badpass
```

Expected probe output now reports:

- `server closed before login response (possible invalid account/registration/ban)`
