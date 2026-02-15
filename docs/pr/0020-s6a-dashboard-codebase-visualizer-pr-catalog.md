# PR 0020 - S6A: dashboard + codebase visualizer + collapsed PR catalog

## Branch

- `codex/s6a-dashboard-codebase-visualizer-pr-catalog`

## Objective

Establish a dashboard-first Zensical UX with explicit roadmap/state/dependency visualization, an interactive codebase visualizer, and a single collapsed PR catalog page.

## Scope

1. Add canonical stage registry and dashboard data generation.
2. Add visual project dashboard HTML page.
3. Add codebase graph generator + interactive treemap visualizer.
4. Add collapsed PR index generator and catalog page.
5. Curate explicit navigation in `zensical.toml`.
6. Update docs landing and governance workflow references.

## Outcome

1. Added canonical stage-state source:
   - `analysis/state/stage_registry.json`
2. Added project dashboard pipeline:
   - `tools/protocol/generate_protocol_matrix.py` now emits `docs/state/protocol-matrix.json`
   - `tools/state/generate_dashboard_data.py`
   - `docs/state/project-dashboard-data.json`
   - `docs/state/project-dashboard.html`
3. Added codebase visualizer pipeline:
   - `tools/state/generate_codebase_graph.py`
   - `docs/state/codebase-graph.json`
   - `docs/state/codebase-visualizer.html`
   - `docs/state/codebase-visualizer.md`
4. Added collapsed PR catalog:
   - `tools/docs/generate_pr_index.py`
   - `docs/pr/index.md`
5. Added dashboard sync wrapper:
   - `scripts/sync_state_dashboards.sh`
6. Updated navigation and landing:
   - `zensical.toml` now uses explicit curated nav.
   - `docs/index.md` is dashboard-first.
7. Updated governance and state docs:
   - `AGENTS.md`
   - `TODO-CODEX.md`
   - `docs/state/project-status.md`
   - `docs/state/verification-status.md`
   - `docs/state/protocol-backlog.md`
   - `docs/state/roadmap.md`
8. Added regression coverage:
   - `tests/state/test_stage_registry.py`
   - `tests/state/test_dashboard_generators.py`
   - `tests/docs/test_pr_index.py`
   - `tests/protocol/test_protocol_matrix.py` updated for matrix JSON assertions.

## Validation

```bash
python3 scripts/kb_validate.py
scripts/run_regression.sh
./.venv-tools/bin/zensical build -f zensical.toml
```

Observed result:

1. `kb_validate`: pass.
2. `run_regression`: pass.
   - Python tests: `32` passed.
   - Rust tests: all crates green.
   - Differential semantic verify: all required runs green.
3. `zensical build`: pass.

## Local Review Loops

1. Round 1 security pass:
   - Verified generators do not read credentials or runtime secrets.
   - Confirmed codebase graph excludes heavy/sensitive directories (`captures`, `.git`, `.venv-tools`, `site`, caches).
   - Confirmed dashboard/visualizer consume local JSON only and do not execute external scripts.
2. Round 1 simplifier pass:
   - Consolidated generation flow in `scripts/sync_state_dashboards.sh` to avoid duplicated manual commands.
   - Kept generator interfaces small (`--out` plus minimal inputs) and aligned data schemas.
3. Round 2 security + simplifier pass:
   - Rechecked test coverage for new generators and catalog.
   - Re-ran full gates and verified no additional complexity or unsafe runtime handling was introduced.

## Retrospective

1. Was there a more maintainable approach?
   - Yes: move state summaries to generated JSON artifacts and keep HTML as pure render layer; this was applied.
2. What was reused to avoid double writing?
   - Reused existing matrix generator and extended it to JSON instead of creating a second matrix pipeline.
   - Reused deterministic generator pattern already used in `tools/protocol` and `tools/runtime`.
3. What was removed to reduce maintenance surface?
   - Removed PR navigation noise by collapsing to one generated index (`docs/pr/index.md`).
   - Centralized state refresh into one command (`scripts/sync_state_dashboards.sh`) instead of multiple manual commands.
