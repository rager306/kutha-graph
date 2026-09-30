"""Closed set of check kinds. New *kind* = kernel change; new *check* = YAML row."""

from __future__ import annotations

import re
from collections.abc import Callable, Mapping
from pathlib import Path

import yaml

from kutha_gov.protocol import CheckResult, Context, Finding, Severity
from kutha_gov.rust_source import (
    DEFAULT_ASSERT_PREFIXES,
    FnItem,
    assert_hits,
    body_text,
    missing_symbols,
    parse_fns,
    strip_noise,
)

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
        "when_match_then_match",
        "yaml_needles_in_glob",
        "git_path_implies",
        "yaml_map_list",
        "glob_paths_in_file",
        "rust_test_asserts",
        "cite_equals",
        "refs_resolve",
        "file_max_lines",
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


def _path_list(step: Step, key: str) -> list[str]:
    value = step.get(key, [])
    if isinstance(value, str) and value.strip():
        return [value]
    if not isinstance(value, list):
        return []
    return [item for item in value if isinstance(item, str) and item.strip()]


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


def _kind_when_match_then_match(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    """If `pattern` matches `path`, require `then_pattern` in the same file."""
    path = _str(step, "path")
    when = _str(step, "pattern")
    then = _str(step, "then_pattern")
    text = ctx.read(path)
    if text is None:
        _high_missing(check, path, result)
        return
    result.scanned += 1
    flags = _re_flags(step)
    if not re.search(when, text, flags):
        result.note = _str(step, "skip_note", "when pattern not matched")
        return
    if re.search(then, text, flags):
        return
    message = _fmt(
        _str(step, "message", "{path} matched when-pattern but not then_pattern"),
        path=path,
    )
    result.findings.append(
        Finding(check, _severity(step), _category(step, "when-then"), message, path)
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
                f"{label} {captured} is not present in {other}",
                path,
                line,
            )
        )
    require_tpl = _str(step, "require_template")
    if require_tpl and captured not in skip:
        needle = require_tpl.format(id=captured)
        if needle not in other_text:
            line = text[: matches[0].start()].count("\n") + 1
            result.findings.append(
                Finding(
                    check,
                    Severity.HIGH,
                    "lease-drift",
                    _fmt(
                        _str(
                            step,
                            "require_message",
                            "{label} {id} is leased but ROADMAP lacks open checkbox `{needle}`",
                        ),
                        label=label,
                        id=captured,
                        needle=needle,
                    ),
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


def _kind_git_path_implies(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    from kutha_gov.gitdiff import changed_relpaths, path_matches

    when_any = _str_list(step, "when_any")
    then_any = _str_list(step, "then_any")
    if not when_any or not then_any:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "git-path-implies",
                "git_path_implies requires when_any and then_any",
            )
        )
        return
    if ctx.changed_paths is not None:
        changed = set(ctx.changed_paths)
    else:
        listed = changed_relpaths(ctx.root, ctx.git_against)
        if listed is None:
            result.note = "no git; coupling skipped"
            return
        changed = listed
    result.scanned += len(changed)
    if not any(path_matches(rel, pattern) for rel in changed for pattern in when_any):
        return
    if any(path_matches(rel, pattern) for rel in changed for pattern in then_any):
        return
    message = _fmt(
        _str(step, "message", "diff matches {when} without {then}"),
        when=", ".join(when_any),
        then=", ".join(then_any),
    )
    result.findings.append(
        Finding(check, _severity(step), _category(step, "docs-coupling"), message)
    )


def _as_str_map(value: object) -> dict[str, str]:
    if not isinstance(value, dict):
        return {}
    out: dict[str, str] = {}
    for key, item in value.items():
        if isinstance(key, str) and isinstance(item, str):
            out[key] = item
    return out


def _yaml_document(
    check: str, path: str, ctx: Context, result: CheckResult
) -> tuple[object | None, bool]:
    text = ctx.read(path)
    if text is None:
        _high_missing(check, path, result)
        return None, False
    try:
        return yaml.safe_load(text), True
    except yaml.YAMLError as exc:
        result.findings.append(Finding(check, Severity.HIGH, "yaml", f"{path}: {exc}", path))
        return None, False


def _yaml_map_list_finding(
    check: str,
    step: Step,
    category: str,
    message_template: str,
    loc: str,
    result: CheckResult,
    **fmt: object,
) -> None:
    result.findings.append(
        Finding(
            check,
            _severity(step),
            _category(step, category),
            _fmt(_str(step, "message", message_template), **fmt),
            loc,
        )
    )


def _yaml_map_rows_missing_fields(maps: list[dict], require: list[str]) -> list[str]:
    missing_rows: list[str] = []
    for row in maps:
        row_id = str(row.get("id", "?"))
        absent = [
            key
            for key in require
            if not isinstance(row.get(key), str) or not str(row.get(key)).strip()
        ]
        if absent:
            missing_rows.append(f"{row_id}:{','.join(absent)}")
    return missing_rows


def _yaml_map_duplicate_values(maps: list[dict], field: str) -> list[str]:
    seen: dict[str, int] = {}
    dups: list[str] = []
    for row in maps:
        val = row.get(field)
        if not isinstance(val, str) or not val.strip():
            continue
        seen[val] = seen.get(val, 0) + 1
        if seen[val] == 2:
            dups.append(val)
    return dups


def _yaml_map_non_allowed_values(maps: list[dict], field: str, allowed: list[str]) -> list[str]:
    return sorted({str(row.get(field)) for row in maps if str(row.get(field, "")) not in allowed})


def _yaml_map_rows_missing_list_fields(maps: list[dict], require: list[str]) -> list[str]:
    missing_rows: list[str] = []
    for row in maps:
        row_id = str(row.get("id", "?"))
        absent = [key for key in require if not isinstance(row.get(key), list)]
        if absent:
            missing_rows.append(f"{row_id}:{','.join(absent)}")
    return missing_rows


def _yaml_map_empty_list_rows(maps: list[dict], field: str) -> list[str]:
    empty: list[str] = []
    for row in maps:
        raw = row.get(field)
        if not isinstance(raw, list) or not any(
            isinstance(item, str) and item.strip() for item in raw
        ):
            empty.append(str(row.get("id", "?")))
    return empty


def _yaml_map_list_values(maps: list[dict], field: str) -> list[str]:
    values: list[str] = []
    for row in maps:
        raw = row.get(field)
        if isinstance(raw, list):
            values.extend(item.strip() for item in raw if isinstance(item, str) and item.strip())
        elif isinstance(raw, str) and raw.strip():
            values.append(raw.strip())
    return values


def _yaml_map_nonempty_field_values(maps: list[dict], field: str) -> list[str]:
    values: list[str] = []
    for row in maps:
        val = row.get(field)
        if isinstance(val, str) and val.strip():
            values.append(val.strip())
    return values


def _kind_yaml_map_list(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    """Interpret a YAML list of maps: required fields, closed vocab, unique ids, cross-file refs."""
    path = _str(step, "path")
    select = _str(step, "select")
    loaded, ok = _yaml_document(check, path, ctx, result)
    if not ok:
        return
    rows = _yaml_select(loaded, select)
    if not isinstance(rows, list):
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "yaml-select",
                f"{path} select {select!r} is not a list",
                path,
            )
        )
        return
    maps = [row for row in rows if isinstance(row, dict)]
    when = _as_str_map(step.get("when"))
    if when:
        maps = [
            row
            for row in maps
            if all(str(row.get(key, "")) == value for key, value in when.items())
        ]
    result.scanned = len(maps)

    require = _str_list(step, "require_fields")
    if require:
        missing_rows = _yaml_map_rows_missing_fields(maps, require)
        if missing_rows:
            _yaml_map_list_finding(
                check,
                step,
                "yaml-map-fields",
                "{path} rows missing fields: {missing}",
                path,
                result,
                path=path,
                missing=", ".join(missing_rows),
            )
            return

    require_lists = _str_list(step, "require_list_fields")
    if require_lists:
        missing_lists = _yaml_map_rows_missing_list_fields(maps, require_lists)
        if missing_lists:
            _yaml_map_list_finding(
                check,
                step,
                "yaml-map-fields",
                "{path} rows missing list fields: {missing}",
                path,
                result,
                path=path,
                missing=", ".join(missing_lists),
            )
            return

    nonempty_list = _str(step, "nonempty_list")
    if nonempty_list:
        empty_rows = _yaml_map_empty_list_rows(maps, nonempty_list)
        if empty_rows:
            _yaml_map_list_finding(
                check,
                step,
                "yaml-map-fields",
                "{path} rows missing {nonempty_list} items: {missing}",
                path,
                result,
                path=path,
                nonempty_list=nonempty_list,
                missing=", ".join(empty_rows),
            )
            return

    unique = _str(step, "unique")
    if unique:
        dups = _yaml_map_duplicate_values(maps, unique)
        if dups:
            _yaml_map_list_finding(
                check,
                step,
                "yaml-map-unique",
                "{path} duplicate {unique}: {missing}",
                path,
                result,
                path=path,
                unique=unique,
                missing=", ".join(dups),
            )
            return

    field = _str(step, "field")
    prefix = _str(step, "prefix")
    allowed = _str_list(step, "allowed")
    if field and allowed:
        bad = _yaml_map_non_allowed_values(maps, field, allowed)
        if bad:
            _yaml_map_list_finding(
                check,
                step,
                "yaml-map-vocab",
                "{path} field {field} not in allowlist: {missing}",
                path,
                result,
                path=path,
                field=field,
                missing=", ".join(bad),
            )
            return

    if field and step.get("must_exist") is True:
        missing_files = sorted(
            {loc for loc in _yaml_map_nonempty_field_values(maps, field) if ctx.read(loc) is None}
        )
        if missing_files:
            _yaml_map_list_finding(
                check,
                step,
                "yaml-map-missing",
                "{path} {field} paths do not exist: {missing}",
                path,
                result,
                path=path,
                field=field,
                missing=", ".join(missing_files),
            )
            return

    in_path_field = _str(step, "in_path_field")
    if field and in_path_field:
        suffix = _str(step, "suffix")
        missing_embed: list[str] = []
        for row in maps:
            loc = row.get(in_path_field)
            val = row.get(field)
            if not isinstance(loc, str) or not loc.strip() or not isinstance(val, str):
                continue
            text = ctx.read(loc.strip())
            needle = f"{prefix}{val}{suffix}"
            if text is None or needle not in text:
                missing_embed.append(f"{row.get('id', '?')}:{val}")
        if missing_embed:
            _yaml_map_list_finding(
                check,
                step,
                "yaml-map-embed",
                "{path} {field} missing from {in_path_field} files: {missing}",
                path,
                result,
                path=path,
                field=field,
                in_path_field=in_path_field,
                missing=", ".join(missing_embed),
            )
            return

    other_paths = _path_list(step, "other")
    list_field = _str(step, "list_field")
    glob_pattern = _str(step, "glob")
    ref_field = list_field or field
    if ref_field and (other_paths or glob_pattern):
        parts: list[str] = []
        if glob_pattern:
            for hit in sorted(ctx.root.glob(glob_pattern)):
                if not hit.is_file():
                    continue
                parts.append(hit.read_text(encoding="utf-8"))
                result.scanned += 1
            other_label = glob_pattern
        else:
            for other_path in other_paths:
                other_text = ctx.read(other_path)
                if other_text is None:
                    _high_missing(check, other_path, result)
                    return
                parts.append(other_text)
            other_label = ", ".join(other_paths)
        haystack = "\n".join(parts)
        values = (
            _yaml_map_list_values(maps, ref_field)
            if list_field
            else _yaml_map_nonempty_field_values(maps, ref_field)
        )
        if step.get("absent_other") is True:
            hits = [val for val in values if f"{prefix}{val}" in haystack]
            if hits:
                _yaml_map_list_finding(
                    check,
                    step,
                    "yaml-map-overlap",
                    "{path} {field} also in {other}: {missing}",
                    path,
                    result,
                    path=path,
                    field=ref_field,
                    other=other_label,
                    missing=", ".join(hits),
                )
            return
        missing = [val for val in values if f"{prefix}{val}" not in haystack]
        if missing:
            _yaml_map_list_finding(
                check,
                step,
                "yaml-map-ref",
                "{path} {field} missing in {other}: {missing}",
                path,
                result,
                path=path,
                field=ref_field,
                other=other_label,
                missing=", ".join(missing),
            )


def _kind_glob_paths_in_file(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    pattern = _str(step, "glob")
    path = _str(step, "path")
    prefix = _str(step, "prefix")
    text = ctx.read(path)
    if text is None:
        _high_missing(check, path, result)
        return
    missing: list[str] = []
    for hit in sorted(ctx.root.glob(pattern)):
        if not hit.is_file():
            continue
        rel = str(hit.relative_to(ctx.root)).replace("\\", "/")
        result.scanned += 1
        if f"{prefix}{rel}" not in text:
            missing.append(rel)
    if missing:
        message = _fmt(
            _str(step, "message", "{glob} paths missing from {path}: {missing}"),
            glob=pattern,
            path=path,
            missing=", ".join(missing),
        )
        result.findings.append(
            Finding(check, _severity(step), _category(step, "glob-paths"), message, path)
        )


def _as_str_list_map(value: object) -> dict[str, list[str]]:
    if not isinstance(value, dict):
        return {}
    out: dict[str, list[str]] = {}
    for key, item in value.items():
        if not isinstance(key, str):
            continue
        if isinstance(item, list):
            out[key] = [entry for entry in item if isinstance(entry, str) and entry.strip()]
        elif isinstance(item, str) and item.strip():
            out[key] = [item]
    return out


def _as_require_when(value: object) -> tuple[str, str] | None:
    if not isinstance(value, dict):
        return None
    field = value.get("field")
    equals = value.get("equals")
    if isinstance(field, str) and field.strip() and isinstance(equals, str) and equals.strip():
        return field.strip(), equals.strip()
    return None


def _unique_names(names: list[str]) -> list[str]:
    seen: set[str] = set()
    out: list[str] = []
    for name in names:
        if name in seen:
            continue
        seen.add(name)
        out.append(name)
    return out


def _names_from_yaml_rows(
    check: str,
    step: Step,
    ctx: Context,
    result: CheckResult,
) -> list[str] | None:
    spec = step.get("names_from_yaml")
    if not isinstance(spec, dict):
        return _str_list(step, "tests")
    path = spec.get("path")
    select = spec.get("select")
    if not isinstance(path, str) or not path.strip() or not isinstance(select, str) or not select:
        result.findings.append(
            Finding(
                check,
                _severity(step),
                _category(step, "yaml-select"),
                "names_from_yaml requires path and select",
            )
        )
        return None
    loaded, ok = _yaml_document(check, path, ctx, result)
    if not ok:
        return None
    selected = _yaml_select(loaded, select)
    if not isinstance(selected, list) or not selected:
        result.findings.append(
            Finding(
                check,
                _severity(step),
                _category(step, "yaml-select"),
                f"{path} select {select!r} is not a non-empty list",
                path,
            )
        )
        return None
    field = spec.get("field")
    field_name = field.strip() if isinstance(field, str) and field.strip() else ""
    require_when = _as_require_when(step.get("require_evidence_when"))
    names: list[str] = []
    empty_named: list[str] = []
    string_rows = [item for item in selected if isinstance(item, str) and item.strip()]
    maps = [item for item in selected if isinstance(item, dict)]
    if field_name:
        evidence_key = field_name
        for row in maps:
            values = row.get(evidence_key)
            entries: list[str] = []
            if isinstance(values, list):
                entries = [item for item in values if isinstance(item, str) and item.strip()]
            elif isinstance(values, str) and values.strip():
                entries = [values.strip()]
            names.extend(entries)
            if require_when is not None:
                when_field, equals = require_when
                if str(row.get(when_field, "")) == equals and not entries:
                    empty_named.append(str(row.get("id", "?")))
    else:
        if string_rows and not maps:
            names.extend(string_rows)
        else:
            result.findings.append(
                Finding(
                    check,
                    _severity(step),
                    _category(step, "yaml-select"),
                    f"{path} select {select!r} needs field: for map rows",
                    path,
                )
            )
            return None
        if require_when is not None:
            result.findings.append(
                Finding(
                    check,
                    _severity(step),
                    _category(step, "yaml-select"),
                    "require_evidence_when needs names_from_yaml field on map rows",
                    path,
                )
            )
            return None
    if empty_named:
        message = _fmt(
            _str(
                step,
                "message",
                "capability named cells missing evidence tests: {missing}",
            ),
            missing=", ".join(empty_named),
        )
        result.findings.append(
            Finding(
                check,
                _severity(step),
                _category(step, "rust-test-evidence"),
                message,
                path,
            )
        )
    return _unique_names(names)


def _rust_search_files(
    check: str, step: Step, ctx: Context, result: CheckResult
) -> list[Path] | None:
    spec = step.get("names_from_yaml")
    glob = _str(step, "glob")
    path = _str(step, "path")
    if isinstance(spec, dict):
        mapped = spec.get("glob")
        if isinstance(mapped, str) and mapped.strip():
            glob = mapped.strip()
        mapped_path = spec.get("files")
        if isinstance(mapped_path, str) and mapped_path.strip() and not glob:
            path = mapped_path.strip()
    if path:
        hit = ctx.root / path
        if not hit.is_file():
            _high_missing(check, path, result)
            return None
        return [hit]
    if not glob:
        result.findings.append(
            Finding(
                check,
                _severity(step),
                _category(step, "rust-test-glob"),
                "rust_test_asserts requires path or glob",
            )
        )
        return None
    hits = [hit for hit in sorted(ctx.root.glob(glob)) if hit.is_file()]
    if not hits:
        result.findings.append(
            Finding(
                check,
                _severity(step),
                _category(step, "rust-test-glob"),
                _fmt(
                    _str(step, "message", "{glob} matched no rust files"),
                    glob=glob,
                ),
                glob,
            )
        )
        return None
    return hits


def _prefixes(step: Step) -> tuple[str, ...]:
    raw = step.get("assert_prefixes")
    if isinstance(raw, list):
        values = tuple(item for item in raw if isinstance(item, str) and item)
        if values:
            return values
    if isinstance(raw, str) and raw:
        return (raw,)
    return DEFAULT_ASSERT_PREFIXES


def _rel_of(ctx: Context, path: Path) -> str:
    try:
        return str(path.relative_to(ctx.root)).replace("\\", "/")
    except ValueError:
        return str(path)


def _kind_rust_test_asserts(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    names = _names_from_yaml_rows(check, step, ctx, result)
    if names is None:
        return
    files = _rust_search_files(check, step, ctx, result)
    if files is None:
        return
    if not names:
        result.findings.append(
            Finding(
                check,
                _severity(step),
                _category(step, "rust-test-names"),
                _str(step, "message", "no test names to prove"),
            )
        )
        return
    prefixes = _prefixes(step)
    require_symbols = _as_str_list_map(step.get("require_symbols"))
    parsed: list[tuple[Path, str, str]] = []
    for path in files:
        src = path.read_text(encoding="utf-8")
        parsed.append((path, src, strip_noise(src)))
        result.scanned += 1
    for name in names:
        _prove_named_test(
            check,
            step,
            ctx,
            result,
            name,
            parsed,
            prefixes,
            require_symbols.get(name, []),
        )


def _prove_named_test(
    check: str,
    step: Step,
    ctx: Context,
    result: CheckResult,
    name: str,
    parsed: list[tuple[Path, str, str]],
    prefixes: tuple[str, ...],
    symbols: list[str],
) -> None:
    matching: list[tuple[Path, str, FnItem, str]] = []
    for path, src, stripped in parsed:
        for item in parse_fns(src):
            if item.name != name:
                continue
            matching.append((path, src, item, body_text(stripped, item)))
    if not matching:
        result.findings.append(
            Finding(
                check,
                _severity(step),
                _category(step, "rust-test-missing"),
                _fmt(
                    _str(step, "message", "{name} is not a fn in the rust glob"),
                    name=name,
                    missing=name,
                ),
            )
        )
        return
    ignored = [row for row in matching if row[2].has_ignore]
    async_tests = [row for row in matching if row[2].async_fn]
    nested = [row for row in matching if row[2].nested]
    qualifying = [
        row
        for row in matching
        if row[2].has_test and not row[2].has_ignore and not row[2].async_fn and not row[2].nested
    ]
    if not qualifying:
        if ignored:
            path, src, item, _body = ignored[0]
            result.findings.append(
                Finding(
                    check,
                    _severity(step),
                    _category(step, "rust-test-ignored"),
                    _fmt(_str(step, "message", "{name} is #[ignore]"), name=name, missing=name),
                    _rel_of(ctx, path),
                    item.line(src),
                )
            )
            return
        if async_tests:
            path, src, item, _body = async_tests[0]
            result.findings.append(
                Finding(
                    check,
                    _severity(step),
                    _category(step, "rust-test-async"),
                    _fmt(_str(step, "message", "{name} is async fn"), name=name, missing=name),
                    _rel_of(ctx, path),
                    item.line(src),
                )
            )
            return
        if nested:
            path, src, item, _body = nested[0]
            result.findings.append(
                Finding(
                    check,
                    _severity(step),
                    _category(step, "rust-test-nested"),
                    _fmt(_str(step, "message", "{name} is a nested fn"), name=name, missing=name),
                    _rel_of(ctx, path),
                    item.line(src),
                )
            )
            return
        path, src, item, _body = matching[0]
        result.findings.append(
            Finding(
                check,
                _severity(step),
                _category(step, "rust-test-not-test"),
                _fmt(
                    _str(step, "message", "{name} is not marked #[test]"),
                    name=name,
                    missing=name,
                ),
                _rel_of(ctx, path),
                item.line(src),
            )
        )
        return
    for path, src, item, body in qualifying:
        if not assert_hits(body, prefixes):
            result.findings.append(
                Finding(
                    check,
                    _severity(step),
                    _category(step, "rust-test-vacuous"),
                    _fmt(
                        _str(step, "message", "{name} has no allowed assert in body"),
                        name=name,
                        missing=name,
                    ),
                    _rel_of(ctx, path),
                    item.line(src),
                )
            )
            return
        absent = missing_symbols(body, symbols)
        if absent:
            result.findings.append(
                Finding(
                    check,
                    _severity(step),
                    _category(step, "rust-test-symbol"),
                    _fmt(
                        _str(step, "message", "{name} missing symbols: {missing}"),
                        name=name,
                        missing=", ".join(absent),
                    ),
                    _rel_of(ctx, path),
                    item.line(src),
                )
            )
            return


def rust_test_mutation_targets(step: Step, root: Path) -> list[tuple[str, str]]:
    """(relpath, test_name) pairs for derived selftest mutations. Read-only on `root`."""
    ctx = Context(root=root)
    probe = CheckResult(check="_selftest")
    names = _names_from_yaml_rows("_selftest", step, ctx, probe)
    files = _rust_search_files("_selftest", step, ctx, probe)
    if not names or not files:
        return []
    wanted = set(names)
    out: list[tuple[str, str]] = []
    for path in files:
        rel = str(path.relative_to(root)).replace("\\", "/")
        src = path.read_text(encoding="utf-8")
        seen: set[str] = set()
        for item in parse_fns(src):
            if (
                item.name in wanted
                and item.name not in seen
                and item.has_test
                and not item.has_ignore
                and not item.async_fn
                and not item.nested
            ):
                seen.add(item.name)
                out.append((rel, item.name))
    return out


def _as_map(value: object) -> dict[str, object]:
    return value if isinstance(value, dict) else {}


def _as_maps(value: object) -> list[dict[str, object]]:
    if not isinstance(value, list):
        return []
    return [item for item in value if isinstance(item, dict)]


def _relpath(path: Path, ctx: Context) -> str:
    return str(path.relative_to(ctx.root)).replace("\\", "/")


def _compile_one_group(
    check: str,
    pattern: str,
    flags: int,
    result: CheckResult,
    loc: str | None,
) -> re.Pattern[str] | None:
    if not pattern.strip():
        result.findings.append(
            Finding(check, Severity.HIGH, "cite-pattern", "pattern is required", loc)
        )
        return None
    try:
        compiled = re.compile(pattern, flags)
    except re.error as exc:
        result.findings.append(
            Finding(check, Severity.HIGH, "cite-pattern", f"invalid pattern: {exc}", loc)
        )
        return None
    if compiled.groups != 1:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "cite-pattern",
                f"pattern must have exactly one capturing group, has {compiled.groups}",
                loc,
            )
        )
        return None
    return compiled


def _spec_flags(spec: Mapping[str, object], step: Step) -> int:
    if "flags" in spec:
        return _re_flags(spec)
    return _re_flags(step)


def _cite_files(item: Mapping[str, object], ctx: Context) -> list[Path]:
    path = item.get("path")
    if isinstance(path, str) and path.strip():
        hit = ctx.root / path
        return [hit] if hit.is_file() else []
    glob = item.get("glob")
    if not isinstance(glob, str) or not glob.strip():
        return []
    return [hit for hit in sorted(ctx.root.glob(glob)) if hit.is_file()]


def cite_equals_mutation_target(step: Step, root: Path) -> tuple[str, str] | None:
    """First existing cite file plus a concrete wrong token line (D-C1)."""
    ctx = Context(root=root)
    for item in _as_maps(step.get("cites")):
        files = _cite_files(item, ctx)
        if not files:
            continue
        pattern = item.get("pattern")
        text = _wrong_cite_line(pattern if isinstance(pattern, str) else "")
        return _relpath(files[0], ctx), text
    return None


def _wrong_cite_line(pattern: str) -> str:
    if "L_delivery=" in pattern:
        return "\nL_delivery=WRONG\n"
    if "L_map=" in pattern:
        return "\nL_map=WRONG\n"
    if "L_capability=" in pattern:
        return "\nL_capability=WRONG\n"
    if "Active Milestone" in pattern or r"M\d" in pattern:
        return "\nActive Milestone: M999\n"
    return "\nL_delivery=WRONG\n"


def _kind_cite_equals(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    source = _as_map(step.get("source"))
    src_path = source.get("path")
    src_rel = src_path if isinstance(src_path, str) else ""
    result.scanned += 1
    if not src_rel:
        result.findings.append(
            Finding(check, Severity.HIGH, "cite-source", "source.path is required")
        )
        return
    src_text = ctx.read(src_rel)
    if src_text is None:
        _high_missing(check, src_rel, result)
        return
    src_pattern = source.get("pattern")
    compiled = _compile_one_group(
        check,
        src_pattern if isinstance(src_pattern, str) else "",
        _spec_flags(source, step),
        result,
        src_rel,
    )
    if compiled is None:
        return
    found = [match.group(1) for match in compiled.finditer(src_text)]
    if not found:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "cite-source",
                f"{src_rel} matched no source capture — fail-closed",
                src_rel,
            )
        )
        return
    if len(set(found)) != 1:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "cite-source",
                f"{src_rel} source captures disagree: {found}",
                src_rel,
            )
        )
        return
    expected = found[0]
    cites = _as_maps(step.get("cites"))
    for item in cites:
        cite_pattern = item.get("pattern")
        cite_re = _compile_one_group(
            check,
            cite_pattern if isinstance(cite_pattern, str) else "",
            _spec_flags(item, step),
            result,
            src_rel,
        )
        if cite_re is None:
            return
        for path in _cite_files(item, ctx):
            rel = _relpath(path, ctx)
            result.scanned += 1
            text = path.read_text(encoding="utf-8")
            for match in cite_re.finditer(text):
                got = match.group(1)
                if got == expected:
                    continue
                message = _fmt(
                    _str(
                        step,
                        "message",
                        "{path} cites {got!r} but {source} is {expected!r}",
                    ),
                    path=rel,
                    got=got,
                    source=src_rel,
                    expected=expected,
                )
                result.findings.append(
                    Finding(
                        check,
                        _severity(step),
                        _category(step, "cite-mismatch"),
                        message,
                        rel,
                    )
                )


_EN_DASH = "\u2013"
ADR_RANGE_RE = re.compile(rf"ADR-(\d{{3}})\s*[{_EN_DASH}-]\s*(\d{{3}})(?!\d)")
ADR_ONE_RE = re.compile(r"ADR-(\d{3})(?!\d)")
LOCK_RE = re.compile(r"\bD([1-9]|10)\b")
HONEYCOMB_LOCKS = ".kutha/dictionaries/honeycomb.yaml"


def _refs_scan_files(step: Step, ctx: Context) -> list[Path]:
    seen: set[str] = set()
    out: list[Path] = []
    for rel in _path_list(step, "paths"):
        hit = ctx.root / rel
        if hit.is_file():
            key = _relpath(hit, ctx)
            if key not in seen:
                seen.add(key)
                out.append(hit)
    for pattern in _path_list(step, "globs"):
        for hit in sorted(ctx.root.glob(pattern)):
            if not hit.is_file():
                continue
            key = _relpath(hit, ctx)
            if key not in seen:
                seen.add(key)
                out.append(hit)
    return out


def refs_resolve_mutation_target(step: Step, root: Path) -> str | None:
    ctx = Context(root=root)
    files = _refs_scan_files(step, ctx)
    if not files:
        return None
    return _relpath(files[0], ctx)


def _allow_dangling(step: Step) -> set[str]:
    out: set[str] = set()
    for raw in _str_list(step, "allow_dangling"):
        text = raw.strip().upper()
        if text.startswith("ADR-"):
            text = text[4:]
        if re.fullmatch(r"\d{3}", text):
            out.add(text)
    return out


def _extract_adr_ids(text: str) -> list[str]:
    found: set[str] = set()
    for match in ADR_RANGE_RE.finditer(text):
        found.add(match.group(1))
        found.add(match.group(2))
    for match in ADR_ONE_RE.finditer(text):
        found.add(match.group(1))
    return sorted(found)


def _extract_lock_ids(text: str) -> list[str]:
    return sorted({f"D{match.group(1)}" for match in LOCK_RE.finditer(text)})


def _honeycomb_lock_ids(ctx: Context) -> set[str] | None:
    raw = ctx.read(HONEYCOMB_LOCKS)
    if raw is None:
        return None
    data = yaml.safe_load(raw)
    if not isinstance(data, dict):
        return set()
    locks = data.get("locks")
    if not isinstance(locks, list):
        return set()
    ids: set[str] = set()
    for row in locks:
        if not isinstance(row, dict):
            continue
        lock_id = row.get("id")
        if isinstance(lock_id, str) and lock_id.strip():
            ids.add(lock_id.strip())
    return ids


def _adr_file_exists(ctx: Context, num: str) -> bool:
    return any(hit.is_file() for hit in ctx.root.glob(f"docs/ADR/ADR-{num}-*.md"))


def _kind_refs_resolve(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    files = _refs_scan_files(step, ctx)
    allowed = _allow_dangling(step)
    lock_ids = _honeycomb_lock_ids(ctx)
    for path in files:
        rel = _relpath(path, ctx)
        text = path.read_text(encoding="utf-8")
        result.scanned += 1
        for num in _extract_adr_ids(text):
            if num in allowed or _adr_file_exists(ctx, num):
                continue
            message = _fmt(
                _str(step, "message", "{path} cites ADR-{num} with no matching ADR file"),
                path=rel,
                num=num,
            )
            result.findings.append(
                Finding(
                    check,
                    _severity(step),
                    _category(step, "refs-adr"),
                    message,
                    rel,
                )
            )
        for lock_id in _extract_lock_ids(text):
            if lock_ids is None:
                _high_missing(check, HONEYCOMB_LOCKS, result)
                lock_ids = set()
                continue
            if lock_id in lock_ids:
                continue
            message = _fmt(
                _str(step, "message", "{path} cites {lock} which is not a honeycomb lock"),
                path=rel,
                lock=lock_id,
            )
            result.findings.append(
                Finding(
                    check,
                    _severity(step),
                    _category(step, "refs-lock"),
                    message,
                    rel,
                )
            )


def file_max_lines_mutation_target(step: Step, root: Path) -> tuple[str, str] | None:
    path = _str(step, "path")
    maximum = _int(step, "max", -1)
    if not path or maximum < 0:
        return None
    if not (root / path).is_file():
        return None
    return path, "\n" * (maximum + 1)


def _wc_lines(text: str) -> int:
    return text.count("\n")


def _kind_file_max_lines(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    path = _str(step, "path")
    maximum = _int(step, "max", -1)
    result.scanned += 1
    if not path:
        result.findings.append(Finding(check, Severity.HIGH, "max-lines", "path is required"))
        return
    if maximum < 0:
        result.findings.append(
            Finding(check, Severity.HIGH, "max-lines", "max must be a non-negative integer")
        )
        return
    text = ctx.read(path)
    if text is None:
        _high_missing(check, path, result)
        return
    n = _wc_lines(text)
    if n <= maximum:
        return
    message = _fmt(
        _str(step, "message", "{path} has {n} lines (wc -l), max {max}"),
        path=path,
        n=n,
        max=maximum,
    )
    result.findings.append(
        Finding(check, _severity(step), _category(step, "max-lines"), message, path)
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
    "when_match_then_match": _kind_when_match_then_match,
    "yaml_needles_in_glob": _kind_yaml_needles_in_glob,
    "git_path_implies": _kind_git_path_implies,
    "yaml_map_list": _kind_yaml_map_list,
    "glob_paths_in_file": _kind_glob_paths_in_file,
    "rust_test_asserts": _kind_rust_test_asserts,
    "cite_equals": _kind_cite_equals,
    "refs_resolve": _kind_refs_resolve,
    "file_max_lines": _kind_file_max_lines,
}
