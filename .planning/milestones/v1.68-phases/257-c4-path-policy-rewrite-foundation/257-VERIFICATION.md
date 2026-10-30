---
phase: 257-c4-path-policy-rewrite-foundation
independent: true
status: passed
verifier_model: zai/glm-5.3-flash
commands:
  - cmd: cargo test -p kutha-runtime --test m175_security_policy --test m175_security_integration
    exit: 0
  - cmd: cargo test --workspace
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: git diff -- crates/kutha-runtime/src/query_ir.rs docs/ADR/ .github/
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/security.rs
  - crates/kutha-runtime/tests/m175_security_policy.rs
  - crates/kutha-runtime/tests/m175_security_integration.rs
  - crates/kutha-runtime/src/query_ir.rs
  - .planning/STATE.md
  - .kutha/STATE.md
---

# Phase 257 verification — C4 path-policy foundation

Two independent verifier contexts PASSed the bounded foundation: closed default denies `Scan`, unlabeled walks, and zero-hop queries; labeled hops require every relation to resolve and be policy-allowed; denied decisions return before `prepare` and never become empty rows. Targeted tests 5/5 and 4/4 passed; workspace 753 passed / 0 failed across 174 suites; precommit exit 0; no QueryIr/ADR/Op/CI changes.

Verifier cleanup findings were addressed after verification: removed the dead relation-name set and `TermId` stub, and introduced `SecurityError::QueryRejected` for query-engine rejections that occur after policy allow (distinct from `PolicyDenied`). Targeted tests pass after cleanup; the final workspace run is recorded in `tmp/m175-final-test.log`.
