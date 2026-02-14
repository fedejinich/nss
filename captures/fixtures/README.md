# Capture Fixtures

Derived and sanitized protocol fixtures used by tests and differential verification.

Expected files:

- `server_login_request.hex`
- `server_file_search_request.hex`
- `peer_transfer_request.hex`
- `peer_transfer_response.hex`

When generated from golden captures, keep source evidence links in:

- `docs/verification/evidence-ledger.md`

Stage 2 usage:

- Fixtures seed deterministic redacted capture runs under `captures/redacted/*`.
- Differential verification compares `official_frames.hex` and `neo_frames.hex` per run.
