---
phase: 10-adr-and-roadmap-correction
plan: 02
subsystem: architecture-review
tags: [ADR-02, ADR-04, D-M1, D-M2, D-C1, D-G1]

requires:
  - phase: 10-adr-and-roadmap-correction
    provides: Proposed review docs/architecture/semantic-gap-review.md with per-cell verdicts
provides:
  - Dated 2026-09-30 Proposed clarifications and open questions on verdict-selected honeycomb ADRs
  - ADR README one-line pointer to semantic-gap-review.md
  - honeycomb.yaml still Proposed; restage comments only
affects: [Phase 11 SEM-08 changelog narrative]

estimate:
  tokens: 36000
  tasks: 3

actuals:
  tokens: 4470
  tasks: 3
  commits: 3

plan_head_before: 1f99c41a88d68b5d11ad92d6b80567fcf4cf2d5d
plan_head_after: 71d174f067d93c350a864bc92f89a1a51683311c
commits: 3

tech-stack:
  added: []
  patterns:
    - D-M1 dated Proposed subsection beside 2026-09-13 clarifications; Status stays Proposed
    - Worklist is the 10-01 verdict table, not the D-R2 hypothesis
    - CHANGELOG Process pointer only; SEM-08 narrative reserved for Phase 11

key-files:
  created:
    - .planning/phases/10-adr-and-roadmap-correction/10-02-SUMMARY.md
  modified:
    - docs/ADR/README.md
    - docs/ADR/ADR-000-kutha-hybrid-architecture-research.md
    - docs/ADR/ADR-010-event-log-runtime-quantum.md
    - docs/ADR/ADR-011-lean-event-schema-lineage.md
    - docs/ADR/ADR-012-snapshots-tiers-vacuum.md
    - docs/ADR/ADR-013-bitemporal-facts-invalidation.md
    - docs/ADR/ADR-014-cascade-budgets-quantum-receipts.md
    - docs/ADR/ADR-040-materialization-plugin-protocol.md
    - docs/ADR/ADR-041-csr-graphblas-hot-path.md
    - docs/ADR/ADR-050-meta-prompt-dictionaries.md
    - docs/ADR/ADR-051-capability-security.md
    - docs/ADR/ADR-060-strict-replay.md
    - docs/ADR/ADR-061-fork-and-diff.md
    - CHANGELOG.md
    - .kutha/dictionaries/honeycomb.yaml

key-decisions:
  - "Worklist SoT is the 10-01 verdict table; first amend row is ADR-012 (not D-R2's ADR-014)"
  - "Honeycomb restage is header comments only; map/delivery/capability unchanged"
  - "ADR-000: English D4 time-scale pointer only; D1–D10 Decision text byte-stable"

patterns-established:
  - "Additions-only ADR diffs (git -U0 removed content lines = 0 per ADR file)"
  - "Target-contract prose in the cell's own vocabulary (D-M2); no crate/function/milestone commitments"

requirements-completed: [ADR-02, ADR-04]

coverage:
  - id: D1
    description: Every amend and open-question verdict has a dated ≤15-line 2026-09-30 subsection; Status remains Proposed; D1–D10 wording untouched
    requirement: ADR-02
    verification:
      - kind: other
        ref: python glob ADR-*.md for Clarification (2026-09-30; Proposed) / Open question (2026-09-30); ## Status Proposed; git diff -U0 docs/ADR/ removed=0; ADR-000 Decision section identical to plan_head_before
        status: pass
    human_judgment: false
  - id: D2
    description: README one-line pointer to semantic-gap-review.md; no new index row; honeycomb map unique value Proposed
    requirement: ADR-04
    verification:
      - kind: other
        ref: grep semantic-gap-review.md docs/ADR/README.md; grep -F '| [100]' count 0; uv run kutha-gov map; uv run kutha-gov precommit --check honeycomb-ledger
        status: pass
    human_judgment: false
  - id: D3
    description: Wave-close ci HIGH 0; D-10 Trajectory; crates and .kutha/STATE.md unedited
    requirement: ADR-02
    verification:
      - kind: other
        ref: uv run kutha-gov ci (0 HIGH, 0 LOW, 32 checks); pytest -q scripts/tests 48 passed; git diff --exit-code -- crates .cursor/rules .kutha/STATE.md
        status: pass
    human_judgment: false

duration: 9min
completed: 2026-09-30
status: complete
---

# Phase 10 Plan 02: ADR and roadmap correction Summary

**Dated Proposed clarifications and open questions on verdict-selected honeycomb cells; ADR README points at the semantic-gap review; map stays Proposed**

## Performance

- **Duration:** 9 min
- **Started:** 2026-09-30T06:48:53Z
- **Completed:** 2026-09-30T06:57:01Z
- **Tasks:** 3
- **Files modified:** 15 (plus this SUMMARY)

## Accomplishments

- Worklist followed the 10-01 cell verdict table in `docs/architecture/semantic-gap-review.md`, not the D-R2 hypothesis. First amend row is ADR-012.
- Every `amend` cell gained `### Clarification (2026-09-30; Proposed)`; every remaining `open question` gained `### Open question (2026-09-30)`; each new subsection is ≤15 lines and cites the review. `## Status` stays Proposed; `## Date` unchanged.
- ADR-000 D1–D10 Decision text is identical to the plan base; one English D4 pointer sits under Open Research Questions. No-change cells (ADR-001, ADR-002, ADR-081) were not edited.
- `docs/ADR/README.md` has a one-line pointer to the review (no new index row). `honeycomb.yaml` map/delivery/capability fields are unchanged (restage comments only). CHANGELOG is a Process pointer; SEM-08 is not authored.

## Task Commits

1. **Task 1: End-to-end one-cell amendment plus ADR README pointer** - `3dc32ec` (docs)
2. **Task 2: Apply remaining amend verdicts as dated Proposed clarifications** - `318ac29` (docs)
3. **Task 3: Apply remaining open questions, honeycomb consistency, and wave-close ci** - `71d174f` (docs)

## Files Created/Modified

- `docs/ADR/ADR-012-snapshots-tiers-vacuum.md` — first amend (snapshot identity verified on open)
- `docs/ADR/ADR-013-bitemporal-facts-invalidation.md` — F3 amend + F2/F7 open question
- `docs/ADR/ADR-014-cascade-budgets-quantum-receipts.md` — F1 amend (outcomes as log records)
- `docs/ADR/ADR-050-meta-prompt-dictionaries.md` — F5 amend + F4 open question
- `docs/ADR/ADR-060-strict-replay.md` — F1 amend (obligation-2 outcomes as log records)
- `docs/ADR/ADR-000-kutha-hybrid-architecture-research.md` — D4 English pointer only
- `docs/ADR/ADR-010-event-log-runtime-quantum.md` — record-kind open question
- `docs/ADR/ADR-011-lean-event-schema-lineage.md` — F4 open question (F2/F3 2026-09-13 left as no-change)
- `docs/ADR/ADR-040-materialization-plugin-protocol.md` — F8 open question
- `docs/ADR/ADR-041-csr-graphblas-hot-path.md` — F8 open question
- `docs/ADR/ADR-051-capability-security.md` — F5 open question
- `docs/ADR/ADR-061-fork-and-diff.md` — F8 open question (F2 2026-09-13 left as no-change)
- `docs/ADR/README.md` — one-line review pointer
- `.kutha/dictionaries/honeycomb.yaml` — restage comments; fields unchanged
- `CHANGELOG.md` — Process pointer; narrative remains Phase 11 SEM-08

## Verdict-to-amendment ledger

| Cell | 10-01 verdict | Heading added | Lines | Skip reason |
|------|---------------|---------------|-------|-------------|
| ADR-010 | open question (F1, F6) | Open question (2026-09-30) | 3 | — |
| ADR-011 | no change (F2, F3); open question (F4) | Open question (2026-09-30) | 3 | F2/F3 already in 2026-09-13 clarification |
| ADR-012 | amend (F6) | Clarification (2026-09-30; Proposed) | 3 | — |
| ADR-013 | amend (F3); open question (F2, F7) | Clarification (2026-09-30; Proposed); Open question (2026-09-30) | 3+3 | — |
| ADR-014 | amend (F1) | Clarification (2026-09-30; Proposed) | 3 | — |
| ADR-040 | open question (F8) | Open question (2026-09-30) | 3 | — |
| ADR-041 | open question (F8) | Open question (2026-09-30) | 3 | — |
| ADR-050 | amend (F5); open question (F4) | Clarification (2026-09-30; Proposed); Open question (2026-09-30) | 3+3 | — |
| ADR-051 | open question (F5) | Open question (2026-09-30) | 3 | — |
| ADR-060 | amend (F1); no change (F4, F6) | Clarification (2026-09-30; Proposed) | 3 | F4/F6 already matched; verify-on-open belongs in ADR-012 |
| ADR-061 | no change (F2); open question (F8) | Open question (2026-09-30) | 3 | F2 portable-reference paragraph already present |
| ADR-081 | no change | skipped | — | Action entity is ADR-050/051; D081 is sandbox isolation |
| ADR-002 | no change | skipped | — | Evidence note lives in the review doc |
| ADR-001 | no change | skipped | — | Vision one-liner already states log = SoT |
| ADR-000 | open question (F7 / D4) | Open question (2026-09-30) | 3 | D1–D10 Decision text not rewritten |

### git-diff removed-line count per ADR file (vs plan_head_before)

All counts are content lines starting with `-` in `git diff -U0` (excluding `---` headers). Working tree plus the three task commits.

| File | Removed | Added |
|------|---------|-------|
| ADR-000-kutha-hybrid-architecture-research.md | 0 | 4 |
| ADR-010-event-log-runtime-quantum.md | 0 | 4 |
| ADR-011-lean-event-schema-lineage.md | 0 | 4 |
| ADR-012-snapshots-tiers-vacuum.md | 0 | 4 |
| ADR-013-bitemporal-facts-invalidation.md | 0 | 8 |
| ADR-014-cascade-budgets-quantum-receipts.md | 0 | 4 |
| ADR-040-materialization-plugin-protocol.md | 0 | 4 |
| ADR-041-csr-graphblas-hot-path.md | 0 | 4 |
| ADR-050-meta-prompt-dictionaries.md | 0 | 8 |
| ADR-051-capability-security.md | 0 | 4 |
| ADR-060-strict-replay.md | 0 | 4 |
| ADR-061-fork-and-diff.md | 0 | 4 |
| README.md | 0 | 2 |

ADR-001, ADR-002, ADR-081: no diff. `.kutha/STATE.md` and `crates/` were not edited.

## Decisions Made

Followed 10-CONTEXT.md D-M1, D-M2, D-C1, D-G1, D-G2. First tracer cell is ADR-012 because it is the first `amend` row in the verdict table. Honeycomb restage is comments naming cells touched; no stage or edge change. Committed on `main` under `git.branching_strategy: none` (same as 10-01). No `--no-verify`. This wave did not run codebase-memory-mcp (`list_projects` was not invoked); no code-graph verification is claimed.

## Deviations from Plan

None — plan executed as written. Using ADR-012 as the tracer cell is the plan's own table-over-hypothesis rule, not a Rule 1–3 auto-fix.

Committed on `main` under `git.branching_strategy: none` (#3552 / #3819 warning; same as Phase 9 and 10-01). No `--no-verify`.

**Total deviations:** 0 auto-fixed
**Impact on plan:** None.

## Issues Encountered

None.

## Trajectory (D-10 / D-G1)

1. **Commands:** `uv run kutha-gov ci`; `uv run kutha-gov explain trajectory`; `uv run kutha-gov precommit`; `uv run kutha-gov map`; `uv run kutha-gov precommit --check honeycomb-ledger`; `uv run pytest -q scripts/tests`.
2. **Outcome:** `ci` exit 0; **HIGH 0**, **LOW 0** (32 checks). `precommit` exit 0. `map` 31 cells, all Proposed. `honeycomb-ledger` OK. `pytest -q scripts/tests` — 48 passed.
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
4. **Honesty:** Green governor is not ADR Accepted and not L_capability. Freeze and Active Slice None were not edited. `git diff --exit-code -- crates .cursor/rules .kutha/STATE.md` is clean.

## Threat Flags

None beyond the plan register (T-10-06…T-10-10, T-10-SC). No new network endpoints, packages, or crate schema. Dated subsections cite `docs/architecture/semantic-gap-review.md`. No crate/function/milestone ids used as commitments in the new prose.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- ADR-02 and ADR-04 are satisfied in the map plane. Phase 11 owns SEM-08 Process narrative and semantic governor kinds.
- No crate work. Do not lease M012a from this SUMMARY.

---
## Self-Check: PASSED

*Phase: 10-adr-and-roadmap-correction*
*Completed: 2026-09-30*
