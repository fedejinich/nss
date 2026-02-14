# NeoSoulSeek Rust Workspace

## Crates

- `protocol`: framing and message codecs.
- `core`: network client helpers (login/search/download path).
- `cli`: operator CLI for message build/send and verification commands.
- `verify`: fixture-based differential checks.

## Commands

```bash
cd rust
cargo test
cargo run -p soul-cli -- verify-fixtures --fixtures-dir ../captures/fixtures --report ../captures/fixtures/verify-report.json
```

## Scope Notes

- This V1 implements protocol and transport slices needed for login/search/single-file download experimentation.
- Runtime parity against official client is validated through fixtures and golden-capture workflow in project root runbooks.
