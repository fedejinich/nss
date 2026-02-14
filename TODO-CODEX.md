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
  status: todo
  depends_on: [S4H-R01]

- id: S4H-Q02
  description: Triage/apply useful feedback from round 1 and resolve/dismiss comments with rationale
  status: todo
  depends_on: [S4H-Q01]

- id: S4H-Q03
  description: Request second `@codex review` after round-1 updates
  status: todo
  depends_on: [S4H-Q02]

- id: S4H-Q04
  description: Triage/apply useful feedback from round 2 and resolve/dismiss comments with rationale
  status: todo
  depends_on: [S4H-Q03]

- id: S4H-T06
  description: Merge-ready closure with final status synchronization
  status: todo
  depends_on: [S4H-Q04]
