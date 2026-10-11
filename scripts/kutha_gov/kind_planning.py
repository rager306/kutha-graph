"""Planning glob budgets and frontmatter path checks (CE #21)."""

from __future__ import annotations

import re
from pathlib import Path

from kutha_gov.kinds import (
    Step,
    _category,
    _fmt,
    _high_missing,
    _int,
    _severity,
    _str,
    _wc_lines,
    _when_glob_idle,
)
from kutha_gov.protocol import CheckResult, Context, Finding, Severity

_PHASE_DIR_RE = re.compile(r"^(\d+)-")


def _planning_phase_nn(rel: str) -> int | None:
    parts = Path(rel).as_posix().split("/")
    if len(parts) < 3 or parts[0] != ".planning" or parts[1] != "phases":
        return None
    match = _PHASE_DIR_RE.match(parts[2])
    if match is None:
        return None
    return int(match.group(1))


def _under_planning_phases(rel: str) -> bool:
    parts = Path(rel).as_posix().split("/")
    return len(parts) >= 3 and parts[0] == ".planning" and parts[1] == "phases"


def _min_phase_gate(
    check: str,
    rel: str,
    min_phase: int,
    result: CheckResult | None,
) -> str:
    """Return 'keep', 'skip', or 'bad' for a path under min_phase filtering."""
    if min_phase <= 0:
        return "keep"
    if not _under_planning_phases(rel):
        return "skip"
    nn = _planning_phase_nn(rel)
    if nn is None:
        if result is not None:
            result.findings.append(
                Finding(
                    check,
                    Severity.HIGH,
                    "min-phase",
                    f"phase dirname lacks NN- prefix: {rel}",
                    rel,
                )
            )
        return "bad"
    if nn < min_phase:
        return "skip"
    return "keep"


def _glob_max_lines_hits(
    step: Step,
    root: Path,
    *,
    check: str | None = None,
    result: CheckResult | None = None,
) -> list[Path]:
    pattern = _str(step, "glob")
    if not pattern:
        return []
    min_phase = _int(step, "min_phase", 0)
    hits: list[Path] = []
    for path in sorted(root.glob(pattern)):
        if not path.is_file():
            continue
        rel = path.relative_to(root).as_posix()
        gate = _min_phase_gate(check or "", rel, min_phase, result)
        if gate != "keep":
            continue
        hits.append(path)
    return hits


def glob_max_lines_mutation_target(step: Step, root: Path) -> tuple[str, str] | None:
    pattern = _str(step, "glob")
    maximum = _int(step, "max", -1)
    if not pattern or maximum < 0:
        return None
    hits = _glob_max_lines_hits(step, root)
    if not hits:
        return None
    rel = hits[0].relative_to(root).as_posix()
    return rel, "\n" * (maximum + 1)


def glob_max_lines_effective_min_hits(step: Step, root: Path) -> int:
    min_hits = _int(step, "min_hits", 0)
    when_glob = _str(step, "when_glob_exists")
    if when_glob and not any(path.is_file() for path in root.glob(when_glob)):
        return 0
    return min_hits


def _glob_max_lines_correspond(
    check: str,
    step: Step,
    ctx: Context,
    result: CheckResult,
    hits: list[Path],
) -> None:
    """HIGH when a min_phase-gated correspond_glob file has no peer glob hit in-dir."""
    correspond = _str(step, "correspond_glob")
    if not correspond:
        return
    marker_step: dict[str, object] = dict(step)
    marker_step["glob"] = correspond
    markers = _glob_max_lines_hits(marker_step, ctx.root, check=check, result=result)
    skip_if = _str(step, "correspond_skip_if")
    hit_parents = {path.parent for path in hits}
    missing: list[str] = []
    for marker in markers:
        if skip_if and any(path.is_file() for path in marker.parent.glob(skip_if)):
            continue
        if marker.parent in hit_parents:
            continue
        missing.append(marker.relative_to(ctx.root).as_posix())
    if not missing:
        return
    result.scanned += 1
    message = _fmt(
        _str(step, "correspond_message", "{glob} missing peer of {missing}"),
        glob=_str(step, "glob"),
        missing=", ".join(missing),
    )
    result.findings.append(
        Finding(check, Severity.HIGH, _category(step, "correspond"), message, correspond)
    )


def _kind_glob_max_lines(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    pattern = _str(step, "glob")
    maximum = _int(step, "max", -1)
    per_dir = _int(step, "max_files_per_dir", -1)
    if not pattern:
        result.scanned += 1
        result.findings.append(Finding(check, Severity.HIGH, "max-lines", "glob is required"))
        return
    if maximum < 0:
        result.scanned += 1
        result.findings.append(
            Finding(check, Severity.HIGH, "max-lines", "max must be a non-negative integer")
        )
        return
    min_hits = glob_max_lines_effective_min_hits(step, ctx.root)
    hits = _glob_max_lines_hits(step, ctx.root, check=check, result=result)
    _glob_max_lines_correspond(check, step, ctx, result, hits)
    if len(hits) < min_hits:
        result.scanned += 1
        message = _fmt(
            _str(
                step,
                "message",
                "glob {glob} matched {n} files, min_hits {min_hits}",
            ),
            glob=pattern,
            n=len(hits),
            min_hits=min_hits,
        )
        result.findings.append(
            Finding(check, Severity.HIGH, _category(step, "min-hits"), message, pattern)
        )
    counts: dict[str, int] = {}
    for path in hits:
        rel = path.relative_to(ctx.root).as_posix()
        result.scanned += 1
        text = ctx.read(rel)
        if text is None:
            _high_missing(check, rel, result)
            continue
        n = _wc_lines(text)
        if n > maximum:
            message = _fmt(
                _str(step, "message", "{path} has {n} lines (wc -l), max {max}"),
                path=rel,
                n=n,
                max=maximum,
            )
            result.findings.append(
                Finding(check, _severity(step), _category(step, "max-lines"), message, rel)
            )
        parent = Path(rel).parent.as_posix()
        counts[parent] = counts.get(parent, 0) + 1
    if per_dir < 0:
        return
    for directory in sorted(counts):
        n_files = counts[directory]
        if n_files <= per_dir:
            continue
        message = _fmt(
            "{dir} has {n} files, max {max_files_per_dir}",
            dir=directory,
            n=n_files,
            max_files_per_dir=per_dir,
        )
        result.findings.append(
            Finding(check, _severity(step), _category(step, "max-files"), message, directory)
        )


_FRONTMATTER_RE = re.compile(r"^---\s*\n(.*?)\n---\s*\n", re.DOTALL)


def _frontmatter_block(text: str) -> str | None:
    match = _FRONTMATTER_RE.match(text)
    return match.group(1) if match else None


def _frontmatter_list_field(block: str, field: str) -> list[str] | None:
    lines = block.splitlines()
    for index, line in enumerate(lines):
        if line.strip() != f"{field}:":
            continue
        items: list[str] = []
        for rest in lines[index + 1 :]:
            if rest.startswith("  - "):
                items.append(rest[4:].strip().strip("'\""))
            elif rest.strip() and not rest.startswith(" "):
                break
        return items
    return None


def frontmatter_paths_exist_mutation_target(step: Step, root: Path) -> tuple[str, str, str] | None:
    field = _str(step, "field", "read_only_inputs")
    for path in _glob_max_lines_hits(step, root):
        rel = path.relative_to(root).as_posix()
        text = path.read_text(encoding="utf-8")
        block = _frontmatter_block(text)
        if block is None:
            continue
        raw = _frontmatter_list_field(block, field)
        if not raw:
            continue
        first = next((item for item in raw if item.strip()), None)
        if not first:
            continue
        bogus = f"{first}-selftest-missing"
        new_text = text.replace(f"  - {first}", f"  - {bogus}", 1)
        if new_text == text:
            new_text = text.replace(first, bogus, 1)
        return rel, new_text, bogus
    return None


def _kind_frontmatter_paths_exist(
    check: str, step: Step, ctx: Context, result: CheckResult
) -> None:
    field = _str(step, "field", "read_only_inputs")
    if _when_glob_idle(step, ctx.root):
        return
    hits = _glob_max_lines_hits(step, ctx.root, check=check, result=result)
    if not hits:
        result.scanned += 1
        if step.get("allow_empty"):
            return
        pattern = _str(step, "glob")
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                _category(step, "glob-empty"),
                f"glob {pattern!r} matched no verification files",
                pattern,
            )
        )
        return
    for path in hits:
        rel = path.relative_to(ctx.root).as_posix()
        result.scanned += 1
        text = ctx.read(rel)
        if text is None:
            _high_missing(check, rel, result)
            continue
        block = _frontmatter_block(text)
        if block is None:
            result.findings.append(
                Finding(
                    check,
                    Severity.HIGH,
                    _category(step, "frontmatter"),
                    f"{rel} missing YAML frontmatter",
                    rel,
                )
            )
            continue
        raw = _frontmatter_list_field(block, field)
        if raw is None:
            result.findings.append(
                Finding(
                    check,
                    Severity.HIGH,
                    _category(step, "frontmatter"),
                    f"{rel} missing frontmatter key {field!r}",
                    rel,
                )
            )
            continue
        if not raw:
            result.findings.append(
                Finding(
                    check,
                    Severity.HIGH,
                    _category(step, "frontmatter"),
                    f"{rel} frontmatter key {field!r} lists no paths",
                    rel,
                )
            )
            continue
        for item in raw:
            if not item.strip():
                result.findings.append(
                    Finding(
                        check,
                        Severity.HIGH,
                        _category(step, "frontmatter-path"),
                        f"{rel} {field!r} has an empty entry",
                        rel,
                    )
                )
                continue
            rel_path = item.strip()
            if rel_path.startswith("/"):
                result.findings.append(
                    Finding(
                        check,
                        Severity.HIGH,
                        _category(step, "frontmatter-path"),
                        f"{rel} lists absolute path {rel_path!r}",
                        rel,
                    )
                )
                continue
            target = (ctx.root / rel_path).resolve()
            if not target.is_relative_to(ctx.root.resolve()):
                result.findings.append(
                    Finding(
                        check,
                        Severity.HIGH,
                        _category(step, "frontmatter-path"),
                        f"{rel} lists path outside the repo {rel_path!r}",
                        rel,
                    )
                )
                continue
            if not target.is_file():
                result.findings.append(
                    Finding(
                        check,
                        Severity.HIGH,
                        _category(step, "frontmatter-path"),
                        f"{rel} lists missing file {rel_path!r}",
                        rel,
                    )
                )
