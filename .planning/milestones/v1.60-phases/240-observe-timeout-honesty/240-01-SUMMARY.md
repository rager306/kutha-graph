---
phase: 240-observe-timeout-honesty
plan: 01
subsystem: harness
tags: [observe, retry, timeout, pytest]
requires: []
provides:
  - "run_cargo_observation stops retrying after 124/127"
  - "a regression with KUTHA_GOV_CARGO_OBSERVE_RETRIES=2"
affects: [241-ci-gate-truth]
actuals:
  tokens: 900
  tasks: 5
  commits: 0
tech-stack:
  added: []
  patterns: [timeout-does-not-retry]
key-files:
  created: []
  modified:
    - scripts/kutha_gov/observe.py
    - scripts/tests/test_fsm.py
    - scripts/tests/test_kutha_gov.py
    - CHANGELOG.md
key-decisions:
  - "124/127 break before the incomplete-retry branch; build skip already handled 124/127."
  - "Partial-output retry remains unchanged."
requirements-completed: [OP-01]
coverage:
  - id: D1
    description: "A cargo timeout or missing binary makes one cargo call and skips the build."
    requirement: OP-01
    verification:
      - kind: unit
        ref: "scripts/tests/test_fsm.py#ObserveBuildTests.test_observe_skips_build_after_timeout"
        status: pass
      - kind: unit
        ref: "scripts/tests/test_fsm.py#ObserveBuildTests.test_observe_timeout_does_not_retry_with_retries_env"
        status: pass
    human_judgment: false
---

# Phase 240 summary — observe timeout honesty

- Added `if code in {124, 127}: break` after findings are assigned in `run_cargo_observation`.
- Existing timeout test now sees one cargo call; added a retries-env regression.
- Independent verifier ran `ObserveBuildTests` and an extra attack with exit 0.
- Parent full pytest: 296 passed, 2 skipped, 21 subtests, 0 failed (326 s). The only red was `test_agents_lean_live_precommit_is_green` still asserting 110 lines; updated to the live 160 bound.
- No product code, no Op, no thaw.
