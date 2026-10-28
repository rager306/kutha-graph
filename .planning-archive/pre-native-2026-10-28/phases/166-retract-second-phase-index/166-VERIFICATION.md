---
phase: 166-retract-second-phase-index
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test -p kutha-runtime --test m095_retract_second_phase --test m054_horn_soundness --test m054_scaling_bench --offline
    exit: 0
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: taskset -c 0 env KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --bench baseline
    exit: 0
  - cmd: uv run kutha-gov precommit --check test-matrix --check docs-coupling --check changelog-planes --check cite-lease --check state-readme
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - .planning/phases/166-retract-second-phase-index/166-CONTEXT.md
  - crates/kutha-runtime/src/eval.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/tests/m095_retract_second_phase.rs
  - docs/architecture/test-matrix.yaml
---

# Phase 166 — independent verification

**Verdict:** passed. Second-phase premise-relation index green; non-trigger
retract still sound; workspace tests green; quiet bench `ingest_plain.n`
≈104.7M ns / `as_of.n` ≈310.3k ns (within prior product-close host band);
full `ci` tip_high=0 after close.
