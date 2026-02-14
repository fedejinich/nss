# PR 0010 - S4F: global/admin/distributed control mapping batch

## Branch

- `codex/s4f-global-admin-distributed-map`

## Objective

Close Stage 4F by expanding authoritative protocol mapping coverage for global/admin/distributed-control server messages using deterministic jump-table evidence.

## Scope

1. Add 8 S4F rows to `analysis/ghidra/maps/message_map.csv` with static high-confidence evidence.
2. Regenerate schema/docs/matrix from authoritative maps.
3. Update stage status artifacts for S4F completion and S4G preview.

S4F mapped message pack:

1. `SM_COMMAND` (`58`)
2. `SM_ADMIN_MESSAGE` (`66`)
3. `SM_GLOBAL_USER_LIST` (`67`)
4. `SM_SEND_DISTRIBUTIONS` (`71`)
5. `SM_NOTE_PARENT` (`73`)
6. `SM_CHILD_PARENT_MAP` (`82`)
7. `SM_DNET_MESSAGE` (`93`)
8. `SM_DNET_RESET` (`130`)

Evidence source:

- `evidence/reverse/message_codes_jump_table.md`

## Outcome

1. `message_map.csv` now contains all S4F rows with confidence `high`.
2. Regenerated artifacts:
   - `analysis/protocol/message_schema.json`
   - `docs/re/static/message-schema.md`
   - `docs/re/static/detangling.md`
   - `docs/verification/evidence-ledger.md`
   - `docs/state/protocol-matrix.md`
3. Matrix snapshot after S4F mapping:
   - tracked: `131`
   - implemented+mapped: `67`
   - mapped-not-implemented: `8`
   - missing: `55`

## Validation

```bash
python3 scripts/kb_validate.py
scripts/run_regression.sh
./.venv-tools/bin/zensical build -f zensical.toml
```

## Notes

This stage is intentionally mapping-first: S4F entries are authoritative and traceable but remain `mapped_not_implemented` until typed protocol/core/CLI support is added in follow-up stages.
