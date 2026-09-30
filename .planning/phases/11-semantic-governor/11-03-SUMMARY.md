---
phase: 11-semantic-governor
plan: 03
subsystem: harness-semantic-citations
tags: [SEM-03, SEM-05, SEM-06, D-C1, D-X1, D-B1, D-G1, cite_equals, refs_resolve, file_max_lines]

requires:
  - phase: 11-semantic-governor
    provides: rust_test_asserts and mutation-proved dictionary; 11-02 selftest runner
provides:
  - cite_equals kind (one capture group; mismatch HIGH; zero cites pass; source miss fail-closed)
  - refs_resolve kind (ADR ranges as endpoints; ADR-XXX ignored; D010-1/D050-2 not locks; ADR-100 allow_dangling)
  - file_max_lines kind plus agents-lean (AGENTS.md wc -l <= 110; lease tokens absent)
  - fsm.yaml defaults.budget 48 and .env.example KUTHA_GOV_BUDGET=48 so ci cannot truncate
affects: [11-04 META kind rows and SEM-08 narrative]

estimate:
  tokens: 36000
  tasks: 3

actuals:
  tokens: 9246
  tasks: 3
  commits: 3

plan_head_before: f8d1d1909571349a99b43ba3a32edec2e8c58f39
plan_head_after: 84cd5ad1d37ca19cda0c81973a028eeed0d9e8fe
commits: 3

tech-stack:
  added: []
  patterns:
    - Concrete lease tokens are cites; empty KEY= and <placeholder> values are not
    - ADR ranges contribute endpoints only; four-digit neighbor ids are not ADR-NNN
    - cui V slices sorted check ids; skipped prints "cui: truncated N slices"

key-files:
  created:
    - .planning/phases/11-semantic-governor/11-03-SUMMARY.md
  modified:
    - scripts/kutha_gov/kinds.py
    - scripts/kutha_gov/selftest.py
    - scripts/tests/test_kutha_gov.py
    - scripts/tests/test_fsm.py
    - .kutha/dictionaries/checks.yaml
    - .kutha/dictionaries/invariants.yaml
    - .kutha/dictionaries/fsm.yaml
    - .env.example
    - CHANGELOG.md

key-decisions:
  - "D-C1: capture [A-Za-z0-9][A-Za-z0-9._-]* after KEY=; backticks do not exempt a concrete token"
  - "D-X1: scan only AGENTS.md, README.md, .kutha/**/*.md, dictionary YAML, docs/process; ADR-100 may dangle"
  - "D-B1: agents-lean is wc -l <= 110 plus file_absent of the four lease needles"
  - "D-G1: fsm default and .env.example budget 48; no local .env (would have truncated at 32)"

patterns-established:
  - "A semantic check is honest only if a tempfile mismatch goes HIGH and a legitimate non-cite stays silent"
  - "Intake: invariants.yaml disposition check row before the checks.yaml id"

requirements-completed: [SEM-03, SEM-05, SEM-06]

coverage:
  - id: D1
    description: cite_equals fails on a stale concrete L_delivery token and passes zero-cite / placeholder prose
    requirement: SEM-03
    verification:
      - kind: unit
        ref: scripts/tests/test_kutha_gov.py#CiteEqualsTests.test_cite_equals_stale_s03_token_even_in_backticks_is_high
        status: pass
      - kind: unit
        ref: scripts/tests/test_kutha_gov.py#CiteEqualsTests.test_cite_equals_placeholder_and_empty_assignment_are_not_cites
        status: pass
    human_judgment: false
  - id: D2
    description: refs_resolve fails on ADR-999; allows ADR-100; ignores ADR-XXX, ADR-0007, D050-2, D010-1
    requirement: SEM-05
    verification:
      - kind: unit
        ref: scripts/tests/test_kutha_gov.py#RefsResolveTests.test_refs_resolve_high_on_dangling_adr_999
        status: pass
      - kind: unit
        ref: scripts/tests/test_kutha_gov.py#RefsResolveTests.test_refs_resolve_d050_2_and_d010_1_are_not_lock_ids
        status: pass
      - kind: other
        ref: uv run kutha-gov precommit --check refs-resolve
        status: pass
    human_judgment: false
  - id: D3
    description: agents-lean enforces AGENTS.md wc -l <= 110 and forbids lease assignment tokens
    requirement: SEM-06
    verification:
      - kind: unit
        ref: scripts/tests/test_kutha_gov.py#FileMaxLinesTests.test_agents_lean_append_lines_over_budget_is_high
        status: pass
      - kind: unit
        ref: scripts/tests/test_kutha_gov.py#FileMaxLinesTests.test_agents_lean_append_delivery_token_is_high
        status: pass
      - kind: other
        ref: uv run kutha-gov precommit --check agents-lean
        status: pass
    human_judgment: false
  - id: D4
    description: ci V=48 runs all 35 dictionary checks; a short V truncates the alphabetical tail
    requirement: SEM-03
    verification:
      - kind: unit
        ref: scripts/tests/test_fsm.py#FsmTests.test_ci_quantum_reaches_ok_on_this_tree
        status: pass
      - kind: unit
        ref: scripts/tests/test_fsm.py#FsmTests.test_cui_budget_truncates_tail_checks_when_v_is_short
        status: pass
    human_judgment: false

duration: 42min
completed: 2026-09-30
status: complete
---

# Phase 11 Plan 03: semantic citation checks Summary

**`cite_equals` / `refs_resolve` / `agents-lean` fail closed on stale lease tokens, dangling ADR/lock ids, and AGENTS.md budget or token regression; ci V=48 runs all 35 checks**

## Performance

- **Duration:** 42 min
- **Started:** 2026-09-30T09:41:07Z
- **Completed:** 2026-09-30T10:23:13Z
- **Tasks:** 3
- **Files modified:** 9

## Accomplishments

- `cite_equals` requires exactly one capturing group. Source miss or disagreeing source captures are HIGH. Each cite capture must equal the source. Zero cites pass.
- Capture rule (documented on `cite-lease`): `KEY=([A-Za-z0-9][A-Za-z0-9._-]*)`. Empty `L_delivery=` and `L_delivery=<lease>` are not cites. A concrete token, including one inside backticks, is a cite.
- `refs_resolve` scans the D-X1 scope only. Ranges (hyphen or U+2013) add both endpoints. `ADR-XXX` is ignored. `ADR-0007` is ignored (`(?!\d)`). `D010-1` / `D050-2` are not lock ids. Lock ids are `D1`…`D10` against honeycomb `locks`. `ADR-100` is `allow_dangling`.
- `file_max_lines` uses `wc -l` (newline count). `agents-lean` is max 110 plus `file_absent` of `L_delivery=`, `L_map=`, `L_capability=`, `M011 is active`.
- Intake order: `I-cite-lease`, `I-refs-resolve`, `I-agents-lean` in `invariants.yaml` before the matching `checks.yaml` ids.
- `fsm.yaml` `defaults.budget` and `.env.example` `KUTHA_GOV_BUDGET` are 48. No local `.env`.

## Live findings

| Kind | Finding | Resolution |
|------|---------|------------|
| Wrong cite | None in D-C1 globs. AGENTS.md says "Read `.kutha/STATE.md`" with no assignment tokens. README/STRATEGY/process/architecture/ADR README have no concrete `L_*=` or Active Milestone M### status phrasing. | No doc edit. Stale `L_delivery=M011-S03-done` remains only in CHANGELOG / `.planning` (out of glob). Red-path tempfile proves that token HIGH. |
| Dangling ADR/lock | None in D-X1 scope. STATE "Do not open ADR-100" is allowed. `docs/process/kutha-harness.md` `ADR-0007` is not a three-digit id. `governor-intake.md` `D050-2` is not a lock. Range `ADR-010–093` endpoints exist. | No process-doc typo to fix. ADR-999 is HIGH only on the tempfile / selftest mutation. |

## Task Commits

1. **Task 1: End-to-end cite_equals for L_delivery versus STATE** - `3383240` (feat)
2. **Task 2: Complete cite-lease steps and add refs_resolve** - `53e03cb` (feat)
3. **Task 3: agents-lean budget, ci budget 48, wave-close** - `84cd5ad` (feat)

**Plan metadata:** `docs(11-03): complete semantic citation checks plan`

_Note: `workflow.tdd_mode` is false; plan type is `execute`. RED/GREEN were not split into `test(11-03)` / `feat(11-03)` commits._

## Files Created/Modified

- `scripts/kutha_gov/kinds.py` - `cite_equals`, `refs_resolve`, `file_max_lines` plus derived-mutation targets
- `scripts/kutha_gov/selftest.py` - derived mutations for the three kinds
- `.kutha/dictionaries/checks.yaml` - `cite-lease` (four steps), `refs-resolve`, `agents-lean`
- `.kutha/dictionaries/invariants.yaml` - `I-cite-lease`, `I-refs-resolve`, `I-agents-lean`
- `.kutha/dictionaries/fsm.yaml` - `defaults.budget: 48`
- `.env.example` - `KUTHA_GOV_BUDGET=48`
- `scripts/tests/test_kutha_gov.py` - tempfile red paths
- `scripts/tests/test_fsm.py` - quantum uses `budget_default`; asserts `skipped == 0`
- `CHANGELOG.md` - Process pointer bullets; SEM-08 narrative reserved

## Decisions Made

Followed 11-CONTEXT.md D-C1, D-X1, D-B1, D-G1, D-G2, D-G3. META.md Allowed kinds rows remain plan 11-04. AGENTS.md selftest command line remains 11-04. No `crates/` or `.kutha/STATE.md` or `docs/ADR/` body edits. Code-graph tools were not queried.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Derived cite mutation used a non-capturing token**
- **Found during:** Task 1 live `selftest --check cite-lease`
- **Issue:** Appending `L_delivery=__cite_selftest__` did not match `[A-Za-z0-9][A-Za-z0-9._-]*`, so the check stayed HIGH-free (VACUOUS).
- **Fix:** Append `L_delivery=WRONG` (and the same for `L_map=` / `L_capability=`).
- **Files modified:** `scripts/kutha_gov/kinds.py`
- **Verification:** `OK cite-lease mutations=1` then `mutations=4` after task 2
- **Commit:** `3383240`

**2. [Rule 1 - Bug] Ruff RUF001 / format / pyrefly on new kinds**
- **Found during:** Task 3 `uv run kutha-gov py`
- **Issue:** Literal en dash in the range regex and a test string; a multiline `Finding` append; `rows` possibly not a list.
- **Fix:** `\u2013` via `_EN_DASH`; collapse the append; narrow `rows` to `list`.
- **Files modified:** `scripts/kutha_gov/kinds.py`, `scripts/tests/test_kutha_gov.py`
- **Verification:** `PYTHON_TOOLING_GREEN`
- **Commit:** `84cd5ad`

---

**Total deviations:** 2 auto-fixed
**Impact on plan:** Capture rule and en-dash ranges still fail closed; no scope creep.

## Issues Encountered

`workflow.tdd_mode` is false; plan type is `execute`. Committed on `main` under `git.branching_strategy: none` (`git.base-branch --is-protected` is true; same as 11-01/11-02). No `--no-verify`. No local `.env`.

## Trajectory (D-10 / D-G1)

1. **Commands:** `uv run kutha-gov selftest`; `uv run kutha-gov ci`; `uv run kutha-gov explain trajectory`.
2. **Outcome:** selftest exit 0 (**34 OK**, **1 SKIP** `docs-coupling`, 0 UNPROVEN, 0 VACUOUS, 0 BASELINE-FAIL). `ci` exit 0; **HIGH 0**, **LOW 0**; **35 checks**; evidence `h5_selftest=35/35`; harness line **H5 dogfood**; no `cui: truncated` line. `uv run kutha-gov precommit` exit 0. `uv run pytest -q scripts/tests` — 98 passed. `uv run kutha-gov py` — PYTHON_TOOLING_GREEN.
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

Cited: D-C1 (cite equals STATE), D-X1 (refs resolve), D-B1 (AGENTS budget), D-G3 (green ≠ Accepted). Docs/Python only: codebase-memory-mcp was not queried; this wave does not claim graph verification. `git diff --exit-code -- crates .kutha/STATE.md` is clean.

## Vacuous checks

Live-repo selftest found **no** vacuous YAML row. Synthetic pytest still reports HIGH for mismatch cite, missing source capture, dangling ADR-999, over-max lines, and appended lease tokens.

Skip reasons (not vacuous): `docs-coupling` depends on the live git diff.

## Budget behaviour

`run_checks` takes `sorted(checks.items())[:budget]` and sets `outcome.skipped`. When `skipped > 0`, `ci` prints `cui: truncated N slices under V={budget}`. With 35 checks, `V=32` would drop the last three alphabetical ids and never run them. `defaults.budget` and `.env.example` are 48. `resolve_budget` prefers CLI, then `KUTHA_GOV_BUDGET`, then fsm default. This checkout has **no `.env`**, so `ci` uses 48. A leftover local `.env` still pinned at 32 would truncate; it was not present and was not committed.

`test_ci_quantum_reaches_ok_on_this_tree` passes `load_machine(ROOT).budget_default` (≥48) and asserts `outcome.skipped == 0` and `len(results) == len(get_checks(ROOT))`.

## Threat Flags

None beyond the plan register (T-11-08…T-11-10, T-11-SC). No new packages, endpoints, or crate schema.

## Next Phase Readiness

- 11-04 authors META kind rows (`cite_equals`, `refs_resolve`, `file_max_lines`), the AGENTS.md `selftest` command line (stay ≤110), and the SEM-08 Process narrative.
- Do not treat these checks as ADR Accepted or L_capability.

---
*Phase: 11-semantic-governor*
*Completed: 2026-09-30*

## Self-Check: PASSED

- `11-03-SUMMARY.md` exists
- Commits `3383240`, `53e03cb`, `84cd5ad` exist on `main`
