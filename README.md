# NeoSoulSeek

NeoSoulSeek is a reverse-engineering and protocol reconstruction project for the SoulseekQt client.

## Current Status

- KB-first workflow is active (evidence-gated promotion).
- Static flow extraction for search/download is implemented.
- Runtime tracing pipeline (Frida + PCAP harness) is implemented.
- Rust workspace includes protocol/core/cli/verify crates with fixture parity checks.

## Repository Layout

- `analysis/` authoritative maps, schemas, static reverse artifacts.
- `docs/` runbooks, state docs, verification ledger.
- `evidence/` forensic and reverse evidence.
- `frida/` runtime hooks.
- `rust/` protocol/client implementation and verifier.
- `scripts/` reproducible workflows (extract, capture, verify, regression).

## Quick Start

```bash
python3 scripts/kb_promote.py
python3 scripts/kb_sync_docs.py
python3 scripts/kb_validate.py
scripts/extract_search_download_flow.sh
scripts/derive_message_schema.sh
scripts/run_regression.sh
```

## Runtime Capture

```bash
scripts/capture_session.sh
SCENARIO=login-search-download DURATION=120 scripts/capture_golden.sh
```

## Differential Verification

```bash
scripts/run_diff_verify.sh
```

Report output:

- `captures/fixtures/verify-report.json`
