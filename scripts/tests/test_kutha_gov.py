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
                "docs-entry",
                "state-readme",
                "version-freeze",
                "changelog-planes",
                "docs-coupling",
                "invariants-ledger",
                "bridges-ledger",
                "honeycomb-ledger",
                "h4-membership-as-of",
                "h4-lease",
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

    def _write_process_relations(self, root: Path, names: list[str]) -> None:
        dict_dir = root / ".kutha" / "dictionaries"
        dict_dir.mkdir(parents=True)
        body = "schema: kutha-harness-relations/v1\nrelations:\n" + "".join(
            f"  - {name}\n" for name in names
        )
        (dict_dir / "relations.yaml").write_text(body, encoding="utf-8")

    def test_membership_edition_appends_one_sorted_snapshot(self) -> None:
        import json
        import tempfile

        from kutha_gov.time_log import (
            LOG_REL,
            MEMBERSHIP_RELATION,
            MEMBERSHIP_SUBJECT,
            append_membership_edition,
            encode_membership_object,
        )

        with tempfile.TemporaryDirectory() as raw:
            root = Path(raw)
            self._write_process_relations(
                root, ["status", "high", "low", "checks", "cargo", "allows"]
            )
            event, rejected = append_membership_edition(
                root, frozenset({"cargo", "status", "allows"})
            )
            self.assertEqual([], rejected)
            self.assertIsNotNone(event)
            assert event is not None
            self.assertEqual(MEMBERSHIP_SUBJECT, event.subject)
            self.assertEqual(MEMBERSHIP_RELATION, event.relation)
            self.assertEqual("allows,cargo,status", event.object)
            self.assertEqual(
                "allows,cargo,status", encode_membership_object({"status", "cargo", "allows"})
            )
            lines = (root / LOG_REL).read_text(encoding="utf-8").strip().splitlines()
            self.assertEqual(1, len(lines))
            row = json.loads(lines[0])
            self.assertEqual("assert", row["op"])
            self.assertEqual("allows,cargo,status", row["object"])

    def test_membership_edition_rejected_when_allows_omitted(self) -> None:
        import tempfile

        from kutha_gov.time_log import LOG_REL, append_membership_edition

        with tempfile.TemporaryDirectory() as raw:
            root = Path(raw)
            self._write_process_relations(root, ["status", "high", "low", "checks", "cargo"])
            event, rejected = append_membership_edition(root, frozenset({"status"}))
            self.assertIn("allows", rejected)
            self.assertIsNone(event)
            self.assertFalse((root / LOG_REL).is_file())

    def test_tip_without_status_rejects_status_keeps_allows(self) -> None:
        import tempfile

        from kutha_gov.time_log import LOG_REL, append_run

        with tempfile.TemporaryDirectory() as raw:
            root = Path(raw)
            self._write_process_relations(root, ["high", "low", "checks", "cargo", "allows"])
            _event, rejected = append_run(root, high=0, low=0, check_count=1)
            self.assertIn("status", rejected)
            log_path = root / LOG_REL
            if log_path.is_file():
                log = log_path.read_text(encoding="utf-8")
                self.assertNotIn('"relation":"status"', log)
            _event2, rejected2 = append_run(
                root,
                high=0,
                low=0,
                check_count=1,
                observations=[("cargo", "ok")],
            )
            self.assertNotIn("cargo", rejected2)
            log = (root / LOG_REL).read_text(encoding="utf-8")
            self.assertIn('"relation":"cargo"', log)
            self.assertNotIn('"relation":"status"', log)

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

    def test_git_path_implies_requires_changelog_for_crate_diff(self) -> None:
        ctx = Context(root=ROOT, changed_paths=frozenset({"crates/kutha-runtime/src/lib.rs"}))
        result = CheckResult(check="probe")
        run_step(
            "probe",
            {
                "kind": "git_path_implies",
                "when_any": ["crates/**/*.rs"],
                "then_any": ["CHANGELOG.md"],
            },
            ctx,
            result,
        )
        highs = [f for f in result.findings if f.severity is Severity.HIGH]
        self.assertEqual(1, len(highs))
        self.assertEqual("docs-coupling", highs[0].category)

    def test_git_path_implies_passes_when_changelog_in_same_diff(self) -> None:
        ctx = Context(
            root=ROOT,
            changed_paths=frozenset({"crates/kutha-runtime/src/lib.rs", "CHANGELOG.md"}),
        )
        result = CheckResult(check="probe")
        run_step(
            "probe",
            {
                "kind": "git_path_implies",
                "when_any": ["crates/**/*.rs"],
                "then_any": ["CHANGELOG.md"],
            },
            ctx,
            result,
        )
        highs = [f for f in result.findings if f.severity is Severity.HIGH]
        self.assertEqual([], highs)

    def test_yaml_map_list_rejects_unknown_disposition(self) -> None:
        result = CheckResult(check="probe")
        run_step(
            "probe",
            {
                "kind": "yaml_map_list",
                "path": ".kutha/dictionaries/invariants.yaml",
                "select": "invariants",
                "field": "disposition",
                "allowed": ["never-a-disposition"],
            },
            self.ctx,
            result,
        )
        highs = [f for f in result.findings if f.severity is Severity.HIGH]
        self.assertTrue(highs)
        self.assertEqual("yaml-map-vocab", highs[0].category)

    def test_yaml_map_list_requires_check_ids_in_other_file(self) -> None:
        result = CheckResult(check="probe")
        run_step(
            "probe",
            {
                "kind": "yaml_map_list",
                "path": ".kutha/dictionaries/checks.yaml",
                "select": "checks",
                "field": "id",
                "other": "AGENTS.md",
                "prefix": "governor-check-id-absent-",
            },
            self.ctx,
            result,
        )
        highs = [f for f in result.findings if f.severity is Severity.HIGH]
        self.assertTrue(highs)
        self.assertEqual("yaml-map-ref", highs[0].category)

    def test_yaml_map_list_absent_other_flags_overlap(self) -> None:
        result = CheckResult(check="probe")
        run_step(
            "probe",
            {
                "kind": "yaml_map_list",
                "path": ".kutha/dictionaries/checks.yaml",
                "select": "checks",
                "field": "id",
                "other": ".kutha/dictionaries/invariants.yaml",
                "prefix": "    check: ",
                "absent_other": True,
            },
            self.ctx,
            result,
        )
        highs = [f for f in result.findings if f.severity is Severity.HIGH]
        self.assertTrue(highs)
        self.assertEqual("yaml-map-overlap", highs[0].category)

    def test_glob_paths_in_file_flags_missing_adr(self) -> None:
        result = CheckResult(check="probe")
        run_step(
            "probe",
            {
                "kind": "glob_paths_in_file",
                "glob": "docs/ADR/ADR-*.md",
                "path": "AGENTS.md",
                "prefix": "    path: ",
            },
            self.ctx,
            result,
        )
        highs = [f for f in result.findings if f.severity is Severity.HIGH]
        self.assertTrue(highs)
        self.assertEqual("glob-paths", highs[0].category)

    def test_map_command_dumps_honeycomb(self) -> None:
        import json
        from contextlib import redirect_stdout
        from io import StringIO

        self.assertEqual(0, main(["--root", str(ROOT), "map"]))
        self.assertEqual(0, main(["--root", str(ROOT), "map", "ADR-042"]))
        self.assertEqual(0, main(["--root", str(ROOT), "map", "adr-042"]))
        buf = StringIO()
        with redirect_stdout(buf):
            self.assertEqual(0, main(["--root", str(ROOT), "--format", "json", "map", "042"]))
        payload = json.loads(buf.getvalue())
        self.assertEqual("kutha-map-report/v1", payload["schema"])
        self.assertFalse(payload["authoritative"])
        ids = {cell["id"] for cell in payload["cells"]}
        self.assertIn("ADR-042", ids)
        self.assertIn("ADR-000", ids)
        self.assertNotIn("ADR-093", ids)
        self.assertEqual(2, main(["--root", str(ROOT), "map", "ADR-999"]))

    def test_precommit_unknown_check_is_contract_error(self) -> None:
        self.assertEqual(2, main(["--root", str(ROOT), "--check", "no-such-check", "precommit"]))

    def test_precommit_one_check_skips_cargo(self) -> None:
        self.assertEqual(0, main(["--root", str(ROOT), "--check", "docs-entry", "precommit"]))

    def test_path_matches_dir_glob_covers_nested_files(self) -> None:
        from kutha_gov.gitdiff import path_matches

        self.assertTrue(path_matches("scripts/kutha_gov/kinds.py", "scripts/kutha_gov/**"))
        self.assertTrue(path_matches("crates/kutha-runtime/src/lib.rs", "crates/**/*.rs"))
        self.assertFalse(path_matches("CHANGELOG.md", "crates/**/*.rs"))

    def test_harness_ids_are_uuid_version_7(self) -> None:
        from kutha_gov.time_log import _now_v7

        parsed = __import__("uuid").UUID(_now_v7())
        self.assertEqual(7, parsed.version)


if __name__ == "__main__":
    unittest.main()
