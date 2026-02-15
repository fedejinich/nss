# NeoSoulSeek Knowledge Base

This site is the canonical project memory for protocol mapping, runtime verification, and CLI-first product evolution.

## Start Here

- [Project Dashboard](state/project-dashboard.html)
- [Roadmap](state/roadmap/)
- [Protocol Matrix](state/protocol-matrix/)
- [Runtime Coverage](state/runtime-coverage/)
- [Capability Dashboard](state/capability-dashboard.html)
- [Capability Matrix](state/capability-matrix/)
- [Release Hardening Audit](state/release-hardening-audit/)
- [Final Closure Checklist](state/final-closure-checklist/)
- [Codebase Visualizer](state/codebase-visualizer/)
- [Verification Status](state/verification-status/)
- [S5A Closure Audit](state/s5a-closure-audit/)
- [S6 Opaque-Tail Plan](state/opaque-tail-plan/)
- [PR Catalog](pr/)

## Core Rules

- The binary is the spec.
- No rename is accepted without evidence.
- High-confidence findings are promoted to authoritative artifacts.
- Medium and low confidence findings remain in review queue until new evidence arrives.
- Project memory is part of the definition of done (`TODO-CODEX.md`, `AGENTS.md`, and canonical docs/artifacts).

## Authoritative Artifacts

- `analysis/ghidra/maps/name_map.json`
- `analysis/ghidra/maps/data_map.json`
- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`
- `analysis/state/stage_registry.json`
- `analysis/state/runtime_coverage_registry.json`
- `analysis/state/capability_registry.json`
- `docs/state/protocol-matrix.md`
- `docs/state/protocol-matrix.json`
- `docs/state/runtime-coverage.json`
- `docs/state/capability-matrix.json`
- `docs/state/release-hardening-audit.json`
- `docs/state/final-closure-checklist.md`
- `docs/state/project-dashboard-data.json`
- `docs/state/codebase-graph.json`
- `docs/state/s5a-closure-audit.json`
- `docs/state/opaque-tail-report.json`
- `docs/verification/evidence-ledger.md`
- `docs/re/static/detangling.md`

## Operations

```bash
python3 scripts/kb_promote.py
python3 scripts/kb_sync_docs.py
python3 scripts/kb_validate.py
python3 tools/protocol/generate_protocol_matrix.py
python3 tools/state/generate_runtime_coverage.py
python3 tools/state/generate_capability_matrix.py
python3 tools/state/generate_dashboard_data.py
python3 tools/state/generate_codebase_graph.py
python3 tools/state/verify_s5a_closure.py
python3 tools/state/verify_release_hardening.py
python3 tools/state/report_opaque_tail.py
python3 tools/docs/generate_pr_index.py
python3 tools/runtime/sanitize_redacted_metadata.py
scripts/sync_state_dashboards.sh
scripts/run_diff_verify.sh
scripts/run_regression.sh
scripts/package_release.sh
.venv-tools/bin/zensical build -f zensical.toml
```
