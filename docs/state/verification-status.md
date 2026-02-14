# Verification Status

## Objective

Ensure evidence traceability and semantic protocol parity for Stage 4A (Recommendations/Discovery batch) while preserving all previous stage guarantees.

## Validation Gates

### KB validation

```bash
python3 scripts/kb_validate.py
```

Checks:

- Name/data maps contain valid evidence.
- `message_map.csv` has valid source links and confidence fields.
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
   - `login-recommendations`
   - `login-user-recommendations`
   - `login-similar-terms`
3. Default mode is semantic (`VERIFY_MODE=semantic`) with bytes mode compatibility.

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

## Stage 4A Coverage Status

S4A 5-message discovery pack is present in:

- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`

Messages:

- `SM_GET_SIMILAR_TERMS`
- `SM_GET_RECOMMENDATIONS`
- `SM_GET_MY_RECOMMENDATIONS`
- `SM_GET_GLOBAL_RECOMMENDATIONS`
- `SM_GET_USER_RECOMMENDATIONS`

Confidence distribution for S4A batch:

- `high=5`
- `medium=0`
- `low=0`

## Runtime Evidence Snapshot

- Official server: `server.slsknet.org:2242`
- Auth tuple used: `160/1`
- Discovery commands validated against authenticated runtime session:
  - `discover recommendations`
  - `discover mine`
  - `discover global`
  - `discover user --target-user <name>`
  - `discover similar-terms --term <term>`
- S4A runtime redacted runs:
  - `captures/redacted/login-recommendations`
  - `captures/redacted/login-user-recommendations`
  - `captures/redacted/login-similar-terms`

## Residual Risk

- Recommendation/discovery payload parsing is summary-oriented and intentionally tolerant to preserve runtime compatibility while mapping coverage improves.
- Additional optional fields in recommendation payloads may still exist and are candidates for deeper parsing in S4B/S4C iterations.
