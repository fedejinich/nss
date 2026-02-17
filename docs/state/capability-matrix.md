# Capability Matrix

This matrix tracks delivery capabilities, final gates, and critical-path blockers.

## Snapshot

- Total capabilities: `29`
- Done: `16`
- In progress: `2`
- Planned: `11`
- Required for final: `18`
- Required done: `8`
- Required pending: `10`
- Runtime verified/static: `131/0`
- Semantic-tail gaps: `0`

## Final Gates

| gate | status | required | blockers |
|---|---|---|---|
| `G0-PLAN-PUBLISHED` | `pass` | Roadmap, stage/capability registries, and TODO are aligned to S9P static/runtime tracks | - |
| `G1-TOOLING-READY` | `pass` | Frida, tcpdump, Ghidra headless, and automation baseline are verified on host | - |
| `G2A-STATIC-ARCH-BASELINE` | `pass` | Transfer stack and dispatch architecture baseline recovered from static evidence | - |
| `G2B-STATIC-FORMAT-BASELINE` | `pass` | Persistence-critical format surfaces are mapped with reader/writer candidates and xref-backed evidence | - |
| `G3A-RUNTIME-CAPTURE-BASELINE` | `blocked` | Official app protocol scenarios can be captured reproducibly with redacted artifacts | CAP-S9P-RUNTIME-CAPTURE-BASELINE status=in_progress; CAP-S9P-RUNTIME-CAPTURE-BASELINE: Need broader official transfer scenario corpus (beyond bootstrap login/search/download traces) before parity synthesis. |
| `G3B-RUNTIME-FORMAT-BASELINE` | `blocked` | Official app persistence I/O events are captured/redacted with payload sampling policy | CAP-S9P-RUNTIME-FORMAT-BASELINE status=in_progress; CAP-S9P-RUNTIME-FORMAT-BASELINE: QSettings and QDataStream symbol hooks are still unresolved on the active specimen, limiting settings/export-import runtime evidence depth. |
| `G4-FLIM-E2E-SUCCESS` | `blocked` | NeoSoulSeek Flim run writes non-zero bytes | CAP-S9P-FLIM-E2E status=planned; CAP-S9P-FLIM-E2E: bytes_written is still zero in live runs |
| `G5-TRANSFER-PARITY` | `blocked` | Transfer scenario semantic diff is clean against official captures | CAP-S9P-TRANSFER-PARITY status=planned; CAP-S9P-TRANSFER-PARITY: Transfer scenarios still diverge from official captures |
| `G6-PROTOCOL-PARITY` | `blocked` | Protocol parity matrix is closed for scoped message surface | CAP-S9P-PROTOCOL-PARITY status=planned; CAP-S9P-PROTOCOL-PARITY: Transfer parity gate not closed |
| `G7-ARCH-FORMAT-SYNTHESIS` | `blocked` | Architecture map, format map, and replicability classification are complete and evidence-linked | CAP-S9P-ARCH-FORMAT-SYNTHESIS status=planned; CAP-S9P-ARCH-FORMAT-SYNTHESIS: Architecture/format synthesis gate not closed |
| `G8-KB-DASHBOARD-SYNC` | `blocked` | State docs, evidence ledger, and generated dashboard artifacts are consistent | CAP-S9P-KB-SYNC status=planned; CAP-S9P-KB-SYNC: Downstream S9P closure gates remain open |

## Capability Table

| id | title | domain | status | required final | depends_on | blockers | evidence |
|---|---|---|---|---|---|---|---|
| `CAP-PROTOCOL-MAPPED` | Protocol mapped and implemented | `protocol` | `done` | yes | - | - | docs/state/protocol-matrix.md |
| `CAP-RUNTIME-COMPLETE` | Runtime evidence for all protocol messages | `runtime` | `done` | yes | CAP-PROTOCOL-MAPPED | - | docs/state/runtime-coverage.md |
| `CAP-SEMANTIC-DEPTH` | Semantic depth closure for partial tails | `schema` | `done` | yes | CAP-RUNTIME-COMPLETE | - | analysis/protocol/message_schema.json |
| `CAP-CLI-DOWNLOAD-AUTO` | Automated search-select-download flow in CLI/core | `cli_core` | `done` | yes | CAP-SEMANTIC-DEPTH | - | docs/runbooks/cli-download-example.md |
| `CAP-DASHBOARD-OPS` | Capabilities and critical-path dashboard | `docs` | `done` | no | CAP-RUNTIME-COMPLETE | - | docs/state/capability-dashboard.html |
| `CAP-TUI-V1` | Minimal TUI for core transfer | `tui` | `done` | no | CAP-CLI-DOWNLOAD-AUTO, CAP-DASHBOARD-OPS | - | rust/tui/src/main.rs |
| `CAP-REDACTION-HARDENING` | Redaction metadata hardening | `security` | `done` | no | CAP-TUI-V1 | - | docs/verification/capture-redaction-policy.md |
| `CAP-PACKAGING-RELEASE` | Release packaging workflow | `release` | `done` | no | CAP-TUI-V1 | - | docs/runbooks/release-packaging.md |
| `CAP-RECOVERY-RUNBOOKS` | Failure recovery runbooks | `ops` | `done` | no | CAP-TUI-V1 | - | docs/runbooks/failure-recovery.md |
| `CAP-CLOSURE-CHECKLIST` | Final closure checklist and audit | `state` | `done` | no | CAP-REDACTION-HARDENING, CAP-PACKAGING-RELEASE, CAP-RECOVERY-RUNBOOKS | - | docs/state/final-closure-checklist.md |
| `CAP-RELEASE-HARDENING` | Release hardening and final gates | `release` | `done` | no | CAP-CLOSURE-CHECKLIST | - | docs/state/verification-status.md |
| `CAP-TUI-S9A-SIMPLIFIED` | S9A TUI simplification and persistence semantics | `tui` | `done` | no | CAP-TUI-V1, CAP-RELEASE-HARDENING | - | docs/runbooks/tui-core-transfer.md, rust/tui/src/main.rs |
| `CAP-S9P-PLAN-PUBLISHED` | S9P v3 plan publication with architecture+format tracks | `state` | `done` | yes | CAP-TUI-S9A-SIMPLIFIED | - | TODO-CODEX.md, docs/state/roadmap.md, analysis/state/stage_registry.json, analysis/state/capability_registry.json |
| `CAP-S9P-TOOLING-READY` | S9P v3 static/runtime tooling readiness | `runtime` | `done` | yes | CAP-S9P-PLAN-PUBLISHED | - | docs/state/verification-status.md, docs/verification/evidence-ledger.md, tools/runtime/official_runner.py, scripts/capture_golden.sh, captures/raw/20260216T235428Z-s9p-v3-t05-runner-both-debug-r2/manifest.raw.json |
| `CAP-S9P-STATIC-ARCH-BASELINE` | Static architecture baseline for transfer/protocol dispatch | `static` | `done` | yes | CAP-S9P-TOOLING-READY | - | analysis/re/official_architecture_map.json, docs/state/decompilation-status.md, docs/re/static/search-download-flow.md |
| `CAP-S9P-STATIC-FORMAT-BASELINE` | Static baseline for persistence-critical formats | `static` | `done` | yes | CAP-S9P-PLAN-PUBLISHED | - | tools/re/extract_format_candidates.py, analysis/re/official_file_format_map.json, docs/re/static/file-format-map.md |
| `CAP-S9P-RUNTIME-CAPTURE-BASELINE` | Runtime protocol capture baseline from official app | `runtime` | `in_progress` | yes | CAP-S9P-TOOLING-READY | Need broader official transfer scenario corpus (beyond bootstrap login/search/download traces) before parity synthesis. | captures/raw, captures/redacted, docs/verification/evidence-ledger.md, captures/raw/20260216T235428Z-s9p-v3-t05-runner-both-debug-r2/manifest.raw.json, captures/redacted/20260216T235428Z-s9p-v3-t05-runner-both-debug-r2/redaction-summary.json |
| `CAP-S9P-RUNTIME-FORMAT-BASELINE` | Runtime baseline for persistence file-I/O | `runtime` | `in_progress` | yes | CAP-S9P-TOOLING-READY, CAP-S9P-STATIC-FORMAT-BASELINE | QSettings and QDataStream symbol hooks are still unresolved on the active specimen, limiting settings/export-import runtime evidence depth. | frida/hooks/soulseek_io_trace.js, captures/redacted, docs/verification/evidence-ledger.md, captures/raw/20260217T000258Z-s9p-v3-t04f-startup-io-r4/manifest.raw.json, captures/redacted/20260217T000258Z-s9p-v3-t04f-startup-io-r4/redaction-summary.json, captures/raw/20260217T010817Z-i3-t04-io-runtime-r5/manifest.raw.json, captures/raw/20260217T010817Z-i3-t04-io-runtime-r5/io-events.raw.jsonl, captures/redacted/20260217T010817Z-i3-t04-io-runtime-r5/redaction-summary.json |
| `CAP-S9P-ARCH-MAP` | Official architecture and state-machine map | `synthesis` | `planned` | yes | CAP-S9P-STATIC-ARCH-BASELINE, CAP-S9P-RUNTIME-CAPTURE-BASELINE, CAP-S9P-STATIC-FORMAT-BASELINE, CAP-S9P-RUNTIME-FORMAT-BASELINE | Static/runtime architecture+format synthesis pending | analysis/re/official_architecture_map.json, analysis/re/official_file_format_map.json, docs/state/decompilation-status.md, docs/re/static/file-format-map.md |
| `CAP-S9P-REPLICABILITY-MATRIX` | Exact vs constrained vs non-goal replicability classification | `synthesis` | `planned` | yes | CAP-S9P-ARCH-MAP | Architecture and file-format map not finalized | analysis/re/protocol_parity_matrix.json, analysis/re/official_file_format_map.json, docs/state/project-status.md |
| `CAP-S9P-TRANSFER-DIFF` | Transfer semantic diff comparator coverage | `verify` | `planned` | yes | CAP-S9P-REPLICABILITY-MATRIX | Transfer scenario corpus not finalized | rust/verify/src/lib.rs, scripts/run_diff_verify.sh |
| `CAP-S9P-FLIM-E2E` | Flim E2E transfer success | `runtime` | `planned` | yes | CAP-S9P-TRANSFER-DIFF | bytes_written is still zero in live runs | tmp/logs, docs/verification/evidence-ledger.md |
| `CAP-S9P-TRANSFER-PARITY` | Transfer stack parity | `protocol` | `planned` | yes | CAP-S9P-FLIM-E2E | Transfer scenarios still diverge from official captures | docs/state/verification-status.md, scripts/run_diff_verify.sh |
| `CAP-S9P-PROTOCOL-PARITY` | Protocol parity matrix closure | `protocol` | `planned` | yes | CAP-S9P-TRANSFER-PARITY | Transfer parity gate not closed | analysis/re/protocol_parity_matrix.json, docs/state/protocol-backlog.md |
| `CAP-S9P-ARCH-FORMAT-SYNTHESIS` | Architecture and file-format synthesis closure | `synthesis` | `planned` | yes | CAP-S9P-PROTOCOL-PARITY, CAP-S9P-REPLICABILITY-MATRIX | Architecture/format synthesis gate not closed | analysis/re/official_architecture_map.json, analysis/re/official_file_format_map.json, docs/re/static/file-format-map.md, docs/state/decompilation-status.md |
| `CAP-S9P-KB-SYNC` | S9P closure documentation and dashboard sync | `docs` | `planned` | yes | CAP-S9P-ARCH-FORMAT-SYNTHESIS | Downstream S9P closure gates remain open | docs/state/project-status.md, docs/state/verification-status.md, docs/state/decompilation-status.md, docs/state/capability-matrix.md |
| `CAP-CLI-JSON-MVP` | CLI JSON mode for GUI integration | `cli` | `planned` | no | CAP-S9P-PROTOCOL-PARITY | S9P has priority and must close first | docs/state/roadmap.md |
| `CAP-SWIFT-GUI-MVP` | SwiftUI macOS GUI MVP | `gui_macos` | `planned` | no | CAP-CLI-JSON-MVP | S9P parity track not closed | docs/state/roadmap.md |
| `CAP-NEXT-GUI-MVP` | Next.js web GUI MVP | `gui_web` | `planned` | no | CAP-SWIFT-GUI-MVP | S9P parity track not closed | docs/state/roadmap.md |

## Critical Path

- `CAP-S9P-KB-SYNC` `planned` deps=`CAP-S9P-ARCH-FORMAT-SYNTHESIS' blockers=`Downstream S9P closure gates remain open`
- `CAP-S9P-ARCH-FORMAT-SYNTHESIS` `planned` deps=`CAP-S9P-PROTOCOL-PARITY, CAP-S9P-REPLICABILITY-MATRIX' blockers=`Architecture/format synthesis gate not closed`
- `CAP-S9P-PROTOCOL-PARITY` `planned` deps=`CAP-S9P-TRANSFER-PARITY' blockers=`Transfer parity gate not closed`
- `CAP-S9P-TRANSFER-PARITY` `planned` deps=`CAP-S9P-FLIM-E2E' blockers=`Transfer scenarios still diverge from official captures`
- `CAP-S9P-FLIM-E2E` `planned` deps=`CAP-S9P-TRANSFER-DIFF' blockers=`bytes_written is still zero in live runs`
- `CAP-S9P-TRANSFER-DIFF` `planned` deps=`CAP-S9P-REPLICABILITY-MATRIX' blockers=`Transfer scenario corpus not finalized`
- `CAP-S9P-REPLICABILITY-MATRIX` `planned` deps=`CAP-S9P-ARCH-MAP' blockers=`Architecture and file-format map not finalized`
- `CAP-S9P-ARCH-MAP` `planned` deps=`CAP-S9P-STATIC-ARCH-BASELINE, CAP-S9P-RUNTIME-CAPTURE-BASELINE, CAP-S9P-STATIC-FORMAT-BASELINE, CAP-S9P-RUNTIME-FORMAT-BASELINE' blockers=`Static/runtime architecture+format synthesis pending`
- `CAP-S9P-RUNTIME-CAPTURE-BASELINE` `in_progress` deps=`CAP-S9P-TOOLING-READY' blockers=`Need broader official transfer scenario corpus (beyond bootstrap login/search/download traces) before parity synthesis.`
- `CAP-S9P-RUNTIME-FORMAT-BASELINE` `in_progress` deps=`CAP-S9P-TOOLING-READY, CAP-S9P-STATIC-FORMAT-BASELINE' blockers=`QSettings and QDataStream symbol hooks are still unresolved on the active specimen, limiting settings/export-import runtime evidence depth.`

## Regeneration

```bash
python3 tools/state/generate_capability_matrix.py
```
