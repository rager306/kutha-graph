"""M073 S01 — clean-tree git_against=auto uses upstream merge-base, not HEAD~1 alone."""

from __future__ import annotations

import subprocess
import tempfile
import unittest
from pathlib import Path

from kutha_gov.gitdiff import changed_relpaths


def _git(cwd: Path, *args: str) -> None:
    subprocess.run(
        ["git", *args],
        cwd=cwd,
        check=True,
        capture_output=True,
        text=True,
    )


class GitAutoCleanTreeTests(unittest.TestCase):
    def test_auto_clean_tree_uses_merge_base_not_head_parent_alone(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            _git(root, "init")
            _git(root, "config", "user.email", "m073@test")
            _git(root, "config", "user.name", "m073")
            (root / "a.txt").write_text("a0\n", encoding="utf-8")
            (root / "b.txt").write_text("b0\n", encoding="utf-8")
            _git(root, "add", "a.txt", "b.txt")
            _git(root, "commit", "-m", "base")
            _git(root, "branch", "-M", "main")
            _git(root, "checkout", "-b", "feature")
            (root / "a.txt").write_text("a1\n", encoding="utf-8")
            (root / "b.txt").write_text("b1\n", encoding="utf-8")
            _git(root, "add", "a.txt", "b.txt")
            _git(root, "commit", "-m", "both")
            (root / "a.txt").write_text("a2\n", encoding="utf-8")
            _git(root, "add", "a.txt")
            _git(root, "commit", "-m", "only-a")
            # Clean worktree: HEAD~1 is only-a (a.txt); branch vs main includes b.txt.
            head_parent = changed_relpaths(root, "head")
            self.assertEqual(head_parent, {"a.txt"})
            auto = changed_relpaths(root, "auto")
            self.assertIsNotNone(auto)
            assert auto is not None
            self.assertIn("a.txt", auto)
            self.assertIn(
                "b.txt",
                auto,
                msg="clean-tree auto must use merge-base…HEAD so branch mates stay visible",
            )


if __name__ == "__main__":
    unittest.main()
