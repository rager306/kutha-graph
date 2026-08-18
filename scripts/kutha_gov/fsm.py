"""Meta-prompt FSM for one harness quantum. Dictionary owns states; Python interprets."""

from __future__ import annotations

from collections.abc import Mapping
from dataclasses import dataclass, field
from pathlib import Path

import yaml

from kutha_gov.checks import get_checks
from kutha_gov.protocol import Check, CheckResult, Context, Finding, Severity
from kutha_gov.time_log import LOG_REL, append_run, fold_log

FSM_REL = ".kutha/dictionaries/fsm.yaml"
SCHEMA = "kutha-harness-fsm/v1"

ALLOWED_STATE_KINDS: frozenset[str] = frozenset(
    {
        "noop",
        "require_file",
        "run_checks",
        "observe_cargo",
        "emit_log",
        "fold_log",
        "decide",
        "terminal",
    }
)

type Step = Mapping[str, object]


@dataclass
class Machine:
    initial: str
    terminal: frozenset[str]
    states: dict[str, Step]
    transitions: list[Step]
    budget_default: int


@dataclass
class QuantumOutcome:
    terminal: str
    trace: list[str]
    high: int = 0
    low: int = 0
    skipped: int = 0
    results: list[CheckResult] = field(default_factory=list)
    findings: list[Finding] = field(default_factory=list)
    observations: list[tuple[str, str]] = field(default_factory=list)


def load_machine(root: Path) -> Machine:
    path = root / FSM_REL
    loaded = yaml.safe_load(path.read_text(encoding="utf-8"))
    if not isinstance(loaded, dict):
        raise ValueError(f"{FSM_REL} root must be a mapping")
    if loaded.get("schema") != SCHEMA:
        raise ValueError(f"unknown FSM schema {loaded.get('schema')!r}")
    defaults = loaded.get("defaults", {})
    budget_raw = None
    if isinstance(defaults, dict):
        budget_raw = defaults.get("budget")
    if not isinstance(budget_raw, int):
        raise ValueError("fsm.yaml defaults.budget must be an integer")
    budget = budget_raw
    terminal_raw = loaded.get("terminal", [])
    terminal = (
        frozenset(item for item in terminal_raw if isinstance(item, str))
        if isinstance(terminal_raw, list)
        else frozenset()
    )
    states_raw = loaded.get("states", {})
    states: dict[str, Step] = {}
    if isinstance(states_raw, dict):
        for name, spec in states_raw.items():
            if isinstance(name, str) and isinstance(spec, dict):
                states[name] = spec
    trans_raw = loaded.get("transitions", [])
    transitions: list[Step] = []
    if isinstance(trans_raw, list):
        transitions = [row for row in trans_raw if isinstance(row, dict)]
    initial = loaded.get("initial", "idle")
    if not isinstance(initial, str):
        initial = "idle"
    return Machine(
        initial=initial,
        terminal=terminal,
        states=states,
        transitions=transitions,
        budget_default=budget,
    )


def step_kind(kind: str) -> Finding | None:
    if kind in ALLOWED_STATE_KINDS:
        return None
    return Finding(
        check="fsm",
        severity=Severity.HIGH,
        category="unknown-fsm-kind",
        message=f"unknown FSM kind {kind!r} (not in META allowlist) — fail-closed",
    )


def _str(step: Step, key: str, default: str = "") -> str:
    value = step.get(key, default)
    return value if isinstance(value, str) else default


def _lookup(machine: Machine, current: str, event: str) -> str | None:
    for row in machine.transitions:
        if _str(row, "from") == current and _str(row, "event") == event:
            nxt = _str(row, "to")
            return nxt or None
    return None


def _run_one(check: Check, ctx: Context) -> CheckResult:
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


def run_quantum(ctx: Context, *, budget: int, start_event: str = "start") -> QuantumOutcome:
    outcome = QuantumOutcome(terminal="fail", trace=[])
    try:
        machine = load_machine(ctx.root)
    except (OSError, ValueError, yaml.YAMLError) as exc:
        outcome.findings.append(Finding("fsm", Severity.HIGH, "load", f"{FSM_REL}: {exc}", FSM_REL))
        outcome.trace = ["fail"]
        outcome.high = 1
        return outcome

    state = machine.initial
    outcome.trace.append(state)
    event = start_event
    guard = 0
    while state not in machine.terminal:
        guard += 1
        if guard > 64:
            outcome.findings.append(
                Finding("fsm", Severity.HIGH, "cycle", "FSM exceeded 64 steps — fail-closed")
            )
            state = "fail"
            outcome.trace.append(state)
            break
        nxt = _lookup(machine, state, event)
        if nxt is None:
            outcome.findings.append(
                Finding(
                    "fsm",
                    Severity.HIGH,
                    "unknown-transition",
                    f"no transition from {state!r} on {event!r} — fail-closed",
                )
            )
            state = "fail"
            outcome.trace.append(state)
            break
        state = nxt
        outcome.trace.append(state)
        spec = machine.states.get(state, {})
        kind = _str(spec, "kind")
        unknown = step_kind(kind)
        if unknown is not None:
            outcome.findings.append(unknown)
            state = "fail"
            outcome.trace.append(state)
            break
        event = _execute(kind, spec, ctx, outcome, budget=budget)

    outcome.terminal = state if state in machine.terminal else "fail"
    outcome.high += sum(r.high_count for r in outcome.results)
    outcome.low += sum(r.low_count for r in outcome.results)
    outcome.high += sum(1 for f in outcome.findings if f.severity is Severity.HIGH)
    outcome.low += sum(1 for f in outcome.findings if f.severity is Severity.LOW)
    return outcome


def _execute(
    kind: str,
    spec: Step,
    ctx: Context,
    outcome: QuantumOutcome,
    *,
    budget: int,
) -> str:
    if kind in {"noop", "terminal"}:
        return "ok"
    if kind == "require_file":
        path = _str(spec, "path")
        if ctx.read(path) is None:
            outcome.findings.append(
                Finding("fsm", Severity.HIGH, "missing", f"missing {path}", path)
            )
            return "fail"
        return "ok"
    if kind == "run_checks":
        checks = get_checks(ctx.root)
        ordered = sorted(checks.items())
        selected = ordered[:budget]
        outcome.skipped = len(ordered) - len(selected)
        outcome.results = [_run_one(chk, ctx) for _, chk in selected]
        return "done"
    if kind == "observe_cargo":
        from kutha_gov.observe import run_cargo_observation

        code, seen_ok, findings = run_cargo_observation(ctx.root, spec)
        outcome.findings.extend(findings)
        status = "ok" if code == 0 and not findings else "fail"
        outcome.observations.append(("cargo", status))
        for name in seen_ok:
            outcome.observations.append((name, "ok"))
        return "done"
    if kind == "emit_log":
        high = sum(r.high_count for r in outcome.results) + sum(
            1 for f in outcome.findings if f.severity is Severity.HIGH
        )
        low = sum(r.low_count for r in outcome.results) + sum(
            1 for f in outcome.findings if f.severity is Severity.LOW
        )
        append_run(
            ctx.root,
            high=high,
            low=low,
            check_count=len(outcome.results),
            observations=outcome.observations,
        )
        return "ok"
    if kind == "fold_log":
        fold_log(ctx.root / LOG_REL)
        return "ok"
    if kind == "decide":
        high = sum(r.high_count for r in outcome.results) + sum(
            1 for f in outcome.findings if f.severity is Severity.HIGH
        )
        low = sum(r.low_count for r in outcome.results) + sum(
            1 for f in outcome.findings if f.severity is Severity.LOW
        )
        if high:
            return "fail"
        if ctx.fail_on_warn and low:
            return "fail"
        return "ok"
    return "fail"
