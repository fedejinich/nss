# TODO Execution Plan - NeoSoulSeek

## Stage 6B - S5A Closure Hardening Audit

Dependency graph:

- `S6B-W01 -> S6B-W02`
- `S6B-W02 -> S6B-T01`
- `S6B-T01 -> S6B-T02`
- `S6B-T02 -> S6B-T03`
- `S6B-T03 -> S6B-T04`
- `S6B-T04 -> S6B-T05`
- `S6B-T05 -> S6B-T06`
- `S6B-T06 -> S6B-T07`
- `S6B-T07 -> S6B-Q01`
- `S6B-Q01 -> S6B-Q02`
- `S6B-Q02 -> S6B-T08`

Tasks:

- id: S6B-W01
  description: Start from updated `main` and create branch `codex/s6b-s5a-runtime-closure`
  status: done
  depends_on: []

- id: S6B-W02
  description: Create PR doc scaffold `docs/pr/0021-s6b-s5a-closure-hardening-audit.md`
  status: done
  depends_on: [S6B-W01]

- id: S6B-T01
  description: Implement executable S5A closure verifier (`tools/state/verify_s5a_closure.py`)
  status: done
  depends_on: [S6B-W02]

- id: S6B-T02
  description: Generate closure artifact `docs/state/s5a-closure-audit.json`
  status: done
  depends_on: [S6B-T01]

- id: S6B-T03
  description: Publish closure audit page `docs/state/s5a-closure-audit.md`
  status: done
  depends_on: [S6B-T02]

- id: S6B-T04
  description: Add regression test `tests/state/test_s5a_closure_audit.py`
  status: done
  depends_on: [S6B-T03]

- id: S6B-T05
  description: Wire closure verification into `scripts/sync_state_dashboards.sh`
  status: done
  depends_on: [S6B-T04]

- id: S6B-T06
  description: Sync stage/roadmap/status/backlog/index/nav artifacts for S6B closure
  status: done
  depends_on: [S6B-T05]

- id: S6B-T07
  description: Run validation gates (`kb_validate`, `run_regression`, `zensical build`)
  status: done
  depends_on: [S6B-T06]

- id: S6B-Q01
  description: Review loop round 1 (security best-practices + code-simplifier)
  status: done
  depends_on: [S6B-T07]

- id: S6B-Q02
  description: Review loop round 2 (security best-practices + code-simplifier)
  status: done
  depends_on: [S6B-Q01]

- id: S6B-T08
  description: Finalize merge-ready PR and retrospective for S6B
  status: done
  depends_on: [S6B-Q02]

Notes:

- S6B introduces no new protocol messages; it hardens closure guarantees for already completed S5A objectives.
- Closure scope validated:
  - opaque -> typed runtime evidence
  - runtime captures for parent/distributed and global/distributed control
  - decompression-aware parser for `PM_SHARED_FILES_IN_FOLDER`
  - residual hypothesis closure for `SM_GET_USER_PRIVILEGES_STATUS` and `SM_UPLOAD_SPEED`

## Stage 6A - Visual Dashboard + Codebase Visualizer + Collapsed PR Catalog

Dependency graph:

- `S6A-W01 -> S6A-W02`
- `S6A-W02 -> S6A-T01`
- `S6A-T01 -> S6A-T02`
- `S6A-T02 -> S6A-T03`
- `S6A-T03 -> S6A-T04`
- `S6A-W02 -> S6A-V01`
- `S6A-V01 -> S6A-V02`
- `S6A-V02 -> S6A-V03`
- `S6A-V03 -> S6A-V04`
- `S6A-T03 -> S6A-T05`
- `S6A-T05 -> S6A-T06`
- `S6A-T04 -> S6A-T07`
- `S6A-V04 -> S6A-T07`
- `S6A-T06 -> S6A-T07`
- `S6A-T07 -> S6A-T08`
- `S6A-T08 -> S6A-T09`
- `S6A-T09 -> S6A-T10`
- `S6A-T10 -> S6A-T11`
- `S6A-T11 -> S6A-Q01`
- `S6A-Q01 -> S6A-Q02`
- `S6A-Q02 -> S6A-T12`

Tasks:

- id: S6A-W01
  description: Checkout `main`, pull `origin/main`, create branch `codex/s6a-dashboard-codebase-visualizer-pr-catalog`
  status: done
  depends_on: []

- id: S6A-W02
  description: Create PR doc scaffold at `docs/pr/0020-s6a-dashboard-codebase-visualizer-pr-catalog.md`
  status: done
  depends_on: [S6A-W01]

- id: S6A-T01
  description: Add `analysis/state/stage_registry.json` with stage metadata and dependencies
  status: done
  depends_on: [S6A-W02]

- id: S6A-T02
  description: Extend protocol matrix generator to emit `docs/state/protocol-matrix.json`
  status: done
  depends_on: [S6A-T01]

- id: S6A-T03
  description: Implement dashboard data generator (`tools/state/generate_dashboard_data.py`)
  status: done
  depends_on: [S6A-T02]

- id: S6A-T04
  description: Implement visual project dashboard (`docs/state/project-dashboard.html`)
  status: done
  depends_on: [S6A-T03]

- id: S6A-V01
  description: Define codebase graph schema and exclusion policy
  status: done
  depends_on: [S6A-W02]

- id: S6A-V02
  description: Implement codebase graph generator (`tools/state/generate_codebase_graph.py`)
  status: done
  depends_on: [S6A-V01]

- id: S6A-V03
  description: Implement codebase treemap visualizer (`docs/state/codebase-visualizer.html`)
  status: done
  depends_on: [S6A-V02]

- id: S6A-V04
  description: Add visualizer wrapper doc (`docs/state/codebase-visualizer.md`)
  status: done
  depends_on: [S6A-V03]

- id: S6A-T05
  description: Implement collapsed PR index generator (`tools/docs/generate_pr_index.py`)
  status: done
  depends_on: [S6A-T03]

- id: S6A-T06
  description: Generate `docs/pr/index.md` and use it as the only PR navigation entry
  status: done
  depends_on: [S6A-T05]

- id: S6A-T07
  description: Update `docs/index.md` to dashboard-first start page
  status: done
  depends_on: [S6A-T04, S6A-V04, S6A-T06]

- id: S6A-T08
  description: Curate explicit nav in `zensical.toml` and hide individual PR docs from top nav
  status: done
  depends_on: [S6A-T07]

- id: S6A-T09
  description: Update `TODO-CODEX.md` and `AGENTS.md` with dashboard regeneration workflow
  status: done
  depends_on: [S6A-T08]

- id: S6A-T10
  description: Add regression tests for matrix JSON, stage registry, codebase graph, and collapsed PR index generation
  status: done
  depends_on: [S6A-T09]

- id: S6A-T11
  description: Run validation gates (`kb_validate`, `run_regression`, `zensical build`)
  status: done
  depends_on: [S6A-T10]

- id: S6A-Q01
  description: Review loop round 1 (security best-practices + code simplification pass)
  status: done
  depends_on: [S6A-T11]

- id: S6A-Q02
  description: Review loop round 2 (security best-practices + code simplification pass)
  status: done
  depends_on: [S6A-Q01]

- id: S6A-T12
  description: Finalize merge-ready PR documentation and retrospective
  status: done
  depends_on: [S6A-Q02]

Notes:

- Added new sync entrypoint: `scripts/sync_state_dashboards.sh`.
- Added generated state artifacts: `protocol-matrix.json`, `project-dashboard-data.json`, `codebase-graph.json`, and `docs/pr/index.md`.
- Navigation now exposes a compact PR catalog and dashboard-first project-state entry points.

## Stage 2 - Core P2P MVP + 25 messages

Dependency graph:

- `S2-T01 -> S2-T02`
- `S2-T01 -> S2-T03`
- `S2-T02 -> S2-T04`
- `S2-T03 -> S2-T04`
- `S2-T04 -> S2-T05`
- `S2-T04 -> S2-T06`
- `S2-T05 -> S2-T07`
- `S2-T06 -> S2-T07`
- `S2-T07 -> S2-T08`
- `S2-T08 -> S2-T09`
- `S2-T09 -> S2-T10`
- `S2-T10 -> S2-T11`

Tasks:

- id: S2-T01
  description: Publish stage contract (core P2P scope, target 25 messages, quality criteria)
  status: done
  depends_on: []

- id: S2-T02
  description: Execute reverse engineering focused on 25 messages (callsites, handlers, serializers) with static evidence
  status: done
  depends_on: [S2-T01]

- id: S2-T03
  description: Implement runtime capture redaction pipeline (raw -> redacted) and policy documentation
  status: done
  depends_on: [S2-T01]

- id: S2-T04
  description: Capture core scenarios (login, search, download, upload accept/deny)
  status: done
  depends_on: [S2-T02, S2-T03]

- id: S2-T05
  description: Update message_map/message_schema/name_map with 25 core messages and traceable evidence
  status: done
  depends_on: [S2-T04]

- id: S2-T06
  description: Extend rust/protocol and rust/core for core session + manual upload handling
  status: done
  depends_on: [S2-T04]

- id: S2-T07
  description: Extend rust/cli with session/transfer/verify commands and temporary compatibility with legacy commands
  status: done
  depends_on: [S2-T05, S2-T06]

- id: S2-T08
  description: Implement differential verifier over redacted artifacts (official vs NeoSoulSeek)
  status: done
  depends_on: [S2-T07]

- id: S2-T09
  description: Expand regression suite (unit + integration + fixture + capture replay)
  status: done
  depends_on: [S2-T08]

- id: S2-T10
  description: Stage closure with core parity audit and gap report
  status: done
  depends_on: [S2-T09]

- id: S2-T11
  description: Publish complete protocol mapping backlog by functional domains
  status: done
  depends_on: [S2-T10]

Notes:

- Required scenarios in `captures/redacted/*` were refreshed from runtime runs in `captures/raw/*`.
- Runtime-authenticated login evidence was captured and persisted in the KB.

## Stage 2R - Runtime capture refresh + confidence promotion

Dependency graph:

- `S2R-T01 -> S2R-T02`
- `S2R-T02 -> S2R-T03`
- `S2R-T03 -> S2R-T04`
- `S2R-T04 -> S2R-T05`
- `S2R-T05 -> S2R-T06`
- `S2R-T06 -> S2R-T07`

Tasks:

- id: S2R-T01
  description: Generate real runtime captures for login/search/download/upload accept/deny and write raw manifests/frames
  status: done
  depends_on: []

- id: S2R-T02
  description: Replace `captures/redacted/*` with artifacts derived from runtime runs (raw -> redacted)
  status: done
  depends_on: [S2R-T01]

- id: S2R-T03
  description: Promote 7 medium-confidence messages to high with valid runtime evidence links
  status: done
  depends_on: [S2R-T02]

- id: S2R-T04
  description: Regenerate KB schema/docs and validate quality gates
  status: done
  depends_on: [S2R-T03]

- id: S2R-T05
  description: Run `scripts/run_diff_verify.sh` with refreshed redacted scenarios
  status: done
  depends_on: [S2R-T04]

- id: S2R-T06
  description: Run full `scripts/run_regression.sh`
  status: done
  depends_on: [S2R-T05]

- id: S2R-T07
  description: Commit and push runtime evidence and confidence promotion changes
  status: done
  depends_on: [S2R-T06]

## DOC-T0 - Institutionalize continuous documentation

Dependency graph:

- `DOC-T01 -> DOC-T02`
- `DOC-T02 -> DOC-T03`
- `DOC-T03 -> DOC-T04`

Tasks:

- id: DOC-T01
  description: Create/update `AGENTS.md` with mandatory knowledge maintenance rules
  status: done
  depends_on: []

- id: DOC-T02
  description: Publish documentation discipline runbook in Zensical docs
  status: done
  depends_on: [DOC-T01]

- id: DOC-T03
  description: Link documentation discipline from `docs/index.md` as an operational rule
  status: done
  depends_on: [DOC-T02]

- id: DOC-T04
  description: Validate and record that TODO-CODEX/AGENTS/KB are updated each iteration
  status: done
  depends_on: [DOC-T03]

## Stage 3A - Authenticated login + semantic parity

Dependency graph:

- `S3A-T01 -> S3A-T02`
- `S3A-T02 -> S3A-T03`
- `S3A-T03 -> S3A-T04`
- `S3A-T04 -> S3A-T05`
- `S3A-T05 -> S3A-T06`
- `S3A-T06 -> S3A-T07`
- `S3A-T07 -> S3A-T08`

Tasks:

- id: S3A-T01
  description: Determine accepted official login version tuple and record runtime evidence
  status: done
  depends_on: []

- id: S3A-T02
  description: Implement correct login codec (including md5hash) and typed login response parser
  status: done
  depends_on: [S3A-T01]

- id: S3A-T03
  description: Update SessionClient login state machine so `LoggedIn` is set only after real success
  status: done
  depends_on: [S3A-T02]

- id: S3A-T04
  description: Extend CLI/tools for env-based credentials and login version probing
  status: done
  depends_on: [S3A-T03]

- id: S3A-T05
  description: Capture authenticated runtime scenarios and refresh raw -> redacted artifacts
  status: done
  depends_on: [S3A-T04]

- id: S3A-T06
  description: Implement semantic differential verifier while keeping bytes mode compatibility
  status: done
  depends_on: [S3A-T05]

- id: S3A-T07
  description: Update maps/schema/docs/ledger and KB state with authenticated evidence
  status: done
  depends_on: [S3A-T06]

- id: S3A-T08
  description: Stage closure with green regression, documented PR, and maintainability retrospective
  status: done
  depends_on: [S3A-T07]

Notes:

- Authenticated runtime tuple confirmed by probe: `client_version=160`, `minor_version=1`.
- Stateful login validated: `LoggedIn` is only set after `LoginResponsePayload::Success`; failures keep `Connected`.
- `scripts/run_diff_verify.sh` defaults to semantic mode (`VERIFY_MODE=semantic`) while preserving bytes mode fallback.
- Mandatory redacted authenticated scenarios were refreshed (`login-only`, `login-search`, `login-search-download`, `upload-deny`, `upload-accept`).

## Stage 3B - Visual roadmap + Rooms/Presence protocol batch

Dependency graph:

- `S3B-R01 -> S3B-R02`
- `S3B-R02 -> S3B-T01`
- `S3B-T01 -> S3B-T02`
- `S3B-T02 -> S3B-T03`
- `S3B-T03 -> S3B-T04`
- `S3B-T04 -> S3B-T05`
- `S3B-T05 -> S3B-T06`
- `S3B-T06 -> S3B-T07`
- `S3B-T07 -> S3B-T08`
- `S3B-T08 -> S3B-T09`
- `S3B-T08 -> S3B-R03`
- `S3B-R03 -> S3B-T09`

Tasks:

- id: S3B-R01
  description: Create visual roadmap page for Zensical (timeline + dependency + status matrix) in `docs/state/roadmap.md`
  status: done
  depends_on: []

- id: S3B-R02
  description: Sync roadmap baseline with current stage status and link from `docs/index.md`
  status: done
  depends_on: [S3B-R01]

- id: S3B-T01
  description: Resolve code/evidence for 8 Rooms+Presence messages using static + runtime sources and ledger traceability
  status: done
  depends_on: [S3B-R02]

- id: S3B-T02
  description: Capture authenticated runtime room scenarios (room-list, join-presence, leave) and redact artifacts
  status: done
  depends_on: [S3B-T01]

- id: S3B-T03
  description: Update `message_map` and `message_schema` for the 8 new messages with valid evidence links and confidence levels
  status: done
  depends_on: [S3B-T02]

- id: S3B-T04
  description: Implement room and presence protocol codec/types in `rust/protocol`
  status: done
  depends_on: [S3B-T03]

- id: S3B-T05
  description: Extend `SessionClient` with room list/join/leave/event collection in `rust/core`
  status: done
  depends_on: [S3B-T04]

- id: S3B-T06
  description: Add `soul-cli room` commands (`list/join/leave/members/watch`) with summary + verbose modes
  status: done
  depends_on: [S3B-T05]

- id: S3B-T07
  description: Extend semantic differential verifier coverage for room/presence payloads
  status: done
  depends_on: [S3B-T06]

- id: S3B-T08
  description: Run full validation gates (`kb_validate`, semantic diff verify, regression) and fix issues
  status: done
  depends_on: [S3B-T07]

- id: S3B-R03
  description: Refresh roadmap/status docs with executed S3B outcomes and S4 preview readiness
  status: done
  depends_on: [S3B-T08]

- id: S3B-T09
  description: Publish S3B PR doc and retrospective (maintainability/reuse/surface reduction)
  status: done
  depends_on: [S3B-T08, S3B-R03]

Notes:

- Runtime-authenticated S3B runs were generated and redacted: `login-room-list`, `login-join-room-presence`, `login-leave-room`.
- Protocol mapping expanded to `33` rows total with S3B batch confidence `high=8`, `medium=0`, `low=0`.
- CLI room commands were validated against the official server in summary mode (`room list/join/members/watch/leave`).
- Validation gates passed: `python3 scripts/kb_validate.py`, `scripts/run_diff_verify.sh`, `scripts/run_regression.sh`.

## Stage 4A - Recommendations/Discovery protocol batch

Dependency graph:

- `S4A-R01 -> S4A-T01`
- `S4A-T01 -> S4A-T02`
- `S4A-T02 -> S4A-T03`
- `S4A-T03 -> S4A-T04`
- `S4A-T04 -> S4A-T05`
- `S4A-T05 -> S4A-T06`
- `S4A-T06 -> S4A-T07`
- `S4A-T07 -> S4A-T08`
- `S4A-T08 -> S4A-R02`
- `S4A-R02 -> S4A-T09`

Tasks:

- id: S4A-R01
  description: Initialize Stage 4A execution plan and roadmap baseline (mark S4A as active batch)
  status: done
  depends_on: []

- id: S4A-T01
  description: Resolve codes/evidence for recommendation/discovery messages from static and runtime sources
  status: done
  depends_on: [S4A-R01]

- id: S4A-T02
  description: Capture authenticated runtime recommendation/discovery scenarios and redact artifacts
  status: done
  depends_on: [S4A-T01]

- id: S4A-T03
  description: Update message_map and message_schema for S4A messages with evidence links and confidence levels
  status: done
  depends_on: [S4A-T02]

- id: S4A-T04
  description: Implement recommendation/discovery codec and payload types in rust/protocol
  status: done
  depends_on: [S4A-T03]

- id: S4A-T05
  description: Extend SessionClient with recommendation/discovery operations in rust/core
  status: done
  depends_on: [S4A-T04]

- id: S4A-T06
  description: Add CLI discover commands for recommendation/discovery flows (summary + verbose)
  status: done
  depends_on: [S4A-T05]

- id: S4A-T07
  description: Extend semantic differential verification for recommendation/discovery payloads and runs
  status: done
  depends_on: [S4A-T06]

- id: S4A-T08
  description: Run full validation gates and fix any failures (kb_validate, diff_verify, regression)
  status: done
  depends_on: [S4A-T07]

- id: S4A-R02
  description: Refresh roadmap/status/backlog/docs with executed S4A outcomes and next-stage readiness
  status: done
  depends_on: [S4A-T08]

- id: S4A-T09
  description: Publish S4A PR doc with maintainability/reuse retrospective and open PR
  status: done
  depends_on: [S4A-R02]

Notes:

- Runtime-authenticated S4A runs were generated and redacted: `login-recommendations`, `login-user-recommendations`, `login-similar-terms`.
- Protocol mapping expanded to `38` rows total with S4A batch confidence `high=5`, `medium=0`, `low=0`.
- CLI discover commands were validated against the official server in summary mode (`discover recommendations/mine/global/user/similar-terms`).
- Validation gates passed: `python3 scripts/kb_validate.py`, `scripts/run_diff_verify.sh`, `scripts/run_regression.sh`.

## Stage 4B - Peer advanced + room moderation + protocol matrix

Dependency graph:

- `S4B-R01 -> S4B-R02`
- `S4B-R02 -> S4B-T01`
- `S4B-T01 -> S4B-T02`
- `S4B-T02 -> S4B-T03`
- `S4B-T03 -> S4B-T04`
- `S4B-T04 -> S4B-T05`
- `S4B-T05 -> S4B-T06`
- `S4B-T06 -> S4B-T07`
- `S4B-T07 -> S4B-R03`
- `S4B-R03 -> S4B-T08`

Tasks:

- id: S4B-R01
  description: Initialize Stage 4B plan and mark roadmap state for active execution
  status: done
  depends_on: []

- id: S4B-R02
  description: Add canonical protocol matrix generation and publish `docs/state/protocol-matrix.md` (mapped/implemented/missing + purpose)
  status: done
  depends_on: [S4B-R01]

- id: S4B-T01
  description: Resolve evidence and codes for S4B target messages (peer advanced + room moderation) from static/runtime/spec sources
  status: done
  depends_on: [S4B-R02]

- id: S4B-T02
  description: Generate S4B runtime captures (official room moderation attempts + local peer advanced scenarios) and redact artifacts
  status: done
  depends_on: [S4B-T01]

- id: S4B-T03
  description: Update `message_map.csv` and `message_schema.json` for S4B targets with confidence and evidence links
  status: done
  depends_on: [S4B-T02]

- id: S4B-T04
  description: Implement protocol codecs/types/builders for S4B messages in `rust/protocol`
  status: done
  depends_on: [S4B-T03]

- id: S4B-T05
  description: Extend `rust/core` and `rust/cli` for room moderation operations and peer advanced protocol usage
  status: done
  depends_on: [S4B-T04]

- id: S4B-T06
  description: Extend semantic differential verification and required run set for S4B scenarios
  status: done
  depends_on: [S4B-T05]

- id: S4B-T07
  description: Add S4B regression coverage (protocol contract + matrix integrity checks where applicable)
  status: done
  depends_on: [S4B-T06]

- id: S4B-R03
  description: Sync roadmap/project/verification/backlog docs and publish S4B status and outcomes
  status: done
  depends_on: [S4B-T07]

- id: S4B-T08
  description: Run final validation gates, prepare PR document, and open S4B PR
  status: done
  depends_on: [S4B-R03]

Notes:

- Stage 4B protocol matrix is published at `docs/state/protocol-matrix.md` and linked from `docs/index.md`.
- Current matrix snapshot: total tracked `130`, implemented+mapped `47`, missing `83`.
- S4B mapping batch landed with confidence gate satisfied: `high=7`, `medium=2`, `low=0`.
- Validation gates passed: `python3 scripts/kb_validate.py`, `scripts/run_diff_verify.sh`, `scripts/run_regression.sh`.

## Stage 4C - Privileges/social control + peer folder domain

Dependency graph:

- `S4C-R01 -> S4C-T01`
- `S4C-T01 -> S4C-T02`
- `S4C-T02 -> S4C-T03`
- `S4C-T03 -> S4C-T04`
- `S4C-T04 -> S4C-T05`
- `S4C-T05 -> S4C-T06`
- `S4C-T06 -> S4C-T07`
- `S4C-T07 -> S4C-R02`
- `S4C-R02 -> S4C-T08`

Tasks:

- id: S4C-R01
  description: Initialize Stage 4C plan and mark roadmap baseline for active execution
  status: done
  depends_on: []

- id: S4C-T01
  description: Resolve message codes and evidence for S4C target set (privileges/social control + peer folder), deferring unresolved ban mapping if needed
  status: done
  depends_on: [S4C-R01]

- id: S4C-T02
  description: Generate S4C runtime captures (authenticated privilege/social requests + deterministic peer-folder exchange) and redact artifacts
  status: done
  depends_on: [S4C-T01]

- id: S4C-T03
  description: Update `message_map.csv` and `message_schema.json` for S4C targets with confidence and evidence links
  status: done
  depends_on: [S4C-T02]

- id: S4C-T04
  description: Implement protocol codecs/types/builders for S4C messages in `rust/protocol`
  status: done
  depends_on: [S4C-T03]

- id: S4C-T05
  description: Extend `rust/core` and `rust/cli` with privileges/social operations and peer-folder helpers
  status: done
  depends_on: [S4C-T04]

- id: S4C-T06
  description: Extend semantic differential verification and required run set for S4C scenarios
  status: done
  depends_on: [S4C-T05]

- id: S4C-T07
  description: Add S4C regression coverage (protocol contract tests + matrix integrity checks)
  status: done
  depends_on: [S4C-T06]

- id: S4C-R02
  description: Sync roadmap/project/verification/backlog docs with S4C outcomes and next-stage preview
  status: done
  depends_on: [S4C-T07]

- id: S4C-T08
  description: Run final validation gates, publish PR document, and open S4C PR
  status: done
  depends_on: [S4C-R02]

Notes:

- Stage 4C mapping batch landed with confidence gate satisfied: `high=8`, `medium=1`, `low=0`.
- Runtime redacted runs were added for S4C: `login-privileges-social`, `peer-folder-local`.
- Protocol matrix snapshot after S4C: tracked `130`, implemented+mapped `56`, missing `74`.
- `SM_BAN_USER` remains deferred to backlog due unresolved authoritative code/evidence.
- Validation gates passed on final S4C snapshot: `python3 scripts/kb_validate.py`, `scripts/run_diff_verify.sh`, `scripts/run_regression.sh`.

## Stage 4D - Privilege/messaging gaps + peer legacy cleanup

Dependency graph:

- `S4D-R01 -> S4D-T01`
- `S4D-T01 -> S4D-T02`
- `S4D-T02 -> S4D-T03`
- `S4D-T03 -> S4D-T04`
- `S4D-T04 -> S4D-T05`
- `S4D-T05 -> S4D-T06`
- `S4D-T06 -> S4D-T07`
- `S4D-T07 -> S4D-R02`
- `S4D-R02 -> S4D-T08`

Tasks:

- id: S4D-R01
  description: Initialize Stage 4D execution plan and mark roadmap baseline for active execution
  status: done
  depends_on: []

- id: S4D-T01
  description: Resolve authoritative message codes/evidence for privilege/messaging and peer-legacy targets from MessageCodeToString jump tables + protocol spec
  status: done
  depends_on: [S4D-R01]

- id: S4D-T02
  description: Generate Stage 4D runtime captures (authenticated privilege/messaging probes + deterministic peer-legacy frames) and redact artifacts
  status: done
  depends_on: [S4D-T01]

- id: S4D-T03
  description: Update `message_map.csv` and `message_schema.json` with S4D target rows and promote legacy search messages to high confidence with runtime evidence
  status: done
  depends_on: [S4D-T02]

- id: S4D-T04
  description: Implement protocol constants/types/codecs/builders for Stage 4D messages in `rust/protocol`
  status: done
  depends_on: [S4D-T03]

- id: S4D-T05
  description: Extend `rust/core` and `rust/cli` with privilege/messaging operations for Stage 4D server flows
  status: done
  depends_on: [S4D-T04]

- id: S4D-T06
  description: Extend semantic verification required-run set and normalization coverage for Stage 4D runs
  status: done
  depends_on: [S4D-T05]

- id: S4D-T07
  description: Add Stage 4D regression coverage (protocol contract tests + matrix coherence updates)
  status: done
  depends_on: [S4D-T06]

- id: S4D-R02
  description: Sync roadmap/project/verification/backlog docs and matrix with Stage 4D outcomes
  status: done
  depends_on: [S4D-T07]

- id: S4D-T08
  description: Run final validation gates, publish Stage 4D PR document, and prepare PR
  status: done
  depends_on: [S4D-R02]

Notes:

- Stage 4D mapping batch landed with confidence gate satisfied: `high=11`, `medium=0`, `low=0`.
- Runtime redacted runs were added for S4D: `login-privilege-messaging`, `peer-legacy-local`.
- Protocol matrix snapshot after S4D: tracked `130`, implemented+mapped `65`, missing `65`.
- `PM_EXACT_FILE_SEARCH_REQUEST` and `PM_INDIRECT_FILE_SEARCH_REQUEST` were promoted from `medium` to `high` using deterministic runtime evidence.
- Validation gates passed on final S4D snapshot: `python3 scripts/kb_validate.py`, `scripts/run_diff_verify.sh`, `scripts/run_regression.sh`.

## Stage 4E - Private messaging + user-state protocol batch

Dependency graph:

- `S4E-W01 -> S4E-W02`
- `S4E-W02 -> S4E-W03`
- `S4E-W03 -> S4E-T01`
- `S4E-T01 -> S4E-T02`
- `S4E-T02 -> S4E-T03`
- `S4E-T03 -> S4E-T04`
- `S4E-T04 -> S4E-T05`
- `S4E-T05 -> S4E-T06`
- `S4E-T06 -> S4E-T07`
- `S4E-T07 -> S4E-S01`
- `S4E-S01 -> S4E-S02`
- `S4E-S02 -> S4E-T08`
- `S4E-T08 -> S4E-R01`
- `S4E-R01 -> S4E-Q01`
- `S4E-Q01 -> S4E-Q02`
- `S4E-Q02 -> S4E-Q03`
- `S4E-Q03 -> S4E-Q04`
- `S4E-Q04 -> S4E-T09`

Tasks:

- id: S4E-W01
  description: Start from updated main and create branch `codex/s4e-private-messaging-user-state`
  status: done
  depends_on: []

- id: S4E-W02
  description: Verify commit signing setup for verified commits before first stage push
  status: done
  depends_on: [S4E-W01]

- id: S4E-W03
  description: Add mandatory two-round `@codex review` loop rule to `AGENTS.md` for all future stages
  status: done
  depends_on: [S4E-W02]

- id: S4E-T01
  description: Resolve authoritative code/payload evidence for S4E message batch (including `SM_PEER_MESSAGE` code conflict handling)
  status: done
  depends_on: [S4E-W03]

- id: S4E-T02
  description: Generate authenticated runtime captures for private messaging and user-state scenarios and redact artifacts
  status: done
  depends_on: [S4E-T01]

- id: S4E-T03
  description: Update `message_map`/`message_schema`/evidence ledger and regenerate protocol matrix for S4E
  status: done
  depends_on: [S4E-T02]

- id: S4E-T04
  description: Implement protocol constants/types/codecs/builders for S4E messages in `rust/protocol`
  status: done
  depends_on: [S4E-T03]

- id: S4E-T05
  description: Extend `SessionClient` with private messaging and user-state typed operations in `rust/core`
  status: done
  depends_on: [S4E-T04]

- id: S4E-T06
  description: Add session messaging/status/stats/peer-address/connect-peer/watch-private commands in `rust/cli`
  status: done
  depends_on: [S4E-T05]

- id: S4E-T07
  description: Extend semantic verifier coverage for S4E payloads and compatibility aliases
  status: done
  depends_on: [S4E-T06]

- id: S4E-S01
  description: Apply security-best-practices pass (adapted) on touched protocol/core/cli/runtime paths
  status: done
  depends_on: [S4E-T07]

- id: S4E-S02
  description: Apply code-simplifier pass on touched Rust files to reduce complexity without behavior changes
  status: done
  depends_on: [S4E-S01]

- id: S4E-T08
  description: Add/update protocol contract tests, runtime redaction tests, and stage regression checks
  status: done
  depends_on: [S4E-S02]

- id: S4E-R01
  description: Sync roadmap/status/backlog/TODO/PR doc and rebuild Zensical docs
  status: done
  depends_on: [S4E-T08]

- id: S4E-Q01
  description: Open S4E PR and post first `@codex review` request comment
  status: done
  depends_on: [S4E-R01]

- id: S4E-Q02
  description: Triage first Codex review, apply useful fixes, dismiss non-useful comments with rationale, and resolve threads
  status: done
  depends_on: [S4E-Q01]

- id: S4E-Q03
  description: Post second `@codex review` request comment after round-one changes are pushed
  status: done
  depends_on: [S4E-Q02]

- id: S4E-Q04
  description: Triage second Codex review, apply or dismiss with rationale, and resolve all threads
  status: done
  depends_on: [S4E-Q03]

- id: S4E-T09
  description: Run final gates, confirm green status, and prepare merge-ready PR with retrospective
  status: done
  depends_on: [S4E-Q04]

Notes:

- Stage 4E mapping batch landed with confidence gate satisfied: `high=8`, `medium=0`, `low=0`.
- Runtime redacted runs were added for S4E: `login-private-message`, `login-user-state`, `login-peer-address-connect`, `login-message-users`, `login-peer-message`.
- Protocol matrix snapshot after S4E: tracked `131`, implemented+mapped `67`, missing `63`, implemented-not-mapped `1` (`SM_PEER_MESSAGE_ALT` compatibility alias).
- PR review-loop process was executed with two `@codex review` requests; no automated Codex feedback was produced during the execution window, and closure proceeded with green validation gates.
- Validation gates passed on final S4E snapshot: `python3 scripts/kb_validate.py`, `scripts/run_diff_verify.sh`, `scripts/run_regression.sh`, `./.venv-tools/bin/zensical build -f zensical.toml`.

## Stage 4F - Global/admin/distributed control mapping batch

Dependency graph:

- `S4F-W01 -> S4F-T01`
- `S4F-T01 -> S4F-T02`
- `S4F-T02 -> S4F-T03`
- `S4F-T03 -> S4F-T04`
- `S4F-T04 -> S4F-T05`
- `S4F-T05 -> S4F-R01`
- `S4F-R01 -> S4F-Q01`
- `S4F-Q01 -> S4F-Q02`
- `S4F-Q02 -> S4F-Q03`
- `S4F-Q03 -> S4F-Q04`
- `S4F-Q04 -> S4F-T06`

Tasks:

- id: S4F-W01
  description: Start from updated main and create branch `codex/s4f-global-admin-distributed-map`
  status: done
  depends_on: []

- id: S4F-T01
  description: Resolve and register authoritative codes/evidence for S4F mapping batch from jump-table extraction
  status: done
  depends_on: [S4F-W01]

- id: S4F-T02
  description: Update `message_map.csv` and regenerate `message_schema.json` for S4F rows
  status: done
  depends_on: [S4F-T01]

- id: S4F-T03
  description: Sync detangling/ledger/schema docs and regenerate protocol matrix
  status: done
  depends_on: [S4F-T02]

- id: S4F-T04
  description: Update roadmap/backlog/project/verification/decompilation status docs for S4F completion and S4G preview
  status: done
  depends_on: [S4F-T03]

- id: S4F-T05
  description: Run validation gates (`kb_validate`, regression, zensical build)
  status: done
  depends_on: [S4F-T04]

- id: S4F-R01
  description: Publish PR doc for S4F and stage closure notes
  status: done
  depends_on: [S4F-T05]

- id: S4F-Q01
  description: Open S4F PR and request first `@codex review`
  status: done
  depends_on: [S4F-R01]

- id: S4F-Q02
  description: Triage/apply useful feedback from round 1 and resolve/dismiss comments with rationale
  status: done
  depends_on: [S4F-Q01]

- id: S4F-Q03
  description: Request second `@codex review` after round-1 updates
  status: done
  depends_on: [S4F-Q02]

- id: S4F-Q04
  description: Triage/apply useful feedback from round 2 and resolve/dismiss comments with rationale
  status: done
  depends_on: [S4F-Q03]

- id: S4F-T06
  description: Merge-ready closure with final status synchronization
  status: done
  depends_on: [S4F-Q04]

Notes:

- Stage 4F mapping batch landed with confidence gate satisfied: `high=8`, `medium=0`, `low=0`.
- Protocol matrix snapshot after S4F: tracked `131`, implemented+mapped `67`, mapped-not-implemented `8`, missing `55`.
- S4F was executed as mapping-first with authoritative static evidence from `message_codes_jump_table.md`; typed protocol/core/cli implementation is scheduled for S4G+.
- Two `@codex review` requests were posted on PR #9 and no automated review comments were produced during the execution window.

## Stage 4G - Parent/distributed tuning mapping continuation

Dependency graph:

- `S4G-W01 -> S4G-T01`
- `S4G-T01 -> S4G-T02`
- `S4G-T02 -> S4G-T03`
- `S4G-T03 -> S4G-T04`
- `S4G-T04 -> S4G-T05`
- `S4G-T05 -> S4G-R01`
- `S4G-R01 -> S4G-Q01`
- `S4G-Q01 -> S4G-Q02`
- `S4G-Q02 -> S4G-Q03`
- `S4G-Q03 -> S4G-Q04`
- `S4G-Q04 -> S4G-T06`

Tasks:

- id: S4G-W01
  description: Start from updated main and create branch `codex/s4g-parent-distributed-tuning-map`
  status: done
  depends_on: []

- id: S4G-T01
  description: Resolve and register authoritative codes/evidence for S4G mapping batch from jump-table extraction
  status: done
  depends_on: [S4G-W01]

- id: S4G-T02
  description: Update `message_map.csv` and regenerate `message_schema.json` for S4G rows
  status: done
  depends_on: [S4G-T01]

- id: S4G-T03
  description: Sync detangling/ledger/schema docs and regenerate protocol matrix
  status: done
  depends_on: [S4G-T02]

- id: S4G-T04
  description: Update roadmap/backlog/project/verification/decompilation status docs for S4G completion and S4H preview
  status: done
  depends_on: [S4G-T03]

- id: S4G-T05
  description: Run validation gates (`kb_validate`, regression, zensical build)
  status: done
  depends_on: [S4G-T04]

- id: S4G-R01
  description: Publish PR doc for S4G and stage closure notes
  status: done
  depends_on: [S4G-T05]

- id: S4G-Q01
  description: Open S4G PR and request first `@codex review`
  status: done
  depends_on: [S4G-R01]

- id: S4G-Q02
  description: Triage/apply useful feedback from round 1 and resolve/dismiss comments with rationale
  status: done
  depends_on: [S4G-Q01]

- id: S4G-Q03
  description: Request second `@codex review` after round-1 updates
  status: done
  depends_on: [S4G-Q02]

- id: S4G-Q04
  description: Triage/apply useful feedback from round 2 and resolve/dismiss comments with rationale
  status: done
  depends_on: [S4G-Q03]

- id: S4G-T06
  description: Merge-ready closure with final status synchronization
  status: done
  depends_on: [S4G-Q04]

Notes:

- Stage 4G mapping batch landed with confidence gate satisfied: `high=8`, `medium=0`, `low=0`.
- Protocol matrix snapshot after S4G: tracked `131`, implemented+mapped `67`, mapped-not-implemented `16`, missing `47`.
- S4G was executed as mapping-first with authoritative static evidence from `message_codes_jump_table.md`; typed protocol/core/cli implementation is scheduled for S4H+.
- Two `@codex review` requests were posted on PR #10 and no automated review comments were produced during the execution window.

## Stage 4H - Global room/system control mapping continuation

Dependency graph:

- `S4H-W01 -> S4H-T01`
- `S4H-T01 -> S4H-T02`
- `S4H-T02 -> S4H-T03`
- `S4H-T03 -> S4H-T04`
- `S4H-T04 -> S4H-T05`
- `S4H-T05 -> S4H-R01`
- `S4H-R01 -> S4H-Q01`
- `S4H-Q01 -> S4H-Q02`
- `S4H-Q02 -> S4H-Q03`
- `S4H-Q03 -> S4H-Q04`
- `S4H-Q04 -> S4H-T06`

Tasks:

- id: S4H-W01
  description: Start from updated main and create branch `codex/s4h-global-system-control-map`
  status: done
  depends_on: []

- id: S4H-T01
  description: Resolve and register authoritative codes/evidence for S4H mapping batch from jump-table extraction
  status: done
  depends_on: [S4H-W01]

- id: S4H-T02
  description: Update `message_map.csv` and regenerate `message_schema.json` for S4H rows
  status: done
  depends_on: [S4H-T01]

- id: S4H-T03
  description: Sync detangling/ledger/schema docs and regenerate protocol matrix
  status: done
  depends_on: [S4H-T02]

- id: S4H-T04
  description: Update roadmap/backlog/project/verification/decompilation status docs for S4H completion and S4I preview
  status: done
  depends_on: [S4H-T03]

- id: S4H-T05
  description: Run validation gates (`kb_validate`, regression, zensical build)
  status: done
  depends_on: [S4H-T04]

- id: S4H-R01
  description: Publish PR doc for S4H and stage closure notes
  status: done
  depends_on: [S4H-T05]

- id: S4H-Q01
  description: Open S4H PR and request first `@codex review`
  status: done
  depends_on: [S4H-R01]

- id: S4H-Q02
  description: Triage/apply useful feedback from round 1 and resolve/dismiss comments with rationale
  status: done
  depends_on: [S4H-Q01]

- id: S4H-Q03
  description: Request second `@codex review` after round-1 updates
  status: done
  depends_on: [S4H-Q02]

- id: S4H-Q04
  description: Triage/apply useful feedback from round 2 and resolve/dismiss comments with rationale
  status: done
  depends_on: [S4H-Q03]

- id: S4H-T06
  description: Merge-ready closure with final status synchronization
  status: done
  depends_on: [S4H-Q04]

Notes:

- Stage 4H mapping batch landed with confidence gate satisfied: `high=8`, `medium=0`, `low=0`.
- Protocol matrix snapshot after S4H: tracked `131`, implemented+mapped `67`, mapped-not-implemented `24`, missing `39`.
- S4H was executed as mapping-first with authoritative static evidence from `message_codes_jump_table.md`; typed protocol/core/cli implementation is scheduled for S4I+.
- Two `@codex review` requests were posted on PR #11 and the connector reported usage-limit responses without actionable findings.

## Stage 4I - Ticker/term control mapping continuation

Dependency graph:

- `S4I-W01 -> S4I-T01`
- `S4I-T01 -> S4I-T02`
- `S4I-T02 -> S4I-T03`
- `S4I-T03 -> S4I-T04`
- `S4I-T04 -> S4I-T05`
- `S4I-T05 -> S4I-R01`
- `S4I-R01 -> S4I-Q01`
- `S4I-Q01 -> S4I-Q02`
- `S4I-Q02 -> S4I-Q03`
- `S4I-Q03 -> S4I-Q04`
- `S4I-Q04 -> S4I-T06`

Tasks:

- id: S4I-W01
  description: Start from updated main and create branch `codex/s4i-ticker-term-control-map`
  status: done
  depends_on: []

- id: S4I-T01
  description: Resolve and register authoritative codes/evidence for S4I mapping batch from jump-table extraction
  status: done
  depends_on: [S4I-W01]

- id: S4I-T02
  description: Update `message_map.csv` and regenerate `message_schema.json` for S4I rows
  status: done
  depends_on: [S4I-T01]

- id: S4I-T03
  description: Sync detangling/ledger/schema docs and regenerate protocol matrix
  status: done
  depends_on: [S4I-T02]

- id: S4I-T04
  description: Update roadmap/backlog/project/verification/decompilation status docs for S4I completion and S4J preview
  status: done
  depends_on: [S4I-T03]

- id: S4I-T05
  description: Run validation gates (`kb_validate`, regression, zensical build)
  status: done
  depends_on: [S4I-T04]

- id: S4I-R01
  description: Publish PR doc for S4I and stage closure notes
  status: done
  depends_on: [S4I-T05]

- id: S4I-Q01
  description: Open S4I PR and request first `@codex review`
  status: done
  depends_on: [S4I-R01]

- id: S4I-Q02
  description: Triage/apply useful feedback from round 1 and resolve/dismiss comments with rationale
  status: done
  depends_on: [S4I-Q01]

- id: S4I-Q03
  description: Request second `@codex review` after round-1 updates
  status: done
  depends_on: [S4I-Q02]

- id: S4I-Q04
  description: Triage/apply useful feedback from round 2 and resolve/dismiss comments with rationale
  status: done
  depends_on: [S4I-Q03]

- id: S4I-T06
  description: Merge-ready closure with final status synchronization
  status: done
  depends_on: [S4I-Q04]

Notes:

- Stage 4I mapping batch landed with confidence gate satisfied: `high=8`, `medium=0`, `low=0`.
- Protocol matrix snapshot after S4I: tracked `131`, implemented+mapped `67`, mapped-not-implemented `32`, missing `31`.
- S4I was executed as mapping-first with authoritative static evidence from `message_codes_jump_table.md`; typed protocol/core/cli implementation is scheduled for S4J+.
- Two `@codex review` requests were posted on PR #12 and the connector reported usage-limit responses without actionable findings.

## Stage 4J - Private-room ownership/membership mapping continuation

Dependency graph:

- `S4J-W01 -> S4J-T01`
- `S4J-T01 -> S4J-T02`
- `S4J-T02 -> S4J-T03`
- `S4J-T03 -> S4J-T04`
- `S4J-T04 -> S4J-T05`
- `S4J-T05 -> S4J-R01`
- `S4J-R01 -> S4J-Q01`
- `S4J-Q01 -> S4J-Q02`
- `S4J-Q02 -> S4J-Q03`
- `S4J-Q03 -> S4J-Q04`
- `S4J-Q04 -> S4J-T06`

Tasks:

- id: S4J-W01
  description: Start from updated main and create branch `codex/s4j-private-room-ownership-map`
  status: done
  depends_on: []

- id: S4J-T01
  description: Resolve and register authoritative codes/evidence for S4J mapping batch from jump-table extraction
  status: done
  depends_on: [S4J-W01]

- id: S4J-T02
  description: Update `message_map.csv` and regenerate `message_schema.json` for S4J rows
  status: done
  depends_on: [S4J-T01]

- id: S4J-T03
  description: Sync detangling/ledger/schema docs and regenerate protocol matrix
  status: done
  depends_on: [S4J-T02]

- id: S4J-T04
  description: Update roadmap/backlog/project/verification/decompilation status docs for S4J completion and S4K preview
  status: done
  depends_on: [S4J-T03]

- id: S4J-T05
  description: Run validation gates (`kb_validate`, regression, zensical build)
  status: done
  depends_on: [S4J-T04]

- id: S4J-R01
  description: Publish PR doc for S4J and stage closure notes
  status: done
  depends_on: [S4J-T05]

- id: S4J-Q01
  description: Open S4J PR and request first `@codex review`
  status: done
  depends_on: [S4J-R01]

- id: S4J-Q02
  description: Triage/apply useful feedback from round 1 and resolve/dismiss comments with rationale
  status: done
  depends_on: [S4J-Q01]

- id: S4J-Q03
  description: Request second `@codex review` after round-1 updates
  status: done
  depends_on: [S4J-Q02]

- id: S4J-Q04
  description: Triage/apply useful feedback from round 2 and resolve/dismiss comments with rationale
  status: done
  depends_on: [S4J-Q03]

- id: S4J-T06
  description: Merge-ready closure with final status synchronization
  status: done
  depends_on: [S4J-Q04]

Notes:

- Stage 4J mapping batch landed with confidence gate satisfied: `high=8`, `medium=0`, `low=0`.
- Protocol matrix snapshot after S4J: tracked `131`, implemented+mapped `67`, mapped-not-implemented `40`, missing `23`.
- S4J was executed as mapping-first with authoritative static evidence from `message_codes_jump_table.md`; typed protocol/core/cli implementation is scheduled for S4K+.
- Two `@codex review` requests were posted on PR #13 and the connector reported usage-limit responses without actionable findings.

## Stage 4K - Missing-code closure + global/distributed peer-control protocol implementation

Dependency graph:

- `S4K-W01 -> S4K-W02`
- `S4K-W02 -> S4K-T01`
- `S4K-T01 -> S4K-T02`
- `S4K-T02 -> S4K-T03`
- `S4K-T03 -> S4K-T04`
- `S4K-T04 -> S4K-T05`
- `S4K-T05 -> S4K-T06`
- `S4K-T06 -> S4K-R01`
- `S4K-R01 -> S4K-Q01`
- `S4K-Q01 -> S4K-Q02`
- `S4K-Q02 -> S4K-Q03`
- `S4K-Q03 -> S4K-Q04`
- `S4K-Q04 -> S4K-T07`

Tasks:

- id: S4K-W01
  description: Start from updated main and create branch `codex/s4k-global-peer-control-implementation`
  status: done
  depends_on: []

- id: S4K-W02
  description: Verify commit signing config is active before first stage push
  status: done
  depends_on: [S4K-W01]

- id: S4K-T01
  description: Resolve authoritative code/evidence for all currently missing protocol names using jump-table outputs
  status: done
  depends_on: [S4K-W02]

- id: S4K-T02
  description: Update `message_map.csv` and regenerate `message_schema.json` to close the missing set and map `SM_PEER_MESSAGE_ALT`
  status: done
  depends_on: [S4K-T01]

- id: S4K-T03
  description: Implement `rust/protocol` constants/types/codecs/builders for the S4K closure batch (server+peer)
  status: done
  depends_on: [S4K-T02]

- id: S4K-T04
  description: Add protocol regression tests covering S4K message roundtrips and compatibility decode behavior
  status: done
  depends_on: [S4K-T03]

- id: S4K-T05
  description: Sync KB artifacts/docs (`detangling`, evidence ledger, schema docs, matrix, status, backlog, roadmap)
  status: done
  depends_on: [S4K-T04]

- id: S4K-T06
  description: Run validation gates (`kb_validate`, diff verify semantic, regression, zensical build)
  status: done
  depends_on: [S4K-T05]

- id: S4K-R01
  description: Publish stage PR document under `docs/pr/0015-s4k-missing-code-closure-protocol-implementation.md`
  status: done
  depends_on: [S4K-T06]

- id: S4K-Q01
  description: Open S4K PR and request first `@codex review`
  status: done
  depends_on: [S4K-R01]

- id: S4K-Q02
  description: Triage first review, apply useful fixes, dismiss non-useful comments with rationale, resolve threads
  status: done
  depends_on: [S4K-Q01]

- id: S4K-Q03
  description: Request second `@codex review` after pushing round-one updates
  status: done
  depends_on: [S4K-Q02]

- id: S4K-Q04
  description: Triage second review, apply or dismiss with rationale, resolve all threads
  status: done
  depends_on: [S4K-Q03]

- id: S4K-T07
  description: Finalize merge-ready stage closure and sync TODO statuses
  status: done
  depends_on: [S4K-Q04]

Notes:

- S4K target in this iteration: close the full `missing` bucket by mapping and implementing all currently unresolved names (server + peer) with jump-table-backed evidence and typed/opaque payload handling in `rust/protocol`.
- Runtime captures are best-effort for this batch; static evidence remains authoritative where runtime is not yet reachable.
- Commit signing configuration check passed before first push (`commit.gpgsign=true`, `gpg.format=ssh`, `user.signingkey=/Users/void_rsk/.ssh/nss_signing_ed25519.pub`).
- Validation gates passed:
  - `python3 scripts/kb_validate.py`
  - `scripts/run_diff_verify.sh`
  - `scripts/run_regression.sh`
  - `./.venv-tools/bin/zensical build -f zensical.toml`
- Protocol matrix snapshot after S4K: tracked `131`, implemented+mapped `91`, mapped-not-implemented `40`, missing `0`.
- Review-loop gate completed on PR #14 with two `@codex review` requests; connector returned usage-limit notices and no actionable findings in both rounds.

## Stage 4L - mapped-not-implemented closure to full protocol coverage

Dependency graph:

- `S4L-W01 -> S4L-W02`
- `S4L-W02 -> S4L-T01`
- `S4L-T01 -> S4L-T02`
- `S4L-T02 -> S4L-T03`
- `S4L-T03 -> S4L-T04`
- `S4L-T04 -> S4L-T05`
- `S4L-T05 -> S4L-T06`
- `S4L-T06 -> S4L-R01`
- `S4L-R01 -> S4L-Q01`
- `S4L-Q01 -> S4L-Q02`
- `S4L-Q02 -> S4L-Q03`
- `S4L-Q03 -> S4L-Q04`
- `S4L-Q04 -> S4L-T07`

Tasks:

- id: S4L-W01
  description: Start from updated main and create branch `codex/s4l-mapped-not-implemented-closure`
  status: done
  depends_on: []

- id: S4L-W02
  description: Verify commit signing config is active before first stage push
  status: done
  depends_on: [S4L-W01]

- id: S4L-T01
  description: Resolve authoritative code/evidence set for all remaining mapped-not-implemented rows (40)
  status: done
  depends_on: [S4L-W02]

- id: S4L-T02
  description: Implement `rust/protocol` constants and decode/encode support for all remaining mapped-not-implemented server messages
  status: done
  depends_on: [S4L-T01]

- id: S4L-T03
  description: Add protocol regression tests covering closure-set decode/encode behavior
  status: done
  depends_on: [S4L-T02]

- id: S4L-T04
  description: Regenerate schema/docs/matrix and sync KB artifacts (`message_schema`, `message-schema`, `detangling`, `evidence-ledger`)
  status: done
  depends_on: [S4L-T03]

- id: S4L-T05
  description: Update stage/state docs (`project-status`, `verification-status`, `decompilation-status`, `roadmap`, `protocol-backlog`) for full implemented+mapped closure
  status: done
  depends_on: [S4L-T04]

- id: S4L-T06
  description: Run validation gates (`kb_validate`, diff verify semantic, regression, zensical build)
  status: done
  depends_on: [S4L-T05]

- id: S4L-R01
  description: Publish stage PR document under `docs/pr/0016-s4l-mapped-not-implemented-closure.md`
  status: done
  depends_on: [S4L-T06]

- id: S4L-Q01
  description: Open S4L PR and complete local review loop round 1 (security pass + code-simplifier pass)
  status: done
  depends_on: [S4L-R01]

- id: S4L-Q02
  description: Triage round-1 findings, apply useful fixes, and document rationale for non-applied suggestions
  status: done
  depends_on: [S4L-Q01]

- id: S4L-Q03
  description: Complete local review loop round 2 after pushing round-one updates
  status: done
  depends_on: [S4L-Q02]

- id: S4L-Q04
  description: Triage round-2 findings, apply or dismiss with rationale, and finalize review notes
  status: done
  depends_on: [S4L-Q03]

- id: S4L-T07
  description: Finalize merge-ready stage closure and sync TODO statuses
  status: done
  depends_on: [S4L-Q04]

Notes:

- S4L target in this iteration: close all remaining `mapped_not_implemented` rows so the protocol matrix reaches `implemented+mapped=131`, `mapped_not_implemented=0`, `missing=0`.
- For uncertain runtime-shape payloads, preserve correctness with explicit opaque decode/encode while keeping code-based traceability.
- Commit signing configuration check passed before first push (`commit.gpgsign=true`, `user.signingkey=/Users/void_rsk/.ssh/nss_signing_ed25519.pub`).
- Security review pass completed on touched protocol/verify paths; no secret-handling or unsafe-path regressions found.
- Code-simplifier pass completed by extracting shared `OPAQUE_SERVER_CONTROL_CODES` to remove duplicate control-code lists.
- Validation gates passed:
  - `python3 scripts/kb_validate.py`
  - `scripts/run_diff_verify.sh`
  - `scripts/run_regression.sh`
  - `./.venv-tools/bin/zensical build -f zensical.toml`
- Workflow update: `@codex review` dependency removed from PR loop due availability constraints; stage review loops now run locally (security + simplifier).
- PR opened: `https://github.com/fedejinich/nss/pull/15`.
- Local review loops completed with no additional blocking findings; stage is merge-ready after green validation gates.

## Stage 5A - typed runtime hardening for opaque control payloads

Dependency graph:

- `S5A-W01 -> S5A-W02`
- `S5A-W02 -> S5A-T01`
- `S5A-T01 -> S5A-T02`
- `S5A-T02 -> S5A-T03`
- `S5A-T03 -> S5A-T04`
- `S5A-T04 -> S5A-T05`
- `S5A-T05 -> S5A-T06`
- `S5A-T06 -> S5A-T07`
- `S5A-T07 -> S5A-S01`
- `S5A-S01 -> S5A-S02`
- `S5A-S02 -> S5A-T08`
- `S5A-T08 -> S5A-R01`
- `S5A-R01 -> S5A-Q01`
- `S5A-Q01 -> S5A-Q02`
- `S5A-Q02 -> S5A-Q03`
- `S5A-Q03 -> S5A-Q04`
- `S5A-Q04 -> S5A-T09`

Tasks:

- id: S5A-W01
  description: Start from updated main and create branch `codex/s5a-typed-runtime-hardening`
  status: done
  depends_on: []

- id: S5A-W02
  description: Verify commit-signing setup is active before first stage push
  status: done
  depends_on: [S5A-W01]

- id: S5A-T01
  description: Build runtime evidence inventory for opaque-control candidates and select promotable messages with authenticated captures
  status: done
  depends_on: [S5A-W02]

- id: S5A-T02
  description: Promote selected S5A messages in `message_map.csv` to runtime-backed entries and update evidence ledger/detangling notes
  status: done
  depends_on: [S5A-T01]

- id: S5A-T03
  description: Replace opaque protocol decoding for selected S5A messages with typed payloads/codecs/builders in `rust/protocol`
  status: done
  depends_on: [S5A-T02]

- id: S5A-T04
  description: Extend `rust/core` typed operations/events for S5A messages
  status: done
  depends_on: [S5A-T03]

- id: S5A-T05
  description: Extend `rust/cli` command surface and summaries for S5A typed messages
  status: done
  depends_on: [S5A-T04]

- id: S5A-T06
  description: Add/update protocol/core/verify tests for S5A typed payload behavior and semantic parity
  status: done
  depends_on: [S5A-T05]

- id: S5A-T07
  description: Regenerate schema/matrix/docs and synchronize state artifacts for S5A
  status: done
  depends_on: [S5A-T06]

- id: S5A-S01
  description: Run security best-practices pass on touched protocol/core/cli/verify paths
  status: done
  depends_on: [S5A-T07]

- id: S5A-S02
  description: Run code-simplifier pass on touched Rust files with behavior-preserving cleanup
  status: done
  depends_on: [S5A-S01]

- id: S5A-T08
  description: Run validation gates (`kb_validate`, diff verify semantic, regression, zensical build) and fix any failures
  status: done
  depends_on: [S5A-S02]

- id: S5A-R01
  description: Publish stage PR document under `docs/pr/0017-s5a-typed-runtime-hardening.md`
  status: done
  depends_on: [S5A-T08]

- id: S5A-Q01
  description: Open S5A PR and complete local review loop round 1 (security + code-simplifier notes)
  status: done
  depends_on: [S5A-R01]

- id: S5A-Q02
  description: Triage round-1 findings, apply useful changes, and document rationale for rejected suggestions
  status: done
  depends_on: [S5A-Q01]

- id: S5A-Q03
  description: Complete local review loop round 2 after pushing round-1 updates
  status: done
  depends_on: [S5A-Q02]

- id: S5A-Q04
  description: Triage round-2 findings, apply/dismiss with rationale, and finalize review notes
  status: done
  depends_on: [S5A-Q03]

- id: S5A-T09
  description: Finalize merge-ready S5A closure and sync TODO statuses
  status: done
  depends_on: [S5A-Q04]

Notes:

- S5A target in this iteration: replace opaque decode/encode paths with typed payload handling for runtime-observed control messages from authenticated captures.
- Baseline preservation gate: keep protocol matrix at `implemented+mapped=131`, `mapped-not-implemented=0`, `missing=0`.
- Primary runtime-backed S5A candidates: `SM_SET_PARENT_MIN_SPEED (83)`, `SM_SET_PARENT_SPEED_CONNECTION_RATIO (84)`, `SM_GET_ROOM_TICKER (113)`.
- Runtime closure completed with new run: `captures/redacted/login-parent-distributed-control`.
- Residual hypotheses closed:
  - `SM_UPLOAD_SPEED (121)` promoted to `verified_runtime`.
  - `SM_GET_USER_PRIVILEGES_STATUS (122)` promoted `medium -> high` with authenticated request/response evidence.
- `PM_SHARED_FILES_IN_FOLDER` now has decompression-aware parsing with a bounded safety limit and regression coverage.
- All S5A gates are green:
  - `python3 scripts/kb_validate.py`
  - `scripts/run_diff_verify.sh`
  - `scripts/run_regression.sh`
  - `./.venv-tools/bin/zensical build -f zensical.toml`
- PR opened: `https://github.com/fedejinich/nss/pull/17`.
- PR merged to `main`: `https://github.com/fedejinich/nss/pull/17` (`merge commit 81213d01165d17a502b43c8220efc41c729a9905`).

## Stage 5C - typed runtime hardening wave 2 (global/distributed remainder)

Dependency graph:

- `S5C-W01 -> S5C-W02`
- `S5C-W02 -> S5C-T01`
- `S5C-T01 -> S5C-T02`
- `S5C-T02 -> S5C-T03`
- `S5C-T03 -> S5C-T04`
- `S5C-T04 -> S5C-T05`
- `S5C-T05 -> S5C-T06`
- `S5C-T06 -> S5C-T07`
- `S5C-T07 -> S5C-S01`
- `S5C-S01 -> S5C-S02`
- `S5C-S02 -> S5C-T08`
- `S5C-T08 -> S5C-R01`
- `S5C-R01 -> S5C-Q01`
- `S5C-Q01 -> S5C-Q02`
- `S5C-Q02 -> S5C-Q03`
- `S5C-Q03 -> S5C-Q04`
- `S5C-Q04 -> S5C-T09`

Tasks:

- id: S5C-W01
  description: Start from updated `main` and create branch `codex/s5c-typed-runtime-hardening-wave2`
  status: done
  depends_on: []

- id: S5C-W02
  description: Verify commit-signing setup is active before first stage push
  status: done
  depends_on: [S5C-W01]

- id: S5C-T01
  description: Build runtime evidence inventory for remaining opaque global/distributed controls and select promotable wave-2 subset
  status: done
  depends_on: [S5C-W02]

- id: S5C-T02
  description: Generate authenticated runtime capture `login-room-term-control` and redact artifacts
  status: done
  depends_on: [S5C-T01]

- id: S5C-T03
  description: Promote selected S5C message rows in `message_map.csv` with runtime-backed evidence
  status: done
  depends_on: [S5C-T02]

- id: S5C-T04
  description: Replace selected opaque protocol decode paths with typed payloads/codecs/builders in `rust/protocol`
  status: done
  depends_on: [S5C-T03]

- id: S5C-T05
  description: Extend `rust/core` and `rust/cli` command surface for S5C typed control operations
  status: done
  depends_on: [S5C-T04]

- id: S5C-T06
  description: Add/update protocol/core/verify/tests for S5C typed behavior and semantic parity
  status: done
  depends_on: [S5C-T05]

- id: S5C-T07
  description: Regenerate schema/matrix/docs and synchronize KB/state artifacts for S5C
  status: done
  depends_on: [S5C-T06]

- id: S5C-S01
  description: Run security best-practices pass on touched protocol/core/cli/verify paths
  status: done
  depends_on: [S5C-T07]

- id: S5C-S02
  description: Run code-simplifier pass on touched Rust files with behavior-preserving cleanup
  status: done
  depends_on: [S5C-S01]

- id: S5C-T08
  description: Run validation gates (`kb_validate`, diff verify semantic, regression, zensical build) and fix failures
  status: done
  depends_on: [S5C-S02]

- id: S5C-R01
  description: Publish stage PR document under `docs/pr/0018-s5c-typed-runtime-hardening-wave2.md`
  status: done
  depends_on: [S5C-T08]

- id: S5C-Q01
  description: Open S5C PR and complete local review loop round 1 (security + code-simplifier notes)
  status: done
  depends_on: [S5C-R01]

- id: S5C-Q02
  description: Triage round-1 findings, apply useful changes, and document rationale for rejected suggestions
  status: done
  depends_on: [S5C-Q01]

- id: S5C-Q03
  description: Complete local review loop round 2 after pushing round-1 updates
  status: done
  depends_on: [S5C-Q02]

- id: S5C-Q04
  description: Triage round-2 findings, apply/dismiss with rationale, and finalize review notes
  status: done
  depends_on: [S5C-Q03]

- id: S5C-T09
  description: Finalize merge-ready S5C closure and sync TODO statuses
  status: done
  depends_on: [S5C-Q04]

Notes:

- S5C target subset for this wave: `SM_ADD_CHATROOM (10)`, `SM_ADD_LIKE_TERM (51)`, `SM_REMOVE_LIKE_TERM (52)`.
- Runtime evidence inventory shows these codes in existing capture corpus, making them high-confidence candidates for typed promotion in this wave.
- Baseline preservation gate: keep protocol matrix at `implemented+mapped=131`, `mapped-not-implemented=0`, `missing=0`.
- New authenticated run generated and redacted: `captures/redacted/login-room-term-control`.
- S5C map/schema promotion is complete with runtime-backed sources for codes `10`, `51`, and `52`.
- S5C command surface now includes:
  - `soul-cli room add --room ...`
  - `soul-cli discover add-like-term --term ...`
  - `soul-cli discover remove-like-term --term ...`
- Stage gates are green:
  - `python3 scripts/kb_validate.py`
  - `scripts/run_diff_verify.sh`
  - `scripts/run_regression.sh`
  - `./.venv-tools/bin/zensical build -f zensical.toml`
- PR opened: `https://github.com/fedejinich/nss/pull/18`.
- PR merged to `main`: `https://github.com/fedejinich/nss/pull/18` (`merge commit b3796fe`).

## Stage 5B - Soulseek UI + functionality exhaustive audit (worktree isolated)

Dependency graph:

- `S5B-W01 -> S5B-W02`
- `S5B-W02 -> S5B-T01`
- `S5B-T01 -> S5B-T02`
- `S5B-T01 -> S5B-T03`
- `S5B-T01 -> S5B-T04`
- `S5B-T02 -> S5B-T05`
- `S5B-T03 -> S5B-T05`
- `S5B-T04 -> S5B-T05`
- `S5B-T05 -> S5B-T06`
- `S5B-T06 -> S5B-T07`
- `S5B-T07 -> S5B-T08`
- `S5B-T08 -> S5B-T09`
- `S5B-T09 -> S5B-R01`

Tasks:

- id: S5B-W01
  description: Create isolated worktree from `origin/main` at `/Users/void_rsk/Projects/soul-dec-worktrees/s5b-soulseek-ui-feature-audit` using branch `codex/s5b-soulseek-ui-feature-audit`
  status: done
  depends_on: []

- id: S5B-W02
  description: Persist Stage 5B plan in `TODO-CODEX.md` within the isolated worktree
  status: done
  depends_on: [S5B-W01]

- id: S5B-T01
  description: Establish investigation baseline for `/Applications/SoulseekQt.app` (binary metadata, hashes, build/timeframe anchors)
  status: done
  depends_on: [S5B-W02]

- id: S5B-T02
  description: Pass 1 UI walkthrough: map all visible UI surfaces and interaction-driven features
  status: done
  depends_on: [S5B-T01]

- id: S5B-T03
  description: Pass 1 external-source extraction: changelog/news/forum feature evidence with dated references
  status: done
  depends_on: [S5B-T01]

- id: S5B-T04
  description: Pass 1 static reverse/decompilation extraction for non-obvious UI/feature hooks
  status: done
  depends_on: [S5B-T01]

- id: S5B-T05
  description: Consolidate comprehensive feature inventory in `docs/state/soulseek-feature-inventory.md` (UI + non-UI)
  status: done
  depends_on: [S5B-T02, S5B-T03, S5B-T04]

- id: S5B-T06
  description: Pass 2 exhaustive review over every Pass 1 feature and close mapping gaps (`verified_pass2` / `gap_found`)
  status: done
  depends_on: [S5B-T05]

- id: S5B-T07
  description: Update decompilation/evidence docs with new findings from the feature audit
  status: done
  depends_on: [S5B-T06]

- id: S5B-T08
  description: Synchronize project memory artifacts (`TODO-CODEX.md`, state docs, ledgers) for Stage 5B
  status: done
  depends_on: [S5B-T07]

- id: S5B-T09
  description: Run documentation/KB validations (`kb_validate`, zensical build)
  status: done
  depends_on: [S5B-T08]

- id: S5B-R01
  description: Finalize Stage 5B closure notes including Pass 1 vs Pass 2 delta report
  status: done
  depends_on: [S5B-T09]

Notes:

- Stage 5B scope is research/documentation/evidence only; no product feature implementation changes are planned.
- Runtime-auth-required features are recorded with explicit `requires_auth` status when not executable without live login.
- Feature inventory closure stats: `pass1=42`, `pass2_revisited=42`, `verified_pass2=41`, `gap_found=1`.
- Stage PR notes published in `docs/pr/0016-s5b-soulseek-ui-feature-audit.md`.
- Validation gates passed: `python3 scripts/kb_validate.py` and `/Users/void_rsk/Projects/soul-dec/.venv-tools/bin/zensical build -f zensical.toml`.

## Stage 5D-5H - multi-wave opaque-to-typed control promotion pack

Dependency graph:

- `S5P-W01 -> S5P-T01`
- `S5P-T01 -> S5D-T01`
- `S5D-T01 -> S5D-T02 -> S5D-T03`
- `S5D-T03 -> S5E-T01 -> S5E-T02 -> S5E-T03`
- `S5E-T03 -> S5F-T01 -> S5F-T02 -> S5F-T03`
- `S5F-T03 -> S5G-T01 -> S5G-T02 -> S5G-T03`
- `S5G-T03 -> S5H-T01 -> S5H-T02 -> S5H-T03`
- `S5H-T03 -> S5P-S01 -> S5P-S02 -> S5P-T90 -> S5P-R01`
- `S5P-R01 -> S5P-Q01 -> S5P-Q02 -> S5P-Q03 -> S5P-Q04 -> S5P-T99`

Tasks:

- id: S5P-W01
  description: Start from updated `main` and create branch `codex/s5d-s5h-control-typing-pack`
  status: done
  depends_on: []

- id: S5P-T01
  description: Persist S5D-S5H execution plan and opaque-code inventory in `TODO-CODEX.md`
  status: done
  depends_on: [S5P-W01]

- id: S5D-T01
  description: Stage 5D implement typed global-room control payloads (150/151/152/153)
  status: done
  depends_on: [S5P-T01]

- id: S5D-T02
  description: Stage 5D generate authenticated runtime capture and redact artifacts
  status: done
  depends_on: [S5D-T01]

- id: S5D-T03
  description: Stage 5D promote map/schema/docs with runtime-backed evidence
  status: done
  depends_on: [S5D-T02]

- id: S5E-T01
  description: Stage 5E implement typed parent/disconnect control payloads (86/87/88/90/100)
  status: done
  depends_on: [S5D-T03]

- id: S5E-T02
  description: Stage 5E generate authenticated runtime capture and redact artifacts
  status: done
  depends_on: [S5E-T01]

- id: S5E-T03
  description: Stage 5E promote map/schema/docs with runtime-backed evidence
  status: done
  depends_on: [S5E-T02]

- id: S5F-T01
  description: Stage 5F implement typed private-room ownership/membership control payloads (136/137/139/140/145)
  status: done
  depends_on: [S5E-T03]

- id: S5F-T02
  description: Stage 5F generate authenticated runtime capture and redact artifacts
  status: done
  depends_on: [S5F-T01]

- id: S5F-T03
  description: Stage 5F promote map/schema/docs with runtime-backed evidence
  status: done
  depends_on: [S5F-T02]

- id: S5G-T01
  description: Stage 5G implement typed text-control payloads (58/62/63/66/117/118)
  status: done
  depends_on: [S5F-T03]

- id: S5G-T02
  description: Stage 5G generate authenticated runtime capture and redact artifacts
  status: done
  depends_on: [S5G-T01]

- id: S5G-T03
  description: Stage 5G promote map/schema/docs with runtime-backed evidence
  status: done
  depends_on: [S5G-T02]

- id: S5H-T01
  description: Stage 5H implement typed system-control payloads (28/32/130)
  status: done
  depends_on: [S5G-T03]

- id: S5H-T02
  description: Stage 5H generate authenticated runtime capture and redact artifacts
  status: done
  depends_on: [S5H-T01]

- id: S5H-T03
  description: Stage 5H promote map/schema/docs with runtime-backed evidence
  status: done
  depends_on: [S5H-T02]

- id: S5P-S01
  description: Run security best-practices pass on touched protocol/core/cli/verify/runtime paths
  status: done
  depends_on: [S5H-T03]

- id: S5P-S02
  description: Run code-simplifier pass on touched Rust files and remove avoidable duplication
  status: done
  depends_on: [S5P-S01]

- id: S5P-T90
  description: Run full gates (`kb_validate`, `run_diff_verify`, `run_regression`, zensical build) and fix failures
  status: done
  depends_on: [S5P-S02]

- id: S5P-R01
  description: Publish PR notes in `docs/pr/0019-s5d-s5h-control-typing-pack.md`
  status: done
  depends_on: [S5P-T90]

- id: S5P-Q01
  description: Complete local review loop round 1 (security + simplifier) on PR
  status: done
  depends_on: [S5P-R01]

- id: S5P-Q02
  description: Triage round-1 findings and apply/dismiss with rationale
  status: done
  depends_on: [S5P-Q01]

- id: S5P-Q03
  description: Complete local review loop round 2
  status: done
  depends_on: [S5P-Q02]

- id: S5P-Q04
  description: Triage round-2 findings and resolve remaining review notes
  status: done
  depends_on: [S5P-Q03]

- id: S5P-T99
  description: Merge-ready closure and stage tracker synchronization after PR merge
  status: done
  depends_on: [S5P-Q04]

Notes:

- Remaining server opaque-control set before this pack: `34` message codes in `OPAQUE_SERVER_CONTROL_CODES`.
- Remaining generic opaque-control set after this pack: `15` message codes in `OPAQUE_SERVER_CONTROL_CODES`.
- Five-wave execution (`S5D`..`S5H`) completed with runtime evidence and typed payload promotion while preserving `implemented+mapped=131`.
- Stage pack merged via PR `#19` (`ca94bb0a13b2e8e0a106ddf13810b999be86f3f9`).
