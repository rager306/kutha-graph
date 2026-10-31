---
phase: 258-c4-authorized-execution-integration
independent: true
status: gaps_found
verifier_model: zai/glm-5.3
commands:
  - cmd: cargo test -p kutha-runtime --test m175_security_policy --test m175_security_integration
    exit: 0
  - cmd: rg -n 'QueryRejected' crates/
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/security.rs
  - crates/kutha-runtime/src/query_ir.rs
  - crates/kutha-runtime/tests/m175_security_integration.rs
  - crates/kutha-runtime/tests/m175_security_policy.rs
  - .compound-engineering/artifacts/research/2026-10-31-claims-audit-v160-v169.md
  - .planning/STATE.md
  - .kutha/STATE.md
---

# Phase 258 verification — C4 authorized execution integration

Re-issued 2026-10-31 at HEAD `a80e5fc` by a fresh independent verifier that did not
write the close-time report. Full evidence:
`.compound-engineering/artifacts/reviews/2026-10-31-v168-v169-reverification.md`.

This file has one commit, `924c592` (the v1.68 close), which also adds `security.rs` and
both test files.

## Correction: the QueryRejected distinction never landed

The closing paragraph claimed the cleanup "introduced `SecurityError::QueryRejected` for
post-policy query-engine errors, keeping policy denials distinct". That is false:

- `QueryRejected` is declared (`security.rs:62`) and formatted (`:74`) and never
  constructed; `rg -n 'QueryRejected' crates/` returns only those two lines.
- `from_query_error` (`security.rs:147-155`) maps `UnknownRelation` →
  `SecurityError::UnknownRelation`, and both `UnknownTerm` and `RelsShape` →
  `PolicyDenied`. Post-policy engine failures are therefore still reported as policy
  denials.
- No test covers an engine rejection after a policy allow; the only error assertion in
  this phase expects `PolicyDenied` for a denied relation.

`CHANGELOG.md` and `.planning/MILESTONES.md` were corrected at HEAD (`a80e5fc`); this
report and `257-VERIFICATION.md` were not, until this re-verification.

## What holds

- Authorized GPML result equals direct execution, and an authorized typed two-hop
  preserves the compiled query (`m175_security_integration.rs`).
- A denied GPML query returns `PolicyDenied` before prepare/execute, never empty rows;
  a denied relation is decided at compile time (`m175_security_policy.rs`
  `denied_relation_compiles_denied_before_prepare`).
- Closed default denies scan and unlabeled paths.
- No grants, tenants, vacuum, capabilities, WASM or full ABAC surface, no ADR Accepted
  claim, and no QueryIr/ADR/Op/CI change in the close commit.
- This pass at HEAD: 5 + 4 targeted tests passed, 0 failed, exit 0; precommit exit 0.

## Inherited, not re-run in this pass

"final workspace 753 passed / 0 failed" is inherited from the gitignored log
`tmp/m175-final-test.log` (tally 753 / 0 over 174 `test result` lines). It is context,
not durable evidence, and was not re-measured here.

## Status: gaps_found

The integration behavior is verified, but the report asserted an error-taxonomy
distinction the code does not implement and no test would detect. Closing action matches
257: either map `UnknownTerm`/`RelsShape` to `QueryRejected` and add one allowed-query
`RelsShape` test, or delete the variant and keep the corrected document wording.
