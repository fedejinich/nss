from __future__ import annotations

import csv
import json
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]

S5D_S5H_MESSAGES = {
    ("server", "SM_JOIN_GLOBAL_ROOM"),
    ("server", "SM_LEAVE_GLOBAL_ROOM"),
    ("server", "SM_SAY_GLOBAL_ROOM"),
    ("server", "SM_SEARCH_CORRELATIONS"),
    ("server", "SM_SET_PARENT_INACTIVITY_BEFORE_DISCONNECT"),
    ("server", "SM_SET_SERVER_INACTIVITY_BEFORE_DISCONNECT"),
    ("server", "SM_NODES_IN_CACHE_BEFORE_DISCONNECT"),
    ("server", "SM_SET_SECONDS_BEFORE_PING_CHILDREN"),
    ("server", "SM_CAN_PARENT"),
    ("server", "SM_REMOVE_OWN_ROOM_MEMBERSHIP"),
    ("server", "SM_GIVE_UP_ROOM"),
    ("server", "SM_ADD_ROOM_MEMBERSHIP"),
    ("server", "SM_REMOVE_ROOM_MEMBERSHIP"),
    ("server", "SM_ADD_ROOM_OPERATORSHIP"),
    ("server", "SM_COMMAND"),
    ("server", "SM_ROOM_ADDED"),
    ("server", "SM_ROOM_REMOVED"),
    ("server", "SM_ADMIN_MESSAGE"),
    ("server", "SM_ADD_HATE_TERM"),
    ("server", "SM_REMOVE_HATE_TERM"),
    ("server", "SM_SET_STATUS"),
    ("server", "SM_HEARTBEAT"),
    ("server", "SM_DNET_RESET"),
}


class Stage5DS5HTypedControlContractTests(unittest.TestCase):
    def test_message_map_contains_stage5d_s5h_pack(self) -> None:
        message_map_path = REPO_ROOT / "analysis/ghidra/maps/message_map.csv"
        self.assertTrue(message_map_path.exists())

        with message_map_path.open("r", encoding="utf-8") as fh:
            rows = list(csv.DictReader(fh))

        index = {(row["scope"].strip(), row["name"].strip()): row for row in rows}
        self.assertEqual(set(index.keys()) & S5D_S5H_MESSAGES, S5D_S5H_MESSAGES)

        batch = [index[key] for key in sorted(S5D_S5H_MESSAGES)]
        high = sum(1 for row in batch if row["confidence"].strip() == "high")
        medium = sum(1 for row in batch if row["confidence"].strip() == "medium")
        low = sum(1 for row in batch if row["confidence"].strip() == "low")

        self.assertGreaterEqual(high, 23)
        self.assertLessEqual(medium, 0)
        self.assertEqual(low, 0)

        for row in batch:
            source = row["source"].strip()
            self.assertTrue(source, f"missing source for {row['name']}")
            source_path = REPO_ROOT / source
            self.assertTrue(source_path.exists(), f"missing source file: {source}")

    def test_message_schema_contains_stage5d_s5h_pack(self) -> None:
        schema_path = REPO_ROOT / "analysis/protocol/message_schema.json"
        self.assertTrue(schema_path.exists())

        payload = json.loads(schema_path.read_text(encoding="utf-8"))
        messages = payload.get("messages", [])
        self.assertIsInstance(messages, list)

        index = {(entry["scope"], entry["name"]): entry for entry in messages}
        self.assertEqual(set(index.keys()) & S5D_S5H_MESSAGES, S5D_S5H_MESSAGES)

        batch = [index[key] for key in sorted(S5D_S5H_MESSAGES)]
        high = sum(1 for entry in batch if entry["confidence"] == "high")
        medium = sum(1 for entry in batch if entry["confidence"] == "medium")
        low = sum(1 for entry in batch if entry["confidence"] == "low")

        self.assertGreaterEqual(high, 23)
        self.assertLessEqual(medium, 0)
        self.assertEqual(low, 0)

        for entry in batch:
            schema_payload = entry.get("payload", [])
            self.assertIsInstance(schema_payload, list)
            self.assertTrue(
                len(schema_payload) >= 1,
                f"payload should be typed for {entry['name']}",
            )
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
