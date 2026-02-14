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
    S4B Peer advanced + room moderation :done, s4b, after s4a, 2d
    S4C Privileges + social control + peer folder domains :done, s4c, after s4b, 2d
    section Preview
    S4D Privilege/messaging gaps + peer legacy cleanup :s4d, after s4c, 3d
```

## S4B Dependency Graph (Executed)

```mermaid
graph TD
    R01["S4B-R01 Initialize plan + roadmap baseline"] --> R02["S4B-R02 Protocol matrix generation"]
    R02 --> T01["S4B-T01 Resolve message codes/evidence"]
    T01 --> T02["S4B-T02 Runtime captures + redaction"]
    T02 --> T03["S4B-T03 message_map + schema updates"]
    T03 --> T04["S4B-T04 rust/protocol peer+moderation codec/types"]
    T04 --> T05["S4B-T05 rust/core + rust/cli operations"]
    T05 --> T06["S4B-T06 rust/verify + required runs updates"]
    T06 --> T07["S4B-T07 Regression coverage for S4B + matrix"]
    T07 --> R03["S4B-R03 Refresh roadmap/status/backlog docs"]
    R03 --> T08["S4B-T08 Final validation + PR publication"]
```

## S4C Dependency Graph (Planned)

```mermaid
graph TD
    R01["S4C-R01 Initialize plan + roadmap baseline"] --> T01["S4C-T01 Resolve message codes/evidence"]
    T01 --> T02["S4C-T02 Runtime captures + redaction"]
    T02 --> T03["S4C-T03 message_map + schema updates"]
    T03 --> T04["S4C-T04 rust/protocol privileges/social/folder codec"]
    T04 --> T05["S4C-T05 rust/core + rust/cli operations"]
    T05 --> T06["S4C-T06 rust/verify + required runs updates"]
    T06 --> T07["S4C-T07 Regression coverage for S4C"]
    T07 --> R02["S4C-R02 Refresh roadmap/status/backlog docs"]
    R02 --> T08["S4C-T08 Final validation + PR publication"]
```

## Stage Status Matrix

| Stage | Owner area | Status | Evidence | Next gate |
|---|---|---|---|---|
| S2 | protocol/core/cli/verify | done | `docs/state/stage2-parity-audit.md` | none |
| S2R | runtime+KB | done | `docs/verification/evidence-ledger.md` | none |
| S3A | auth+semantic verify | done | `docs/pr/0003-s3a-auth-semantic-parity.md` | S3B start |
| S3B | rooms/presence batch | done | `docs/pr/0004-s3b-rooms-presence-roadmap.md` | S4A start |
| S4A | recommendations/discovery batch | done | `docs/pr/0005-s4a-recommendations-discovery.md` | select S4B scope |
| S4B | peer advanced + room moderation | done | `docs/pr/0006-s4b-peer-room-matrix.md` | define S4C message batch |
| S4C | privileges/social control + peer folder domains | done | `docs/pr/0007-s4c-privileges-social-peer-folder.md` | define S4D batch |
| S4D preview | privilege/messaging gaps + peer legacy cleanup | preview | `docs/state/protocol-backlog.md` | publish S4D dependency graph |

## S4B Target Contract

Required 9-message pack:

1. `SM_ADD_ROOM_MEMBER`
2. `SM_REMOVE_ROOM_MEMBER`
3. `SM_ADD_ROOM_OPERATOR`
4. `SM_REMOVE_ROOM_OPERATOR`
5. `PM_USER_INFO_REQUEST`
6. `PM_USER_INFO_REPLY`
7. `PM_EXACT_FILE_SEARCH_REQUEST`
8. `PM_INDIRECT_FILE_SEARCH_REQUEST`
9. `PM_UPLOAD_PLACE_IN_LINE_REQUEST`

Confidence gate for this batch:

- `high >= 7`
- `medium <= 2`
- `low = 0`

All entries must include valid evidence links.

## S4C Target Contract (Executed)

Required target pack:

1. `SM_IGNORE_USER`
2. `SM_UNIGNORE_USER`
3. `SM_GET_OWN_PRIVILEGES_STATUS`
4. `SM_GET_USER_PRIVILEGES_STATUS`
5. `SM_GIVE_PRIVILEGE`
6. `SM_INFORM_USER_OF_PRIVILEGES`
7. `SM_INFORM_USER_OF_PRIVILEGES_ACK`
8. `PM_GET_SHARED_FILES_IN_FOLDER`
9. `PM_SHARED_FILES_IN_FOLDER`

Confidence gate for this batch:

- `high >= 7`
- `medium <= 2`
- `low = 0`

Backlog note:

- `SM_BAN_USER` remains deferred pending authoritative code/evidence resolution.
