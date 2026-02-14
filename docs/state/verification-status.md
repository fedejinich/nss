# Verification Status

## Objective

Ensure evidence traceability and semantic protocol parity for Stage 3B (Rooms+Presence batch) while preserving Stage 2/S3A guarantees.

## Validation Gates

### KB validation

```bash
python3 scripts/kb_validate.py
```

Checks:

- Name/data maps contain valid evidence.
- `message_map.csv` has valid links and confidence values.
- `message_schema.json` has valid evidence links and schema integrity.

### Differential verification

```bash
scripts/run_diff_verify.sh
```

Runs:

1. Fixture parity (`captures/fixtures/*`).
2. Runtime redacted capture parity for mandatory scenarios:
   - `login-only`
   - `login-search`
   - `login-search-download`
   - `upload-deny`
   - `upload-accept`
   - `login-room-list`
   - `login-join-room-presence`
   - `login-leave-room`
3. Default mode is semantic (`VERIFY_MODE=semantic`), with backward-compatible bytes mode.

### Full regression

```bash
scripts/run_regression.sh
```

Includes:

1. Python unit tests (`tests/kb`, `tests/protocol`, `tests/runtime`).
2. Rust unit/integration tests (`cargo test`).
3. KB validation gate.
4. Differential verification gate.
5. Zensical build check (if available).

## Stage 3B Coverage Status

- S3B 8-message Rooms+Presence pack is present in both:
  - `analysis/ghidra/maps/message_map.csv`
  - `analysis/protocol/message_schema.json`
- Confidence distribution for S3B pack:
  - `high=8`
  - `medium=0`
  - `low=0`
- Runtime evidence is linked for all S3B rows.

## Runtime Evidence Snapshot

- Official server: `server.slsknet.org:2242`
- Auth tuple used: `160/1`
- Real room commands validated against authenticated session:
  - `room list`
  - `room join`
  - `room members`
  - `room watch`
  - `room leave`
- S3B runtime redacted scenarios:
  - `captures/redacted/login-room-list`
  - `captures/redacted/login-join-room-presence`
  - `captures/redacted/login-leave-room`

## Residual Risk

- `SM_JOIN_ROOM`, `SM_ROOM_MEMBERS`, and `SM_ROOM_OPERATORS` payloads are currently summary-oriented parsers for CLI/verify stability; full field-level exhaustive parsing remains future work.
- Unknown/partially mapped server messages continue to use semantic fallback normalization (`payload_md5`) in verifier mode.
