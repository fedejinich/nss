# Verification Status

## Objective

Ensure evidence traceability and semantic protocol parity for Stage 4C (privileges/social control + peer-folder batch) while preserving all previous stage guarantees.

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
   - `login-room-moderation`
   - `peer-advanced-local`
   - `login-privileges-social`
   - `peer-folder-local`
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

## Stage 4C Coverage Status

S4C 9-message batch is present in:

- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`

Messages:

- `SM_IGNORE_USER`
- `SM_UNIGNORE_USER`
- `SM_GET_OWN_PRIVILEGES_STATUS`
- `SM_GET_USER_PRIVILEGES_STATUS`
- `SM_GIVE_PRIVILEGE`
- `SM_INFORM_USER_OF_PRIVILEGES`
- `SM_INFORM_USER_OF_PRIVILEGES_ACK`
- `PM_GET_SHARED_FILES_IN_FOLDER`
- `PM_SHARED_FILES_IN_FOLDER`

Confidence distribution for S4C batch:

- `high=8`
- `medium=1`
- `low=0`

Protocol matrix status:

- Tracked message names from static string tables: `130`
- Implemented + mapped: `56`
- Missing: `74`
- Matrix source: `docs/state/protocol-matrix.md`

## Runtime Evidence Snapshot

- Official server: `server.slsknet.org:2242`
- Auth tuple used: `160/1`
- S4C runtime redacted runs:
  - `captures/redacted/login-privileges-social`
  - `captures/redacted/peer-folder-local`
- Privileges/social runtime scenario includes authenticated outbound/request-response frames for codes `11`, `12`, `92`, `122`, `123`, `124`, `125`.
- Peer-folder deterministic runtime scenario includes peer frames `36`, `37`.

## Residual Risk

- `SM_GET_USER_PRIVILEGES_STATUS` remains `medium` because code `122` is deprecated in spec and behavior can vary by server implementation.
- `SM_BAN_USER` remains unmapped in the backlog pending authoritative code-level evidence.
- Peer-folder response payload is tracked as `directory + compressed bytes`; deep schema of compressed listing remains for a follow-up parsing pass.
