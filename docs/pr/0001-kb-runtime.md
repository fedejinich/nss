# PR 0001 - KB-first reverse flow + runtime capture

## Branch

- `codex/kb-first-zensical`

## Commits

- `6c9825b` feat: add reverse flow extraction and runtime capture pipeline

## Scope

- Static extraction pipeline for search/download flow.
- Evidence bundle for key server/peer/transfer functions.
- Frida hooks and synchronized capture harness (Frida + PCAP + manifest).
- Message schema derivation from authoritative maps/evidence.
- KB workflow hardening: candidate queue consumption + regression test.

## Key Files

- `scripts/extract_search_download_flow.sh`
- `analysis/re/flow_graph.json`
- `frida/hooks/soulseek_trace.js`
- `tools/runtime/capture_harness.py`
- `scripts/capture_session.sh`
- `scripts/capture_golden.sh`
- `tools/protocol/derive_schema.py`
- `analysis/protocol/message_schema.json`

## Validation

- `python3 scripts/kb_promote.py`
- `python3 scripts/kb_sync_docs.py`
- `python3 scripts/kb_validate.py`
- `python3 -m unittest discover -s tests -p 'test_kb_workflow.py' -v`
- `./.venv-tools/bin/zensical build -f zensical.toml`
