"""M077 S01 — comma-separated --check ids for precommit/json/selftest."""

from __future__ import annotations

import io
import unittest
from contextlib import redirect_stderr, redirect_stdout
from pathlib import Path

from kutha_gov.__main__ import _parse_check_ids, _select_checks, main


ROOT = Path(__file__).resolve().parents[2]


class MultiCheckSelectTests(unittest.TestCase):
    def test_parse_check_ids_splits_and_dedupes(self) -> None:
        ids, err = _parse_check_ids("docs-entry, version-freeze,docs-entry")
        self.assertEqual(0, err)
        self.assertEqual(["docs-entry", "version-freeze"], ids)

    def test_parse_check_ids_rejects_empty_token(self) -> None:
        ids, err = _parse_check_ids("docs-entry,,version-freeze")
        self.assertEqual(2, err)
        self.assertEqual([], ids)

    def test_select_checks_returns_both_dictionary_rows(self) -> None:
        selected, err = _select_checks(ROOT, "docs-entry,version-freeze")
        self.assertEqual(0, err)
        self.assertEqual({"docs-entry", "version-freeze"}, set(selected))

    def test_select_checks_unknown_in_list_is_contract_error(self) -> None:
        selected, err = _select_checks(ROOT, "docs-entry,no-such-check")
        self.assertEqual(2, err)
        self.assertEqual({}, selected)

    def test_precommit_comma_list_runs_both(self) -> None:
        buf = io.StringIO()
        with redirect_stdout(buf):
            code = main(
                [
                    "--root",
                    str(ROOT),
                    "--check",
                    "docs-entry,version-freeze",
                    "precommit",
                ]
            )
        self.assertEqual(0, code)
        out = buf.getvalue()
        self.assertIn("docs-entry", out)
        self.assertIn("version-freeze", out)

    def test_precommit_empty_token_is_contract_error(self) -> None:
        err = io.StringIO()
        with redirect_stderr(err):
            code = main(
                ["--root", str(ROOT), "--check", "docs-entry,", "precommit"]
            )
        self.assertEqual(2, code)
        self.assertIn("empty check id", err.getvalue())


if __name__ == "__main__":
    unittest.main()
