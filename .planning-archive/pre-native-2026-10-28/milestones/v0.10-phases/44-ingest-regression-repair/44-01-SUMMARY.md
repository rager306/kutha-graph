---
phase: 44-ingest-regression-repair
plan: 01
subsystem: ingest-perf
tags: [PERF-01, bisect, horn-examine]
status: complete
completed: 2026-10-03
duration: 26min
requirements-completed: [PERF-01]
actuals: { tokens: 1259, tasks: 2, commits: 2 }
plan_head_before: 4d669ca97f539a82e5883121fcd656a76c33d29c
plan_head_after: 1866721e844f6c77de04ae8c671fb68a17864282
key-files:
  created: [crates/kutha-runtime/tests/m017_ingest_scaling.rs]
  modified: [crates/kutha-runtime/src/fold.rs, crates/kutha-runtime/src/eval.rs, CHANGELOG.md]
key-decisions:
  - "D-01 culprit cfe4a1f: live_horns scans facts() twice per extra relatedTo (genesis inverse_knows)"
---
# Phase 44 Plan 01: Ingest bisect + RED oracle Summary

Same-host N=10000 Eager bisect names `cfe4a1f` (38-01 Horn eval); RED `extra_relatedto_horn_visits_constant_in_n` fails `[144, 528]`.

## Bisect `ingest_plain.n` / `as_of.n` (ns)

| commit | ingest_plain.n | as_of.n |
| 1fee7a7 v0.08 | 115322845 | 252890 |
| 598ee74 37-01 | 117579077 | 256619 |
| 3947ff5 37-02 | 116139730 | 309768 |
| **cfe4a1f 38-01** | **272562212** | 301061 |
| c9ec151 38-02 | skip (no runtime/common src) | — |
| 59e52d9 39-01 | 300166637 | 245297 |
| 6b11346 39-fix | 327563448 | 277041 |
| f0a5c61 / 428f3ff | skip (no src) | — |
| 4d669ca HEAD | 296189698 | 287957 |

First >10% jump: **cfe4a1f +134.7%** vs 3947ff5. 59e52d9 +10.1% is a second policy step, not the 108M→301M tripling. Two-run means when within 15% of previous.

## Culprit / profile (throwaway `cfe4a1f` worktree, uncommitted)

O(N) call: `Runtime::live_horns` over `fold.facts()` (rule_follow_ons ×2). Extra relatedTo: n=64 facts=73 live_horns_rows=144 search_at=0 admit=1 from_canonical=2 retract_derived=0; n=256 facts=265 live_horns=528 (others constant). inverse_knows premise is `knows`, so relatedTo still pays the full facts scan.

## RED / hook

`horn_examine` on GraphFold (`serde skip`, not fingerprint); bump per `live_horns`/`search_at` row. Test panics `horn visits must not grow with N: [144, 528]`. Commit `1866721`. No ingest repair.

## v0.08..v0.09 crates log

4931d5b 36-01 trybuild; c008f77 cite swap; 217e657 known-on; 635028f snapshot tail; 598ee74 AST; 3947ff5 hash; 8a138ec/c784361 37 tests; cfe4a1f Horn; c9ec151 n-ary; 58a33cb 38 tests; 59e52d9 policy; f8a9d72 39 tests; 6b11346 pin skip; f0a5c61 C01.

## Deviations

[Rule 1] 598ee74 r2=162650131 ns host noise (r1=117.6M, r3=128.9M); table uses r1 so AST is not a false culprit. STATE.md untouched.

## Self-Check: PASSED
