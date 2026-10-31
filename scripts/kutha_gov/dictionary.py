"""Load governor checks from `.kutha/dictionaries/checks.yaml`."""

from __future__ import annotations

from collections.abc import Mapping
from pathlib import Path

import yaml

from kutha_gov.kinds import run_step
from kutha_gov.protocol import Check, CheckResult, Context, Finding, Severity

DICT_REL = ".kutha/dictionaries/checks.yaml"
SCHEMA = "kutha-harness-checks/v1"


def replace_regex_substring_error(pattern: object, replacement: object) -> str:
    """M112/M113: `with` must not contain `pattern` as a substring (vacuous proof)."""
    if not isinstance(pattern, str) or not pattern:
        return ""
    repl = replacement if isinstance(replacement, str) else ""
    if pattern in repl:
        return (
            "replace_regex `with` still contains pattern as a substring "
            f"(pattern={pattern!r}); use a replacement that removes the needle"
        )
    return ""


def selftest_declared_mutation_errors(check_id: str, selftest: object) -> list[str]:
    """Static errors on declared selftest.mutations (load-time dogfood)."""
    if not isinstance(selftest, dict) or "mutations" not in selftest:
        return []
    raw = selftest.get("mutations")
    if not isinstance(raw, list):
        return [f"dictionary check {check_id!r} selftest.mutations must be a list"]
    errors: list[str] = []
    for index, item in enumerate(raw):
        if not isinstance(item, dict):
            errors.append(
                f"dictionary check {check_id!r} selftest.mutations[{index}] must be a mapping"
            )
            continue
        if item.get("op") != "replace_regex":
            continue
        msg = replace_regex_substring_error(item.get("pattern"), item.get("with", ""))
        if msg:
            errors.append(f"dictionary check {check_id!r} selftest: {msg}")
    return errors


class DictCheck(Check):
    """One dictionary row. Steps are kinds from META.md, not Python subclasses."""

    source = DICT_REL

    def __init__(self, spec: Mapping[str, object]) -> None:
        self.name = str(spec.get("id", ""))
        self.description = str(spec.get("description", ""))
        self.spec = spec
        raw_steps = spec.get("steps", [])
        self.steps: list[Mapping[str, object]] = []
        if isinstance(raw_steps, list):
            self.steps = [step for step in raw_steps if isinstance(step, dict)]

    def run(self, ctx: Context) -> CheckResult:
        result = CheckResult(check=self.name)
        for step in self.steps:
            before = len(result.findings)
            run_step(self.name, step, ctx, result)
            if step.get("stop_on_fail") is True:
                added = result.findings[before:]
                if any(finding.severity is Severity.HIGH for finding in added):
                    break
        return result


class LoadErrorCheck(Check):
    name = "_dictionary"
    description = "failed to load checks dictionary"

    def __init__(self, message: str, file: str = DICT_REL) -> None:
        self._message = message
        self._file = file

    def run(self, ctx: Context) -> CheckResult:
        del ctx
        result = CheckResult(check=self.name)
        result.findings.append(Finding(self.name, Severity.HIGH, "load", self._message, self._file))
        return result


def default_root() -> Path:
    here = Path.cwd().resolve()
    for candidate in [here, *here.parents]:
        if (candidate / DICT_REL).is_file():
            return candidate
    return Path(__file__).resolve().parents[2]


def get_checks(root: Path | None = None) -> dict[str, Check]:
    base = root if root is not None else default_root()
    path = base / DICT_REL
    if not path.is_file():
        return {LoadErrorCheck.name: LoadErrorCheck(f"missing {DICT_REL}")}
    try:
        loaded = yaml.safe_load(path.read_text(encoding="utf-8"))
    except yaml.YAMLError as exc:
        return {LoadErrorCheck.name: LoadErrorCheck(f"YAML error: {exc}")}
    if not isinstance(loaded, dict):
        return {LoadErrorCheck.name: LoadErrorCheck("dictionary root must be a mapping")}
    schema = loaded.get("schema")
    if schema != SCHEMA:
        return {
            LoadErrorCheck.name: LoadErrorCheck(
                f"unknown dictionary schema {schema!r} (expected {SCHEMA})"
            )
        }
    rows = loaded.get("checks")
    if not isinstance(rows, list):
        return {LoadErrorCheck.name: LoadErrorCheck("dictionary.checks must be a list")}
    found: dict[str, Check] = {}
    # Keep valid checks; fail closed via LoadError alongside them (CE #20 / #6-#8).
    load_errors: list[str] = []
    for index, row in enumerate(rows):
        if not isinstance(row, dict):
            load_errors.append(f"dictionary check row {index} is not a mapping")
            continue
        check = DictCheck(row)
        if not check.name:
            load_errors.append(f"dictionary check row {index} missing id")
            continue
        if check.name in found:
            load_errors.append(f"duplicate check id {check.name!r} (kept first)")
            continue
        raw_steps = row.get("steps", [])
        if not isinstance(raw_steps, list) or not raw_steps:
            load_errors.append(f"dictionary check {check.name!r} needs a non-empty steps list")
            continue
        if not check.steps:
            load_errors.append(f"dictionary check {check.name!r} has no mapping steps")
            continue
        load_errors.extend(selftest_declared_mutation_errors(check.name, row.get("selftest")))
        found[check.name] = check
    if load_errors:
        found[LoadErrorCheck.name] = LoadErrorCheck("; ".join(load_errors))
    if not found:
        return {LoadErrorCheck.name: LoadErrorCheck("dictionary.checks is empty")}
    return found
