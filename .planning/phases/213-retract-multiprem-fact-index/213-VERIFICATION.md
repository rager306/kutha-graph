---
phase: 213-retract-multiprem-fact-index
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: cargo test -p kutha-runtime --test m139_retract_multiprem_fact_index --test m095_retract_second_phase --test m136_retract_n_scaling -q
    exit: 0
  - cmd: uv run kutha-gov ci
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/eval.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/tests/m139_retract_multiprem_fact_index.rs
  - docs/architecture/test-matrix.yaml
---

# Phase 213 — independent verification

**Verdict:** passed. RED: relation-bucket phase 2 → ratio 16 (n=8→64, n=32→1024). GREEN: premise-fact index → ratio 4 (8→8, 32→32); M095/M136 still green; `kutha-gov ci` run_high=0 (97 checks).
