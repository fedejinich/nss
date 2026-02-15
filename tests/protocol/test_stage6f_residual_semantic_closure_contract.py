from __future__ import annotations

import csv
import json
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]

S6F_RESIDUAL_CLOSED = {
    ("server", "SM_DNET_DELIVERY_REPORT"): {"field": "report"},
    ("server", "SM_FLOOD"): {"field": "flood_code"},
}


class Stage6FResidualSemanticClosureContractTests(unittest.TestCase):
    def test_message_map_marks_residual_codes_as_runtime_verified(self) -> None:
        message_map_path = REPO_ROOT / "analysis/ghidra/maps/message_map.csv"
        self.assertTrue(message_map_path.exists())

        with message_map_path.open("r", encoding="utf-8") as fh:
            rows = list(csv.DictReader(fh))

        index = {(row["scope"].strip(), row["name"].strip()): row for row in rows}
        self.assertEqual(set(index.keys()) & set(S6F_RESIDUAL_CLOSED.keys()), set(S6F_RESIDUAL_CLOSED.keys()))

        for key in sorted(S6F_RESIDUAL_CLOSED):
            row = index[key]
            self.assertEqual(row["status"].strip(), "verified_runtime")
            source = row["source"].strip()
            self.assertTrue(source)
            source_path = REPO_ROOT / source
            self.assertTrue(source_path.exists(), f"missing source file: {source}")
            notes = row["notes"].strip().lower()
            self.assertNotIn("unresolved", notes)

    def test_message_schema_contains_typed_payload_fields_for_residual_codes(self) -> None:
        schema_path = REPO_ROOT / "analysis/protocol/message_schema.json"
        self.assertTrue(schema_path.exists())

        payload = json.loads(schema_path.read_text(encoding="utf-8"))
        index = {(entry["scope"], entry["name"]): entry for entry in payload.get("messages", [])}

        for key, expectations in sorted(S6F_RESIDUAL_CLOSED.items()):
            self.assertIn(key, index)
            entry = index[key]
            fields = [field.get("name") for field in entry.get("payload", [])]
            self.assertIn(expectations["field"], fields)
            self.assertIn("raw_tail", fields)

            evidence = entry.get("evidence", [])
            self.assertTrue(evidence)
            runtime_links = [
                ev.get("source", "")
                for ev in evidence
                if "captures/redacted/login-legacy-residual-control" in ev.get("source", "")
            ]
            self.assertTrue(runtime_links, f"missing S6F runtime evidence for {entry['name']}")


if __name__ == "__main__":
    unittest.main()
