---
phase: 05-persisted-quantum-outcome
plan: 01
subsystem: runtime-store
tags: [quantum, outcomes, sidecar, persist, resume, OUT-01, OUT-02]

requires:
  - phase: 04-partial-correction-with-residual-intervals
    provides: budget storm fixtures and GATE-01 registration pattern
provides:
  - quantum_outcomes.jsonl sidecar via store persist/open
  - OutcomeDisposition Zero/Partial/Full/Resume + PersistedQuantumOutcome
  - Runtime outcome buffer, record_resume, disposition helper
  - Named OUT-01 and OUT-02 integration tests
affects: [05-02 GATE-01 needles, ADR-014 evidence]

estimate:
  tokens: 48000
  tasks: 3

actuals:
  tokens: 9216
  tasks: 3
  commits: 3

plan_head_before: b849f89cfb21a49684423a253a8a7490bfeb4519
plan_head_after: 16f2dce6960c43f6fa8ec6c482b1e4aaf417c8f0
commits: 3

tech-stack:
  added: []
  patterns:
    - "Authoritative JSONL sidecar beside event log (not snapshot lease)"
    - "Disposition from aborted_on_budget × events_in_quantum"
    - "Explicit record_resume; open never invents Full or Resume"

key-files:
  created:
    - crates/kutha-runtime/tests/m011_quantum_outcome.rs
    - .planning/phases/05-persisted-quantum-outcome/05-01-SUMMARY.md
  modified:
    - crates/kutha-runtime/src/quantum.rs
    - crates/kutha-runtime/src/store.rs
    - crates/kutha-runtime/src/receipt.rs
    - crates/kutha-runtime/src/lib.rs
    - CHANGELOG.md

key-decisions:
  - "D-O1: quantum_outcomes.jsonl authoritative beside log; load on every open path"
  - "D-O2: Zero/Partial/Full from abort×count; Runtime::new(0) for Zero"
  - "D-O3: incomplete = absence of Full; resume only via record_resume"
  - "D-O4: thin DuplicateResume fail-closed; no ADR-062"
  - "D-O7: emit cascade math unchanged except post-Ok outcome recording"

patterns-established:
  - "Pattern: persist events/WAL before outcomes so mid-crash cannot orphan Full"
  - "Pattern: discard snapshot still keeps outcome history"

requirements-completed: [OUT-01, OUT-02]

coverage:
  - id: D1
    description: Budgets 0/1/2 readable as Zero/Partial/Full after persist→open without snapshot
    requirement: OUT-01
    verification:
      - kind: integration
        ref: crates/kutha-runtime/tests/m011_quantum_outcome.rs#budgets_0_1_2_distinguish_zero_partial_full_after_persist_open
        status: pass
    human_judgment: false
  - id: D2
    description: Crash after committed prefix shows no terminal Full until explicit record_resume
    requirement: OUT-02
    verification:
      - kind: integration
        ref: crates/kutha-runtime/tests/m011_quantum_outcome.rs#crash_after_prefix_has_no_terminal_success_until_explicit_resume
        status: pass
    human_judgment: false

duration: 4min
completed: 2026-09-29
status: complete
---

# Phase 05 Plan 01: Persisted quantum outcome Summary

**Sidecar `quantum_outcomes.jsonl` makes budgets 0/1/2 durable as Zero/Partial/Full and keeps crash-after-prefix from looking like terminal success until an explicit Resume row.**

## Performance

- **Duration:** ~4 min
- **Started:** 2026-09-29T16:45:49Z
- **Completed:** 2026-09-29T16:49:18Z
- **Tasks:** 3
- **Files modified:** 6 (+ SUMMARY)

## Accomplishments

- Runtime buffers `PersistedQuantumOutcome` on every successful `emit`; disposition maps abort×count to Zero/Partial/Full (D-O2).
- `store::persist` rewrites `quantum_outcomes.jsonl` after events; `store::open` loads the sidecar on every return path including snapshot early-return (D-O1).
- `Runtime::record_resume` appends Resume with fail-closed duplicate `resume_of`; open never auto-resumes (D-O3, D-O4).
- Named oracles OUT-01 and OUT-02 green; Correct/CorrectInterval cascade math untouched (D-O7).

## Task Commits

1. **Task 1: End-to-end budgets 0/1/2 Zero/Partial/Full after persist→open** - `de798c1` (feat)
2. **Task 2: Crash after prefix needs explicit resume** - `16f2dce` (test)
3. **Task 3: Product changelog + wave-close trajectory** - `d485b76` (docs)

## Trajectory (D-10 / D-O5)

1. **Commands:** `uv run kutha-gov ci`; `uv run kutha-gov explain trajectory`
2. **ci:** exit 0; **HIGH 0**, **LOW 0**, 28 checks (H4 dogfood). No WARN check ids to ledger.
3. **explain trajectory (paraphrase):** Active Milestone/Slice pointers must exist on ROADMAP; harness does not accept ADRs or claim product readiness; steps verify `.kutha/STATE.md` / `.kutha/ROADMAP.md` and pointer cross-checks.
4. **Orthogonality:** Green governor is not ADR Accepted and not L_capability. Active Slice remains S05; GATE-01 needle registration stays on plan 05-02 (D-O6). Decisions D-O1…D-O4, D-O7 honored.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing critical functionality] `record_resume` landed in tracer task**
- **Found during:** Task 1 (needed for buffer surface / RuntimeError::DuplicateResume)
- **Issue:** Plan placed `record_resume` on Task 2, but durable Resume API belongs with the outcome buffer introduced in Task 1.
- **Fix:** Implemented `record_resume` + `DuplicateResume` with Task 1; Task 2 added the named OUT-02 oracle only.
- **Files modified:** `crates/kutha-runtime/src/quantum.rs`
- **Commit:** `de798c1`

TDD RED for Task 2 was therefore not a separate failing commit — implementation already present from tracer; GREEN verified with exact named test.

## Threat Flags

None beyond plan register (T-05-01…T-05-05 mitigated: no invented Full, typed JSONL parse, events-before-outcomes write order, fail-closed duplicate resume, outcomes on snapshot open path).

## Known Stubs

None.

## Self-Check: PASSED

- `crates/kutha-runtime/tests/m011_quantum_outcome.rs` FOUND
- Named tests FOUND and passing
- Commits `de798c1`, `16f2dce` FOUND
- CHANGELOG 2026-09-29 + quantum_outcomes + both fn names FOUND
- `cargo test --workspace --offline` exit 0
- `uv run kutha-gov ci` HIGH 0
- Trajectory section includes ci / explain / Accepted-or-capability / HIGH|LOW|WARN
