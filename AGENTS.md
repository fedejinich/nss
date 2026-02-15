# AGENTS.md - NeoSoulSeek

## Scope

This file defines project-level execution and documentation rules for all contributors/agents.

## Mandatory Documentation Discipline

Every relevant work session must update project memory in the same change set.

Required updates (when applicable):

1. `TODO-CODEX.md`
   - Add/update task IDs, statuses, and dependencies.
   - Record new blockers, assumptions, and outcomes.
2. `AGENTS.md`
   - Update this file whenever process-level learnings or governance rules change.
3. Zensical knowledge base docs (`docs/`)
   - Reflect latest state, evidence paths, and runbooks.
   - Keep status/audit docs aligned with code and captures.
4. Authoritative artifacts (`.json`, `.csv`, `.md`)
   - Persist technical learnings in canonical maps/schemas/ledgers.
   - Avoid leaving critical knowledge only in commit messages or chat.

## Capability-First Planning Discipline

Before implementing a new stage or long-session objective:

1. Publish the stage plan in KB/dashboard artifacts first:
   - `analysis/state/stage_registry.json`
   - `analysis/state/capability_registry.json`
   - `docs/state/roadmap.md`
   - `TODO-CODEX.md`
2. Break the stage into explicit capabilities with:
   - `id`
   - `status`
   - `depends_on`
   - blockers
   - evidence target
3. Ensure capability dependencies align with stage dependencies.
4. Regenerate dashboard artifacts before code changes so the current plan is visible in Zensical.
5. Keep execution and reporting separated by capability domain (`runtime`, `schema`, `core/cli`, `tui`, `release`, `security`, `ops`).

## KB-First Rule

- High-confidence findings with valid evidence are promoted.
- Medium/low confidence stays in review queue until new evidence arrives.
- No evidence -> no promotion.
- Broken evidence links invalidate the entry.

## Definition of Done (Documentation)

A task is not done until:

1. Runtime/static evidence is written to canonical files.
2. `TODO-CODEX.md` reflects real status.
3. Zensical docs reflect the new state.
4. Validation/regression scripts pass for the affected area.

## Runtime Auth Discipline

When work touches authenticated runtime flows:

1. Keep credentials local-only via `.env.local` and never commit secrets.
2. Update `.env.example` whenever required runtime vars change.
3. Refresh redacted captures and re-run differential verification in semantic mode.
4. Record accepted login tuple and evidence paths in state docs and ledger.

## Protocol Decode Discipline

1. Server and peer message code spaces overlap; generic `decode_message(...)` can be ambiguous without transport context.
2. For deterministic protocol tests and feature logic, prefer scoped decoders (`decode_server_message(...)` or `decode_peer_message(...)`) when the channel is known.
3. Keep semantic diff fallback behavior, but avoid using ambiguous generic decode output as authoritative protocol classification.

## Compressed Payload Safety Discipline

When promoting compressed protocol payloads from opaque to typed:

1. Enforce explicit decompression size limits to prevent memory-amplification regressions.
2. Preserve format fallback classification (`typed`, `text`, `opaque`) instead of failing hard on unknown shapes.
3. Add regression tests for oversized compressed payload rejection and at least one valid decompression path.

## Message-Code Resolution Discipline

When a protocol code mapping is unresolved but the symbol name exists in static tables:

1. Prefer deterministic jump-table extraction from the binary before runtime-only assumptions.
2. Persist extractor tooling under `tools/re/` and commit machine-readable + human-readable evidence outputs.
3. Promote mapping confidence only after evidence is registered in `message_map.csv` and synchronized into schema/docs.

## Stage Iteration Discipline

When a stage closes (for example S3A, S3B):

1. Update `docs/state/roadmap.md` status matrix and next-gate column.
2. Sync `docs/state/project-status.md`, `docs/state/verification-status.md`, and `docs/state/protocol-backlog.md`.
3. Persist stage task closure in `TODO-CODEX.md` with dependency graph + final statuses.
4. Add or update a PR stage document under `docs/pr/` with validation commands and retrospective.
5. Add or refresh stage-specific runtime capture generator tooling under `tools/runtime/` when new runtime scenarios are required.
6. Add or refresh protocol contract tests under `tests/protocol/` for every new mapped message batch.
7. Regenerate `docs/state/protocol-matrix.md` whenever message coverage or protocol constants change.
8. For long-session work, add a capability-level execution breakdown in `docs/state/roadmap.md` and keep it synchronized with `analysis/state/capability_registry.json`.

## Dashboard and Catalog Discipline

When stage status, protocol coverage, or PR documentation changes:

1. Update `analysis/state/stage_registry.json` as the canonical stage-state source.
2. Regenerate state artifacts with `scripts/sync_state_dashboards.sh`:
   - `docs/state/protocol-matrix.md`
   - `docs/state/protocol-matrix.json`
   - `docs/state/runtime-coverage.json`
   - `docs/state/runtime-coverage.md`
   - `docs/state/capability-matrix.json`
   - `docs/state/capability-matrix.md`
   - `docs/state/release-hardening-audit.json`
   - `docs/state/release-hardening-audit.md`
   - `docs/state/project-dashboard-data.json`
   - `docs/state/codebase-graph.json`
   - `docs/state/s5a-closure-audit.json`
   - `docs/state/opaque-tail-report.json`
   - `docs/pr/index.md`
3. Keep `docs/state/project-dashboard.html` and `docs/state/codebase-visualizer.html` aligned with generated JSON schemas.
4. Keep `docs/state/capability-dashboard.html` aligned with `docs/state/capability-matrix.json`.
5. Keep individual PR docs in `docs/pr/*.md` and expose only `docs/pr/index.md` in top-level Zensical navigation.
6. Use route-safe navigation links (`.../slug/`) instead of direct `.md` links in dashboard and nav pages.
7. Validate the regenerated artifacts via `scripts/run_regression.sh` and `zensical build` before merge.

## PR Review Loop Discipline

For each stage branch/PR, run two local review loops before final merge (without `@codex review` calls):

1. Open/update PR and run review loop round 1:
   - `blockchain_protocol_engineer` review pass
   - `code_simplifier` pass on touched Rust files
   - `web3_security_review_expert` review pass
2. Apply useful fixes and document rationale for rejected suggestions in the PR notes.
3. Run review loop round 2 after round-one updates are pushed.
4. Merge only after both local review loops are complete and validation gates are green.

## Long Session Objective Discipline

When a long-session objective is active (for example: "minimal TUI search + download"):

1. Keep the objective explicit in dashboard/roadmap/current status.
2. Execute in iterative capability slices while preserving gate stability.
3. Prioritize product-usable flow first, then hardening/packaging, then closure gates.
4. Do not mark final closure capabilities as done until all underlying capability blockers are cleared and evidenced.

## Mandatory Review Passes Per Stage PR

For each stage PR, run these blocking review passes during both mandatory review loops:

1. `blockchain_protocol_engineer` pass:
   - execute protocol-scope review for network/protocol-facing changes and document assumptions/evidence.
2. `web3_security_review_expert` pass:
   - perform security-focused review of touched runtime/protocol/core/cli paths
   - explicitly check input parsing assumptions, panic/unwrap usage in production paths, and sensitive-data handling
3. `code_simplifier` pass:
   - run a maintainability-focused simplification pass on touched Rust files
   - remove avoidable duplication/complexity while preserving exact behavior

Record all three passes in the stage PR document under `docs/pr/`.

## Branch Start Discipline

Before starting work on a new branch/PR, use this default flow unless explicitly instructed otherwise:

1. Checkout `main`.
2. Pull latest changes from `origin/main`.
3. Create the new working branch from the updated `main`.

Exception:

1. If the user explicitly requests another base branch or commit, branch from that specified base.

## Canonical Stage Execution Workflow

For every stage iteration, follow this end-to-end loop without skipping steps:

1. Start from updated `main`:
   - `git checkout main`
   - `git pull origin main`
   - `git checkout -b codex/<stage-name>`
2. Publish/update the execution plan first (dependency graph + persistable TODO statuses) in:
   - `TODO-CODEX.md`
   - `analysis/state/stage_registry.json`
   - capability and roadmap docs when scope changes
3. Implement the stage scope in small, testable commits.
4. Run tests/harness/gates before PR update:
   - `python3 scripts/kb_validate.py`
   - `scripts/run_diff_verify.sh`
   - `scripts/run_regression.sh`
   - `./.venv-tools/bin/zensical build -f zensical.toml`
   - plus stage-specific runtime capture/verify commands
5. Open or update one stage PR with its PR document in `docs/pr/`.
6. Run mandatory blocking review loop round 1:
   - `blockchain_protocol_engineer`
   - `code_simplifier`
   - `web3_security_review_expert`
7. Apply useful feedback, document dismissals with rationale, and resolve threads.
8. Run mandatory blocking review loop round 2 with the same three passes, then re-run required gates.
9. Merge to `main` only after green gates and completed review loops, then immediately repeat from step 1 for the next stage.

Notes:

1. `@codex review` is disabled for this workflow and is not part of the blocking loop.
2. Keep repository content in English, while planning chat can remain in Spanish.

## Repository Language Policy

- All repository code comments, commit-ready docs, runbooks, status files, and canonical artifacts must be written in English.
- Non-English text is not allowed in tracked repository content.
- If a source artifact is captured in another language, document an English summary in the canonical KB files.
- Working conversations and non-committed scratch notes may be written in Spanish.
- Before committing, any repository content drafted in Spanish must be translated to English.
