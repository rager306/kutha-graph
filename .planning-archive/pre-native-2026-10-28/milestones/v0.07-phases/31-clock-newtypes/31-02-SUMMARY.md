---
phase: 31-clock-newtypes
plan: 02
subsystem: runtime
tags: [clocks, CSR, justification, TIME-01]
requires: [{ phase: 31-clock-newtypes, provides: CLK-01 query newtypes }]
provides: ["CLK-01 CSR/justification cuts", "CLK-03 TIME-01 wrap + workspace green"]
affects: [32-as-known-on]
actuals: { tokens: 1955, tasks: 3, commits: 3 }
plan_head_before: ae0cf417b9e551cae5345af5be33e1e127aecdec
plan_head_after: 820d7accca574fd8ad22285480390577a0718b8a
tech-stack: { added: [], patterns: ["impl Into CSR/justification cuts", "TIME-01 ValidTime::from wrap"] }
key-files: { created: [], modified: [crates/kutha-runtime/src/quantum.rs, crates/kutha-runtime/src/csr.rs, crates/kutha-runtime/src/materializer.rs, crates/kutha-runtime/src/justification.rs, crates/kutha-runtime/src/provenance.rs, crates/kutha-runtime/src/admission.rs, crates/kutha-runtime/tests/m012a_time_scale.rs, CHANGELOG.md] }
key-decisions: ["CSR/justification cuts take Into clocks; JustificationCite tt/vt stay u64", "TIME-01 lets wrapped only; no other oracle constructor wraps"]
patterns-established: ["Public cuts: impl Into then as_u64; log fields stay integers"]
requirements-completed: [CLK-01, CLK-03]
coverage:
  - id: D1
    description: CSR and justification/derivation cuts take distinct clock newtypes
    requirement: CLK-01
    verification: [{ kind: integration, ref: "cargo test -p kutha-runtime --test ff5_legal_pit --test m011_typed_csr --test m012_admission_facts", status: pass }]
    human_judgment: false
  - id: D2
    description: TIME-01 ValidTime::from wraps; named oracles green; assertion bodies unchanged
    requirement: CLK-03
    verification: [{ kind: integration, ref: "cargo test --workspace --offline", status: pass }]
    human_judgment: false
duration: 3min
completed: 2026-10-02
status: complete
---

# Phase 31 Plan 02: Remaining cuts and TIME-01 Summary

**CSR/justification cuts take typed clocks; TIME-01 YearCe lets wrap `ValidTime::from`; workspace oracles green.**

- **Duration:** 3 min · **Started:** 2026-10-02T07:18:37Z · **Completed:** 2026-10-02T07:22:06Z · **Tasks:** 3 · **Files:** 8

- `csr_lease_at` / `from_fold` / `Materializer::build` take `impl Into` clocks
- Justification/action/admission/derivation/conflict cuts typed; cite `tt`/`vt` stay `u64`
- `cargo test --workspace --offline` ok; FF5/FF6/STATE.md byte-unchanged

## Task Commits
1. **CSR cuts** - `828a6c9` (feat)
2. **Justification/derivation cuts** - `9c54f96` (feat)
3. **TIME-01 bindings** - `820d7ac` (test)

## Old-to-new (D-03)

| Oracle | Old | New |
|--------|-----|-----|
| TIME-01 | `let y2015: ValidTime = 2015` (and y2017, y2021) | `ValidTime::from(...)` |
| TIME-01 asserts | `assert_eq!(y2015, 2015)` | unchanged |
| FF5 / FF6 | — | no edits |

## Deviations from Plan
None - plan executed exactly as written.

## Next Phase Readiness
Phase 31 code complete. Independent verifier writes `31-VERIFICATION.md` (D-05). Do not start Phase 32. Self-check PASSED: 31-01-SUMMARY.md; commits 828a6c9, 9c54f96, 820d7ac
