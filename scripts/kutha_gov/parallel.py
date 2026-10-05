"""Process-pool evaluation of independent dictionary checks. Merge is by check id."""

from __future__ import annotations

import multiprocessing
import os
import time
from concurrent.futures import ProcessPoolExecutor, as_completed
from pathlib import Path

from kutha_gov.protocol import Check, CheckResult, Context, Finding, Severity

_ENV_DELAY = "KUTHA_GOV_TEST_DELAY_CHECK"


def _delay_if_requested(check_name: str) -> None:
    raw = os.environ.get(_ENV_DELAY, "").strip()
    if not raw or "=" not in raw:
        return
    want, _, secs = raw.partition("=")
    if want != check_name:
        return
    time.sleep(float(secs))


def _mp_context():
    return multiprocessing.get_context("spawn")


def eval_check(check: Check, ctx: Context) -> CheckResult:
    """Serial reference: exceptions become check-error HIGH (jobs 1 oracle)."""
    _delay_if_requested(check.name)
    try:
        result = check.run(ctx)
    except Exception as exc:
        result = CheckResult(check=check.name)
        result.findings.append(
            Finding(
                check=check.name,
                severity=Severity.HIGH,
                category="check-error",
                message=f"{type(exc).__name__}: {exc}",
            )
        )
    result.check = check.name
    return result


def worker_error_result(check: str, exc: BaseException) -> CheckResult:
    result = CheckResult(check=check)
    result.findings.append(
        Finding(
            check=check,
            severity=Severity.HIGH,
            category="worker-error",
            message=f"{type(exc).__name__}: {exc}",
        )
    )
    return result


def _eval_check_worker(payload: tuple[Check, str, bool, str, bool]) -> CheckResult:
    check, root, fail_on_warn, git_against, skip_cargo = payload
    _delay_if_requested(check.name)
    ctx = Context(
        root=Path(root),
        fail_on_warn=fail_on_warn,
        git_against=git_against,
        skip_cargo=skip_cargo,
        jobs=1,
    )
    result = check.run(ctx)
    result.check = check.name
    return result


def run_checks_parallel(
    selected: list[tuple[str, Check]], ctx: Context
) -> tuple[list[CheckResult], list[Finding]]:
    """Evaluate checks; merge in `selected` order. Extra findings are pool-fallback LOW."""
    extra: list[Finding] = []
    if ctx.jobs <= 1 or len(selected) <= 1:
        return [eval_check(chk, ctx) for _, chk in selected], extra

    by_name: dict[str, CheckResult | None] = dict.fromkeys(name for name, _ in selected)
    workers = max(1, min(ctx.jobs, len(selected)))
    try:
        with ProcessPoolExecutor(max_workers=workers, mp_context=_mp_context()) as pool:
            futs = {
                pool.submit(
                    _eval_check_worker,
                    (
                        chk,
                        str(ctx.root),
                        ctx.fail_on_warn,
                        ctx.git_against,
                        ctx.skip_cargo,
                    ),
                ): name
                for name, chk in selected
            }
            try:
                for fut in as_completed(futs):
                    name = futs[fut]
                    try:
                        by_name[name] = fut.result()
                    except Exception as exc:
                        by_name[name] = worker_error_result(name, exc)
            except Exception as exc:
                extra.append(
                    Finding(
                        "fsm",
                        Severity.LOW,
                        "pool-fallback",
                        f"process pool broken ({type(exc).__name__}: {exc}); "
                        "unfinished checks ran serial",
                    )
                )
                for name, chk in selected:
                    if by_name[name] is None:
                        by_name[name] = eval_check(chk, ctx)
    except Exception as exc:
        extra.append(
            Finding(
                "fsm",
                Severity.LOW,
                "pool-fallback",
                f"process pool failed ({type(exc).__name__}: {exc}); unfinished checks ran serial",
            )
        )
        for name, chk in selected:
            if by_name[name] is None:
                by_name[name] = eval_check(chk, ctx)

    results: list[CheckResult] = []
    for name, chk in selected:
        got = by_name.get(name)
        if got is None:
            got = eval_check(chk, ctx)
        results.append(got)
    return results, extra
