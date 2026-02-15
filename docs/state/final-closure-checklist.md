# Final Closure Checklist (v1)

This checklist tracks final release-hardening closure before v1 is considered complete.

## Capability Gates

- [x] `FG-RUNTIME-100`
- [x] `FG-SEMANTIC-DEPTH`
- [x] `FG-CORE-AUTO-DL`
- [x] `FG-TUI-V1`
- [x] `FG-RELEASE-HARDENING`

## S8C Hardening Items

- [x] Redaction metadata hardening is enforced (no absolute path metadata in committed redacted artifacts).
- [x] Release packaging workflow is published and reproducible (`scripts/package_release.sh` + runbook).
- [x] Failure recovery runbook is published for auth/search/download/TUI recovery.
- [x] Release hardening audit report is generated and green.

## Final Validation

- [x] `python3 scripts/kb_validate.py`
- [x] `scripts/run_diff_verify.sh`
- [x] `scripts/run_regression.sh`
- [x] `./.venv-tools/bin/zensical build -f zensical.toml`
- [x] S8C closure package is merge-ready and final v1 closure gate is promoted in stage/capability registries.

## Notes

When all boxes are checked, update:

1. `analysis/state/capability_registry.json`
2. `analysis/state/stage_registry.json`
3. `docs/state/project-status.md`
4. `docs/state/verification-status.md`
5. `docs/state/roadmap.md`
