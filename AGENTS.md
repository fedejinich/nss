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

## Dashboard and Catalog Discipline

When stage status, protocol coverage, or PR documentation changes:

1. Update `analysis/state/stage_registry.json` as the canonical stage-state source.
2. Regenerate state artifacts with `scripts/sync_state_dashboards.sh`:
   - `docs/state/protocol-matrix.md`
   - `docs/state/protocol-matrix.json`
   - `docs/state/project-dashboard-data.json`
   - `docs/state/codebase-graph.json`
   - `docs/pr/index.md`
3. Keep `docs/state/project-dashboard.html` and `docs/state/codebase-visualizer.html` aligned with generated JSON schemas.
4. Keep individual PR docs in `docs/pr/*.md` and expose only `docs/pr/index.md` in top-level Zensical navigation.
5. Validate the regenerated artifacts via `scripts/run_regression.sh` and `zensical build` before merge.

## PR Review Loop Discipline

For each stage branch/PR, run two local review loops before final merge (without `@codex review` calls):

1. Open/update PR and run review loop round 1:
   - security best-practices review on touched paths
   - code-simplifier pass on touched Rust files
2. Apply useful fixes and document rationale for rejected suggestions in the PR notes.
3. Run review loop round 2 after round-one updates are pushed.
4. Merge only after both local review loops are complete and validation gates are green.

## Mandatory Review Passes Per Stage PR

Before opening each stage PR, run two additional code-review passes on touched code:

1. Security best-practices pass:
   - perform a security-focused review of touched runtime/protocol/core/cli paths
   - explicitly check input parsing assumptions, panic/unwrap usage in production paths, and sensitive-data handling
2. Code simplifier pass:
   - run a maintainability-focused simplification pass on touched Rust files
   - remove avoidable duplication/complexity while preserving exact behavior

Record both passes in the stage PR document under `docs/pr/`.

## Branch Start Discipline

Before starting work on a new branch/PR, use this default flow unless explicitly instructed otherwise:

1. Checkout `main`.
2. Pull latest changes from `origin/main`.
3. Create the new working branch from the updated `main`.

Exception:

1. If the user explicitly requests another base branch or commit, branch from that specified base.

## Repository Language Policy

- All repository code comments, commit-ready docs, runbooks, status files, and canonical artifacts must be written in English.
- Non-English text is not allowed in tracked repository content.
- If a source artifact is captured in another language, document an English summary in the canonical KB files.
- Working conversations and non-committed scratch notes may be written in Spanish.
- Before committing, any repository content drafted in Spanish must be translated to English.
