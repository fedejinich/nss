# PR 0007 - S4C: privileges/social control + peer-folder protocol batch

## Branch

- `codex/s4c-privileges-social-peer-folder`

## Objective

Close Stage 4C with:

1. Privileges/social-control protocol batch mapped and implemented.
2. Peer-folder protocol batch mapped and implemented.
3. Runtime evidence for S4C scenarios (authenticated privileges/social run + deterministic peer-folder run).
4. Semantic differential verification and regression coverage extended for S4C.
5. KB/docs/state synchronized for stage closure.

## Scope

### Runtime capture and evidence

- Added generator:
  - `tools/runtime/generate_stage4c_privileges_social_captures.py`
- Added redacted runs:
  - `captures/redacted/login-privileges-social`
  - `captures/redacted/peer-folder-local`
- Extended required differential runs in `scripts/run_diff_verify.sh`.

### Mapping and schema

- Expanded `analysis/ghidra/maps/message_map.csv` with S4C batch:
  - `SM_IGNORE_USER`
  - `SM_UNIGNORE_USER`
  - `SM_GET_OWN_PRIVILEGES_STATUS`
  - `SM_GET_USER_PRIVILEGES_STATUS`
  - `SM_GIVE_PRIVILEGE`
  - `SM_INFORM_USER_OF_PRIVILEGES`
  - `SM_INFORM_USER_OF_PRIVILEGES_ACK`
  - `PM_GET_SHARED_FILES_IN_FOLDER`
  - `PM_SHARED_FILES_IN_FOLDER`
- Regenerated:
  - `analysis/protocol/message_schema.json`
  - `docs/re/static/message-schema.md`
  - `docs/re/static/detangling.md`
  - `docs/verification/evidence-ledger.md`
  - `docs/state/protocol-matrix.md`

### Rust implementation

- `rust/protocol`:
  - Added S4C constants, payloads, enum variants, encode/decode logic, and builders.
  - Added folder response raw-byte handling (`directory + compressed payload bytes`).
- `rust/core`:
  - Added social/privilege methods:
    - `ignore_user(...)`
    - `unignore_user(...)`
    - `get_own_privileges_status(...)`
    - `get_user_privileges_status(...)`
    - `give_privilege(...)`
    - `inform_user_of_privileges(...)`
    - `inform_user_of_privileges_ack(...)`
- `rust/cli`:
  - Added session commands:
    - `session ignore-user --target-user <name>`
    - `session unignore-user --target-user <name>`
    - `session own-privileges`
    - `session user-privileges --target-user <name>`
    - `session give-privilege --target-user <name> --days <n>`
    - `session inform-privileges --token <n> --target-user <name>`
    - `session inform-privileges-ack --token <n>`
  - Added helper command:
    - `build-peer-folder-request --directory <path>`

### Tests and contracts

- Added Stage 4C contract test:
  - `tests/protocol/test_stage4c_privileges_social_contract.py`
- Extended Rust protocol/core tests for S4C messages and session operations.

### Documentation

- Added current CLI download walkthrough:
  - `docs/runbooks/cli-download-example.md`
- Linked runbook from:
  - `README.md`
  - `docs/index.md`

## S4C Contract Outcome

Required 9-message pack:

1. `SM_IGNORE_USER`
2. `SM_UNIGNORE_USER`
3. `SM_GET_OWN_PRIVILEGES_STATUS`
4. `SM_GET_USER_PRIVILEGES_STATUS`
5. `SM_GIVE_PRIVILEGE`
6. `SM_INFORM_USER_OF_PRIVILEGES`
7. `SM_INFORM_USER_OF_PRIVILEGES_ACK`
8. `PM_GET_SHARED_FILES_IN_FOLDER`
9. `PM_SHARED_FILES_IN_FOLDER`

Confidence outcome:

- `high=8`
- `medium=1`
- `low=0`

Deferred from batch:

- `SM_BAN_USER` (authoritative code/evidence unresolved).

## Validation

```bash
python3 scripts/kb_validate.py
scripts/run_diff_verify.sh
scripts/run_regression.sh
```

All checks passed on this branch snapshot.

## Runtime Verification Snapshot

- Official server: `server.slsknet.org:2242`
- Auth tuple: `160/1`
- S4C runtime scenarios:
  - `login-privileges-social` (authenticated server traffic including outbound/response codes `11`, `12`, `92`, `122`, `123`, `124`, `125`)
  - `peer-folder-local` (deterministic peer frames for codes `36`, `37`)

## Retrospective

### Was there a more maintainable approach?

Yes. Keeping protocol/batch contracts as tests (`test_stage4*.py`) plus generated schema/matrix avoids manual drift and makes stage closure binary (pass/fail) instead of subjective.

### What was reused to avoid double writing?

1. Existing redaction and semantic diff pipeline for new S4C captures.
2. Existing schema/doc generation flow (`derive_schema.py` + `kb_sync_docs.py` + protocol matrix generator).
3. Existing authenticated CLI session flow to add social/privilege commands without duplicating connection logic.

### What was removed or avoided to reduce maintenance surface?

1. Avoided bespoke S4C verification scripts by extending the existing required-run set in `scripts/run_diff_verify.sh`.
2. Avoided ad-hoc decoding in CLI by centralizing all new decode logic in `rust/protocol`.
3. Kept peer-folder payload parsing intentionally summary-level (`directory + compressed bytes`) and deferred deep decompression schema until dedicated follow-up scope.
