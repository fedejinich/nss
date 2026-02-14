# Project Status

## Date

- 2026-02-14

## Current Phase

- Stage 2 complete at repository level: Core P2P MVP + KB-first coverage contract for 25 core messages.
- Product direction updated: app evolutiva propia, no clon 1:1 del cliente oficial.

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

## Operational Note

- Redacted stage2 runs are currently bootstrap deterministic fixtures (`source_type=synthetic_fixture_replay`) for reproducible verification.
- Operator-run real-account captures can replace these runs without changing the workflow.
