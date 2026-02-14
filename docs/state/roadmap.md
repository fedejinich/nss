# NeoSoulSeek Roadmap

## Scope

This roadmap tracks staged execution with protocol mapping first, then incremental SDK/CLI capabilities for a custom evolvable client.

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
    S4A Recommendations + Discovery batch :done, s4a, after s3b, 1d
    section Preview
    S4B Peer advanced + room moderation :s4b, after s4a, 3d
```

## S4A Dependency Graph (Executed)

```mermaid
graph TD
    R01["S4A-R01 Initialize plan + roadmap baseline"] --> T01["S4A-T01 Resolve message codes/evidence"]
    T01 --> T02["S4A-T02 Runtime captures + redaction"]
    T02 --> T03["S4A-T03 message_map + schema updates"]
    T03 --> T04["S4A-T04 rust/protocol discovery codec/types"]
    T04 --> T05["S4A-T05 rust/core discovery operations"]
    T05 --> T06["S4A-T06 rust/cli discover commands"]
    T06 --> T07["S4A-T07 rust/verify semantic extensions"]
    T07 --> T08["S4A-T08 Validation gates"]
    T08 --> R02["S4A-R02 Refresh roadmap/status/backlog docs"]
    R02 --> T09["S4A-T09 Publish PR doc + retrospective"]
```

## Stage Status Matrix

| Stage | Owner area | Status | Evidence | Next gate |
|---|---|---|---|---|
| S2 | protocol/core/cli/verify | done | `docs/state/stage2-parity-audit.md` | none |
| S2R | runtime+KB | done | `docs/verification/evidence-ledger.md` | none |
| S3A | auth+semantic verify | done | `docs/pr/0003-s3a-auth-semantic-parity.md` | S3B start |
| S3B | rooms/presence batch | done | `docs/pr/0004-s3b-rooms-presence-roadmap.md` | S4A start |
| S4A | recommendations/discovery batch | done | `docs/pr/0005-s4a-recommendations-discovery.md` | select S4B scope |
| S4B preview | peer advanced + room moderation | preview | `docs/state/protocol-backlog.md` | approve S4B dependency graph |

## S4A Target Contract

Required 5-message pack:

1. `SM_GET_SIMILAR_TERMS`
2. `SM_GET_RECOMMENDATIONS`
3. `SM_GET_MY_RECOMMENDATIONS`
4. `SM_GET_GLOBAL_RECOMMENDATIONS`
5. `SM_GET_USER_RECOMMENDATIONS`

Confidence gate for this batch:

- `high >= 5`
- `medium <= 0`
- `low = 0`

All entries must include valid evidence links.

## S4B Preview (After S4A)

1. Peer advanced message batch.
2. Room moderation follow-up (`SM_ADD_ROOM_MEMBER`, `SM_REMOVE_ROOM_MEMBER`).
3. Differential replay coverage expansion for the new domains.
