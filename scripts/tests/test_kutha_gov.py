from __future__ import annotations

import atexit
import os
import shutil
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))

import yaml  # noqa: E402

from kutha_gov.__main__ import main  # noqa: E402
from kutha_gov.checks import get_checks  # noqa: E402
from kutha_gov.dictionary import DictCheck  # noqa: E402
from kutha_gov.kinds import run_step  # noqa: E402
from kutha_gov.protocol import CheckResult, Context, Finding, Severity  # noqa: E402
from kutha_gov.rust_source import first_assert_macro_span, parse_fns, strip_noise  # noqa: E402
from kutha_gov.selftest import _derive_mutations  # noqa: E402


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
                "freeze-rocks",
                "freeze-cypher-deps",
                "freeze-hnsw-dep",
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
                "idle-delivery-closed",
                "m010-semantic-open",
                "m011-claim-supports",
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

    def test_stub_ci_is_not_named_green_on_this_tree(self) -> None:
        src = Path(__file__).read_text(encoding="utf-8")
        lying = "def test_ci_" + "green_on_this_tree"
        self.assertNotIn(
            lying,
            src,
            "stubbed cargo quantum must not claim live-tree green (CE review P1 #7)",
        )

    def test_ci_fsm_reaches_ok_with_stubbed_observe(self) -> None:
        # Stub cargo/tenant: proves FSM visits observe_cargo, not live-tree green.
        # Exit may be nonzero when other HIGHs already exist (skip_heavy_observe).
        # Live observe_cargo is `uv run kutha-gov ci` or KUTHA_LIVE_CARGO=1.
        from contextlib import redirect_stdout
        from io import StringIO

        bindir, tenant = _stub_ci_bins()
        previous_path = os.environ.get("PATH", "")
        previous_tenant = os.environ.get("KUTHA_TENANT_BIN")
        buf = StringIO()
        try:
            os.environ["PATH"] = bindir + os.pathsep + previous_path
            os.environ["KUTHA_TENANT_BIN"] = tenant
            with redirect_stdout(buf):
                main(["--root", str(ROOT), "ci"])
            cargo_on_path = Path(bindir) / "cargo"
            self.assertTrue(cargo_on_path.is_file(), "stub cargo must be first on PATH")
            out = buf.getvalue()
            self.assertIn("observe_cargo", out)
            self.assertNotIn("observe-missing", out)
            self.assertIn("stub-ci-honest-name", out)
            self.assertRegex(out, r"OK\s+stub-ci-honest-name")
        finally:
            os.environ["PATH"] = previous_path
            if previous_tenant is None:
                os.environ.pop("KUTHA_TENANT_BIN", None)
            else:
                os.environ["KUTHA_TENANT_BIN"] = previous_tenant

    @unittest.skipUnless(
        os.environ.get("KUTHA_LIVE_CARGO") == "1",
        "set KUTHA_LIVE_CARGO=1 to run real observe_cargo (serial; not pytest default)",
    )
    def test_ci_observe_cargo_live(self) -> None:
        self.assertIsNone(os.environ.get("KUTHA_TENANT_BIN"))
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
            dict_dir.mkdir(parents=True, exist_ok=True)
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
        dict_dir.mkdir(parents=True, exist_ok=True)
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

    def test_process_relations_reject_comma_in_name(self) -> None:
        import tempfile

        from kutha_gov.process_allow import load_process_relations

        with tempfile.TemporaryDirectory() as raw:
            root = Path(raw)
            self._write_process_relations(root, ["status", "a,b"])
            with self.assertRaisesRegex(ValueError, "must not contain"):
                load_process_relations(root)

    def test_load_map_rejects_non_mapping_cells(self) -> None:
        import tempfile

        from kutha_gov.honeycomb import load_map

        with tempfile.TemporaryDirectory() as raw:
            root = Path(raw)
            path = root / ".kutha" / "dictionaries" / "honeycomb.yaml"
            path.parent.mkdir(parents=True)
            path.write_text(
                "schema: kutha-map-honeycomb/v1\ncells:\n  - not-a-map\n",
                encoding="utf-8",
            )
            with self.assertRaisesRegex(ValueError, "list of mappings"):
                load_map(root)

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

    def test_yaml_needles_live_precommit_is_green(self) -> None:
        """M129: live dictionary uses yaml_needles_in_glob (not derive-only)."""
        from contextlib import redirect_stdout
        from io import StringIO

        buf = StringIO()
        with redirect_stdout(buf):
            code = main(["--root", str(ROOT), "--check", "yaml-needles-live", "precommit"])
        self.assertEqual(0, code, msg=buf.getvalue())
        self.assertIn("yaml-needles-live", buf.getvalue())

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

    def test_yaml_map_list_rejects_scalar_rows(self) -> None:
        """CE 2026-10-15 residual: scalar rows must not be dropped before oracles run."""
        rel = "tmp/m099-yaml-scalar.yaml"
        path = ROOT / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(
            "rows:\n  - id: kept\n  - just-a-string\n",
            encoding="utf-8",
        )
        try:
            result = CheckResult(check="probe")
            run_step(
                "probe",
                {
                    "kind": "yaml_map_list",
                    "path": rel,
                    "select": "rows",
                    "require_fields": ["id"],
                },
                self.ctx,
                result,
            )
            highs = [f for f in result.findings if f.severity is Severity.HIGH]
            self.assertTrue(highs)
            self.assertIn("non-mapping", highs[0].message)
        finally:
            path.unlink(missing_ok=True)

    def test_yaml_map_list_rejects_empty_select_list(self) -> None:
        """M131 / CE residual: empty select must not vacuous-pass require/unique."""
        rel = "tmp/m131-yaml-empty.yaml"
        path = ROOT / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text("items: []\n", encoding="utf-8")
        try:
            result = CheckResult(check="probe")
            run_step(
                "probe",
                {
                    "kind": "yaml_map_list",
                    "path": rel,
                    "select": "items",
                    "require_fields": ["id"],
                },
                self.ctx,
                result,
            )
            highs = [f for f in result.findings if f.severity is Severity.HIGH]
            self.assertTrue(highs, msg="empty select must HIGH")
            self.assertEqual("yaml-select", highs[0].category)
            self.assertIn("not a non-empty list", highs[0].message)
        finally:
            path.unlink(missing_ok=True)

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

    def test_sync_membership_appends_when_tip_differs(self) -> None:
        import tempfile

        from kutha_gov.time_log import (
            LOG_REL,
            MEMBERSHIP_RELATION,
            sync_membership_edition_if_changed,
        )

        with tempfile.TemporaryDirectory() as raw:
            root = Path(raw)
            self._write_process_relations(
                root, ["status", "high", "low", "checks", "cargo", "allows"]
            )
            appended, rejected = sync_membership_edition_if_changed(root)
            self.assertEqual([], rejected)
            self.assertTrue(appended)
            log = (root / LOG_REL).read_text(encoding="utf-8")
            self.assertIn(f'"relation":"{MEMBERSHIP_RELATION}"', log)
            self.assertIn("allows,cargo,checks,high,low,status", log)

            appended2, rejected2 = sync_membership_edition_if_changed(root)
            self.assertEqual([], rejected2)
            self.assertFalse(appended2)
            log2 = (root / LOG_REL).read_text(encoding="utf-8")
            self.assertEqual(1, log2.count('"relation":"allows"'))

    def test_sync_membership_appends_again_after_tip_change(self) -> None:
        import tempfile

        from kutha_gov.time_log import LOG_REL, sync_membership_edition_if_changed

        with tempfile.TemporaryDirectory() as raw:
            root = Path(raw)
            self._write_process_relations(
                root, ["status", "high", "low", "checks", "cargo", "allows"]
            )
            sync_membership_edition_if_changed(root)
            self._write_process_relations(
                root, ["status", "high", "low", "checks", "cargo", "allows", "tenant"]
            )
            appended, rejected = sync_membership_edition_if_changed(root)
            self.assertEqual([], rejected)
            self.assertTrue(appended)
            log = (root / LOG_REL).read_text(encoding="utf-8")
            self.assertEqual(2, log.count('"relation":"allows"'))
            self.assertIn("allows,cargo,checks,high,low,status,tenant", log)

    def test_harness_ids_are_uuid_version_7(self) -> None:
        from kutha_gov.time_log import _now_v7

        parsed = __import__("uuid").UUID(_now_v7())
        self.assertEqual(7, parsed.version)

    def test_when_match_then_match_skips_without_when(self) -> None:
        import tempfile

        with tempfile.TemporaryDirectory() as raw:
            root = Path(raw)
            state = root / "STATE.md"
            state.write_text(
                "**Active Milestone:** M002\nL_delivery=M002-open\n",
                encoding="utf-8",
            )
            ctx = Context(root=root)
            result = CheckResult(check="probe")
            run_step(
                "probe",
                {
                    "kind": "when_match_then_match",
                    "path": "STATE.md",
                    "pattern": r"^\*\*Active Milestone:\*\*\s*None\s*$",
                    "flags": ["multiline"],
                    "then_pattern": r"(?m)^L_delivery=\S+-closed\s*$",
                },
                ctx,
                result,
            )
            self.assertEqual([], result.findings)

    def test_when_match_then_match_fails_when_then_missing(self) -> None:
        import tempfile

        with tempfile.TemporaryDirectory() as raw:
            root = Path(raw)
            state = root / "STATE.md"
            state.write_text(
                "**Active Milestone:** None\nL_delivery=M001-open\n",
                encoding="utf-8",
            )
            ctx = Context(root=root)
            result = CheckResult(check="probe")
            run_step(
                "probe",
                {
                    "kind": "when_match_then_match",
                    "path": "STATE.md",
                    "pattern": r"^\*\*Active Milestone:\*\*\s*None\s*$",
                    "flags": ["multiline"],
                    "then_pattern": r"(?m)^L_delivery=\S+-closed\s*$",
                    "message": "idle without closed delivery",
                },
                ctx,
                result,
            )
            self.assertEqual(1, len(result.findings))
            self.assertIn("idle without closed delivery", result.findings[0].message)

    def test_when_match_then_match_passes_idle_closed(self) -> None:
        import tempfile

        with tempfile.TemporaryDirectory() as raw:
            root = Path(raw)
            state = root / "STATE.md"
            state.write_text(
                "**Active Milestone:** None\nL_delivery=M001-closed\n",
                encoding="utf-8",
            )
            ctx = Context(root=root)
            result = CheckResult(check="probe")
            run_step(
                "probe",
                {
                    "kind": "when_match_then_match",
                    "path": "STATE.md",
                    "pattern": r"^\*\*Active Milestone:\*\*\s*None\s*$",
                    "flags": ["multiline"],
                    "then_pattern": r"(?m)^L_delivery=\S+-closed\s*$",
                },
                ctx,
                result,
            )
            self.assertEqual([], result.findings)

    def test_freeze_split_ids_name_real_gates(self) -> None:
        yaml_text = (ROOT / ".kutha" / "dictionaries" / "checks.yaml").read_text(encoding="utf-8")
        self.assertNotIn("before M001 L_capability is green", yaml_text)
        self.assertNotIn("until Active Milestone names M002", yaml_text)
        gates = {
            "freeze-rocks": "optional",
            "freeze-cypher-deps": "M003",
            "freeze-hnsw-dep": "M005",
        }
        checks = get_checks(ROOT)
        for check_id, gate in gates.items():
            self.assertIn(check_id, checks)
            chk = checks[check_id]
            self.assertIsInstance(chk, DictCheck)
            assert isinstance(chk, DictCheck)
            globs = [step.get("glob") for step in chk.steps]
            self.assertEqual(["crates/*/Cargo.toml", "Cargo.toml"], globs)
            for i, step in enumerate(chk.steps):
                self.assertEqual("cargo_dep_constraint", step.get("kind"))
                self.assertEqual("frozen-dep", step.get("category"))
                message = str(step.get("message", ""))
                self.assertIn(gate, message)
                self.assertNotIn("until Active Milestone names M002", message)
                self.assertNotIn("STATE freeze still blocks", message)
                self.assertNotIn("unless_file_contains", step)
                self.assertNotIn("unless_line_contains", step)
                rules = step.get("rules")
                self.assertIsInstance(rules, dict)
                assert isinstance(rules, dict)
                if check_id == "freeze-rocks" and i == 0:
                    self.assertEqual("require_optional", rules.get("rocksdb"))
                elif check_id == "freeze-rocks":
                    self.assertEqual("forbid", rules.get("rocksdb"))
                else:
                    self.assertTrue(rules)
                    self.assertTrue(all(action == "forbid" for action in rules.values()))


def _stub_ci_bins() -> tuple[str, str]:
    """PATH `cargo` + KUTHA_TENANT_BIN that satisfy observe without compiling."""
    from kutha_gov.fsm import load_machine

    raw = tempfile.mkdtemp(prefix="kutha-ci-stub-")
    atexit.register(shutil.rmtree, raw, ignore_errors=True)
    bindir = Path(raw)
    machine = load_machine(ROOT)
    ws_raw = machine.states["observe_cargo"].get("required", [])
    rocks_raw = machine.states["observe_rocks"].get("required", [])
    ws = [n for n in ws_raw if isinstance(n, str)] if isinstance(ws_raw, list) else []
    rocks = [n for n in rocks_raw if isinstance(n, str)] if isinstance(rocks_raw, list) else []
    cargo = bindir / "cargo"
    cargo.write_text(
        "#!/usr/bin/env python3\n"
        "import sys\n"
        f"WS = {ws!r}\n"
        f"ROCKS = {rocks!r}\n"
        "args = sys.argv[1:]\n"
        "joined = ' '.join(args)\n"
        "if 'build' in args:\n"
        "    sys.exit(0)\n"
        "names = ROCKS if 'rocks-index' in joined else WS\n"
        "for name in names:\n"
        "    print(f'test {name} ... ok')\n"
        "sys.exit(0)\n",
        encoding="utf-8",
    )
    cargo.chmod(0o755)
    tenant = bindir / "kutha-tenant"
    tenant.write_text(
        "#!/bin/sh\n"
        "echo 'tenant: ingested=1 last_status=ok last_vf=1 as_of_match=1 dir=.kutha/tenant'\n",
        encoding="utf-8",
    )
    tenant.chmod(0o755)
    return str(bindir), str(tenant)


def _rust_tree(files: dict[str, str]) -> Path:
    raw = tempfile.mkdtemp(prefix="kutha-rust-test-")
    atexit.register(shutil.rmtree, raw, ignore_errors=True)
    root = Path(raw)
    for rel, text in files.items():
        path = root / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
    return root


def _rust_run(root: Path, step: dict[str, object]) -> CheckResult:
    result = CheckResult(check="probe")
    run_step("probe", step, Context(root=root), result)
    return result


def _rust_highs(result: CheckResult) -> list[str]:
    return [f.category for f in result.findings if f.severity is Severity.HIGH]


_FREEZE_IDS = ("freeze-rocks", "freeze-cypher-deps", "freeze-hnsw-dep")
_CLEAN_CRATE_TOML = '[package]\nname = "kutha-common"\nversion = "0.0.0"\n'
_CLEAN_WORKSPACE_TOML = '[workspace]\nmembers = ["crates/kutha-common"]\n'
_STATE_M002 = "**Active Milestone:** M002\n"
_STATE_OTHER = "**Active Milestone:** M018\n"


def _crate_deps(body: str) -> str:
    return _CLEAN_CRATE_TOML + "[dependencies]\n" + body


def _workspace_deps(body: str) -> str:
    return _CLEAN_WORKSPACE_TOML + "[workspace.dependencies]\n" + body


def _freeze_tree(
    *,
    crate: str = _CLEAN_CRATE_TOML,
    workspace: str = _CLEAN_WORKSPACE_TOML,
    state: str | None = None,
) -> Path:
    files = {
        "crates/kutha-common/Cargo.toml": crate,
        "Cargo.toml": workspace,
    }
    if state is not None:
        files[".kutha/STATE.md"] = state
    return _rust_tree(files)


def _freeze_highs(root: Path) -> dict[str, list[Finding]]:
    checks = get_checks(ROOT)
    out: dict[str, list[Finding]] = {}
    for check_id in _FREEZE_IDS:
        result = checks[check_id].run(Context(root=root))
        out[check_id] = [f for f in result.findings if f.severity is Severity.HIGH]
    return out


class FreezeDepTests(unittest.TestCase):
    def test_clean_cargo_tomls_pass(self) -> None:
        highs = _freeze_highs(_freeze_tree())
        self.assertEqual({check_id: [] for check_id in _FREEZE_IDS}, highs)

    def test_rocksdb_in_crate_hits_freeze_rocks_only(self) -> None:
        highs = _freeze_highs(_freeze_tree(crate=_crate_deps('rocksdb = "0.22"\n')))
        self.assertEqual(1, len(highs["freeze-rocks"]))
        finding = highs["freeze-rocks"][0]
        self.assertEqual("frozen-dep", finding.category)
        self.assertEqual("crates/kutha-common/Cargo.toml", finding.file)
        self.assertIn("rocksdb", finding.message)
        self.assertIn("optional", finding.message)
        self.assertEqual([], highs["freeze-cypher-deps"])
        self.assertEqual([], highs["freeze-hnsw-dep"])

    def test_rocksdb_in_workspace_root_hits_freeze_rocks_only(self) -> None:
        highs = _freeze_highs(_freeze_tree(workspace=_workspace_deps('rocksdb = "0.22"\n')))
        self.assertEqual(1, len(highs["freeze-rocks"]))
        finding = highs["freeze-rocks"][0]
        self.assertEqual("Cargo.toml", finding.file)
        self.assertIn("rocksdb", finding.message)
        self.assertIn("workspace", finding.message)
        self.assertEqual([], highs["freeze-cypher-deps"])
        self.assertEqual([], highs["freeze-hnsw-dep"])

    def test_neo4j_in_crate_hits_freeze_cypher_deps_only(self) -> None:
        highs = _freeze_highs(_freeze_tree(crate=_crate_deps('neo4j = "0.1"\n')))
        self.assertEqual(1, len(highs["freeze-cypher-deps"]))
        finding = highs["freeze-cypher-deps"][0]
        self.assertEqual("crates/kutha-common/Cargo.toml", finding.file)
        self.assertIn("neo4j", finding.message)
        self.assertIn("M003", finding.message)
        self.assertEqual([], highs["freeze-rocks"])
        self.assertEqual([], highs["freeze-hnsw-dep"])

    def test_graphiti_and_falkordb_hit_freeze_cypher_deps_only(self) -> None:
        for needle in ("graphiti", "falkordb"):
            with self.subTest(needle=needle):
                highs = _freeze_highs(_freeze_tree(crate=_crate_deps(f'{needle} = "0.1"\n')))
                self.assertEqual(1, len(highs["freeze-cypher-deps"]))
                self.assertIn(needle, highs["freeze-cypher-deps"][0].message)
                self.assertEqual([], highs["freeze-rocks"])
                self.assertEqual([], highs["freeze-hnsw-dep"])

    def test_hnswlib_in_crate_hits_freeze_hnsw_dep_only(self) -> None:
        highs = _freeze_highs(_freeze_tree(crate=_crate_deps('hnswlib = "0.1"\n')))
        self.assertEqual(1, len(highs["freeze-hnsw-dep"]))
        finding = highs["freeze-hnsw-dep"][0]
        self.assertEqual("crates/kutha-common/Cargo.toml", finding.file)
        self.assertIn("hnswlib", finding.message)
        self.assertIn("M005", finding.message)
        self.assertEqual([], highs["freeze-rocks"])
        self.assertEqual([], highs["freeze-cypher-deps"])

    def test_optional_rocksdb_passes_regardless_of_active_milestone(self) -> None:
        optional = _crate_deps('rocksdb = { version = "0.22", optional = true }\n')
        for state in (None, _STATE_M002, _STATE_OTHER):
            with self.subTest(state=state):
                highs = _freeze_highs(_freeze_tree(crate=optional, state=state))
                self.assertEqual([], highs["freeze-rocks"])
                self.assertEqual([], highs["freeze-cypher-deps"])
                self.assertEqual([], highs["freeze-hnsw-dep"])

    def test_non_optional_rocksdb_table_form_fails(self) -> None:
        highs = _freeze_highs(
            _freeze_tree(
                crate=_crate_deps('rocksdb = { version = "0.22" }\n'),
                state=_STATE_M002,
            )
        )
        self.assertEqual(1, len(highs["freeze-rocks"]))
        finding = highs["freeze-rocks"][0]
        self.assertEqual("frozen-dep", finding.category)
        self.assertIn("rocksdb", finding.message)
        self.assertIn("optional", finding.message)
        self.assertEqual([], highs["freeze-cypher-deps"])
        self.assertEqual([], highs["freeze-hnsw-dep"])

    def test_optional_rocksdb_comment_does_not_trip_freeze_rocks(self) -> None:
        crate = _crate_deps(
            "# those are rocksdb defaults\n" + 'rocksdb = { version = "0.22", optional = true }\n'
        )
        highs = _freeze_highs(_freeze_tree(crate=crate, state=_STATE_OTHER))
        self.assertEqual([], highs["freeze-rocks"])

    def test_cypher_and_hnsw_fail_regardless_of_m002(self) -> None:
        cases = (
            ("neo4j", "freeze-cypher-deps", "M003"),
            ("graphiti", "freeze-cypher-deps", "M003"),
            ("falkordb", "freeze-cypher-deps", "M003"),
            ("hnswlib", "freeze-hnsw-dep", "M005"),
        )
        for needle, check_id, gate in cases:
            with self.subTest(needle=needle):
                highs = _freeze_highs(
                    _freeze_tree(
                        crate=_crate_deps(f'{needle} = "0.1"\n'),
                        state=_STATE_M002,
                    )
                )
                self.assertEqual(1, len(highs[check_id]))
                self.assertIn(needle, highs[check_id][0].message)
                self.assertIn(gate, highs[check_id][0].message)
                self.assertEqual([], highs["freeze-rocks"])

    def test_table_form_dependencies_rocksdb_is_high(self) -> None:
        crate = _CLEAN_CRATE_TOML + '[dependencies.rocksdb]\nversion = "0.22"\n'
        highs = _freeze_highs(_freeze_tree(crate=crate))
        self.assertEqual(1, len(highs["freeze-rocks"]))
        self.assertEqual("frozen-dep", highs["freeze-rocks"][0].category)
        self.assertIn("rocksdb", highs["freeze-rocks"][0].message)
        self.assertEqual([], highs["freeze-cypher-deps"])
        self.assertEqual([], highs["freeze-hnsw-dep"])

    def test_rocksdb_assignment_without_spaces_is_high(self) -> None:
        crate = _CLEAN_CRATE_TOML + '[dependencies]\nrocksdb="0.22"\n'
        highs = _freeze_highs(_freeze_tree(crate=crate))
        self.assertEqual(1, len(highs["freeze-rocks"]))
        self.assertIn("rocksdb", highs["freeze-rocks"][0].message)

    def test_same_line_optional_true_comment_is_high(self) -> None:
        crate = _CLEAN_CRATE_TOML + '[dependencies]\nrocksdb = "0.22" # optional = true\n'
        highs = _freeze_highs(_freeze_tree(crate=crate))
        self.assertEqual(1, len(highs["freeze-rocks"]))
        self.assertIn("optional", highs["freeze-rocks"][0].message)

    def test_same_line_optional_true_on_another_key_is_high(self) -> None:
        crate = (
            _CLEAN_CRATE_TOML + '[dependencies]\nrocksdb = "0.22"  unused = { optional = true }\n'
        )
        highs = _freeze_highs(_freeze_tree(crate=crate))
        self.assertGreaterEqual(len(highs["freeze-rocks"]), 1)

    def test_workspace_dependencies_rocksdb_table_is_high(self) -> None:
        workspace = _CLEAN_WORKSPACE_TOML + '[workspace.dependencies.rocksdb]\nversion = "0.22"\n'
        highs = _freeze_highs(_freeze_tree(workspace=workspace))
        self.assertEqual(1, len(highs["freeze-rocks"]))
        self.assertEqual("Cargo.toml", highs["freeze-rocks"][0].file)
        self.assertIn("workspace", highs["freeze-rocks"][0].message)

    def test_target_cfg_unix_dependencies_rocksdb_is_high(self) -> None:
        crate = (
            _CLEAN_CRATE_TOML + "[target.'cfg(unix)'.dependencies.rocksdb]\nversion = \"0.22\"\n"
        )
        highs = _freeze_highs(_freeze_tree(crate=crate))
        self.assertEqual(1, len(highs["freeze-rocks"]))
        self.assertIn("rocksdb", highs["freeze-rocks"][0].message)

    def test_package_rename_to_rocksdb_is_high(self) -> None:
        crate = (
            _CLEAN_CRATE_TOML + '[dependencies]\nfoo = { package = "rocksdb", version = "0.22" }\n'
        )
        highs = _freeze_highs(_freeze_tree(crate=crate))
        self.assertEqual(1, len(highs["freeze-rocks"]))
        self.assertIn("rocksdb", highs["freeze-rocks"][0].message)

    def test_comment_only_rocksdb_mention_passes(self) -> None:
        crate = _CLEAN_CRATE_TOML + '[dependencies]\n# rocksdb = "0.22"\n'
        highs = _freeze_highs(_freeze_tree(crate=crate))
        self.assertEqual([], highs["freeze-rocks"])
        self.assertEqual([], highs["freeze-cypher-deps"])
        self.assertEqual([], highs["freeze-hnsw-dep"])

    def test_live_repo_optional_rocksdb_passes(self) -> None:
        highs = _freeze_highs(ROOT)
        self.assertEqual({check_id: [] for check_id in _FREEZE_IDS}, highs)

    def test_cypher_name_in_description_is_not_a_dep(self) -> None:
        crate = '[package]\nname = "x"\nversion = "0.0.0"\ndescription = "neo4j notes"\n'
        highs = _freeze_highs(_freeze_tree(crate=crate))
        self.assertEqual([], highs["freeze-cypher-deps"])

    def test_workspace_optional_rocksdb_is_still_forbidden(self) -> None:
        workspace = _workspace_deps('rocksdb = { version = "0.22", optional = true }\n')
        highs = _freeze_highs(_freeze_tree(workspace=workspace))
        self.assertEqual(1, len(highs["freeze-rocks"]))

    def test_unparsable_cargo_toml_is_high(self) -> None:
        result = _rust_run(
            _rust_tree({"crates/k/Cargo.toml": "[dependencies\nrocksdb = true\n"}),
            {
                "kind": "cargo_dep_constraint",
                "glob": "crates/*/Cargo.toml",
                "rules": {"rocksdb": "require_optional"},
                "category": "frozen-dep",
            },
        )
        self.assertEqual(["frozen-dep"], _rust_highs(result))

    def test_missing_glob_match_is_not_high(self) -> None:
        result = _rust_run(
            _rust_tree({"README.md": "x\n"}),
            {
                "kind": "cargo_dep_constraint",
                "glob": "crates/*/Cargo.toml",
                "rules": {"rocksdb": "require_optional"},
            },
        )
        self.assertEqual([], _rust_highs(result))


class GlobAbsentUnlessTests(unittest.TestCase):
    def test_unless_file_contains_skips_when_needle_present(self) -> None:
        root = _rust_tree(
            {
                "crates/k/Cargo.toml": 'rocksdb = "0.22"\n',
                "STATE.md": "thawed\n",
            }
        )
        result = _rust_run(
            root,
            {
                "kind": "glob_absent",
                "glob": "crates/*/Cargo.toml",
                "needles": ["rocksdb"],
                "unless_file_contains": {"file": "STATE.md", "needle": "thawed"},
            },
        )
        self.assertEqual([], _rust_highs(result))

    def test_unless_file_contains_missing_file_does_not_skip(self) -> None:
        root = _rust_tree({"crates/k/Cargo.toml": 'rocksdb = "0.22"\n'})
        result = _rust_run(
            root,
            {
                "kind": "glob_absent",
                "glob": "crates/*/Cargo.toml",
                "needles": ["rocksdb"],
                "unless_file_contains": {"file": "STATE.md", "needle": "thawed"},
            },
        )
        self.assertEqual(["glob-absent"], _rust_highs(result))

    def test_hash_comment_only_needle_passes_without_unless_line(self) -> None:
        # CE #22: comment exemption must not depend on unless_line_contains.
        root = _rust_tree({"notes.md": "# forbidden-token here\n"})
        result = _rust_run(
            root,
            {
                "kind": "glob_absent",
                "glob": "notes.md",
                "needles": ["forbidden-token"],
            },
        )
        self.assertEqual([], _rust_highs(result))

    def test_slash_comment_only_needle_passes_without_unless_line(self) -> None:
        root = _rust_tree({"src/x.rs": "// forbidden-token here\n"})
        result = _rust_run(
            root,
            {
                "kind": "glob_absent",
                "glob": "src/*.rs",
                "needles": ["forbidden-token"],
            },
        )
        self.assertEqual([], _rust_highs(result))

    def test_code_line_needle_high_without_unless_line(self) -> None:
        root = _rust_tree({"src/x.rs": "let forbidden-token = 1;\n"})
        result = _rust_run(
            root,
            {
                "kind": "glob_absent",
                "glob": "src/*.rs",
                "needles": ["forbidden-token"],
            },
        )
        self.assertEqual(["glob-absent"], _rust_highs(result))

    def test_hash_comment_only_still_passes_with_unless_line(self) -> None:
        root = _rust_tree({"notes.md": "# forbidden-token here\n"})
        result = _rust_run(
            root,
            {
                "kind": "glob_absent",
                "glob": "notes.md",
                "needles": ["forbidden-token"],
                "unless_line_contains": "allow-this",
            },
        )
        self.assertEqual([], _rust_highs(result))


class GlobRegexAbsentTests(unittest.TestCase):
    def test_join_tmp_literal_high(self) -> None:
        root = _rust_tree(
            {
                "crates/k/src/lib.rs": (
                    'PathBuf::from("x")\n        .join("tmp")\n        .join("fixed")\n'
                ),
            }
        )
        result = _rust_run(
            root,
            {
                "kind": "glob_regex_absent",
                "glob": "crates/**/*.rs",
                "patterns": [r'\.join\("tmp"\)\s*\.join\("'],
            },
        )
        self.assertEqual(["glob-regex-absent"], _rust_highs(result))

    def test_join_tmp_format_passes(self) -> None:
        root = _rust_tree(
            {
                "crates/k/src/lib.rs": (
                    'PathBuf::from("x")\n        .join("tmp")\n'
                    '        .join(format!("leaf-{n}"))\n'
                ),
            }
        )
        result = _rust_run(
            root,
            {
                "kind": "glob_regex_absent",
                "glob": "crates/**/*.rs",
                "patterns": [r'\.join\("tmp"\)\s*\.join\("'],
            },
        )
        self.assertEqual([], _rust_highs(result))

    def test_invalid_regex_high(self) -> None:
        root = _rust_tree({"crates/k/src/lib.rs": "fn ok() {}\n"})
        result = _rust_run(
            root,
            {
                "kind": "glob_regex_absent",
                "glob": "crates/**/*.rs",
                "patterns": [r"[unclosed"],
            },
        )
        self.assertEqual(["bad-regex"], _rust_highs(result))


class RustTestAssertsTests(unittest.TestCase):
    def test_rust_test_asserts_high_when_fn_missing(self) -> None:
        root = _rust_tree({"crates/k/tests/t.rs": "fn other() { assert!(true); }\n"})
        result = _rust_run(
            root,
            {
                "kind": "rust_test_asserts",
                "path": "crates/k/tests/t.rs",
                "tests": ["the_named_test"],
            },
        )
        self.assertEqual(["rust-test-missing"], _rust_highs(result))

    def test_rust_test_asserts_high_when_ignored(self) -> None:
        root = _rust_tree(
            {
                "crates/k/tests/t.rs": (
                    "#[cfg(test)]\n#[ignore]\n#[test]\nfn ignored_one() { assert!(true); }\n"
                )
            }
        )
        result = _rust_run(
            root,
            {
                "kind": "rust_test_asserts",
                "path": "crates/k/tests/t.rs",
                "tests": ["ignored_one"],
            },
        )
        self.assertEqual(["rust-test-ignored"], _rust_highs(result))

    def test_rust_test_asserts_high_when_no_assert_in_body(self) -> None:
        root = _rust_tree({"crates/k/tests/t.rs": "#[test]\nfn empty_body() { let _x = 1; }\n"})
        result = _rust_run(
            root,
            {
                "kind": "rust_test_asserts",
                "path": "crates/k/tests/t.rs",
                "tests": ["empty_body"],
            },
        )
        self.assertEqual(["rust-test-vacuous"], _rust_highs(result))

    def test_rust_test_asserts_high_when_assert_only_in_comment_or_string(self) -> None:
        src = (
            "#[test]\n"
            "fn commented() {\n"
            "    // assert!(true);\n"
            '    let _s = "assert_eq!(1, 1)";\n'
            '    let _r = r#"assert_ne!(1, 2)"#;\n'
            "}\n"
        )
        root = _rust_tree({"crates/k/tests/t.rs": src})
        result = _rust_run(
            root,
            {
                "kind": "rust_test_asserts",
                "path": "crates/k/tests/t.rs",
                "tests": ["commented"],
            },
        )
        self.assertEqual(["rust-test-vacuous"], _rust_highs(result))

    def test_rust_test_asserts_brace_inside_string_does_not_break_body(self) -> None:
        src = (
            "#[cfg(test)]\n"
            "#[test]\n"
            "fn braces() {\n"
            '    let _s = r#"{ not the body assert!(false); }"#;\n'
            "    let _c = '{';\n"
            "    let _q = '\"';\n"
            "    let _u = '\\u{7b}';\n"
            "    assert!(true);\n"
            "}\n"
        )
        root = _rust_tree({"crates/k/tests/t.rs": src})
        result = _rust_run(
            root,
            {
                "kind": "rust_test_asserts",
                "path": "crates/k/tests/t.rs",
                "tests": ["braces"],
            },
        )
        self.assertEqual([], _rust_highs(result))

    def test_rust_test_asserts_helper_prefix_counts(self) -> None:
        src = (
            "fn assert_graph_ok(ok: bool) { let _ = ok; }\n"
            "#[test]\n"
            "fn uses_helper() { assert_graph_ok(true); }\n"
        )
        root = _rust_tree({"crates/k/tests/t.rs": src})
        result = _rust_run(
            root,
            {
                "kind": "rust_test_asserts",
                "path": "crates/k/tests/t.rs",
                "tests": ["uses_helper"],
            },
        )
        self.assertEqual([], _rust_highs(result))

    def test_rust_test_asserts_high_when_required_symbol_missing(self) -> None:
        root = _rust_tree({"crates/k/tests/t.rs": "#[test]\nfn has_assert() { assert!(true); }\n"})
        result = _rust_run(
            root,
            {
                "kind": "rust_test_asserts",
                "path": "crates/k/tests/t.rs",
                "tests": ["has_assert"],
                "require_symbols": {"has_assert": ["DoesNotExist"]},
            },
        )
        self.assertEqual(["rust-test-symbol"], _rust_highs(result))

    def test_rust_test_asserts_high_on_async_and_nested(self) -> None:
        src = (
            "#[test]\n"
            "async fn async_one() { assert!(true); }\n"
            "#[test]\n"
            "fn outer() {\n"
            "    fn nested_one() { assert!(true); }\n"
            "    assert!(true);\n"
            "}\n"
        )
        root = _rust_tree({"crates/k/tests/t.rs": src})
        async_result = _rust_run(
            root,
            {
                "kind": "rust_test_asserts",
                "path": "crates/k/tests/t.rs",
                "tests": ["async_one"],
            },
        )
        nested_result = _rust_run(
            root,
            {
                "kind": "rust_test_asserts",
                "path": "crates/k/tests/t.rs",
                "tests": ["nested_one"],
            },
        )
        self.assertEqual(["rust-test-async"], _rust_highs(async_result))
        self.assertEqual(["rust-test-nested"], _rust_highs(nested_result))

    def test_rust_test_strip_noise_blanks_unicode_char_braces(self) -> None:
        stripped = strip_noise("let c = '\\u{7b}'; { assert!(true); }")
        self.assertNotIn("{7b}", stripped.replace(" ", ""))
        self.assertIn("assert!(true)", stripped)

    def test_rust_test_first_assert_macro_span_skips_string_noise(self) -> None:
        src = '#[test]\nfn t() {\n    let _s = "assert!(false)";\n    assert_eq!(1, 1);\n}\n'
        span = first_assert_macro_span(src, "t")
        self.assertIsNotNone(span)
        assert span is not None
        self.assertEqual("assert_eq!(", src[span[0] : span[1]])

    def test_rust_test_parse_fns_reads_attribute_block(self) -> None:
        src = "#[cfg(test)]\n#[test]\nfn named() { assert!(true); }\n"
        items = parse_fns(src)
        self.assertEqual(1, len(items))
        self.assertTrue(items[0].has_test)
        self.assertFalse(items[0].has_ignore)

    def test_h4_membership_as_of_is_non_vacuous(self) -> None:
        self.assertEqual(
            0, main(["--root", str(ROOT), "precommit", "--check", "h4-membership-as-of"])
        )

    def test_rust_test_names_from_yaml_empty_named_evidence_is_high(self) -> None:
        root = _rust_tree(
            {
                ".kutha/dictionaries/honeycomb.yaml": (
                    "cells:\n  - id: ADR-X\n    capability: named\n    evidence: []\n"
                ),
                "crates/k/tests/t.rs": "#[test]\nfn dummy() { assert!(true); }\n",
            }
        )
        result = _rust_run(
            root,
            {
                "kind": "rust_test_asserts",
                "names_from_yaml": {
                    "path": ".kutha/dictionaries/honeycomb.yaml",
                    "select": "cells",
                    "field": "evidence",
                    "glob": "crates/**/*.rs",
                },
                "require_evidence_when": {"field": "capability", "equals": "named"},
            },
        )
        self.assertIn("rust-test-evidence", _rust_highs(result))

    def test_rust_test_names_from_yaml_high_when_evidence_fn_missing(self) -> None:
        root = _rust_tree(
            {
                ".kutha/dictionaries/honeycomb.yaml": (
                    "cells:\n  - id: ADR-X\n    capability: named\n    evidence: [no_such_test]\n"
                ),
                "crates/k/tests/t.rs": "#[test]\nfn dummy() { assert!(true); }\n",
            }
        )
        result = _rust_run(
            root,
            {
                "kind": "rust_test_asserts",
                "names_from_yaml": {
                    "path": ".kutha/dictionaries/honeycomb.yaml",
                    "select": "cells",
                    "field": "evidence",
                    "glob": "crates/**/*.rs",
                },
                "require_evidence_when": {"field": "capability", "equals": "named"},
            },
        )
        self.assertIn("rust-test-missing", _rust_highs(result))

    def test_rust_fn_index_scratch_copy_does_not_see_original_cache(self) -> None:
        step: dict[str, object] = {
            "kind": "rust_test_asserts",
            "path": "crates/k/tests/t.rs",
            "tests": ["cache_probe"],
        }
        original = _rust_tree(
            {"crates/k/tests/t.rs": "#[test]\nfn cache_probe() { assert!(true); }\n"}
        )
        self.assertEqual([], _rust_highs(_rust_run(original, step)))
        scratch = _rust_tree({"crates/k/tests/t.rs": "#[test]\nfn cache_probe() { let _x = 1; }\n"})
        self.assertEqual(["rust-test-vacuous"], _rust_highs(_rust_run(scratch, step)))

    def test_rust_fn_index_invalidates_when_same_root_file_changes(self) -> None:
        root = _rust_tree({"crates/k/tests/t.rs": "#[test]\nfn cache_probe() { assert!(true); }\n"})
        step: dict[str, object] = {
            "kind": "rust_test_asserts",
            "path": "crates/k/tests/t.rs",
            "tests": ["cache_probe"],
        }
        self.assertEqual([], _rust_highs(_rust_run(root, step)))
        (root / "crates/k/tests/t.rs").write_text(
            "#[test]\nfn cache_probe() { let _x = 1; }\n",
            encoding="utf-8",
        )
        self.assertEqual(["rust-test-vacuous"], _rust_highs(_rust_run(root, step)))


_CITE_TOKEN = r"L_delivery=([A-Za-z0-9][A-Za-z0-9._-]*)"
_CITE_SOURCE = r"(?m)^L_delivery=([A-Za-z0-9][A-Za-z0-9._-]*)\s*$"


def _cite_step(
    source_text: str,
    cite_text: str,
    *,
    source_pattern: str = _CITE_SOURCE,
    cite_pattern: str = _CITE_TOKEN,
) -> tuple[Path, dict[str, object]]:
    root = _rust_tree({".kutha/STATE.md": source_text, "docs/note.md": cite_text})
    step: dict[str, object] = {
        "kind": "cite_equals",
        "source": {"path": ".kutha/STATE.md", "pattern": source_pattern},
        "cites": [{"path": "docs/note.md", "pattern": cite_pattern}],
    }
    return root, step


class CiteEqualsTests(unittest.TestCase):
    def test_cite_equals_high_when_capture_mismatches(self) -> None:
        root, step = _cite_step("L_delivery=FOO\n", "L_delivery=BAR\n")
        result = _rust_run(root, step)
        self.assertEqual(["cite-mismatch"], _rust_highs(result))

    def test_cite_equals_zero_cites_passes(self) -> None:
        root, step = _cite_step(
            "L_delivery=FOO\n",
            "Read `.kutha/STATE.md` first. No assignment token here.\n",
        )
        result = _rust_run(root, step)
        self.assertEqual([], _rust_highs(result))

    def test_cite_equals_multi_capture_one_stale_is_high(self) -> None:
        root, step = _cite_step(
            "L_delivery=M011-S08-done\n",
            "L_delivery=M011-S08-done\nAlso L_delivery=M011-S03-done in the same file.\n",
        )
        result = _rust_run(root, step)
        self.assertEqual(["cite-mismatch"], _rust_highs(result))
        self.assertTrue(any("M011-S03-done" in f.message for f in result.findings))

    def test_cite_equals_source_with_no_match_is_high(self) -> None:
        root, step = _cite_step(
            "lease lives here but the assignment line is missing\n",
            "L_delivery=FOO\n",
        )
        result = _rust_run(root, step)
        self.assertEqual(["cite-source"], _rust_highs(result))

    def test_cite_equals_stale_s03_token_even_in_backticks_is_high(self) -> None:
        root, step = _cite_step(
            "L_delivery=M011-S08-done\n",
            "The live lease is `L_delivery=M011-S03-done`.\n",
        )
        result = _rust_run(root, step)
        self.assertEqual(["cite-mismatch"], _rust_highs(result))

    def test_cite_equals_placeholder_and_empty_assignment_are_not_cites(self) -> None:
        root, step = _cite_step(
            "L_delivery=M011-S08-done\n",
            "Format is `L_delivery=<lease>` or a bare `L_delivery=` key. Read STATE.md.\n",
        )
        result = _rust_run(root, step)
        self.assertEqual([], _rust_highs(result))

    def test_cite_equals_pattern_must_have_one_group(self) -> None:
        root, step = _cite_step(
            "L_delivery=FOO\n",
            "L_delivery=FOO\n",
            source_pattern=r"L_delivery=\S+",
        )
        result = _rust_run(root, step)
        self.assertEqual(["cite-pattern"], _rust_highs(result))

    def test_cite_equals_live_check_excludes_changelog_and_planning(self) -> None:
        import yaml

        raw = yaml.safe_load(
            (ROOT / ".kutha" / "dictionaries" / "checks.yaml").read_text(encoding="utf-8")
        )
        rows = raw.get("checks") if isinstance(raw, dict) else []
        if not isinstance(rows, list):
            rows = []
        cite = next(row for row in rows if isinstance(row, dict) and row.get("id") == "cite-lease")
        blob = yaml.safe_dump(cite.get("steps"))
        self.assertNotIn("CHANGELOG.md", blob)
        # .planning/ is out of scope except PROJECT.md (GOV-02).
        self.assertIn(".planning/PROJECT.md", blob)
        self.assertNotIn(".planning/STATE.md", blob)
        self.assertNotIn(".planning/ROADMAP.md", blob)
        self.assertNotIn(".planning/phases", blob)

    def test_cite_equals_live_precommit_is_green(self) -> None:
        self.assertEqual(0, main(["--root", str(ROOT), "precommit", "--check", "cite-lease"]))

    def test_cite_equals_active_milestone_stale_is_high(self) -> None:
        root = _rust_tree(
            {
                ".kutha/STATE.md": "**Active Milestone:** M011\n",
                "docs/note.md": "Active Milestone: M010 is the lease.\n",
            }
        )
        result = _rust_run(
            root,
            {
                "kind": "cite_equals",
                "source": {
                    "path": ".kutha/STATE.md",
                    "pattern": r"(?m)^\*\*Active Milestone:\*\*\s*(M\d{3}|None)\s*$",
                },
                "cites": [
                    {
                        "path": "docs/note.md",
                        "pattern": (
                            r"Active Milestone(?:\*\*)?(?:\s+is|\s+stays)?"
                            r"(?::\*\*|:)?\s+\**?(M\d{3})\b"
                        ),
                    }
                ],
            },
        )
        self.assertEqual(["cite-mismatch"], _rust_highs(result))

    def test_cite_equals_active_milestone_names_m002_is_not_a_cite(self) -> None:
        root = _rust_tree(
            {
                ".kutha/STATE.md": "**Active Milestone:** M011\n",
                "docs/note.md": "Do not start Rocks until Active Milestone names M002.\n",
            }
        )
        result = _rust_run(
            root,
            {
                "kind": "cite_equals",
                "source": {
                    "path": ".kutha/STATE.md",
                    "pattern": r"(?m)^\*\*Active Milestone:\*\*\s*(M\d{3}|None)\s*$",
                },
                "cites": [
                    {
                        "path": "docs/note.md",
                        "pattern": (
                            r"Active Milestone(?:\*\*)?(?:\s+is|\s+stays)?"
                            r"(?::\*\*|:)?\s+\**?(M\d{3})\b"
                        ),
                    }
                ],
            },
        )
        self.assertEqual([], _rust_highs(result))


def _refs_step(**extra: object) -> dict[str, object]:
    step: dict[str, object] = {
        "kind": "refs_resolve",
        "paths": ["docs/note.md"],
        "globs": [],
    }
    step.update(extra)
    return step


class RefsResolveTests(unittest.TestCase):
    def test_refs_resolve_high_on_dangling_adr_999(self) -> None:
        root = _rust_tree(
            {
                "docs/note.md": "See ADR-999.\n",
                ".kutha/dictionaries/honeycomb.yaml": "locks:\n  - id: D1\n",
            }
        )
        result = _rust_run(root, _refs_step())
        self.assertEqual(["refs-adr"], _rust_highs(result))

    def test_refs_resolve_allows_adr_100_when_listed(self) -> None:
        root = _rust_tree(
            {
                "docs/note.md": "Do not open ADR-100.\n",
                ".kutha/dictionaries/honeycomb.yaml": "locks:\n  - id: D1\n",
            }
        )
        result = _rust_run(root, _refs_step(allow_dangling=["ADR-100"]))
        self.assertEqual([], _rust_highs(result))

    def test_refs_resolve_ignores_adr_xxx_placeholder(self) -> None:
        root = _rust_tree(
            {
                "docs/note.md": "A future cell is ADR-XXX until numbered.\n",
                ".kutha/dictionaries/honeycomb.yaml": "locks:\n  - id: D1\n",
            }
        )
        result = _rust_run(root, _refs_step())
        self.assertEqual([], _rust_highs(result))

    def test_refs_resolve_d050_2_and_d010_1_are_not_lock_ids(self) -> None:
        root = _rust_tree(
            {
                "docs/note.md": "Analog of ADR-050 D050-2 and D010-1.\n",
                "docs/ADR/ADR-050-meta.md": "# ADR-050\n",
                ".kutha/dictionaries/honeycomb.yaml": "locks:\n  - id: D1\n",
            }
        )
        result = _rust_run(root, _refs_step())
        self.assertEqual([], _rust_highs(result))

    def test_refs_resolve_range_endpoints_only(self) -> None:
        root = _rust_tree(
            {
                "docs/note.md": "Honeycomb cells (ADR-010\u2013093, all Proposed).\n",
                "docs/ADR/ADR-010-event.md": "# ten\n",
                "docs/ADR/ADR-093-science.md": "# ninety-three\n",
                ".kutha/dictionaries/honeycomb.yaml": "locks:\n  - id: D1\n",
            }
        )
        result = _rust_run(root, _refs_step())
        self.assertEqual([], _rust_highs(result))

    def test_refs_resolve_range_missing_endpoint_is_high(self) -> None:
        root = _rust_tree(
            {
                "docs/note.md": "See ADR-010-093.\n",
                "docs/ADR/ADR-010-event.md": "# ten\n",
                ".kutha/dictionaries/honeycomb.yaml": "locks:\n  - id: D1\n",
            }
        )
        result = _rust_run(root, _refs_step())
        self.assertEqual(["refs-adr"], _rust_highs(result))
        self.assertTrue(any("ADR-093" in f.message for f in result.findings))

    def test_refs_resolve_four_digit_neighbor_is_ignored(self) -> None:
        root = _rust_tree(
            {
                "docs/note.md": "neighbors' ADR-0007 lesson\n",
                ".kutha/dictionaries/honeycomb.yaml": "locks:\n  - id: D1\n",
            }
        )
        result = _rust_run(root, _refs_step())
        self.assertEqual([], _rust_highs(result))

    def test_refs_resolve_unknown_lock_is_high(self) -> None:
        root = _rust_tree(
            {
                "docs/note.md": "Honor D2 forever.\n",
                ".kutha/dictionaries/honeycomb.yaml": "locks:\n  - id: D1\n",
            }
        )
        result = _rust_run(root, _refs_step())
        self.assertEqual(["refs-lock"], _rust_highs(result))

    def test_refs_resolve_live_precommit_is_green(self) -> None:
        self.assertEqual(0, main(["--root", str(ROOT), "precommit", "--check", "refs-resolve"]))


class FileMaxLinesTests(unittest.TestCase):
    def test_file_max_lines_high_when_tempfile_exceeds_max(self) -> None:
        root = _rust_tree({"AGENTS.md": "a\nb\nc\n"})
        result = _rust_run(root, {"kind": "file_max_lines", "path": "AGENTS.md", "max": 2})
        self.assertEqual(["max-lines"], _rust_highs(result))

    def test_file_max_lines_at_budget_passes(self) -> None:
        root = _rust_tree({"AGENTS.md": "a\nb\n"})
        result = _rust_run(root, {"kind": "file_max_lines", "path": "AGENTS.md", "max": 2})
        self.assertEqual([], _rust_highs(result))

    def test_agents_lean_append_lines_over_budget_is_high(self) -> None:
        text = (ROOT / "AGENTS.md").read_text(encoding="utf-8")
        root = _rust_tree({"AGENTS.md": text + ("\n" * 20)})
        result = _rust_run(root, {"kind": "file_max_lines", "path": "AGENTS.md", "max": 110})
        self.assertEqual(["max-lines"], _rust_highs(result))

    def test_agents_lean_append_delivery_token_is_high(self) -> None:
        text = (ROOT / "AGENTS.md").read_text(encoding="utf-8")
        root = _rust_tree({"AGENTS.md": text + "\nL_delivery=WRONG\n"})
        result = _rust_run(
            root,
            {
                "kind": "file_absent",
                "path": "AGENTS.md",
                "needles": ["L_delivery=", "L_map=", "L_capability=", "M011 is active"],
            },
        )
        self.assertEqual(["absent"], _rust_highs(result))

    def test_agents_lean_live_precommit_is_green(self) -> None:
        self.assertEqual(0, main(["--root", str(ROOT), "precommit", "--check", "agents-lean"]))
        agents = (ROOT / "AGENTS.md").read_text(encoding="utf-8")
        self.assertLessEqual(agents.count("\n"), 160)


class GlobMaxLinesTests(unittest.TestCase):
    def test_glob_max_lines_high_when_tempfile_exceeds_max(self) -> None:
        root = _rust_tree({".planning/phases/22-x/22-PLAN.md": "a\nb\nc\n"})
        result = _rust_run(
            root,
            {
                "kind": "glob_max_lines",
                "glob": ".planning/phases/*-*/*-PLAN.md",
                "max": 2,
            },
        )
        self.assertEqual(["max-lines"], _rust_highs(result))

    def test_glob_max_lines_at_budget_passes(self) -> None:
        root = _rust_tree({".planning/phases/22-x/22-PLAN.md": "a\nb\n"})
        result = _rust_run(
            root,
            {
                "kind": "glob_max_lines",
                "glob": ".planning/phases/*-*/*-PLAN.md",
                "max": 2,
            },
        )
        self.assertEqual([], _rust_highs(result))

    def test_glob_max_lines_high_when_too_many_files_per_dir(self) -> None:
        root = _rust_tree(
            {
                ".planning/phases/22-x/22-01-PLAN.md": "a\n",
                ".planning/phases/22-x/22-02-PLAN.md": "a\n",
                ".planning/phases/22-x/22-03-PLAN.md": "a\n",
            }
        )
        result = _rust_run(
            root,
            {
                "kind": "glob_max_lines",
                "glob": ".planning/phases/*-*/*-PLAN.md",
                "max": 150,
                "max_files_per_dir": 2,
                "min_phase": 22,
            },
        )
        self.assertEqual(["max-files"], _rust_highs(result))

    def test_glob_max_lines_ignores_phase_below_min_phase(self) -> None:
        root = _rust_tree(
            {
                ".planning/phases/21-old/21-PLAN.md": "a\nb\nc\n",
                ".planning/phases/22-new/22-PLAN.md": "a\n",
            }
        )
        result = _rust_run(
            root,
            {
                "kind": "glob_max_lines",
                "glob": ".planning/phases/*-*/*-PLAN.md",
                "max": 2,
                "min_phase": 22,
            },
        )
        self.assertEqual([], _rust_highs(result))

    def test_glob_max_lines_missing_glob_is_high(self) -> None:
        root = _rust_tree({".planning/phases/22-x/22-PLAN.md": "a\n"})
        result = _rust_run(root, {"kind": "glob_max_lines", "max": 2})
        self.assertEqual(["max-lines"], _rust_highs(result))

    def test_glob_max_lines_max_less_than_zero_is_high(self) -> None:
        root = _rust_tree({".planning/phases/22-x/22-PLAN.md": "a\n"})
        result = _rust_run(
            root,
            {"kind": "glob_max_lines", "glob": ".planning/phases/*-*/*-PLAN.md", "max": -1},
        )
        self.assertEqual(["max-lines"], _rust_highs(result))

    def test_glob_max_lines_unknown_kind_stays_high(self) -> None:
        root = _rust_tree({".planning/phases/22-x/22-PLAN.md": "a\n"})
        result = _rust_run(root, {"kind": "not_a_kind"})
        self.assertEqual(["unknown-kind"], _rust_highs(result))

    def test_glob_max_lines_verification_over_60_is_high(self) -> None:
        root = _rust_tree({".planning/phases/22-x/22-VERIFICATION.md": "a\n" * 61})
        result = _rust_run(
            root,
            {
                "kind": "glob_max_lines",
                "glob": ".planning/phases/*-*/*-VERIFICATION.md",
                "max": 60,
                "min_phase": 22,
                "max_files_per_dir": 1,
            },
        )
        self.assertEqual(["max-lines"], _rust_highs(result))

    def test_glob_max_lines_min_hits_zero_allows_empty_glob(self) -> None:
        root = _rust_tree({})
        result = _rust_run(
            root,
            {
                "kind": "glob_max_lines",
                "glob": ".planning/phases/*-*/*-PLAN.md",
                "max": 150,
                "min_phase": 22,
            },
        )
        self.assertEqual([], _rust_highs(result))

    def test_glob_max_lines_min_hits_one_empty_glob_is_high(self) -> None:
        root = _rust_tree({})
        result = _rust_run(
            root,
            {
                "kind": "glob_max_lines",
                "glob": ".planning/phases/*-*/*-PLAN.md",
                "max": 150,
                "min_phase": 22,
                "min_hits": 1,
            },
        )
        self.assertEqual(["min-hits"], _rust_highs(result))

    def test_glob_max_lines_when_glob_exists_idle_allows_empty(self) -> None:
        root = _rust_tree({})
        result = _rust_run(
            root,
            {
                "kind": "glob_max_lines",
                "glob": ".planning/phases/*-*/*-SUMMARY.md",
                "max": 60,
                "min_phase": 22,
                "min_hits": 1,
                "when_glob_exists": ".planning/phases/*-*/*-CONTEXT.md",
            },
        )
        self.assertEqual([], _rust_highs(result))

    def test_glob_max_lines_when_glob_exists_live_empty_summary_is_high(self) -> None:
        root = _rust_tree({".planning/phases/22-x/22-CONTEXT.md": "ctx\n"})
        result = _rust_run(
            root,
            {
                "kind": "glob_max_lines",
                "glob": ".planning/phases/*-*/*-SUMMARY.md",
                "max": 60,
                "min_phase": 22,
                "min_hits": 1,
                "when_glob_exists": ".planning/phases/*-*/*-CONTEXT.md",
            },
        )
        self.assertEqual(["min-hits"], _rust_highs(result))

    def test_glob_max_lines_summary_two_files_per_dir_is_high(self) -> None:
        root = _rust_tree(
            {
                ".planning/phases/22-x/22-01-SUMMARY.md": "a\n",
                ".planning/phases/22-x/22-02-SUMMARY.md": "a\n",
            }
        )
        result = _rust_run(
            root,
            {
                "kind": "glob_max_lines",
                "glob": ".planning/phases/*-*/*-SUMMARY.md",
                "max": 60,
                "min_phase": 22,
                "max_files_per_dir": 1,
            },
        )
        self.assertEqual(["max-files"], _rust_highs(result))


class VerificationIndependentTests(unittest.TestCase):
    def test_verification_independent_glob_absent_high_on_plan_path(self) -> None:
        root = _rust_tree(
            {".planning/phases/22-x/22-VERIFICATION.md": "read_only:\n  - foo-PLAN.md\n"}
        )
        result = _rust_run(
            root,
            {
                "kind": "glob_absent",
                "glob": ".planning/phases/*-*/*-VERIFICATION.md",
                "needles": ["-PLAN.md", "-SUMMARY.md"],
            },
        )
        self.assertEqual(["glob-absent"], _rust_highs(result))

    def test_verification_independent_glob_absent_prose_passes(self) -> None:
        root = _rust_tree(
            {".planning/phases/22-x/22-VERIFICATION.md": "no plan or summary files read\n"}
        )
        result = _rust_run(
            root,
            {
                "kind": "glob_absent",
                "glob": ".planning/phases/*-*/*-VERIFICATION.md",
                "needles": ["-PLAN.md", "-SUMMARY.md"],
            },
        )
        self.assertEqual([], _rust_highs(result))

    def test_verification_independent_file_contains_high_without_independent(self) -> None:
        root = _rust_tree(
            {
                ".planning/phases/22-x/22-VERIFICATION.md": (
                    "verifier_model: x\ncommands:\nread_only_inputs:\nexit: 0\n"
                )
            }
        )
        result = _rust_run(
            root,
            {
                "kind": "file_contains",
                "path": ".planning/phases/22-x/22-VERIFICATION.md",
                "needles": [
                    "independent: true",
                    "verifier_model",
                    "commands:",
                    "read_only_inputs:",
                    "exit:",
                ],
                "require": "all",
            },
        )
        self.assertEqual(["contains"], _rust_highs(result))

    def test_verification_independent_glob_file_contains_zero_hits_high(self) -> None:
        root = _rust_tree({})
        result = _rust_run(
            root,
            {
                "kind": "file_contains",
                "glob": ".planning/phases/*-*/*-VERIFICATION.md",
                "min_phase": 22,
                "needles": ["independent: true"],
                "require": "all",
            },
        )
        self.assertEqual(["glob-empty"], _rust_highs(result))

    def test_verification_context_without_verification_is_high(self) -> None:
        # CE review P1 #16: live CONTEXT + missing VERIFICATION must not vacuous-pass.
        root = _rust_tree({".planning/phases/22-x/22-CONTEXT.md": "ctx\n"})
        result = _rust_run(
            root,
            {
                "kind": "file_contains",
                "glob": ".planning/phases/*-*/*-VERIFICATION.md",
                "min_phase": 22,
                "when_glob_exists": ".planning/phases/*-*/*-CONTEXT.md",
                "needles": ["independent: true"],
                "require": "all",
            },
        )
        self.assertEqual(["glob-empty"], _rust_highs(result))

    def test_verification_idle_without_context_skips(self) -> None:
        root = _rust_tree({})
        result = _rust_run(
            root,
            {
                "kind": "file_contains",
                "glob": ".planning/phases/*-*/*-VERIFICATION.md",
                "min_phase": 22,
                "when_glob_exists": ".planning/phases/*-*/*-CONTEXT.md",
                "needles": ["independent: true"],
                "require": "all",
            },
        )
        self.assertEqual([], _rust_highs(result))

    def test_frontmatter_context_without_verification_is_high(self) -> None:
        root = _rust_tree({".planning/phases/22-x/22-CONTEXT.md": "ctx\n"})
        result = _rust_run(
            root,
            {
                "kind": "frontmatter_paths_exist",
                "glob": ".planning/phases/*-*/*-VERIFICATION.md",
                "min_phase": 22,
                "when_glob_exists": ".planning/phases/*-*/*-CONTEXT.md",
                "field": "read_only_inputs",
            },
        )
        self.assertEqual(["glob-empty"], _rust_highs(result))

    def test_frontmatter_paths_exist_high_on_missing_input(self) -> None:
        body = "---\nindependent: true\nread_only_inputs:\n  - docs/missing.md\n---\n# verify\n"
        root = _rust_tree({".planning/phases/22-x/22-VERIFICATION.md": body})
        result = _rust_run(
            root,
            {
                "kind": "frontmatter_paths_exist",
                "glob": ".planning/phases/*-*/*-VERIFICATION.md",
                "min_phase": 22,
                "field": "read_only_inputs",
            },
        )
        self.assertEqual(["frontmatter-path"], _rust_highs(result))

    def test_frontmatter_paths_exist_passes_when_inputs_exist(self) -> None:
        body = (
            "---\n"
            "independent: true\n"
            "read_only_inputs:\n"
            "  - docs/process/governor-intake.md\n"
            "---\n"
            "# verify\n"
        )
        root = _rust_tree(
            {
                ".planning/phases/22-x/22-VERIFICATION.md": body,
                "docs/process/governor-intake.md": "# intake\n",
            }
        )
        result = _rust_run(
            root,
            {
                "kind": "frontmatter_paths_exist",
                "glob": ".planning/phases/*-*/*-VERIFICATION.md",
                "min_phase": 22,
            },
        )
        self.assertEqual([], _rust_highs(result))

    def test_frontmatter_paths_exist_high_on_parent_escape(self) -> None:
        body = "---\nindependent: true\nread_only_inputs:\n  - ../../etc/passwd\n---\n# verify\n"
        root = _rust_tree({".planning/phases/22-x/22-VERIFICATION.md": body})
        result = _rust_run(
            root,
            {
                "kind": "frontmatter_paths_exist",
                "glob": ".planning/phases/*-*/*-VERIFICATION.md",
                "min_phase": 22,
            },
        )
        self.assertEqual(["frontmatter-path"], _rust_highs(result))

    def test_frontmatter_paths_exist_high_on_empty_list(self) -> None:
        body = "---\nindependent: true\nread_only_inputs:\n---\n# verify\n"
        root = _rust_tree({".planning/phases/22-x/22-VERIFICATION.md": body})
        result = _rust_run(
            root,
            {
                "kind": "frontmatter_paths_exist",
                "glob": ".planning/phases/*-*/*-VERIFICATION.md",
                "min_phase": 22,
            },
        )
        self.assertEqual(["frontmatter"], _rust_highs(result))


_PLAN_BUDGET_STEP: dict[str, object] = {
    "kind": "glob_max_lines",
    "glob": ".planning/phases/*-*/*-PLAN.md",
    "min_phase": 22,
    "min_hits": 1,
    "when_glob_exists": ".planning/phases/*-*/*-CONTEXT.md",
    "correspond_glob": ".planning/phases/*-*/*-CONTEXT.md",
    "max": 150,
    "max_files_per_dir": 2,
}


def _artifact_budget_plan_step() -> dict[str, object]:
    loaded = yaml.safe_load(
        (ROOT / ".kutha" / "dictionaries" / "checks.yaml").read_text(encoding="utf-8")
    )
    row = next(c for c in loaded["checks"] if c.get("id") == "artifact-budget")
    return next(s for s in row["steps"] if str(s.get("glob", "")).endswith("PLAN.md"))


class ArtifactBudgetHonestyTests(unittest.TestCase):
    """CE #14: PLAN min_hits must not idle on the same empty PLAN glob."""

    def test_plan_step_gates_on_context_not_plan_glob(self) -> None:
        plan = _artifact_budget_plan_step()
        self.assertEqual(
            ".planning/phases/*-*/*-CONTEXT.md",
            plan.get("when_glob_exists"),
        )
        self.assertNotEqual(plan.get("when_glob_exists"), plan.get("glob"))
        self.assertEqual(
            ".planning/phases/*-*/*-CONTEXT.md",
            plan.get("correspond_glob"),
        )

    def test_artifact_budget_context_without_plan_is_high(self) -> None:
        root = _rust_tree(
            {
                ".planning/phases/22-x/22-CONTEXT.md": "ctx\n",
                ".planning/phases/22-x/22-SUMMARY.md": "sum\n",
                ".planning/phases/22-x/22-VERIFICATION.md": "ver\n",
            }
        )
        result = get_checks(ROOT)["artifact-budget"].run(Context(root=root))
        self.assertIn("min-hits", _rust_highs(result))

    def test_artifact_budget_peer_phase_without_plan_is_high(self) -> None:
        root = _rust_tree(
            {
                ".planning/phases/22-a/22-CONTEXT.md": "a\n",
                ".planning/phases/22-a/22-01-PLAN.md": "plan\n",
                ".planning/phases/22-a/22-SUMMARY.md": "sum\n",
                ".planning/phases/22-a/22-VERIFICATION.md": "ver\n",
                ".planning/phases/23-b/23-CONTEXT.md": "b\n",
            }
        )
        result = get_checks(ROOT)["artifact-budget"].run(Context(root=root))
        self.assertIn("correspond", _rust_highs(result))

    def test_glob_max_lines_correspond_open_context_without_plan_is_high(self) -> None:
        root = _rust_tree(
            {
                ".planning/phases/22-a/22-CONTEXT.md": "a\n",
                ".planning/phases/22-a/22-01-PLAN.md": "plan\n",
                ".planning/phases/23-b/23-CONTEXT.md": "b\n",
            }
        )
        result = _rust_run(root, _PLAN_BUDGET_STEP)
        self.assertIn("correspond", _rust_highs(result))

    def test_glob_max_lines_correspond_skip_closed_verification_without_plan(self) -> None:
        root = _rust_tree(
            {
                ".planning/phases/22-a/22-CONTEXT.md": "a\n",
                ".planning/phases/22-a/22-01-PLAN.md": "plan\n",
                ".planning/phases/22-a/22-SUMMARY.md": "sum\n",
                ".planning/phases/86-b/86-CONTEXT.md": "b\n",
                ".planning/phases/86-b/86-VERIFICATION.md": "ver\n",
            }
        )
        step = dict(_PLAN_BUDGET_STEP)
        step["correspond_skip_if"] = "*-VERIFICATION.md"
        result = _rust_run(root, step)
        self.assertEqual([], _rust_highs(result))

    def test_artifact_budget_idle_without_context_allows_empty_plan(self) -> None:
        root = _rust_tree({})
        result = get_checks(ROOT)["artifact-budget"].run(Context(root=root))
        self.assertEqual([], _rust_highs(result))

    def test_selftest_derives_delete_only_plan_files(self) -> None:
        root = _rust_tree(
            {
                ".planning/phases/22-x/22-CONTEXT.md": "ctx\n",
                ".planning/phases/22-x/22-01-PLAN.md": "plan\n",
            }
        )
        units = _derive_mutations([_PLAN_BUDGET_STEP], root)
        deletes = [
            m.get("path")
            for unit in units
            for m in unit
            if m.get("op") == "delete_file"
        ]
        self.assertTrue(
            any(str(path).endswith("-PLAN.md") for path in deletes),
            msg=f"expected PLAN delete_file mutation, got {units!r}",
        )
        self.assertFalse(
            any(str(path).endswith("-CONTEXT.md") for path in deletes),
            msg=f"CONTEXT must remain: {units!r}",
        )


class LeaseBundleHonestyTests(unittest.TestCase):
    """M056 S01: lease must keep trajectory/state-readme green (bundle)."""

    def test_contract_names_lease_bundle_surfaces(self) -> None:
        text = (ROOT / "docs" / "process" / "gsd-auto-contract.md").read_text(
            encoding="utf-8"
        )
        self.assertIn("Lease (bundle)", text)
        self.assertIn("ROADMAP.md", text)
        self.assertIn("README.md", text)
        self.assertIn("M056 lesson", text)

    def test_trajectory_require_message_cites_lease_bundle(self) -> None:
        loaded = yaml.safe_load(
            (ROOT / ".kutha" / "dictionaries" / "checks.yaml").read_text(
                encoding="utf-8"
            )
        )
        row = next(c for c in loaded["checks"] if c.get("id") == "trajectory")
        step = next(
            s
            for s in row["steps"]
            if s.get("kind") == "pointer_in_other_file"
            and "require_template" in s
        )
        msg = str(step.get("require_message", ""))
        self.assertIn("lease bundle", msg)
        self.assertIn("ROADMAP", msg)

    def test_leased_slice_without_open_checkbox_is_high(self) -> None:
        root = _rust_tree(
            {
                ".kutha/STATE.md": (
                    "**Active Milestone:** M056\n"
                    "**Active Slice:** S01\n"
                    "**Phase:** H5\n"
                ),
                ".kutha/ROADMAP.md": (
                    "## M056: x\n"
                    "- [x] **S01: done already**\n"
                ),
                "README.md": "M056\nH5\n",
            }
        )
        result = get_checks(ROOT)["trajectory"].run(Context(root=root))
        self.assertIn("lease-drift", _rust_highs(result))


class BudgetCoversChecksTests(unittest.TestCase):
    """M060 S01: fsm defaults.budget must cover dictionary check count."""

    def test_budget_covers_checks_is_registered(self) -> None:
        self.assertIn("budget-covers-checks", get_checks(ROOT))

    def test_short_fsm_budget_is_high(self) -> None:
        result = CheckResult(check="probe")
        run_step(
            "probe",
            {"kind": "budget_covers_checks", "budget": 1},
            Context(root=ROOT),
            result,
        )
        highs = [f for f in result.findings if f.severity is Severity.HIGH]
        self.assertEqual(1, len(highs))
        self.assertEqual("budget-covers", highs[0].category)

    def test_live_budget_covers_checks_is_green(self) -> None:
        result = get_checks(ROOT)["budget-covers-checks"].run(Context(root=ROOT))
        highs = [f for f in result.findings if f.severity is Severity.HIGH]
        self.assertEqual([], highs)


class EnvBudgetFloorTests(unittest.TestCase):
    """M061 S01: live KUTHA_GOV_BUDGET when set must be ≥ fsm defaults.budget."""

    def test_env_budget_floor_is_registered(self) -> None:
        self.assertIn("env-budget-floor", get_checks(ROOT))

    def test_short_env_value_is_high(self) -> None:
        result = CheckResult(check="probe")
        run_step(
            "probe",
            {
                "kind": "env_int_at_least",
                "env": "KUTHA_GOV_BUDGET",
                "at_least": 80,
                "value": 1,
            },
            Context(root=ROOT),
            result,
        )
        highs = [f for f in result.findings if f.severity is Severity.HIGH]
        self.assertEqual(1, len(highs))
        self.assertEqual("env-int-floor", highs[0].category)

    def test_unset_env_is_green(self) -> None:
        result = CheckResult(check="probe")
        run_step(
            "probe",
            {
                "kind": "env_int_at_least",
                "env": "KUTHA_GOV_BUDGET_UNSET_PROBE_M061",
                "at_least": 80,
            },
            Context(root=ROOT),
            result,
        )
        highs = [f for f in result.findings if f.severity is Severity.HIGH]
        self.assertEqual([], highs)

    def test_live_env_budget_floor_is_green(self) -> None:
        old = os.environ.pop("KUTHA_GOV_BUDGET", None)
        try:
            from kutha_gov.config import load_dotenv

            load_dotenv(ROOT)
            result = get_checks(ROOT)["env-budget-floor"].run(Context(root=ROOT))
            highs = [f for f in result.findings if f.severity is Severity.HIGH]
            self.assertEqual([], highs)
        finally:
            if old is None:
                os.environ.pop("KUTHA_GOV_BUDGET", None)
            else:
                os.environ["KUTHA_GOV_BUDGET"] = old


class BudgetTemplateAlignTests(unittest.TestCase):
    """M059 S01: .env.example budget must match fsm.yaml defaults.budget."""

    def test_budget_template_align_is_registered(self) -> None:
        self.assertIn("budget-template-align", get_checks(ROOT))

    def test_stale_env_example_budget_is_high(self) -> None:
        root = _rust_tree(
            {
                ".kutha/dictionaries/fsm.yaml": (
                    "schema: kutha-harness-fsm/v1\n"
                    "defaults:\n"
                    "  budget: 80\n"
                ),
                ".env.example": "KUTHA_GOV_BUDGET=79\n",
            }
        )
        result = get_checks(ROOT)["budget-template-align"].run(Context(root=root))
        highs = [f for f in result.findings if f.severity is Severity.HIGH]
        self.assertTrue(highs, msg="expected cite-mismatch HIGH")
        self.assertTrue(
            any("79" in f.message and "80" in f.message for f in highs),
            msg=repr([f.message for f in highs]),
        )

    def test_live_precommit_budget_template_align_is_green(self) -> None:
        result = get_checks(ROOT)["budget-template-align"].run(Context(root=ROOT))
        highs = [f for f in result.findings if f.severity is Severity.HIGH]
        self.assertEqual([], highs)


class DiskFreePreflightTests(unittest.TestCase):
    """M058 S01: disk_free kind fails when free space is below YAML threshold."""

    def test_absurd_min_free_gib_is_high(self) -> None:
        result = CheckResult(check="probe")
        run_step(
            "probe",
            {
                "kind": "disk_free",
                "path": ".",
                "min_free_gib": 999999,
            },
            Context(root=ROOT),
            result,
        )
        highs = [f for f in result.findings if f.severity is Severity.HIGH]
        self.assertEqual(1, len(highs))
        self.assertEqual("disk-free", highs[0].category)
        self.assertIn("min_free_gib", highs[0].message)

    def test_live_min_free_gib_one_passes(self) -> None:
        result = CheckResult(check="probe")
        run_step(
            "probe",
            {"kind": "disk_free", "path": ".", "min_free_gib": 1},
            Context(root=ROOT),
            result,
        )
        highs = [f for f in result.findings if f.severity is Severity.HIGH]
        self.assertEqual([], highs)

    def test_disk_free_check_is_registered(self) -> None:
        self.assertIn("disk-free", get_checks(ROOT))


class LeaseRoadmapCouplingTests(unittest.TestCase):
    """M057 S01: STATE lease diff must include ROADMAP (docs-coupling)."""

    def test_state_diff_without_roadmap_is_high(self) -> None:
        ctx = Context(
            root=ROOT,
            changed_paths=frozenset(
                {".kutha/STATE.md", "CHANGELOG.md", "README.md"}
            ),
        )
        result = get_checks(ROOT)["docs-coupling"].run(ctx)
        highs = [f.message for f in result.findings if f.severity is Severity.HIGH]
        self.assertTrue(
            any("ROADMAP" in m for m in highs),
            msg=f"expected ROADMAP coupling HIGH, got {highs!r}",
        )

    def test_state_diff_with_roadmap_passes_roadmap_rule(self) -> None:
        ctx = Context(
            root=ROOT,
            changed_paths=frozenset(
                {
                    ".kutha/STATE.md",
                    ".kutha/ROADMAP.md",
                    "CHANGELOG.md",
                    "README.md",
                }
            ),
        )
        result = get_checks(ROOT)["docs-coupling"].run(ctx)
        highs = [f.message for f in result.findings if f.severity is Severity.HIGH]
        self.assertFalse(any("ROADMAP" in m for m in highs), msg=highs)


if __name__ == "__main__":
    unittest.main()
