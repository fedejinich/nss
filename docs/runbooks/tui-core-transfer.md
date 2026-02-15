# TUI Core Transfer Runbook

This runbook covers the simplified NeoSoulSeek terminal UI for core transfer workflows.

## Scope

The TUI is intentionally minimal and focuses on:

1. Mandatory login modal at startup.
2. Search.
3. Select a result.
4. Download selected file through core orchestration.
5. Monitor persisted download history.
6. Toggle downloads panel visibility.
7. Clear download history (history only, files remain on disk).
8. Persist credentials, query, and download history between restarts.

## Prerequisites

1. Valid credentials.
2. Built Rust workspace.

Recommended setup:

```bash
./scripts/setup_credentials_wizard.sh
set -a; source .env.local; set +a
```

## Run

```bash
cd rust
cargo run -p soul-tui
```

## Key Bindings

### Login modal

- `Tab`/`Shift+Tab`: move focus across server/username/password.
- `Enter`: login.
- `Esc`: clear login error.
- `q`: quit.

### Main view

- `/`: enter query edit mode.
- `Enter`: submit search.
- `Esc` (while editing query): cancel query edit.
- `up`/`down`: move selected search result.
- `d`: run `search_select_and_download(...)` for selected result.
- `t`: show/hide downloads panel.
- `c`: clear persisted download history (files on disk are kept).
- `l`: return to login modal (logout).
- `q`: quit.

## Notes

1. The TUI uses `soul-core` APIs and does not implement protocol encoding directly.
2. Login is mandatory; search/download actions are blocked until login succeeds.
3. The download path uses `SessionClient::search_select_and_download(...)`.
4. Search uses `SessionClient::search_and_collect(...)`.
5. Persisted state location:
   - macOS Application Support via `directories::ProjectDirs`
   - file name: `tui-state-v1.json`
   - in-progress entries are converted to `interrupted` on startup
6. Optional env defaults:
   - `NSS_TUI_QUERY`
   - `NSS_TUI_OUTPUT_DIR`
