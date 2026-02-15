from __future__ import annotations

import json
import subprocess
import unittest
from pathlib import Path


class ReleaseHardeningAuditTests(unittest.TestCase):
    def test_release_hardening_audit_report_is_consistent_with_stage_state(self) -> None:
        repo_root = Path(__file__).resolve().parents[2]
        script = repo_root / "tools/state/verify_release_hardening.py"
        out = repo_root / "docs/state/release-hardening-audit.json"

        subprocess.run(["python3", str(script)], check=True, cwd=repo_root)
        report = json.loads(out.read_text(encoding="utf-8"))

        self.assertTrue(report["overall_ok"])
        self.assertTrue(report["checks"]["redacted_metadata_paths"]["ok"])
        self.assertTrue(report["checks"]["required_files"]["ok"])

        stage_status = report.get("stage_state", {}).get("status")
        if stage_status == "done":
            self.assertTrue(report["strict_closure_ok"])
            self.assertTrue(report["checks"]["final_closure_checklist"]["ok"])
            self.assertTrue(report["checks"]["capability_registry_state"]["ok"])
        elif stage_status == "in_progress":
            self.assertTrue(report["operational_ok"])
        else:
            self.fail(f"unexpected stage status in release hardening report: {stage_status}")


if __name__ == "__main__":
    unittest.main()
