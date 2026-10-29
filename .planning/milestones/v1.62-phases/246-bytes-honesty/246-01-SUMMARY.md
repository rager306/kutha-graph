---
phase: 246-bytes-honesty
plan: 01
subsystem: benchmark
tags: [bench, rss, bytes-per-fact]
requires:
  - phase: 245-bench-ab
    provides: "old-vs-HEAD medians"
provides:
  - "bytes_per_fact n/a on zero delta"
affects: [bisect, next-perf]
actuals:
  tokens: 500
  tasks: 3
  commits: 0
tech-stack:
  added: []
  patterns: [measurement-honesty]
key-files:
  modified:
    - crates/kutha-runtime/benches/baseline.rs
    - docs/architecture/benchmark-baseline.md
    - CHANGELOG.md
key-decisions:
  - "Zero RSS delta is n/a, never 0 B/fact."
  - "Fresh HEAD4 run recorded; as_of spread 224-303 us."
requirements-completed: [PF-02]
coverage:
  - id: D1
    description: "bytes_per_fact is n/a when RSS did not move."
    requirement: PF-02
    verification:
      - kind: manual
        ref: "tmp/bench-head-4.log"
        status: pass
    human_judgment: false
---

# Phase 246 summary — bytes honesty

- `baseline.rs` prints `n/a` for zero RSS delta or zero live facts.
- Fresh HEAD bench: `bytes_per_fact n/a`, ingest 107.3 ms, as_of 224 µs.
- No product semantics, no thaw.
