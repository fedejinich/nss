from __future__ import annotations

import csv
import json
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]

S6E_TYPED_MESSAGES = {
    ("server", "SM_DNET_LEVEL"),
    ("server", "SM_DNET_GROUP_LEADER"),
    ("server", "SM_DNET_CHILD_DEPTH"),
    ("server", "SM_REMOVE_ROOM_OPERATORSHIP"),
    ("server", "SM_REMOVE_OWN_ROOM_OPERATORSHIP"),
}

S6E_RESIDUAL_MESSAGES = {
    ("server", "SM_DNET_DELIVERY_REPORT"),
    ("server", "SM_FLOOD"),
}


class Stage6ELegacyOpaqueReductionContractTests(unittest.TestCase):
    def test_message_map_contains_s6e_target_set(self) -> None:
        message_map_path = REPO_ROOT / "analysis/ghidra/maps/message_map.csv"
        self.assertTrue(message_map_path.exists())

        with message_map_path.open("r", encoding="utf-8") as fh:
            rows = list(csv.DictReader(fh))

        index = {(row["scope"].strip(), row["name"].strip()): row for row in rows}
        expected = S6E_TYPED_MESSAGES | S6E_RESIDUAL_MESSAGES
        self.assertEqual(set(index.keys()) & expected, expected)

        for key in sorted(expected):
            row = index[key]
            source = row["source"].strip()
            self.assertTrue(source, f"missing source for {row['name']}")
            source_path = REPO_ROOT / source
            self.assertTrue(source_path.exists(), f"missing source file: {source}")
            self.assertEqual(
                row["status"].strip(),
                "verified_runtime",
                f"expected verified_runtime status for {row['name']}",
            )

    def test_message_schema_typed_and_residual_expectations(self) -> None:
        schema_path = REPO_ROOT / "analysis/protocol/message_schema.json"
        self.assertTrue(schema_path.exists())

        payload = json.loads(schema_path.read_text(encoding="utf-8"))
        messages = payload.get("messages", [])
        self.assertIsInstance(messages, list)
        index = {(entry["scope"], entry["name"]): entry for entry in messages}

        for key in sorted(S6E_TYPED_MESSAGES):
            self.assertIn(key, index, f"missing schema entry for {key}")
            entry = index[key]
            schema_payload = entry.get("payload", [])
            self.assertIsInstance(schema_payload, list)
            self.assertGreaterEqual(
                len(schema_payload),
                1,
                f"expected typed payload schema for {entry['name']}",
            )
            evidence = entry.get("evidence", [])
            self.assertTrue(evidence, f"missing evidence for {entry['name']}")
            runtime_links = [
                ev.get("source", "")
                for ev in evidence
                if "captures/redacted/login-legacy-" in ev.get("source", "")
            ]
            self.assertTrue(runtime_links, f"missing runtime evidence for {entry['name']}")

        for key in sorted(S6E_RESIDUAL_MESSAGES):
            self.assertIn(key, index, f"missing schema entry for {key}")
            entry = index[key]
            evidence = entry.get("evidence", [])
            self.assertTrue(evidence, f"missing evidence for residual {entry['name']}")
            runtime_links = [
                ev.get("source", "")
                for ev in evidence
                if "captures/redacted/login-legacy-distributed-control" in ev.get("source", "")
            ]
            self.assertTrue(runtime_links, f"missing runtime evidence for {entry['name']}")


if __name__ == "__main__":
    unittest.main()
