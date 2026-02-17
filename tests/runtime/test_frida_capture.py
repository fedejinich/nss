from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime, timezone
import unittest
from typing import Any

from tools.runtime.frida_capture import attach_target, candidate_pids


@dataclass
class FakeProcess:
    name: str
    pid: int
    parameters: dict[str, Any]


class FakeSession:
    def __init__(self, target: int | str) -> None:
        self.target = target


class FakeDevice:
    def __init__(
        self,
        processes: list[FakeProcess],
        *,
        attach_failures: set[int] | None = None,
        supports_scope: bool = True,
    ) -> None:
        self._processes = processes
        self._attach_failures = attach_failures or set()
        self._supports_scope = supports_scope
        self.attach_calls: list[int | str] = []

    def enumerate_processes(self, scope: str | None = None) -> list[FakeProcess]:
        if scope is not None and not self._supports_scope:
            raise TypeError("scope unsupported")
        return list(self._processes)

    def attach(self, target: int | str) -> FakeSession:
        self.attach_calls.append(target)
        if isinstance(target, int) and target in self._attach_failures:
            raise RuntimeError(f"attach failed for pid {target}")
        return FakeSession(target)


class FridaCaptureProcessSelectionTests(unittest.TestCase):
    def test_candidate_pids_filters_by_name_and_path(self) -> None:
        procs = [
            FakeProcess(
                name="SoulseekQt",
                pid=200,
                parameters={
                    "path": "/Applications/SoulseekQt.app/Contents/MacOS/SoulseekQt",
                    "started": datetime(2026, 2, 17, 1, 0, 0, tzinfo=timezone.utc),
                },
            ),
            FakeProcess(
                name="SoulseekQt",
                pid=100,
                parameters={
                    "path": "/Users/void/Applications/SoulseekQt-Debug.app/Contents/MacOS/SoulseekQt",
                    "started": datetime(2026, 2, 17, 2, 0, 0, tzinfo=timezone.utc),
                },
            ),
            FakeProcess(
                name="OtherApp",
                pid=300,
                parameters={
                    "path": "/Applications/OtherApp.app/Contents/MacOS/OtherApp",
                    "started": datetime(2026, 2, 17, 3, 0, 0, tzinfo=timezone.utc),
                },
            ),
        ]

        self.assertEqual(candidate_pids(procs, process_name="SoulseekQt"), [100, 200])
        self.assertEqual(
            candidate_pids(
                procs,
                process_name="SoulseekQt",
                process_path_contains="SoulseekQt-Debug.app",
            ),
            [100],
        )

    def test_attach_target_uses_filtered_pid(self) -> None:
        device = FakeDevice(
            [
                FakeProcess(
                    name="SoulseekQt",
                    pid=100,
                    parameters={
                        "path": "/Users/void/Applications/SoulseekQt-Debug.app/Contents/MacOS/SoulseekQt",
                        "started": datetime(2026, 2, 17, 2, 0, 0, tzinfo=timezone.utc),
                    },
                ),
                FakeProcess(
                    name="SoulseekQt",
                    pid=200,
                    parameters={
                        "path": "/Applications/SoulseekQt.app/Contents/MacOS/SoulseekQt",
                        "started": datetime(2026, 2, 17, 1, 0, 0, tzinfo=timezone.utc),
                    },
                ),
            ]
        )

        session, attached = attach_target(
            device,
            "SoulseekQt",
            process_path_contains="SoulseekQt-Debug.app",
        )
        self.assertEqual(attached, 100)
        self.assertEqual(session.target, 100)
        self.assertEqual(device.attach_calls, [100])

    def test_attach_target_skips_failed_pid_and_tries_next(self) -> None:
        device = FakeDevice(
            [
                FakeProcess(
                    name="SoulseekQt",
                    pid=200,
                    parameters={
                        "path": "/Applications/SoulseekQt.app/Contents/MacOS/SoulseekQt",
                        "started": datetime(2026, 2, 17, 2, 0, 0, tzinfo=timezone.utc),
                    },
                ),
                FakeProcess(
                    name="SoulseekQt",
                    pid=100,
                    parameters={
                        "path": "/Applications/SoulseekQt.app/Contents/MacOS/SoulseekQt",
                        "started": datetime(2026, 2, 17, 1, 0, 0, tzinfo=timezone.utc),
                    },
                ),
            ],
            attach_failures={200},
        )

        session, attached = attach_target(device, "SoulseekQt")
        self.assertEqual(attached, 100)
        self.assertEqual(session.target, 100)
        self.assertEqual(device.attach_calls, [200, 100])

    def test_attach_target_falls_back_to_name_when_no_candidates(self) -> None:
        device = FakeDevice([], supports_scope=False)
        session, attached = attach_target(device, "SoulseekQt")
        self.assertEqual(attached, "SoulseekQt")
        self.assertEqual(session.target, "SoulseekQt")
        self.assertEqual(device.attach_calls, ["SoulseekQt"])


if __name__ == "__main__":
    unittest.main()
