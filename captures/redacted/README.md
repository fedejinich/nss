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

Mandatory stage 4B scenarios:

1. `login-room-moderation`
2. `peer-advanced-local`

Mandatory stage 4C scenarios:

1. `login-privileges-social`
2. `peer-folder-local`

Mandatory stage 4D scenarios:

1. `login-privilege-messaging`
2. `peer-legacy-local`

Mandatory stage 4E scenarios:

1. `login-private-message`
2. `login-user-state`
3. `login-peer-address-connect`
4. `login-message-users`
5. `login-peer-message`

Mandatory stage 5A scenarios:

1. `login-parent-distributed-control`

Current state:

- Stage 2 scenarios were regenerated from runtime runs (`captures/raw/* -> captures/redacted/*`).
- Stage 3B room/presence scenarios were generated from authenticated runtime sessions and redacted with the same policy.
- Stage 4A recommendations/discovery scenarios were generated from authenticated runtime sessions and redacted with the same policy.
- Stage 4B room moderation scenario was generated from authenticated runtime sessions and redacted with the same policy.
- Stage 4B peer-advanced scenario is a deterministic local peer runtime run for protocol coverage and semantic verification.
- Stage 4C privileges/social scenario is generated from authenticated runtime sessions and redacted with the same policy.
- Stage 4C peer-folder scenario is a deterministic local peer runtime run for protocol coverage and semantic verification.
- Stage 4D privilege/messaging scenario is generated from authenticated runtime sessions; peer-legacy is deterministic local runtime for parser coverage.
- Stage 4E private messaging and user-state scenarios are generated from authenticated runtime sessions plus deterministic local peer-message alias coverage.
- Stage 5A parent/distributed control scenario is generated from authenticated runtime sessions and includes evidence for codes `83`, `84`, `113`, `121`, and `122`.
- `login-*` scenarios include live server socket traffic to `server.slsknet.org:2242`.
- `download/upload` peer flows are runtime local-peer exchanges used for deterministic protocol evidence and regression.
