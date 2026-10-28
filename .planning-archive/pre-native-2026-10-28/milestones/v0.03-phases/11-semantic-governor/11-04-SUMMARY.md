---
phase: 11-semantic-governor
plan: 04
subsystem: harness-semantic-close
tags: [SEM-07, SEM-08, D-D1, D-Q2, D-Q3, D-Q4, D-G1, D-G2, D-G3, H5, deferred-invariants]

requires:
  - phase: 11-semantic-governor
    provides: selftest, rust_test_asserts, cite_equals, refs_resolve, file_max_lines
provides:
  - I-F1-outcomes … I-F8-hot-reads deferred (until M012a / M012 / M002)
  - META kinds rust_test_asserts, cite_equals, refs_resolve, file_max_lines; FSM run_selftest
  - governor-intake “What a green check proves”
  - AGENTS.md selftest command (101 lines)
  - Real Process/Trajectory CHANGELOG for Phases 9–11
  - H5 dogfood checkbox closed; h4-lease requires done form
affects: [phase-11-close, unleased-M012a]

estimate:
  tokens: 28000
  tasks: 3

actuals:
  tokens: 7837
  tasks: 3
  commits: 3

plan_head_before: e0f0ced7e4956a45573af28a7c5a68f3808671dc
plan_head_after: 76942f226c4199366f89265db40870a74a9e6ce2
commits: 3

tech-stack:
  added: []
  patterns:
    - Deferred product gaps are invariants.yaml rows with until, not HIGH checks
    - checks.yaml edits share the git diff with invariants.yaml or bridges.yaml

key-files:
  created:
    - .planning/phases/11-semantic-governor/11-04-SUMMARY.md
  modified:
    - .kutha/dictionaries/invariants.yaml
    - .kutha/META.md
    - .kutha/dictionaries/checks.yaml
    - docs/process/governor-intake.md
    - docs/process/kutha-harness.md
    - .kutha/ROADMAP.md
    - AGENTS.md
    - CHANGELOG.md

key-decisions:
  - "D-D1: I-F4 until is STATE names M012 (rule registry / n-ary derivation), not M012a"
  - "D-D1: I-F6 until names M012a; durability protocol waits until STATE names M002"
  - "D-Q4: h4-lease id kept; ROADMAP needle is the checked H5 checkbox only"
  - "D-G3: green governor is not product correctness and not ADR Accepted"

patterns-established:
  - "Record F-gaps as deferred control-loop claims sourced from the review note, never docs/ADR/ADR- in invariants.yaml"
  - "Close a dogfood rung by flipping the ROADMAP checkbox and retargeting the existing lease check"

requirements-completed: [SEM-07, SEM-08]

coverage:
  - id: D1
    description: Eight deferred I-F1…I-F8 rows with until, review source, no product check ids
    requirement: SEM-07
    verification:
      - kind: other
        ref: uv run kutha-gov precommit --check invariants-ledger
        status: pass
    human_judgment: false
  - id: D2
    description: META/intake/AGENTS document new kinds and selftest; CHANGELOG is real Process/Trajectory
    requirement: SEM-08
    verification:
      - kind: other
        ref: uv run kutha-gov precommit --check meta-prompt
        status: pass
      - kind: other
        ref: uv run kutha-gov precommit --check changelog-planes
        status: pass
    human_judgment: false
  - id: D3
    description: H5 checkbox checked; h4-lease requires done form; STATE byte-stable this plan
    requirement: SEM-08
    verification:
      - kind: other
        ref: uv run kutha-gov precommit --check h4-lease
        status: pass
    human_judgment: false
  - id: D4
    description: Full governor gate green (selftest, ci HIGH 0, pytest, py, cargo)
    requirement: SEM-08
    verification:
      - kind: other
        ref: uv run kutha-gov ci
        status: pass
      - kind: unit
        ref: scripts/tests (98 passed)
        status: pass
    human_judgment: false

duration: 34min
completed: 2026-09-30
status: complete
---

# Phase 11 Plan 04: Deferred F-gaps, process history, and H5 close Summary

**Eight F-gaps ledgered as deferred control-loop claims; Phases 9–11 CHANGELOG is real Process/Trajectory; H5 dogfood is checked without touching crates or `.kutha/STATE.md`**

## Performance

- **Duration:** 34 min
- **Started:** 2026-09-30T10:28:53Z
- **Completed:** 2026-09-30T11:02:46Z
- **Tasks:** 3
- **Files modified:** 8 (plus this SUMMARY)

## Accomplishments

- `I-F1-outcomes` … `I-F8-hot-reads` exist with `disposition: deferred`, `source: docs/architecture/semantic-gap-review.md`, and `until` naming M012a / M012 / M002. No check ids. No crate symbols as commitments. The ledger has zero `docs/ADR/ADR-` substrings.
- META lists `rust_test_asserts`, `cite_equals`, `refs_resolve`, `file_max_lines`, and FSM `run_selftest`. “How to add a check” documents `selftest:` (mutations or skip-with-reason; unproven fails). Intake has “What a green check proves”. `AGENTS.md` has one `uv run kutha-gov selftest` Commands line (101 ≤ 110; no lease tokens).
- CHANGELOG pointer stubs (“Narrative remains Phase 11 SEM-08”) from Phases 9, 10, and 11-01…11-03 are replaced by dated Process + Trajectory entries (H5 lease and AGENTS diet 194→~100; semantic-gap review and 12 Proposed ADR notes; semantic governor kinds, Cui budget 32→48, no vacuous named test, deferred F1–F8). No SemVer, tag, or release wording.
- ROADMAP `- [x] **H5**`; `h4-lease` description and needle require the done checkbox; `kutha-harness.md` marks H5 **Done** (current rung, no H6). `.kutha/STATE.md` untouched this plan.

## Task Commits

1. **Task 1: End-to-end one deferred F1 invariant plus intake meaning section** - `6eafc3d` (docs)
2. **Task 2: Remaining F2–F8, META kinds, AGENTS selftest line** - `9ad834a` (docs)
3. **Task 3: Replace changelog stubs, close H5, final gate** - `76942f2` (docs)

## Files Created/Modified

- `.kutha/dictionaries/invariants.yaml` — I-F1…I-F8 deferred; I-h4 claim names H5 closed
- `.kutha/META.md` — new kinds, `run_selftest`, selftest intake step
- `.kutha/dictionaries/checks.yaml` — meta-prompt needles; h4-lease done-form
- `docs/process/governor-intake.md` — What a green check proves
- `docs/process/kutha-harness.md` — H5 Done
- `.kutha/ROADMAP.md` — H5 checkbox checked
- `AGENTS.md` — selftest command
- `CHANGELOG.md` — real Process/Trajectory for Phases 9–11
- `.planning/phases/11-semantic-governor/11-04-SUMMARY.md` — this file

## Decisions Made

- I-F4 `until: STATE names M012` (ROADMAP later-milestones + parent quality bar: rule registry and n-ary derivation are M012). Plan interfaces had M012a.
- I-F6 `until` names M012a for verify-on-open / atomic persist / stable Define ids; durability protocol waits until STATE names M002.
- I-F1/F2/F3/F7/F8 until M012a; I-F5 until M012.
- H5 close keeps check id `h4-lease`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] checks.yaml H5 retarget required a same-diff invariants row**
- **Found during:** Task 3 live `kutha-gov ci`
- **Issue:** Uncommitted `checks.yaml` without `invariants.yaml` tripped `docs-coupling` HIGH (`then_any` ledger files).
- **Fix:** Updated `I-h4-adr-090-overlay` claim to “H5 dogfood is closed and H4 overlay remains delivered”.
- **Files modified:** `.kutha/dictionaries/invariants.yaml`
- **Verification:** `ci` HIGH 0; `docs-coupling` OK
- **Committed in:** `76942f2` (Task 3)

**2. [Rule 2 - Missing Critical] I-F4 until follows ROADMAP M012, not plan M012a**
- **Found during:** Task 2 (parent quality bar vs plan interfaces)
- **Issue:** Plan interfaces listed I-F4 until M012a; later-milestones put rule registry / n-ary derivation on M012.
- **Fix:** `until: "STATE names M012"` on `I-F4-derivation`.
- **Files modified:** `.kutha/dictionaries/invariants.yaml`
- **Verification:** eight I-F rows; `invariants-ledger` OK
- **Committed in:** `9ad834a` (Task 2)

---

**Total deviations:** 2 auto-fixed (Rule 2)
**Impact on plan:** Coupling and until-naming keep the ledger honest. No crate scope creep.

## Issues Encountered

`workflow.tdd_mode` is false; plan type is `execute`. Committed on `main` under `git.branching_strategy: none` (`git.base-branch --is-protected` is true; same as 11-01…11-03). No `--no-verify`. No local `.env`. Code-graph tools were not queried.

## User Setup Required

None - no external service configuration required.

## Trajectory (D-10 / D-G1)

1. **Commands:** `uv run kutha-gov selftest`; `uv run kutha-gov ci`; `uv run kutha-gov explain trajectory`.
2. **Outcome:** selftest exit 0 (**34 OK**, **1 SKIP** `docs-coupling`, 0 UNPROVEN, 0 VACUOUS, 0 BASELINE-FAIL). `ci` exit 0; **HIGH 0**, **LOW 0**; **35 checks**; evidence `h5_selftest=35/35`; harness line **H5 dogfood**; no `cui: truncated` line. `uv run kutha-gov precommit` exit 0. `uv run pytest -q scripts/tests` — 98 passed. `uv run kutha-gov py` — PYTHON_TOOLING_GREEN. `RUSTC_WRAPPER= cargo test --workspace` — all ok. `git diff --exit-code 76da78d..HEAD -- crates .cursor/rules` — clean.
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
4. **Honesty:** Green governor is not product correctness, not ADR Accepted, and not L_capability. F1–F8 wait on unleased M012a (F4/F5 on M012; F6 durability protocol on M002).

Cited: D-D1 (deferred F-gaps), D-Q2 (META/intake/AGENTS), D-Q3 (CHANGELOG), D-Q4 (H5 close), D-G3 (green ≠ Accepted). Docs/process only: codebase-memory-mcp was not queried; this wave does not claim graph verification. `git diff --exit-code -- crates .kutha/STATE.md` is clean. `git diff --stat 76da78d..HEAD -- .kutha/STATE.md` is the Phase 9 H5 lease only (2 insertions / 2 deletions); this plan did not touch it.

## Vacuous checks

Live-repo selftest found **no** vacuous YAML row and **no** vacuous named test. Synthetic pytest still reports HIGH for mismatch cite, dangling ADR-999, over-max lines, and empty-needle `file_contains`.

Skip reasons (not vacuous): `docs-coupling` depends on the live git diff.

## Budget behaviour

`defaults.budget` and `.env.example` remain 48 from 11-03. This run: **35 checks**, `skipped == 0`, no `cui: truncated` line.

## Final gate (paste)

```
selftest: 34 OK, 1 SKIP docs-coupling, exit 0
ci: HIGH 0, LOW 0, 35 checks, h5_selftest=35/35, (H5 dogfood), decide → ok
precommit: all 35 OK, exit 0
pytest -q scripts/tests: 98 passed
kutha-gov py: PYTHON_TOOLING_GREEN
cargo test --workspace: ok (RUSTC_WRAPPER=)
git diff 76da78d..HEAD -- crates .cursor/rules: empty
```

## Threat Flags

None beyond the plan register (T-11-11…T-11-14, T-11-SC). No new packages, endpoints, or crate schema.

## Next Phase Readiness

- Phase 11 SEM-07/SEM-08 delivered. H5 is closed on the dogfood ladder. Do not lease M012a/M012/M002. Do not treat governor green as product correctness.

---
*Phase: 11-semantic-governor*
*Completed: 2026-09-30*

## Self-Check: PASSED

- `11-04-SUMMARY.md` exists
- Commits `6eafc3d`, `9ad834a`, `76942f2` exist
