from __future__ import annotations

import os
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))

from kutha_gov.config import resolve_budget  # noqa: E402
from kutha_gov.fsm import load_machine, run_quantum, step_kind  # noqa: E402
from kutha_gov.protocol import Context, Severity  # noqa: E402


class FsmTests(unittest.TestCase):
    def test_machine_loads_from_dictionary(self) -> None:
        machine = load_machine(ROOT)
        self.assertEqual("idle", machine.initial)
        self.assertIn("ok", machine.terminal)
        self.assertIn("fail", machine.terminal)
        self.assertGreaterEqual(len(machine.transitions), 8)

    def test_unknown_state_kind_fails_closed(self) -> None:
        finding = step_kind("not_a_kind")
        self.assertIsNotNone(finding)
        assert finding is not None
        self.assertEqual(finding.severity, Severity.HIGH)
        self.assertEqual(finding.category, "unknown-fsm-kind")

    def test_unknown_transition_fails_closed(self) -> None:
        ctx = Context(root=ROOT)
        outcome = run_quantum(ctx, budget=32, start_event="no-such-event")
        self.assertEqual("fail", outcome.terminal)
        self.assertTrue(
            any(f.category == "unknown-transition" for f in outcome.findings),
            msg=outcome.findings,
        )

    def test_ci_quantum_reaches_ok_on_this_tree(self) -> None:
        ctx = Context(root=ROOT)
        outcome = run_quantum(ctx, budget=32, start_event="start")
        self.assertEqual("ok", outcome.terminal)
        self.assertEqual(
            [
                "idle",
                "load_constitution",
                "load_checks",
                "load_fsm",
                "run_checks",
                "emit",
                "fold",
                "decide",
                "ok",
            ],
            outcome.trace,
        )
        self.assertEqual(0, outcome.high)

    def test_budget_comes_from_env_or_fsm_not_python_literal(self) -> None:
        self.assertEqual(4, resolve_budget(cli=4, fsm_default=32))
        previous = os.environ.pop("KUTHA_GOV_BUDGET", None)
        try:
            os.environ["KUTHA_GOV_BUDGET"] = "7"
            self.assertEqual(7, resolve_budget(cli=None, fsm_default=32))
            del os.environ["KUTHA_GOV_BUDGET"]
            self.assertEqual(32, resolve_budget(cli=None, fsm_default=32))
        finally:
            if previous is None:
                os.environ.pop("KUTHA_GOV_BUDGET", None)
            else:
                os.environ["KUTHA_GOV_BUDGET"] = previous


if __name__ == "__main__":
    unittest.main()
