# soul-dec Knowledge Base

This documentation is the project memory for decompilation and parity work against SoulseekQt.

## Core Rules

- The binary is the spec.
- No rename is accepted without evidence.
- Only high-confidence candidates are auto-promoted into authoritative maps.
- Medium/low confidence candidates are queued for review.

## Authoritative Sources

- `analysis/ghidra/maps/name_map.json`
- `analysis/ghidra/maps/data_map.json`
- `analysis/ghidra/maps/message_map.csv`
- `analysis/protocol/message_schema.json`
- `docs/verification/evidence-ledger.md`
- `docs/re/static/detangling.md`
- `docs/re/static/search-download-flow.md`
- `docs/re/static/message-schema.md`

## Commands

```bash
python3 scripts/kb_promote.py
python3 scripts/kb_sync_docs.py
python3 scripts/kb_validate.py
scripts/extract_search_download_flow.sh
scripts/derive_message_schema.sh
scripts/run_regression.sh
```
