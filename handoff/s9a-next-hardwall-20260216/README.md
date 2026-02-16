# S9A-NEXT Hardwall Handoff Bundle (2026-02-16)

This folder contains a self-contained handoff package so a fresh clone can resume the investigation immediately.

## Included

- `nss-handoff-manifest.json`: machine-readable checkpoint metadata.
- `nss-handoff-checkpoint.txt`: quick branch/head/status snapshot.
- `nss-codex-resume-prompt.txt`: prompt to paste into Codex on the destination machine.
- `nss-handoff-destination-commands.sh`: bootstrap script for destination clone parity checks.
- `nss-handoff-artifacts.tgz`: sidecar archive built from runtime hardwall logs.
- `logs/*.log`: unpacked runtime logs used in this hardwall iteration.

## Clone-and-resume flow

1. Clone repository and checkout `main`.
2. Open Codex in repo root.
3. Paste `nss-codex-resume-prompt.txt`.
4. Run `nss-handoff-destination-commands.sh` if you want automated parity checks.

## Credentials

`.env.local` is intentionally excluded from git. Copy it securely on the destination machine before live runtime tests.
