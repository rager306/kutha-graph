---
phase: 251-b3-workload-and-baseline
plan: 01
subsystem: benchmark
tags: [b3, hop, workload, baseline]
requires: []
provides:
  - "fixed B3 layered relatedTo workload"
  - "Kutha hop2 baseline and ratio floor"
affects: [252-b3-decision]
actuals: {tokens: 0, tasks: 4, commits: 0}
tech-stack: {added: [], patterns: [hop_k-layered-workload, taskset-four-run]}
key-files:
  created:
    - .compound-engineering/artifacts/research/2026-10-30-b3-workload.md
    - .compound-engineering/artifacts/research/2026-10-30-b3-hop-logs/
  modified: [CHANGELOG.md]
key-decisions:
  - "Use the existing hop_k typed-IR workload, no parser."
  - "Use taskset -c 0 and four runs; median ratios: 2N/N 2.074, 4N/N 4.189."
requirements-completed: [B3-01]
coverage:
  - id: D1
    description: "Fixed workload and same-host Kutha baseline."
    requirement: B3-01
    verification:
      - kind: manual
        ref: ".compound-engineering/artifacts/research/2026-10-30-b3-workload.md"
        status: pass
    human_judgment: false
---

# Phase 251 summary — B3 workload and baseline

Four `taskset -c 0` runs at KUTHA_HOP_N=256: medians 170,568 ns (n), 353,765 ns (2n), 714,556 ns (4n). Median ratios: 2N/N 2.074 (floor 2.28), 4N/N 4.189 (floor 4.61), 4N/2N 2.020 (floor 2.22). Workload is the existing layered `relatedTo`/typed-IR `hop_k`; no parser, HNSW, planner, Cypher, or legal pack. Raw logs are archived under `.compound-engineering/artifacts/research/2026-10-30-b3-hop-logs/`; `KUTHA_HOP_N=256 cargo bench -p kutha-runtime --bench hop_k` exit 0, `kutha-gov precommit` exit 0. Phase 252 runs the competitor comparison or records the blocker.
