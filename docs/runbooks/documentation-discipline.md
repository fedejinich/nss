# Documentation Discipline Runbook

## Goal

Keep NeoSoulSeek project memory current at all times.

## Non-Negotiable Rule

Every meaningful change must ship with documentation updates in the same delivery cycle.

## Required Surfaces

1. `TODO-CODEX.md`
   - Keep task graph and statuses truthful.
2. `AGENTS.md`
   - Record process-level lessons and policy changes.
3. Zensical docs under `docs/`
   - Update state, verification, and runbooks.
4. Canonical technical artifacts
   - `analysis/ghidra/maps/*.json|csv`
   - `analysis/protocol/*.json`
   - verification ledgers and detailed markdown notes

## Session Checklist

1. Capture new evidence (static/runtime).
2. Write/refresh canonical artifact files.
3. Sync docs and state pages.
4. Update execution plan in `TODO-CODEX.md`.
5. Run validations/regression:

```bash
python3 scripts/kb_validate.py
scripts/run_regression.sh
```

6. Rebuild KB site:

```bash
./.venv-tools/bin/zensical build -f zensical.toml
```

