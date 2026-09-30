---
phase: 11-semantic-governor
plan: 02
subsystem: harness-rust-test-asserts
tags: [SEM-02, SEM-04, D-T1, D-T2, D-E1, rust_test_asserts, names_from_yaml]

requires:
  - phase: 11-semantic-governor
    provides: kutha-gov selftest copy-once tempfile runner; derived mutations; FSM run_selftest
provides:
  - rust_test_asserts kind (brace-matched #[test] body, comments/strings/chars stripped)
  - All former fn-needle checks converted; unnamed-csr stays file_absent
  - names_from_yaml + require_evidence_when on honeycomb named cells
  - Live conversion: no unresolved or vacuous named tests
affects: [11-03 cite_equals, 11-04 META kind row and SEM-08 narrative]

estimate:
  tokens: 38000
  tasks: 3

actuals:
  tokens: 11734
  tasks: 3
  commits: 3

plan_head_before: 1c8b3e770f2ba6e86fdef4700482d946ef40575c
plan_head_after: a59d55578b9352032e33c8a155d2ff984fb52e4a
commits: 3

tech-stack:
  added: []
  patterns:
    - Strip comments, strings (including raw), and char literals before brace-matching a #[test] body
    - names_from_yaml omits field when the YAML select is already a string list
    - Derived rust_test_asserts mutation inserts #[ignore] so multi-assert and helper-prefix bodies still HIGH

key-files:
  created:
    - scripts/kutha_gov/rust_source.py
    - .planning/phases/11-semantic-governor/11-02-SUMMARY.md
  modified:
    - scripts/kutha_gov/kinds.py
    - scripts/kutha_gov/selftest.py
    - scripts/tests/test_kutha_gov.py
    - .kutha/dictionaries/checks.yaml
    - .kutha/dictionaries/invariants.yaml
    - .kutha/dictionaries/bridges.yaml
    - CHANGELOG.md

key-decisions:
  - "D-T1: #[test] without #[ignore]; reject async and nested fn; helpers with prefix assert_ count"
  - "D-T2: convert fn needles; keep source-symbol file_contains; do not edit crates/"
  - "D-E1: require_evidence_when capability=named; honeycomb evidence names already matched live tests"
  - "Derived mutation uses #[ignore] insertion (D-T1 alternative) because rewriting one assert!( leaves sibling macros and assert_* helpers"

patterns-established:
  - "A named Rust test is honest only if the brace-matched body still asserts after noise strip"
  - "capability named with empty evidence is HIGH even when other cells still list tests"

requirements-completed: [SEM-02, SEM-04]

coverage:
  - id: D1
    description: rust_test_asserts proves named #[test] bodies assert after comment/string/char strip
    requirement: SEM-02
    verification:
      - kind: unit
        ref: scripts/tests/test_kutha_gov.py#RustTestAssertsTests.test_rust_test_asserts_high_when_assert_only_in_comment_or_string
        status: pass
      - kind: unit
        ref: scripts/tests/test_kutha_gov.py#RustTestAssertsTests.test_rust_test_asserts_brace_inside_string_does_not_break_body
        status: pass
    human_judgment: false
  - id: D2
    description: Former fn-needle checks and honeycomb named evidence use rust_test_asserts
    requirement: SEM-02
    verification:
      - kind: other
        ref: uv run kutha-gov precommit --check observe-required-fn / honeycomb-ledger / m010-semantic-open / m011-e2e
        status: pass
    human_judgment: false
  - id: D3
    description: names_from_yaml plus require_evidence_when HIGH on empty named evidence and missing fn
    requirement: SEM-04
    verification:
      - kind: unit
        ref: scripts/tests/test_kutha_gov.py#RustTestAssertsTests.test_rust_test_names_from_yaml_empty_named_evidence_is_high
        status: pass
      - kind: unit
        ref: scripts/tests/test_kutha_gov.py#RustTestAssertsTests.test_rust_test_names_from_yaml_high_when_evidence_fn_missing
        status: pass
    human_judgment: false

duration: 93min
completed: 2026-09-30
status: complete
---

# Phase 11 Plan 02: rust_test_asserts Summary

**`rust_test_asserts` proves named `#[test]` bodies assert after stripping comments/strings/chars; every former `fn` needle now uses that kind; honeycomb `capability: named` empty evidence is HIGH**

## Performance

- **Duration:** 93 min
- **Started:** 2026-09-30T07:57:10Z
- **Completed:** 2026-09-30T09:30:25Z
- **Tasks:** 3
- **Files modified:** 8

## Accomplishments

- `scripts/kutha_gov/rust_source.py` blanks line/block comments, regular/raw/byte strings, and char literals (including `'{'`, `'"'`, `'\u{7b}'`) before brace-matching. Nested `fn` and `async fn` are rejected. `#[cfg(...)]` plus `#[test]` is accepted; `#[ignore]` is HIGH.
- `ALLOWED_KINDS` / `RUNNERS` gained `rust_test_asserts`. Optional `names_from_yaml`, `require_symbols`, `assert_prefixes` (default `assert_`), `require_evidence_when`.
- Converted check ids: `h4-membership-as-of`, `m010-semantic-open` (fn steps only), `m011-claim-supports` (fn steps only), `m011-partial-correction`, `m011-quantum-outcome`, `m011-typed-csr`, `m011-provenance`, `m011-e2e`, `observe-required-fn`, `honeycomb-ledger` (fn-prefix glob step only). `unnamed-csr` stays `file_absent` on `pub fn csr_lease(`.
- Live inventory of FSM required names, honeycomb evidence, and converted `tests:` lists: **0 unresolved, 0 vacuous**. `honeycomb.yaml` evidence spelling was already correct — no rename.

## Converted checks

| Check id | Kind now | Names |
|----------|----------|--------|
| h4-membership-as-of | rust_test_asserts path+tests | `h4_prior_cut_keeps_status_membership_after_later_edition_drops_it` |
| m010-semantic-open | rust_test_asserts + leftover source `file_contains` | 3 intern/open tests |
| m011-claim-supports | rust_test_asserts + leftover source `file_contains` | 4 claim tests |
| m011-partial-correction | rust_test_asserts + leftover source `file_contains` | 2 correction tests |
| m011-quantum-outcome | rust_test_asserts | 2 outcome tests |
| m011-typed-csr | rust_test_asserts + leftover source `file_contains` | 2 CSR tests |
| m011-provenance | rust_test_asserts + leftover source `file_contains` | 2 provenance tests |
| m011-e2e | rust_test_asserts + leftover source `file_contains` | 3 e2e tests |
| observe-required-fn | names_from_yaml on `fsm.yaml` `states.observe_cargo.required` | FSM required list |
| honeycomb-ledger | names_from_yaml on `cells.evidence` + `require_evidence_when` capability named | all evidence strings |

## Findings (unresolved / vacuous named tests)

**None on the live tree.** Every converted name and every honeycomb evidence string resolved to a `#[test]` (not `#[ignore]`, not `async`, not nested) whose stripped body contains `assert!` / `assert_eq!` / `assert_ne!` / `debug_assert*!` or an `assert_*` helper. No `honeycomb.yaml` evidence rename. No `crates/` edit.

Representative file:line (first qualifying item): `h4_prior_cut_keeps_status_membership_after_later_edition_drops_it` at `crates/kutha-runtime/tests/h4_process_allows.rs:17`; `ff5_as_of_t1_differs_from_as_of_t2_on_statute_log` at `crates/kutha-runtime/tests/ff5_legal_pit.rs:14`; `e2e_fixture_supports_and_conflict_at_named_cuts` at `crates/kutha-runtime/tests/m011_e2e_fixture.rs:173`.

## Task Commits

1. **Task 1: End-to-end rust_test_asserts on h4-membership-as-of** - `08162ee` (feat)
2. **Task 2: Convert remaining fn-needle checks and resolve honeycomb evidence** - `c878c05` (feat)
3. **Rule 1 fix: derived mutation via #[ignore]** - `a59d555` (fix)
4. **Task 3: Wave-close Trajectory** - this SUMMARY (docs)

## Files Created/Modified

- `scripts/kutha_gov/rust_source.py` - noise strip + fn/attr/body parse
- `scripts/kutha_gov/kinds.py` - `rust_test_asserts` runner
- `scripts/kutha_gov/selftest.py` - derived `#[ignore]` mutation + `require_evidence_when` evidence-empty mutation
- `.kutha/dictionaries/checks.yaml` - converted steps
- `.kutha/dictionaries/invariants.yaml` / `bridges.yaml` - honest claim updates (not fabricated I-F rows)
- `scripts/tests/test_kutha_gov.py` - tempfile red paths
- `CHANGELOG.md` - Process pointer; SEM-08 narrative reserved

## Decisions Made

Followed 11-CONTEXT.md D-T1, D-T2, D-E1, D-G1, D-G2, D-G3. META.md Allowed kinds row remains plan 11-04. No crates/ or `.kutha/STATE.md` edits. Code-graph tools were not queried.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] First-assert rewrite left the check green**
- **Found during:** Task 3 live `selftest` (`m011-e2e`, `m011-partial-correction`)
- **Issue:** Replacing `assert…!(` openings in one body left sibling macros and/or `assert_*` helpers, so the check stayed HIGH-free (VACUOUS).
- **Fix:** Derived mutation inserts `#[ignore]` above the named `fn` (D-T1 alternative). `require_evidence_when` still empties a named cell's flow `evidence: […]`.
- **Files modified:** `scripts/kutha_gov/selftest.py`
- **Verification:** `OK m011-e2e mutations=3`; `OK m011-partial-correction mutations=3`; live selftest 31 OK / 1 SKIP
- **Commit:** `a59d555`

**2. [Rule 1 - Bug] require_evidence_when mutation was a no-op**
- **Found during:** Task 2 live `selftest --check honeycomb-ledger`
- **Issue:** DOTALL plus unanchored `capability: named` matched a later `evidence: []`, replacement unchanged.
- **Fix:** `(?m)^` + indent; require nonempty `[^\]]+` list.
- **Files modified:** `scripts/kutha_gov/selftest.py`
- **Verification:** `OK honeycomb-ledger mutations=5`
- **Commit:** `c878c05` (regex) / confirmed after `a59d555`

**Total deviations:** 2 auto-fixed
**Impact on plan:** Kind still fails closed on vacuous bodies; selftest proves HIGH without crate edits.

## Issues Encountered

`workflow.tdd_mode` is false; plan type is `execute`. RED/GREEN were not split into `test(11-02)` / `feat(11-02)` commits. Committed on `main` under `git.branching_strategy: none` (#3552 / #3819 warning). No `--no-verify`. `RUSTC_WRAPPER=` for cargo observe.

## Trajectory (D-10 / D-G1)

1. **Commands:** `uv run kutha-gov selftest`; `uv run kutha-gov ci`; `uv run kutha-gov explain trajectory`.
2. **Outcome:** selftest exit 0 (**31 OK**, **1 SKIP** `docs-coupling`, 0 UNPROVEN, 0 VACUOUS, 0 BASELINE-FAIL). `ci` exit 0; **HIGH 0**, **LOW 0**; evidence `h5_selftest=32/32`; harness line **H5 dogfood**. `uv run kutha-gov precommit` exit 0. `uv run pytest -q scripts/tests` — 72 passed. `uv run kutha-gov py` — PYTHON_TOOLING_GREEN.
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

Cited: D-T1 (body asserts), D-T2 (no crate edits), D-E1 (named evidence), D-G3 (green ≠ Accepted). Docs/Python only: codebase-memory-mcp was not queried; this wave does not claim graph verification. `git diff --exit-code -- crates .kutha/STATE.md` is clean.

## Vacuous checks

Live-repo selftest found **no** vacuous YAML row after the `#[ignore]` derived mutation. Synthetic pytest still reports HIGH for missing fn, `#[ignore]`, no-assert, comment/string-only assert, missing symbol, empty named evidence, and missing evidence fn.

Skip reasons (not vacuous): `docs-coupling` depends on the live git diff.

## Threat Flags

None beyond the plan register (T-11-05…T-11-07, T-11-SC). No new packages, endpoints, or crate schema.

## Next Phase Readiness

- 11-03 can add `cite_equals` without touching `rust_test_asserts`.
- 11-04 authors META `rust_test_asserts` kind row and the SEM-08 Process narrative.

---
*Phase: 11-semantic-governor*
*Completed: 2026-09-30*
