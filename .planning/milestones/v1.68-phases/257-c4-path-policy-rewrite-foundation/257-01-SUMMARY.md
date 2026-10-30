---
phase: 257-c4-path-policy-rewrite-foundation
plan: 01
subsystem: security-query
tags: [c4, security, path-policy, query-ir]
requires: []
provides:
  - "closed-default PathPolicy compiler"
  - "deny-before-prepare AuthorizedQuery decision"
affects: [258-c4-authorized-execution-integration]
actuals: {tokens: 0, tasks: 5, commits: 0}
tech-stack: {added: [], patterns: [closed-default-policy, deny-before-prepare]}
key-files:
  created: [crates/kutha-runtime/src/security.rs, crates/kutha-runtime/tests/m175_security_policy.rs]
  modified: [crates/kutha-runtime/src/lib.rs]
key-decisions:
  - "Scan and unlabeled walks deny by default; labeled hops require every relation allowed."
  - "Denied queries are compile decisions and never become empty rows."
requirements-completed: [SEC-01]
coverage:
  - id: D1
    description: "Closed-default policy decision before prepare."
    requirement: SEC-01
    verification:
      - kind: manual
        ref: "crates/kutha-runtime/tests/m175_security_policy.rs"
        status: pass
    human_judgment: false
---

# Phase 257 summary — C4 path-policy foundation

Added `security.rs` with `PathPolicy`, `AuthorizedQuery`, `SecurityError`, and `compile_path_policy`. `Scan`, unlabeled walks, and zero-hop queries deny by default. Labeled hops require every relation to resolve and be policy-allowed. Denied decisions occur before `prepare`; no grants, tenant slices, vacuum authority, capabilities, WASM, full ABAC language, or ADR status change. Five focused policy tests pass.
