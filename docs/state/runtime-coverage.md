# Runtime Coverage

This page tracks runtime-vs-static evidence closure and semantic-tail closure status.

## Snapshot

- Total messages: `131`
- Verified runtime: `131`
- Verified static: `0`
- Runtime coverage: `100.0%`
- Runtime gap: `0`
- Messages with unresolved semantic tail fields: `0`

## Runtime Target Progress

| scope | code | message | scenario | current status | source |
|---|---:|---|---|---|---|
| peer | 1 | `PM_SAY` | `peer-static-runtime` | `verified_runtime` | `captures/redacted/peer-static-runtime/official_frames.hex` |
| peer | 4 | `PM_GET_SHARED_FILE_LIST` | `peer-static-runtime` | `verified_runtime` | `captures/redacted/peer-static-runtime/official_frames.hex` |
| peer | 5 | `PM_SHARED_FILE_LIST` | `peer-static-runtime` | `verified_runtime` | `captures/redacted/peer-static-runtime/official_frames.hex` |
| peer | 8 | `PM_FILE_SEARCH_REQUEST` | `peer-static-runtime` | `verified_runtime` | `captures/redacted/peer-static-runtime/official_frames.hex` |
| peer | 15 | `PM_USER_INFO_REQUEST` | `peer-static-runtime` | `verified_runtime` | `captures/redacted/peer-static-runtime/official_frames.hex` |
| peer | 16 | `PM_USER_INFO_REPLY` | `peer-static-runtime` | `verified_runtime` | `captures/redacted/peer-static-runtime/official_frames.hex` |
| peer | 33 | `PM_SEND_CONNECT_TOKEN` | `peer-static-runtime` | `verified_runtime` | `captures/redacted/peer-static-runtime/official_frames.hex` |
| peer | 40 | `PM_TRANSFER_REQUEST` | `peer-static-runtime` | `verified_runtime` | `captures/redacted/peer-static-runtime/official_frames.hex` |
| peer | 41 | `PM_TRANSFER_RESPONSE` | `peer-static-runtime` | `verified_runtime` | `captures/redacted/peer-static-runtime/official_frames.hex` |
| peer | 42 | `PM_PLACEHOLD_UPLOAD` | `peer-static-runtime` | `verified_runtime` | `captures/redacted/peer-static-runtime/official_frames.hex` |
| peer | 43 | `PM_QUEUE_UPLOAD` | `peer-static-runtime` | `verified_runtime` | `captures/redacted/peer-static-runtime/official_frames.hex` |
| peer | 51 | `PM_UPLOAD_PLACE_IN_LINE_REQUEST` | `peer-static-runtime` | `verified_runtime` | `captures/redacted/peer-static-runtime/official_frames.hex` |
| peer | 52 | `PM_NOTHING` | `peer-static-runtime` | `verified_runtime` | `captures/redacted/peer-static-runtime/official_frames.hex` |
| server | 2 | `SM_SET_WAIT_PORT` | `login-static-server-runtime` | `verified_runtime` | `captures/redacted/login-static-server-runtime/official_frames.hex` |
| server | 5 | `SM_ADD_USER` | `login-static-server-runtime` | `verified_runtime` | `captures/redacted/login-static-server-runtime/official_frames.hex` |
| server | 6 | `SM_REMOVE_USER` | `login-static-server-runtime` | `verified_runtime` | `captures/redacted/login-static-server-runtime/official_frames.hex` |
| server | 26 | `SM_FILE_SEARCH` | `login-static-server-runtime` | `verified_runtime` | `captures/redacted/login-static-server-runtime/official_frames.hex` |
| server | 33 | `SM_SEND_CONNECT_TOKEN` | `login-static-server-runtime` | `verified_runtime` | `captures/redacted/login-static-server-runtime/official_frames.hex` |
| server | 34 | `SM_DOWNLOAD_SPEED` | `login-static-server-runtime` | `verified_runtime` | `captures/redacted/login-static-server-runtime/official_frames.hex` |
| server | 35 | `SM_SHARED_FOLDERS_FILES` | `login-static-server-runtime` | `verified_runtime` | `captures/redacted/login-static-server-runtime/official_frames.hex` |
| server | 59 | `SM_PLACE_IN_LINE` | `login-static-server-runtime` | `verified_runtime` | `captures/redacted/login-static-server-runtime/official_frames.hex` |
| server | 60 | `SM_PLACE_IN_LINE_RESPONSE` | `login-static-server-runtime` | `verified_runtime` | `captures/redacted/login-static-server-runtime/official_frames.hex` |
| server | 91 | `SM_ADD_PRIVILEGED_USER` | `login-static-server-runtime` | `verified_runtime` | `captures/redacted/login-static-server-runtime/official_frames.hex` |
| server | 103 | `SM_LOW_PRIORITY_FILE_SEARCH` | `login-static-server-runtime` | `verified_runtime` | `captures/redacted/login-static-server-runtime/official_frames.hex` |
| server | 104 | `SM_WISHLIST_WAIT` | `login-static-server-runtime` | `verified_runtime` | `captures/redacted/login-static-server-runtime/official_frames.hex` |
| server | 132 | `SM_BAN_USER` | `login-static-server-runtime` | `verified_runtime` | `captures/redacted/login-static-server-runtime/official_frames.hex` |
| server | 134 | `SM_ADD_ROOM_MEMBER` | `login-static-server-runtime` | `verified_runtime` | `captures/redacted/login-static-server-runtime/official_frames.hex` |
| server | 135 | `SM_REMOVE_ROOM_MEMBER` | `login-static-server-runtime` | `verified_runtime` | `captures/redacted/login-static-server-runtime/official_frames.hex` |
| server | 143 | `SM_ADD_ROOM_OPERATOR` | `login-static-server-runtime` | `verified_runtime` | `captures/redacted/login-static-server-runtime/official_frames.hex` |
| server | 144 | `SM_REMOVE_ROOM_OPERATOR` | `login-static-server-runtime` | `verified_runtime` | `captures/redacted/login-static-server-runtime/official_frames.hex` |

## Static-Only Messages

No static-only messages remain.

## Unresolved Semantic Tail Fields

No unresolved semantic tail fields remain.

## Regeneration

```bash
python3 tools/state/generate_runtime_coverage.py
```
