---
phase: 24-fold-indexes-for-ingest
verified: 2026-10-01T15:10:00Z
status: passed
score: 3/3 requirements verified (IDX-03 via controller override)
independent: true
verifier_model: composer-2.5-fast
behavior_unverified: 0
overrides_applied: 1
overrides:
  - must_have: "IDX-03 / D-06 — ingest and as_of regress no more than 10% vs Phase 22 under Eager default"
    reason: "Absolute ingest_plain.n remains +17…+29% vs Phase 22; controller accepted write-cost on Eager default and shipped Lazy (thin write) via KUTHA_FOLD_INDEX_MODE; visit oracle remains the index correctness gate."
    accepted_by: "controller (interim decision 2026-10-01)"
    accepted_at: "2026-10-01T00:00:00Z"
covered_files:
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/phases/24-fold-indexes-for-ingest/24-01-PLAN.md
  - .planning/phases/24-fold-indexes-for-ingest/24-02-PLAN.md
  - .planning/phases/24-fold-indexes-for-ingest/24-01-SUMMARY.md
  - .planning/phases/24-fold-indexes-for-ingest/24-02-SUMMARY.md
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/tests/m013_fold_indexes.rs
  - crates/kutha-runtime/tests/m013_fold_index_mode.rs
  - docs/architecture/benchmark-baseline.md
  - .env.example
  - .compound-engineering/artifacts/research/notes/2026-10-01-d06-index-lease-interim-decision.md
covered_digest: "v2:sha256:2581cce0aced48eb65036a16e5ae1d06871a4a30cd965d5b96688a109fac3b89"
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/tests/m013_fold_indexes.rs
  - crates/kutha-runtime/tests/m013_fold_index_mode.rs
  - docs/architecture/benchmark-baseline.md
  - .env.example
  # PLAN/SUMMARY listed in covered_files for GSD fingerprint gate only; not used for kill-tests.
commands:
  - cmd: "cargo test -p kutha-runtime --offline --test m013_fold_indexes -- --exact"
    exit: 0
  - cmd: "cargo test -p kutha-runtime --offline --test m013_fold_index_mode"
    exit: 0
  - cmd: "cargo test -p kutha-runtime --offline --lib"
    exit: 0
  - cmd: "rg -n 'facts\\(\\)\\.iter\\(\\)\\.(any|find)' crates/kutha-runtime/src"
    exit: 0
  - cmd: "rg -n 'log\\.iter\\(\\)\\.find' crates/kutha-runtime/src"
    exit: 1
  - cmd: "codebase-memory-mcp list_projects"
    exit: 0
  - cmd: "codebase-memory-mcp check_index_coverage (cited paths)"
    exit: 0
---

# Phase 24: Fold indexes for ingest — Independent Verification

**Phase goal (ROADMAP):** Ingest, retract, correct, claim admit, and follow-ons cost O(1) in log size.

**Verified:** 2026-10-01T15:05:29Z  
**Status:** passed  
**Re-verification:** Yes — staleness refresh after REQUIREMENTS.md IDX-01..03 marked [x]; kill-tests 1–2 re-run green  
**Inputs:** REQUIREMENTS IDX-01..03, ROADMAP success criteria, public runtime API, oracles, benchmark baseline, `.env.example` only (no PLAN/SUMMARY).

## Kill-tests

| # | Command / check | Result | Status |
|---|-----------------|--------|--------|
| 1 | `cargo test -p kutha-runtime --offline --test m013_fold_indexes -- --exact` | 3/3 pass (`green_ingest_visits_constant_in_n`, persist/retract, fork) | ✓ |
| 2 | `cargo test -p kutha-runtime --offline --test m013_fold_index_mode` | 2/2 pass | ✓ |
| 3 | `cargo test -p kutha-runtime --offline --lib` | 17/17 pass | ✓ |
| 4 | D-03 `rg` arbiter: `facts().iter().(any\|find)` in `src/` | 1 hit: `quantum.rs:1378` inside `#[cfg(test)]` assert only | ✓ |
| 5 | D-03: `log.iter().find` in `src/` | no matches | ✓ |
| 6 | `FoldIndexMode` + `KUTHA_FOLD_INDEX_MODE` in `.env.example` | present with bench/docs pointers | ✓ |
| 7 | `benchmark-baseline.md` Phase 24 + FoldIndexMode | D-06 regression documented; controller Eager/Lazy acceptance recorded | ✓ |

## Roadmap success criteria

| # | Criterion | Status | Evidence |
|---|-----------|--------|----------|
| 1 | Scaling oracle fails pre-change, passes after | ✓ VERIFIED | `green_ingest_visits_constant_in_n`: visits at N=256 ≤ N=64 + 8; test green |
| 2 | No whole-log / whole-fact-vector scan on ingest paths (Eager default) | ✓ VERIFIED | `Runtime::event_by_id` / delivery-key path use `by_event_id` / `by_delivery_key`; fold lookups use `claim_facts`, `fact_by_event`, `live_triples` when `hot_maps_warm` (Eager). Cold `facts.iter` only when `!hot_maps_warm` (Lazy lease). Full `log.iter` on hydrate/replay/provenance, not per-assert emit |
| 3 | Baseline ≤10% ingest/as_of + named oracles green | ✓ PASSED (override) | Oracles green (kill-tests 1–3). Absolute ingest_plain.n +17…+29% vs Phase 22 documented; **not** ≤10% under Eager. Override: `benchmark-baseline.md` § Phase 24 delta + § FoldIndexMode; interim note `2026-10-01-d06-index-lease-interim-decision.md` |

## Requirements (REQUIREMENTS.md)

| ID | Status | Evidence |
|----|--------|----------|
| IDX-01 | ✓ VERIFIED | Eager default warms maps on `push_fact`; ingest paths in `quantum.rs` call indexed `claim_known`, `fact_index_by_event_id`, `has_live_triple` with `ingest_examine` bumps only |
| IDX-02 | ✓ VERIFIED | Visit oracle test + `ingest_examine_count` API; constant-in-N assertion |
| IDX-03 | ✓ PASSED (controller override) | Re-bench recorded in `benchmark-baseline.md`; D-06 absolute gate explicitly **not met** for ingest under Eager; controller accepted Eager write-tax + Lazy config instead of blocking close |

## Public API (verified surfaces)

| Surface | Provides |
|---------|----------|
| `lib.rs` | Re-exports `FoldIndexMode`, `GraphFold`, `Runtime` |
| `fold.rs` | `FoldIndexMode::{Eager,Lazy}`, `from_env`, hot-map leases, `ensure_hot_maps` |
| `quantum.rs` | `with_index_mode`, `set_index_mode`, `index_mode`, `ensure_fold_indexes`, `ingest_examine_count` |

## D-06 / FoldIndexMode documentation

- **Regression facts:** Phase 22 `ingest_plain.n` 95383904 ns; Phase 24 runs +16.9%…+28.5% (table in `benchmark-baseline.md` § Phase 24 delta).
- **Controller policy:** Eager default = accept write-cost for O(1) leases; Lazy (`lazy`|`thin`) = thin write with scan/rebuild path; bench prints `fold_index_mode`.
- **Override is on record** in baseline doc and interim decision note — not a silent waiver.

## Code graph (advisory)

- CBM: `list_projects` OK; cited paths show `metadata_changed` / `not_tracked` — verification used source + `rg`, not graph-only claims.
- GitNexus `detect_changes`: high churn elsewhere in tree; Phase 24 evidence is oracle + allowed source reads.

## Observable truths

| Truth | Status |
|-------|--------|
| Ingest examine count does not grow with N (Eager) | ✓ |
| Hot-map indexes wired on default ingest path | ✓ |
| No production `facts().iter().any/find` scan sites (rg arbiter) | ✓ |
| Named runtime oracles green | ✓ |
| D-06 absolute ≤10% under Eager | Override accepted (documented) |

**Gaps:** none for phase close under controller D-06 policy.

---

_Verified: 2026-10-01T15:05:29Z_  
_Verifier: gsd-verifier (composer-2.5-fast, independent)_
