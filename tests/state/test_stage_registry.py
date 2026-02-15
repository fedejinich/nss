from __future__ import annotations

import json
import unittest
from pathlib import Path


class StageRegistryTests(unittest.TestCase):
    def test_stage_registry_has_required_fields_and_valid_dependencies(self) -> None:
        repo_root = Path(__file__).resolve().parents[2]
        path = repo_root / "analysis/state/stage_registry.json"
        payload = json.loads(path.read_text(encoding="utf-8"))

        stages = payload.get("stages", [])
        self.assertIsInstance(stages, list)
        self.assertGreater(len(stages), 0)

        required = {
            "id",
            "title",
            "status",
            "owner_area",
            "depends_on",
            "evidence",
            "next_gate",
            "notes",
        }

        stage_ids = set()
        for stage in stages:
            self.assertTrue(required.issubset(stage.keys()))
            self.assertIn(stage["status"], {"done", "in_progress", "planned"})
            self.assertIsInstance(stage["depends_on"], list)
            self.assertNotIn(stage["id"], stage_ids)
            stage_ids.add(stage["id"])

        for stage in stages:
            for dep in stage["depends_on"]:
                self.assertIn(dep, stage_ids)


if __name__ == "__main__":
    unittest.main()
