from __future__ import annotations

import json
import subprocess
import unittest
from pathlib import Path


class S5AClosureAuditTests(unittest.TestCase):
    def test_s5a_closure_audit_report_is_green(self) -> None:
        repo_root = Path(__file__).resolve().parents[2]
        script = repo_root / "tools/state/verify_s5a_closure.py"
        out = repo_root / "docs/state/s5a-closure-audit.json"

        subprocess.run(["python3", str(script)], check=True, cwd=repo_root)
        report = json.loads(out.read_text(encoding="utf-8"))

        self.assertTrue(report["overall_ok"])
        self.assertTrue(report["objectives"]["opaque_to_typed_runtime_evidence"]["ok"])
        self.assertTrue(report["objectives"]["global_distributed_runtime_captures"]["ok"])
        self.assertTrue(report["objectives"]["pm_shared_files_decompression_parser"]["ok"])
        self.assertTrue(report["objectives"]["residual_hypotheses_closed"]["ok"])


if __name__ == "__main__":
    unittest.main()
