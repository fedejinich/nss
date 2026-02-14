# Project Status

## Date

- 2026-02-14

## Current Phase

- Stage 3A complete at repository level: authenticated login real + semantic parity verifier.
- Stage 2 complete: Core P2P MVP + KB-first coverage contract for 25 core messages.
- Stage 2R complete: runtime capture refresh + confidence promotion (`medium -> high`) for all core messages.
- Documentation discipline institutionalized: `TODO-CODEX.md` + `AGENTS.md` + Zensical docs are mandatory per iteration.
- Product direction updated: app evolutiva propia, no clon 1:1 del cliente oficial.

## Stage 3A Completion

1. Auth runtime tuple accepted by official server: `160/1` (`client_version/minor_version`).
2. Login request migrated to `username + password + client_version + md5hash + minor_version`.
3. Session auth is stateful: `LoggedIn` only after valid `LoginResponsePayload::Success`.
4. CLI/runtime moved to `--password` plain, `.env.local` local-only, `--password-md5` deprecated (explicit error).
5. Semantic differential verification implemented and enabled by default in `scripts/run_diff_verify.sh`.
6. Mandatory redacted scenarios regenerated with authenticated runtime evidence.

## Stage 2 Completion

1. Protocol contract published and enforced (`25/25` core messages).
2. Runtime pipeline moved to `raw -> redacted` with documented redaction policy.
3. Rust SDK/CLI extended for session/download/upload manual/verify captures.
4. Differential verification expanded to redacted capture runs.
5. Regression suite expanded (Python + Rust + capture replay + KB validation).

## Core Artifacts

- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`
- `docs/re/static/message-schema.md`
- `docs/verification/capture-redaction-policy.md`
- `captures/redacted/login-only/manifest.redacted.json`
- `captures/redacted/login-search/manifest.redacted.json`
- `captures/redacted/login-search-download/manifest.redacted.json`
- `captures/redacted/upload-deny/manifest.redacted.json`
- `captures/redacted/upload-accept/manifest.redacted.json`
- `tools/runtime/probe_login_versions.py`
- `tools/runtime/provision_test_credentials.py`

## Operational Note

- Redacted runs were regenerated from authenticated runtime runs in `captures/raw/*`.
- Current coverage confidence is `high=25`, `medium=0`, `low=0`.
- Authenticated login now succeeds with tuple `160/1`, validated via runtime probe and session login command.
