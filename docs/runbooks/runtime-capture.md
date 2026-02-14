# Runtime Capture Runbook

## Goal

Collect synchronized runtime evidence for search/download behavior:

- Frida hooks (`frida-events.jsonl`)
- Network traffic (`traffic.pcap`)
- Session manifest (`manifest.json`)

## Prerequisites

1. SoulseekQt running locally.
2. Tooling initialized with `scripts/setup_toolchain.sh`.
3. Packet capture permission for `tcpdump` (root/admin entitlement on macOS).

## Standard Session

```bash
scripts/capture_session.sh
```

Environment variables:

- `DURATION` (default `60`)
- `PROCESS_NAME` (default `SoulseekQt`)
- `IFACE` (optional; default auto)
- `BPF_FILTER` (default `tcp`)
- `SKIP_PCAP=1` to collect Frida-only traces

## Golden Session

```bash
SCENARIO=login-search-download DURATION=120 scripts/capture_golden.sh
```

Outputs are stored under `captures/golden/<timestamp>-<scenario>/`.

## Evidence Registration

After each run:

1. Add run directory path to `docs/verification/evidence-ledger.md`.
2. Promote only high-confidence findings to `analysis/ghidra/maps/name_map.json` or `analysis/ghidra/maps/data_map.json`.
3. Keep medium/low confidence in `analysis/ghidra/queue/review_queue.jsonl`.
