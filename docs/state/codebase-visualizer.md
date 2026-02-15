# Codebase Visualizer

Interactive treemap for NeoSoulSeek source and documentation topology.

- Open visualizer: [codebase-visualizer.html](codebase-visualizer.html)
- Data source: [codebase-graph.json](codebase-graph.json)

## Interaction

- Click a tile: select and inspect path details.
- Double click a folder/domain: zoom to the subtree.
- `Reset focus`: return to repository root view.
- Filters: show or hide folders/files.
- `min tile`: adjust minimum tile area to reduce visual noise.

## Refresh data

```bash
python3 tools/state/generate_codebase_graph.py
```

Or use the combined dashboard sync script:

```bash
scripts/sync_state_dashboards.sh
```
