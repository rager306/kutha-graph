---
phase: 210-retract-n-scaling
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test -p kutha-runtime --test m136_retract_n_scaling --offline
    exit: 0
  - cmd: cargo test -p kutha-runtime --test m095_retract_second_phase --offline
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/eval.rs
  - crates/kutha-runtime/tests/m136_retract_n_scaling.rs
  - docs/architecture/test-matrix.yaml
---

# Phase 210 — independent verification

**Verdict:** passed. RED: drop single-premise second-phase skip → quadratic examine (~15× for 4× N). GREEN: near-linear; M095 still green; `kutha-gov ci` run_high=0 (97 checks).
