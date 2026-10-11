"""PAR-01: jobs-1 vs jobs-N parity, config, isolation, merge order."""

from __future__ import annotations

import atexit
import os
import shutil
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

_ENV_JOBS = "KUTHA_GOV_JOBS"
_ENV_DELAY = "KUTHA_GOV_TEST_DELAY_CHECK"


def _write_mini(checks: list[dict[str, object]], files: dict[str, str] | None = None) -> Path:
    raw = tempfile.mkdtemp(prefix="kutha-jobs-mini-")
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


def _capture(args: list[str]) -> tuple[int, str, str]:
    buf, err = StringIO(), StringIO()
    with patch.object(sys, "stdout", buf), patch.object(sys, "stderr", err):
        try:
            code = main(args)
        except SystemExit as exc:
            raw = exc.code
            if raw is None:
                code = 1
            elif isinstance(raw, int):
                code = raw
            else:
                code = 1
    return code, buf.getvalue(), err.getvalue()


class JobsConfigTests(unittest.TestCase):
    def tearDown(self) -> None:
        os.environ.pop(_ENV_JOBS, None)
        os.environ.pop(_ENV_DELAY, None)

    def test_flag_overrides_env_overrides_default(self) -> None:
        from kutha_gov.config import resolve_jobs

        previous = os.environ.pop(_ENV_JOBS, None)
        try:
            default = resolve_jobs(cli=None)
            self.assertEqual(min(os.cpu_count() or 1, 8), default)
            os.environ[_ENV_JOBS] = "4"
            self.assertEqual(4, resolve_jobs(cli=None))
            self.assertEqual(2, resolve_jobs(cli=2))
            self.assertEqual(1, resolve_jobs(cli=1))
        finally:
            os.environ.pop(_ENV_JOBS, None)
            if previous is not None:
                os.environ[_ENV_JOBS] = previous

    def test_invalid_jobs_rejected(self) -> None:
        from kutha_gov.config import resolve_jobs

        with self.assertRaises(ValueError):
            resolve_jobs(cli=0)
        with self.assertRaises(ValueError):
            resolve_jobs(cli=-1)
        with self.assertRaises(ValueError):
            resolve_jobs(cli="nope")
        previous = os.environ.pop(_ENV_JOBS, None)
        try:
            os.environ[_ENV_JOBS] = "0"
            with self.assertRaises(ValueError):
                resolve_jobs(cli=None)
            os.environ[_ENV_JOBS] = "abc"
            with self.assertRaises(ValueError):
                resolve_jobs(cli=None)
        finally:
            os.environ.pop(_ENV_JOBS, None)
            if previous is not None:
                os.environ[_ENV_JOBS] = previous

    def test_cli_invalid_jobs_is_contract_error(self) -> None:
        code, _out, err = _capture(["--jobs", "0", "--root", str(ROOT), "precommit"])
        self.assertEqual(2, code)
        self.assertTrue(err.strip())
        code, _out, err = _capture(["--jobs", "nope", "--root", str(ROOT), "precommit"])
        self.assertEqual(2, code)
        self.assertTrue(err.strip())

    def test_help_documents_jobs(self) -> None:
        code, out, err = _capture(["--help"])
        self.assertEqual(0, code)
        text = out + err
        self.assertIn("--jobs", text)
        self.assertIn("KUTHA_GOV_JOBS", text)


class JobsParityTests(unittest.TestCase):
    def tearDown(self) -> None:
        os.environ.pop(_ENV_JOBS, None)
        os.environ.pop(_ENV_DELAY, None)

    def test_precommit_real_tree_jobs_1_matches_jobs_n(self) -> None:
        args_1 = ["--jobs", "1", "--root", str(ROOT), "precommit"]
        args_n = ["--jobs", "2", "--root", str(ROOT), "precommit"]
        code_1, out_1, err_1 = _capture(args_1)
        code_n, out_n, err_n = _capture(args_n)
        self.assertEqual(0, code_1, msg=out_1 + err_1)
        self.assertEqual(code_1, code_n, msg=out_n + err_n)
        self.assertEqual(out_1, out_n)
        self.assertEqual(err_1, err_n)

    def test_mini_tree_seeded_failures_jobs_1_matches_jobs_n(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "alpha-high",
                    "description": "seeded HIGH",
                    "steps": [
                        {
                            "kind": "file_contains",
                            "path": "README.md",
                            "needles": ["MISSING-HIGH"],
                        }
                    ],
                },
                {
                    "id": "beta-low",
                    "description": "seeded LOW",
                    "steps": [
                        {
                            "kind": "file_contains",
                            "path": "README.md",
                            "needles": ["MISSING-LOW"],
                            "severity": "low",
                        }
                    ],
                },
                {
                    "id": "gamma-ok",
                    "description": "seeded OK",
                    "steps": [{"kind": "file_exists", "path": "README.md"}],
                },
            ],
            {"README.md": "hello\n"},
        )
        code_1, out_1, err_1 = _capture(
            ["--jobs", "1", "--root", str(root), "--fail-on-warn", "precommit"]
        )
        code_n, out_n, err_n = _capture(
            ["--jobs", "4", "--root", str(root), "--fail-on-warn", "precommit"]
        )
        self.assertEqual(1, code_1, msg=out_1 + err_1)
        self.assertEqual(code_1, code_n)
        self.assertEqual(out_1, out_n)
        self.assertEqual(err_1, err_n)
        self.assertIn("alpha-high", out_1)
        self.assertIn("beta-low", out_1)
        self.assertLess(out_1.index("alpha-high"), out_1.index("beta-low"))
        self.assertLess(out_1.index("beta-low"), out_1.index("gamma-ok"))
        self.assertIn("HIGH", out_1)
        self.assertIn("low", out_1.lower())

    def test_eval_check_worker_propagates_skip_emit_and_changed_paths(self) -> None:
        # CE #48: parallel worker Context must not drop skip_emit / changed_paths.
        from kutha_gov.parallel import _eval_check_worker
        from kutha_gov.protocol import Check, CheckResult, Context, Finding, Severity

        class Probe(Check):
            name = "ctx-probe"
            description = "records Context control fields"

            def run(self, ctx: Context) -> CheckResult:
                result = CheckResult(check=self.name)
                paths = ",".join(sorted(ctx.changed_paths or ()))
                result.findings.append(
                    Finding(
                        self.name,
                        Severity.LOW,
                        "ctx-flags",
                        f"skip_emit={ctx.skip_emit};paths={paths}",
                    )
                )
                return result

        result = _eval_check_worker(
            (
                Probe(),
                str(ROOT),
                False,
                "auto",
                False,
                True,
                frozenset({"a.md", "b.md"}),
            )
        )
        messages = [f.message for f in result.findings]
        self.assertTrue(
            any("skip_emit=True" in m and "a.md,b.md" in m for m in messages),
            msg=messages,
        )

    def test_mini_tree_check_raise_is_check_error_jobs_1_and_2(self) -> None:
        # CE #23: check.run exceptions are check-error under both serial and parallel.
        root = _write_mini(
            [
                {
                    "id": "boom-regex",
                    "description": "invalid regex raises in the worker",
                    "steps": [
                        {
                            "kind": "pointer_in_other_file",
                            "path": "a.md",
                            "pattern": "(?P<id",
                            "other": "b.md",
                        }
                    ],
                },
                {
                    "id": "ok-exists",
                    "description": "sibling still runs",
                    "steps": [{"kind": "file_exists", "path": "a.md"}],
                },
            ],
            {"a.md": "X\n", "b.md": "X\n"},
        )
        for jobs in ("1", "2"):
            code, out, err = _capture(["--jobs", jobs, "--root", str(root), "precommit"])
            self.assertEqual(1, code, msg=f"jobs={jobs}: {out}{err}")
            joined = out + err
            self.assertIn("check-error", joined, msg=f"jobs={jobs}: {joined}")
            self.assertNotIn("worker-error", joined, msg=f"jobs={jobs}: {joined}")
            self.assertIn("boom-regex", joined)
            self.assertIn("HIGH", joined)

    def test_selftest_mini_tree_jobs_1_matches_jobs_n(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "needs-hello",
                    "description": "file contains",
                    "steps": [{"kind": "file_contains", "path": "README.md", "needles": ["hello"]}],
                },
                {
                    "id": "needs-file",
                    "description": "file exists",
                    "steps": [{"kind": "file_exists", "path": "README.md"}],
                },
            ],
            {"README.md": "hello\n"},
        )
        code_1, out_1, err_1 = _capture(["--jobs", "1", "--root", str(root), "selftest"])
        code_n, out_n, err_n = _capture(["--jobs", "2", "--root", str(root), "selftest"])
        self.assertEqual(0, code_1, msg=out_1 + err_1)
        self.assertEqual(code_1, code_n, msg=out_n + err_n)
        self.assertEqual(out_1, out_n)


class JobsIsolationTests(unittest.TestCase):
    def tearDown(self) -> None:
        os.environ.pop(_ENV_JOBS, None)
        os.environ.pop(_ENV_DELAY, None)

    def test_two_roots_no_cache_leak_and_no_leftover_scratch(self) -> None:
        rust_ok = (
            "#[test]\nfn cache_probe() { assert!(true); }\n",
            "#[test]\nfn cache_probe() { let _x = 1; }\n",
        )
        checks: list[dict[str, object]] = [
            {
                "id": "rust-probe",
                "description": "rust fn index keyed by root",
                "steps": [
                    {
                        "kind": "rust_test_asserts",
                        "path": "crates/k/tests/t.rs",
                        "tests": ["cache_probe"],
                    }
                ],
            }
        ]
        root_ok = _write_mini(checks, {"crates/k/tests/t.rs": rust_ok[0]})
        root_bad = _write_mini(checks, {"crates/k/tests/t.rs": rust_ok[1]})
        for jobs in ("1", "2"):
            code_ok, out_ok, err_ok = _capture(
                ["--jobs", jobs, "--root", str(root_ok), "precommit"]
            )
            code_bad, out_bad, err_bad = _capture(
                ["--jobs", jobs, "--root", str(root_bad), "precommit"]
            )
            self.assertEqual(0, code_ok, msg=f"jobs={jobs} ok: {out_ok}{err_ok}")
            self.assertEqual(1, code_bad, msg=f"jobs={jobs} bad: {out_bad}{err_bad}")
            self.assertIn("rust-test-vacuous", out_bad, msg=f"jobs={jobs} {out_bad}")
        for root in (root_ok, root_bad):
            leftover = list((root / "tmp").glob("kutha-*")) if (root / "tmp").exists() else []
            self.assertEqual([], leftover, msg=leftover)

    def test_selftest_scratch_removed_after_jobs_n(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "needs-hello",
                    "description": "file contains",
                    "steps": [{"kind": "file_contains", "path": "README.md", "needles": ["hello"]}],
                }
            ],
            {"README.md": "hello\n"},
        )
        code, out, err = _capture(["--jobs", "2", "--root", str(root), "selftest"])
        self.assertEqual(0, code, msg=out + err)
        leftover = list((root / "tmp").glob("kutha-*")) if (root / "tmp").exists() else []
        self.assertEqual([], leftover, msg=leftover)


class JobsOrderingTests(unittest.TestCase):
    def tearDown(self) -> None:
        os.environ.pop(_ENV_JOBS, None)
        os.environ.pop(_ENV_DELAY, None)

    def test_first_check_sleep_does_not_reorder_output(self) -> None:
        root = _write_mini(
            [
                {
                    "id": "aaa-first",
                    "description": "dictionary-first; delayed in the worker",
                    "steps": [{"kind": "file_exists", "path": "README.md"}],
                },
                {
                    "id": "zzz-second",
                    "description": "dictionary-second; finishes first without merge-by-id",
                    "steps": [{"kind": "file_exists", "path": "README.md"}],
                },
            ],
            {"README.md": "hello\n"},
        )
        os.environ[_ENV_DELAY] = "aaa-first=0.4"
        code_1, out_1, err_1 = _capture(["--jobs", "1", "--root", str(root), "precommit"])
        code_n, out_n, err_n = _capture(["--jobs", "2", "--root", str(root), "precommit"])
        self.assertEqual(0, code_1, msg=out_1 + err_1)
        self.assertEqual(0, code_n, msg=out_n + err_n)
        self.assertEqual(out_1, out_n)
        self.assertLess(out_n.index("aaa-first"), out_n.index("zzz-second"))


class JobsAtMostOnceTests(unittest.TestCase):
    """CE #34: submit-after-first-success must not re-run accepted checks."""

    def test_submit_after_first_success_runs_missing_serial_once(self) -> None:
        from concurrent.futures import Future
        from unittest.mock import patch

        from kutha_gov.parallel import run_checks_parallel
        from kutha_gov.protocol import Check, CheckResult, Context

        class CountingCheck(Check):
            def __init__(self, name: str) -> None:
                self.name = name
                self.description = name
                self.runs = 0

            def run(self, ctx: Context) -> CheckResult:
                self.runs += 1
                return CheckResult(check=self.name)

        class FailAfterFirstPool:
            def __init__(self, *args: object, **kwargs: object) -> None:
                self.n = 0

            def __enter__(self) -> FailAfterFirstPool:
                return self

            def __exit__(self, *args: object) -> bool:
                return False

            def submit(self, fn: object, payload: object) -> Future:
                self.n += 1
                if self.n > 1:
                    raise RuntimeError("submit-after-first-success")
                fut: Future = Future()
                fut.set_result(fn(payload))  # type: ignore[misc, operator]
                return fut

        alpha = CountingCheck("alpha")
        beta = CountingCheck("beta")
        ctx = Context(root=ROOT, jobs=2)
        with patch("kutha_gov.parallel.ProcessPoolExecutor", FailAfterFirstPool):
            results, extra = run_checks_parallel(
                [("alpha", alpha), ("beta", beta)],
                ctx,
            )
        self.assertEqual(1, alpha.runs, msg="accepted submit must not re-run serial")
        self.assertEqual(1, beta.runs, msg="missing check runs serial once")
        self.assertEqual(["alpha", "beta"], [r.check for r in results])
        self.assertTrue(
            any(f.category == "pool-fallback" for f in extra),
            msg=[f.category for f in extra],
        )
