# NeoSoulSeek Knowledge Base

This site is the canonical project memory for protocol mapping, runtime verification, and CLI-first product evolution.

## Start Here

- [Project Dashboard](state/project-dashboard.html)
- [Roadmap](state/roadmap.md)
- [Protocol Matrix](state/protocol-matrix.md)
- [Codebase Visualizer](state/codebase-visualizer.md)
- [Verification Status](state/verification-status.md)
- [PR Catalog](pr/index.md)

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
- `docs/state/protocol-matrix.md`
- `docs/state/protocol-matrix.json`
- `docs/state/project-dashboard-data.json`
- `docs/state/codebase-graph.json`
- `docs/verification/evidence-ledger.md`
- `docs/re/static/detangling.md`

## Operations

```bash
python3 scripts/kb_promote.py
python3 scripts/kb_sync_docs.py
python3 scripts/kb_validate.py
python3 tools/protocol/generate_protocol_matrix.py
python3 tools/state/generate_dashboard_data.py
python3 tools/state/generate_codebase_graph.py
python3 tools/docs/generate_pr_index.py
scripts/sync_state_dashboards.sh
scripts/run_diff_verify.sh
scripts/run_regression.sh
.venv-tools/bin/zensical build -f zensical.toml
```
