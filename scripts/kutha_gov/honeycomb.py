"""Load the L_map honeycomb dictionary for compact context dumps."""

from __future__ import annotations

from pathlib import Path
from typing import Any

import yaml

MAP_REL = ".kutha/dictionaries/honeycomb.yaml"
SCHEMA = "kutha-map-honeycomb/v1"


def load_map(root: Path) -> dict[str, Any]:
    path = root / MAP_REL
    try:
        loaded = yaml.safe_load(path.read_text(encoding="utf-8"))
    except yaml.YAMLError as exc:
        raise ValueError(f"{MAP_REL}: {exc}") from exc
    if not isinstance(loaded, dict):
        raise ValueError(f"{MAP_REL} root must be a mapping")
    if loaded.get("schema") != SCHEMA:
        raise ValueError(f"{MAP_REL} unknown schema {loaded.get('schema')!r}")
    cells = loaded.get("cells")
    if not isinstance(cells, list) or any(not isinstance(cell, dict) for cell in cells):
        raise ValueError(f"{MAP_REL} cells must be a list of mappings")
    return loaded


def _as_maps(loaded: dict[str, Any], key: str) -> list[dict[str, Any]]:
    rows = loaded.get(key, [])
    if not isinstance(rows, list):
        return []
    return [row for row in rows if isinstance(row, dict)]


def _str_items(value: object) -> list[str]:
    if not isinstance(value, list):
        return []
    return [item for item in value if isinstance(item, str)]


def resolve_cell_id(by_id: dict[str, dict[str, Any]], focus: str) -> str | None:
    key = focus.strip()
    if key in by_id:
        return key
    upper = key.upper()
    if upper in by_id:
        return upper
    padded = upper if upper.startswith("ADR-") else f"ADR-{upper if upper.isdigit() else key}"
    if padded in by_id:
        return padded
    return None


def format_map(loaded: dict[str, Any], *, focus: str | None = None) -> str:
    cells = _as_maps(loaded, "cells")
    by_id = {str(row.get("id", "")): row for row in cells}
    if focus:
        key = resolve_cell_id(by_id, focus)
        selected = neighborhood(by_id, key) if key else []
        if not selected:
            return f"unknown cell: {focus}\n"
        rows = [by_id[cid] for cid in selected if cid in by_id]
    else:
        rows = cells
    lines = [
        "id       axis          map       delivery  cap    must",
        "-------- ------------- --------- --------- ------ ----",
    ]
    for row in rows:
        lines.append(
            f"{row.get('id', '')!s:<8} "
            f"{row.get('axis', '')!s:<13} "
            f"{row.get('map', '')!s:<9} "
            f"{row.get('delivery', '')!s:<9} "
            f"{row.get('capability', '')!s:<6} "
            f"{row.get('must', '')}"
        )
        deps = _str_items(row.get("depends_on"))
        locks = _str_items(row.get("locks"))
        evidence = _str_items(row.get("evidence"))
        extra = []
        if deps:
            extra.append("deps=" + ",".join(deps))
        if locks:
            extra.append("locks=" + ",".join(locks))
        if evidence:
            extra.append("evidence=" + ",".join(evidence))
        freeze_as = row.get("freeze_as")
        if isinstance(freeze_as, str) and freeze_as.strip():
            extra.append(f"freeze_as={freeze_as.strip()}")
        if extra:
            lines.append("         " + " ".join(extra))
    lines.append(f"\n{len(rows)} cell(s) from {MAP_REL} (L_map index, not SoT, not backlog)")
    return "\n".join(lines) + "\n"


def neighborhood(by_id: dict[str, dict[str, Any]], focus: str) -> list[str]:
    if focus not in by_id:
        return []
    seen: list[str] = [focus]
    row = by_id[focus]
    deps: list[str] = _str_items(row.get("depends_on"))
    for item in deps:
        if item not in seen:
            seen.append(item)
    for cid, other in by_id.items():
        other_deps: list[str] = _str_items(other.get("depends_on"))
        if focus in other_deps and cid not in seen:
            seen.append(cid)
    return seen
