---
phase: 176-policy-pin-newest
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
  - crates/kutha-runtime/src/fold_query.rs
  - crates/kutha-runtime/tests/m105_audit_residuals.rs
---

# Phase 176 — independent verification

**Verdict:** passed. Newest live policy pin examines one hot row
(`newest_policy_pin_examines_one_live_row`). Workspace tests exit 0.
Bench `ingest_plain.n` 96637743 ns, `as_of.n` 217945 ns. `kutha-gov ci`
tip_high=0 run_high=0.
