from __future__ import annotations

import atexit
import hashlib
import shutil
import subprocess
import sys
import tempfile
import unittest
from io import StringIO
from pathlib import Path
from unittest.mock import patch

import yaml

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))

from kutha_gov.__main__ import main  # noqa: E402
from kutha_gov.dictionary import DictCheck, get_checks  # noqa: E402
from kutha_gov.selftest import (  # noqa: E402
    BASELINE_FAIL,
    CLASS_DERIVED,
    CLASS_EXPLICIT_MUTATION,
    CLASS_EXPLICIT_SKIP,
    CLASS_FILE_EXISTS_ONLY,
    CLASS_UNPROVEN,
    SKIP,
    UNPROVEN,
    VACUOUS,
    classify_check,
    copy_harness_tree,
    format_row,
    run_selftest,
)


def _write_mini(checks: list[dict[str, object]], files: dict[str, str] | None = None) -> Path:
    raw = tempfile.mkdtemp(prefix="kutha-selftest-mini-")
    atexit.register(shutil.rmtree, raw, ignore_errors=True)
    root = Path(raw)
    (root / ".kutha" / "dictionaries").mkdir(parents=True)
    (root / "docs" / "ADR").mkdir(parents=True)
    payload = {"schema": "kutha-harness-checks/v1", "checks": checks}
    (root / ".kutha" / "dictionaries" / "checks.yaml").write_text(
        yaml.safe_dump(payload, sort_keys=False),
        encoding="utf-8",
    )
    for rel, text in (files or {}).items():
        path = root / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
    return root


def _tracked_harness_digest(root: Path) -> str:
    listed = subprocess.check_output(
        ["git", "ls-files", "-z", "--", "scripts", ".kutha"],
        cwd=root,
    )
    digest = hashlib.sha256()
    for rel in listed.split(b"\0"):
        if not rel:
            continue
        digest.update(rel)
        digest.update(b"\0")
        path = root / rel.decode()
        if path.is_file():
            digest.update(path.read_bytes())
    return digest.hexdigest()


class SelftestTests(unittest.TestCase):
    def test_empty_file_contains_needles_is_vacuous(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "empty-needles",
                    "description": "synthetic vacuous file_contains",
                    "steps": [
                        {"kind": "file_contains", "path": "README.md", "needles": []},
                    ],
                }
            ],
            {"README.md": "hello\n"},
        )
        rows, code = run_selftest(root)
        self.assertEqual(1, code)
        self.assertEqual(1, len(rows))
        self.assertEqual(VACUOUS, rows[0].status)
        self.assertEqual("empty-needles", rows[0].check_id)
        buf = StringIO()
        with patch.object(sys, "stdout", buf):
            cli = main(["--root", str(root), "selftest"])
        self.assertEqual(1, cli)
        self.assertIn("VACUOUS empty-needles", buf.getvalue())

    def test_selftest_freeze_ok_leaves_tracked_harness_bytes(self) -> None:
        before = _tracked_harness_digest(ROOT)
        porcelain_before = subprocess.check_output(
            ["git", "status", "--porcelain", "--", "crates", ".kutha/STATE.md"],
            cwd=ROOT,
        )
        for check_id in ("freeze-rocks", "freeze-cypher-deps", "freeze-hnsw-dep"):
            buf = StringIO()
            with patch.object(sys, "stdout", buf):
                code = main(["--root", str(ROOT), "--check", check_id, "selftest"])
            self.assertEqual(0, code, msg=buf.getvalue())
            line = buf.getvalue().strip().splitlines()[0]
            self.assertTrue(line.startswith(f"OK {check_id} mutations="), msg=line)
        after = _tracked_harness_digest(ROOT)
        self.assertEqual(before, after)
        porcelain_after = subprocess.check_output(
            ["git", "status", "--porcelain", "--", "crates", ".kutha/STATE.md"],
            cwd=ROOT,
        )
        self.assertEqual(porcelain_before, porcelain_after)

    def test_live_repo_selftest_is_green(self) -> None:
        before = _tracked_harness_digest(ROOT)
        buf = StringIO()
        with patch.object(sys, "stdout", buf):
            code = main(["--root", str(ROOT), "selftest"])
        text = buf.getvalue()
        self.assertEqual(0, code, msg=text)
        self.assertNotRegex(text, r"UNPROVEN|VACUOUS|BASELINE-FAIL")
        self.assertEqual(before, _tracked_harness_digest(ROOT))

    def test_copy_excludes_dv1_heavy_trees(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            dest = Path(raw) / "tree"
            copy_harness_tree(ROOT, dest)
            self.assertFalse((dest / ".git").exists())
            self.assertFalse((dest / "target").exists())
            self.assertFalse((dest / ".cursor" / "gsd-core").exists())
            self.assertFalse((dest / ".cursor" / "skills").exists())
            self.assertTrue((dest / ".kutha" / "STATE.md").is_file())
            self.assertTrue((dest / "crates" / "kutha-runtime").is_dir())

    def test_copy_skips_compound_engineering_and_planning_milestones(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            dest = Path(raw) / "tree"
            copy_harness_tree(ROOT, dest)
            self.assertFalse((dest / ".compound-engineering").exists())
            self.assertFalse((dest / ".planning" / "milestones").exists())
            self.assertTrue((dest / ".planning" / "STATE.md").is_file())

    def test_synthetic_pointer_derives_own_mutation(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "pointer-only",
                    "description": "pointer own-rule mutation",
                    "steps": [
                        {
                            "kind": "pointer_in_other_file",
                            "path": "a.md",
                            "pattern": r"(?P<id>X)",
                            "other": "b.md",
                        }
                    ],
                }
            ],
            {"a.md": "X\n", "b.md": "X\n"},
        )
        rows, code = run_selftest(root)
        self.assertEqual(0, code, msg=rows)
        self.assertEqual("OK", rows[0].status)
        self.assertEqual(CLASS_DERIVED, rows[0].classification)

    def test_file_exists_only_is_classified_low(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "only-exists",
                    "description": "file_exists deletion is not sibling proof",
                    "steps": [{"kind": "file_exists", "path": "README.md"}],
                }
            ],
            {"README.md": "x\n"},
        )
        rows, code = run_selftest(root)
        self.assertEqual(0, code, msg=rows)
        self.assertEqual("OK", rows[0].status)
        self.assertEqual(CLASS_FILE_EXISTS_ONLY, rows[0].classification)
        self.assertIn("file-exists-only", format_row(rows[0]))

    def test_every_live_check_is_classified(self) -> None:
        allowed = {
            CLASS_DERIVED,
            CLASS_EXPLICIT_MUTATION,
            CLASS_EXPLICIT_SKIP,
            CLASS_FILE_EXISTS_ONLY,
        }
        lows: list[str] = []
        live = get_checks(ROOT)
        self.assertEqual(CLASS_EXPLICIT_MUTATION, classify_check(live["adr-status"], ROOT)[0])
        for check in live.values():
            cls, reason = classify_check(check, ROOT)
            self.assertIn(cls, allowed, msg=f"{check.name} -> {cls} {reason}")
            self.assertNotEqual(CLASS_UNPROVEN, cls, msg=check.name)
            if cls == CLASS_EXPLICIT_SKIP:
                self.assertTrue(reason, msg=check.name)
            if cls == CLASS_FILE_EXISTS_ONLY:
                lows.append(check.name)
        if lows:
            print("LOW file-exists-only:", ",".join(lows))

    def test_docs_coupling_is_explicit_skip(self) -> None:
        checks = get_checks(ROOT)
        cls, reason = classify_check(checks["docs-coupling"], ROOT)
        self.assertEqual(CLASS_EXPLICIT_SKIP, cls)
        self.assertTrue(reason)

    def test_synthetic_yaml_map_list_unique_derives(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "ledger",
                    "description": "unique ids",
                    "steps": [
                        {
                            "kind": "yaml_map_list",
                            "path": "ledger.yaml",
                            "select": "rows",
                            "unique": "id",
                        }
                    ],
                }
            ],
            {"ledger.yaml": "rows:\n  - id: a\n  - id: b\n"},
        )
        rows, code = run_selftest(root)
        self.assertEqual(0, code, msg=rows)
        self.assertEqual("OK", rows[0].status)
        self.assertEqual(CLASS_DERIVED, rows[0].classification)

    def test_synthetic_glob_none_derives(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "no-egg",
                    "description": "glob none",
                    "steps": [{"kind": "glob_none", "glob": "**/*.egg-info/*"}],
                }
            ],
            {"README.md": "ok\n"},
        )
        rows, code = run_selftest(root)
        self.assertEqual(0, code, msg=rows)
        self.assertEqual("OK", rows[0].status)

    def test_synthetic_glob_regex_absent_derives(self) -> None:
        # M124: glob_regex_absent must be DERIVABLE without explicit selftest YAML.
        root = _write_mini(
            [
                {
                    "id": "no-join-tmp",
                    "description": "no fixed join after tmp",
                    "steps": [
                        {
                            "kind": "glob_regex_absent",
                            "glob": "crates/**/*.rs",
                            "patterns": [r'\.join\("tmp"\)\s*\.join\("'],
                        }
                    ],
                }
            ],
            {
                "crates/k/src/lib.rs": (
                    'PathBuf::from("x")\n        .join("tmp")\n'
                    '        .join(format!("leaf-{n}"))\n'
                ),
            },
        )
        rows, code = run_selftest(root)
        self.assertEqual(0, code, msg=rows)
        self.assertEqual("OK", rows[0].status)
        self.assertEqual(CLASS_DERIVED, rows[0].classification)

    def test_synthetic_yaml_needles_in_glob_derives(self) -> None:
        # M125: yaml_needles_in_glob must be DERIVABLE without explicit selftest YAML.
        root = _write_mini(
            [
                {
                    "id": "needles",
                    "description": "yaml names must appear in glob",
                    "steps": [
                        {
                            "kind": "yaml_needles_in_glob",
                            "path": "names.yaml",
                            "select": "items",
                            "glob": "src/**/*.rs",
                            "prefix": "fn ",
                        }
                    ],
                }
            ],
            {
                "names.yaml": "items:\n  - foo\n  - bar\n",
                "src/lib.rs": "fn foo() {}\nfn bar() {}\n",
            },
        )
        rows, code = run_selftest(root)
        self.assertEqual(0, code, msg=rows)
        self.assertEqual("OK", rows[0].status)
        self.assertEqual(CLASS_DERIVED, rows[0].classification)

    def test_allowed_kinds_are_derivable_or_skip_listed(self) -> None:
        # M125: only git_path_implies may stay non-DERIVABLE (docs-coupling skip).
        from kutha_gov.kinds import ALLOWED_KINDS
        from kutha_gov.selftest import DERIVABLE_KINDS

        self.assertEqual(ALLOWED_KINDS - DERIVABLE_KINDS, frozenset({"git_path_implies"}))

    def test_synthetic_glob_paths_in_file_derives(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "paths",
                    "description": "glob paths must be cited",
                    "steps": [
                        {
                            "kind": "glob_paths_in_file",
                            "glob": "docs/ADR/ADR-*.md",
                            "path": "index.yaml",
                            "prefix": "    path: ",
                        }
                    ],
                }
            ],
            {
                "docs/ADR/ADR-001.md": "# one\n",
                "index.yaml": "    path: docs/ADR/ADR-001.md\n",
            },
        )
        rows, code = run_selftest(root)
        self.assertEqual(0, code, msg=rows)
        self.assertEqual("OK", rows[0].status)

    def test_synthetic_when_match_then_match_derives(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "when-then",
                    "description": "when then",
                    "steps": [
                        {
                            "kind": "when_match_then_match",
                            "path": "STATE.md",
                            "pattern": r"^Active: None\s*$",
                            "flags": ["multiline"],
                            "then_pattern": r"(?m)^L_delivery=\S+-closed\s*$",
                        }
                    ],
                }
            ],
            {"STATE.md": "Active: None\nL_delivery=idle-closed\n"},
        )
        rows, code = run_selftest(root)
        self.assertEqual(0, code, msg=rows)
        self.assertEqual("OK", rows[0].status)

    def test_synthetic_markdown_heading_tag_derives(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "status-tag",
                    "description": "status tag",
                    "steps": [
                        {
                            "kind": "markdown_heading_tag",
                            "glob": "docs/ADR/ADR-*.md",
                            "heading": "## Status",
                            "allowed": ["Proposed", "Accepted"],
                        }
                    ],
                }
            ],
            {"docs/ADR/ADR-001.md": "## Status\n\n**Proposed**\n"},
        )
        rows, code = run_selftest(root)
        self.assertEqual(0, code, msg=rows)
        self.assertEqual("OK", rows[0].status)

    def test_synthetic_skip_is_success(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "skipped",
                    "description": "declared skip",
                    "selftest": {"skip": "depends on the live git diff"},
                    "steps": [
                        {
                            "kind": "git_path_implies",
                            "when_any": ["crates/**/*.rs"],
                            "then_any": ["CHANGELOG.md"],
                        }
                    ],
                }
            ]
        )
        rows, code = run_selftest(root)
        self.assertEqual(0, code)
        self.assertEqual(SKIP, rows[0].status)
        self.assertEqual("depends on the live git diff", rows[0].reason)
        self.assertIn("reason=depends on the live git diff", format_row(rows[0]))

    def test_skip_unless_glob_derives_when_live(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "budget",
                    "description": "skip unless live context",
                    "selftest": {
                        "skip": "no live CONTEXT",
                        "skip_unless_glob": ".planning/phases/*-*/*-CONTEXT.md",
                    },
                    "steps": [
                        {
                            "kind": "glob_max_lines",
                            "glob": ".planning/phases/*-*/*-SUMMARY.md",
                            "min_phase": 22,
                            "min_hits": 1,
                            "when_glob_exists": ".planning/phases/*-*/*-CONTEXT.md",
                            "max": 60,
                            "max_files_per_dir": 1,
                        }
                    ],
                }
            ],
            {
                ".planning/phases/22-x/22-CONTEXT.md": "ctx\n",
                ".planning/phases/22-x/22-01-SUMMARY.md": "ok\n",
            },
        )
        rows, code = run_selftest(root)
        self.assertEqual(0, code, msg=rows)
        self.assertEqual("OK", rows[0].status)

    def test_skip_unless_glob_skips_when_idle(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "budget-idle",
                    "description": "skip when no context",
                    "selftest": {
                        "skip": "no live CONTEXT",
                        "skip_unless_glob": ".planning/phases/*-*/*-CONTEXT.md",
                    },
                    "steps": [
                        {
                            "kind": "glob_max_lines",
                            "glob": ".planning/phases/*-*/*-SUMMARY.md",
                            "max": 60,
                            "min_hits": 1,
                        }
                    ],
                }
            ]
        )
        rows, code = run_selftest(root)
        self.assertEqual(0, code)
        self.assertEqual(SKIP, rows[0].status)
        self.assertEqual("no live CONTEXT", rows[0].reason)

    def test_baseline_fail_on_unmodified_copy(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "missing-needle",
                    "description": "fails before mutation",
                    "steps": [
                        {
                            "kind": "file_contains",
                            "path": "README.md",
                            "needles": ["not-in-file"],
                        }
                    ],
                }
            ],
            {"README.md": "hello\n"},
        )
        rows, code = run_selftest(root)
        self.assertEqual(1, code)
        self.assertEqual(BASELINE_FAIL, rows[0].status)

    def test_declared_mutation_that_does_not_trip_is_vacuous(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "noop-mut",
                    "description": "declared mutation is a no-op",
                    "selftest": {
                        "mutations": [
                            {
                                "op": "append_text",
                                "path": "README.md",
                                "text": "\n harmless",
                            }
                        ]
                    },
                    "steps": [
                        {
                            "kind": "file_contains",
                            "path": "README.md",
                            "needles": ["hello"],
                        }
                    ],
                }
            ],
            {"README.md": "hello\n"},
        )
        rows, code = run_selftest(root)
        self.assertEqual(1, code)
        self.assertEqual(VACUOUS, rows[0].status)

    def test_require_any_removes_all_needles_in_one_mutation(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "any-needles",
                    "description": "require any is one atomic mutation",
                    "steps": [
                        {
                            "kind": "file_contains",
                            "path": "ROADMAP.md",
                            "needles": ["- [ ] **H5**", "- [x] **H5**"],
                            "require": "any",
                        }
                    ],
                }
            ],
            {"ROADMAP.md": "- [ ] **H5**\n- [x] **H4**\n"},
        )
        rows, code = run_selftest(root)
        self.assertEqual(0, code, msg=rows)
        self.assertEqual("OK", rows[0].status)
        self.assertEqual(1, rows[0].mutations)

    def test_nameless_dict_id_is_load_error(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "ok-check",
                    "description": "valid",
                    "steps": [{"kind": "file_exists", "path": "README.md"}],
                },
                {
                    "description": "missing id",
                    "steps": [{"kind": "file_exists", "path": "README.md"}],
                },
            ],
            {"README.md": "x\n"},
        )
        from kutha_gov.dictionary import LoadErrorCheck
        from kutha_gov.protocol import Context

        checks = get_checks(root)
        self.assertEqual({"ok-check", LoadErrorCheck.name}, set(checks))
        self.assertIn("ok-check", checks, "valid rows must survive a nameless sibling (CE #20)")
        result = checks[LoadErrorCheck.name].run(Context(root=root))
        self.assertGreaterEqual(result.high_count, 1)
        self.assertTrue(
            any("row 1 missing id" in f.message for f in result.findings),
            msg=result.findings,
        )

    def test_scalar_check_row_is_load_error(self) -> None:
        """M094 / CE #7: non-mapping checks.yaml rows must not be silent-skipped."""
        root = _write_mini(
            [
                {
                    "id": "ok-check",
                    "description": "valid",
                    "steps": [{"kind": "file_exists", "path": "README.md"}],
                },
            ],
            {"README.md": "x\n"},
        )
        path = root / ".kutha" / "dictionaries" / "checks.yaml"
        path.write_text(
            "schema: kutha-harness-checks/v1\n"
            "checks:\n"
            "  - id: ok-check\n"
            "    description: valid\n"
            "    steps:\n"
            "      - kind: file_exists\n"
            "        path: README.md\n"
            "  - just-a-string\n",
            encoding="utf-8",
        )
        from kutha_gov.dictionary import LoadErrorCheck
        from kutha_gov.protocol import Context

        checks = get_checks(root)
        self.assertEqual({"ok-check", LoadErrorCheck.name}, set(checks))
        result = checks[LoadErrorCheck.name].run(Context(root=root))
        self.assertGreaterEqual(result.high_count, 1)
        self.assertTrue(
            any("not a mapping" in f.message for f in result.findings),
            msg=result.findings,
        )

    def test_duplicate_check_id_is_load_error(self) -> None:
        """M094 / CE #8: duplicate check ids keep-first and LoadError."""
        root = _write_mini(
            [
                {
                    "id": "docs-entry",
                    "description": "first",
                    "steps": [{"kind": "file_exists", "path": "README.md"}],
                },
                {
                    "id": "docs-entry",
                    "description": "weaker second",
                    "steps": [{"kind": "file_exists", "path": "MISSING.md"}],
                },
            ],
            {"README.md": "x\n"},
        )
        from kutha_gov.dictionary import LoadErrorCheck
        from kutha_gov.protocol import Context

        checks = get_checks(root)
        self.assertEqual({"docs-entry", LoadErrorCheck.name}, set(checks))
        first = checks["docs-entry"]
        self.assertEqual("first", first.description)
        result = checks[LoadErrorCheck.name].run(Context(root=root))
        self.assertGreaterEqual(result.high_count, 1)
        self.assertTrue(
            any("duplicate" in f.message and "docs-entry" in f.message for f in result.findings),
            msg=result.findings,
        )

    def test_stepless_check_is_load_error(self) -> None:
        """M094 / CE #6: missing/scalar/empty steps must not vacuous-OK."""
        from kutha_gov.dictionary import LoadErrorCheck
        from kutha_gov.protocol import Context

        cases = [
            (
                "missing-steps",
                {
                    "id": "vacuous",
                    "description": "no steps key",
                },
                "non-empty steps",
            ),
            (
                "scalar-steps",
                {
                    "id": "vacuous",
                    "description": "scalar steps",
                    "steps": "not-a-list",
                },
                "non-empty steps",
            ),
            (
                "scalar-step-items",
                {
                    "id": "vacuous",
                    "description": "scalar items",
                    "steps": ["just-a-string"],
                },
                "mapping steps",
            ),
        ]
        for label, row, needle in cases:
            with self.subTest(label=label):
                root = _write_mini(
                    [
                        {
                            "id": "ok-check",
                            "description": "valid",
                            "steps": [{"kind": "file_exists", "path": "README.md"}],
                        },
                        row,
                    ],
                    {"README.md": "x\n"},
                )
                checks = get_checks(root)
                self.assertEqual({"ok-check", LoadErrorCheck.name}, set(checks))
                self.assertNotIn("vacuous", checks)
                result = checks[LoadErrorCheck.name].run(Context(root=root))
                self.assertGreaterEqual(result.high_count, 1)
                self.assertTrue(
                    any(needle in f.message for f in result.findings),
                    msg=result.findings,
                )

    def test_missing_cite_path_is_high(self) -> None:
        from kutha_gov.protocol import Context

        root = _write_mini(
            [
                {
                    "id": "cite-miss",
                    "description": "cite path must exist",
                    "steps": [
                        {
                            "kind": "cite_equals",
                            "source": {
                                "path": "SOURCE.md",
                                "pattern": r"version:\s*(\S+)",
                            },
                            "cites": [
                                {
                                    "path": "MISSING.md",
                                    "pattern": r"version:\s*(\S+)",
                                }
                            ],
                        }
                    ],
                }
            ],
            {"SOURCE.md": "version: 1.0\n"},
        )
        check = get_checks(root)["cite-miss"]
        result = check.run(Context(root=root))
        self.assertGreaterEqual(result.high_count, 1)
        self.assertTrue(
            any(f.category == "missing" and "MISSING.md" in f.message for f in result.findings),
            msg=result.findings,
        )

    def test_min_phase_unprefixed_dirname_is_high(self) -> None:
        from kutha_gov.protocol import Context

        root = _write_mini(
            [
                {
                    "id": "phase-dirs",
                    "description": "unprefixed phase dir",
                    "steps": [
                        {
                            "kind": "glob_max_lines",
                            "glob": ".planning/phases/*/*.md",
                            "min_phase": 1,
                            "max": 200,
                            "min_hits": 0,
                        }
                    ],
                }
            ],
            {".planning/phases/nophase/x.md": "# hi\n"},
        )
        check = get_checks(root)["phase-dirs"]
        result = check.run(Context(root=root))
        self.assertGreaterEqual(result.high_count, 1)
        self.assertTrue(
            any(
                "nophase" in (f.message or "") or "nophase" in (f.file or "")
                for f in result.findings
            ),
            msg=result.findings,
        )

    def test_empty_markdown_heading_glob_is_high(self) -> None:
        from kutha_gov.protocol import Context

        root = _write_mini(
            [
                {
                    "id": "empty-heading",
                    "description": "empty ADR glob",
                    "steps": [
                        {
                            "kind": "markdown_heading_tag",
                            "glob": "docs/ADR/ADR-*.md",
                            "heading": "## Status",
                            "allowed": ["Proposed", "Accepted"],
                        }
                    ],
                }
            ],
            {"docs/ADR/.keep": ""},
        )
        check = get_checks(root)["empty-heading"]
        result = check.run(Context(root=root))
        self.assertGreaterEqual(result.high_count, 1)

    def test_unknown_mutation_op_is_unproven(self) -> None:
        check = DictCheck(
            {
                "id": "bad-op",
                "description": "unknown op",
                "selftest": {"mutations": [{"op": "chmod", "path": "README.md"}]},
                "steps": [{"kind": "file_exists", "path": "README.md"}],
            }
        )
        root = _write_mini([dict(check.spec)], {"README.md": "x\n"})
        rows, code = run_selftest(root)
        self.assertEqual(1, code)
        self.assertEqual(UNPROVEN, rows[0].status)


if __name__ == "__main__":
    unittest.main()
