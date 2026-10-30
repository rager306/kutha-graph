---
phase: 258-c4-authorized-execution-integration
plan: 01
subsystem: security-query
tags: [c4, security, integration, gpml]
requires: [257-01]
provides:
  - "execute_authorized integration for typed and GPML queries"
affects: []
actuals: {tokens: 0, tasks: 5, commits: 0}
tech-stack: {added: [], patterns: [authorize-before-execute]}
key-files:
  created: [crates/kutha-runtime/tests/m175_security_integration.rs]
  modified: [crates/kutha-runtime/src/security.rs]
key-decisions:
  - "Denied GPML/typed queries return SecurityError::PolicyDenied, not empty rows."
  - "Authorized GPML/typed queries preserve direct execution results."
requirements-completed: [SEC-02]
coverage:
  - id: D1
    description: "Authorized execution integration."
    requirement: SEC-02
    verification:
      - kind: manual
        ref: "crates/kutha-runtime/tests/m175_security_integration.rs"
        status: pass
    human_judgment: false
---

# Phase 258 summary — C4 authorized execution integration

Added `execute_authorized` integration and tests proving authorized GPML/typed queries preserve direct execution results while denied queries return `SecurityError::PolicyDenied`. Closed default denies scan/unlabeled paths. Four integration tests pass. No grants, tenants, vacuum, capabilities, WASM, or full ABAC behavior.
