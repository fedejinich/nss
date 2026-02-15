from __future__ import annotations

import json
import subprocess
import unittest
from pathlib import Path


class ProtocolMatrixTests(unittest.TestCase):
    def test_matrix_generation_produces_expected_markers(self) -> None:
        repo_root = Path(__file__).resolve().parents[2]
        script = repo_root / "tools/protocol/generate_protocol_matrix.py"
        out_md = repo_root / "docs/state/protocol-matrix.md"
        out_json = repo_root / "docs/state/protocol-matrix.json"

        subprocess.run(["python3", str(script)], check=True, cwd=repo_root)
        rendered = out_md.read_text(encoding="utf-8")
        rendered_json = json.loads(out_json.read_text(encoding="utf-8"))

        self.assertIn("# Protocol Message Matrix", rendered)
        self.assertIn("`SM_LOGIN`", rendered)
        self.assertIn("`PM_USER_INFO_REQUEST`", rendered)
        self.assertIn("`missing`", rendered)
        self.assertIn("snapshot", rendered_json)
        self.assertIn("rows", rendered_json)
        self.assertGreater(rendered_json["snapshot"]["total_messages"], 0)


if __name__ == "__main__":
    unittest.main()
