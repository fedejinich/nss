from __future__ import annotations

import subprocess
import unittest
from pathlib import Path


class ProtocolMatrixTests(unittest.TestCase):
    def test_matrix_generation_produces_expected_markers(self) -> None:
        repo_root = Path(__file__).resolve().parents[2]
        script = repo_root / "tools/protocol/generate_protocol_matrix.py"
        out = repo_root / "docs/state/protocol-matrix.md"

        subprocess.run(["python3", str(script)], check=True, cwd=repo_root)
        rendered = out.read_text(encoding="utf-8")

        self.assertIn("# Protocol Message Matrix", rendered)
        self.assertIn("`SM_LOGIN`", rendered)
        self.assertIn("`PM_USER_INFO_REQUEST`", rendered)
        self.assertIn("`missing`", rendered)


if __name__ == "__main__":
    unittest.main()
