# PR 0003 - S3A: authenticated login handshake + semantic diff verifier

## Branch

- `codex/s3a-auth-login-semantic-parity`

## Objective

Close Stage 3A with:

1. Auth+Search real working against official server.
2. Stateful login transition (`LoggedIn` only after valid success response).
3. Differential verification with semantic mode (bytes mode still available).
4. KB-first docs/maps/status fully refreshed.

## Atomic Commits

1. `3002da8` protocol: add typed auth codec and code64 summary parsing
2. `cb1e632` core: enforce stateful auth handshake and typed auth errors
3. `aec8a64` cli/tools: add env credential flow and runtime login probing
4. `faff073` verify/scripts: add semantic diff mode with bytes compatibility
5. `71a36d1` captures/kb/docs: authenticated runtime captures + maps/schema/docs refresh
6. `<this commit>` retrospective: maintainability review and cleanup

## Scope

- `rust/protocol`:
  - Login request now encodes `username,password,client_version,md5hash,minor_version`.
  - Added typed login response decode and failure reasons.
  - Added helper `compute_login_md5hash`.
  - Added code 64 summary parser with fallback for observed runtime room-list shape.
- `rust/core`:
  - Added `AuthError` and stateful auth handshake.
  - `SessionClient::login` now waits for `code=1` response and transitions state only on success.
  - Added login version probing helper with ordered tuple matrix.
- `rust/cli`:
  - Runtime auth uses `--password`; `--password-md5` now explicit deprecation error.
  - Added `.env.local`-backed runtime credential resolution.
  - Added `session probe-login-version`.
  - Added `verify captures --mode bytes|semantic` (default semantic).
- `rust/verify`:
  - Added `ComparisonMode`.
  - Added semantic normalization for known messages and raw payload hash fallback for unknowns.
  - Reports now include `bytes_match`, `semantic_matches`, and `semantic_first_diff_field`.
- Runtime tooling:
  - Added `tools/runtime/probe_login_versions.py`.
  - Added `tools/runtime/provision_test_credentials.py`.
  - Added shared helper `tools/runtime/slsk_runtime.py`.
  - Updated stage2 runtime capture generator for plain-password login format + accepted tuple defaults.
  - Redaction updated to sanitize password/hash fields.

## Validation

```bash
python3 -m py_compile tools/runtime/slsk_runtime.py tools/runtime/probe_login_versions.py tools/runtime/provision_test_credentials.py tools/runtime/generate_stage2_real_captures.py
cd rust && cargo test
scripts/run_diff_verify.sh
scripts/run_regression.sh
```

## Runtime Evidence Snapshot

- Official server login tuple accepted: `160/1`.
- Real login command result: `session.login ok state=LoggedIn`.
- Real search command result: `session.search ... collected_server_messages=1`.
- Redacted runtime scenarios refreshed:
  - `login-only`
  - `login-search`
  - `login-search-download`
  - `upload-deny`
  - `upload-accept`

## Retrospective

### Was there a more maintainable approach?

Yes. Centralizing framing/login helpers in `tools/runtime/slsk_runtime.py` avoided duplicated logic across probe/provision/capture scripts and reduced errors during the login format migration.

### What did we reuse to avoid double writing?

1. `compute_login_md5hash` in `rust/protocol` and the shared Python equivalent to keep login request parity.
2. `compare_capture_sequences_with_mode` to extend the existing verifier with semantic mode while preserving bytes mode.
3. `scripts/kb_sync_docs.py` + `derive_message_schema.sh` to regenerate docs/schema without manual edits in multiple artifacts.

### What did we remove to reduce maintenance surface?

1. Runtime dependency on `--password-md5` (deprecated with explicit error).
2. The `fire-and-forget` login assumption in `core`; now replaced with typed, verifiable handshake flow.
3. Verification coupling to byte-only comparison; now semantic mode is reusable and default-capable.
