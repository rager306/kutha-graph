---
independent: true
verifier_model: cursor-grok-4.6-xhigh
status: passed
score: 1/1
exit: pass
commands:
  - cmd: "cargo test --offline -p kutha-common --test m016_s02_kill --test rule_ast"; exit: 0
  - cmd: "cargo test --offline -p kutha-runtime --test m016_s02_kill --test m016_rule_ast --test m012_rule_registry --test ff5_legal_pit --test ff6_allowlist --test m012_multi_hop"; exit: 0
  - cmd: "git diff --exit-code -- .kutha/STATE.md"; exit: 0
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/phases/37-rule-ast-and-hash/37-CONTEXT.md
  - crates/kutha-common/src/rule.rs
  - crates/kutha-common/src/event.rs
  - crates/kutha-common/src/lib.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-common/tests/m016_s02_kill.rs
  - crates/kutha-runtime/tests/m016_s02_kill.rs
  - .kutha/STATE.md
---

# Phase 37 — independent verification

**ROADMAP SC:** RULE-01. Kill-tests: BE length/truncation/tag 0x03; crafted Horn; `rule_definition_hash` ≠ domain+UTF-8 ident; `StoreBlob` JSON with Named pin does not append.

| REQ | Status | Evidence |
|-----|--------|----------|
| RULE-01 | satisfied | `from_canonical` exact-consume; Named/Horn round-trip; `emit_rule` stores tagged bytes; pin = body hash; JSON/utf8/non-ident fail closed; M012 registry + FF5/FF6 green; `inverse_knows` cascade still present |

**Score:** 1/1. Residual: evaluator/cascade delete is Phase 38. No `.kutha/STATE.md` edit this verify.
