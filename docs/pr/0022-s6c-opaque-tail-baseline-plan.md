# PR 0022 - S6C: opaque-tail executable baseline and batch plan

## Branch

- `codex/s6c-opaque-tail-report`

## Objective

Establish an executable baseline for the remaining opaque control-message tail and publish a concrete batch plan for S6 typed-promotion implementation.

## Scope

1. Add opaque-tail report generator.
2. Add generated opaque-tail report artifact.
3. Add opaque-tail execution plan page.
4. Add regression test for report generation.
5. Wire report generation into state sync workflow.
6. Sync stage/roadmap/status/backlog/TODO artifacts.

## Outcome

1. Added executable opaque-tail report generator:
   - `tools/state/report_opaque_tail.py`
2. Added generated report artifact:
   - `docs/state/opaque-tail-report.json`
3. Added published S6 plan page:
   - `docs/state/opaque-tail-plan.md`
4. Added regression coverage:
   - `tests/state/test_opaque_tail_report.py`
5. Wired report into sync workflow:
   - `scripts/sync_state_dashboards.sh`
6. Updated stage/status/governance artifacts for S6C:
   - `analysis/state/stage_registry.json`
   - `docs/state/roadmap.md`
   - `docs/state/project-status.md`
   - `docs/state/verification-status.md`
   - `docs/state/protocol-backlog.md`
   - `docs/index.md`
   - `AGENTS.md`
   - `TODO-CODEX.md`
   - `zensical.toml`

## Validation

```bash
python3 tools/state/report_opaque_tail.py
python3 scripts/kb_validate.py
scripts/run_regression.sh
./.venv-tools/bin/zensical build -f zensical.toml
```

Observed result:

1. `report_opaque_tail.py`: pass.
2. `kb_validate.py`: pass.
3. `run_regression.sh`: pass.
   - Python tests: all green including opaque-tail report test.
   - Rust tests: all crates green.
   - Semantic differential verify: all required runs green.
4. `zensical build`: pass.

## Local Review Loops

1. Round 1 security pass:
   - verified report generator reads repository-local artifacts only.
   - no sensitive runtime payloads or raw captures are accessed.
2. Round 1 code-simplifier pass:
   - kept parser/report flow linear and deterministic.
   - avoided redundant parsers by reusing protocol constants + message map.
3. Round 2 security + simplifier pass:
   - rechecked batch classification and report schema after full gates.
   - no additional fixes required.

## Retrospective

1. More maintainable approach:
   - keep opaque-tail baseline as generated artifact rather than manual lists in docs.
2. Reuse to avoid double writing:
   - reused existing protocol constants and message map as the only sources of truth.
3. Surface reduction:
   - centralized opaque-tail inventory in one tool (`report_opaque_tail.py`) and one JSON output consumed by docs.
