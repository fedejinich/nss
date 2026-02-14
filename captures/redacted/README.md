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

Mandatory stage 3B scenarios:

1. `login-room-list`
2. `login-join-room-presence`
3. `login-leave-room`

Mandatory stage 4A scenarios:

1. `login-recommendations`
2. `login-user-recommendations`
3. `login-similar-terms`

Current state:

- Stage 2 scenarios were regenerated from runtime runs (`captures/raw/* -> captures/redacted/*`).
- Stage 3B room/presence scenarios were generated from authenticated runtime sessions and redacted with the same policy.
- Stage 4A recommendations/discovery scenarios were generated from authenticated runtime sessions and redacted with the same policy.
- `login-*` scenarios include live server socket traffic to `server.slsknet.org:2242`.
- `download/upload` peer flows are runtime local-peer exchanges used for deterministic protocol evidence and regression.
