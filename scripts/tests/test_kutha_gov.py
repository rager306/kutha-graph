from __future__ import annotations

import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))

from kutha_gov.__main__ import main  # noqa: E402
from kutha_gov.checks import get_checks  # noqa: E402
from kutha_gov.kinds import run_step  # noqa: E402
from kutha_gov.protocol import CheckResult, Context, Severity  # noqa: E402
from kutha_gov.rust_source import first_assert_macro_span, parse_fns, strip_noise  # noqa: E402


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

    def test_freeze_message_does_not_cite_closed_m001_gate(self) -> None:
        yaml_text = (ROOT / ".kutha" / "dictionaries" / "checks.yaml").read_text(encoding="utf-8")
        self.assertNotIn("before M001 L_capability is green", yaml_text)
        self.assertIn("until Active Milestone names M002", yaml_text)


def _rust_tree(files: dict[str, str]) -> Path:
    raw = tempfile.mkdtemp(prefix="kutha-rust-test-")
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


if __name__ == "__main__":
    unittest.main()
