# TUI Core Transfer Runbook

This runbook covers the minimal NeoSoulSeek terminal UI for core transfer workflows.

## Scope

The TUI is intentionally minimal and focuses on:

1. Login.
2. Search.
3. Select a result.
4. Download selected file through core orchestration.
5. Monitor transfer outcomes.
6. Toggle upload decision mode (accept or deny).

## Prerequisites

1. Valid local credentials in `.env.local` or shell environment:
   - `NSS_TEST_SERVER`
   - `NSS_TEST_USERNAME`
   - `NSS_TEST_PASSWORD`
2. Built Rust workspace.

## Run

```bash
cd rust
cargo run -p soul-tui
```

## Key Bindings

- `l`: connect and login.
- `s`: execute search (`query` value in app state).
- `up`/`down`: move selected search result.
- `d`: run `search_select_and_download(...)` for selected result.
- `a`: set upload decision to `accept`.
- `x`: set upload decision to `deny`.
- `q`: quit.

## Notes

1. The TUI uses `soul-core` APIs and does not implement protocol encoding directly.
2. The download path uses `SessionClient::search_select_and_download(...)`.
3. Search uses `SessionClient::search_and_collect(...)` to keep result browsing responsive.
