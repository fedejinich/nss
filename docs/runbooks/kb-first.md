# KB-first Runbook

## Purpose

Establish a mandatory knowledge-base gate before further reverse engineering and implementation.

## Workflow

1. Add candidates to queue files:
   - `analysis/ghidra/queue/name_candidates.jsonl`
   - `analysis/ghidra/queue/data_candidates.jsonl`
2. Run promotion workflow:

```bash
python3 scripts/kb_promote.py
```

3. Regenerate docs from authoritative maps:

```bash
python3 scripts/kb_sync_docs.py
```

4. Validate all map entries and evidence links:

```bash
python3 scripts/kb_validate.py
```

## Promotion Rules

- `high` confidence + valid evidence: auto-promote to authoritative map (`status=approved`).
- `medium` or `low` confidence: append to `analysis/ghidra/queue/review_queue.jsonl`.
- Missing fields/evidence or broken local evidence links: reject and append to review queue.

## Required Evidence per Entry

Every rename or data label entry must include at least one evidence item with:

- `kind`: evidence type (`string`, `call_pattern`, `xref`, `ghidra_decompile`, `frida_trace`, `pcap`, `runtime_log`, `manual_note`)
- `source`: local path or URL

If the source is a local path, it must exist.

## Review Queue Policy

Review queue rows are append-only records with:

- `status`: `review_required` or `rejected`
- `reason`: why it was not promoted
- full `candidate` payload for follow-up

## Notes

Binary Ninja remains optional and non-blocking.
