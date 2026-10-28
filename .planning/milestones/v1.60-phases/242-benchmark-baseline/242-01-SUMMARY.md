---
phase: 242-benchmark-baseline
plan: 01
subsystem: harness
tags: [benchmark, baseline, regression]
requires:
  - phase: 241-ci-gate-truth
    provides: "truthful CI and pytest"
provides:
  - "2026-10-28 same-host baseline"
  - "explicit regression residuals"
affects: [next-product-milestone]
actuals:
  tokens: 600
  tasks: 4
  commits: 0
tech-stack:
  added: []
  patterns: [bench-alone, record-regression]
key-files:
  created: []
  modified:
    - docs/architecture/benchmark-baseline.md
    - CHANGELOG.md
key-decisions:
  - "Discarded the first sample: full pytest ran concurrently and inflated it."
  - "The fresh numbers are a regression reference, not the B3 competitor gate."
requirements-completed: [OP-03]
coverage:
  - id: D1
    description: "A fresh N=10000 baseline with gate keys and ratios is recorded."
    requirement: OP-03
    verification:
      - kind: other
        ref: "tmp/bench-m166-clean.log"
        status: pass
    human_judgment: false
---

# Phase 242 summary — benchmark baseline

- Ran the baseline alone after stopping concurrent pytest/verifier work.
- Recorded `ingest_plain.n` 115,733,914 ns and `as_of.n` 334,032 ns in `docs/architecture/benchmark-baseline.md`.
- `bytes_per_fact` is 0 B (RSS sampling failed); old 13,674 B is not comparable.
- Against 2026-10-01: ingest +21.3%, as_of 4.0x, persist -58.7%. Follow-up RED, not B3.
- No product code, no Op.
