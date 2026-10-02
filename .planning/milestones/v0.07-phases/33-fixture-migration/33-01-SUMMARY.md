---
phase: 33-fixture-migration
plan: 01
subsystem: testing
tags: [clocks, FIX-01, FIX-02, FF5]
requires: [{ phase: 32-tt-anchor-and-as-known-on, provides: ANK-01 Op::TtAnchor log facts }]
provides: ["FIX-01 explicit clock constructors on fixtures", "FIX-02 old-to-new wrap table"]
affects: [m014-close]
actuals: { tokens: 11874, tasks: 3, commits: 3 }
plan_head_before: 8d6513a489539928fd59778fa1e300d76807301a
plan_head_after: 58b3bc99b60e6e1d0a5aeeb1b14a3120c0603c0b
tech-stack: { added: [], patterns: ["ValidTime::from / TransactionTime::MAX at public cuts"] }
key-files: { created: [], modified: [crates/kutha-runtime/tests/ff5_legal_pit.rs, crates/kutha-runtime/tests/ff6_allowlist.rs, crates/kutha-runtime/tests/m011_e2e_fixture.rs, CHANGELOG.md] }
key-decisions: ["Wrap public clock API only; Op valid_from stays u64", "WallTime unused in FIX-01 files"]
patterns-established: ["tt u64::MAX → TransactionTime::MAX; vt literals → ValidTime::from"]
requirements-completed: [FIX-01, FIX-02]
coverage:
  - id: D1
    description: FF5/M011/M012/M013 public cuts wrap typed constructors
    requirement: FIX-01
    verification: [{ kind: integration, ref: "cargo test -p kutha-runtime --offline --test ff5_legal_pit --test ff6_allowlist", status: pass }]
    human_judgment: false
  - id: D2
    description: Oracle predicates unchanged; valid_from absent from wrap diff
    requirement: FIX-02
    verification: [{ kind: integration, ref: "cargo test --workspace --offline", status: pass }]
    human_judgment: false
duration: 6min
completed: 2026-10-02
status: complete
---

# Phase 33 Plan 01: Fixture clock wraps Summary

**FF5/M011/M012/M013 public clock cuts wrap `ValidTime`/`TransactionTime`; assertion predicates unchanged.**

- **Duration:** 6 min · **Started:** 2026-10-02T08:32:04Z · **Completed:** 2026-10-02T08:37:23Z · **Tasks:** 3 · **Files:** 21
- Wrap map: vt → `ValidTime::from`; tt `u64::MAX` → `TransactionTime::MAX`; other tt → `TransactionTime::from`
- `cargo test --workspace --offline` ok; `.kutha/STATE.md` and `m014_as_known_on.rs` byte-unchanged

## Task Commits
1. **FF5 wraps** - `0e07ca1` (test)
2. **M011/M012/M013 + FF6** - `7546274` (test)
3. **SUMMARY** - `58b3bc9` (docs)

## Old-to-new (FIX-02)

| Oracle | Old | New |
|--------|-----|-----|
| FF5 | `as_of(T_OLD)`; `csr_lease_at(u64::MAX, T_OLD)` | `ValidTime::from`; `TransactionTime::MAX` |
| FF6 | `live_count(u64::MAX, 2010)` | `TransactionTime::MAX, ValidTime::from(2010)` |
| M011 | `live_at` / CSR / `record_*` / derivation u64 | same wrap map |
| M012/a | `as_of` / `live_count` / admission / TIME-01 `as_of(2017)` | same wrap map |
| M013 | `as_of` / `dictionary_as_of` / write-path cuts | same wrap map |
| WallTime / Op / m014 / empty-clock files | — | no wraps / no edits |

## Deviations from Plan
None. Verifier writes `33-VERIFICATION.md` (D-05). Do not start M015.
## Self-Check: PASSED
FOUND: ff5_legal_pit.rs, 33-01-PLAN.md, 33-01-SUMMARY.md; commits 0e07ca1, 7546274