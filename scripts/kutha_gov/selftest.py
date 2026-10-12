"""Prove dictionary checks can fail. Mutations apply only under a tempfile copy (D-V1)."""

from __future__ import annotations

import multiprocessing
import re
import shutil
import tempfile
from collections.abc import Mapping
from concurrent.futures import ProcessPoolExecutor, as_completed
from dataclasses import dataclass
from pathlib import Path
from tempfile import TemporaryDirectory

from kutha_gov.dictionary import DictCheck, get_checks
from kutha_gov.protocol import Check, CheckResult, Context, Finding, Severity

type Step = Mapping[str, object]
type Mutation = Mapping[str, object]

OK = "OK"
SKIP = "SKIP"
UNPROVEN = "UNPROVEN"
VACUOUS = "VACUOUS"
BASELINE_FAIL = "BASELINE-FAIL"

CLASS_DERIVED = "derived-by-own-mutation"
CLASS_EXPLICIT_MUTATION = "explicit-mutation"
CLASS_EXPLICIT_SKIP = "explicit-skip"
CLASS_FILE_EXISTS_ONLY = "file-exists-only"
CLASS_UNPROVEN = "unproven"
SUCCESS = frozenset({OK, SKIP})
MUTATION_OPS = frozenset({"delete_file", "remove_text", "append_text", "replace_regex"})
DERIVABLE_KINDS = frozenset(
    {
        "file_exists",
        "file_contains",
        "file_absent",
        "concat_absent",
        "glob_absent",
        "file_equals",
        "concat_contains_any",
        "rust_test_asserts",
        "cite_equals",
        "refs_resolve",
        "file_max_lines",
        "glob_max_lines",
        "frontmatter_paths_exist",
        "cargo_dep_constraint",
        "test_matrix",
        "yaml_map_list",
        "pointer_in_other_file",
        "markdown_heading_tag",
        "glob_none",
        "glob_paths_in_file",
        "when_match_then_match",
        "disk_free",
    }
)
SKIP_DIR_NAMES = frozenset(
    {
        ".git",
        "target",
        ".venv",
        "node_modules",
        "__pycache__",
        ".pytest_cache",
        ".ruff_cache",
        ".claude",
        ".compound-engineering",
    }
)
SKIP_CURSOR_CHILDREN = frozenset({"gsd-core", "skills"})
SKIP_PLANNING_CHILDREN = frozenset({"milestones"})
SCRATCH_DIR = "tmp"
WORKER_ERROR = "WORKER-ERROR"


@dataclass(frozen=True)
class SelftestRow:
    status: str
    check_id: str
    mutations: int
    reason: str = ""
    classification: str = ""


def format_row(row: SelftestRow) -> str:
    line = f"{row.status} {row.check_id} mutations={row.mutations}"
    if row.reason:
        line += f" reason={row.reason}"
    if row.classification:
        line += f" class={row.classification}"
    return line


def copy_harness_tree(src: Path, dst: Path) -> None:
    """Copy `src` to `dst`, dropping D-V1 heavy trees. Never writes `src`."""
    src_root = src.resolve()

    def ignore(directory: str, names: list[str]) -> set[str]:
        dropped = {name for name in names if _skip_name(name)}
        try:
            rel = Path(directory).resolve().relative_to(src_root)
        except ValueError:
            return dropped
        if rel == Path(".cursor"):
            dropped.update(name for name in names if name in SKIP_CURSOR_CHILDREN)
        if rel == Path(".planning"):
            dropped.update(name for name in names if name in SKIP_PLANNING_CHILDREN)
        if rel == Path("."):
            dropped.update(name for name in names if name == SCRATCH_DIR)
        return dropped

    shutil.copytree(src_root, dst, ignore=ignore, symlinks=True, ignore_dangling_symlinks=True)


def run_selftest(
    root: Path, check_id: str | None = None, *, jobs: int = 1
) -> tuple[list[SelftestRow], int]:
    """Copy the tree once (jobs=1) or per check (jobs>=2). Never writes `root`."""
    checks = get_checks(root)
    if check_id is not None:
        if check_id not in checks:
            return [], 2
        selected = {check_id: checks[check_id]}
    else:
        selected = checks
    if jobs <= 1 or len(selected) <= 1:
        rows = _run_selftest_serial(root, selected)
    else:
        rows = _run_selftest_parallel(root, selected, jobs)
    code = 0 if rows and all(_success(row) for row in rows) else 1
    if not rows:
        code = 1
    return rows, code


def _run_selftest_serial(root: Path, selected: dict[str, Check]) -> list[SelftestRow]:
    rows: list[SelftestRow] = []
    scratch = root / SCRATCH_DIR
    scratch.mkdir(exist_ok=True)
    with TemporaryDirectory(prefix="kutha-selftest-", dir=scratch) as raw:
        work = Path(raw) / "tree"
        copy_harness_tree(root, work)
        work_checks = get_checks(work)
        for name in sorted(selected):
            probe = work_checks.get(name)
            if probe is None:
                probe = selected[name]
            rows.append(_row_for_probe(probe, work))
    return rows


def _eval_selftest_worker(payload: tuple[str, str, str]) -> SelftestRow:
    check_id, _root_str, template_str = payload
    template = Path(template_str)
    scratch_parent = template.parent
    scratch_parent.mkdir(parents=True, exist_ok=True)
    raw = tempfile.mkdtemp(prefix="kutha-selftest-", dir=str(scratch_parent))
    try:
        work = Path(raw) / "tree"
        copy_harness_tree(template, work)
        work_checks = get_checks(work)
        probe = work_checks.get(check_id)
        if probe is None:
            return SelftestRow(UNPROVEN, check_id, 0, "check missing on copy")
        return _row_for_probe(probe, work)
    finally:
        shutil.rmtree(raw, ignore_errors=True)


def _run_selftest_parallel(root: Path, selected: dict[str, Check], jobs: int) -> list[SelftestRow]:
    names = sorted(selected)
    scratch = root / SCRATCH_DIR
    scratch.mkdir(exist_ok=True)
    template_dir = tempfile.mkdtemp(prefix="kutha-selftest-template-", dir=str(scratch))
    by_name: dict[str, SelftestRow | None] = dict.fromkeys(names)
    try:
        template = Path(template_dir) / "tree"
        copy_harness_tree(root, template)
        workers = max(1, min(jobs, len(names)))
        ctx = multiprocessing.get_context("spawn")
        with ProcessPoolExecutor(max_workers=workers, mp_context=ctx) as pool:
            futs = {
                pool.submit(_eval_selftest_worker, (name, str(root), str(template))): name
                for name in names
            }
            try:
                for fut in as_completed(futs):
                    name = futs[fut]
                    try:
                        by_name[name] = fut.result()
                    except Exception as exc:
                        by_name[name] = SelftestRow(
                            WORKER_ERROR,
                            name,
                            0,
                            f"{type(exc).__name__}: {exc}",
                        )
            except Exception:
                leftover = {name: selected[name] for name in names if by_name[name] is None}
                if leftover:
                    for row in _run_selftest_serial(root, leftover):
                        by_name[row.check_id] = row
    finally:
        shutil.rmtree(template_dir, ignore_errors=True)
    rows: list[SelftestRow] = []
    for name in names:
        got = by_name.get(name)
        if got is None:
            got = SelftestRow(WORKER_ERROR, name, 0, "missing worker result")
        rows.append(got)
    return rows


def _row_for_probe(probe: Check, work: Path) -> SelftestRow:
    proved = _prove_check(probe, work)
    cls, cls_reason = classify_check(probe, work)
    reason = proved.reason
    if cls == CLASS_FILE_EXISTS_ONLY and not reason:
        reason = cls_reason
    return SelftestRow(
        proved.status,
        proved.check_id,
        proved.mutations,
        reason,
        cls,
    )


def _success(row: SelftestRow) -> bool:
    if row.status == OK:
        return True
    return row.status == SKIP and bool(row.reason)


def classify_check(check: Check, work: Path) -> tuple[str, str]:
    """Classify how selftest proves a dictionary check. git_path_implies stays skip."""
    if not isinstance(check, DictCheck):
        return CLASS_UNPROVEN, "not a dictionary check"
    spec = check.spec
    raw_selftest = spec.get("selftest")
    skip_reason = ""
    if isinstance(raw_selftest, dict):
        skip = raw_selftest.get("skip")
        skip_reason = skip.strip() if isinstance(skip, str) and skip.strip() else ""
        unless_glob = raw_selftest.get("skip_unless_glob")
        live = False
        if isinstance(unless_glob, str) and unless_glob.strip():
            live = any(path.is_file() for path in work.glob(unless_glob))
        if skip_reason and not live:
            return CLASS_EXPLICIT_SKIP, skip_reason
        if "mutations" in raw_selftest:
            return CLASS_EXPLICIT_MUTATION, ""
        if skip_reason and live:
            pass
        else:
            return CLASS_UNPROVEN, "selftest mapping has neither mutations nor skip"
    elif raw_selftest is not None:
        return CLASS_UNPROVEN, "selftest must be a mapping"
    high_kinds = [
        _str(step, "kind")
        for step in check.steps
        if _str(step, "severity", "high").lower() != "low"
    ]
    derivable = [kind for kind in high_kinds if kind in DERIVABLE_KINDS]
    if not _any_derivable(check.steps):
        return CLASS_UNPROVEN, "no derivable step and no selftest declaration"
    if derivable and set(derivable) <= {"file_exists"}:
        return CLASS_FILE_EXISTS_ONLY, "derived only by deleting file_exists paths"
    return CLASS_DERIVED, ""


def _skip_name(name: str) -> bool:
    return name in SKIP_DIR_NAMES or name.startswith(".cbm")


def _prove_check(check: Check, work: Path) -> SelftestRow:
    if not isinstance(check, DictCheck):
        return SelftestRow(BASELINE_FAIL, check.name, 0, "dictionary load error")
    spec = check.spec
    raw_selftest = spec.get("selftest")
    if isinstance(raw_selftest, dict):
        skip = raw_selftest.get("skip")
        skip_reason = skip.strip() if isinstance(skip, str) and skip.strip() else ""
        unless_glob = raw_selftest.get("skip_unless_glob")
        live = False
        if isinstance(unless_glob, str) and unless_glob.strip():
            live = any(path.is_file() for path in work.glob(unless_glob))
        if skip_reason and not live:
            return SelftestRow(SKIP, check.name, 0, skip_reason)
        declared = raw_selftest.get("mutations")
        if "mutations" in raw_selftest:
            mutations, error = _parse_mutations(declared)
            if error:
                return SelftestRow(UNPROVEN, check.name, 0, error)
            return _run_mutations(check, work, mutations)
        if skip_reason and live:
            pass
        else:
            return SelftestRow(
                UNPROVEN, check.name, 0, "selftest mapping has neither mutations nor skip"
            )
    elif raw_selftest is not None:
        return SelftestRow(UNPROVEN, check.name, 0, "selftest must be a mapping")
    if not _any_derivable(check.steps):
        return SelftestRow(UNPROVEN, check.name, 0, "no derivable step and no selftest declaration")
    mutations = _derive_mutations(check.steps, work)
    if not mutations:
        if _has_high(check, work):
            return SelftestRow(BASELINE_FAIL, check.name, 0, "baseline HIGH on unmodified copy")
        return SelftestRow(VACUOUS, check.name, 0, "derivable steps produced no mutation")
    return _run_mutations(check, work, mutations)


def _any_derivable(steps: list[Step]) -> bool:
    return any(_str(step, "kind") in DERIVABLE_KINDS for step in steps)


def _run_mutations(check: DictCheck, work: Path, units: list[list[Mutation]]) -> SelftestRow:
    if _has_high(check, work):
        return SelftestRow(BASELINE_FAIL, check.name, 0, "baseline HIGH on unmodified copy")
    if not units:
        return SelftestRow(VACUOUS, check.name, 0, "no mutations to apply")
    for unit in units:
        error = _unit_error(work, unit)
        if error:
            return SelftestRow(UNPROVEN, check.name, 0, error)
        snaps: dict[Path, tuple[bool, bytes | None]] = {}
        try:
            for mutation in unit:
                target = _resolve_in(work, _str(mutation, "path"))
                if target is not None and target not in snaps:
                    snaps[target] = _snapshot(target)
                _apply_mutation(work, mutation)
            if not _has_high(check, work):
                first = unit[0]
                return SelftestRow(
                    VACUOUS,
                    check.name,
                    len(units),
                    "mutation op="
                    + _str(first, "op")
                    + " path="
                    + _str(first, "path")
                    + " did not produce HIGH",
                )
        finally:
            for target, (existed, snapshot) in snaps.items():
                _restore(target, existed, snapshot)
    return SelftestRow(OK, check.name, len(units))


def _unit_error(work: Path, unit: list[Mutation]) -> str:
    if not unit:
        return "empty mutation unit"
    for mutation in unit:
        op = _str(mutation, "op")
        if op not in MUTATION_OPS:
            return f"unknown mutation op {op!r}"
        if _resolve_in(work, _str(mutation, "path")) is None:
            return "mutation path escapes the copy"
    return ""


def _parse_mutations(raw: object) -> tuple[list[list[Mutation]], str]:
    if not isinstance(raw, list):
        return [], "selftest.mutations must be a list"
    units: list[list[Mutation]] = []
    for item in raw:
        if not isinstance(item, dict):
            return [], "selftest.mutations items must be mappings"
        op = item.get("op")
        if not isinstance(op, str) or op not in MUTATION_OPS:
            return [], f"unknown mutation op {op!r}"
        path = item.get("path")
        if not isinstance(path, str) or not path.strip():
            return [], "mutation path is required"
        units.append([item])
    return units, ""


def _derive_mutations(steps: list[Step], work: Path) -> list[list[Mutation]]:
    derived: list[list[Mutation]] = []
    for step in steps:
        if _str(step, "severity", "high").lower() == "low":
            continue
        kind = _str(step, "kind")
        if kind == "file_exists":
            path = _str(step, "path")
            if path:
                derived.append([{"op": "delete_file", "path": path}])
        elif kind == "file_contains":
            unit = _derive_file_contains(step, work)
            if unit:
                derived.append(unit)
        elif kind == "file_absent":
            unit = _derive_append_needles(_str(step, "path"), _str_list(step, "needles"))
            if unit:
                derived.append(unit)
        elif kind == "concat_absent":
            paths = _str_list(step, "paths")
            needles = _str_list(step, "needles")
            if paths:
                unit = _derive_append_needles(paths[0], needles)
                if unit:
                    derived.append(unit)
        elif kind == "glob_absent":
            hit = _first_glob_file(work, _str(step, "glob"))
            needles = _str_list(step, "needles")
            if hit:
                unit = _derive_append_needles(hit, needles)
                if unit:
                    derived.append(_unless_disable_unit(step) + unit)
        elif kind == "file_equals":
            path = _str(step, "path")
            if path:
                derived.append([{"op": "append_text", "path": path, "text": "\n#selftest"}])
        elif kind == "concat_contains_any":
            unit = _derive_concat_contains_any(step)
            if unit:
                derived.append(unit)
        elif kind == "rust_test_asserts":
            for unit in _derive_rust_test_asserts(step, work):
                derived.append(unit)
        elif kind == "cite_equals":
            unit = _derive_cite_equals(step, work)
            if unit:
                derived.append(unit)
        elif kind == "refs_resolve":
            unit = _derive_refs_resolve(step, work)
            if unit:
                derived.append(unit)
        elif kind == "file_max_lines":
            unit = _derive_file_max_lines(step, work)
            if unit:
                derived.append(unit)
        elif kind == "glob_max_lines":
            derived.extend(_derive_glob_max_lines(step, work))
        elif kind == "frontmatter_paths_exist":
            unit = _derive_frontmatter_paths_exist(step, work)
            if unit:
                derived.append(unit)
        elif kind == "cargo_dep_constraint":
            unit = _derive_cargo_dep_constraint(step, work)
            if unit:
                derived.append(unit)
        elif kind == "test_matrix":
            unit = _derive_test_matrix(step, work)
            if unit:
                derived.append(unit)
        elif kind == "yaml_map_list":
            unit = _derive_yaml_map_list(step, work)
            if unit:
                derived.append(unit)
        elif kind == "pointer_in_other_file":
            unit = _derive_pointer_in_other_file(step, work)
            if unit:
                derived.append(unit)
        elif kind == "markdown_heading_tag":
            unit = _derive_markdown_heading_tag(step, work)
            if unit:
                derived.append(unit)
        elif kind == "glob_none":
            unit = _derive_glob_none(step, work)
            if unit:
                derived.append(unit)
        elif kind == "glob_paths_in_file":
            unit = _derive_glob_paths_in_file(step, work)
            if unit:
                derived.append(unit)
        elif kind == "when_match_then_match":
            unit = _derive_when_match_then_match(step, work)
            if unit:
                derived.append(unit)
        elif kind == "disk_free":
            unit = _derive_disk_free(step, work)
            if unit:
                derived.append(unit)
    return derived


def _derive_disk_free(step: Step, work: Path) -> list[Mutation]:
    from kutha_gov.kinds import disk_free_mutation_target

    target = disk_free_mutation_target(step, work)
    if target is None:
        return []
    path, pattern, replacement = target
    return [{"op": "replace_regex", "path": path, "pattern": pattern, "with": replacement}]


def _derive_test_matrix(step: Step, work: Path) -> list[Mutation]:
    from kutha_gov.kinds import test_matrix_mutation_target

    target = test_matrix_mutation_target(step, work)
    if target is None:
        return []
    path, pattern, replacement = target
    return [{"op": "replace_regex", "path": path, "pattern": pattern, "with": replacement}]


def _derive_yaml_map_list(step: Step, work: Path) -> list[Mutation]:
    from kutha_gov.kinds import yaml_map_list_mutation_target

    target = yaml_map_list_mutation_target(step, work)
    if target is None:
        return []
    path, pattern, replacement = target
    return [{"op": "replace_regex", "path": path, "pattern": pattern, "with": replacement}]


def _derive_pointer_in_other_file(step: Step, work: Path) -> list[Mutation]:
    from kutha_gov.kinds import pointer_in_other_file_mutation_target

    target = pointer_in_other_file_mutation_target(step, work)
    if target is None:
        return []
    path, pattern, replacement = target
    return [{"op": "replace_regex", "path": path, "pattern": pattern, "with": replacement}]


def _derive_markdown_heading_tag(step: Step, work: Path) -> list[Mutation]:
    from kutha_gov.kinds import markdown_heading_tag_mutation_target

    target = markdown_heading_tag_mutation_target(step, work)
    if target is None:
        return []
    path, pattern, replacement = target
    return [{"op": "replace_regex", "path": path, "pattern": pattern, "with": replacement}]


def _derive_glob_none(step: Step, work: Path) -> list[Mutation]:
    from kutha_gov.kinds import glob_none_mutation_target

    target = glob_none_mutation_target(step)
    if target is None:
        return []
    path, text = target
    return [{"op": "append_text", "path": path, "text": text}]


def _derive_glob_paths_in_file(step: Step, work: Path) -> list[Mutation]:
    from kutha_gov.kinds import glob_paths_in_file_mutation_target

    target = glob_paths_in_file_mutation_target(step)
    if target is None:
        return []
    path, text = target
    return [{"op": "append_text", "path": path, "text": text}]


def _derive_when_match_then_match(step: Step, work: Path) -> list[Mutation]:
    from kutha_gov.kinds import when_match_then_match_mutation_target

    target = when_match_then_match_mutation_target(step, work)
    if target is None:
        return []
    path, pattern, replacement = target
    return [{"op": "replace_regex", "path": path, "pattern": pattern, "with": replacement}]


def _derive_cargo_dep_constraint(step: Step, work: Path) -> list[Mutation]:
    from kutha_gov.kinds import cargo_dep_constraint_mutation_target

    target = cargo_dep_constraint_mutation_target(step, work)
    if target is None:
        return []
    path, text = target
    return [{"op": "append_text", "path": path, "text": text}]


def _derive_file_max_lines(step: Step, work: Path) -> list[Mutation]:
    from kutha_gov.kinds import file_max_lines_mutation_target

    target = file_max_lines_mutation_target(step, work)
    if target is None:
        return []
    path, text = target
    return [{"op": "append_text", "path": path, "text": text}]


def _derive_frontmatter_paths_exist(step: Step, work: Path) -> list[Mutation]:
    from kutha_gov.kinds import frontmatter_paths_exist_mutation_target

    target = frontmatter_paths_exist_mutation_target(step, work)
    if target is None:
        return []
    path, text, _bogus = target
    return [{"op": "replace_regex", "path": path, "pattern": ".*", "with": text}]


def _derive_glob_max_lines(step: Step, work: Path) -> list[list[Mutation]]:
    from kutha_gov.kinds import (
        _glob_max_lines_hits,
        glob_max_lines_effective_min_hits,
        glob_max_lines_mutation_target,
    )

    units: list[list[Mutation]] = []
    target = glob_max_lines_mutation_target(step, work)
    if target is not None:
        path, text = target
        units.append([{"op": "append_text", "path": path, "text": text}])
    min_hits = glob_max_lines_effective_min_hits(step, work)
    pattern = _str(step, "glob")
    when_glob = _str(step, "when_glob_exists")
    hits = _glob_max_lines_hits(step, work)
    independent_gate = bool(when_glob) and when_glob != pattern
    if min_hits > 0 and hits and (independent_gate or not units):
        units.append(
            [{"op": "delete_file", "path": path.relative_to(work).as_posix()} for path in hits]
        )
    return units


def _derive_refs_resolve(step: Step, work: Path) -> list[Mutation]:
    from kutha_gov.kinds import refs_resolve_mutation_target

    path = refs_resolve_mutation_target(step, work)
    if not path:
        return []
    return [{"op": "append_text", "path": path, "text": "\nADR-999\n"}]


def _derive_cite_equals(step: Step, work: Path) -> list[Mutation]:
    from kutha_gov.kinds import cite_equals_mutation_target

    target = cite_equals_mutation_target(step, work)
    if target is None:
        return []
    path, text = target
    return [{"op": "append_text", "path": path, "text": text}]


def _derive_rust_test_asserts(step: Step, work: Path) -> list[list[Mutation]]:
    from kutha_gov.kinds import rust_test_mutation_targets

    units: list[list[Mutation]] = []
    for rel, name in rust_test_mutation_targets(step, work):
        path = work / rel
        if not path.is_file():
            continue
        units.append(
            [
                {
                    "op": "replace_regex",
                    "path": rel,
                    "pattern": r"(fn\s+" + re.escape(name) + r"\b)",
                    "with": r"#[ignore]\n\1",
                }
            ]
        )
        break
    nfy = step.get("names_from_yaml")
    when = step.get("require_evidence_when")
    if isinstance(nfy, dict) and isinstance(when, dict):
        yaml_path = nfy.get("path")
        field = when.get("field")
        equals = when.get("equals")
        if (
            isinstance(yaml_path, str)
            and yaml_path.strip()
            and isinstance(field, str)
            and isinstance(equals, str)
        ):
            units.append(
                [
                    {
                        "op": "replace_regex",
                        "path": yaml_path,
                        "pattern": (
                            r"(?m)^("
                            + r"[ \t]*"
                            + re.escape(f"{field}: {equals}")
                            + r"\n(?:[ \t]+.+\n)*?[ \t]+evidence: )(\[[^\]]+\])"
                        ),
                        "with": r"\1[]",
                    }
                ]
            )
    return units


def _derive_file_contains(step: Step, work: Path) -> list[Mutation]:
    from kutha_gov.kinds import _glob_max_lines_hits

    pattern = _str(step, "glob")
    needles = _str_list(step, "needles")
    if not needles:
        return []
    if pattern:
        hits = _glob_max_lines_hits(step, work)
        if not hits:
            return []
        path = hits[0].relative_to(work).as_posix()
    else:
        path = _str(step, "path")
    if not path:
        return []
    require = _str(step, "require", "all").lower()
    if require == "any":
        return [{"op": "remove_text", "path": path, "texts": needles}]
    return [{"op": "remove_text", "path": path, "text": needles[0]}]


def _unless_disable_unit(step: Step) -> list[Mutation]:
    """Turn off `unless_file_contains` on the copy so appending a needle still fails."""
    raw = step.get("unless_file_contains")
    if not isinstance(raw, dict):
        return []
    path = raw.get("file")
    needle = raw.get("needle")
    if not isinstance(path, str) or not path.strip():
        return []
    if not isinstance(needle, str) or not needle:
        return []
    return [
        {
            "op": "replace_regex",
            "path": path,
            "pattern": re.escape(needle),
            "with": "SELFTEST_UNLESS_DISABLED",
        }
    ]


def _derive_append_needles(path: str, needles: list[str]) -> list[Mutation]:
    if not path or not needles:
        return []
    return [{"op": "append_text", "path": path, "text": "\n" + needles[0]}]


def _derive_concat_contains_any(step: Step) -> list[Mutation]:
    paths = _str_list(step, "paths")
    needles = _str_list(step, "needles")
    if not paths or not needles:
        return []
    return [{"op": "remove_text", "path": path, "texts": needles} for path in paths]


def _remove_needles(content: str, mutation: Mutation) -> str:
    needles = _str_list(mutation, "texts")
    if not needles:
        needles = [_str(mutation, "text")]
    for needle in needles:
        content = _remove_text(content, needle)
    return content


def _apply_mutation(work: Path, mutation: Mutation) -> None:
    op = _str(mutation, "op")
    path = _resolve_in(work, _str(mutation, "path"))
    if path is None:
        return
    if op == "delete_file":
        if path.is_file():
            path.unlink()
        return
    if op == "append_text":
        path.parent.mkdir(parents=True, exist_ok=True)
        with path.open("a", encoding="utf-8") as handle:
            handle.write(_str(mutation, "text"))
        return
    text = path.read_text(encoding="utf-8") if path.is_file() else ""
    if op == "remove_text":
        path.write_text(_remove_needles(text, mutation), encoding="utf-8")
        return
    if op == "replace_regex":
        pattern = _str(mutation, "pattern")
        replacement = mutation.get("with", "")
        repl = replacement if isinstance(replacement, str) else ""
        path.write_text(re.sub(pattern, repl, text, count=1), encoding="utf-8")


def _remove_text(content: str, needle: str) -> str:
    if not needle:
        return content
    if needle in content:
        return content.replace(needle, "")
    folded = content.lower()
    key = needle.lower()
    pieces: list[str] = []
    start = 0
    while True:
        found = folded.find(key, start)
        if found < 0:
            pieces.append(content[start:])
            break
        pieces.append(content[start:found])
        start = found + len(needle)
    return "".join(pieces)


def _has_high(check: Check, root: Path) -> bool:
    return _run_check(check, root).high_count > 0


def _run_check(check: Check, root: Path) -> CheckResult:
    try:
        result = check.run(Context(root=root))
    except Exception as exc:
        result = CheckResult(check=check.name)
        result.findings.append(
            Finding(
                check.name,
                Severity.HIGH,
                "check-error",
                f"{type(exc).__name__}: {exc}",
            )
        )
    result.check = check.name
    return result


def _first_glob_file(root: Path, pattern: str) -> str | None:
    if not pattern:
        return None
    for path in sorted(root.glob(pattern)):
        if path.is_file():
            return str(path.relative_to(root)).replace("\\", "/")
    return None


def _resolve_in(root: Path, rel: str) -> Path | None:
    if not rel or rel.startswith("/") or ".." in Path(rel).parts:
        return None
    path = (root / rel).resolve()
    try:
        path.relative_to(root.resolve())
    except ValueError:
        return None
    return path


def _snapshot(path: Path) -> tuple[bool, bytes | None]:
    if path.is_file():
        return True, path.read_bytes()
    return False, None


def _restore(path: Path, existed: bool, data: bytes | None) -> None:
    if existed:
        path.parent.mkdir(parents=True, exist_ok=True)
        if data is not None:
            path.write_bytes(data)
        return
    if path.is_file():
        path.unlink()


def _str(step: Step, key: str, default: str = "") -> str:
    value = step.get(key, default)
    return value if isinstance(value, str) else default


def _str_list(step: Step, key: str) -> list[str]:
    value = step.get(key, [])
    if not isinstance(value, list):
        return []
    return [item for item in value if isinstance(item, str)]
