# S6 Opaque-Tail Plan

This page tracks S6 opaque-tail execution state and typed-promotion completion status.

## Baseline Report

- Report JSON: [opaque-tail-report.json](opaque-tail-report.json)

## Current Scope

Current baseline in `rust/protocol/src/lib.rs`:

- `OPAQUE_SERVER_CONTROL_CODES = []` (generic opaque-tail closure complete)

Completed objective:

1. Replace remaining opaque decode/encode branches with typed payloads backed by runtime evidence.
2. Preserve semantic differential parity for all existing capture runs.
3. Keep matrix closure (`implemented+mapped=131`) while increasing typed payload depth.

## Executed Batches

1. `S6-Batch-1` (global/session list controls): `41`, `61`, `67`, `70`
2. `S6-Batch-2` (distributed/parent controls): `71`, `73`, `82`, `93`, `102`
3. `S6-Batch-3` (ticker/private-room controls): `114`, `115`, `116`, `138`, `141`, `142`

Runtime artifacts:

1. `captures/redacted/login-s6-batch1-control`
2. `captures/redacted/login-s6-batch2-control`
3. `captures/redacted/login-s6-batch3-control`

Post-S6 status:

1. Dedicated legacy follow-up from S6E and S6F is complete:
   - room-operatorship revocation families (`146`, `147`)
   - distributed residual families (`128`, `131`)
2. Remaining protocol work continues as post-closure audit/expansion, not opaque-tail reduction.

## Regeneration

```bash
python3 tools/state/report_opaque_tail.py
```

Or via the combined workflow:

```bash
scripts/sync_state_dashboards.sh
```
