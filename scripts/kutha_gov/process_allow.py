"""H3: process-plane relation allowlist. Not ADR-050's six kinds. Fail-closed."""

from __future__ import annotations

import os
from pathlib import Path

import yaml

DICT_REL = ".kutha/dictionaries/relations.yaml"
SCHEMA = "kutha-harness-relations/v1"
ENV_PATH = "KUTHA_HARNESS_RELATIONS_PATH"


def relations_path(root: Path) -> Path:
    raw = os.environ.get(ENV_PATH, "").strip()
    if raw:
        path = Path(raw)
        return path if path.is_absolute() else root / path
    return root / DICT_REL


def load_process_relations(root: Path) -> frozenset[str]:
    path = relations_path(root)
    try:
        loaded = yaml.safe_load(path.read_text(encoding="utf-8"))
    except OSError:
        return frozenset()
    if not isinstance(loaded, dict) or loaded.get("schema") != SCHEMA:
        return frozenset()
    raw = loaded.get("relations", [])
    if not isinstance(raw, list):
        return frozenset()
    names = frozenset(item for item in raw if isinstance(item, str) and item.strip())
    return names


def admit(root: Path, relation: str) -> bool:
    return relation in load_process_relations(root)
