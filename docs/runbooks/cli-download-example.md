# CLI Download Example (Current State)

This runbook documents how to download a single file with the current NeoSoulSeek CLI.

Use this only for content you are authorized to share/download.

## Scope

- Current state supports single-file transfer in both manual and orchestrated workflows.
- The new orchestrated command is `session download-auto` (`search -> select -> download`).
- Use deterministic local transfer when you need fully reproducible behavior.

## Prerequisites

```bash
cd rust
cargo test -q
```

For authenticated commands, set local credentials:

```bash
cp ../.env.example ../.env.local
# edit NSS_TEST_SERVER, NSS_TEST_USERNAME, NSS_TEST_PASSWORD
```

## Option A (Recommended): Deterministic Local Transfer

This is the most reliable way to validate download behavior today.

1. In terminal A, start a local upload server:

```bash
cd rust
SOURCE_FILE="/absolute/path/to/Track.flac"
SOURCE_SIZE="$(stat -f%z "$SOURCE_FILE")"

cargo run -q -p soul-cli -- transfer serve-upload \
  --bind 127.0.0.1:2242 \
  --manual \
  --decision accept \
  --source-file "$SOURCE_FILE"
```

2. In terminal B, download from that local peer:

```bash
cd rust
SOURCE_SIZE="$(stat -f%z "/absolute/path/to/Track.flac")"

cargo run -q -p soul-cli -- transfer download \
  --peer 127.0.0.1:2242 \
  --token 555 \
  --path "Music\\Sample\\Track.flac" \
  --size "$SOURCE_SIZE" \
  --output /tmp/Track.flac
```

3. Verify file integrity:

```bash
shasum -a 256 /absolute/path/to/Track.flac /tmp/Track.flac
```

Expected output shape:

- uploader side: accepted upload request and bytes sent
- downloader side: `transfer.download ok bytes=<n> output=/tmp/Track.flac`

## Option B: Orchestrated Search-Select-Download (`session download-auto`)

This command runs search collection, selects a result/file, and executes the transfer.

```bash
cd rust
cargo run -q -p soul-cli -- session download-auto \
  --client-version 160 \
  --minor-version 1 \
  --token 9001 \
  --query \"aphex twin\" \
  --transfer-token 555 \
  --result-index 0 \
  --file-index 0 \
  --search-timeout-secs 6 \
  --max-messages 32 \
  --output /tmp/Track-auto.flac
```

If peer lookup/connect should be overridden for deterministic tests:

```bash
cd rust
cargo run -q -p soul-cli -- session download-auto \
  --client-version 160 \
  --minor-version 1 \
  --token 9001 \
  --query \"runtime auto download\" \
  --transfer-token 555 \
  --peer 127.0.0.1:2242 \
  --skip-connect-probe \
  --output /tmp/Track-auto.flac
```

Expected output shape:

- `session.download-auto ok user=<u> path=<p> size=<n> peer=<host:port> token=<t> bytes=<n> output=<path>`

## Option C: Real Network Workflow (Experimental)

1. Run a search in an authenticated session:

```bash
cd rust
cargo run -q -p soul-cli -- session search \
  --client-version 160 \
  --minor-version 1 \
  --token 9001 \
  --query "aphex twin"
```

2. From search output, identify candidate file metadata:

- `username`
- `files[].file_path`
- `files[].file_size`

3. Run download once you have a reachable peer endpoint and transfer token:

```bash
cd rust
cargo run -q -p soul-cli -- transfer download \
  --peer <peer_host:peer_port> \
  --token <transfer_token> \
  --path "<virtual_path_from_result>" \
  --size <file_size_from_result> \
  --output /tmp/Track.flac
```

Notes:

- Real-network success depends on peer availability, queue policy, and transfer permission.
- For stable checks, prefer deterministic local peer mode (`--peer 127.0.0.1:2242 --skip-connect-probe`).

## Option D: Live Login + "Flim" Download Proof Test (Opt-in)

Use the dedicated script to validate live login and the orchestrated `download-auto` path for `aphex twin flim`.

```bash
scripts/test_live_login_download_flim.sh <username> <password>
```

Default behavior:

- server: `server.slsknet.org:2416`
- query: `aphex twin flim`
- result index: `0`
- file index: `0`
- output: `tmp/live-downloads/flim-<timestamp>.bin`

Optional overrides:

```bash
NSS_TEST_SERVER=server.slsknet.org:2416 \
NSS_TEST_QUERY="aphex twin flim" \
NSS_TEST_RESULT_INDEX=0 \
NSS_TEST_FILE_INDEX=0 \
NSS_TEST_OUTPUT_PATH=/tmp/flim.bin \
scripts/test_live_login_download_flim.sh <username> <password>
```

Runtime diagnostics knobs (useful while S9A-NEXT transfer closure is in progress):

```bash
NSS_MAX_CANDIDATE_ATTEMPTS=6 \
NSS_QUEUE_WAIT_SECS=180 \
NSS_DEBUG_TRANSFER=1 \
NSS_SEND_CONNECT_TOKEN_ON_PEER_INIT=0 \
NSS_SEND_CONNECT_TOKEN_ON_OUTBOUND_FILE_INIT=1 \
NSS_OUTBOUND_FILE_VARIANT_ORDER=f_init_first \
cargo run -q -p soul-cli -- session download-auto ...
```

Notes:

- `NSS_MAX_CANDIDATE_ATTEMPTS` limits distributed peer attempts per run.
- `NSS_QUEUE_WAIT_SECS` keeps queued transfer requests open waiting for peer grant.
- `NSS_DEBUG_TRANSFER=1` prints transfer-path diagnostics for runtime triage.
- `NSS_SEND_CONNECT_TOKEN_ON_PEER_INIT=0` disables peer-init `PM_SEND_CONNECT_TOKEN` as a diagnostic handshake variant.
- `NSS_SEND_CONNECT_TOKEN_ON_OUTBOUND_FILE_INIT=1` adds `PM_SEND_CONNECT_TOKEN` after outbound file-socket peer-init variants.
- `NSS_OUTBOUND_FILE_VARIANT_ORDER` controls outbound file init order (`no_init_first`, `f_init_first`, `p_init_first`).
