---
phase: 39-policy-and-replay
plan: 01
subsystem: rules
tags: [policy, admission, execution-replay]
requires:
  - phase: 38-rule-evaluator
    provides: Horn eval and hashed inverse_knows pin
provides:
  - deny policy body fails check_admission
  - Horn execution replay in replay_check
affects: [kutha-runtime]
actuals:
  tokens: 4143
  tasks: 1
  commits: 1
plan_head_before: 68fe32cdf3d5b2696152166064f29b6d7a950795
plan_head_after: 59e52d909a3cdb63cd0e48a850a6cdf54ddaae78
tech-stack:
  added: []
  patterns: [UTF-8 policy blob deny; Horn reproducible at ingested_at]
key-files:
  created:
    - crates/kutha-runtime/tests/m016_policy_replay.rs
  modified:
    - crates/kutha-runtime/src/admission.rs
    - crates/kutha-runtime/src/eval.rs
    - crates/kutha-runtime/src/provenance.rs
    - CHANGELOG.md
key-decisions:
  - "D-01: trimmed live policy body deny denies with reason policy"
  - "D-02: empty-pin Horn Behavior fails replay_check; fingerprint stays facts-only"
requirements-completed: [RULE-03, RULE-04]
duration: 12min
completed: 2026-10-02
status: complete
---

# Phase 39 Plan 01: Policy eval and execution replay Summary

Live `deny` blob fails `check_admission`; `leased-policy` still admits. Horn-pinned Behaviors must re-derive at `ingested_at`.

## Deviations from Plan

**1. [Rule 1 - Bug] Missing non-empty pin is Named skip**
- **Found during:** `kutha-gov ci` observe_cargo
- **Issue:** `horn_behavior_reproducible` treated any missing blob as fail, so `provenance_detects_rule_version_change_when_state_fingerprint_matches` (pin → `r2`) failed `replay_check`.
- **Fix:** empty pin still fails closed; missing non-empty blob skips (Named / provenance oracle). Unmatched Horn with a live blob still fails.
- **Files:** eval.rs
- **Commit:** (this wave)

## Self-Check: PASSED
