# NeoSoulSeek Roadmap

## Scope

This roadmap represents the execution path after S3A. It focuses on protocol mapping first, then incremental SDK/CLI capabilities for a custom evolvable client.

## Stage Timeline

```mermaid
gantt
    title NeoSoulSeek Stages (KB-first)
    dateFormat  YYYY-MM-DD
    axisFormat  %m/%d
    section Completed
    S2 Core P2P MVP (25 messages) :done, s2, 2026-02-10, 2d
    S2R Runtime refresh + confidence promotion :done, s2r, after s2, 1d
    S3A Authenticated login + semantic parity :done, s3a, after s2r, 1d
    S3B Rooms + Presence (8-message pack) :done, s3b, 2026-02-14, 2d
    section Preview
    S4 Recommendations + peer advanced + room moderation :s4, after s3b, 3d
```

## S3B Dependency Graph

```mermaid
graph TD
    R01["S3B-R01 Roadmap page"] --> R02["S3B-R02 Baseline sync + docs index link"]
    R02 --> T01["S3B-T01 Resolve message codes/evidence"]
    T01 --> T02["S3B-T02 Runtime captures + redaction"]
    T02 --> T03["S3B-T03 message_map + message_schema updates"]
    T03 --> T04["S3B-T04 rust/protocol rooms codec/types"]
    T04 --> T05["S3B-T05 rust/core room operations/events"]
    T05 --> T06["S3B-T06 rust/cli room commands"]
    T06 --> T07["S3B-T07 rust/verify semantic extensions"]
    T07 --> T08["S3B-T08 Validation gates"]
    T08 --> R03["S3B-R03 Refresh roadmap and status docs"]
    T08 --> T09["S3B-T09 PR doc + retrospective"]
    R03 --> T09
```

## Stage Status Matrix

| Stage | Owner area | Status | Evidence | Next gate |
|---|---|---|---|---|
| S2 | protocol/core/cli/verify | done | `docs/state/stage2-parity-audit.md` | none |
| S2R | runtime+KB | done | `docs/verification/evidence-ledger.md` | none |
| S3A | auth+semantic verify | done | `docs/pr/0003-s3a-auth-semantic-parity.md` | S3B start |
| S3B | rooms/presence batch | done (branch scope) | `captures/redacted/login-join-room-presence/manifest.redacted.json` | S4 domain batch selection |
| S4 preview | discovery+peer advanced | preview | `docs/state/protocol-backlog.md` | finalize S3B outcomes |

## S3B Target Contract

Required 8-message pack:

1. `SM_ROOM_LIST`
2. `SM_JOIN_ROOM`
3. `SM_LEAVE_ROOM`
4. `SM_USER_JOINED_ROOM`
5. `SM_USER_LEFT_ROOM`
6. `SM_ROOM_MEMBERS`
7. `SM_ROOM_OPERATORS`
8. `SM_SAY_CHATROOM`

Confidence gate for this batch:

- `high >= 6`
- `medium <= 2`
- `low = 0`

All entries must include valid evidence links.

## S4 Preview (Not in S3B implementation)

1. Recommendations and discovery message batch.
2. Peer advanced message batch.
3. Room moderation follow-up (`SM_ADD_ROOM_MEMBER`, `SM_REMOVE_ROOM_MEMBER`).
4. Differential replay coverage expansion for the new domains.
