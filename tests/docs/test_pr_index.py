from __future__ import annotations

import subprocess
import unittest
from pathlib import Path


class PrIndexTests(unittest.TestCase):
    def test_pr_index_is_collapsed_and_lists_known_prs(self) -> None:
        repo_root = Path(__file__).resolve().parents[2]
        script = repo_root / "tools/docs/generate_pr_index.py"
        out = repo_root / "docs/pr/index.md"

        subprocess.run(["python3", str(script)], check=True, cwd=repo_root)
        rendered = out.read_text(encoding="utf-8")

        self.assertIn("<details>", rendered)
        self.assertIn("</details>", rendered)
        self.assertIn("0003-s3a-auth-semantic-parity.md", rendered)
        self.assertIn("0019-s5d-s5h-control-typing-pack.md", rendered)


if __name__ == "__main__":
    unittest.main()
