"""Harness settings from the environment. No product secrets. CLI flags win."""

from __future__ import annotations

import os
from pathlib import Path

ENV_BUDGET = "KUTHA_GOV_BUDGET"
ENV_FAIL_ON_WARN = "KUTHA_GOV_FAIL_ON_WARN"
ENV_JOBS = "KUTHA_GOV_JOBS"


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


def default_jobs() -> int:
    return max(1, min(os.cpu_count() or 1, 8))


def resolve_jobs(*, cli: int | str | None) -> int:
    """Flag > env > default min(CPU, 8). Reject 0 and non-integers."""
    if cli is not None:
        return _parse_jobs(cli, "--jobs")
    raw = os.environ.get(ENV_JOBS)
    if raw is not None and raw.strip() != "":
        return _parse_jobs(raw, ENV_JOBS)
    return default_jobs()


def _parse_jobs(raw: int | str, source: str) -> int:
    try:
        n = int(str(raw).strip())
    except (TypeError, ValueError) as exc:
        raise ValueError(f"{source}: invalid jobs value {raw!r} (need an integer >= 1)") from exc
    if n < 1:
        raise ValueError(f"{source}: jobs must be >= 1, got {n}")
    return n
