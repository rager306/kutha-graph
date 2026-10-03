---
phase: 43-determinism-and-knowledge-cut
plan: 01
subsystem: testing
tags: [determinism, fold, replay, persist]
requires: [{phase: 42-replay-integrity, provides: atomic snapshot/WAL, FoldIndexMode pin}]
provides: [seeded DET-01 LCG properties]
affects: [43-02]
actuals: {tokens: 2534, tasks: 2, commits: 2}
tech-stack: {added: [], patterns: [seeded LCG in test source]}
key-files:
  created: [crates/kutha-runtime/tests/m017_determinism.rs]
  modified: [CHANGELOG.md]
key-decisions: ["k{i%3} DeliveryKeyConflict on distinct s{i} falls back to unkeyed Assert"]
patterns-established: ["Eager vs Lazy: sorted triples, never fingerprints"]
requirements-completed: [DET-01]
coverage:
  - id: D1
    description: DET-01 seeded replay/views/persist+open and delivery-key retry
    requirement: DET-01
    verification: [{kind: unit, ref: "crates/kutha-runtime/tests/m017_determinism.rs#det01_all_seeds_replay_eager_lazy_persist_open", status: pass}]
    human_judgment: false
duration: 8min
completed: 2026-10-03
status: complete
plan_head_before: 61c26cf49be00368c2ccdf0e4e39a31ca3ecb91d
plan_head_after: 0dd151a2a2879cb5a4f11bd7e16f70e92b96f7bd
---

# Phase 43 Plan 01: DET-01 Summary

**Seeded LCG mix (N=24) proves replay==fold, Eager/Lazy sorted views, persist+open, and delivery-key retry with no new crates.**

## Seeds / op mix
LCG `x=x.wrapping_mul(1664525).wrapping_add(1013904223); (x>>16)%100`. SEEDS `[1,7,13,42,99,12345,0xA5A5_A5A5,0xC0FFEE]`. N=24. 0–39 Assert relatedTo `s{i}` vf=2010+i%5 (1/4 `k{i%3}`); 40–54 Retract else Assert; 55–69 CorrectInterval 2015–2020 `P-prime` else Assert; 70–84 TtAnchor `2014+i` (skip reject); 85–99 intern `t{i}`.

## Kill-mutation (KILL_SEED=42)
Skip `CorrectInterval` in `apply`: seed 42 panics op 15 Retract unknown fact. Skip it in `GraphFold::replay`: `DET-01 fail seed=42 replay!=fold`. Both reverted; `src/` clean. No fold defect.

## Task commits
1. `d480d85` test(43-01): seeded replay and Eager/Lazy views
2. `0dd151a` test(43-01): DET-01 full seed list

## Counts
`cargo test --workspace --offline`: 179 passed / 0 failed (52 bins); `m017_determinism` 3/3. `uv run kutha-gov precommit` HIGH 0.
CBM `root-kutha-graph`: `GraphFold::replay` inbound 73; coverage fold.rs `metadata_changed`, new test `not_tracked` (index @ c702d71). GN `kutha-graph`: persist upstream 36, open 5 (1 dropped).
rg: `GraphFold::replay` 6 files; `emit_tt_anchor` 10 files; `FoldIndexMode` 10 files.

## Deviations
Rule 3: `k{i%3}` collides on distinct `s{i}` → unkeyed Assert. CHANGELOG in test commits (docs-coupling). No fold/eval patch. Named oracles untouched.

## Self-Check: PASSED
