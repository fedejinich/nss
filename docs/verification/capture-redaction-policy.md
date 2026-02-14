# Capture Redaction Policy

## Scope

This policy defines how runtime capture artifacts are sanitized before commit.

## Directories

- Raw local-only: `captures/raw/<run_id>/`
- Redacted commit-safe: `captures/redacted/<run_id>/`

## Mandatory redaction targets

- Usernames and aliases.
- Passwords / auth hashes.
- IP addresses and ports.
- Local and virtual file paths.
- Private message or chat text.

## Mechanism

- Use deterministic tokenization in the form `<redacted:<kind>:<hash>>`.
- Hashes are deterministic per run (`salt=run_id` by default), so fields can be correlated inside the same run.
- Raw artifacts are never committed.

## Required redacted artifacts per run

- `manifest.redacted.json`
- `frida-events.redacted.jsonl` (if runtime hooks were enabled)
- `official_frames.hex`
- `neo_frames.hex`
- `redaction-summary.json`

## Workflow

1. Capture raw run:

```bash
scripts/capture_session.sh
```

2. Redact manually when needed:

```bash
RUN_DIR=captures/raw/<run_id> scripts/redact_capture_run.sh
```

3. Verify parity on redacted outputs:

```bash
scripts/run_diff_verify.sh
```

## Enforcement

- `python3 scripts/kb_validate.py` must pass.
- No entry in KB maps/schema may point to missing evidence links.
- Differential verification must pass for required stage 2 scenarios.
