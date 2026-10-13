---
phase: 144-git-auto-base
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: uv run pytest scripts/tests/test_gitdiff_auto.py scripts/tests/test_ci_summary_labels.py -q
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - .planning/phases/144-git-auto-base/144-CONTEXT.md
  - .kutha/META.md
  - .kutha/dictionaries/checks.yaml
  - .kutha/dictionaries/fsm.yaml
  - scripts/kutha_gov/gitdiff.py
  - scripts/kutha_gov/__main__.py
---

# Phase 144 — independent verification

**Verdict:** passed. Clean-tree auto uses merge-base window; ci labels separate this-run HIGH from fold tip; check `git-auto-base` green at budget 83.
