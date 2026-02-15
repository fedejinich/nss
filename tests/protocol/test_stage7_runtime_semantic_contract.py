from __future__ import annotations

import csv
import json
import unittest
from pathlib import Path


class Stage7RuntimeSemanticContractTests(unittest.TestCase):
    def test_runtime_coverage_is_strictly_complete(self) -> None:
        repo_root = Path(__file__).resolve().parents[2]
        message_map_path = repo_root / "analysis/ghidra/maps/message_map.csv"
        with message_map_path.open("r", encoding="utf-8") as fh:
            rows = list(csv.DictReader(fh))

        verified_runtime = sum(1 for row in rows if row.get("status") == "verified_runtime")
        verified_static = sum(1 for row in rows if row.get("status") == "verified_static")

        self.assertEqual(len(rows), 131)
        self.assertEqual(verified_runtime, 131)
        self.assertEqual(verified_static, 0)

    def test_message_schema_has_no_unresolved_raw_tail_fields(self) -> None:
        repo_root = Path(__file__).resolve().parents[2]
        payload = json.loads(
            (repo_root / "analysis/protocol/message_schema.json").read_text(encoding="utf-8")
        )
        messages = payload.get("messages", [])
        unresolved = []

        for message in messages:
            fields = message.get("payload", [])
            if not isinstance(fields, list):
                continue
            for field in fields:
                if not isinstance(field, dict):
                    continue
                name = str(field.get("name") or "")
                if name in {"raw_tail", "raw_payload"}:
                    unresolved.append(
                        (
                            message.get("scope"),
                            message.get("code"),
                            message.get("name"),
                            name,
                        )
                    )

        self.assertEqual(unresolved, [])


if __name__ == "__main__":
    unittest.main()
