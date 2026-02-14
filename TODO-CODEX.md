# TODO Execution Plan - NeoSoulSeek

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
