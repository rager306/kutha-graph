---
phase: 257-c4-path-policy-rewrite-foundation
independent: true
status: gaps_found
verifier_model: zai/glm-5.3
commands:
  - cmd: cargo test -p kutha-runtime --test m175_security_policy --test m175_security_integration
    exit: 0
  - cmd: rg -n 'QueryRejected' crates/
    exit: 0
  - cmd: git show --name-only --format= 924c592 | rg 'query_ir|docs/ADR|src/event.rs|.github/'
    exit: 1
    note: exit 1 is rg's no-match status; the close commit touches none of those paths
  - cmd: uv run kutha-gov precommit
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/security.rs
  - crates/kutha-runtime/src/query_ir.rs
  - crates/kutha-runtime/tests/m175_security_policy.rs
  - crates/kutha-runtime/tests/m175_security_integration.rs
  - .compound-engineering/artifacts/research/2026-10-31-claims-audit-v160-v169.md
  - .planning/STATE.md
  - .kutha/STATE.md
---

# Phase 257 verification — C4 path-policy foundation

Re-issued 2026-10-31 at HEAD `a80e5fc` by a fresh independent verifier that did not
write the close-time report. Full evidence:
`.compound-engineering/artifacts/reviews/2026-10-31-v168-v169-reverification.md`.

This file has one commit, `924c592` (the v1.68 close). That commit also adds
`security.rs` and both test files, so the closing paragraph labelled "verifier cleanup
findings were addressed after verification" is an executor note carried inside the
report, not something the verifier observed.

## Correction: the QueryRejected distinction never landed

The closing paragraph claimed the cleanup "introduced `SecurityError::QueryRejected` for
query-engine rejections that occur after policy allow (distinct from `PolicyDenied`)".
That is false at `924c592` and still false at HEAD.

- `QueryRejected` is declared (`security.rs:62`) and formatted (`security.rs:74`).
  `rg -n 'QueryRejected' crates/` returns exactly those two lines: nothing constructs it.
- `SecurityError::from_query_error` (`security.rs:147-155`) maps
  `QueryError::UnknownRelation` → `SecurityError::UnknownRelation`,
  `QueryError::UnknownTerm` → `PolicyDenied`, and `QueryError::RelsShape` →
  `PolicyDenied`. An engine rejection after a policy allow is therefore still reported
  as a policy denial, which is the opposite of "keeping policy denials distinct".
- No test drives the branch. `m175_security_integration.rs` has four tests; its only
  error assertion (`denied_gpml_returns_policy_error_not_empty_rows`) expects
  `PolicyDenied` for a policy-denied relation.
- The claim was repeated in `CHANGELOG.md` and `.planning/MILESTONES.md`; `a80e5fc`
  corrected both to "reserved and never constructed". This report had not been
  corrected until now.

## What holds

- Closed default denies `Scan`, unlabeled walks and zero-hop walks
  (`m175_security_policy.rs`, `m175_security_integration.rs`).
- A labeled walk requires every relation to be allowlisted and resolvable; a denied
  decision is returned before `execute`, never as empty rows.
- Authorized GPML equals direct execution; an authorized typed two-hop preserves the
  compiled query.
- `924c592` touches no `query_ir.rs`, `docs/ADR/`, `src/event.rs` (Op wire) or
  `.github/` path (the `rg` filter above exits 1, i.e. no match), so the
  "no QueryIr/ADR/Op/CI changes" clause is true.
- This pass at HEAD: targeted `m175_security_policy` 5 passed + `m175_security_integration`
  4 passed, 0 failed, exit 0; `uv run kutha-gov precommit` exit 0.

## Inherited, not re-run in this pass

"workspace 753 passed / 0 failed across 174 suites" is not re-verified here. The
surviving gitignored log `tmp/m175-final-test.log` tallies 753 passed / 0 failed over
174 `test result` lines, so the figure matches a real run, but scratch is not durable
evidence and is not listed under `read_only_inputs`.

## Status: gaps_found

The phase behavior is real and its tests pass. The report nevertheless certified an API
distinction that does not exist, and the code still carries an unused public variant
while reporting engine rejections as policy denials. Closing action, one of:

1. map `UnknownTerm`/`RelsShape` to `QueryRejected` in `from_query_error` and add one
   integration test where an allowlisted query fails with `RelsShape` and returns
   `QueryRejected`; or
2. delete `QueryRejected` and keep the "reserved and never constructed" wording the
   CHANGELOG and MILESTONES now carry.

The documents are corrected at HEAD; only the code or the variant decision remains.
