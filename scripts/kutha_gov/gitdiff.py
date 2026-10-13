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
        # Clean tree (M073): branch window vs upstream/main, not last commit alone.
        # HEAD~1 false-positives docs-coupling when only checks.yaml landed in tip.
        base = _clean_tree_diff_base(root)
        if base:
            return _name_only(
                root, ["diff", "--name-only", "--diff-filter=ACMR", f"{base}...HEAD"]
            )
        return _name_only(root, ["diff", "--name-only", "--diff-filter=ACMR", "HEAD~1"])
    return None


def _clean_tree_diff_base(root: Path) -> str | None:
    """Merge-base with upstream or local main/master for clean-tree auto mode."""
    for candidate in _upstream_candidates(root):
        mb = _git(root, ["merge-base", "HEAD", candidate])
        if mb:
            tip = mb.strip()
            if tip:
                return tip
    return None


def _upstream_candidates(root: Path) -> list[str]:
    names: list[str] = []
    upstream = _git(root, ["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{upstream}"])
    if upstream:
        ref = upstream.strip()
        if ref and ref != "HEAD":
            names.append(ref)
    origin_head = _git(root, ["rev-parse", "--abbrev-ref", "origin/HEAD"])
    if origin_head:
        ref = origin_head.strip()
        if ref and ref not in names:
            names.append(ref)
    for local in ("main", "master"):
        if _git(root, ["rev-parse", "--verify", local]) is not None and local not in names:
            names.append(local)
    return names


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
