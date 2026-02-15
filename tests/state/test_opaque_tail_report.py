from __future__ import annotations

import json
import subprocess
import unittest
from pathlib import Path


class OpaqueTailReportTests(unittest.TestCase):
    def test_opaque_tail_report_generation(self) -> None:
        repo_root = Path(__file__).resolve().parents[2]
        script = repo_root / "tools/state/report_opaque_tail.py"
        out = repo_root / "docs/state/opaque-tail-report.json"

        subprocess.run(["python3", str(script)], check=True, cwd=repo_root)
        report = json.loads(out.read_text(encoding="utf-8"))

        self.assertIn("opaque_tail_count", report)
        self.assertIn("entries", report)
        self.assertEqual(report["opaque_tail_count"], len(report["entries"]))
        self.assertGreater(report["opaque_tail_count"], 0)

        codes = {entry["code"] for entry in report["entries"]}
        self.assertIn(67, codes)
        self.assertIn(142, codes)


if __name__ == "__main__":
    unittest.main()
