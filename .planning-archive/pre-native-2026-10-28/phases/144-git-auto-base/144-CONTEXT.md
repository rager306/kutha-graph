---
phase: 144-git-auto-base
---

# Phase 144 — Context

M072 residual: clean-tree `git_against=auto` fell back to `HEAD~1`, so `docs-coupling` falsely HIGHed when the tip touched `checks.yaml` without `invariants.yaml`/`bridges.yaml` even though the branch window was coupled.

Thin slice: `_clean_tree_diff_base` (upstream / origin/HEAD / main merge-base…HEAD); `ci` summary prints `run_high` / `tip_high`. Check `git-auto-base`; budget 82→83.
