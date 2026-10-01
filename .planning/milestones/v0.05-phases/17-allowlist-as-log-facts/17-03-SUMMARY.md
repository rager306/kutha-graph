---
phase: 17-allowlist-as-log-facts
plan: 03
subsystem: harness
tags: [governor, fsm, allowlist, ALL-01, ALL-02, ALL-03]

requires:
  - phase: 17-allowlist-as-log-facts
    provides: named ALL oracles in m012_allowlist_facts.rs
provides:
  - "fsm.yaml observe_cargo.required names for ALL-01..03"
  - "m012-s01-allowlist-facts rust_test_asserts + B-m012-s01"
  - "ADR-011 evidence includes ALL names; ADR-050 stays Proposed/frozen"
affects: [verify-work, kutha-gov-ci]

actuals:
  tokens: 1517
  tasks: 2
  commits: 1
  plan_head_before: fa34146051f3305881e82ab89102a7dc501a51e9
  plan_head_after: 0d9c11c439007d5c011e4747002d866b813f8882

tech-stack:
  added: []
  patterns:
    - "Governor intake: bridges.yaml + checks.yaml rust_test_asserts + fsm observe names; honeycomb evidence append only"

key-files:
  created: []
  modified:
    - .kutha/dictionaries/fsm.yaml
    - .kutha/dictionaries/checks.yaml
    - .kutha/dictionaries/bridges.yaml
    - .kutha/dictionaries/honeycomb.yaml
    - CHANGELOG.md

key-decisions:
  - "ALL file oracles are required observe names; FF6 remains required"
  - "ADR-011 evidence append only; honeycomb map stays Proposed; ADR-050 delivery stays frozen"

patterns-established:
  - "M012 slice governor rows use category m012-s01 and bridge B-m012-s01"

requirements-completed: [ALL-01, ALL-02, ALL-03]

coverage:
  - id: D1
    description: "Three ALL file tests plus FF6 are required observe names; rust_test_asserts cites m012_allowlist_facts.rs"
    requirement: ALL-01
    verification:
      - kind: other
        ref: uv run kutha-gov ci
        status: pass
    human_judgment: false
  - id: D2
    description: "admit_consults_fold_allowlist_not_yaml_alone is observed (ALL-02)"
    requirement: ALL-02
    verification:
      - kind: other
        ref: uv run kutha-gov precommit --check observe-required-fn
        status: pass
    human_judgment: false
  - id: D3
    description: "kutha-gov ci 0 HIGH; FF5/FF6 green; honeycomb Proposed; ADR-050 frozen"
    requirement: ALL-03
    verification:
      - kind: other
        ref: uv run kutha-gov ci
        status: pass
    human_judgment: false

duration: 4min
completed: 2026-10-01
status: complete
---

# Phase 17 Plan 03: Governor observation Summary

**Governor observes ALL-01..03 named cargo tests; ci stays 0 HIGH; honeycomb remains Proposed**

## Performance

- **Duration:** 4 min
- **Started:** 2026-10-01T01:51:46Z
- **Completed:** 2026-10-01T01:56:15Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- `observe_cargo.required` lists `allow_relation_appends_as_log_fact`, `retract_allow_relation_drops_admit_at_later_cut`, `admit_consults_fold_allowlist_not_yaml_alone`. FF6 and LOG/REF/ING/DUR/TIME/HOT names remain.
- `checks.yaml` `m012-s01-allowlist-facts` + `bridges.yaml` `B-m012-s01` cite `m012_allowlist_facts.rs`.
- ADR-011 evidence includes the three ALL names. ADR-050 map Proposed, delivery frozen, `freeze_as` unchanged.
- `uv run kutha-gov ci`: 0 HIGH, 0 LOW, 42 checks, `h5_selftest=42/42`.

## Task Commits

1. **Task 1: Confirm the three ALL oracles still pass** - no commit (read-only cargo confirmation)
2. **Task 2: Register ALL oracles with the governor and keep freeze/Proposed** - `0d9c11c` (feat)

**Plan metadata:** pending docs commit

## Files Created/Modified

- `.kutha/dictionaries/fsm.yaml` - three ALL observe names
- `.kutha/dictionaries/checks.yaml` - `m012-s01-allowlist-facts`; state-readme selftest mutation retargeted to `M012`
- `.kutha/dictionaries/bridges.yaml` - `B-m012-s01`
- `.kutha/dictionaries/honeycomb.yaml` - ADR-011 evidence append
- `CHANGELOG.md` - Process plane governor registration

## Decisions Made

- Evidence append on ADR-011 only; no cell restaged to Accepted.
- Do not put the check id on `invariants.yaml` (bridge ledger only).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] state-readme selftest still mutated M012a**
- **Found during:** Task 2 (`uv run kutha-gov ci`)
- **Issue:** After the M012 lease, README shows `M012`. Selftest `replace_regex` of `M012a` was a no-op, so `state-readme` was VACUOUS (1 HIGH).
- **Fix:** Mutation pattern is now `M012` → `M999`.
- **Files modified:** `.kutha/dictionaries/checks.yaml`
- **Verification:** `kutha-gov ci` 0 HIGH, `h5_selftest=42/42`
- **Committed in:** `0d9c11c` (Task 2)

---

**Total deviations:** 1 auto-fixed (blocking)
**Impact on plan:** Required for success criterion 4 (ci 0 HIGH). No product-scope creep. `.kutha/STATE.md` lease fields untouched.

## Issues Encountered

Task 1 had nothing to commit: the three ALL oracles and M012a files were already green from 17-01/17-02.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 17 execute complete. Parent may close S01 after this verification; this plan does not edit `.kutha/STATE.md`.
- Phase 18 (rule registry) needs Active Slice S02. Do not start ADR-050 six dictionaries.

## Self-Check: PASSED

- FOUND: .kutha/dictionaries/fsm.yaml
- FOUND: .kutha/dictionaries/checks.yaml
- FOUND: .kutha/dictionaries/bridges.yaml
- FOUND: .kutha/dictionaries/honeycomb.yaml
- FOUND: CHANGELOG.md
- FOUND: 0d9c11c

---
*Phase: 17-allowlist-as-log-facts*
*Completed: 2026-10-01*
