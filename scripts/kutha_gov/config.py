"""Harness settings from the environment. No product secrets. CLI flags win."""

from __future__ import annotations

import os
from pathlib import Path

ENV_BUDGET = "KUTHA_GOV_BUDGET"
ENV_FAIL_ON_WARN = "KUTHA_GOV_FAIL_ON_WARN"


def load_dotenv(root: Path) -> None:
    path = root / ".env"
    if not path.is_file():
        return
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, _, value = line.partition("=")
        key = key.strip()
        value = value.strip().strip('"').strip("'")
        if key and key not in os.environ:
            os.environ[key] = value


def env_flag(name: str) -> bool:
    raw = os.environ.get(name, "").strip().lower()
    return raw in {"1", "true", "yes", "on"}


def resolve_budget(*, cli: int | None, fsm_default: int) -> int:
    if cli is not None:
        return max(1, cli)
    raw = os.environ.get(ENV_BUDGET)
    if raw:
        try:
            return max(1, int(raw.strip()))
        except ValueError:
            return max(1, fsm_default)
    return max(1, fsm_default)
