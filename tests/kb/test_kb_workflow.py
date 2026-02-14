from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from tools.kb.docs_sync import sync_docs
from tools.kb.validate import validate_maps
from tools.kb.workflow import promote_candidates


class KBWorkflowTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        (self.root / "analysis/ghidra/maps").mkdir(parents=True)
        (self.root / "analysis/ghidra/queue").mkdir(parents=True)
        (self.root / "docs/re/static").mkdir(parents=True)
        (self.root / "docs/verification").mkdir(parents=True)
        (self.root / "evidence").mkdir(parents=True)

        (self.root / "analysis/ghidra/maps/name_map.json").write_text(
            json.dumps({"version": 1, "generated_by": "test", "entries": []}), encoding="utf-8"
        )
        (self.root / "analysis/ghidra/maps/data_map.json").write_text(
            json.dumps({"version": 1, "generated_by": "test", "entries": []}), encoding="utf-8"
        )
        (self.root / "analysis/ghidra/maps/message_map.csv").write_text(
            "scope,code,name,confidence,source,status,notes\n",
            encoding="utf-8",
        )

        (self.root / "analysis/ghidra/queue/name_candidates.jsonl").write_text("", encoding="utf-8")
        (self.root / "analysis/ghidra/queue/data_candidates.jsonl").write_text("", encoding="utf-8")
        (self.root / "analysis/ghidra/queue/review_queue.jsonl").write_text("", encoding="utf-8")

    def tearDown(self) -> None:
        self.tmp.cleanup()

    def _append_name_candidate(self, payload: dict) -> None:
        target = self.root / "analysis/ghidra/queue/name_candidates.jsonl"
        with target.open("a", encoding="utf-8") as fh:
            fh.write(json.dumps(payload) + "\n")

    def _promote(self) -> None:
        promote_candidates(
            repo_root=self.root,
            name_map_path=self.root / "analysis/ghidra/maps/name_map.json",
            data_map_path=self.root / "analysis/ghidra/maps/data_map.json",
            name_candidates_path=self.root / "analysis/ghidra/queue/name_candidates.jsonl",
            data_candidates_path=self.root / "analysis/ghidra/queue/data_candidates.jsonl",
            review_queue_path=self.root / "analysis/ghidra/queue/review_queue.jsonl",
        )

    def _sync_docs(self) -> None:
        sync_docs(
            name_map_path=self.root / "analysis/ghidra/maps/name_map.json",
            data_map_path=self.root / "analysis/ghidra/maps/data_map.json",
            review_queue_path=self.root / "analysis/ghidra/queue/review_queue.jsonl",
            message_map_path=self.root / "analysis/ghidra/maps/message_map.csv",
            detangling_doc_path=self.root / "docs/re/static/detangling.md",
            ledger_doc_path=self.root / "docs/verification/evidence-ledger.md",
        )

    def test_high_confidence_candidate_promotes_and_is_published(self) -> None:
        (self.root / "evidence/static.txt").write_text("xref", encoding="utf-8")
        self._append_name_candidate(
            {
                "binary": "SoulseekQt",
                "address": "0x10006c590",
                "original_name": "FUN_10006c590",
                "new_name": "Server_MessageCodeToString",
                "kind": "function_rename",
                "confidence": "high",
                "evidence": [{"kind": "xref", "source": "evidence/static.txt"}],
            }
        )
        self._promote()
        self._sync_docs()

        name_map = json.loads((self.root / "analysis/ghidra/maps/name_map.json").read_text(encoding="utf-8"))
        self.assertEqual(len(name_map["entries"]), 1)
        self.assertEqual(name_map["entries"][0]["status"], "approved")

        detangling = (self.root / "docs/re/static/detangling.md").read_text(encoding="utf-8")
        self.assertIn("Server_MessageCodeToString", detangling)

    def test_medium_confidence_stays_in_review_queue(self) -> None:
        (self.root / "evidence/static.txt").write_text("xref", encoding="utf-8")
        self._append_name_candidate(
            {
                "binary": "SoulseekQt",
                "address": "0x10006c686",
                "original_name": "FUN_10006c686",
                "new_name": "Server_FileSearch",
                "kind": "function_rename",
                "confidence": "medium",
                "evidence": [{"kind": "string", "source": "evidence/static.txt"}],
            }
        )
        self._promote()

        name_map = json.loads((self.root / "analysis/ghidra/maps/name_map.json").read_text(encoding="utf-8"))
        self.assertEqual(len(name_map["entries"]), 0)

        queue_lines = (self.root / "analysis/ghidra/queue/review_queue.jsonl").read_text(encoding="utf-8").splitlines()
        self.assertEqual(len(queue_lines), 1)
        self.assertIn("review_required", queue_lines[0])

    def test_missing_evidence_is_rejected(self) -> None:
        self._append_name_candidate(
            {
                "binary": "SoulseekQt",
                "address": "0x10006c6aa",
                "original_name": "FUN_10006c6aa",
                "new_name": "Server_SendConnectToken",
                "kind": "function_rename",
                "confidence": "high",
                "evidence": [],
            }
        )
        self._promote()

        queue_lines = (self.root / "analysis/ghidra/queue/review_queue.jsonl").read_text(encoding="utf-8")
        self.assertIn("rejected", queue_lines)
        self.assertIn("missing_evidence", queue_lines)

    def test_broken_evidence_link_is_invalid(self) -> None:
        self._append_name_candidate(
            {
                "binary": "SoulseekQt",
                "address": "0x10006c6c2",
                "original_name": "FUN_10006c6c2",
                "new_name": "Server_SharedFoldersFiles",
                "kind": "function_rename",
                "confidence": "high",
                "evidence": [{"kind": "xref", "source": "evidence/missing.txt"}],
            }
        )
        self._promote()

        errors = validate_maps(
            repo_root=self.root,
            name_map_path=self.root / "analysis/ghidra/maps/name_map.json",
            data_map_path=self.root / "analysis/ghidra/maps/data_map.json",
        )
        self.assertEqual(errors["name_map"], [])
        queue_lines = (self.root / "analysis/ghidra/queue/review_queue.jsonl").read_text(encoding="utf-8")
        self.assertIn("source_not_found", queue_lines)

    def test_docs_regeneration_is_stable(self) -> None:
        (self.root / "evidence/static.txt").write_text("xref", encoding="utf-8")
        self._append_name_candidate(
            {
                "binary": "SoulseekQt",
                "address": "0x10006c632",
                "original_name": "FUN_10006c632",
                "new_name": "Server_JoinRoom",
                "kind": "function_rename",
                "confidence": "high",
                "evidence": [{"kind": "xref", "source": "evidence/static.txt"}],
            }
        )
        self._promote()
        self._sync_docs()
        first = (self.root / "docs/re/static/detangling.md").read_text(encoding="utf-8")

        self._sync_docs()
        second = (self.root / "docs/re/static/detangling.md").read_text(encoding="utf-8")

        self.assertEqual(first, second)

    def test_candidate_queue_is_consumed_after_promotion(self) -> None:
        (self.root / "evidence/static.txt").write_text("xref", encoding="utf-8")
        self._append_name_candidate(
            {
                "binary": "SoulseekQt",
                "address": "0x10006c700",
                "original_name": "FUN_10006c700",
                "new_name": "Server_TestCandidate",
                "kind": "function_rename",
                "confidence": "high",
                "evidence": [{"kind": "xref", "source": "evidence/static.txt"}],
            }
        )

        first = promote_candidates(
            repo_root=self.root,
            name_map_path=self.root / "analysis/ghidra/maps/name_map.json",
            data_map_path=self.root / "analysis/ghidra/maps/data_map.json",
            name_candidates_path=self.root / "analysis/ghidra/queue/name_candidates.jsonl",
            data_candidates_path=self.root / "analysis/ghidra/queue/data_candidates.jsonl",
            review_queue_path=self.root / "analysis/ghidra/queue/review_queue.jsonl",
        )
        self.assertEqual(first["name"].promoted, 1)
        self.assertEqual((self.root / "analysis/ghidra/queue/name_candidates.jsonl").read_text(encoding="utf-8"), "")

        second = promote_candidates(
            repo_root=self.root,
            name_map_path=self.root / "analysis/ghidra/maps/name_map.json",
            data_map_path=self.root / "analysis/ghidra/maps/data_map.json",
            name_candidates_path=self.root / "analysis/ghidra/queue/name_candidates.jsonl",
            data_candidates_path=self.root / "analysis/ghidra/queue/data_candidates.jsonl",
            review_queue_path=self.root / "analysis/ghidra/queue/review_queue.jsonl",
        )
        self.assertEqual(second["name"].promoted, 0)


if __name__ == "__main__":
    unittest.main()
