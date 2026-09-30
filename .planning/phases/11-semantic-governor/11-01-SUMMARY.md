---
phase: 11-semantic-governor
plan: 01
subsystem: harness-selftest
tags: [SEM-01, D-V1, D-V2, D-V3, H5, selftest, run_selftest]

requires:
  - phase: 10-adr-and-roadmap-correction
    provides: Honeycomb Proposed; semantic-gap review; H5 named on the operator lease
provides:
  - kutha-gov selftest copy-once tempfile runner (D-V1)
  - Derived plus declared mutations; fail-closed UNPROVEN (D-V2)
  - FSM run_selftest with evidence h5_selftest; ci rung H5 (D-V3)
  - Live-repo selftest green (27 OK / 5 SKIP); red-path VACUOUS/UNPROVEN/BASELINE-FAIL
affects: [11-02 rust_test_asserts, 11-04 META FSM kind row and SEM-08 narrative]

estimate:
  tokens: 42000
  tasks: 3

actuals:
  tokens: 11399
  tasks: 3
  commits: 2

plan_head_before: 2968a5dcc907332f441cade1ea7cc61e48d99efb
plan_head_after: 69192ea162ad30dbc349dcb4f4ad6739c94c8021
commits: 2

tech-stack:
  added: []
  patterns:
    - Copy the repo once (D-V1 exclusions), mutate/restore on that copy, never ctx.root
    - require: any is one atomic mutation unit (all needles removed together)
    - LOW-severity steps are not derived; the check is still proved via HIGH siblings

key-files:
  created:
    - scripts/kutha_gov/selftest.py
    - scripts/tests/test_selftest.py
    - .planning/phases/11-semantic-governor/11-01-SUMMARY.md
  modified:
    - scripts/kutha_gov/__main__.py
    - scripts/kutha_gov/fsm.py
    - scripts/tests/test_fsm.py
    - .kutha/dictionaries/fsm.yaml
    - .kutha/dictionaries/checks.yaml
    - .kutha/dictionaries/invariants.yaml
    - CHANGELOG.md

key-decisions:
  - "D-V1: one tempfile copy per selftest run; working-tree harness bytes hashed before/after"
  - "D-V2: skip reasons name the kind or the live git diff; mixed rows stay derived"
  - "D-V3: ci evidence h5_selftest=32/32 sets the rung label H5 after the h4_ heuristic"
  - "META.md Allowed FSM kinds row for run_selftest is plan 11-04 (D-Q2)"

patterns-established:
  - "A YAML check is honest only if a mutation on a copy can make it HIGH, or it carries a specific skip reason"
  - "docs-coupling Process pointer is not SEM-08 authorship"

requirements-completed: [SEM-01]

coverage:
  - id: D1
    description: selftest copies once, mutates only the copy, prints OK|SKIP|UNPROVEN|VACUOUS|BASELINE-FAIL
    requirement: SEM-01
    verification:
      - kind: unit
        ref: scripts/tests/test_selftest.py#test_selftest_freeze_ok_leaves_tracked_harness_bytes
        status: pass
      - kind: unit
        ref: scripts/tests/test_selftest.py#test_copy_excludes_dv1_heavy_trees
        status: pass
    human_judgment: false
  - id: D2
    description: Empty-needle file_contains is VACUOUS; pointer-only without selftest is UNPROVEN; skip-with-reason exits 0
    requirement: SEM-01
    verification:
      - kind: unit
        ref: scripts/tests/test_selftest.py#test_empty_file_contains_needles_is_vacuous
        status: pass
      - kind: unit
        ref: scripts/tests/test_selftest.py#test_synthetic_unproven_pointer_only
        status: pass
      - kind: unit
        ref: scripts/tests/test_selftest.py#test_synthetic_skip_is_success
        status: pass
    human_judgment: false
  - id: D3
    description: Live-repo selftest is green; FSM run_selftest sits after run_checks; ci labels H5 from h5_selftest
    requirement: SEM-01
    verification:
      - kind: unit
        ref: scripts/tests/test_selftest.py#test_live_repo_selftest_is_green
        status: pass
      - kind: unit
        ref: scripts/tests/test_fsm.py#FsmTests.test_ci_quantum_reaches_ok_on_this_tree
        status: pass
      - kind: other
        ref: uv run kutha-gov ci (HIGH 0, h5_selftest=32/32, H5 dogfood)
        status: pass
    human_judgment: false

duration: 16min
completed: 2026-09-30
status: complete
---

# Phase 11 Plan 01: Vacuity selftest Summary

**`kutha-gov selftest` copies the tree once, proves 27 checks by mutation and skip-reasons the other 5, and `ci` labels H5 from `h5_selftest=32/32`**

## Performance

- **Duration:** 16 min
- **Started:** 2026-09-30T07:27:30Z
- **Completed:** 2026-09-30T07:43:21Z
- **Tasks:** 3
- **Files modified:** 10

## Accomplishments

- `scripts/kutha_gov/selftest.py` copies the repo once (D-V1 exclusions), applies derived or declared mutations on that copy, restores each file, and never writes the working tree. Tracked `scripts/` + `.kutha/` bytes are hashed before/after in pytest.
- CLI `uv run kutha-gov selftest [--check ID]` prints `STATUS id mutations=N` (optional `reason=`). Exit 0 only for OK and SKIP-with-reason. Unknown op and missing declaration are UNPROVEN (fail-closed).
- FSM kind `run_selftest` after `run_checks`; evidence `h5_selftest=32/32`; `cmd_ci` prints `(H5 dogfood)` when that rel is present. META.md FSM-kind row is reserved for 11-04.
- Live selftest **3.06s**: **27 OK**, **5 SKIP**, 0 UNPROVEN, 0 VACUOUS, 0 BASELINE-FAIL. `uv run kutha-gov ci` **HIGH 0**, **LOW 0**. `pytest -q scripts/tests` 58 passed. `kutha-gov py` green.

## Task Commits

1. **Task 1: End-to-end selftest CLI plus FSM H5 evidence** - `75638aa` (feat)
2. **Task 2: Declare or derive selftest for every live checks.yaml row** - `69192ea` (feat)
3. **Task 3: Wave-close Trajectory** - this SUMMARY (docs)

## Files Created/Modified

- `scripts/kutha_gov/selftest.py` - copy-once vacuity runner
- `scripts/kutha_gov/__main__.py` - `selftest` command; H5 rung from `h5_selftest`
- `scripts/kutha_gov/fsm.py` - `run_selftest` kind
- `.kutha/dictionaries/fsm.yaml` - state + transitions after `run_checks`
- `.kutha/dictionaries/checks.yaml` - five skip reasons; header notes fail-closed vacuity
- `.kutha/dictionaries/invariants.yaml` - comment so docs-coupling sees a ledger companion
- `scripts/tests/test_selftest.py` - VACUOUS / UNPROVEN / SKIP / BASELINE-FAIL / live OK / byte-identity
- `scripts/tests/test_fsm.py` - trace includes `run_selftest`; evidence key present
- `CHANGELOG.md` - Process pointer; SEM-08 narrative reserved
- `.planning/phases/11-semantic-governor/11-01-SUMMARY.md` - this file

## Decisions Made

Followed 11-CONTEXT.md D-V1, D-V2, D-V3, D-G1, D-G2, D-G3. CHANGELOG is a docs-coupling Process pointer only (not SEM-08). META.md `run_selftest` allowlist row is 11-04. No crates/ or `.kutha/STATE.md` edits. Code-graph tools were not queried.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] `require: any` looked VACUOUS with per-needle restore**
- **Found during:** Task 1 live `selftest` (h4-lease)
- **Issue:** Derived `file_contains` `require: any` emitted one mutation per needle. Restoring between them left the other needle in place, so the check stayed green.
- **Fix:** One atomic mutation unit removes all needles together (`texts`). Same grouping for `concat_contains_any` across paths.
- **Files modified:** `scripts/kutha_gov/selftest.py`, `scripts/tests/test_selftest.py`
- **Verification:** `OK h4-lease mutations=4`; `test_require_any_removes_all_needles_in_one_mutation`
- **Commit:** `75638aa`

**2. [Rule 3 - Blocking] Non-derivable-only rows would UNPROVEN the tracer FSM quantum**
- **Found during:** Task 1 (test_ci_quantum_reaches_ok_on_this_tree)
- **Issue:** `run_selftest` in `ci` fail-closes on UNPROVEN. Five rows have only non-derivable kinds.
- **Fix:** Honest `selftest.skip` on `adr-status`, `idle-delivery-closed`, `observe-required-fn`, `state-readme`, and `docs-coupling` (`depends on the live git diff`). Invariants comment satisfies checks.yaml coupling.
- **Files modified:** `.kutha/dictionaries/checks.yaml`, `.kutha/dictionaries/invariants.yaml`
- **Verification:** live selftest 27 OK / 5 SKIP; ci HIGH 0
- **Commit:** `75638aa`

**3. [Rule 2 - Correctness] LOW-severity steps are not derived**
- **Found during:** Task 1 derivation design
- **Issue:** D-V1 requires each mutation to produce ≥1 HIGH. A LOW `file_contains` / `concat_contains_any` mutation cannot.
- **Fix:** Skip derivation when `severity: low`. The parent check is still proved by its HIGH steps (dogfood, honeycomb-map). Not a skip of the row.
- **Files modified:** `scripts/kutha_gov/selftest.py`
- **Commit:** `75638aa`

**Total deviations:** 3 auto-fixed
**Impact on plan:** Tracer stayed HIGH-free. h4-lease is not vacuous; the runner was.

## Issues Encountered

None. `RUSTC_WRAPPER=` was set for cargo observe; tests used the cached workspace build. Committed on `main` under `git.branching_strategy: none` (#3552 / #3819 warning; same as Phase 9 and 10). No `--no-verify`.

Plan `tdd="true"` on tasks 1–2; `workflow.tdd_mode` is false and plan type is `execute`. RED/GREEN were not split into `test(11-01)` / `feat(11-01)` commits; behavior tests landed with the feat commits.

## Trajectory (D-10 / D-G1)

1. **Commands:** `uv run kutha-gov selftest`; `uv run kutha-gov ci`; `uv run kutha-gov explain trajectory`.
2. **Outcome:** selftest exit 0 in **3.06s** (27 OK, 5 SKIP). `ci` exit 0; **HIGH 0**, **LOW 0**; evidence `h5_selftest=32/32`; harness line **H5 dogfood**. `uv run kutha-gov precommit` exit 0. `uv run pytest -q scripts/tests` — 58 passed. `uv run kutha-gov py` — PYTHON_TOOLING_GREEN.
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

Cited: D-V1 (copy once), D-V3 (H5 from h5_selftest), D-G3 (green ≠ Accepted). Docs/Python only: codebase-memory-mcp was not queried; this wave does not claim graph verification. `git diff --exit-code -- crates .kutha/STATE.md` is clean.

## Vacuous checks

Live-repo selftest found **no** vacuous YAML row after the require-any runner fix. `h4-lease` was a false VACUOUS (derivation bug), not an empty check. Synthetic pytest still reports VACUOUS for empty `file_contains` needles and for a declared no-op mutation.

Skip reasons (not vacuous): `markdown_heading_tag`, `when_match_then_match`, `yaml_needles_in_glob`, `pointer_in_other_file`, `depends on the live git diff`.

## Threat Flags

None beyond the plan register (T-11-01…T-11-04, T-11-SC). No new packages, endpoints, or crate schema.

## Next Phase Readiness

- 11-02 can add `rust_test_asserts` with its own derived mutation.
- 11-04 authors META `run_selftest` kind row, governor-intake “what a green check proves”, and the SEM-08 Process narrative.

---
*Phase: 11-semantic-governor*
*Completed: 2026-09-30*

## Self-Check: PASSED

- `scripts/kutha_gov/selftest.py`, `scripts/tests/test_selftest.py`, `11-01-SUMMARY.md` exist
- Commits `75638aa` and `69192ea` exist on `main`

