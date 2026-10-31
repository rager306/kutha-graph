"""M113 S01 — get_checks fail-closes on substring-preserving selftest replace_regex."""

from __future__ import annotations

import tempfile
import unittest
from pathlib import Path
from typing import cast

import yaml

from kutha_gov.dictionary import DICT_REL, LoadErrorCheck, get_checks

ROOT = Path(__file__).resolve().parents[2]


def _write_checks(checks: list[dict]) -> Path:
    raw = tempfile.mkdtemp(prefix="kutha-m113-")
    root = Path(raw)
    path = root / DICT_REL
    path.parent.mkdir(parents=True)
    payload = {"schema": "kutha-harness-checks/v1", "checks": checks}
    path.write_text(yaml.safe_dump(payload, sort_keys=False), encoding="utf-8")
    return root


class SelftestMutationStaticTests(unittest.TestCase):
    def test_replace_regex_with_still_contains_pattern_is_load_error(self) -> None:
        root = _write_checks(
            [
                {
                    "id": "ok-check",
                    "steps": [
                        {
                            "kind": "file_contains",
                            "path": "README.md",
                            "needles": ["x"],
                            "require": "all",
                        }
                    ],
                },
                {
                    "id": "vacuous-selftest",
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
                },
            ]
        )
        (root / "README.md").write_text("needle\n", encoding="utf-8")
        loaded = get_checks(root)
        self.assertIn("ok-check", loaded)
        self.assertIn(LoadErrorCheck.name, loaded)
        msg = cast(LoadErrorCheck, loaded[LoadErrorCheck.name])._message
        self.assertIn("vacuous-selftest", msg)
        self.assertIn("substring", msg.lower())


if __name__ == "__main__":
    unittest.main()
