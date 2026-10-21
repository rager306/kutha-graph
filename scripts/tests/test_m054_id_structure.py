"""M054 S05 / Phase 121 — kinds.py family split (CE #21).

M138: behavioral import/dispatch oracles — not type-annotation text needles.
"""

from __future__ import annotations

import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GOV = ROOT / "scripts" / "kutha_gov"
sys.path.insert(0, str(ROOT / "scripts"))


class KindsFamilySplitTests(unittest.TestCase):
    def test_run_step_and_runners_stay_in_kinds_py(self) -> None:
        from kutha_gov import kinds

        self.assertTrue(callable(kinds.run_step))
        self.assertIsInstance(kinds.RUNNERS, dict)
        self.assertIn("frontmatter_paths_exist", kinds.RUNNERS)
        self.assertIn("cargo_dep_constraint", kinds.RUNNERS)
        self.assertIn("test_matrix", kinds.RUNNERS)

    def test_runners_dispatch_uses_sibling_family_callables(self) -> None:
        """M138 — pin live RUNNERS callables, not source-string needles."""
        from kutha_gov import kinds
        from kutha_gov.kind_cargo import _kind_cargo_dep_constraint
        from kutha_gov.kind_matrix import _kind_test_matrix
        from kutha_gov.kind_planning import _kind_frontmatter_paths_exist

        self.assertIs(kinds.RUNNERS["test_matrix"], _kind_test_matrix)
        self.assertIs(kinds.RUNNERS["cargo_dep_constraint"], _kind_cargo_dep_constraint)
        self.assertIs(
            kinds.RUNNERS["frontmatter_paths_exist"], _kind_frontmatter_paths_exist
        )

    def test_policy_families_live_in_sibling_modules(self) -> None:
        self.assertTrue((GOV / "kind_planning.py").is_file())
        self.assertTrue((GOV / "kind_cargo.py").is_file())
        self.assertTrue((GOV / "kind_matrix.py").is_file())
        kinds_src = (GOV / "kinds.py").read_text(encoding="utf-8")
        self.assertNotIn("def _kind_frontmatter_paths_exist(", kinds_src)
        self.assertNotIn("def _kind_glob_max_lines(", kinds_src)
        self.assertNotIn("def _kind_cargo_dep_constraint(", kinds_src)
        self.assertNotIn("def _kind_test_matrix(", kinds_src)
        extras = [p.name for p in (GOV / "checks").glob("*.py") if p.name != "__init__.py"]
        self.assertEqual([], extras)


if __name__ == "__main__":
    unittest.main()
