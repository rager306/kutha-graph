"""M112 S01 — replace_regex must not leave the pattern as a substring of `with`."""

from __future__ import annotations

import tempfile
import unittest
from pathlib import Path

from kutha_gov.dictionary import DictCheck
from kutha_gov.selftest import UNPROVEN, run_selftest

ROOT = Path(__file__).resolve().parents[2]


def _write_mini(checks: list[dict], files: dict[str, str]) -> Path:
    raw = tempfile.mkdtemp(prefix="kutha-m112-")
    root = Path(raw)
    (root / ".kutha" / "dictionaries").mkdir(parents=True)
    import yaml

    payload = {"schema": "kutha-harness-checks/v1", "checks": checks}
    (root / ".kutha" / "dictionaries" / "checks.yaml").write_text(
        yaml.safe_dump(payload, sort_keys=False),
        encoding="utf-8",
    )
    for rel, body in files.items():
        path = root / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(body, encoding="utf-8")
    return root


class SelftestSubstringTrapTests(unittest.TestCase):
    def test_replace_regex_with_still_contains_pattern_is_unproven(self) -> None:
        check = DictCheck(
            {
                "id": "substring-trap",
                "description": "vacuous replace keeps needle",
                "selftest": {
                    "mutations": [
                        {
                            "op": "replace_regex",
                            "path": "README.md",
                            "pattern": "needle",
                            "with": "needle_REMOVED",
                        }
                    ]
                },
                "steps": [
                    {
                        "kind": "file_contains",
                        "path": "README.md",
                        "needles": ["needle"],
                        "require": "all",
                    }
                ],
            }
        )
        root = _write_mini([dict(check.spec)], {"README.md": "hello needle world\n"})
        rows, code = run_selftest(root)
        self.assertEqual(1, code)
        # M113: load also emits LoadErrorCheck; assert the named check row.
        trap = next(row for row in rows if row.check_id == "substring-trap")
        self.assertEqual(UNPROVEN, trap.status)
        self.assertIn("substring", trap.reason.lower())

    def test_yaml_map_list_require_fields_swap_is_not_substring(self) -> None:
        from kutha_gov.kinds import yaml_map_list_mutation_target

        root = _write_mini(
            [
                {
                    "id": "ledger",
                    "steps": [
                        {
                            "kind": "yaml_map_list",
                            "path": "ledger.yaml",
                            "select": "rows",
                            "require_fields": ["id"],
                        }
                    ],
                }
            ],
            {"ledger.yaml": "rows:\n  - id: a\n    claim: x\n"},
        )
        step = {
            "kind": "yaml_map_list",
            "path": "ledger.yaml",
            "select": "rows",
            "require_fields": ["id"],
        }
        target = yaml_map_list_mutation_target(step, root)
        self.assertIsNotNone(target)
        assert target is not None
        _path, pattern, replacement = target
        self.assertNotIn(pattern.replace("\\", ""), replacement)
        self.assertIn("__selftest_rm_id__", replacement)


if __name__ == "__main__":
    unittest.main()
