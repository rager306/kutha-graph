"""M073 S01 — ci summary separates this-run HIGH from fold tip."""

from __future__ import annotations

import unittest
from io import StringIO
from pathlib import Path
from unittest import mock

from kutha_gov.__main__ import cmd_ci
from kutha_gov.fsm import QuantumOutcome
from kutha_gov.protocol import CheckResult, Context, Finding, Severity
from kutha_gov.time_log import HarnessFold


ROOT = Path(__file__).resolve().parents[2]


class CiSummaryLabelTests(unittest.TestCase):
    def test_cmd_ci_prints_run_high_and_tip_high(self) -> None:
        outcome = QuantumOutcome(terminal="ok", trace=["ok"])
        outcome.high = 0
        outcome.low = 2
        outcome.results = [CheckResult(check="x")]
        picture = HarnessFold()
        picture.run_count = 3
        picture.last_run = "fail"
        picture.last_high = 5
        picture.last_cargo = "skipped"
        picture.last_tenant = "skipped"
        ctx = Context(root=ROOT, skip_cargo=True)
        buf = StringIO()
        with (
            mock.patch("kutha_gov.__main__.run_quantum", return_value=outcome),
            mock.patch("kutha_gov.__main__.fold_log", return_value=picture),
            mock.patch("sys.stdout", buf),
        ):
            code = cmd_ci(ctx, budget=1)
        text = buf.getvalue()
        self.assertEqual(0, code)
        self.assertIn("run_high=0", text)
        self.assertIn("tip_high=5", text)
        self.assertIn("tip_last=fail", text)
        self.assertNotRegex(text, r"(?m)^harness: \d+ HIGH")


if __name__ == "__main__":
    unittest.main()
