# PR 0011 - S4G: parent/distributed tuning mapping continuation

## Branch

- `codex/s4g-parent-distributed-tuning-map`

## Objective

Close Stage 4G by extending authoritative mapping coverage for parent/distributed tuning control messages with deterministic jump-table evidence.

## Scope

1. Add 8 S4G rows to `analysis/ghidra/maps/message_map.csv`.
2. Regenerate schema/docs/matrix from authoritative maps.
3. Sync state artifacts for S4G completion and S4H preview.

S4G mapped message pack:

1. `SM_SET_PARENT_MIN_SPEED` (`83`)
2. `SM_SET_PARENT_SPEED_CONNECTION_RATIO` (`84`)
3. `SM_SET_PARENT_INACTIVITY_BEFORE_DISCONNECT` (`86`)
4. `SM_SET_SERVER_INACTIVITY_BEFORE_DISCONNECT` (`87`)
5. `SM_NODES_IN_CACHE_BEFORE_DISCONNECT` (`88`)
6. `SM_SET_SECONDS_BEFORE_PING_CHILDREN` (`90`)
7. `SM_CAN_PARENT` (`100`)
8. `SM_POSSIBLE_PARENTS` (`102`)

Evidence source:

- `evidence/reverse/message_codes_jump_table.md`

## Outcome

1. `message_map.csv` now includes the full S4G pack with confidence `high`.
2. Regenerated artifacts:
   - `analysis/protocol/message_schema.json`
   - `docs/re/static/message-schema.md`
   - `docs/re/static/detangling.md`
   - `docs/verification/evidence-ledger.md`
   - `docs/state/protocol-matrix.md`
3. Matrix snapshot after S4G mapping:
   - tracked: `131`
   - implemented+mapped: `67`
   - mapped-not-implemented: `16`
   - missing: `47`

## Validation

```bash
python3 scripts/kb_validate.py
scripts/run_regression.sh
./.venv-tools/bin/zensical build -f zensical.toml
```

## Notes

S4G remains mapping-first. Typed protocol/core/CLI support for S4F+S4G mapped rows is deferred to follow-up implementation stages.
