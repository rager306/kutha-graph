from __future__ import annotations

import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))

from kutha_gov.__main__ import main  # noqa: E402
from kutha_gov.checks import get_checks  # noqa: E402
from kutha_gov.kinds import run_step  # noqa: E402
from kutha_gov.protocol import CheckResult, Context, Severity  # noqa: E402


class HarnessTests(unittest.TestCase):
    def setUp(self) -> None:
        self.ctx = Context(root=ROOT)

    def test_discovers_core_checks(self) -> None:
        names = set(get_checks(ROOT))
        self.assertTrue(
            {
                "trajectory",
                "lifecycles",
                "adr-status",
                "freeze",
                "honeycomb-map",
                "dogfood",
                "toolchain",
                "meta-prompt",
                "relation-allowlist",
                "observe-required-fn",
                "unnamed-csr",
                "harness-relations",
                "plane-mix-dicts",
                "tenant-bin",
            }.issubset(names)
        )

    def test_checks_come_from_dictionary_not_python_classes(self) -> None:
        check_dir = ROOT / "scripts" / "kutha_gov" / "checks"
        extras = [p.name for p in check_dir.glob("*.py") if p.name != "__init__.py"]
        self.assertEqual([], extras)
        yaml_text = (ROOT / ".kutha" / "dictionaries" / "checks.yaml").read_text(encoding="utf-8")
        self.assertIn("schema: kutha-harness-checks/v1", yaml_text)

    def test_unknown_kind_fails_closed(self) -> None:
        result = CheckResult(check="probe")
        run_step("probe", {"kind": "not_a_kind"}, self.ctx, result)
        highs = [f for f in result.findings if f.severity is Severity.HIGH]
        self.assertEqual(1, len(highs))
        self.assertEqual("unknown-kind", highs[0].category)

    def test_ci_green_on_this_tree(self) -> None:
        self.assertEqual(0, main(["--root", str(ROOT), "ci"]))

    def test_no_high_on_live_tree(self) -> None:
        for name, chk in get_checks(ROOT).items():
            result = chk.run(self.ctx)
            highs = [f for f in result.findings if f.severity is Severity.HIGH]
            self.assertEqual([], highs, msg=f"{name}: {highs}")

    def test_harness_fold_is_deterministic(self) -> None:
        from kutha_gov.time_log import HarnessEvent, HarnessFold

        events = [
            HarnessEvent("a", "assert", "harness.run", "status", "ok", 1, 1),
            HarnessEvent("b", "assert", "harness.run", "high", "0", 1, 1, "a"),
            HarnessEvent("c", "assert", "harness.run", "status", "fail", 2, 2),
            HarnessEvent("d", "assert", "harness.run", "high", "2", 2, 2, "c"),
            HarnessEvent("e", "assert", "harness.observe", "cargo", "ok", 2, 2, "c"),
        ]
        first, second = HarnessFold(), HarnessFold()
        for event in events:
            first.apply(event)
            second.apply(event)
        self.assertEqual(first.last_run, second.last_run)
        self.assertEqual("fail", first.last_run)
        self.assertEqual(2, first.last_high)
        self.assertEqual(2, first.run_count)
        self.assertEqual("ok", first.last_cargo)

    def test_unknown_process_relation_does_not_append(self) -> None:
        import tempfile

        from kutha_gov.time_log import LOG_REL, append_run

        with tempfile.TemporaryDirectory() as raw:
            root = Path(raw)
            dict_dir = root / ".kutha" / "dictionaries"
            dict_dir.mkdir(parents=True)
            (dict_dir / "relations.yaml").write_text(
                "schema: kutha-harness-relations/v1\nrelations:\n"
                "  - status\n  - high\n  - low\n  - checks\n  - cargo\n",
                encoding="utf-8",
            )
            _event, rejected = append_run(
                root,
                high=0,
                low=0,
                check_count=1,
                observations=[("notAProcessRelation", "ok")],
            )
            self.assertIn("notAProcessRelation", rejected)
            log = (root / LOG_REL).read_text(encoding="utf-8")
            self.assertNotIn("notAProcessRelation", log)
            self.assertIn('"relation":"status"', log)

    def test_yaml_needles_in_glob_fails_when_fn_missing(self) -> None:
        result = CheckResult(check="probe")
        run_step(
            "probe",
            {
                "kind": "yaml_needles_in_glob",
                "path": ".kutha/dictionaries/fsm.yaml",
                "select": "states.observe_cargo.required",
                "glob": "AGENTS.md",
                "prefix": "fn ",
            },
            self.ctx,
            result,
        )
        highs = [f for f in result.findings if f.severity is Severity.HIGH]
        self.assertTrue(highs)
        self.assertEqual("yaml-needles", highs[0].category)

    def test_harness_ids_are_uuid_version_7(self) -> None:
        from kutha_gov.time_log import _now_v7

        parsed = __import__("uuid").UUID(_now_v7())
        self.assertEqual(7, parsed.version)


if __name__ == "__main__":
    unittest.main()
