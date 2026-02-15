# PR 0015 - S4K: missing-code closure + global/distributed peer-control protocol implementation

## Branch

- `codex/s4k-global-peer-control-implementation`

## Objective

Close Stage 4K by eliminating the matrix `missing` bucket and adding protocol codec coverage for the global/distributed tail and peer-control unresolved set.

## Scope

1. Map all previously missing names with authoritative jump-table evidence.
2. Add protocol constants and server/peer codec handling in `rust/protocol`.
3. Regenerate schema/docs/matrix from authoritative artifacts.
4. Sync stage/state docs and TODO records.

S4K closure set:

1. `SM_ADD_USER` (`5`)
2. `SM_REMOVE_USER` (`6`)
3. `SM_SEND_CONNECT_TOKEN` (`33`)
4. `SM_PLACE_IN_LINE` (`59`)
5. `SM_PLACE_IN_LINE_RESPONSE` (`60`)
6. `SM_ADD_PRIVILEGED_USER` (`91`)
7. `SM_LOW_PRIORITY_FILE_SEARCH` (`103`)
8. `SM_WISHLIST_WAIT` (`104`)
9. `SM_DNET_LEVEL` (`126`)
10. `SM_DNET_GROUP_LEADER` (`127`)
11. `SM_DNET_DELIVERY_REPORT` (`128`)
12. `SM_DNET_CHILD_DEPTH` (`129`)
13. `SM_FLOOD` (`131`)
14. `SM_REMOVE_ROOM_OPERATORSHIP` (`146`)
15. `SM_REMOVE_OWN_ROOM_OPERATORSHIP` (`147`)
16. `SM_JOIN_GLOBAL_ROOM` (`150`)
17. `SM_LEAVE_GLOBAL_ROOM` (`151`)
18. `SM_SAY_GLOBAL_ROOM` (`152`)
19. `SM_SEARCH_CORRELATIONS` (`153`)
20. `SM_PEER_MESSAGE_ALT` (`292`, map closure)
21. `PM_SAY` (`1`)
22. `PM_SEND_CONNECT_TOKEN` (`33`)
23. `PM_PLACEHOLD_UPLOAD` (`42`)
24. `PM_NOTHING` (`52`)

Evidence sources:

- `evidence/reverse/message_codes_jump_table.md`
- `captures/redacted/login-peer-message/official_frames.hex` (for `SM_PEER_MESSAGE_ALT` mapping closure)

## Outcome

1. `analysis/ghidra/maps/message_map.csv` now maps all tracked protocol names (`131/131`).
2. `rust/protocol` now includes constants and decode/encode coverage for all S4K messages.
3. Matrix snapshot after S4K:
   - tracked: `131`
   - implemented+mapped: `91`
   - mapped-not-implemented: `40`
   - implemented-not-mapped: `0`
   - missing: `0`

## Validation

```bash
cargo test -p protocol
python3 scripts/kb_validate.py
scripts/run_diff_verify.sh
scripts/run_regression.sh
./.venv-tools/bin/zensical build -f zensical.toml
```

## Review Loop

1. Round 1 `@codex review`: completed. Connector returned usage-limit notice with no actionable findings.
2. Round 2 `@codex review`: completed. Connector returned usage-limit notice with no actionable findings.

## Maintainability Notes

1. Added `OpaquePayload` for unresolved runtime-shape messages to avoid speculative field assumptions while preserving protocol coverage.
2. Kept typed payload parsing where wire shape is stable (`UserLookupPayload`, `FileSearchPayload`).
3. Next stage (`S4L`) should replace S4K opaque payloads with typed schemas as runtime captures become available.
