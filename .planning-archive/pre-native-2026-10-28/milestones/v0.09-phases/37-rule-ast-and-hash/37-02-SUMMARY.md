---
phase: 37-rule-ast-and-hash
plan: 02
subsystem: rules
tags: [emit_rule, rule_version, fail-closed, RuleAst]
requires:
  - phase: 37-rule-ast-and-hash
    provides: RuleAst canonical encode and rule_body_hash
provides:
  - emit_rule stores tagged AST bytes; pin is body hash
  - non-canonical register fails UnknownRuleVersion
  - emit_rule_ast for Horn bodies
affects: [kutha-runtime, Phase 38 evaluator]
actuals:
  tokens: 2517
  tasks: 2
  commits: 2
plan_head_before: 2d6950829af75feefba8d3bddb37b6a08998939e
plan_head_after: 8a138ec08ca4aa83faa71532f383ea83cb0ea507
tech-stack:
  added: []
  patterns: [rule-domain blob admit requires from_canonical]
key-files:
  created:
    - crates/kutha-runtime/tests/m016_rule_ast.rs
  modified:
    - crates/kutha-common/src/event.rs
    - crates/kutha-runtime/src/pipeline.rs
    - CHANGELOG.md
key-decisions:
  - "rule_definition_hash(ident) = Named canonical hash; non-idents keep domain+utf8"
  - "blob_digest_matches rule-domain only if from_canonical succeeds"
requirements-completed: [RULE-01]
duration: 2min
completed: 2026-10-02
status: complete
---

# Phase 37 Plan 02: canonical hash is rule_version Summary

`emit_rule` stores tagged `RuleAst` bytes; pin equals body hash; JSON/padded names fail closed. M012/FF5/FF6 oracles untouched and green.

## What shipped

- `m012_rule_registry` 5/5, `ff5_legal_pit` 2/2, `ff6_allowlist` 2/2, `m016_rule_ast` 3/3.
- `inverse_knows` follow_ons unchanged. No evaluator.
- utf8 `"derive_pq"` bytes do not match the Named pin (`blob_digest_matches` false).

## Deviations from Plan

None - plan executed exactly as written.

## Self-Check: PASSED

- FOUND: crates/kutha-runtime/tests/m016_rule_ast.rs, emit_rule_ast in pipeline.rs
- FOUND: 3947ff5, 8a138ec
