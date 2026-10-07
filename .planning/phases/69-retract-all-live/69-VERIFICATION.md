---
independent: true
verifier_model: composer-2.5-fast
status: passed
phase: 69
milestone: M021
slice: S02
verified_at: 2026-10-07
---

# Phase 69 independent verification (M021 S02 Retract-all-live)

Verifier did **not** read `.planning/phases/69-retract-all-live/*PLAN*`, `*SUMMARY*`, or other executor artifacts under that directory.

## Requirement checked

| Req ID | Criterion | Result |
|--------|-----------|--------|
| RETRACT-01 | `Op::Retract { event_id }` invalidates every live Fact with that event id (CorrectInterval multi-residual); RED first; bench ≤10% | **pass** |

## Read-only inputs

- `.planning/REQUIREMENTS.md` (RETRACT-01 pin only; not phase PLAN/SUMMARY)
- `crates/kutha-runtime/src/fold.rs` (`Op::Retract` arm via `rg`)
- `crates/kutha-runtime/tests/m021_retract_all_live.rs`, `m019_sem_meta.rs` (via `cargo test`)
- `docs/architecture/test-matrix.yaml` (`SEM-retract-all-live`)
- `CHANGELOG.md` (Product Phase 69 entry; Phase 66 bench baseline from M020 cumulative line)
- CBM: `list_projects`, `check_index_coverage` on `fold.rs` and `m021_retract_all_live.rs`

## Kill-tests (commands and exit codes)

| # | Command | Exit | Notes |
|---|---------|------|-------|
| 1 | `set -a && source .env && set +a && unset CARGO_TARGET_DIR && cargo test -p kutha-runtime --offline --test m021_retract_all_live -- --exact` | 0 | **2/2** passed: `retract_correct_interval_event_id_clears_all_residuals`, `retract_assert_event_id_still_clears_single_row` |
| 2 | `set -a && source .env && set +a && unset CARGO_TARGET_DIR && cargo test -p kutha-runtime --offline --test m019_sem_meta -- horn_correct_interval_prefix_suffix_derived_remain_live --exact` | 0 | **1/1** passed |
| 3 | `rg -n 'Op::Retract' -A25 crates/kutha-runtime/src/fold.rs \| head -40` | 0 | `Op::Retract` at ~698: collects **all** live indices (`facts` scan when cold; `fact_by_event` multi-index when warm); `for idx in live_idxs { invalidate_fact_at(...) }`. Does **not** use `fact_index_by_event_id` single-first-live (that helper remains on `Op::Correct` / `Op::CorrectInterval` only). |
| 4 | `rg 'SEM-retract-all-live' -A8 docs/architecture/test-matrix.yaml` | 0 | Row present; `status: killed`; kill_mutation documents first-live-only retract |
| 5 | CHANGELOG Product Phase 69 bench cites | — | Entry records `KUTHA_BENCH_N=10000`: `ingest_plain.n` **132950275→120718182**, `as_of.n` **315267→242161** (vs Phase 66 post per same file M020 cumulative line). Arithmetic vs Phase 66 post: ingest −9.1%, as_of −23.2% (improvements; no >10% regression on either metric). |

## Kill-test score

**5 / 5** kill-test gates green.

## CBM coverage note

`list_projects`: `kutha-graph` indexed. `check_index_coverage`: `fold.rs` `metadata_changed`; `m021_retract_all_live.rs` `not_tracked`. Verification relied on executed tests and source/`rg`, not graph completeness.

## Verdict

RETRACT-01 is satisfied: retract-all-live oracles green, matrix kill recorded, fold invalidates all live rows per event id, CHANGELOG documents bench numbers with no >10% regression vs Phase 66 post.
