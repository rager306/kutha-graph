---
phase: 179-horn-replay-vt
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test --workspace --offline --no-fail-fast
    exit: 0
  - cmd: taskset -c 0 env KUTHA_BENCH_N=10000 cargo bench -p kutha-runtime --offline --bench baseline
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/eval.rs
  - crates/kutha-runtime/src/provenance.rs
  - crates/kutha-runtime/src/store.rs
---

# Phase 179 — independent verification

**Verdict:** passed. Horn fire keeps every premise window that covers the
Behavior VT. `open` installs the replay fold and restores the snapshot
`FoldIndexMode`. Correct walks every live residual. Workspace tests exit 0,
including snapshot index-mode open regressions. Bench `ingest_plain.n`
96637743 ns, `as_of.n` 217945 ns. `kutha-gov ci` tip_high=0 run_high=0.
