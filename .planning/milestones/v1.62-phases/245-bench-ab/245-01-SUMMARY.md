---
phase: 245-bench-ab
plan: 01
subsystem: benchmark
tags: [bench, ab, as-of, regression]
requires: []
provides:
  - "same-host old-vs-HEAD medians"
  - "as_of regression confirmed"
affects: [246-bytes-honesty, bisect]
actuals:
  tokens: 1600
  tasks: 5
  commits: 0
tech-stack:
  added: []
  patterns: [interleaved-ab, worktree-baseline]
key-files:
  created: []
  modified:
    - docs/architecture/benchmark-baseline.md
    - CHANGELOG.md
key-decisions:
  - "Old cdf1942 reproduces the 2026-10-01 as_of and bytes_per_fact, so the gap is real."
  - "HEAD1 4.6 ms is an outlier; HEAD2/3 at 303/286 us are the stable shape."
requirements-completed: [PF-01]
coverage:
  - id: D1
    description: "Interleaved same-host medians for cdf1942 vs HEAD."
    requirement: PF-01
    verification:
      - kind: manual
        ref: "docs/architecture/benchmark-baseline.md#2026-10-29"
        status: pass
    human_judgment: false
---

# Phase 245 summary — bench A/B

- Old `cdf1942` in a detached worktree; old/HEAD interleaved.
- Old medians: ingest 113.24 ms, as_of 91.3 µs, bytes 13,682 B.
- HEAD medians: ingest 127.47 ms, as_of 303.4 µs, bytes 0.
- Ratios: ingest +12.6%, as_of 3.32x, open -3.0%, persist -67.0%, replay +1.1%.
- Old reproduces the 2026-10-01 as_of and bytes, so the as_of gap is real; next step is bisect.
- No product semantics, no thaw.
