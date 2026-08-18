"""Load governor checks from `.kutha/dictionaries/checks.yaml`."""

from __future__ import annotations

from collections.abc import Mapping
from pathlib import Path

import yaml

from kutha_gov.kinds import run_step
from kutha_gov.protocol import Check, CheckResult, Context, Finding, Severity

DICT_REL = ".kutha/dictionaries/checks.yaml"
SCHEMA = "kutha-harness-checks/v1"


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
    for row in rows:
        if not isinstance(row, dict):
            continue
        check = DictCheck(row)
        if check.name:
            found[check.name] = check
    if not found:
        return {LoadErrorCheck.name: LoadErrorCheck("dictionary.checks is empty")}
    return found
