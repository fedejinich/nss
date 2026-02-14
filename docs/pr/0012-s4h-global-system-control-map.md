# PR 0012 - S4H: global room/system control mapping continuation

## Branch

- `codex/s4h-global-system-control-map`

## Objective

Close Stage 4H by extending authoritative mapping coverage for global room/system control messages using deterministic jump-table evidence.

## Scope

1. Add 8 S4H rows to `analysis/ghidra/maps/message_map.csv`.
2. Regenerate schema/docs/matrix from authoritative maps.
3. Sync state artifacts for S4H completion and S4I preview.

S4H mapped message pack:

1. `SM_ADD_CHATROOM` (`10`)
2. `SM_SET_STATUS` (`28`)
3. `SM_HEARTBEAT` (`32`)
4. `SM_RELOGGED` (`41`)
5. `SM_USER_LIST` (`61`)
6. `SM_ROOM_ADDED` (`62`)
7. `SM_ROOM_REMOVED` (`63`)
8. `SM_CONNECT_TO_CLIENT` (`70`)

Evidence source:

- `evidence/reverse/message_codes_jump_table.md`

## Outcome

1. `message_map.csv` now includes the full S4H pack with confidence `high`.
2. Regenerated artifacts:
   - `analysis/protocol/message_schema.json`
   - `docs/re/static/message-schema.md`
   - `docs/re/static/detangling.md`
   - `docs/verification/evidence-ledger.md`
   - `docs/state/protocol-matrix.md`
3. Matrix snapshot after S4H mapping:
   - tracked: `131`
   - implemented+mapped: `67`
   - mapped-not-implemented: `24`
   - missing: `39`

## Validation

```bash
python3 scripts/kb_validate.py
scripts/run_regression.sh
./.venv-tools/bin/zensical build -f zensical.toml
```

## Notes

S4H remains mapping-first. Typed protocol/core/CLI support for S4F+S4G+S4H mapped rows is deferred to follow-up implementation stages.
