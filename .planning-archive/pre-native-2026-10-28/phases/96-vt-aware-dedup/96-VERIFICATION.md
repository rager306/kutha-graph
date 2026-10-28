---
phase: 96-vt-aware-dedup
verified: 2026-10-10T03:45:00Z
independent: true
verifier_model: composer-2.5-fast
status: passed
read_only_inputs:
  - .planning/phases/96-vt-aware-dedup/96-CONTEXT.md
  - crates/kutha-runtime/src/fold_query.rs
  - crates/kutha-runtime/src/eval.rs
  - crates/kutha-runtime/tests/m036_vt_aware_dedup.rs
commands:
  - { cmd: "cargo test -p kutha-runtime --test m036_vt_aware_dedup --offline", exit: 0 }
  - { cmd: "cargo test -p kutha-runtime --test m034_correct_refire --offline", exit: 0 }
  - { cmd: "cargo test --workspace --offline --no-fail-fast", exit: 0 }
  - { cmd: "uv run kutha-gov ci", exit: 0 }
---

# Phase 96 — Independent Verification

Kill-claim: two Asserts of the same triple with disjoint VT windows produce only one derived inverse because `has_live_triple` ignores valid-time.

Workspace ci at close (0 HIGH). Quiet bench median within ≤10% ingest regression vs M035 / Phase 80.

**Verdict:** passed.
