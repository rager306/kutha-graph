"""Meta-prompt FSM for one harness quantum. Dictionary owns states; Python interprets."""

from __future__ import annotations

import time
from collections.abc import Mapping
from concurrent.futures import Future, ThreadPoolExecutor
from contextlib import suppress
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

import yaml

from kutha_gov.checks import get_checks
from kutha_gov.parallel import run_checks_parallel
from kutha_gov.protocol import CheckResult, Context, Finding, Severity
from kutha_gov.time_log import (
    LOG_REL,
    append_observe,
    append_run,
    fold_log,
    sync_membership_edition_if_changed,
)

FSM_REL = ".kutha/dictionaries/fsm.yaml"
SCHEMA = "kutha-harness-fsm/v1"

ALLOWED_STATE_KINDS: frozenset[str] = frozenset(
    {
        "noop",
        "require_file",
        "run_checks",
        "run_selftest",
        "observe_cargo",
        "emit_log",
        "emit_tenant",
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
    evidence: list[tuple[str, str]] = field(default_factory=list)
    phase_ms: list[tuple[str, int]] = field(default_factory=list)


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


def _high_so_far(outcome: QuantumOutcome) -> bool:
    return any(r.high_count for r in outcome.results) or any(
        f.severity is Severity.HIGH for f in outcome.findings
    )


def _skip_heavy_observe(ctx: Context, outcome: QuantumOutcome) -> bool:
    return ctx.skip_cargo or _high_so_far(outcome)


def _note_observe_skipped(outcome: QuantumOutcome, state: str, cause: str) -> None:
    """LOW breadcrumb when an observe_cargo state does not run (CE #39).

    Relation stays allowlisted `cargo` (same as `_apply_cargo_pack`); state+cause
    live in the finding message so emit_log does not fail-closed on unknown rels.
    """
    outcome.findings.append(
        Finding(
            "observe_cargo",
            Severity.LOW,
            "observe-skipped",
            f"{state}: skipped ({cause})",
        )
    )
    outcome.observations.append(("cargo", "skipped"))


def _overlap_specs(machine: Machine) -> list[tuple[str, Step]]:
    specs: list[tuple[str, Step]] = []
    for name, spec in machine.states.items():
        if _str(spec, "kind") != "observe_cargo":
            continue
        raw = spec.get("overlap_with")
        if isinstance(raw, list) and raw:
            specs.append((name, spec))
    return specs


def _overlap_pipeline(
    root: Path, specs: list[tuple[str, Step]]
) -> dict[str, tuple[int, list[str], list[Finding]] | None]:
    from kutha_gov.observe import run_cargo_observation

    out: dict[str, tuple[int, list[str], list[Finding]] | None] = {}
    skip_rest = False
    for name, spec in specs:
        if skip_rest:
            out[name] = None
            continue
        code, seen_ok, findings = run_cargo_observation(root, spec)
        out[name] = (code, seen_ok, findings)
        if any(finding.severity is Severity.HIGH for finding in findings):
            skip_rest = True
    return out


def _maybe_start_overlap(machine: Machine, ctx: Context, overlap: dict[str, Any]) -> None:
    if overlap.get("fut") is not None:
        return
    if ctx.jobs <= 1 or ctx.skip_cargo:
        return
    specs = _overlap_specs(machine)
    if not specs:
        return
    pool = ThreadPoolExecutor(max_workers=1)
    overlap["pool"] = pool
    overlap["fut"] = pool.submit(_overlap_pipeline, ctx.root, specs)


def _shutdown_overlap(overlap: dict[str, Any]) -> None:
    fut = overlap.get("fut")
    if isinstance(fut, Future):
        with suppress(Exception):
            fut.result()
    pool = overlap.get("pool")
    if isinstance(pool, ThreadPoolExecutor):
        pool.shutdown(wait=True)


def _apply_cargo_pack(
    outcome: QuantumOutcome, packed: tuple[int, list[str], list[Finding]]
) -> None:
    code, seen_ok, findings = packed
    outcome.findings.extend(findings)
    status = "ok" if code == 0 and not findings else "fail"
    outcome.observations.append(("cargo", status))
    for name in seen_ok:
        outcome.evidence.append((name, "ok"))


def run_quantum(ctx: Context, *, budget: int, start_event: str = "start") -> QuantumOutcome:
    outcome = QuantumOutcome(terminal="fail", trace=[])
    overlap: dict[str, Any] = {}
    try:
        try:
            machine = load_machine(ctx.root)
        except (OSError, ValueError, yaml.YAMLError) as exc:
            outcome.findings.append(
                Finding("fsm", Severity.HIGH, "load", f"{FSM_REL}: {exc}", FSM_REL)
            )
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
            if kind == "run_checks":
                _maybe_start_overlap(machine, ctx, overlap)
            started = time.monotonic()
            event = _execute(
                kind,
                spec,
                ctx,
                outcome,
                budget=budget,
                state=state,
                overlap=overlap,
            )
            outcome.phase_ms.append((kind, int((time.monotonic() - started) * 1000)))

        outcome.terminal = state if state in machine.terminal else "fail"
        outcome.high += sum(r.high_count for r in outcome.results)
        outcome.low += sum(r.low_count for r in outcome.results)
        outcome.high += sum(1 for f in outcome.findings if f.severity is Severity.HIGH)
        outcome.low += sum(1 for f in outcome.findings if f.severity is Severity.LOW)
        return outcome
    finally:
        _shutdown_overlap(overlap)


def _execute(
    kind: str,
    spec: Step,
    ctx: Context,
    outcome: QuantumOutcome,
    *,
    budget: int,
    state: str,
    overlap: dict[str, Any],
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
        if outcome.skipped > 0:
            outcome.findings.append(
                Finding(
                    "fsm",
                    Severity.HIGH,
                    "budget",
                    f"truncated {outcome.skipped} checks under budget={budget}",
                )
            )
        results, extra = run_checks_parallel(selected, ctx)
        outcome.results = results
        outcome.findings.extend(extra)
        return "done"
    if kind == "run_selftest":
        from kutha_gov.selftest import SUCCESS, WORKER_ERROR, run_selftest

        rows, _code = run_selftest(ctx.root, jobs=ctx.jobs)
        proved = sum(1 for row in rows if row.status in SUCCESS)
        total = len(rows)
        outcome.evidence.append(("h5_selftest", f"{proved}/{total}"))
        for row in rows:
            if row.status in SUCCESS:
                continue
            if row.status == WORKER_ERROR:
                outcome.findings.append(
                    Finding(
                        row.check_id,
                        Severity.HIGH,
                        "worker-error",
                        row.reason or "worker raised",
                    )
                )
                continue
            extra = f" ({row.reason})" if row.reason else ""
            outcome.findings.append(
                Finding(
                    "selftest",
                    Severity.HIGH,
                    "selftest",
                    f"{row.check_id} is {row.status}{extra}",
                )
            )
        return "done"
    if kind == "observe_cargo":
        if _skip_heavy_observe(ctx, outcome):
            cause = "skip_cargo" if ctx.skip_cargo else "prior-high"
            _note_observe_skipped(outcome, state, cause)
            return "done"
        raw_overlap = spec.get("overlap_with")
        if ctx.jobs > 1 and not (isinstance(raw_overlap, list) and raw_overlap):
            outcome.findings.append(
                Finding(
                    "observe_cargo",
                    Severity.HIGH,
                    "overlap-required",
                    f"{state}: jobs>1 requires overlap_with on observe_cargo (not silent skip)",
                )
            )
            return "done"
        fut = overlap.get("fut")
        if isinstance(fut, Future):
            try:
                pipeline = fut.result()
            except Exception:
                pipeline = None
            if isinstance(pipeline, dict):
                packed = pipeline.get(state)
                if packed is None:
                    _note_observe_skipped(outcome, state, "overlap-after-high")
                    return "done"
                _apply_cargo_pack(outcome, packed)
                return "done"
        from kutha_gov.observe import run_cargo_observation

        code, seen_ok, findings = run_cargo_observation(ctx.root, spec)
        _apply_cargo_pack(outcome, (code, seen_ok, findings))
        return "done"
    if kind == "emit_log":
        if ctx.skip_emit:
            return "ok"
        high = sum(r.high_count for r in outcome.results) + sum(
            1 for f in outcome.findings if f.severity is Severity.HIGH
        )
        low = sum(r.low_count for r in outcome.results) + sum(
            1 for f in outcome.findings if f.severity is Severity.LOW
        )
        _event, rejected = append_run(
            ctx.root,
            high=high,
            low=low,
            check_count=len(outcome.results),
            observations=outcome.observations,
        )
        for rel in rejected:
            outcome.findings.append(
                Finding(
                    "emit_log",
                    Severity.HIGH,
                    "unknown-relation",
                    f"unknown process relation {rel!r} (not in allowlist) — fail-closed",
                )
            )
        _, membership_rejected = sync_membership_edition_if_changed(ctx.root)
        for rel in membership_rejected:
            outcome.findings.append(
                Finding(
                    "emit_log",
                    Severity.HIGH,
                    "unknown-relation",
                    f"unknown process relation {rel!r} (not in allowlist) — fail-closed",
                )
            )
        return "ok"
    if kind == "emit_tenant":
        if ctx.skip_emit:
            return "done"
        if _skip_heavy_observe(ctx, outcome):
            return "done"
        from kutha_gov.tenant import run_tenant_observation

        code, findings, _text = run_tenant_observation(ctx.root, spec)
        outcome.findings.extend(findings)
        status = "ok" if code == 0 and not findings else "fail"
        outcome.observations.append(("tenant", status))
        if not append_observe(ctx.root, "tenant", status):
            outcome.findings.append(
                Finding(
                    "emit_tenant",
                    Severity.HIGH,
                    "unknown-relation",
                    "unknown process relation 'tenant' (not in allowlist) — fail-closed",
                )
            )
        return "done"
    if kind == "fold_log":
        try:
            fold_log(ctx.root / LOG_REL)
        except ValueError as exc:
            outcome.findings.append(
                Finding("fold_log", Severity.HIGH, "fold", str(exc))
            )
            outcome.observations.append(("fold_log", "fail"))
            return "fail"
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
