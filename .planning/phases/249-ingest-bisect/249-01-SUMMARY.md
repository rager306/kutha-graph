---
phase: 249-ingest-bisect
plan: 01
status: blocked
subsystem: benchmark
tags: [ingest, bisect, regression]
requires: []
provides:
  - "first candidate 199807d"
affects: [250-ingest-fix-or-accept]
actuals: {tokens: 0, tasks: 5, commits: 0}
tech-stack: {added: [], patterns: [ingest-only-bisect]}
key-files:
  created: [.planning/phases/249-ingest-bisect/249-01-PLAN.md]
  modified: []
key-decisions:
  - "Record 199807d as the first index commit, but do not call phase 249 passed on n=2."
  - "Phase 250 should accept the known per-assert map cost; no product patch from a noisy scalar."
requirements-completed: []
coverage: []
---

# Phase 249 summary — ingest bisect (blocked)

- `git bisect` on the ingest-only probe found `199807d feat(24): fold ingest indexes with constant visit oracle` as the first candidate.
- The commit adds `Runtime::append_indexed` / `by_event_id` / `by_delivery_key` and `GraphFold` fact indexes; its own message and the Phase 24 note record +16.9% / +28.5% (and +17..28%) versus Phase 22.
- Independent verifier #3 returned **BLOCKED**: the confirming full-bench evidence is n=2 with a 1.68x outlier (parent/bad medians 120.3/120.1 ms; mins 89.7/109.6 ms), the required `249` artifact table is absent, and the ingest-only probe was untracked and used min instead of median.
- Current controlled ingest-only data: cdf1942 min 100.7 ms, HEAD min 102.3 ms — not a reliable gap; full bench is load-sensitive on this host.
- No product code changed in v1.64; M171-S01 remains open.

## Unblock

Record a >=5-sample interleaved N=10000 table for `cdf1942`, `c26bf35`, `199807d`, `929bbc6` on an idle host; report min and median; then update `docs/architecture/benchmark-baseline.md` and CHANGELOG. Phase 250 accepts the tradeoff unless the table shows a new, separable cost.
