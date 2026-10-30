---
phase: 249-ingest-bisect
plan: 01
subsystem: benchmark
tags: [ingest, bisect, regression]
requires: []
provides:
  - "first bad commit 199807d"
  - "cause: append_indexed by_event_id/by_delivery_key"
affects: [250-ingest-fix-or-accept]
actuals: {tokens: 0, tasks: 5, commits: 0}
tech-stack: {added: [], patterns: [ingest-only-bisect]}
key-files:
  created: []
  modified: [docs/architecture/benchmark-baseline.md, CHANGELOG.md]
key-decisions:
  - "Use five interleaved N=10000 samples; report min and median."
  - "199807d is first bad; current HEAD is lower than 199807d because 027a2ce adds reserve_ingest_indexes."
requirements-completed: [PF-05]
coverage:
  - id: D1
    description: "First bad commit for ingest_plain.n."
    requirement: PF-05
    verification:
      - kind: manual
        ref: "docs/architecture/benchmark-baseline.md#2026-10-30-m171-ingest-bisect"
        status: pass
    human_judgment: false
---

# Phase 249 summary — ingest bisect

- Five interleaved N=10000 samples, `taskset -c 0`, std-only `baseline` with `KUTHA_BENCH_INGEST_ONLY` early exit after the `ingest_plain` timing block.
- Medians: `cdf1942` 96,346,464; parent `c26bf35` 93,452,813; first bad `199807d` 130,445,862; HEAD `929bbc6` 114,143,499 ns.
- Parent -> bad +39.6%; old -> HEAD +18.5%. All 20 samples are in `tmp/ingest-only-*.log`.
- First bad `199807d` adds `Runtime::append_indexed` (`by_event_id`, `by_delivery_key`) and GraphFold indexes. The Phase 24 note records +16.9%/+28.5%; later `027a2ce` capacity reserve lowers HEAD but does not clear D-06.
- No product code changed. Phase 250 accepts the tradeoff.
