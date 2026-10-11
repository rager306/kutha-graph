"""Cargo.toml dependency-constraint family (CE #21)."""

from __future__ import annotations

import tomllib
from collections.abc import Mapping
from pathlib import Path

from kutha_gov.kinds import Step, _category, _fmt, _severity, _str
from kutha_gov.protocol import CheckResult, Context, Finding, Severity

_DEP_TABLE_NAMES = ("dependencies", "dev-dependencies", "build-dependencies")
_RULE_FORBID = "forbid"
_RULE_REQUIRE_OPTIONAL = "require_optional"


def _cargo_dep_rules(step: Step) -> dict[str, str] | None:
    raw = step.get("rules")
    if not isinstance(raw, dict) or not raw:
        return None
    rules: dict[str, str] = {}
    for key, action in raw.items():
        if not isinstance(key, str) or not key.strip():
            return None
        if not isinstance(action, str) or action not in {_RULE_FORBID, _RULE_REQUIRE_OPTIONAL}:
            return None
        rules[key] = action
    return rules


def _dep_names_and_optional(key: str, value: object) -> tuple[tuple[str, ...], bool]:
    if isinstance(value, str):
        return (key,), False
    if not isinstance(value, dict):
        return (key,), False
    optional = value.get("optional") is True
    pkg = value.get("package")
    if isinstance(pkg, str) and pkg.strip() and pkg.strip() != key:
        return (key, pkg.strip()), optional
    if isinstance(pkg, str) and pkg.strip():
        return (pkg.strip(),), optional
    return (key,), optional


def _collect_dep_tables(data: Mapping[str, object]) -> list[Mapping[str, object]]:
    tables: list[Mapping[str, object]] = []
    for name in _DEP_TABLE_NAMES:
        raw = data.get(name)
        if isinstance(raw, dict):
            tables.append(raw)
    workspace = data.get("workspace")
    if isinstance(workspace, dict):
        deps = workspace.get("dependencies")
        if isinstance(deps, dict):
            tables.append(deps)
    target = data.get("target")
    if isinstance(target, dict):
        for cfg in target.values():
            if not isinstance(cfg, dict):
                continue
            for name in _DEP_TABLE_NAMES:
                raw = cfg.get(name)
                if isinstance(raw, dict):
                    tables.append(raw)
    return tables


def cargo_dep_constraint_mutation_target(step: Step, root: Path) -> tuple[str, str] | None:
    pattern = _str(step, "glob")
    rules = _cargo_dep_rules(step)
    if not pattern or not rules:
        return None
    hit: Path | None = None
    for path in sorted(root.glob(pattern)):
        if path.is_file():
            hit = path
            break
    if hit is None:
        return None
    name = next(iter(rules))
    rel = hit.relative_to(root).as_posix()
    return rel, f'\n[dependencies]\n{name} = "0.0.0"\n'


def _kind_cargo_dep_constraint(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    pattern = _str(step, "glob")
    if not pattern:
        result.scanned += 1
        result.findings.append(Finding(check, Severity.HIGH, "cargo-dep", "glob is required"))
        return
    rules = _cargo_dep_rules(step)
    if rules is None:
        result.scanned += 1
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "cargo-dep",
                "rules must map crate names to forbid|require_optional",
            )
        )
        return
    paths = [path for path in sorted(ctx.root.glob(pattern)) if path.is_file()]
    for path in paths:
        rel = path.relative_to(ctx.root).as_posix()
        result.scanned += 1
        try:
            loaded = tomllib.loads(path.read_text(encoding="utf-8"))
        except (OSError, UnicodeError, tomllib.TOMLDecodeError) as exc:
            result.findings.append(
                Finding(
                    check,
                    _severity(step),
                    _category(step, "cargo-toml"),
                    f"unparsable Cargo.toml ({type(exc).__name__}: {exc})",
                    rel,
                )
            )
            continue
        if not isinstance(loaded, dict):
            result.findings.append(
                Finding(
                    check,
                    _severity(step),
                    _category(step, "cargo-toml"),
                    "unparsable Cargo.toml (root is not a table)",
                    rel,
                )
            )
            continue
        seen: set[str] = set()
        for table in _collect_dep_tables(loaded):
            for key, value in table.items():
                if not isinstance(key, str) or not key:
                    continue
                names, optional = _dep_names_and_optional(key, value)
                for crate_name in names:
                    action = rules.get(crate_name)
                    if action is None or crate_name in seen:
                        continue
                    if action == _RULE_REQUIRE_OPTIONAL and optional:
                        continue
                    seen.add(crate_name)
                    message = _fmt(
                        _str(step, "message", "found {needle}"),
                        needle=crate_name,
                    )
                    result.findings.append(
                        Finding(
                            check,
                            _severity(step),
                            _category(step, "cargo-dep"),
                            message,
                            rel,
                        )
                    )
