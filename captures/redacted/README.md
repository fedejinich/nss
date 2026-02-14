# Redacted Captures

This directory stores commit-safe runtime artifacts.

Policy:

- Only redacted artifacts are versioned.
- Raw runs stay local under `captures/raw/<run_id>/`.
- Every run must include:
  - `manifest.redacted.json`
  - `frida-events.redacted.jsonl` (when available)
  - `official_frames.hex`
  - `neo_frames.hex`

Mandatory stage 2 scenarios:

1. `login-only`
2. `login-search`
3. `login-search-download`
4. `upload-deny`
5. `upload-accept`

Current state:

- Stage 2 scenarios were regenerated from runtime runs (`captures/raw/* -> captures/redacted/*`).
- `login-*` scenarios include live server socket traffic to `server.slsknet.org:2242`.
- `download/upload` peer flows are runtime local-peer exchanges used for deterministic protocol evidence and regression.
