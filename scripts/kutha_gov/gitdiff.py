"""Git path listing for coupling checks. Not a Check subclass; kinds.py interprets YAML."""

from __future__ import annotations

import re
import subprocess
from pathlib import Path, PurePosixPath

_GIT_TIMEOUT_SEC = 5


def is_git_repo(root: Path) -> bool:
    return _git(root, ["rev-parse", "--git-dir"]) is not None


def changed_relpaths(root: Path, against: str) -> set[str] | None:
    """Return repo-relative changed paths, or None when git is unavailable."""
    if not is_git_repo(root):
        return None
    mode = against.strip().lower() or "auto"
    if mode == "staged":
        return _name_only(root, ["diff", "--cached", "--name-only", "--diff-filter=ACMR"])
    if mode == "worktree":
        return _worktree(root)
    if mode == "head":
        return _name_only(root, ["diff", "--name-only", "--diff-filter=ACMR", "HEAD~1"])
    if mode == "auto":
        staged = _name_only(root, ["diff", "--cached", "--name-only", "--diff-filter=ACMR"])
        if staged:
            return staged
        worktree = _worktree(root)
        if worktree:
            return worktree
        return _name_only(root, ["diff", "--name-only", "--diff-filter=ACMR", "HEAD~1"])
    return None


def path_matches(rel: str, pattern: str) -> bool:
    """Match a git path against a glob. `dir/**` covers the tree; `**` is recursive."""
    posix = rel.replace("\\", "/")
    if posix == pattern:
        return True
    prefix = pattern.removesuffix("/**")
    if prefix != pattern:
        return posix == prefix or posix.startswith(prefix + "/")
    if "*" in pattern:
        escaped = re.escape(pattern)
        regex = (
            escaped.replace(r"\*\*/", r"(?:.*/)?").replace(r"\*\*", r".*").replace(r"\*", r"[^/]*")
        )
        return re.fullmatch(regex, posix) is not None
    return PurePosixPath(posix).match(pattern)


def _worktree(root: Path) -> set[str]:
    paths = _name_only(root, ["diff", "--name-only", "--diff-filter=ACMR"])
    paths |= _name_only(root, ["diff", "--cached", "--name-only", "--diff-filter=ACMR"])
    extra = _git(root, ["ls-files", "--others", "--exclude-standard"])
    if extra:
        paths.update(line.strip() for line in extra.splitlines() if line.strip())
    return paths


def _name_only(root: Path, args: list[str]) -> set[str]:
    text = _git(root, args)
    if text is None:
        return set()
    return {line.strip() for line in text.splitlines() if line.strip()}


def _git(root: Path, args: list[str]) -> str | None:
    try:
        completed = subprocess.run(
            ["git", *args],
            cwd=root,
            capture_output=True,
            text=True,
            timeout=_GIT_TIMEOUT_SEC,
            check=False,
        )
    except (OSError, subprocess.TimeoutExpired):
        return None
    if completed.returncode != 0:
        return None
    return completed.stdout
