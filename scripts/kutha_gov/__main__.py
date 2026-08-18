"""CLI: uv run kutha-gov [list|ci|explain NAME|fsm]. Python >=3.13."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

from kutha_gov.checks import get_checks
from kutha_gov.config import ENV_FAIL_ON_WARN, env_flag, load_dotenv, resolve_budget
from kutha_gov.dictionary import DICT_REL, DictCheck
from kutha_gov.fsm import FSM_REL, load_machine, run_quantum
from kutha_gov.protocol import CheckResult, Context, Finding, Severity
from kutha_gov.time_log import LOG_REL, fold_log


def _detect_root(start: Path) -> Path:
    here = start.resolve()
    for candidate in [here, *here.parents]:
        if all((candidate / hint).exists() for hint in (".kutha", "docs/ADR")):
            return candidate
    return here


def _run_one(check, ctx: Context) -> CheckResult:
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


def cmd_list(root: Path) -> int:
    checks = get_checks(root)
    width = max((len(n) for n in checks), default=0)
    for name, chk in sorted(checks.items()):
        print(f"  {name:<{width}}  {chk.description}")
    print(f"\n{len(checks)} check(s) from {DICT_REL} (append a row to add one).")
    return 0


def cmd_explain(root: Path, name: str) -> int:
    checks = get_checks(root)
    chk = checks.get(name)
    if not chk:
        print(f"unknown check: {name}", file=sys.stderr)
        return 2
    print(f"check: {chk.name}")
    print(f"purpose: {chk.description}")
    print("authority: none — harness does not accept ADRs or claim product readiness")
    if isinstance(chk, DictCheck):
        print(f"source: {chk.source}")
        print("steps:")
        for step in chk.steps:
            kind = step.get("kind", "?")
            target = step.get("path") or step.get("glob") or step.get("paths") or ""
            print(f"  - {kind}  {target}")
    else:
        print("kind: load-error (dictionary failed closed)")
    return 0


def cmd_ci(ctx: Context, *, budget: int) -> int:
    outcome = run_quantum(ctx, budget=budget, start_event="start")
    print("fsm: " + " → ".join(outcome.trace))
    for result in outcome.results:
        status = "OK" if result.passed else "FAIL"
        print(
            f"{status:4} {result.check}  high={result.high_count} low={result.low_count}"
            + (f"  {result.note}" if result.note else "")
        )
        for finding in result.findings:
            if finding.severity is Severity.HIGH or ctx.fail_on_warn:
                print(f"     {finding.format()}")
    if outcome.observations:
        joined = ", ".join(f"{rel}={obj}" for rel, obj in outcome.observations)
        print(f"observe: {joined}  (evidence, not SoT)")
    for finding in outcome.findings:
        if finding.severity is Severity.HIGH or ctx.fail_on_warn:
            print(f"     {finding.format()}")
    rung = "H0"
    if any(rel == "cargo" for rel, _obj in outcome.observations):
        rung = "H1"
    if any(rel == "tenant" for rel, _obj in outcome.observations):
        rung = "H2"
    print(
        f"\nharness: {outcome.high} HIGH, {outcome.low} LOW, "
        f"{len(outcome.results)} checks  ({rung} dogfood)"
    )
    if outcome.skipped:
        print(
            f"cui: truncated {outcome.skipped} slices under V={budget} "
            "(max-convolution comes at H2)"
        )
    picture = fold_log(ctx.root / LOG_REL)
    print(
        f"fold: runs={picture.run_count} last={picture.last_run} "
        f"high={picture.last_high} cargo={picture.last_cargo} "
        f"tenant={picture.last_tenant}  "
        f"({LOG_REL} = process log, not product SoT)"
    )
    if outcome.terminal != "ok":
        return 1
    return 0


def cmd_json(ctx: Context) -> int:
    checks = get_checks(ctx.root)
    results = [_run_one(chk, ctx) for _, chk in sorted(checks.items())]
    payload = {
        "schema": "kutha-harness-report/v1",
        "authoritative": False,
        "rung": "H0",
        "checks": [
            {
                "check": r.check,
                "passed": r.passed,
                "high": r.high_count,
                "low": r.low_count,
                "findings": [
                    {
                        "severity": f.severity.value,
                        "category": f.category,
                        "message": f.message,
                        "file": f.file,
                        "line": f.line,
                    }
                    for f in r.findings
                ],
            }
            for r in results
        ],
    }
    print(json.dumps(payload, indent=2))
    return 0 if all(r.passed for r in results) else 1


def cmd_fsm(root: Path) -> int:
    machine = load_machine(root)
    print(f"source: {FSM_REL}")
    print(f"initial: {machine.initial}")
    print("terminal: " + ", ".join(sorted(machine.terminal)))
    print(f"defaults.budget: {machine.budget_default}")
    print("states:")
    for name, spec in machine.states.items():
        print(f"  {name}: kind={spec.get('kind')}")
    print("transitions:")
    for row in machine.transitions:
        print(f"  {row.get('from')} --{row.get('event')}--> {row.get('to')}")
    print("\nappend a state/transition in the YAML to extend the quantum.")
    return 0


def cmd_py(root: Path) -> int:
    """Recursive dogfood: ruff (Astral), ty (Astral), pyrefly (Meta)."""
    import subprocess

    tools: list[tuple[str, list[str]]] = [
        ("ruff check", ["uv", "run", "ruff", "check", "scripts/"]),
        ("ruff format --check", ["uv", "run", "ruff", "format", "--check", "scripts/"]),
        ("ty check", ["uv", "run", "ty", "check"]),
        # Editable install maps site-packages -> scripts/ via .pth; project-mode
        # then excludes that tree. File-mode + explicit excludes keeps dogfood honest.
        (
            "pyrefly check",
            [
                "uv",
                "run",
                "pyrefly",
                "check",
                "scripts/kutha_gov",
                "scripts/tests",
                "--project-excludes",
                ".venv/**",
                "--project-excludes",
                "**/__pycache__/**",
            ],
        ),
    ]
    exit_code = 0
    for name, cmd in tools:
        rc = subprocess.call(cmd, cwd=str(root))
        status = "OK" if rc == 0 else "FAIL"
        print(f"  {status} {name}")
        if rc != 0:
            exit_code = 1
    print("\nPYTHON_TOOLING_GREEN" if exit_code == 0 else "\nPYTHON_TOOLING_FAIL")
    return exit_code


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(prog="kutha_gov", description="Kutha parallel harness (H2)")
    parser.add_argument("--root", type=Path, default=None)
    parser.add_argument("--fail-on-warn", action="store_true")
    parser.add_argument(
        "--budget",
        type=int,
        default=None,
        help="Cui-lite V (overrides KUTHA_GOV_BUDGET and fsm.yaml defaults.budget)",
    )
    parser.add_argument(
        "command",
        nargs="?",
        default="ci",
        choices=("list", "ci", "explain", "json", "fold", "py", "fsm"),
    )
    parser.add_argument("name", nargs="?", help="check name for explain")
    args = parser.parse_args(argv)
    root = _detect_root(args.root or Path.cwd())
    load_dotenv(root)
    fail_on_warn = args.fail_on_warn or env_flag(ENV_FAIL_ON_WARN)
    ctx = Context(root=root, fail_on_warn=fail_on_warn)
    if args.command == "list":
        return cmd_list(root)
    if args.command == "explain":
        if not args.name:
            print("explain requires a check name", file=sys.stderr)
            return 2
        return cmd_explain(root, args.name)
    if args.command == "json":
        return cmd_json(ctx)
    if args.command == "py":
        return cmd_py(root)
    if args.command == "fsm":
        return cmd_fsm(root)
    if args.command == "fold":
        picture = fold_log(root / LOG_REL)
        print(
            json.dumps(
                {
                    "log": LOG_REL,
                    "soT": "harness-events-jsonl (H0); product log is crates/kutha-runtime",
                    "run_count": picture.run_count,
                    "last_run": picture.last_run,
                    "last_high": picture.last_high,
                    "last_low": picture.last_low,
                    "last_cargo": picture.last_cargo,
                    "last_tenant": picture.last_tenant,
                },
                indent=2,
            )
        )
        return 0
    machine = load_machine(root)
    budget = resolve_budget(cli=args.budget, fsm_default=machine.budget_default)
    return cmd_ci(ctx, budget=budget)


if __name__ == "__main__":
    raise SystemExit(main())
