---
phase: 38-rule-evaluator
plan: 01
subsystem: rules
tags: [Horn, evaluator, inverse_knows]
requires:
  - phase: 37-rule-ast-and-hash
    provides: RuleAst Horn canonical bytes and body hash
provides:
  - Horn unify over live triples
  - genesis inverse_knows Horn
affects: [kutha-runtime, 38-02]
actuals:
  tokens: 3332
  tasks: 1
  commits: 1
plan_head_before: e4cfec887d5ac96e6336a96011c3d5787960cba4
plan_head_after: cfe4a1f2ee58d50526e5c2a915908c47e73775fe
tech-stack:
  added: []
  patterns: [Horn eval on Assert/Behavior; Horn-only derived Retract]
key-files:
  created:
    - crates/kutha-runtime/src/eval.rs
  modified:
    - crates/kutha-runtime/src/pipeline.rs
    - crates/kutha-runtime/src/quantum.rs
    - crates/kutha-runtime/tests/m012_multi_hop.rs
    - CHANGELOG.md
key-decisions:
  - "D-01: genesis Horn replaces follow_ons knows special-case"
  - "D-04: residual-spike pin is Horn hash; Named two-hop facts stay live on root retract"
requirements-completed: []
duration: 12min
completed: 2026-10-02
status: complete
---

# Phase 38 Plan 01: Horn evaluator Summary

Genesis `knownBy(Y,X):-knows(X,Y)` fires `inverse_knows` with a hashed pin; hard-coded cascade deleted.

## Deviations from Plan

**1. [Rule 1 - Bug] Retract of Horn-derived inverse in this plan**
- **Found during:** Task 1
- **Issue:** Broad caused_by Retract invalidated Named two-hop facts.
- **Fix:** Retract only Behaviors whose pin decodes as Horn. `retract_keeps_loser` live_count 0.
- **Files:** eval.rs, quantum_invariants.rs
- **Commit:** cfe4a1f
