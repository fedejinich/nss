# Project Status

## Date

- 2026-02-14

## Current Phase

- KB-first gate implemented and enforced.
- Reverse flow extraction + runtime capture harness implemented.
- Rust protocol/client verification V1 scaffold implemented.

## Completed Tasks

- T0A Provisioned Zensical with reproducible installer (`scripts/setup_zensical.sh`).
- T0B Bootstrapped KB site (`zensical.toml`, docs skeleton).
- T0C Defined authoritative map schemas and files.
- T0D Implemented confidence-based promotion workflow and validators.
- T0E Published KB-first runbook.
- T1 Installed baseline reverse/trace tools: Ghidra, Frida, tcpdump verified.
- T2 Established state and process tracking docs.
- T3 Generated forensic intake report and evidence bundle.
- T4 Implemented and validated Ghidra headless pipeline (`scripts/ghidra_pipeline.sh`).
- T5 Evaluated Binary Ninja as optional/non-blocking.
- T6 Added initial evidence-backed map baseline (`name_map`, `message_map.csv`).
- T7 Extracted static search/download flow (`scripts/extract_search_download_flow.sh`, `analysis/re/flow_graph.json`).
- T8 Added Frida hooks for key server/peer/transfer functions (`frida/hooks/soulseek_trace.js`).
- T9 Added synchronized capture harness (`tools/runtime/capture_harness.py`).
- T10 Added golden capture workflow (`scripts/capture_golden.sh`) and fixture baseline.
- T11 Derived protocol schema from evidence (`analysis/protocol/message_schema.json`).
- T12 Created Rust workspace (`rust/protocol`, `rust/core`, `rust/cli`, `rust/verify`).
- T13 Implemented framing and codec in Rust protocol crate.
- T14 Implemented login/search send paths in Rust core/cli.
- T15 Implemented single-file download path in Rust core.
- T16 Implemented differential fixture verifier (`scripts/run_diff_verify.sh`, `rust/verify`).
- T17 Added regression suite (`scripts/run_regression.sh` + Python/Rust tests).
- T18 Published V1 closure/status artifacts.

## Artifacts

- `analysis/ghidra/maps/name_map.json`
- `analysis/ghidra/maps/data_map.json`
- `analysis/ghidra/maps/message_map.csv`
- `analysis/re/flow_graph.json`
- `analysis/protocol/message_schema.json`
- `docs/re/static/search-download-flow.md`
- `docs/re/static/message-schema.md`
- `docs/verification/evidence-ledger.md`
- `captures/fixtures/*.hex`
- `captures/fixtures/verify-report.json`

## Operational Note

- Golden captures with real account traffic are ready to run, but production-like capture content depends on test credentials and operator execution window.
