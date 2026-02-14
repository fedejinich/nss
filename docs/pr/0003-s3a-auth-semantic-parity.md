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

### ¿Había una forma más mantenible?

Sí: centralizar helpers de framing/login en `tools/runtime/slsk_runtime.py` evitó mantener lógica duplicada en probe/provision/capture scripts y redujo errores en el cambio de formato de login.

### ¿Qué reutilizamos para evitar double writing?

1. `compute_login_md5hash` en `rust/protocol` y equivalente Python compartido para mantener paridad de request login.
2. `compare_capture_sequences_with_mode` para reutilizar pipeline existente y extenderlo con modo semántico sin romper modo bytes.
3. `scripts/kb_sync_docs.py` + `derive_message_schema.sh` para regenerar docs/schema sin editar manualmente múltiples artefactos.

### ¿Qué eliminamos para reducir superficie de mantenimiento?

1. Dependencia runtime en `--password-md5` (deprecada con error explícito).
2. Supuesto de login “fire-and-forget” en `core`; ahora handshake tipado y verificable.
3. Acoplamiento de verificación a comparación byte-a-byte solamente; ahora hay un modo semántico reutilizable.
