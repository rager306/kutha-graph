"""M111 S01 / Phase 185 — hot-path lease honesty dogfood (rebuild live + tt_by_ingest)."""

from __future__ import annotations

import unittest
from pathlib import Path

from kutha_gov.checks import get_checks
from kutha_gov.protocol import Context, Severity

ROOT = Path(__file__).resolve().parents[2]


class HotPathLeaseHonestyTests(unittest.TestCase):
    def test_check_enrolled(self) -> None:
        self.assertIn("hot-path-lease-honesty", get_checks(ROOT))

    def test_readme_and_code_needles_pass(self) -> None:
        result = get_checks(ROOT)["hot-path-lease-honesty"].run(Context(root=ROOT))
        highs = [f for f in result.findings if f.severity is Severity.HIGH]
        self.assertEqual([], highs, [f.message for f in highs])


if __name__ == "__main__":
    unittest.main()
