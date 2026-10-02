---
phase: 28-independent-verification-and-artifact-budget
reviewed: 2026-10-02T01:51:00Z
depth: standard
files_reviewed: 9
files_reviewed_list:
  - scripts/kutha_gov/kinds.py
  - scripts/kutha_gov/selftest.py
  - scripts/tests/test_kutha_gov.py
  - .kutha/META.md
  - .kutha/dictionaries/checks.yaml
  - .kutha/dictionaries/invariants.yaml
  - .planning/phases/28-independent-verification-and-artifact-budget/28-01-SUMMARY.md
  - .planning/phases/28-independent-verification-and-artifact-budget/28-02-SUMMARY.md
  - .planning/v0.06-MILESTONE-AUDIT.md
findings:
  critical: 0
  warning: 2
  info: 2
  total: 4
status: issues_found
---

# Phase 28: Code Review Report

**Reviewed:** 2026-10-02T01:51:00Z  
**Depth:** standard  
**Files Reviewed:** 9  
**Status:** issues_found  

## Summary

Reviewed harness changes from `8b6701a..HEAD` (7 commits): new `glob_max_lines` kind, `artifact-budget` / `verification-independent` / `id-map-cite` dictionary rows, selftest derivation, unit tests, milestone audit artifact, and Phase 28 summaries. Scope excluded unrelated WIP (untracked `28-VERIFICATION.md`).

**Verified at review time:** `uv run pytest` GlobMaxLinesTests + VerificationIndependentTests (11 passed); `uv run kutha-gov precommit --check artifact-budget|verification-independent|id-map-cite` (all OK); `uv run kutha-gov selftest artifact-budget` → `OK artifact-budget mutations=4`, `OK verification-independent mutations=7`; `git diff 8b6701a..HEAD -- crates/ .kutha/STATE.md` empty.

Implementation matches D-02/D-03/D-05 intent: line budgets use `_wc_lines`, `min_phase` filters `.planning/phases/NN-*`, independence needles are literal `-PLAN.md` / `-SUMMARY.md` (no false positives in committed 22–27 VERIFICATION bodies). No Critical-tier defects found; two robustness/coverage warnings remain.

## Critical Issues

_(none)_

## Warnings

### WR-01: `glob_max_lines` passes when the glob matches zero files

**File:** `scripts/kutha_gov/kinds.py:1682-1704`  
**Issue:** If the glob matches no files (typo, deleted tree, or `min_phase` filters everything), `_kind_glob_max_lines` completes with `scanned=0` and no findings. The budget check then passes even though no artifacts were validated—fail-open on scope loss, not on line excess.  
**Fix:** After the loop, if `min_phase > 0` or the check is budget-related, require at least one hit (or a separate `glob_none` / minimum-count step). Example guard:

```python
    if not counts and min_phase > 0:
        result.findings.append(
            Finding(check, Severity.HIGH, "max-lines", "glob matched no files under min_phase filter")
        )
```

Alternatively document that CI must always have phase ≥22 artifacts present (weaker).

### WR-02: `verification-independent` frontmatter steps omit Phase 28

**File:** `.kutha/dictionaries/checks.yaml:1228-1250`  
**Issue:** `file_contains` steps are hard-coded for phases 22–27 only. D-05 requires every v0.06 `NN-VERIFICATION.md` to carry independence frontmatter; Phase 28 is intentionally deferred (D-08), but once `28-VERIFICATION.md` lands it will only be covered by `glob_absent`, not the structural `independent: true` / `commands:` / `read_only_inputs:` / `exit:` bundle—unless YAML is updated.  
**Fix:** When committing `28-VERIFICATION.md`, append a `file_contains` step for `.planning/phases/28-independent-verification-and-artifact-budget/28-VERIFICATION.md` with the same needles as 22–27, or replace the six steps with one `yaml_needles_in_glob` / glob-scoped pattern if the dictionary supports it.

## Info

### IN-01: No unit test for zero-match `glob_max_lines` pass

**File:** `scripts/tests/test_kutha_gov.py:1063-1156`  
**Issue:** GlobMaxLinesTests cover oversize, per-dir cap, and `min_phase`, but not the empty-match path (WR-01).  
**Fix:** Add a test with an empty phase directory and assert either pass (documented) or HIGH once WR-01 is fixed.

### IN-02: Milestone audit and PROC-02 honestly leave a process gap

**File:** `.planning/v0.06-MILESTONE-AUDIT.md` (frontmatter `status: gaps_found`)  
**Issue:** Audit correctly records missing committed `28-VERIFICATION.md` and open PROC-02; not a code defect, but reviewers should treat milestone close as blocked until independent verify-work ships.  
**Fix:** None in this diff; complete D-08 verification in a follow-up wave.

---

_Reviewed: 2026-10-02T01:51:00Z_  
_Reviewer: Claude (gsd-code-reviewer)_  
_Depth: standard_
