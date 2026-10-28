---
phase: 114-lock-before-heal
independent: true
verifier_model: parent-session-fresh-pass
commands:
  - cmd: cargo test -p kutha-runtime --test m052_lock_before_heal --offline
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: export KUTHA_GOV_BUDGET=76; uv run kutha-gov ci
    exit: 0
    note: test-matrix/changelog/docs-coupling 0 HIGH. Live tree HIGH on pre-existing 113 SUMMARY 143>60 and STATE Active Slice M052-S03 (pattern S\\d+|None).
  - cmd: KUTHA_BENCH_N=10000 taskset -c 0 cargo bench -p kutha-runtime --bench baseline -- --quick
    exit: 0
    note: three quiet runs; median ingest_plain.n ≈106.7M as_of.n ≈303.3k
read_only_inputs:
  - .planning/phases/114-lock-before-heal/114-CONTEXT.md
  - crates/kutha-runtime/src/persist_append.rs
  - crates/kutha-runtime/tests/m052_lock_before_heal.rs
---

# Phase 114 — independent verification

**Verdict:** passed. Losing attach under held `kutha.lock` after interrupted seal is WouldBlock with unchanged store bytes; attach heals after drop. Matrix `SEM-m052-lock-before-heal` killed; budget 76; workspace cargo green.
