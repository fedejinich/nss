# NeoSoulSeek

NeoSoulSeek is a KB-first project to build an evolvable Soulseek client.

Current objective:

- Deliver core functionality (`login`, `search`, single-file `download`, manual accept/deny `upload`).
- Map the Soulseek protocol with traceable evidence.
- Iterate in stages instead of targeting a 1:1 clone of the official client.

## Requirements

- `python3`
- `git`
- `cargo` (Rust)
- macOS with `tcpdump` for runtime captures

## Start Zensical

1. Install Zensical inside the project virtual environment:

```bash
scripts/setup_zensical.sh
```

2. Build the KB site:

```bash
./.venv-tools/bin/zensical build -f zensical.toml
```

3. Serve locally:

```bash
./.venv-tools/bin/zensical serve -f zensical.toml -a 127.0.0.1:8000
```

Local site: `http://127.0.0.1:8000`

## Project Usage

### 1. KB-first workflow (mandatory)

```bash
python3 scripts/kb_promote.py
python3 scripts/kb_sync_docs.py
python3 scripts/kb_validate.py
```

### 2. Protocol schema derivation

```bash
scripts/derive_message_schema.sh
```

Artifacts:

- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`
- `docs/re/static/message-schema.md`

### 3. Runtime captures (raw -> redacted)

Standard capture session:

```bash
scripts/capture_session.sh
```

Specific scenario:

```bash
SCENARIO=login-search-download DURATION=120 scripts/capture_golden.sh
```

Manual redaction:

```bash
RUN_DIR=captures/raw/<run_id> scripts/redact_capture_run.sh
```

Policy:

- Local-only raw artifacts: `captures/raw/*`
- Versioned redacted artifacts: `captures/redacted/*`

### 4. Rust SDK/CLI

Run tests:

```bash
cd rust
cargo test
```

Main commands:

```bash
cd rust
cargo run -q -p soul-cli -- session login --server <host:port> --username <user> --password <plain>
cargo run -q -p soul-cli -- session search --server <host:port> --username <user> --password <plain> --token 123 --query "aphex twin"
cargo run -q -p soul-cli -- session probe-login-version --server <host:port> --username <user> --password <plain>
cargo run -q -p soul-cli -- room list
cargo run -q -p soul-cli -- room join --room nicotine
cargo run -q -p soul-cli -- room members --room nicotine --timeout-secs 6
cargo run -q -p soul-cli -- room watch --room nicotine --timeout-secs 15
cargo run -q -p soul-cli -- room leave --room nicotine
cargo run -q -p soul-cli -- transfer download --peer <host:port> --token 555 --path "Music\\Track.flac" --size 1234 --output /tmp/out.bin
cargo run -q -p soul-cli -- transfer serve-upload --manual --decision accept --source-file /tmp/file.bin
cargo run -q -p soul-cli -- verify captures --run login-join-room-presence --base-dir ../captures/redacted --mode semantic
```

Environment credentials (`.env.local`, local-only):

```bash
cp .env.example .env.local
# edit NSS_TEST_SERVER / NSS_TEST_USERNAME / NSS_TEST_PASSWORD
```

### 5. Differential verification and regression

```bash
scripts/run_diff_verify.sh
scripts/run_regression.sh
```

## Structure

- `analysis/`: authoritative maps and static artifacts.
- `captures/`: fixtures, local raw runs, versioned redacted runs.
- `docs/`: runbooks, status pages, and evidence documentation.
- `evidence/`: forensic/reverse evidence.
- `frida/`: runtime hooks.
- `rust/`: `protocol`, `core`, `cli`, `verify` crates.
- `scripts/`: reproducible workflows.
- `tools/`: KB/protocol/runtime utilities.
