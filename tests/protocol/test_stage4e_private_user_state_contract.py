from __future__ import annotations

import csv
import json
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]

S4E_MESSAGES = {
    ("server", "SM_MESSAGE_USER"),
    ("server", "SM_MESSAGE_ACKED"),
    ("server", "SM_GET_USER_STATUS"),
    ("server", "SM_GET_USER_STATS"),
    ("server", "SM_GET_PEER_ADDRESS"),
    ("server", "SM_CONNECT_TO_PEER"),
    ("server", "SM_MESSAGE_USERS"),
    ("server", "SM_PEER_MESSAGE"),
}


class Stage4EPrivateUserStateContractTests(unittest.TestCase):
    def test_message_map_contains_stage4e_pack(self) -> None:
        message_map_path = REPO_ROOT / "analysis/ghidra/maps/message_map.csv"
        self.assertTrue(message_map_path.exists())

        with message_map_path.open("r", encoding="utf-8") as fh:
            rows = list(csv.DictReader(fh))

        index = {(row["scope"].strip(), row["name"].strip()): row for row in rows}
        self.assertEqual(set(index.keys()) & S4E_MESSAGES, S4E_MESSAGES)

        batch = [index[key] for key in sorted(S4E_MESSAGES)]
        high = sum(1 for row in batch if row["confidence"].strip() == "high")
        medium = sum(1 for row in batch if row["confidence"].strip() == "medium")
        low = sum(1 for row in batch if row["confidence"].strip() == "low")

        self.assertGreaterEqual(high, 6)
        self.assertLessEqual(medium, 2)
        self.assertEqual(low, 0)

        for row in batch:
            source = row["source"].strip()
            self.assertTrue(source, f"missing source for {row['name']}")
            source_path = REPO_ROOT / source
            self.assertTrue(source_path.exists(), f"missing source file: {source}")

    def test_message_schema_contains_stage4e_pack(self) -> None:
        schema_path = REPO_ROOT / "analysis/protocol/message_schema.json"
        self.assertTrue(schema_path.exists())

        payload = json.loads(schema_path.read_text(encoding="utf-8"))
        messages = payload.get("messages", [])
        self.assertIsInstance(messages, list)

        index = {(entry["scope"], entry["name"]): entry for entry in messages}
        self.assertEqual(set(index.keys()) & S4E_MESSAGES, S4E_MESSAGES)

        batch = [index[key] for key in sorted(S4E_MESSAGES)]
        high = sum(1 for entry in batch if entry["confidence"] == "high")
        medium = sum(1 for entry in batch if entry["confidence"] == "medium")
        low = sum(1 for entry in batch if entry["confidence"] == "low")

        self.assertGreaterEqual(high, 6)
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
