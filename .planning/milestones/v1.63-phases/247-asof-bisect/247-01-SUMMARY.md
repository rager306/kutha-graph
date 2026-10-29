---
phase: 247-asof-bisect
plan: 01
subsystem: benchmark
tags: [as-of, bisect, regression]
requires: []
provides:
  - "first bad commit cec5b64"
  - "code cause: per-fact dictionary HashSet"
affects: [248-fix-or-accept]
actuals: {tokens: 900, tasks: 4, commits: 0}
tech-stack: {added: [], patterns: [n2000-bisect]}
key-files:
  created: []
  modified: [docs/architecture/benchmark-baseline.md, CHANGELOG.md]
key-decisions:
  - "Use stable 2N/4N at N=2000; the single .n is a noisy first call."
  - "cec5b64 is the first bad commit."
requirements-completed: [PF-03]
coverage:
  - id: D1
    description: "First bad commit for the as_of step."
    requirement: PF-03
    verification:
      - kind: manual
        ref: "docs/architecture/benchmark-baseline.md#2026-10-29-m169"
        status: pass
    human_judgment: false
---

# Phase 247 summary — as_of bisect

- Parent 34.2/68.7 µs; cec5b64 129.2/212.1 µs at N=2000.
- First bad commit cec5b64 added `LiveFactFilter::Product` with a per-fact HashSet probe.
- Head pre-fix 112/220 µs; no full git bisect needed.
