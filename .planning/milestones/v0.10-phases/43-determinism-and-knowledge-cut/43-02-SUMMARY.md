---
phase: 43-determinism-and-knowledge-cut
plan: 02
subsystem: testing
tags: [known-on, horn, characterization]
requires: [{phase: 43-01, provides: DET-01 seeded properties}]
provides: [KNOWN-04 Horn vs as_known_on characterization]
affects: []
actuals: {tokens: 3413, tasks: 2, commits: 2}
tech-stack: {added: [], patterns: [tests/common shared statute fixture]}
key-files:
  created: [crates/kutha-runtime/tests/m017_known_on_horn.rs, crates/kutha-runtime/tests/common/mod.rs]
  modified: [docs/ADR/ADR-013-bitemporal-facts-invalidation.md, .kutha/dictionaries/honeycomb.yaml, CHANGELOG.md, crates/kutha-runtime/tests/m014_as_known_on.rs, crates/kutha-runtime/tests/m016_known_on_hardening.rs, crates/kutha-runtime/tests/m016_golden_fixture.rs]
key-decisions: ["F05 confirmed: Horn remaining-premise lookup is TT=MAX", "leased_pin_and_derive skipped (3-tuple vs 4-tuple)"]
patterns-established: ["characterization asserts today's fold vs as_known_on; eval.rs untouched"]
requirements-completed: [KNOWN-04]
coverage:
  - id: K1
    description: KNOWN-04 Horn-at-MAX vs as_known_on(early) hiding the later-visible premise
    requirement: KNOWN-04
    verification: [{kind: unit, ref: "crates/kutha-runtime/tests/m017_known_on_horn.rs#known04_horn_fires_when_premise_hidden_on_early_cut", status: pass}]
    human_judgment: false
duration: 8min
completed: 2026-10-03
status: complete
plan_head_before: a8df917a44f68e5c08c81281be7997f9bb2b7842
plan_head_after: 91ae202621e0f0b4680f9db298a7a35827504059
---

# Phase 43 Plan 02: KNOWN-04 Summary

**Two-premise Horn mints derived `relatedTo(A,C)` at `TT=MAX` while `as_known_on(2014)` hides A–B; eval unchanged.**

## Characterization
`TtAnchor` 2014 → Assert A–B then B–C (vf 2010) → `TtAnchor` 2020. Fold: derived live. `as_known_on(2014)` lacks A–B and derived; `as_known_on(2020)` has A–B. `horn_fires` → `search_prems(..., u64::MAX)`; `live_horns` `is_live_at(u64::MAX, vt)`. Decision pending.

## Mechanical edits (old → new)
| Old | New |
|-----|-----|
| `m014_as_known_on.rs` / `m016_known_on_hardening.rs` `seed_statute_between_anchors` | `tests/common/mod.rs` (same body; every assert kept) |
| FIXG-01 `assert!(C01.contains("15.1"))` and `01.01.2028` | Dropped; `RuleTerm::constant` none + VT vs known-on remain |

`leased_pin_and_derive` skipped (arity / `live_count` diverge).

## Task commits
1. `4b7defe` test(43-02): KNOWN-04 Horn vs known-on cut
2. `91ae202` test(43-02): share statute fixture; drop FIXG-01 tautology

## Counts
`cargo test --workspace --offline`: 180 passed / 0 failed (53 bins); `m017_known_on_horn` 1/1. `uv run kutha-gov precommit` HIGH 0 (ci not run; controller).
CBM `root-kutha-graph`: `live_horns` inbound 3 (`horn_fires`/`rule_follow_ons`); `eval.rs` `metadata_match`; new test `not_tracked` (index @ c702d71). GN `kutha-graph`: `horn_fires` upstream 1; `as_known_on` 0 (rg wins).
rg: `is_live_at(u64::MAX` eval.rs:147; `search_prems(..., u64::MAX)` eval.rs:123.

## Deviations
None on eval. D-04 partial: statute shared; pin helper skipped as planned. CHANGELOG Product+Trajectory in test commits. `.kutha/STATE.md` untouched.

## Self-Check: PASSED
