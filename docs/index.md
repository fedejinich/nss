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
- `docs/verification/evidence-ledger.md`
- `docs/re/static/detangling.md`

## Commands

```bash
python3 scripts/kb_promote.py
python3 scripts/kb_sync_docs.py
python3 scripts/kb_validate.py
```
