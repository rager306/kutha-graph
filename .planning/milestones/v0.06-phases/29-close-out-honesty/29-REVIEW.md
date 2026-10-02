---
phase: 29-close-out-honesty
reviewed: 2026-10-02T05:30:00Z
depth: standard
files_reviewed: 28
files_reviewed_list:
  - scripts/kutha_gov/kinds.py
  - scripts/kutha_gov/fsm.py
  - scripts/kutha_gov/selftest.py
  - scripts/tests/test_fsm.py
  - scripts/tests/test_kutha_gov.py
  - .kutha/META.md
  - .kutha/ROADMAP.md
  - .kutha/dictionaries/checks.yaml
  - .kutha/dictionaries/fsm.yaml
  - .kutha/dictionaries/honeycomb.yaml
  - .kutha/dictionaries/invariants.yaml
  - docs/process/governor-intake.md
  - docs/process/kutha-harness.md
  - docs/process/gsd-auto-contract.md
  - docs/process/codex-subagents.md
  - docs/architecture/semantic-gap-review.md
  - docs/ADR/README.md
  - docs/ADR/ADR-000-kutha-hybrid-architecture-research.md
  - docs/ADR/ADR-010-event-log-runtime-quantum.md
  - docs/ADR/ADR-011-lean-event-schema-lineage.md
  - docs/ADR/ADR-012-snapshots-tiers-vacuum.md
  - docs/ADR/ADR-013-bitemporal-facts-invalidation.md
  - docs/ADR/ADR-014-cascade-budgets-quantum-receipts.md
  - docs/ADR/ADR-040-materialization-plugin-protocol.md
  - docs/ADR/ADR-041-csr-graphblas-hot-path.md
  - docs/ADR/ADR-050-meta-prompt-dictionaries.md
  - docs/ADR/ADR-060-strict-replay.md
  - docs/ADR/ADR-061-fork-and-diff.md
findings:
  critical: 1
  warning: 3
  info: 1
  total: 5
status: issues_found
---

# Phase 29: Code Review Report

**Status:** issues_found. CBM stale (`metadata_changed`); evidence from Read/rg. `min_phase` parses `29-close-out-honesty` as 29. `skipped>0` HIGH is CUI budget only — `ci --skip-cargo` stays green when budget ≥ check count.

## Critical Issues

### CR-01: `frontmatter_paths_exist` accepts paths outside the repo

**File:** `scripts/kutha_gov/kinds.py:1890-1910`
**Issue:** Only `startswith("/")` is blocked. `(ctx.root / rel_path).is_file()` follows `..`. Here `../../etc/passwd` and `../law-nexus/README.md` exist and **pass**. META requires repo-relative. Tests omit traversal. Selftest `_resolve_in` already rejects `..`.
**Fix:** Reject `".." in Path(rel).parts`; `resolve()` + `relative_to(ctx.root.resolve())`; HIGH on escape; then `is_file()`.

## Warnings

### WR-01: Empty `read_only_inputs` list fail-open

**File:** `scripts/kutha_gov/kinds.py:1865-1877`
**Issue:** Missing key → HIGH; `read_only_inputs:` with zero items returns `[]` and emits nothing. Empty glob is HIGH; empty field is not.
**Fix:** HIGH when the parsed list has no non-empty strings.

### WR-02: Frontmatter is not YAML (`safe_load` never runs)

**File:** `scripts/kutha_gov/kinds.py:1783-1803`
**Issue:** Line scanner, not `yaml.safe_load`. `YAMLError` is never HIGH (unlike `_yaml_document`). Quoted `"docs/a.md"` looks up a filename that includes quotes (false HIGH). Flow lists look like a missing key.
**Fix:** `yaml.safe_load(block)`; HIGH on error or non-list; use loaded scalars.

### WR-03: Restaged I-F checks are presence-not-meaning

**File:** `.kutha/dictionaries/checks.yaml:1268-1376` · `invariants.yaml:141-191`
**Issue:** Eleven `rust_test_asserts` rows have no `require_symbols`. `i-f11` cites a test that `let _ =` justification/action (no `UnknownRelation` assert). Comments still say I-F rows are “not enforced” while `disposition: check`.
**Fix:** Pin `require_symbols`; assert Err on those constructors; rewrite the comments.

## Info

### IN-01: META `file_contains` omits glob / `min_phase` (`.kutha/META.md:54`) — kind now globs; META still reads as a single-file needle check.

---
_Reviewed: 2026-10-02T05:30:00Z · Reviewer: Claude (gsd-code-reviewer) · Depth: standard_
