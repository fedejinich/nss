# Project Status

## Date

- 2026-02-14

## Current Phase

- Stage 2 complete at repository level: Core P2P MVP + KB-first coverage contract for 25 core messages.
- Stage 2R complete: runtime capture refresh + confidence promotion (`medium -> high`) for all core messages.
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

- Redacted stage2 runs were regenerated from runtime runs in `captures/raw/*`.
- Current coverage confidence is `high=25`, `medium=0`, `low=0`.
- Login runtime capture currently observes server response `INVALIDVERSION` for the sent client version tuple; framing/message evidence remains valid for stage gates.
