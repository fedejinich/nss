# S6 Opaque-Tail Plan

This page tracks the remaining opaque server-control payload set and the execution batches for Stage S6.

## Baseline Report

- Report JSON: [opaque-tail-report.json](opaque-tail-report.json)

## Current Scope

The remaining opaque tail is currently represented by `OPAQUE_SERVER_CONTROL_CODES` in `rust/protocol/src/lib.rs`.

Current objective:

1. Replace remaining opaque decode/encode branches with typed payloads backed by runtime evidence.
2. Preserve semantic differential parity for all existing capture runs.
3. Keep matrix closure (`implemented+mapped=131`) while increasing typed payload depth.

## Proposed Batches

1. `S6-Batch-1` (global/session list controls): `41`, `61`, `67`, `70`
2. `S6-Batch-2` (distributed/parent controls): `71`, `73`, `82`, `93`, `102`
3. `S6-Batch-3` (ticker/private-room controls): `114`, `115`, `116`, `138`, `141`, `142`

## Regeneration

```bash
python3 tools/state/report_opaque_tail.py
```

Or via the combined workflow:

```bash
scripts/sync_state_dashboards.sh
```
