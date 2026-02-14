from __future__ import annotations

import csv
import json
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]

S4C_MESSAGES = {
    ("server", "SM_IGNORE_USER"),
    ("server", "SM_UNIGNORE_USER"),
    ("server", "SM_GET_OWN_PRIVILEGES_STATUS"),
    ("server", "SM_GET_USER_PRIVILEGES_STATUS"),
    ("server", "SM_GIVE_PRIVILEGE"),
    ("server", "SM_INFORM_USER_OF_PRIVILEGES"),
    ("server", "SM_INFORM_USER_OF_PRIVILEGES_ACK"),
    ("peer", "PM_GET_SHARED_FILES_IN_FOLDER"),
    ("peer", "PM_SHARED_FILES_IN_FOLDER"),
}


class Stage4CPrivilegesSocialContractTests(unittest.TestCase):
    def test_message_map_contains_stage4c_pack(self) -> None:
        message_map_path = REPO_ROOT / "analysis/ghidra/maps/message_map.csv"
        self.assertTrue(message_map_path.exists())

        with message_map_path.open("r", encoding="utf-8") as fh:
            rows = list(csv.DictReader(fh))

        index = {(row["scope"].strip(), row["name"].strip()): row for row in rows}
        self.assertEqual(set(index.keys()) & S4C_MESSAGES, S4C_MESSAGES)

        batch = [index[key] for key in sorted(S4C_MESSAGES)]
        high = sum(1 for row in batch if row["confidence"].strip() == "high")
        medium = sum(1 for row in batch if row["confidence"].strip() == "medium")
        low = sum(1 for row in batch if row["confidence"].strip() == "low")

        self.assertGreaterEqual(high, 7)
        self.assertLessEqual(medium, 2)
        self.assertEqual(low, 0)

        for row in batch:
            source = row["source"].strip()
            self.assertTrue(source, f"missing source for {row['name']}")
            source_path = REPO_ROOT / source
            self.assertTrue(source_path.exists(), f"missing source file: {source}")

    def test_message_schema_contains_stage4c_pack(self) -> None:
        schema_path = REPO_ROOT / "analysis/protocol/message_schema.json"
        self.assertTrue(schema_path.exists())

        payload = json.loads(schema_path.read_text(encoding="utf-8"))
        messages = payload.get("messages", [])
        self.assertIsInstance(messages, list)

        index = {(entry["scope"], entry["name"]): entry for entry in messages}
        self.assertEqual(set(index.keys()) & S4C_MESSAGES, S4C_MESSAGES)

        batch = [index[key] for key in sorted(S4C_MESSAGES)]
        high = sum(1 for entry in batch if entry["confidence"] == "high")
        medium = sum(1 for entry in batch if entry["confidence"] == "medium")
        low = sum(1 for entry in batch if entry["confidence"] == "low")

        self.assertGreaterEqual(high, 7)
        self.assertLessEqual(medium, 2)
        self.assertEqual(low, 0)

        for entry in batch:
            evidence = entry.get("evidence", [])
            self.assertTrue(evidence, f"missing evidence for {entry['name']}")
            for ev in evidence:
                source = ev.get("source", "").strip()
                self.assertTrue(source, f"evidence without source for {entry['name']}")
                if source.startswith(("http://", "https://")):
                    continue
                source_path = REPO_ROOT / source
                self.assertTrue(source_path.exists(), f"missing evidence source: {source}")


if __name__ == "__main__":
    unittest.main()
