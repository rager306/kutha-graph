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
        self.assertEqual(
            ["docs-entry", "version-freeze"],
            [ln.split()[1] for ln in out.splitlines() if ln.startswith("OK")],
        )

    def test_precommit_empty_token_is_contract_error(self) -> None:
        err = io.StringIO()
        with redirect_stderr(err):
            code = main(["--root", str(ROOT), "--check", "docs-entry,", "precommit"])
        self.assertEqual(2, code)
        self.assertIn("empty check id", err.getvalue())

    def test_precommit_empty_check_flag_is_contract_error(self) -> None:
        """CE 2026-10-15 testing gap: --check '' must not run the full suite."""
        err = io.StringIO()
        with redirect_stderr(err):
            code = main(["--root", str(ROOT), "--check", "", "precommit"])
        self.assertEqual(2, code)
        self.assertIn("empty check id", err.getvalue())

    def test_precommit_repeated_check_flags_runs_both(self) -> None:
        """M079: --check a --check b must not drop all but the last flag."""
        buf = io.StringIO()
        with redirect_stdout(buf):
            code = main(
                [
                    "--root",
                    str(ROOT),
                    "--check",
                    "docs-entry",
                    "--check",
                    "version-freeze",
                    "precommit",
                ]
            )
        self.assertEqual(0, code)
        out = buf.getvalue()
        self.assertEqual(
            ["docs-entry", "version-freeze"],
            [ln.split()[1] for ln in out.splitlines() if ln.startswith("OK")],
        )

    def test_json_repeated_check_flags_runs_both(self) -> None:
        """M085: json shares --check append with precommit via _join_check_flags."""
        import json as json_mod

        buf = io.StringIO()
        with redirect_stdout(buf):
            code = main(
                [
                    "--root",
                    str(ROOT),
                    "--check",
                    "docs-entry",
                    "--check",
                    "version-freeze",
                    "json",
                ]
            )
        self.assertEqual(0, code)
        payload = json_mod.loads(buf.getvalue())
        names = {row["check"] for row in payload["checks"]}
        self.assertEqual({"docs-entry", "version-freeze"}, names)

    def test_selftest_repeated_check_flags_runs_both(self) -> None:
        """M086: selftest shares --check append with precommit/json via _join_check_flags."""
        buf = io.StringIO()
        with redirect_stdout(buf):
            code = main(
                [
                    "--root",
                    str(ROOT),
                    "--check",
                    "docs-entry",
                    "--check",
                    "version-freeze",
                    "selftest",
                ]
            )
        self.assertEqual(0, code)
        out = buf.getvalue()
        self.assertIn("docs-entry", out)
        self.assertIn("version-freeze", out)
        self.assertRegex(out, r"(?m)^OK docs-entry ")
        self.assertRegex(out, r"(?m)^OK version-freeze ")
        self.assertEqual(
            ["docs-entry", "version-freeze"],
            [ln.split()[1] for ln in out.splitlines() if ln.startswith("OK")],
        )

    def test_ci_rejects_empty_check_flag(self) -> None:
        """M115: --check '' on ci must not be treated as 'no flag' (full suite)."""
        err = io.StringIO()
        with redirect_stderr(err):
            code = main(["--root", str(ROOT), "--check", "", "ci"])
        self.assertEqual(2, code)
        self.assertIn(
            "--check is only valid with json, precommit, or selftest",
            err.getvalue(),
        )

    def test_ci_rejects_check_flag(self) -> None:
        """M088/M089: selective --check stays off ci; stderr redirects to check surfaces."""
        err = io.StringIO()
        with redirect_stderr(err):
            code = main(
                [
                    "--root",
                    str(ROOT),
                    "--check",
                    "docs-entry",
                    "ci",
                ]
            )
        self.assertEqual(2, code)
        out = err.getvalue()
        self.assertIn(
            "--check is only valid with json, precommit, or selftest",
            out,
        )
        # M140: redirect to check surfaces — not an exact `hint:` template needle.
        self.assertIn("kutha-gov", out)
        self.assertTrue(
            any(s in out for s in ("json", "precommit", "selftest")),
            f"stderr must name a --check surface: {out!r}",
        )
        self.assertIn("--check", out)

    def test_non_ci_rejects_skip_cargo_flag(self) -> None:
        """M090: --skip-cargo stays on ci; stderr redirects to ci --skip-cargo."""
        err = io.StringIO()
        with redirect_stderr(err):
            code = main(
                [
                    "--root",
                    str(ROOT),
                    "--skip-cargo",
                    "precommit",
                ]
            )
        self.assertEqual(2, code)
        out = err.getvalue()
        self.assertIn("--skip-cargo is only valid with ci", out)
        # M140: point at ci --skip-cargo without pinning exact hint wording.
        self.assertIn("kutha-gov", out)
        self.assertIn("ci", out)
        self.assertIn("--skip-cargo", out)

    def test_join_check_flags_merges_append_and_comma(self) -> None:
        from kutha_gov.__main__ import _join_check_flags

        self.assertIsNone(_join_check_flags(None))
        self.assertEqual(
            "docs-entry,version-freeze,agents-lean",
            _join_check_flags(["docs-entry,version-freeze", "agents-lean"]),
        )


if __name__ == "__main__":
    unittest.main()
