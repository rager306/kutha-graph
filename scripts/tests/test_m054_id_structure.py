"""M054 S05 / Phase 121 — kinds.py family split (CE #21)."""

from __future__ import annotations

import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GOV = ROOT / "scripts" / "kutha_gov"


class KindsFamilySplitTests(unittest.TestCase):
    def test_run_step_and_runners_stay_in_kinds_py(self) -> None:
        text = (GOV / "kinds.py").read_text(encoding="utf-8")
        self.assertIn("def run_step(", text)
        self.assertIn("RUNNERS: dict[str, Runner]", text)
        self.assertIn('"frontmatter_paths_exist": _kind_frontmatter_paths_exist', text)
        self.assertIn('"cargo_dep_constraint": _kind_cargo_dep_constraint', text)
        self.assertIn('"test_matrix": _kind_test_matrix', text)

    def test_policy_families_live_in_sibling_modules(self) -> None:
        self.assertTrue((GOV / "kind_planning.py").is_file())
        self.assertTrue((GOV / "kind_cargo.py").is_file())
        self.assertTrue((GOV / "kind_matrix.py").is_file())
        kinds = (GOV / "kinds.py").read_text(encoding="utf-8")
        self.assertNotIn("def _kind_frontmatter_paths_exist(", kinds)
        self.assertNotIn("def _kind_glob_max_lines(", kinds)
        self.assertNotIn("def _kind_cargo_dep_constraint(", kinds)
        self.assertNotIn("def _kind_test_matrix(", kinds)
        extras = [p.name for p in (GOV / "checks").glob("*.py") if p.name != "__init__.py"]
        self.assertEqual([], extras)


if __name__ == "__main__":
    unittest.main()
