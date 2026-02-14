# Verification Status

## Objective

Guarantee that protocol reconstruction and KB claims remain traceable to static/runtime evidence.

## Current Controls

- Promotion policy:
  - `high` confidence + valid evidence -> auto-promote.
  - `medium`/`low` -> review queue.
  - Missing/broken evidence -> rejected.
- Candidate queues are consumed after each promotion run.
- Validation command:

```bash
python3 scripts/kb_validate.py
```

- Docs sync command:

```bash
python3 scripts/kb_sync_docs.py
```

## Differential Verification

- Fixture diff command:

```bash
scripts/run_diff_verify.sh
```

- Latest report:
  - `captures/fixtures/verify-report.json`

## Regression Suite

```bash
scripts/run_regression.sh
```

Includes:

1. KB workflow unit tests.
2. Rust protocol/core/verify unit tests.
3. Fixture parity checks (login/search/transfer request/response).

## Residual Risk

- Runtime parity against official client network sessions requires running golden captures with a dedicated test account.
