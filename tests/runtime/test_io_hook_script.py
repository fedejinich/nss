from __future__ import annotations

import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
IO_HOOK_SCRIPT = REPO_ROOT / "frida/hooks/soulseek_io_trace.js"


class IoHookScriptRegressionTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.source = IO_HOOK_SCRIPT.read_text(encoding="utf-8")

    def test_qt_symbol_candidates_include_underscore_variants(self) -> None:
        required_symbols = [
            "_ZN11QDataStream10writeBytesEPKcx",
            "__ZN11QDataStream10writeBytesEPKcx",
            "_ZN11QDataStream11readRawDataEPcx",
            "__ZN11QDataStream11readRawDataEPcx",
            "_ZN5QFile4openE6QFlagsIN13QIODeviceBase12OpenModeFlagEE",
            "__ZN5QFile4openE6QFlagsIN13QIODeviceBase12OpenModeFlagEE",
            "_ZN9QSettings8setValueE14QAnyStringViewRK8QVariant",
            "__ZN9QSettings8setValueE14QAnyStringViewRK8QVariant",
            "_ZNK9QSettings5valueE14QAnyStringView",
            "__ZNK9QSettings5valueE14QAnyStringView",
        ]

        for symbol in required_symbols:
            self.assertIn(f'"{symbol}"', self.source)

    def test_export_lookup_has_find_and_get_fallback(self) -> None:
        self.assertIn("function safeFindExportByName", self.source)
        self.assertIn('typeof Module.findExportByName === "function"', self.source)
        self.assertIn('typeof Module.getExportByName === "function"', self.source)
        self.assertIn("function enumerateModuleSymbols", self.source)

    def test_libc_module_tokens_match_runtime_modules(self) -> None:
        self.assertIn('moduleToken: "libsystem_c"', self.source)
        self.assertIn('moduleToken: "libsystem_kernel"', self.source)


if __name__ == "__main__":
    unittest.main()
