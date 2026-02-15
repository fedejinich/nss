# Capability Matrix

This matrix tracks delivery capabilities, final gates, and critical-path blockers.

## Snapshot

- Total capabilities: `11`
- Done: `11`
- In progress: `0`
- Planned: `0`
- Required for final: `10`
- Required done: `10`
- Required pending: `0`
- Runtime verified/static: `131/0`
- Semantic-tail gaps: `0`

## Final Gates

| gate | status | required | blockers |
|---|---|---|---|
| `FG-RUNTIME-100` | `pass` | verified_runtime=131 and verified_static=0 | - |
| `FG-SEMANTIC-DEPTH` | `pass` | no raw_tail/raw_payload unresolved schema fields | - |
| `FG-CORE-AUTO-DL` | `pass` | session download-auto succeeds in authenticated flow | - |
| `FG-TUI-V1` | `pass` | login/search/select/download/monitor/upload-decision available in TUI | - |
| `FG-RELEASE-HARDENING` | `pass` | config/log redaction, packaging, recovery runbooks, and closure checklist complete | - |

## Capability Table

| id | title | domain | status | required final | depends_on | blockers | evidence |
|---|---|---|---|---|---|---|---|
| `CAP-PROTOCOL-MAPPED` | Protocol mapped and implemented | `protocol` | `done` | yes | - | - | docs/state/protocol-matrix.md |
| `CAP-RUNTIME-COMPLETE` | Runtime evidence for all protocol messages | `runtime` | `done` | yes | CAP-PROTOCOL-MAPPED | - | docs/state/runtime-coverage.md |
| `CAP-SEMANTIC-DEPTH` | Semantic depth closure for partial tails | `schema` | `done` | yes | CAP-RUNTIME-COMPLETE | - | analysis/protocol/message_schema.json |
| `CAP-CLI-DOWNLOAD-AUTO` | Automated search-select-download flow in CLI/core | `cli_core` | `done` | yes | CAP-SEMANTIC-DEPTH | - | docs/runbooks/cli-download-example.md |
| `CAP-DASHBOARD-OPS` | Capabilities and critical-path dashboard | `docs` | `done` | no | CAP-RUNTIME-COMPLETE | - | docs/state/capability-dashboard.html |
| `CAP-TUI-V1` | Minimal TUI for core transfer | `tui` | `done` | yes | CAP-CLI-DOWNLOAD-AUTO, CAP-DASHBOARD-OPS | - | rust/tui/src/main.rs |
| `CAP-REDACTION-HARDENING` | Redaction metadata hardening | `security` | `done` | yes | CAP-TUI-V1 | - | docs/verification/capture-redaction-policy.md |
| `CAP-PACKAGING-RELEASE` | Release packaging workflow | `release` | `done` | yes | CAP-TUI-V1 | - | docs/runbooks/release-packaging.md |
| `CAP-RECOVERY-RUNBOOKS` | Failure recovery runbooks | `ops` | `done` | yes | CAP-TUI-V1 | - | docs/runbooks/failure-recovery.md |
| `CAP-CLOSURE-CHECKLIST` | Final closure checklist and audit | `state` | `done` | yes | CAP-REDACTION-HARDENING, CAP-PACKAGING-RELEASE, CAP-RECOVERY-RUNBOOKS | - | docs/state/final-closure-checklist.md |
| `CAP-RELEASE-HARDENING` | Release hardening and final gates | `release` | `done` | yes | CAP-CLOSURE-CHECKLIST | - | docs/state/verification-status.md |

## Critical Path

No pending required capabilities.

## Regeneration

```bash
python3 tools/state/generate_capability_matrix.py
```
