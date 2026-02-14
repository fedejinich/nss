# PR 0014 - S4J: private-room ownership/membership mapping continuation

## Branch

- `codex/s4j-private-room-ownership-map`

## Objective

Close Stage 4J by extending authoritative mapping coverage for private-room ownership/membership messages with deterministic jump-table evidence.

## Scope

1. Add 8 S4J rows to `analysis/ghidra/maps/message_map.csv`.
2. Regenerate schema/docs/matrix from authoritative maps.
3. Sync state artifacts for S4J completion and S4K preview.

S4J mapped message pack:

1. `SM_REMOVE_OWN_ROOM_MEMBERSHIP` (`136`)
2. `SM_GIVE_UP_ROOM` (`137`)
3. `SM_TRANSFER_ROOM_OWNERSHIP` (`138`)
4. `SM_ADD_ROOM_MEMBERSHIP` (`139`)
5. `SM_REMOVE_ROOM_MEMBERSHIP` (`140`)
6. `SM_ENABLE_PRIVATE_ROOM_ADD` (`141`)
7. `SM_CHANGE_PASSWORD` (`142`)
8. `SM_ADD_ROOM_OPERATORSHIP` (`145`)

Evidence source:

- `evidence/reverse/message_codes_jump_table.md`

## Outcome

1. `message_map.csv` now includes the full S4J pack with confidence `high`.
2. Regenerated artifacts:
   - `analysis/protocol/message_schema.json`
   - `docs/re/static/message-schema.md`
   - `docs/re/static/detangling.md`
   - `docs/verification/evidence-ledger.md`
   - `docs/state/protocol-matrix.md`
3. Matrix snapshot after S4J mapping:
   - tracked: `131`
   - implemented+mapped: `67`
   - mapped-not-implemented: `40`
   - missing: `23`

## Validation

```bash
python3 scripts/kb_validate.py
scripts/run_regression.sh
./.venv-tools/bin/zensical build -f zensical.toml
```

## Notes

S4J remains mapping-first. Typed protocol/core/CLI support for S4F through S4J mapped rows is deferred to follow-up implementation stages.
