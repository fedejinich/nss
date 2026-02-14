# Decompilation Status

## Objective

Build a reproducible static/runtime reverse workflow and maintain evidence-backed mappings for SoulseekQt.

## Current

- Binary target present: `SoulseekQt-2025-10-11.dmg`
- Extracted binary: `analysis/binaries/SoulseekQt`
- Static flow extraction implemented:
  - `evidence/reverse/search_download_symbols_nm.txt`
  - `evidence/reverse/search_download_strings.txt`
  - `evidence/reverse/disasm/*.txt`
  - `analysis/re/flow_graph.json`
- Authoritative mapping storage initialized and populated:
  - `analysis/ghidra/maps/name_map.json`
  - `analysis/ghidra/maps/data_map.json`
  - `analysis/ghidra/maps/message_map.csv`

## Runtime Trace Path

- Frida hooks: `frida/hooks/soulseek_trace.js`
- Frida collector: `tools/runtime/frida_capture.py`
- Synchronized harness: `tools/runtime/capture_harness.py`
- Golden runner: `scripts/capture_golden.sh`

## Remaining Manual Operation

- Execute golden captures with test account credentials to enrich runtime evidence and refine medium-confidence assumptions.
