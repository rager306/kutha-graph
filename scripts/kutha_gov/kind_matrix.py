"""Test-matrix policy family (CE #21)."""

from __future__ import annotations

import re
from pathlib import Path

import yaml

from kutha_gov.kinds import (
    Step,
    _high_missing,
    _prefixes,
    _prove_named_test,
    _rust_fn_index,
    _str,
    _unique_names,
    _yaml_document,
)
from kutha_gov.protocol import CheckResult, Context, Finding, Severity

_MATRIX_SCHEMA = "kutha-test-matrix/v1"
_MATRIX_STATUSES = frozenset({"killed", "characterization", "gap", "blocked", "spec-only", "gate"})
_MATRIX_CHAR_KILL = re.compile(
    r"mirror|alias|tautolog|honor[- ]system|env[- ]floor|"
    r"not an independent kill|not a fold kill|not domain",
    re.IGNORECASE,
)
_MATRIX_PLANES = frozenset({"product", "harness"})
_MATRIX_LAYERS = frozenset(
    {"decision", "characterization", "property", "crash", "kill", "governor", "bench"}
)
_MATRIX_BLOCKED_OWNERS = frozenset(
    {
        "C01",
        "HORN-KC-01",
        "HORN-CORR-01",
        "ASOF-ELIG-01",
        "TR-02",
        "DET-02",
        "FIXG-01",
    }
)
_MATRIX_SLICE_RE = re.compile(r"^(M\d+[a-z]?)-S(\d+)$")
_MATRIX_HEADING_RE = re.compile(r"^## (M\d+[a-z]?)\b")
_MATRIX_SLICE_LINE_RE = re.compile(r"\*\*S(\d+):")
_MATRIX_FEATURE_RE = re.compile(r"^feature:(.+)$")
_MATRIX_REQUIRE_STR = (
    "id",
    "plane",
    "invariant",
    "layer",
    "kill_mutation",
    "status",
    "gate",
    "owner",
    "source",
)


def _roadmap_slice_ids(text: str) -> set[str]:
    current: str | None = None
    found: set[str] = set()
    for line in text.splitlines():
        heading = _MATRIX_HEADING_RE.match(line)
        if heading:
            current = heading.group(1)
            continue
        if current is None:
            continue
        slice_hit = _MATRIX_SLICE_LINE_RE.search(line)
        if slice_hit:
            found.add(f"{current}-S{slice_hit.group(1)}")
    return found


def _observe_cargo_states(loaded: object) -> set[str]:
    if not isinstance(loaded, dict):
        return set()
    states = loaded.get("states")
    if not isinstance(states, dict):
        return set()
    names: set[str] = set()
    for key, body in states.items():
        if isinstance(key, str) and isinstance(body, dict) and body.get("kind") == "observe_cargo":
            names.add(key)
    return names


def _check_ids_from_yaml(loaded: object) -> set[str]:
    if not isinstance(loaded, dict):
        return set()
    rows = loaded.get("checks")
    if not isinstance(rows, list):
        return set()
    return {
        str(row.get("id"))
        for row in rows
        if isinstance(row, dict) and isinstance(row.get("id"), str) and str(row.get("id")).strip()
    }


def _matrix_oracles(row: dict) -> list[str]:
    raw = row.get("oracle")
    if not isinstance(raw, list):
        return []
    return [item.strip() for item in raw if isinstance(item, str) and item.strip()]


def _kind_test_matrix(check: str, step: Step, ctx: Context, result: CheckResult) -> None:
    """Matrix consistency: named oracles exist; statuses match D-04 rules."""
    path = _str(step, "path") or "docs/architecture/test-matrix.yaml"
    rust_glob = _str(step, "rust_glob") or "crates/**/*.rs"
    checks_path = _str(step, "checks") or ".kutha/dictionaries/checks.yaml"
    roadmap_path = _str(step, "roadmap") or ".kutha/ROADMAP.md"
    fsm_path = _str(step, "fsm") or ".kutha/dictionaries/fsm.yaml"
    loaded, ok = _yaml_document(check, path, ctx, result)
    if not ok:
        return
    if not isinstance(loaded, dict):
        result.findings.append(
            Finding(check, Severity.HIGH, "yaml", f"{path} root is not a mapping", path)
        )
        return
    schema = loaded.get("schema")
    if schema != _MATRIX_SCHEMA:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "matrix-schema",
                f"{path} schema {schema!r} is not {_MATRIX_SCHEMA}",
                path,
            )
        )
        return
    rows = loaded.get("rows")
    if not isinstance(rows, list) or not rows:
        result.findings.append(
            Finding(
                check, Severity.HIGH, "yaml-select", f"{path} rows is not a non-empty list", path
            )
        )
        return
    non_maps = [index for index, row in enumerate(rows) if not isinstance(row, dict)]
    if non_maps:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "yaml-select",
                f"{path} rows contains non-mapping entries at index "
                f"{', '.join(str(i) for i in non_maps)}",
                path,
            )
        )
        return
    maps = [row for row in rows if isinstance(row, dict)]
    result.scanned = len(maps)

    missing_fields: list[str] = []
    for row in maps:
        row_id = str(row.get("id", "?"))
        absent = [
            key
            for key in _MATRIX_REQUIRE_STR
            if not isinstance(row.get(key), str) or not str(row.get(key)).strip()
        ]
        if not isinstance(row.get("oracle"), list):
            absent.append("oracle")
        if absent:
            missing_fields.append(f"{row_id}:{','.join(absent)}")
    if missing_fields:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "matrix-fields",
                f"{path} rows missing fields: {', '.join(missing_fields)}",
                path,
            )
        )
        return

    seen: dict[str, int] = {}
    dups: list[str] = []
    for row in maps:
        row_id = str(row["id"])
        seen[row_id] = seen.get(row_id, 0) + 1
        if seen[row_id] == 2:
            dups.append(row_id)
    if dups:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "matrix-unique",
                f"{path} duplicate id: {', '.join(dups)}",
                path,
            )
        )
        return

    bad_status = sorted(
        {str(row["status"]) for row in maps if str(row["status"]) not in _MATRIX_STATUSES}
    )
    if bad_status:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "matrix-vocab",
                f"{path} status not in allowlist: {', '.join(bad_status)}",
                path,
            )
        )
        return
    bad_plane = sorted(
        {str(row["plane"]) for row in maps if str(row["plane"]) not in _MATRIX_PLANES}
    )
    if bad_plane:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "matrix-vocab",
                f"{path} plane not in allowlist: {', '.join(bad_plane)}",
                path,
            )
        )
        return
    bad_layer = sorted(
        {str(row["layer"]) for row in maps if str(row["layer"]) not in _MATRIX_LAYERS}
    )
    if bad_layer:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "matrix-vocab",
                f"{path} layer not in allowlist: {', '.join(bad_layer)}",
                path,
            )
        )
        return

    killed_none = [
        str(row["id"])
        for row in maps
        if str(row["status"]) == "killed" and str(row["kill_mutation"]).strip().lower() == "none"
    ]
    if killed_none:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "matrix-killed",
                f"{path} killed rows with kill_mutation none: {', '.join(killed_none)}",
                path,
            )
        )
        return
    killed_char = [
        str(row["id"])
        for row in maps
        if str(row["status"]) == "killed" and _MATRIX_CHAR_KILL.search(str(row["kill_mutation"]))
    ]
    if killed_char:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "matrix-killed-characterization",
                f"{path} killed rows with characterization-shaped kill_mutation: "
                f"{', '.join(killed_char)}",
                path,
            )
        )
        return
    empty_killed = [
        str(row["id"])
        for row in maps
        if str(row["status"]) in {"killed", "characterization"} and not _matrix_oracles(row)
    ]
    if empty_killed:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "matrix-fields",
                f"{path} killed/characterization rows missing oracle items: {', '.join(empty_killed)}",
                path,
            )
        )
        return

    roadmap_text = ctx.read(roadmap_path)
    if roadmap_text is None:
        _high_missing(check, roadmap_path, result)
        return
    slices = _roadmap_slice_ids(roadmap_text)
    gap_bad: list[str] = []
    slice_bad: list[str] = []
    blocked_bad: list[str] = []
    for row in maps:
        row_id = str(row["id"])
        owner = str(row["owner"]).strip()
        status = str(row["status"])
        if status == "blocked":
            if owner not in _MATRIX_BLOCKED_OWNERS:
                blocked_bad.append(f"{row_id}:{owner}")
            continue
        if not _MATRIX_SLICE_RE.match(owner) or owner not in slices:
            if status == "gap":
                gap_bad.append(f"{row_id}:{owner}")
            else:
                slice_bad.append(f"{row_id}:{owner}")
    if gap_bad:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "matrix-gap-owner",
                f"{path} gap owner not a ROADMAP slice: {', '.join(gap_bad)}",
                path,
            )
        )
        return
    if slice_bad:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "matrix-gap-owner",
                f"{path} owner not a ROADMAP slice: {', '.join(slice_bad)}",
                path,
            )
        )
        return
    if blocked_bad:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "matrix-blocked-owner",
                f"{path} blocked owner not in decision allowlist: {', '.join(blocked_bad)}",
                path,
            )
        )
        return

    fsm_loaded, fsm_ok = _yaml_document(check, fsm_path, ctx, result)
    if not fsm_ok:
        return
    observe = _observe_cargo_states(fsm_loaded)
    feature_bad: list[str] = []
    gate_bad: list[str] = []
    for row in maps:
        gate = str(row["gate"]).strip()
        if gate in {"default", "controller"}:
            continue
        feat = _MATRIX_FEATURE_RE.match(gate)
        if not feat:
            gate_bad.append(f"{row.get('id')}:{gate}")
            continue
        observe_id = feat.group(1).strip()
        if observe_id not in observe:
            feature_bad.append(f"{row.get('id')}:{gate}")
    if gate_bad:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "matrix-feature-gate",
                f"{path} gate not default|controller|feature:<observe>: {', '.join(gate_bad)}",
                path,
            )
        )
        return
    if feature_bad:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "matrix-feature-gate",
                f"{path} feature gate missing FSM observe row: {', '.join(feature_bad)}",
                path,
            )
        )
        return

    checks_loaded, checks_ok = _yaml_document(check, checks_path, ctx, result)
    if not checks_ok:
        return
    check_ids = _check_ids_from_yaml(checks_loaded)
    missing_checks: list[str] = []
    product_names: list[str] = []
    for row in maps:
        names = _matrix_oracles(row)
        if str(row["plane"]) == "harness":
            missing_checks.extend(name for name in names if name not in check_ids)
        else:
            product_names.extend(names)
    if missing_checks:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "matrix-oracle-check",
                f"{path} harness oracle missing in checks.yaml: {', '.join(sorted(set(missing_checks)))}",
                path,
            )
        )
        return

    files = [hit for hit in sorted(ctx.root.glob(rust_glob)) if hit.is_file()]
    if product_names and not files:
        result.findings.append(
            Finding(
                check,
                Severity.HIGH,
                "rust-test-glob",
                f"{rust_glob} matched no rust files",
                rust_glob,
            )
        )
        return
    if product_names:
        index = _rust_fn_index(ctx.root, files)
        result.scanned += len(files)
        prefixes = _prefixes(step)
        for name in _unique_names(product_names) or []:
            _prove_named_test(check, step, ctx, result, name, index.get(name, []), prefixes, [])


def test_matrix_mutation_target(step: Step, root: Path) -> tuple[str, str, str] | None:
    path = _str(step, "path") or "docs/architecture/test-matrix.yaml"
    loc = root / path
    if not loc.is_file():
        return None
    text = loc.read_text(encoding="utf-8")
    try:
        loaded = yaml.safe_load(text)
    except yaml.YAMLError:
        return None
    if not isinstance(loaded, dict):
        return None
    rows = loaded.get("rows")
    if not isinstance(rows, list):
        return None
    for row in rows:
        if not isinstance(row, dict) or str(row.get("plane", "")) != "product":
            continue
        names = _matrix_oracles(row)
        if not names:
            continue
        needle = f"- {names[0]}"
        if needle in text:
            return path, re.escape(needle), "- __selftest_missing_oracle__"
    return None
