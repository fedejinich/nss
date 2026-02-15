from __future__ import annotations

import json
import subprocess
import unittest
from pathlib import Path


class DashboardGeneratorTests(unittest.TestCase):
    def test_dashboard_data_generation(self) -> None:
        repo_root = Path(__file__).resolve().parents[2]
        matrix_script = repo_root / "tools/protocol/generate_protocol_matrix.py"
        runtime_script = repo_root / "tools/state/generate_runtime_coverage.py"
        capability_script = repo_root / "tools/state/generate_capability_matrix.py"
        dashboard_script = repo_root / "tools/state/generate_dashboard_data.py"
        out = repo_root / "docs/state/project-dashboard-data.json"

        subprocess.run(["python3", str(matrix_script)], check=True, cwd=repo_root)
        subprocess.run(["python3", str(runtime_script)], check=True, cwd=repo_root)
        subprocess.run(["python3", str(capability_script)], check=True, cwd=repo_root)
        subprocess.run(["python3", str(dashboard_script)], check=True, cwd=repo_root)

        payload = json.loads(out.read_text(encoding="utf-8"))
        self.assertIn("stage_summary", payload)
        self.assertIn("protocol_summary", payload)
        self.assertIn("runtime_summary", payload)
        self.assertIn("capability_summary", payload)
        self.assertIn("stages", payload)
        self.assertIn("dependencies", payload)
        self.assertGreater(payload["stage_summary"]["total"], 0)

    def test_codebase_graph_generation_excludes_heavy_dirs(self) -> None:
        repo_root = Path(__file__).resolve().parents[2]
        graph_script = repo_root / "tools/state/generate_codebase_graph.py"
        out = repo_root / "docs/state/codebase-graph.json"

        subprocess.run(["python3", str(graph_script)], check=True, cwd=repo_root)

        payload = json.loads(out.read_text(encoding="utf-8"))
        self.assertIn("nodes", payload)
        self.assertIn("edges", payload)
        self.assertGreater(payload["stats"]["file_count"], 0)

        node_paths = [str(node.get("path", "")) for node in payload.get("nodes", [])]
        self.assertTrue(all("captures/raw" not in path for path in node_paths))
        self.assertTrue(all("captures/redacted" not in path for path in node_paths))
        self.assertTrue(all(".venv-tools" not in path for path in node_paths))

    def test_project_dashboard_uses_absolute_evidence_routes(self) -> None:
        repo_root = Path(__file__).resolve().parents[2]
        dashboard_html = repo_root / "docs/state/project-dashboard.html"
        source = dashboard_html.read_text(encoding="utf-8")

        self.assertIn("return `/${clean}`;", source)
        self.assertNotIn("return `../${clean}`;", source)


if __name__ == "__main__":
    unittest.main()
