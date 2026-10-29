---
phase: 244-pit-probe
plan: 01
subsystem: legal-probe
tags: [ecfr, pit, as-of, known-on, norm-view, checkpoint]
requires:
  - phase: 243-ecfr-fixture
    provides: "9 eCFR text files"
provides:
  - "real-text PIT probe"
  - "persist/checkpoint parity"
affects: [next-legal-milestone]
actuals:
  tokens: 1400
  tasks: 6
  commits: 0
tech-stack:
  added: []
  patterns: [real-text-pit, tt-only-known-on]
key-files:
  created:
    - crates/kutha-runtime/tests/m167_ecfr_pit.rs
  modified:
    - CHANGELOG.md
key-decisions:
  - "Day-scale VT (YYYYMMDD) is probe-only; 2017-01-23 and 2017-03-14 must be distinct."
  - "as_known_on is TT-only and lists superseded editions; norm_view is force∩knowledge."
requirements-completed: [LP-02]
coverage:
  - id: D1
    description: "Real eCFR text answers correctly at three VT cuts and three knowledge cuts."
    requirement: LP-02
    verification:
      - kind: integration
        ref: "cargo test -p kutha-runtime --test m167_ecfr_pit"
        status: pass
    human_judgment: false
---

# Phase 244 summary — eCFR PIT probe

- Built the log from 9 fixture files with `YYYYMMDD` VT intervals and `TtAnchor` walls.
- `as_of(2016)`, `as_of(2017-02)`, `as_of(2018)` return the matching edition for 304.1/304.2/304.10.
- `as_known_on` is TT-only: later walls include superseded editions, early walls never show a future edition.
- `norm_view` does not expose a future edition at an early wall.
- Persist → `checkpoint_sealed_prefix` → open → `replay_check` parity passed.
- No new Op, no core schema change, no thaw.
