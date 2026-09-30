---
phase: 10-adr-and-roadmap-correction
plan: 01
subsystem: architecture-review
tags: [ADR-01, ADR-03, F1-F8, M012a, D-R1, D-R2, D-O1, D-O2, D-G1]

requires:
  - phase: 09-h5-lease-and-lean-agent-context
    provides: H5 lease named; AGENTS diet; freeze holds; Active Slice None
provides:
  - Proposed review docs/architecture/semantic-gap-review.md with F1–F8 evidence and per-cell verdicts
  - Later-milestones M012a before M012; M002 log durability first; unique M### = 8
  - D-O2 harness parenthetical; STRATEGY.md no-hit
affects: [10-02 dated ADR amendments, Phase 11 SEM-08 changelog narrative]

estimate:
  tokens: 40000
  tasks: 3

actuals:
  tokens: 8310
  tasks: 3
  commits: 3

plan_head_before: d11edf987a4601a348ce547bb746580d8b56e3c0
plan_head_after: 9275ab52138a9cb7bd9ce3a0c9caf9b5701268f8
commits: 3

tech-stack:
  added: []
  patterns:
    - Architecture review register matches semantic-contract-validation.md (Proposed, not an ADR, not delivery authorization)
    - D-R2 hypotheses are verified against ADR prose; changed verdicts are named
    - CHANGELOG Process pointer only; SEM-08 narrative reserved for Phase 11

key-files:
  created:
    - docs/architecture/semantic-gap-review.md
    - .planning/phases/10-adr-and-roadmap-correction/10-01-SUMMARY.md
  modified:
    - .kutha/ROADMAP.md
    - CHANGELOG.md
    - docs/process/kutha-harness.md

key-decisions:
  - "D-R1: one Proposed review artifact; lease only as a pointer to .kutha/STATE.md"
  - "D-R2: verdicts from ADR text; four hypothesis changes named in the review"
  - "D-O1: M012a before M012; M002 is log durability first; unique M### stays 8"
  - "D-O2: kutha-harness.md M002 sentence gained a durability parenthetical; STRATEGY.md had no M002 hit"
  - "D-G2 as green-ci: CHANGELOG Process pointer only; SEM-08 not authored; .kutha/STATE.md unedited"

patterns-established:
  - "Findings table distinguishes measured / read in source / not verified by a failing test"
  - "docs-coupling Process pointer is not SEM-08 authorship"

requirements-completed: [ADR-01, ADR-03]

coverage:
  - id: D1
    description: Review artifact maps F1–F8 with file+symbol or 2026-09-30 measurement, cell verdicts, D1–D10 assessment
    requirement: ADR-01
    verification:
      - kind: other
        ref: grep F1–F8, JUSTIFICATIONS_REL, verdict table, D4 in docs/architecture/semantic-gap-review.md
        status: pass
    human_judgment: false
  - id: D2
    description: Later milestones list M012a before M012; M002 log durability first; unique M### ≤ 12; H5 unchecked
    requirement: ADR-03
    verification:
      - kind: other
        ref: python unique \\bM\\d{3}\\b count; grep M012a before **M012**; awk Rocks adapter = 0 in Later milestones
        status: pass
    human_judgment: false
  - id: D3
    description: Wave-close ci HIGH 0; D-10 Trajectory; STATE and crates untouched
    requirement: ADR-01
    verification:
      - kind: other
        ref: uv run kutha-gov ci (0 HIGH, 0 LOW, 32 checks); pytest -q scripts/tests 48 passed
        status: pass
    human_judgment: false

duration: 12min
completed: 2026-09-30
status: complete
---

# Phase 10 Plan 01: ADR and roadmap correction Summary

**Proposed semantic-gap review maps F1–F8 to honeycomb verdicts from ADR prose; Later milestones list M012a before M012 with M002 as log durability first**

## Performance

- **Duration:** 12 min
- **Started:** 2026-09-30T06:32:00Z
- **Completed:** 2026-09-30T06:44:00Z
- **Tasks:** 3
- **Files modified:** 5 (STRATEGY.md unstaged — D-O2 no-hit)

## Accomplishments

- `docs/architecture/semantic-gap-review.md` is a Proposed review (not an ADR, not delivery authorization) with eight source-backed F-rows, a cell verdict table, D1–D10 lock assessment (D4 time-scale question only), Phase 8 D-F2 supersession note, and D-R1 baselines at one N.
- `.kutha/ROADMAP.md` Later milestones: M012a (one token) before M012; M012 names the thawed subset; M002 is log durability first; benchmark after M012a indexes; thin test-only legal golden before M003. Unique `\bM\d{3}\b` count is **8**. H5 stays unchecked.
- `docs/process/kutha-harness.md` freeze sentence gained a parenthetical that M002 is log durability first. `STRATEGY.md` had no sentence that equates M002 with a Rocks adapter — left unstaged.

## Task Commits

1. **Task 1: End-to-end F1–F8 evidence path plus M012a before M012** - `b4445ab` (docs)
2. **Task 2: Read candidate ADRs and settle cell verdicts** - `58d6eb2` (docs)
3. **Task 3: Finish baselines, Later-milestones text, D-O2, wave-close ci** - `9275ab5` (docs)

## Files Created/Modified

- `docs/architecture/semantic-gap-review.md` - F1–F8 evidence, verdicts, lock assessment, D-F2 note, measured baselines
- `.kutha/ROADMAP.md` - Later-milestones proposed order (non-authoritative)
- `docs/process/kutha-harness.md` - M002 durability parenthetical (D-O2)
- `CHANGELOG.md` - Process pointer; SEM-08 narrative reserved
- `.planning/phases/10-adr-and-roadmap-correction/10-01-SUMMARY.md` - this file

## Decisions Made

Followed 10-CONTEXT.md D-R1, D-R2, D-O1, D-O2, D-G1, D-G2. CHANGELOG is a docs-coupling Process pointer only (not SEM-08). Committed on `main` under `git.branching_strategy: none` (#3552 warning). No `--no-verify`. This wave did not run codebase-memory-mcp (`list_projects` was not invoked); evidence is source reading plus the copied 2026-09-30 bench numbers.

### Cell verdicts (final)

| Cell | Verdict |
|------|---------|
| ADR-010 | open question |
| ADR-011 | no change (F2, F3); open question (F4) |
| ADR-012 | amend |
| ADR-013 | amend (F3); open question (F2, F7) |
| ADR-014 | amend |
| ADR-040 | open question |
| ADR-041 | open question |
| ADR-050 | amend (F5); open question (F4) |
| ADR-051 | open question |
| ADR-060 | amend (F1); no change (F4, F6) |
| ADR-061 | no change (F2); open question (F8) |
| ADR-081 | no change |
| ADR-002 | no change |
| ADR-001 | no change |
| ADR-000 | open question (D4 time scale only; lock wording untouched) |

### Hypothesis changes vs D-R2

| D-R2 guess | After ADR text | Why |
|------------|----------------|-----|
| Amend ADR-011 for F2 | no change | 2026-09-13 already: `fact_seq` is not portable; identities and delivery keys named |
| Open question ADR-061 for F2 | no change on refs | Same portable-reference paragraph; F8 still open-questions D061-3 diff |
| Amend ADR-060 if it states one-hop (F4) | no change for F4 | It does not state one-hop; F1 still amends fingerprint/outcome scope |
| Open question ADR-081 for the action entity | no change | Action is ADR-050/051; D081 is WASM isolation |

## Deviations from Plan

None - plan executed as written. D-R2 verdict changes are required by the plan (verify against ADR text), not Rule 1–3 auto-fixes.

Committed on `main` under `git.branching_strategy: none` (same as Phase 9). No `--no-verify`.

**Total deviations:** 0 auto-fixed
**Impact on plan:** None. STRATEGY.md correctly omitted (D-O2 no-hit).

## Issues Encountered

None.

## Trajectory (D-10 / D-G1)

1. **Commands:** `uv run kutha-gov ci`; `uv run kutha-gov explain trajectory`; `uv run kutha-gov precommit`; `uv run kutha-gov precommit --check trajectory`; `uv run pytest -q scripts/tests`.
2. **Outcome:** `ci` exit 0; **HIGH 0**, **LOW 0** (32 checks; no WARN ids to ledger under D-11). `precommit` exit 0. `precommit --check trajectory` OK (`no Active Slice`). `pytest -q scripts/tests` — 48 passed.
3. **Explain excerpt (≤8 lines):**
   ```
   check: trajectory
   purpose: Active Milestone/Slice are None or exist on ROADMAP
   authority: none — harness does not accept ADRs or claim product readiness
   source: .kutha/dictionaries/checks.yaml
   steps:
     - file_exists  .kutha/STATE.md
     - file_exists  .kutha/ROADMAP.md
     - pointer_in_other_file  .kutha/STATE.md
   ```
4. **Honesty:** Green governor is not ADR Accepted and not L_capability.

Unique `\bM\d{3}\b` in `.kutha/ROADMAP.md`: **8** (`M001` `M002` `M003` `M004` `M005` `M010` `M011` `M012`). `M012a` is one token and does not inflate the set. M012a appears before `**M012**` in Later milestones. D-O2: `STRATEGY.md` no M002 hit (unstaged); `kutha-harness.md` one freeze sentence parenthetical. This plan did not claim code-graph verification. `git diff --exit-code -- crates .kutha/STATE.md` is clean.

## Threat Flags

None beyond the plan register (T-10-01…T-10-05, T-10-SC). No new network endpoints, packages, or crate schema. No live-lease assignment tokens in the review body.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for 10-02: dated Proposed ADR clarifications/open questions from the amend and open-question rows; `docs/ADR/README.md` pointer; honeycomb.yaml only if an edge changes. Do not edit `crates/`. Do not edit `.kutha/STATE.md`. Do not flip H5. Do not author SEM-08. Do not lease M012a or M002.

## Self-Check: PASSED

- FOUND: docs/architecture/semantic-gap-review.md (register, F1–F8, verdicts, D-F2, one-N baselines)
- FOUND: .kutha/ROADMAP.md M012a before M012; unique M### = 8; H5 unchecked
- FOUND: docs/process/kutha-harness.md M002 durability parenthetical; STRATEGY.md unstaged
- FOUND: commits b4445ab, 58d6eb2, 9275ab5
- FOUND: 10-01-SUMMARY.md D-10 Trajectory; ci HIGH 0 LOW 0
- FOUND: crates/ and .kutha/STATE.md clean vs this plan

---
*Phase: 10-adr-and-roadmap-correction*
*Completed: 2026-09-30*
