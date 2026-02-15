# PR 0026 - S7R/S8B: runtime-complete closure and minimal TUI foundation

## Branch

- `codex/s7r-s8c-runtime-complete-minimal-tui`

## Objective

Execute the replan baseline from post-S6 closure through:

1. Strict runtime closure (`verified_runtime=131`, `verified_static=0`).
2. Semantic-tail closure (`raw_tail/raw_payload` unresolved fields = `0`).
3. Core orchestration flow (`search_select_and_download`) with CLI `session download-auto`.
4. Capabilities and critical-path dashboard artifacts.
5. Minimal TUI v1 shell wired to `soul-core`.

## Scope

1. Add runtime/capability state registries and generators.
2. Add runtime closure capture generators and redacted artifacts for static-only and semantic-tail targets.
3. Promote message-map/schema semantics and regression contracts for strict closure.
4. Add core orchestration API and CLI auto-download command.
5. Add capability dashboard and matrix pages in Zensical nav.
6. Add `rust/tui` crate, TUI runbook, and unit tests.
7. Synchronize roadmap/status/backlog/TODO/AGENTS with S7/S8 progression.

## Runtime Artifacts Added

1. `captures/redacted/login-static-server-runtime`
2. `captures/redacted/peer-static-runtime`
3. `captures/redacted/login-partial-tail-runtime`
4. `captures/redacted/login-search-download-auto`

## Key Added Artifacts

1. `analysis/state/runtime_coverage_registry.json`
2. `analysis/state/capability_registry.json`
3. `tools/state/generate_runtime_coverage.py`
4. `tools/state/generate_capability_matrix.py`
5. `docs/state/runtime-coverage.json`
6. `docs/state/runtime-coverage.md`
7. `docs/state/capability-matrix.json`
8. `docs/state/capability-matrix.md`
9. `docs/state/capability-dashboard.html`
10. `docs/runbooks/tui-core-transfer.md`
11. `tests/protocol/test_stage7_runtime_semantic_contract.py`
12. `tests/state/test_runtime_and_capability_generators.py`

## Validation

```bash
python3 scripts/kb_validate.py
scripts/run_diff_verify.sh
scripts/run_regression.sh
./.venv-tools/bin/zensical build -f zensical.toml
```

Observed result:

1. `scripts/run_regression.sh`: pass.
2. `scripts/run_diff_verify.sh`: pass in semantic mode, including new Stage 7 runs.
3. `python3 scripts/kb_validate.py`: pass.
4. `./.venv-tools/bin/zensical build -f zensical.toml`: pass.

## Mandatory Blocking Review Loops

1. Round 1 (pending on PR):
   - `blockchain_protocol_engineer`
   - `code_simplifier`
   - `web3_security_review_expert`
2. Round 2 (pending on PR after round-1 fixes):
   - `blockchain_protocol_engineer`
   - `code_simplifier`
   - `web3_security_review_expert`

## Retrospective

1. Was there a more maintainable approach?
   - Yes: centralizing runtime/capability reporting in generators reduced manual status drift across roadmap, dashboard, and verification pages.
2. What did we reuse to avoid double writing?
   - Reused the existing capture harness + redaction pipeline and existing state-sync script entrypoint.
   - Reused core SDK APIs in TUI instead of introducing protocol-level duplicate logic.
3. What did we remove to reduce maintenance surface?
   - Removed residual `raw_tail/raw_payload` placeholders from canonical schema.
   - Removed static-only runtime status from the protocol map.
