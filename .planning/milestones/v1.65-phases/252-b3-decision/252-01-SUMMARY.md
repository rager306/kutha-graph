---
phase: 252-b3-decision
plan: 01
subsystem: benchmark
tags: [b3, competitor, decision]
requires: [251-01]
provides:
  - "B3 no-gap decision; B4/B5 skipped"
affects: []
actuals: {tokens: 0, tasks: 3, commits: 3}
tech-stack: {added: [], patterns: [falkordb, samyama, hop2]}
key-files:
  created:
    - .compound-engineering/artifacts/research/2026-10-30-b3-decision.md
    - .compound-engineering/artifacts/research/2026-10-30-b3-decision-runs.json
  modified: [CHANGELOG.md]
key-decisions:
  - "B4 (HNSW) and B5 (planner) are skipped: Kutha is ~29.5x faster than FalkorDB and ~89.6x faster than Samyama on this workload."
  - "B6 (Cypher AS OF) still needs M003; this note does not thaw it."
requirements-completed: [B3-02]
coverage:
  - id: D1
    description: "Competitor comparison and B4/B5 decision."
    requirement: B3-02
    verification:
      - kind: manual
        ref: ".compound-engineering/artifacts/research/2026-10-30-b3-decision.md"
        status: pass
    human_judgment: false
---

# Phase 252 summary — B3 decision

Kutha walk-only 161,912 ns; with `csr_lease.n_ns` 350,584 the prepared path is 512,496 ns. FalkorDB median 4,779 µs; Samyama 14,511 µs. Walk-only ratios ≈29.5×/89.6×; prepared ratios ≈9.3×/28.3× (≈3.1× vs Falkor internal 1.58 ms). Decision: **skip B4/B5**; B6 still requires M003. Phase-251 ratio floor is not evaluated by this single comparison; this run's 2N/N 2.292 is +0.5% over the provisional 2.28 cap and is recorded, not used as a decision. Caveats: different transports, output cardinality differs, one same-sitting Kutha run, no data-load time, no AS OF comparison. Artifacts under `.compound-engineering/artifacts/research/2026-10-30-b3-decision*`.
