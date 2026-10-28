---
phase: 34-known-on-and-anchor-admission
plan: 02
subsystem: runtime
tags: [TtAnchor, admit, ANK-ADM-01]
requires: [{ phase: 34-known-on-and-anchor-admission, provides: KNOWN-01 TT-only as_known_on }]
provides: ["ANK-ADM-01 fail-closed TtAnchor admit", "ANK-ADM-02 monotone persist/open"]
affects: [35]
actuals: { tokens: 1408, tasks: 3, commits: 3 }
plan_head_before: 400654232e2a5cb9e65f4a5a2b426e12038fbd0c
plan_head_after: f829f4b5c8605c19e122d4a77b44bf31e8919493
tech-stack: { added: [], patterns: ["admit wall <= max → TtAnchorRejected"] }
key-files: { created: [crates/kutha-runtime/tests/m015_anchor_admit.rs], modified: [crates/kutha-runtime/src/admission.rs, crates/kutha-runtime/src/quantum.rs, CHANGELOG.md] }
key-decisions: ["D-03: reject wall <= max existing; apply/rebuild last-wins unchanged"]
patterns-established: ["TtAnchorRejected at admit; fold unchanged on reject"]
requirements-completed: [ANK-ADM-01, ANK-ADM-02]
coverage:
  - id: D1
    description: Duplicate and backwards TtAnchor walls fail closed; log len unchanged
    requirement: ANK-ADM-01
    verification: [{ kind: integration, ref: "crates/kutha-runtime/tests/m015_anchor_admit.rs#duplicate_tt_anchor_wall_is_rejected", status: pass }]
    human_judgment: false
  - id: D2
    description: Monotone unique walls persist/open; greatest-anchor-≤D holds
    requirement: ANK-ADM-02
    verification: [{ kind: integration, ref: "crates/kutha-runtime/tests/m015_anchor_admit.rs#monotone_unique_anchors_persist_open", status: pass }]
    human_judgment: false
duration: 2min
completed: 2026-10-02
status: complete
---

# Phase 34 Plan 02: Fail-closed TtAnchor admit Summary

**Duplicate or backwards `TtAnchor` walls return `TtAnchorRejected`; monotone unique walls still round-trip.**

- **Duration:** 2 min · **Started:** 2026-10-02T09:35:12Z · **Completed:** 2026-10-02T09:36:53Z · **Tasks:** 3 · **Files:** 4

- RED: `duplicate_tt_anchor_wall_is_rejected` and `backwards_tt_anchor_wall_is_rejected` failed on HEAD last-wins (`tmp/p34-02-red.txt`)
- GREEN: `admit` rejects `wall <= max`; `RuntimeError::TtAnchorRejected`; apply/rebuild insert unchanged
- Persist/open `tmp/p34-02-store`; FF5/m014/m015 known-on green; `.kutha/STATE.md` untouched; no `34-VERIFICATION`

## Task Commits
1. **RED oracles** - `ba3129c` (test)
2. **Admit reject** - `513d9d6` (feat)
3. **Persist + CHANGELOG** - `f829f4b` (test)

## Deviations from Plan
None - plan executed as written. Cargo libtest RED (not TAP). No renames.

## Next Phase Readiness
Phase 34 executor work complete. Independent verifier writes `34-VERIFICATION.md` (D-05). Do not start Phase 35.

## Self-Check: PASSED
FOUND: admission.rs, quantum.rs, m015_anchor_admit.rs, 34-02-SUMMARY.md; commits ba3129c, 513d9d6, f829f4b
