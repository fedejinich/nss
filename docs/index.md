# NeoSoulSeek Knowledge Base

This documentation is the project memory for decompilation and parity work against SoulseekQt.

## Core Rules

- The binary is the spec.
- No rename is accepted without evidence.
- Only high-confidence candidates are auto-promoted into authoritative maps.
- Medium/low confidence candidates are queued for review.
- Project memory must be updated continuously (`TODO-CODEX.md`, `AGENTS.md`, and canonical KB docs/artifacts).

## Authoritative Sources

- `analysis/ghidra/maps/name_map.json`
- `analysis/ghidra/maps/data_map.json`
- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`
- `docs/verification/evidence-ledger.md`
- `docs/verification/capture-redaction-policy.md`
- `docs/re/static/detangling.md`
- `docs/re/static/search-download-flow.md`
- `docs/re/static/message-schema.md`
- `docs/state/stage2-parity-audit.md`
- `docs/state/protocol-backlog.md`
- `docs/state/protocol-matrix.md`
- `docs/state/roadmap.md`
- `docs/runbooks/documentation-discipline.md`
- `docs/runbooks/cli-download-example.md`

## Commands

```bash
python3 scripts/kb_promote.py
python3 scripts/kb_sync_docs.py
python3 scripts/kb_validate.py
scripts/extract_search_download_flow.sh
scripts/derive_message_schema.sh
python3 tools/protocol/generate_protocol_matrix.py
scripts/run_diff_verify.sh
scripts/run_regression.sh
.venv-tools/bin/zensical build -f zensical.toml
```
