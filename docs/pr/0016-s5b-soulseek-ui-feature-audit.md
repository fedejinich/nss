# PR 0016 - S5B Soulseek UI + Functionality Audit

## Scope

Stage 5B delivers a research-only, evidence-backed inventory of SoulseekQt functionality with explicit UI mapping and a required second-pass closure.

Primary deliverable:

- `docs/state/soulseek-feature-inventory.md`

## What Was Added

1. Full app baseline evidence bundle under `evidence/ui_audit/`:
- binary hash
- plist metadata
- code-signing metadata
- binary type + linked frameworks
- app content tree
- extracted UI/control strings

2. External-source snapshots and structured extracts:
- `evidence/ui_audit/external/changelog_structured.json`
- `evidence/ui_audit/external/news_structured.json`
- `evidence/ui_audit/external/forum_topics_structured.json`

3. Static reverse/decomp extraction for UI/feature bridge mapping:
- `evidence/ui_audit/decomp/mainwindow_methods.txt`
- `evidence/ui_audit/decomp/server_methods.txt`
- `evidence/ui_audit/decomp/peer_methods.txt`
- `evidence/ui_audit/decomp/transfer_methods.txt`
- `evidence/reverse/ui_handler_symbols_nm.txt`

4. Knowledge-base synchronization updates:
- `docs/state/soulseek-feature-inventory.md`
- `docs/state/decompilation-status.md`
- `docs/re/static/detangling.md`
- `docs/verification/evidence-ledger.md`
- `docs/state/roadmap.md`
- `docs/state/project-status.md`
- `docs/state/verification-status.md`
- `docs/state/protocol-backlog.md`
- `docs/index.md`
- `TODO-CODEX.md`

## Pass-2 Closure

- Pass-1 mapped feature entries: `42`
- Pass-2 revisited entries: `42`
- `verified_pass2`: `41`
- `gap_found`: `1`

Gap:

- macOS assistive-access denial while attempting live menu extraction (`osascript` error `-1719`), documented in `evidence/ui_audit/ui_menu_bar_items.err`.

## Validation Commands

```bash
python3 scripts/kb_validate.py
/Users/void_rsk/Projects/soul-dec/.venv-tools/bin/zensical build -f zensical.toml
```

Results:

- KB validation passed.
- Zensical build passed.

## Review Loops

Round 1 (security best-practices pass):

- Reviewed touched paths (`docs/*`, `evidence/*`, `TODO-CODEX.md`).
- No secret handling regression detected.
- No executable/runtime parsing path altered.

Round 1 (code simplifier pass):

- Not applicable to Rust runtime code in this stage (docs/evidence-only changes).

Round 2:

- Re-ran consistency review on changed artifacts.
- No additional corrective actions required.

## Risk Notes

- Forum extraction relies on the dynamic page payload captured at run time; thread snapshots are informational, not canonical protocol truth.
- Authenticated runtime UI verification remains bounded by local login/session conditions.
