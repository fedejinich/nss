# PR 0013 - S4I: ticker/term control mapping continuation

## Branch

- `codex/s4i-ticker-term-control-map`

## Objective

Close Stage 4I by extending authoritative mapping coverage for ticker/term-control messages with deterministic jump-table evidence.

## Scope

1. Add 8 S4I rows to `analysis/ghidra/maps/message_map.csv`.
2. Regenerate schema/docs/matrix from authoritative maps.
3. Sync state artifacts for S4I completion and S4J preview.

S4I mapped message pack:

1. `SM_ADD_LIKE_TERM` (`51`)
2. `SM_REMOVE_LIKE_TERM` (`52`)
3. `SM_GET_ROOM_TICKER` (`113`)
4. `SM_ROOM_TICKER_USER_ADDED` (`114`)
5. `SM_ROOM_TICKER_USER_REMOVED` (`115`)
6. `SM_SET_TICKER` (`116`)
7. `SM_ADD_HATE_TERM` (`117`)
8. `SM_REMOVE_HATE_TERM` (`118`)

Evidence source:

- `evidence/reverse/message_codes_jump_table.md`

## Outcome

1. `message_map.csv` now includes the full S4I pack with confidence `high`.
2. Regenerated artifacts:
   - `analysis/protocol/message_schema.json`
   - `docs/re/static/message-schema.md`
   - `docs/re/static/detangling.md`
   - `docs/verification/evidence-ledger.md`
   - `docs/state/protocol-matrix.md`
3. Matrix snapshot after S4I mapping:
   - tracked: `131`
   - implemented+mapped: `67`
   - mapped-not-implemented: `32`
   - missing: `31`

## Validation

```bash
python3 scripts/kb_validate.py
scripts/run_regression.sh
./.venv-tools/bin/zensical build -f zensical.toml
```

## Notes

S4I remains mapping-first. Typed protocol/core/CLI support for S4F+S4G+S4H+S4I mapped rows is deferred to follow-up implementation stages.
