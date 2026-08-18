"""Closed set of check kinds. New *kind* = kernel change; new *check* = YAML row."""

from __future__ import annotations

import re
from collections.abc import Callable, Mapping
from pathlib import Path

from kutha_gov.protocol import CheckResult, Context, Finding, Severity

ALLOWED_KINDS: frozenset[str] = frozenset(
    {
        "file_exists",
        "file_equals",
        "file_contains",
        "file_absent",
        "concat_absent",
        "concat_contains_any",
        "glob_absent",
        "glob_none",
        "markdown_heading_tag",
        "pointer_in_other_file",
        "yaml_needles_in_glob",
    }
)

type Step = Mapping[str, object]
type Runner = Callable[[str, Step, Context, CheckResult], None]


def run_step(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    kind = _str(step, "kind")
    if kind not in ALLOWED_KINDS:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "unknown-kind",
                f"unknown kind {kind!r} (not in META allowlist) — fail-closed",
            )
        )
        return
    RUNNERS[kind](check, step, ctx, result)


def _str(step: Step, key: str, default: str = "") -> str:
    value = step.get(key, default)
    return value if isinstance(value, str) else default


def _str_list(step: Step, key: str) -> list[str]:
    value = step.get(key, [])
    if not isinstance(value, list):
        return []
    return [item for item in value if isinstance(item, str)]


def _int(step: Step, key: str, default: int) -> int:
    value = step.get(key, default)
    return value if isinstance(value, int) else default


def _severity(step: Step) -> Severity:
    raw = _str(step, "severity", "high").lower()
    return Severity.LOW if raw == "low" else Severity.HIGH


def _category(step: Step, default: str) -> str:
    return _str(step, "category", default)


def _fmt(template: str, **kwargs: object) -> str:
    try:
        return template.format(**kwargs)
    except (KeyError, IndexError, ValueError):
        return template


def _casefold(step: Step, text: str) -> str:
    if _str(step, "case", "sensitive").lower() == "lower":
        return text.lower()
    return text


def _re_flags(step: Step) -> int:
    flags = 0
    raw = step.get("flags", [])
    names = raw if isinstance(raw, list) else []
    mapping = {
        "multiline": re.MULTILINE,
        "ignorecase": re.IGNORECASE,
        "dotall": re.DOTALL,
    }
    for item in names:
        if isinstance(item, str) and item.lower() in mapping:
            flags |= mapping[item.lower()]
    return flags


def _high_missing(check: str, path: str, result: CheckResult) -> None:
    result.findings.append(Finding(check, Severity.HIGH, "missing", f"missing {path}", path))


def _kind_file_exists(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    path = _str(step, "path")
    result.scanned += 1
    if ctx.read(path) is None:
        message = _fmt(_str(step, "message", "missing {path}"), path=path)
        result.findings.append(
            Finding(check, _severity(step), _category(step, "missing"), message, path)
        )


def _kind_file_equals(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    path = _str(step, "path")
    expected = _str(step, "equals")
    result.scanned += 1
    text = ctx.read(path)
    if text is None:
        _high_missing(check, path, result)
        return
    found = text.strip()
    if found != expected:
        message = _fmt(
            _str(step, "message", "{path} must be {equals}, found {found!r}"),
            path=path,
            equals=expected,
            found=found,
        )
        result.findings.append(
            Finding(check, _severity(step), _category(step, "equals"), message, path)
        )


def _kind_file_contains(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    path = _str(step, "path")
    needles = _str_list(step, "needles")
    result.scanned += 1
    text = ctx.read(path)
    if text is None:
        _high_missing(check, path, result)
        return
    haystack = _casefold(step, text)
    folded = [_casefold(step, needle) for needle in needles]
    require = _str(step, "require", "all").lower()
    missing = [needle for needle, key in zip(needles, folded, strict=True) if key not in haystack]
    if require == "any":
        if len(missing) == len(needles):
            message = _fmt(
                _str(step, "message", "{path} matched none of the needles"),
                path=path,
                missing=", ".join(missing),
            )
            result.findings.append(
                Finding(check, _severity(step), _category(step, "contains"), message, path)
            )
        return
    if missing:
        message = _fmt(
            _str(step, "message", "{path} missing: {missing}"),
            path=path,
            missing=", ".join(missing),
        )
        result.findings.append(
            Finding(check, _severity(step), _category(step, "contains"), message, path)
        )


def _kind_file_absent(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    path = _str(step, "path")
    needles = _str_list(step, "needles")
    result.scanned += 1
    text = ctx.read(path)
    if text is None:
        return
    haystack = _casefold(step, text)
    for needle in needles:
        if _casefold(step, needle) in haystack:
            message = _fmt(
                _str(step, "message", "{path} contains forbidden {needle}"),
                path=path,
                needle=needle,
            )
            result.findings.append(
                Finding(check, _severity(step), _category(step, "absent"), message, path)
            )


def _concat(ctx: Context, paths: list[str]) -> str:
    parts: list[str] = []
    for path in paths:
        parts.append(ctx.read(path) or "")
    return "\n".join(parts)


def _kind_concat_absent(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    paths = _str_list(step, "paths")
    needles = _str_list(step, "needles")
    result.scanned += len(paths)
    blob = _casefold(step, _concat(ctx, paths))
    loc = paths[0] if paths else None
    for needle in needles:
        if _casefold(step, needle) in blob:
            message = _fmt(
                _str(step, "message", "forbidden {needle}"),
                needle=needle,
            )
            result.findings.append(
                Finding(check, _severity(step), _category(step, "absent"), message, loc)
            )


def _kind_concat_contains_any(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    paths = _str_list(step, "paths")
    needles = _str_list(step, "needles")
    result.scanned += len(paths)
    blob = _casefold(step, _concat(ctx, paths))
    loc = paths[0] if paths else None
    if any(_casefold(step, needle) in blob for needle in needles):
        return
    message = _fmt(_str(step, "message", "none of the needles found"), missing=", ".join(needles))
    result.findings.append(
        Finding(check, _severity(step), _category(step, "contains"), message, loc)
    )


def _kind_glob_absent(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    pattern = _str(step, "glob")
    needles = _str_list(step, "needles")
    for path in sorted(ctx.root.glob(pattern)):
        if not path.is_file():
            continue
        text = _casefold(step, path.read_text(encoding="utf-8"))
        result.scanned += 1
        rel = str(path.relative_to(ctx.root))
        for needle in needles:
            if _casefold(step, needle) in text:
                message = _fmt(
                    _str(step, "message", "found {needle}"),
                    needle=needle,
                )
                result.findings.append(
                    Finding(check, _severity(step), _category(step, "glob-absent"), message, rel)
                )


def _kind_glob_none(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    pattern = _str(step, "glob")
    hits = [path for path in sorted(ctx.root.glob(pattern)) if path.is_file() or path.is_dir()]
    result.scanned = len(hits)
    if not hits:
        return
    rel = str(hits[0].relative_to(ctx.root))
    message = _fmt(
        _str(step, "message", "forbidden path exists: {path}"),
        path=rel,
        n=len(hits),
    )
    result.findings.append(
        Finding(check, _severity(step), _category(step, "glob-none"), message, rel)
    )


def _kind_markdown_heading_tag(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    pattern = _str(step, "glob")
    heading = _str(step, "heading", "## Status")
    allowed = _str_list(step, "allowed")
    parent = (ctx.root / pattern).parent if pattern else ctx.root
    if not parent.is_dir():
        rel = str(parent.relative_to(ctx.root)) if parent != ctx.root else pattern
        _high_missing(check, rel, result)
        return
    tag_alt = "|".join(re.escape(tag) for tag in allowed) or "Never"
    status_re = re.compile(
        rf"^{re.escape(heading)}\s*\n+\s*\*\*(?P<tag>{tag_alt})\*\*",
        re.MULTILINE,
    )
    forbid_prefix = _str(step, "forbid_accepted_prefix")
    forbid_not = _str(step, "forbid_accepted_not_prefix")
    accepted: list[str] = []
    vocab = "|".join(allowed)
    for path in sorted(ctx.root.glob(pattern)):
        if not path.is_file():
            continue
        text = path.read_text(encoding="utf-8")
        result.scanned += 1
        rel = str(path.relative_to(ctx.root))
        match = status_re.search(text)
        if not match:
            result.findings.append(
                Finding(
                    check,
                    _severity(step),
                    _category(step, "vocabulary"),
                    f"Status tag not in {vocab}",
                    rel,
                )
            )
            continue
        tag = match.group("tag")
        if tag == "Accepted":
            accepted.append(rel)
    if forbid_prefix:
        honeycomb = [
            rel
            for rel in accepted
            if Path(rel).name.startswith(forbid_prefix)
            and (not forbid_not or not Path(rel).name.startswith(forbid_not))
        ]
        if honeycomb:
            result.findings.append(
                Finding(
                    check,
                    Severity.HIGH,
                    "bulk-accept",
                    "honeycomb Accepted without harness promotion packet: " + ", ".join(honeycomb),
                )
            )


def _kind_pointer_in_other_file(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    path = _str(step, "path")
    other = _str(step, "other")
    pattern = _str(step, "pattern")
    text = ctx.read(path)
    other_text = ctx.read(other)
    if text is None:
        _high_missing(check, path, result)
        return
    if other_text is None:
        _high_missing(check, other, result)
        return
    compiled = re.compile(pattern, _re_flags(step))
    matches = list(compiled.finditer(text))
    result.scanned = len(matches)
    label = _str(step, "label", "Active Milestone")
    if len(matches) != 1:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "active-count",
                f"STATE must declare exactly one {label}, found {len(matches)}",
                path,
            )
        )
        return
    captured = matches[0].groupdict().get("id") or matches[0].group(1)
    skip = set(_str_list(step, "skip_values"))
    if captured in skip:
        result.note = f"no {label}"
    elif captured not in other_text:
        line = text[: matches[0].start()].count("\n") + 1
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "phantom",
                f"{label} {captured} is not present in ROADMAP",
                path,
                line,
            )
        )
    inflation = _str(step, "inflation_pattern")
    if inflation:
        ids = set(re.findall(inflation, other_text))
        limit = _int(step, "inflation_max", 12)
        if len(ids) > limit:
            result.findings.append(
                Finding(
                    check,
                    Severity.LOW,
                    "inflation",
                    f"ROADMAP already names {len(ids)} M### ids — inflation risk (law-nexus lesson)",
                    other,
                )
            )


def _yaml_select(loaded: object, dotted: str) -> object:
    cur: object = loaded
    for part in dotted.split("."):
        if not isinstance(cur, dict) or part not in cur:
            return None
        cur = cur[part]
    return cur


def _kind_yaml_needles_in_glob(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    import yaml

    path = _str(step, "path")
    select = _str(step, "select")
    pattern = _str(step, "glob")
    prefix = _str(step, "prefix")
    text = ctx.read(path)
    if text is None:
        _high_missing(check, path, result)
        return
    try:
        loaded = yaml.safe_load(text)
    except yaml.YAMLError as exc:
        result.findings.append(Finding(check, Severity.HIGH, "yaml", f"{path}: {exc}", path))
        return
    names = _yaml_select(loaded, select)
    if not isinstance(names, list) or not names:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "yaml-select",
                f"{path} select {select!r} is not a non-empty list",
                path,
            )
        )
        return
    parts: list[str] = []
    for hit in sorted(ctx.root.glob(pattern)):
        if hit.is_file():
            parts.append(hit.read_text(encoding="utf-8"))
            result.scanned += 1
    blob = "\n".join(parts)
    missing = [item for item in names if isinstance(item, str) and f"{prefix}{item}" not in blob]
    if missing:
        message = _fmt(
            _str(
                step, "message", "FSM required names missing as {prefix}<name> in {glob}: {missing}"
            ),
            prefix=prefix,
            glob=pattern,
            missing=", ".join(missing),
        )
        result.findings.append(
            Finding(check, _severity(step), _category(step, "yaml-needles"), message, path)
        )


RUNNERS: dict[str, Runner] = {
    "file_exists": _kind_file_exists,
    "file_equals": _kind_file_equals,
    "file_contains": _kind_file_contains,
    "file_absent": _kind_file_absent,
    "concat_absent": _kind_concat_absent,
    "concat_contains_any": _kind_concat_contains_any,
    "glob_absent": _kind_glob_absent,
    "glob_none": _kind_glob_none,
    "markdown_heading_tag": _kind_markdown_heading_tag,
    "pointer_in_other_file": _kind_pointer_in_other_file,
    "yaml_needles_in_glob": _kind_yaml_needles_in_glob,
}
