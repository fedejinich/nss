from __future__ import annotations

import json
import subprocess
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
REDACTOR = REPO_ROOT / "tools/runtime/redact_capture_run.py"


class RuntimeRedactionTests(unittest.TestCase):
    def test_redaction_pipeline_generates_expected_artifacts(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            root = Path(tmpdir)
            raw_run = root / "raw" / "run-login"
            raw_run.mkdir(parents=True)

            manifest = {
                "scenario": "login-search",
                "username": "alice",
                "password": "my-secret-password",
                "peer_addr": "10.0.0.4:2234",
                "download_path": "/Users/alice/Music/track.flac",
                "private_message": "hello this is a private message",
            }
            (raw_run / "manifest.raw.json").write_text(json.dumps(manifest), encoding="utf-8")
            (raw_run / "frida-events.raw.jsonl").write_text(
                json.dumps({"event": "msg", "from_user": "alice", "to_user": "bob"}) + "\n",
                encoding="utf-8",
            )
            (raw_run / "io-events.raw.jsonl").write_text(
                json.dumps(
                    {
                        "event": "qsettings_setvalue",
                        "file_path": "/Users/alice/Library/Application Support/SoulseekQt/config.ini",
                        "key": "minimize_on_close",
                        "value_sample": "true",
                        "username": "alice",
                    }
                )
                + "\n",
                encoding="utf-8",
            )
            (raw_run / "official_frames.raw.hex").write_text("0a0b0c\n# comment\n0D0E0F\n", encoding="utf-8")
            (raw_run / "neo_frames.raw.hex").write_text("0a0b0c\n0d0e0f\n", encoding="utf-8")

            out_root = root / "redacted"
            cmd = [
                "python3",
                str(REDACTOR),
                "--run-dir",
                str(raw_run),
                "--out-root",
                str(out_root),
                "--run-id",
                "login-search",
                "--salt",
                "test-salt",
            ]
            subprocess.run(cmd, check=True, cwd=REPO_ROOT)

            run_dir = out_root / "login-search"
            self.assertTrue((run_dir / "manifest.redacted.json").exists())
            self.assertTrue((run_dir / "frida-events.redacted.jsonl").exists())
            self.assertTrue((run_dir / "io-events.redacted.jsonl").exists())
            self.assertTrue((run_dir / "official_frames.hex").exists())
            self.assertTrue((run_dir / "neo_frames.hex").exists())
            self.assertTrue((run_dir / "redaction-summary.json").exists())

            redacted_manifest = json.loads((run_dir / "manifest.redacted.json").read_text(encoding="utf-8"))
            raw_text = json.dumps(redacted_manifest)
            self.assertNotIn("alice", raw_text)
            self.assertNotIn("my-secret-password", raw_text)
            self.assertNotIn("10.0.0.4", raw_text)
            self.assertNotIn("/Users/alice/Music/track.flac", raw_text)
            self.assertNotIn("private message", raw_text)
            self.assertNotIn(str(raw_run), raw_text)
            self.assertEqual(redacted_manifest["redaction"]["policy_version"], "2")

            summary = json.loads((run_dir / "redaction-summary.json").read_text(encoding="utf-8"))
            summary_text = json.dumps(summary)
            self.assertNotIn(str(raw_run), summary_text)
            self.assertTrue(summary["raw_dir"].startswith("<external:path:"))
            self.assertTrue(summary["redacted_dir"].startswith("<external:path:"))
            self.assertIn("io_events", summary["artifacts"])

            official = (run_dir / "official_frames.hex").read_text(encoding="utf-8").splitlines()
            neo = (run_dir / "neo_frames.hex").read_text(encoding="utf-8").splitlines()
            self.assertEqual(official, ["0a0b0c", "0d0e0f"])
            self.assertEqual(neo, ["0a0b0c", "0d0e0f"])

            io_rows = (run_dir / "io-events.redacted.jsonl").read_text(encoding="utf-8")
            self.assertNotIn("/Users/alice/Library/Application Support/SoulseekQt/config.ini", io_rows)
            self.assertNotIn("\"alice\"", io_rows)


if __name__ == "__main__":
    unittest.main()
