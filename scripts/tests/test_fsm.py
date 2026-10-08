from __future__ import annotations

import os
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))

from kutha_gov.checks import get_checks  # noqa: E402
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
        ctx = Context(root=ROOT, skip_cargo=True, skip_emit=True)
        machine = load_machine(ROOT)
        budget = machine.budget_default
        self.assertGreaterEqual(budget, 48)
        outcome = run_quantum(ctx, budget=budget, start_event="start")
        self.assertEqual("ok", outcome.terminal)
        self.assertEqual(
            [
                "idle",
                "load_constitution",
                "load_checks",
                "load_fsm",
                "load_relations",
                "load_invariants",
                "load_bridges",
                "load_honeycomb",
                "run_checks",
                "run_selftest",
                "observe_cargo",
                "observe_rocks",
                "emit",
                "emit_tenant",
                "fold",
                "decide",
                "ok",
            ],
            outcome.trace,
        )
        self.assertEqual(0, outcome.high)
        self.assertTrue(
            any(rel == "h5_selftest" for rel, _obj in outcome.evidence),
            msg=outcome.evidence,
        )
        self.assertEqual(0, outcome.skipped)
        self.assertEqual(len(get_checks(ROOT)), len(outcome.results))

    def test_skip_emit_does_not_call_append_run(self) -> None:
        from unittest.mock import patch

        from kutha_gov.protocol import Check, CheckResult
        from kutha_gov.selftest import OK, SelftestRow

        class QuietCheck(Check):
            name = "quiet"
            description = "quiet"

            def run(self, ctx: Context) -> CheckResult:
                return CheckResult(check=self.name)

        ctx = Context(root=ROOT, skip_cargo=True, skip_emit=True)
        with (
            patch("kutha_gov.fsm.append_run") as append_run,
            patch(
                "kutha_gov.selftest.run_selftest",
                return_value=([SelftestRow(OK, "probe", 1)], 0),
            ),
            patch("kutha_gov.fsm.get_checks", return_value={"quiet": QuietCheck()}),
        ):
            outcome = run_quantum(ctx, budget=48, start_event="start")
        append_run.assert_not_called()
        self.assertEqual("ok", outcome.terminal)
        self.assertIn("emit", outcome.trace)

    def test_jobs_gt1_observe_without_overlap_is_high(self) -> None:
        from unittest.mock import patch

        from kutha_gov.fsm import Machine

        machine = Machine(
            initial="idle",
            terminal=frozenset({"ok", "fail"}),
            budget_default=8,
            states={
                "idle": {"kind": "require_file", "path": "Cargo.toml"},
                "obs": {"kind": "observe_cargo", "bin": "cargo", "args": ["test"]},
                "ok": {"kind": "terminal"},
                "fail": {"kind": "terminal"},
            },
            transitions=[
                {"from": "idle", "event": "start", "to": "obs"},
                {"from": "obs", "event": "done", "to": "ok"},
                {"from": "obs", "event": "fail", "to": "fail"},
            ],
        )
        ctx = Context(root=ROOT, skip_cargo=False, jobs=2)
        with patch("kutha_gov.fsm.load_machine", return_value=machine):
            outcome = run_quantum(ctx, budget=8, start_event="start")
        self.assertTrue(
            any(f.category == "overlap-required" for f in outcome.findings),
            msg=outcome.findings,
        )

    def test_skip_cargo_does_not_call_cargo_or_tenant(self) -> None:
        from unittest.mock import patch

        from kutha_gov.protocol import Check, CheckResult
        from kutha_gov.selftest import OK, SelftestRow

        class QuietCheck(Check):
            name = "quiet"
            description = "quiet"

            def run(self, ctx: Context) -> CheckResult:
                return CheckResult(check=self.name)

        ctx = Context(root=ROOT, skip_cargo=True)
        with (
            patch("kutha_gov.observe.run_cargo_observation") as cargo,
            patch("kutha_gov.tenant.run_tenant_observation") as tenant,
            patch(
                "kutha_gov.selftest.run_selftest",
                return_value=([SelftestRow(OK, "probe", 1)], 0),
            ) as selftest,
            patch("kutha_gov.fsm.get_checks", return_value={"quiet": QuietCheck()}),
        ):
            outcome = run_quantum(ctx, budget=48, start_event="start")
        cargo.assert_not_called()
        tenant.assert_not_called()
        selftest.assert_called()
        self.assertEqual("ok", outcome.terminal)
        self.assertEqual([], outcome.observations)
        self.assertTrue(any(rel == "h5_selftest" for rel, _obj in outcome.evidence))
        self.assertIn("observe_cargo", outcome.trace)
        self.assertIn("observe_rocks", outcome.trace)
        self.assertIn("emit_tenant", outcome.trace)
        self.assertIn("run_selftest", outcome.trace)

    def test_high_from_checks_skips_cargo_after_selftest(self) -> None:
        from unittest.mock import patch

        from kutha_gov.protocol import Check, CheckResult, Finding
        from kutha_gov.selftest import OK, SelftestRow

        class HighCheck(Check):
            name = "forced-high"
            description = "forced"

            def run(self, ctx: Context) -> CheckResult:
                result = CheckResult(check=self.name)
                result.findings.append(Finding(self.name, Severity.HIGH, "forced", "forced HIGH"))
                return result

        ctx = Context(root=ROOT)
        with (
            patch("kutha_gov.observe.run_cargo_observation") as cargo,
            patch("kutha_gov.tenant.run_tenant_observation") as tenant,
            patch(
                "kutha_gov.selftest.run_selftest",
                return_value=([SelftestRow(OK, "probe", 1)], 0),
            ) as selftest,
            patch("kutha_gov.fsm.get_checks", return_value={"forced-high": HighCheck()}),
        ):
            outcome = run_quantum(ctx, budget=48, start_event="start")
        cargo.assert_not_called()
        tenant.assert_not_called()
        selftest.assert_called()
        self.assertEqual("fail", outcome.terminal)
        self.assertGreater(outcome.high, 0)
        self.assertEqual([], outcome.observations)
        self.assertTrue(any(rel == "h5_selftest" for rel, _obj in outcome.evidence))
        self.assertIn("run_selftest", outcome.trace)
        self.assertIn("observe_cargo", outcome.trace)
        self.assertIn("observe_rocks", outcome.trace)

    def test_cui_budget_truncates_tail_checks_when_v_is_short(self) -> None:
        checks = get_checks(ROOT)
        n = len(checks)
        self.assertGreater(n, 1)
        budget = n - 1
        ordered = sorted(checks.items())
        selected = ordered[:budget]
        skipped = len(ordered) - len(selected)
        self.assertEqual(1, skipped)
        self.assertEqual(budget, len(selected))
        self.assertGreaterEqual(load_machine(ROOT).budget_default, n)

    def test_cui_budget_skipped_checks_is_high(self) -> None:
        from unittest.mock import patch

        from kutha_gov.protocol import Check, CheckResult

        class ProbeCheck(Check):
            name = "probe-a"

            def run(self, ctx: Context) -> CheckResult:
                return CheckResult(check=self.name)

        class ProbeCheckB(Check):
            name = "probe-b"

            def run(self, ctx: Context) -> CheckResult:
                return CheckResult(check=self.name)

        ctx = Context(root=ROOT, skip_cargo=True)
        with patch(
            "kutha_gov.fsm.get_checks",
            return_value={"probe-a": ProbeCheck(), "probe-b": ProbeCheckB()},
        ):
            outcome = run_quantum(ctx, budget=1, start_event="start")
        self.assertEqual(1, outcome.skipped)
        budget_highs = [
            f for f in outcome.findings if f.category == "budget" and f.severity is Severity.HIGH
        ]
        self.assertEqual(1, len(budget_highs))
        self.assertEqual("fail", outcome.terminal)

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

    def test_fold_log_bad_jsonl_raises_value_error_with_line(self) -> None:
        import json
        import tempfile

        from kutha_gov.time_log import HarnessEvent, fold_log

        raw = tempfile.mkdtemp(prefix="kutha-fold-")
        path = Path(raw) / "events.jsonl"
        good = HarnessEvent(
            id="01",
            op="assert",
            subject="harness.run",
            relation="status",
            object="ok",
            ingested_at=1,
            valid_from=1,
        )
        path.write_text(json.dumps(good.to_json()) + "\nNOT_JSON\n", encoding="utf-8")
        with self.assertRaises(ValueError) as ctx:
            fold_log(path)
        msg = str(ctx.exception)
        self.assertIn("invalid JSONL at line 2", msg)
        self.assertIsInstance(ctx.exception, ValueError)
        self.assertNotIsInstance(ctx.exception, json.JSONDecodeError)

    def test_fsm_fold_log_bad_jsonl_fails_observed(self) -> None:
        import json
        import tempfile

        from kutha_gov.fsm import QuantumOutcome, _execute
        from kutha_gov.time_log import LOG_REL, HarnessEvent

        raw = tempfile.mkdtemp(prefix="kutha-fsm-fold-")
        root = Path(raw)
        (root / ".kutha").mkdir()
        good = HarnessEvent(
            id="01",
            op="assert",
            subject="harness.run",
            relation="status",
            object="ok",
            ingested_at=1,
            valid_from=1,
        )
        (root / LOG_REL).write_text(json.dumps(good.to_json()) + "\nbad\n", encoding="utf-8")
        outcome = QuantumOutcome(terminal="", trace=[])
        event = _execute(
            "fold_log",
            {},
            Context(root=root),
            outcome,
            budget=1,
            state="fold",
            overlap={},
        )
        self.assertEqual("fail", event)
        self.assertTrue(any(f.severity is Severity.HIGH for f in outcome.findings))
        self.assertIn(("fold_log", "fail"), outcome.observations)


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

    def test_required_ff_ok_accepts_module_prefix(self) -> None:
        from kutha_gov.observe import interpret_cargo_output

        name = "open_without_snapshot_recovers_intern_meanings"
        line = "test quantum::tests::open_without_snapshot_recovers_intern_meanings ... ok\n"
        seen, findings = interpret_cargo_output(line, [name])
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
