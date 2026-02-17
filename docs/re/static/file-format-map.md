# SoulseekQt File-Format Map (Persistence-Critical v1)

## Objective

Provide a canonical static baseline for persistence-critical formats in the official SoulseekQt client and prepare runtime validation targets for S9P v3.

## Method

1. Specimen lock: current macOS SoulseekQt build.
2. No guessing policy: map entries only when supported by symbol/method/string evidence.
3. Static-first baseline: seed runtime capture targets (`soulseek_io_trace.js`) for confirmation.

Primary extraction artifact:

- `analysis/re/format_candidates.json`

Canonical format map:

- `analysis/re/official_file_format_map.json`

## Format Coverage (v1)

| ID | Purpose | Static confidence | Replicability (current) | Runtime closure needed |
|---|---|---|---|---|
| `FMT-SETTINGS-QSETTINGS` | UI/transfer option persistence via QSettings | high | behavior parity feasible | Resolve concrete backend file path per profile |
| `FMT-TRANSFER-STATE` | queued/in-progress/completed transfer state | high | behavior parity feasible with constraints | Confirm exact record framing and filename(s) |
| `FMT-USERLIST-IMPORT` | legacy `hotlist.cfg` import | medium | exact parity likely feasible | Confirm parser framing and edge-case handling |
| `FMT-CLIENT-DATA-EXPORT-IMPORT` | full client data backup/restore | high | behavior parity feasible | Confirm snapshot boundaries and versioning behavior |
| `FMT-SHARE-SCAN-CACHE` | share scan/index cache persistence | medium | behavior parity feasible with constraints | Confirm cache files, invalidation, and rebuild semantics |
| `FMT-SEARCH-HISTORY-PREFERENCES` | search history + related options | high | exact parity likely feasible | Confirm key naming and retention semantics |

## Static Evidence Anchors

- `evidence/ui_audit/decomp/nm_demangled_full.txt`
- `evidence/ui_audit/decomp/mainwindow_methods.txt`
- `evidence/ui_audit/decomp/transfer_methods.txt`
- `evidence/ui_audit/ui_strings_feature_candidates.txt`
- `docs/re/static/file-format-candidates.md`

## Runtime Next Steps

1. Completed baseline:
   - deterministic profile-root captures are running through:
     - `tools/runtime/capture_harness.py`
     - `scripts/capture_golden.sh`
     - `tools/runtime/official_runner.py`
   - arm64 hook offsets were reconciled from static `nm` evidence and applied to:
     - `frida/hooks/soulseek_io_trace.js`
   - Frida target selection is now disambiguated by executable path token:
     - `tools/runtime/frida_capture.py --process-path-contains ...`
   - redacted I/O artifacts are being produced under:
     - `captures/redacted/*/io-events.redacted.jsonl`
   - high-signal transfer-store persistence events are now observed in deterministic runs, for example:
     - `captures/raw/20260217T010817Z-i3-t04-io-runtime-r5/io-events.raw.jsonl`
     - `writestring` (301 events), `mainwindow_save_data_enter`, `datasaver_save_enter`, `datasaver_save_to_file_enter`
2. Current limitation:
   - QSettings and QDataStream hooks remain unresolved on the active specimen, so settings/export-import key-level payload evidence is still incomplete.
3. Next closure step:
   - expand trigger scenarios and refine Qt symbol resolution until each persistence format has reader+writer runtime samples strong enough for confidence promotion.
