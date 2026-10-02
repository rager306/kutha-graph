---
phase: 37-rule-ast-and-hash
plan: 01
subsystem: rules
tags: [RuleAst, canonical-bytes, rule_body_hash]
requires:
  - phase: 18-rule-registry
    provides: rule_definition_hash domain tag kutha-rule-def
provides:
  - RuleAst Named+Horn tagged LE encoding
  - rule_body_hash of canonical bytes
affects: [kutha-common, 37-02]
actuals:
  tokens: 2946
  tasks: 1
  commits: 1
plan_head_before: d2edc8814d658a9352b230e57ce2477ca7214b66
plan_head_after: 598ee7401740b7fe0204bca00ee037c69ce493b7
tech-stack:
  added: []
  patterns: [tagged LE canonical AST; SHA-256 domain || bytes]
key-files:
  created:
    - crates/kutha-common/src/rule.rs
    - crates/kutha-common/tests/rule_ast.rs
  modified:
    - crates/kutha-common/src/lib.rs
    - CHANGELOG.md
key-decisions:
  - "D-01: Named tag 0x01 / Horn 0x02; u16 LE lengths; exact-consume from_canonical"
  - "rule_definition_hash utf8 path left for 37-02"
requirements-completed: []
duration: 1min
completed: 2026-10-02
status: complete
---

# Phase 37 Plan 01: RuleAst canonical encoding Summary

Tagged Named/Horn bytes with SHA-256(`kutha-rule-def` || canonical); JSON/trailing/raw utf8 fail `from_canonical`.

## What shipped

- `cargo test -p kutha-common --test rule_ast`: 4 passed (round-trip, rejects, tagged hash, bounds).
- Lib `named_hash_is_not_utf8_domain_hash`: Named pin ≠ utf8 domain hash of `"derive_pq"`.
- `rule_definition_hash` still hashes `definition.as_bytes()` (37-02 wires ident → Named).

## Deviations from Plan

None - plan executed exactly as written.

## Self-Check: PASSED

- FOUND: crates/kutha-common/src/rule.rs, tests/rule_ast.rs
- FOUND: 598ee74
