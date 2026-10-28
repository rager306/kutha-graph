---
phase: 115-open-replay
independent: true
verifier_model: parent-session-fresh-pass
commands:
  - cmd: cargo test -p kutha-runtime --test m052_open_replay --offline
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: export KUTHA_GOV_BUDGET=76; uv run kutha-gov ci
    exit: 0
    note: test-matrix/changelog/docs-coupling 0 HIGH. ROADMAP S04 checkbox left open so trajectory lease-drift stays clean while STATE still leases S04.
  - cmd: KUTHA_BENCH_N=10000 taskset -c 0 cargo bench -p kutha-runtime --bench baseline -- --quick
    exit: 0
    note: three quiet runs; median ingest_plain.n ≈103.9M as_of.n ≈299.2k
read_only_inputs:
  - .planning/phases/115-open-replay/115-CONTEXT.md
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/tests/m052_open_replay.rs
---

# Phase 115 — independent verification

**Verdict:** passed. Snapshot-free Define/terms open reject an empty-pin unreproducible log that snapshot-backed open rejects. Matrix `SEM-m052-open-replay` killed; budget 76; workspace cargo green; ci 0 HIGH / 1 LOW / 76 checks.
