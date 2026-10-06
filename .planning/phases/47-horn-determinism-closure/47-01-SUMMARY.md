---
phase: 47-horn-determinism-closure
plan: 01
subsystem: runtime
tags: [horn, eventid, cascade, admit]
requires:
  - phase: 46-c01-knowledge-cut
    provides: M018 S02 C01/HORN-KC/ASOF
provides:
  - hash-stable derived EventIds
  - Correct/CorrectInterval re-derive
  - cascade hop admit
affects: [eval, pipeline, Event, test-matrix]
actuals:
  tokens: 9506
  tasks: 3
  commits: 3
  plan_head_before: 56d0001d82d592e8b85d1dd2db3b57b02912cf43
  plan_head_after: 017b46fd632ff745a104b4b5643c135ea24c6091
tech-stack:
  added: []
  patterns: [Event::stable_derived analog of stable_define]
key-files:
  created: []
  modified: [crates/kutha-common/src/event.rs, crates/kutha-runtime/src/eval.rs, crates/kutha-runtime/src/pipeline.rs, crates/kutha-runtime/tests/m019_sem_meta.rs, docs/architecture/test-matrix.yaml, .kutha/dictionaries/honeycomb.yaml, CHANGELOG.md]
key-decisions:
  - "stable_derived hashes kutha-derived-id + pin + ingested_at + bindings + B/R; not user Assert v7"
  - "UnknownRelation only when a matched firing Horn head is missing; unmatched dict.id None continues"
requirements-completed: [DET-02, HORN-CORR-01, CASC-01]
coverage:
  - {id: D1, description: two-ingest derived Behavior ids equal, requirement: DET-02, verification: [{kind: unit, ref: crates/kutha-runtime/tests/m019_sem_meta.rs#det02_two_ingest_derived_event_ids_equal, status: pass}], human_judgment: false}
  - {id: D2, description: stale derived not live after Correct, requirement: HORN-CORR-01, verification: [{kind: unit, ref: crates/kutha-runtime/tests/m019_sem_meta.rs#horn_correct_derived_not_live_after_premise_correct, status: pass}], human_judgment: false}
  - {id: D3, description: failing hop absent from log, requirement: CASC-01, verification: [{kind: unit, ref: crates/kutha-runtime/tests/m019_sem_meta.rs#casc01_disallowed_second_hop_absent_from_log, status: pass}], human_judgment: false}
duration: 11min
completed: 2026-10-06
status: complete
---
# Phase 47 Plan 01: Horn determinism closure Summary

**Hash-stable derived EventIds (`Event::stable_derived`), Correct re-derive, hop admit before push**

## Task Commits
1. `b47e833` test(47-01): RED two-ingest derived EventId equality
2. `6f8516b` feat(47-01): stable derived ids, Correct re-derive, hop admit
3. `017b46f` docs(47-01): matrix ADR honeycomb changelog planes

## Deviations
- Classifier `INVALID_RED` (cargo libtest unsupported); semantic RED: det02 FAILED on UUID v7 inequality; GREEN `ok. 1 passed`. Plan type execute / tdd_mode false.
- `kutha-gov ci` 1 HIGH on pre-existing `46-VERIFICATION.md` missing needle `exit:` (out of slice). Planes precommit green. Selftest not overlapped.

## D-SCOPE
Honeycomb map **Proposed**. `.kutha/STATE.md` untouched. No `47-VERIFICATION.md`. `search_prems` still `u64::MAX`. User `Event::new` stays UUID v7. FF5 unchanged.

## Self-Check: PASSED
