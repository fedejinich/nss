from __future__ import annotations

import csv
import json
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]

CORE_MESSAGES = {
    ("server", "SM_LOGIN"),
    ("server", "SM_SET_WAIT_PORT"),
    ("server", "SM_GET_PEER_ADDRESS"),
    ("server", "SM_CONNECT_TO_PEER"),
    ("server", "SM_FILE_SEARCH"),
    ("server", "SM_SEARCH_ROOM"),
    ("server", "SM_EXACT_FILE_SEARCH"),
    ("server", "SM_SEARCH_USER_FILES"),
    ("server", "SM_MESSAGE_USER"),
    ("server", "SM_MESSAGE_ACKED"),
    ("server", "SM_GET_USER_STATS"),
    ("server", "SM_GET_USER_STATUS"),
    ("server", "SM_SHARED_FOLDERS_FILES"),
    ("server", "SM_DOWNLOAD_SPEED"),
    ("server", "SM_UPLOAD_SPEED"),
    ("peer", "PM_GET_SHARED_FILE_LIST"),
    ("peer", "PM_SHARED_FILE_LIST"),
    ("peer", "PM_FILE_SEARCH_REQUEST"),
    ("peer", "PM_FILE_SEARCH_RESULT"),
    ("peer", "PM_TRANSFER_REQUEST"),
    ("peer", "PM_TRANSFER_RESPONSE"),
    ("peer", "PM_QUEUE_UPLOAD"),
    ("peer", "PM_UPLOAD_PLACE_IN_LINE"),
    ("peer", "PM_UPLOAD_FAILED"),
    ("peer", "PM_UPLOAD_DENIED"),
}


class Stage2ProtocolContractTests(unittest.TestCase):
    def test_message_map_has_25_core_messages_with_confidence_thresholds(self) -> None:
        message_map_path = REPO_ROOT / "analysis/ghidra/maps/message_map.csv"
        self.assertTrue(message_map_path.exists())

        with message_map_path.open("r", encoding="utf-8") as fh:
            rows = list(csv.DictReader(fh))

        keys = {(row["scope"].strip(), row["name"].strip()) for row in rows}
        self.assertEqual(keys & CORE_MESSAGES, CORE_MESSAGES)

        core_rows = [row for row in rows if (row["scope"].strip(), row["name"].strip()) in CORE_MESSAGES]
        self.assertEqual(len(core_rows), 25)

        high = sum(1 for row in core_rows if row["confidence"].strip() == "high")
        medium = sum(1 for row in core_rows if row["confidence"].strip() == "medium")
        low = sum(1 for row in core_rows if row["confidence"].strip() == "low")

        self.assertGreaterEqual(high, 18)
        self.assertLessEqual(medium, 7)
        self.assertEqual(low, 0)

        for row in core_rows:
            source = row["source"].strip()
            self.assertTrue(source, f"missing source for {row['name']}")
            source_path = REPO_ROOT / source
            self.assertTrue(source_path.exists(), f"missing source file: {source}")

    def test_message_schema_covers_contract_and_evidence(self) -> None:
        schema_path = REPO_ROOT / "analysis/protocol/message_schema.json"
        self.assertTrue(schema_path.exists())

        payload = json.loads(schema_path.read_text(encoding="utf-8"))
        messages = payload.get("messages", [])
        self.assertIsInstance(messages, list)

        index = {(entry["scope"], entry["name"]): entry for entry in messages}
        self.assertEqual(set(index.keys()) & CORE_MESSAGES, CORE_MESSAGES)

        core_entries = [index[key] for key in sorted(CORE_MESSAGES)]
        high = sum(1 for entry in core_entries if entry["confidence"] == "high")
        medium = sum(1 for entry in core_entries if entry["confidence"] == "medium")
        low = sum(1 for entry in core_entries if entry["confidence"] == "low")

        self.assertGreaterEqual(high, 18)
        self.assertLessEqual(medium, 7)
        self.assertEqual(low, 0)

        for entry in core_entries:
            evidence = entry.get("evidence", [])
            self.assertTrue(evidence, f"missing evidence for {entry['name']}")
            for ev in evidence:
                source = ev.get("source", "").strip()
                self.assertTrue(source, f"evidence without source for {entry['name']}")
                if source.startswith("http://") or source.startswith("https://"):
                    continue
                source_path = REPO_ROOT / source
                self.assertTrue(source_path.exists(), f"missing evidence source: {source}")


if __name__ == "__main__":
    unittest.main()
