---
phase: 24-fold-indexes-for-ingest
verified: 2026-10-01T15:10:00Z
status: passed
score: 3/3 (IDX-03 controller override)
independent: true
verifier_model: composer-2.5-fast
behavior_unverified: 0
overrides_applied: 1
overrides:
  - must_have: "IDX-03 / D-06 ingest/as_of ≤10% vs Phase 22 Eager"
    reason: "ingest_plain.n +17…+29% vs Phase 22; Eager write-cost + Lazy via KUTHA_FOLD_INDEX_MODE accepted."
    accepted_by: "controller (interim 2026-10-01)"
    accepted_at: "2026-10-01T00:00:00Z"
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/tests/m013_fold_indexes.rs
  - crates/kutha-runtime/tests/m013_fold_index_mode.rs
  - docs/architecture/benchmark-baseline.md
  - .env.example
commands:
  - cmd: "cargo test -p kutha-runtime --offline --test m013_fold_indexes -- --exact"; exit: 0
  - cmd: "cargo test -p kutha-runtime --offline --test m013_fold_index_mode"; exit: 0
  - cmd: "cargo test -p kutha-runtime --offline --lib"; exit: 0
  - cmd: "rg -n 'facts\\(\\)\\.iter\\(\\)\\.(any|find)' crates/kutha-runtime/src"; exit: 0
  - cmd: "rg -n 'log\\.iter\\(\\)\\.find' crates/kutha-runtime/src"; exit: 1
  - cmd: "codebase-memory-mcp list_projects"; exit: 0
  - cmd: "codebase-memory-mcp check_index_coverage (cited paths)"; exit: 0
---

# Phase 24: Fold indexes — independent verification

**IDX-01..03.** Inputs: REQUIREMENTS, public API, oracles, baseline, `.env.example`.

## Kill-tests

| # | Result |
|---|--------|
| 1 m013_fold_indexes `--exact` | 3/3 pass |
| 2 m013_fold_index_mode | 2/2 pass |
| 3 `--lib` | 17/17 pass |
| 4 `facts().iter().(any\|find)` in src | 1 hit: `quantum.rs` `#[cfg(test)]` only |
| 5 `log.iter().find` in src | no matches |
| 6 FoldIndexMode + `.env.example` | present |
| 7 baseline Phase 24 + mode | D-06 documented; override on record |

## Requirements

| ID | Status |
|----|--------|
| IDX-01 | ✓ Eager maps on ingest |
| IDX-02 | ✓ visit oracle constant-in-N |
| IDX-03 | ✓ override: ingest +17…+29% vs Phase 22; not ≤10% Eager |

**Gaps:** none under controller D-06 policy.
