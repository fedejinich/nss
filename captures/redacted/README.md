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

Bootstrap note:

- Current files are deterministic redacted fixtures (`source_type=synthetic_fixture_replay`) used to keep differential verification reproducible.
- Replace them with real account captures when operator credentials/session window are available.
