---
phase: 167-bench-rss-before-sample
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test -p kutha-runtime --test m054_scaling_bench --offline -- bench_rss
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
  - .planning/phases/167-bench-rss-before-sample/167-CONTEXT.md
  - crates/kutha-runtime/benches/baseline.rs
  - crates/kutha-runtime/tests/m054_scaling_bench.rs
  - docs/architecture/test-matrix.yaml
---

# Phase 167 — independent verification

**Verdict:** passed. `rss_before` sits immediately before the dedicated sample;
workspace tests green; quiet bench `ingest_plain.n` ≈131.3M ns / `as_of.n`
≈342.3k ns; full `ci` tip_high=0 after close.
