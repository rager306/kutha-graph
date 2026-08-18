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
                "load_relations",
                "run_checks",
                "observe_cargo",
                "emit",
                "emit_tenant",
                "fold",
                "decide",
                "ok",
            ],
            outcome.trace,
        )
        self.assertEqual(0, outcome.high)
        self.assertIn(("cargo", "ok"), outcome.observations)
        self.assertIn(("tenant", "ok"), outcome.observations)
        self.assertIn(("ff5_as_of_t1_differs_from_as_of_t2_on_statute_log", "ok"), outcome.evidence)

    def test_fsm_transition_keys_are_not_yaml_booleans(self) -> None:
        machine = load_machine(ROOT)
        for row in machine.transitions:
            self.assertIn("event", row)
            self.assertNotIn(True, row.keys())
            self.assertNotIn(False, row.keys())

    def test_glob_none_fails_when_a_match_exists(self) -> None:
        from kutha_gov.kinds import run_step
        from kutha_gov.protocol import CheckResult

        result = CheckResult(check="probe")
        ctx = Context(root=ROOT)
        run_step("probe", {"kind": "glob_none", "glob": "Cargo.toml"}, ctx, result)
        highs = [f for f in result.findings if f.severity is Severity.HIGH]
        self.assertEqual(1, len(highs))
        self.assertEqual("glob-none", highs[0].category)

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


class ObserveTests(unittest.TestCase):
    def test_missing_required_ff_is_high_evidence_not_sot(self) -> None:
        from kutha_gov.observe import interpret_cargo_output

        findings = interpret_cargo_output(
            "test foo ... ok\n", ["ff5_as_of_t1_differs_from_as_of_t2_on_statute_log"]
        )[1]
        highs = [f for f in findings if f.severity is Severity.HIGH]
        self.assertEqual(1, len(highs))
        self.assertEqual("observe-missing", highs[0].category)

    def test_required_ff_ok_is_observation_without_high(self) -> None:
        from kutha_gov.observe import interpret_cargo_output

        name = "ff5_as_of_t1_differs_from_as_of_t2_on_statute_log"
        seen, findings = interpret_cargo_output(f"test {name} ... ok\n", [name])
        self.assertEqual([name], seen)
        self.assertEqual([], findings)

    def test_required_ff_failed_is_high(self) -> None:
        from kutha_gov.observe import interpret_cargo_output

        name = "ff5_as_of_t1_differs_from_as_of_t2_on_statute_log"
        seen, findings = interpret_cargo_output(f"test {name} ... FAILED\n", [name])
        self.assertEqual([], seen)
        self.assertEqual(1, len(findings))
        self.assertEqual("observe-fail", findings[0].category)


class TenantTests(unittest.TestCase):
    def test_missing_tenant_line_is_high(self) -> None:
        from kutha_gov.tenant import interpret_tenant_output

        findings = interpret_tenant_output("cargo finished\n")
        self.assertEqual(1, len(findings))
        self.assertEqual("tenant-missing", findings[0].category)

    def test_as_of_match_is_silent(self) -> None:
        from kutha_gov.tenant import interpret_tenant_output

        text = "tenant: ingested=4 last_status=ok last_vf=2000 as_of_match=1 dir=.kutha/tenant\n"
        self.assertEqual([], interpret_tenant_output(text))

    def test_tenant_bin_prefers_env(self) -> None:
        from kutha_gov.tenant import resolve_tenant_bin

        previous = os.environ.get("KUTHA_TENANT_BIN")
        try:
            os.environ["KUTHA_TENANT_BIN"] = "/tmp/kutha-tenant-probe"
            self.assertEqual(
                "/tmp/kutha-tenant-probe",
                resolve_tenant_bin(ROOT, {"bin": "ignored"}),
            )
        finally:
            if previous is None:
                os.environ.pop("KUTHA_TENANT_BIN", None)
            else:
                os.environ["KUTHA_TENANT_BIN"] = previous

    def test_tenant_bin_uses_cargo_target_dir(self) -> None:
        from kutha_gov.tenant import resolve_tenant_bin

        previous_bin = os.environ.pop("KUTHA_TENANT_BIN", None)
        previous_td = os.environ.get("CARGO_TARGET_DIR")
        try:
            os.environ["CARGO_TARGET_DIR"] = "/tmp/kutha-target"
            self.assertEqual(
                "/tmp/kutha-target/debug/kutha-tenant",
                resolve_tenant_bin(ROOT, {"bin": "kutha-tenant"}),
            )
        finally:
            if previous_bin is None:
                os.environ.pop("KUTHA_TENANT_BIN", None)
            else:
                os.environ["KUTHA_TENANT_BIN"] = previous_bin
            if previous_td is None:
                os.environ.pop("CARGO_TARGET_DIR", None)
            else:
                os.environ["CARGO_TARGET_DIR"] = previous_td

    def test_tenant_observation_execs_env_bin_not_cargo(self) -> None:
        from unittest.mock import MagicMock, patch

        from kutha_gov.tenant import run_tenant_observation

        previous = os.environ.get("KUTHA_TENANT_BIN")
        calls: list[list[str]] = []

        def fake_run(cmd: list[str], **_kwargs: object) -> MagicMock:
            calls.append(list(cmd))
            completed = MagicMock()
            completed.returncode = 0
            completed.stdout = (
                "tenant: ingested=2 last_status=ok last_vf=1001 as_of_match=1 dir=.kutha/tenant\n"
            )
            completed.stderr = ""
            return completed

        try:
            os.environ["KUTHA_TENANT_BIN"] = "/tmp/kutha-tenant-probe"
            with patch("kutha_gov.tenant.subprocess.run", side_effect=fake_run):
                code, findings, _text = run_tenant_observation(ROOT, {"bin": "kutha-tenant"})
            self.assertEqual(0, code)
            self.assertEqual([], findings)
            self.assertEqual([["/tmp/kutha-tenant-probe"]], calls)
        finally:
            if previous is None:
                os.environ.pop("KUTHA_TENANT_BIN", None)
            else:
                os.environ["KUTHA_TENANT_BIN"] = previous


class ObserveBuildTests(unittest.TestCase):
    def test_observe_runs_build_after_test(self) -> None:
        from unittest.mock import MagicMock, patch

        from kutha_gov.observe import run_cargo_observation

        calls: list[list[str]] = []

        def fake_run(cmd: list[str], **_kwargs: object) -> MagicMock:
            calls.append(list(cmd))
            completed = MagicMock()
            completed.returncode = 0
            completed.stdout = "test foo ... ok\n"
            completed.stderr = ""
            return completed

        spec = {
            "bin": "cargo",
            "args": ["test", "--workspace", "--offline"],
            "build": {
                "bin": "cargo",
                "args": ["build", "--offline", "-p", "kutha-runtime", "--bin", "kutha-tenant"],
            },
            "required": ["foo"],
        }
        with patch("kutha_gov.observe.subprocess.run", side_effect=fake_run):
            code, seen, findings = run_cargo_observation(ROOT, spec)
        self.assertEqual(0, code)
        self.assertEqual(["foo"], seen)
        self.assertEqual([], findings)
        self.assertEqual(
            [
                ["cargo", "test", "--workspace", "--offline"],
                ["cargo", "build", "--offline", "-p", "kutha-runtime", "--bin", "kutha-tenant"],
            ],
            calls,
        )

    def test_observe_skips_build_after_timeout(self) -> None:
        import subprocess
        from unittest.mock import patch

        from kutha_gov.observe import run_cargo_observation

        calls: list[list[str]] = []

        def fake_run(cmd: list[str], **_kwargs: object) -> object:
            calls.append(list(cmd))
            raise subprocess.TimeoutExpired(cmd=cmd, timeout=1)

        spec = {
            "bin": "cargo",
            "args": ["test", "--offline"],
            "build": {"bin": "cargo", "args": ["build", "--offline"]},
            "required": ["foo"],
        }
        with patch("kutha_gov.observe.subprocess.run", side_effect=fake_run):
            code, _seen, findings = run_cargo_observation(ROOT, spec)
        self.assertEqual(124, code)
        self.assertEqual(1, len(calls))
        self.assertTrue(any(f.category == "observe-error" for f in findings))


if __name__ == "__main__":
    unittest.main()
