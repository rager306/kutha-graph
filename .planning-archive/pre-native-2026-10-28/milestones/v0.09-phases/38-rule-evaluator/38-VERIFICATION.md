---
independent: true
verifier_model: cursor-grok-4.6-xhigh
status: passed
score: 1/1
exit: pass
commands:
  - cmd: "cargo test --offline -p kutha-runtime --test m016_s03_kill --test m016_rule_eval --test m012_multi_hop --test ff5_legal_pit --test ff6_allowlist --test m012_rule_registry"; exit: 0
  - cmd: "rg -n 'if *relation == self.knows' crates/kutha-runtime/src/pipeline.rs"; exit: 1
  - cmd: "git diff --exit-code -- .kutha/STATE.md"; exit: 0
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/phases/38-rule-evaluator/38-CONTEXT.md
  - crates/kutha-runtime/src/eval.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/tests/m016_s03_kill.rs
  - crates/kutha-runtime/tests/m016_rule_eval.rs
  - crates/kutha-runtime/tests/m012_multi_hop.rs
  - .kutha/STATE.md
---

# Phase 38 — independent verification

**ROADMAP SC:** RULE-02. Kill-tests: inverse pin is genesis Horn hash (not empty); relatedTo does not mint inverse; retract of knows drops live knownBy; two-premise Horn needs both premises.

| REQ | Status | Evidence |
|-----|--------|----------|
| RULE-02 | satisfied | `eval.rs` Horn unify; genesis inverse; `pipeline.rs` has no knows special-case; n-ary test; two-hop eligibility oracles green; FF5/FF6/registry green |

**Score:** 1/1. Residual: policy eval + execution replay is Phase 39. No `.kutha/STATE.md` edit this verify.
