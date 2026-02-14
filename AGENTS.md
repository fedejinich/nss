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

