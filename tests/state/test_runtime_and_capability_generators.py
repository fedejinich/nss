from __future__ import annotations

import json
import subprocess
import unittest
from pathlib import Path


class RuntimeAndCapabilityGeneratorTests(unittest.TestCase):
    def test_runtime_coverage_generation(self) -> None:
        repo_root = Path(__file__).resolve().parents[2]
        script = repo_root / "tools/state/generate_runtime_coverage.py"
        out_json = repo_root / "docs/state/runtime-coverage.json"
        out_md = repo_root / "docs/state/runtime-coverage.md"

        subprocess.run(["python3", str(script)], check=True, cwd=repo_root)

        payload = json.loads(out_json.read_text(encoding="utf-8"))
        rendered = out_md.read_text(encoding="utf-8")

        self.assertIn("summary", payload)
        self.assertIn("gaps", payload)
        self.assertIn("targets", payload)
        self.assertIn("Runtime Coverage", rendered)

        summary = payload["summary"]
        self.assertGreater(summary["total_messages"], 0)
        self.assertGreaterEqual(summary["verified_runtime"], 0)
        self.assertGreaterEqual(summary["verified_static"], 0)

    def test_capability_matrix_generation(self) -> None:
        repo_root = Path(__file__).resolve().parents[2]
        runtime_script = repo_root / "tools/state/generate_runtime_coverage.py"
        matrix_script = repo_root / "tools/protocol/generate_protocol_matrix.py"
        capability_script = repo_root / "tools/state/generate_capability_matrix.py"
        out_json = repo_root / "docs/state/capability-matrix.json"
        out_md = repo_root / "docs/state/capability-matrix.md"

        subprocess.run(["python3", str(matrix_script)], check=True, cwd=repo_root)
        subprocess.run(["python3", str(runtime_script)], check=True, cwd=repo_root)
        subprocess.run(["python3", str(capability_script)], check=True, cwd=repo_root)

        payload = json.loads(out_json.read_text(encoding="utf-8"))
        rendered = out_md.read_text(encoding="utf-8")

        self.assertIn("summary", payload)
        self.assertIn("final_gates", payload)
        self.assertIn("critical_path", payload)
        self.assertIn("capabilities", payload)
        self.assertIn("Capability Matrix", rendered)
        self.assertGreater(payload["summary"]["total_capabilities"], 0)


if __name__ == "__main__":
    unittest.main()
