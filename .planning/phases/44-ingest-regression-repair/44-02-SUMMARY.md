---
phase: 44-ingest-regression-repair
plan: 02
subsystem: ingest-perf
tags: [PERF-02, horn-lease, live_horns]
status: complete
completed: 2026-10-03
duration: 18min
requirements-completed: [PERF-02]
requires:
  - phase: 44-01
    provides: RED extra_relatedto_horn_visits_constant_in_n + culprit live_horns
provides:
  - "D-05 horns_by_premise lease; ingest_plain.n within 10% of same-host v0.08"
affects: [M017-S04]
actuals: { tokens: 2719, tasks: 3, commits: 2 }
plan_head_before: 2d64891c6b35f204df800d064223ab29aa84261d
plan_head_after: 62770195cdcaecde2bb1b65e2eb5a921794371de
key-files:
  created: []
  modified: [crates/kutha-runtime/src/fold.rs, crates/kutha-runtime/src/eval.rs, crates/kutha-runtime/src/fold_query.rs, CHANGELOG.md, docs/architecture/benchmark-baseline.md]
key-decisions:
  - "D-05 lease keyed by Horn premise relation name; decode AST at rule index; skip facts() when asserted rel is not a premise"
---
# Phase 44 Plan 02: Horn premise-rel ingest lease Summary

`horns_by_premise` (serde skip, not fingerprint) maps premise relation → live Horn slots; `horn_fires` looks up the asserted relation. GREEN `extra_relatedto_horn_visits_constant_in_n`. ingest_plain.n within 10% of same-host v0.08.

## Bench (Eager, N=10000, two runs)

| | ingest_plain.n | as_of.n | vs 44-01 v0.08 115322845 | vs 428f3ff 300881900 |
| a | 111283177 | 401528 | −3.5% | −63% |
| b | 105460301 | 290851 | −8.6% | −65% |

Cap 126855129. Per-fact ~11.1 / 10.5 µs vs v0.08 ~11.5 µs. D-06 STOP: no.

## Tests

Workspace `--offline`: 184 passed, 0 failed. Named oracles (FF5/FF6/M010/M012/M013/M016/M017 incl. replay/determinism/known-on-horn) green. `kutha-gov ci` exit 0.

## Mechanical test edits

None (no assertion changes).

## Deviations

[Rule 2] `invalidate_fact_at` also drops horn slots (`fold_query.rs`) — T-44-02. CHANGELOG in feat commit for docs-coupling. Restored accidental `cargo fmt -p` on unrelated tests. `.planning/STATE.md` untouched (D-07 / lease). CBM coverage `metadata_changed`; no `index_repository`.

## Self-Check: PASSED
