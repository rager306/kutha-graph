from __future__ import annotations

import hashlib
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
from kutha_gov.dictionary import DictCheck  # noqa: E402
from kutha_gov.selftest import (  # noqa: E402
    BASELINE_FAIL,
    SKIP,
    UNPROVEN,
    VACUOUS,
    copy_harness_tree,
    format_row,
    run_selftest,
)


def _write_mini(checks: list[dict[str, object]], files: dict[str, str] | None = None) -> Path:
    raw = tempfile.mkdtemp(prefix="kutha-selftest-mini-")
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
        buf = StringIO()
        with patch.object(sys, "stdout", buf):
            code = main(["--root", str(ROOT), "--check", "freeze", "selftest"])
        self.assertEqual(0, code, msg=buf.getvalue())
        line = buf.getvalue().strip().splitlines()[0]
        self.assertTrue(line.startswith("OK freeze mutations="), msg=line)
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

    def test_synthetic_unproven_pointer_only(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "pointer-only",
                    "description": "no derivable step",
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
        self.assertEqual(1, code)
        self.assertEqual(UNPROVEN, rows[0].status)

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
