---
phase: "242"
name: "benchmark-baseline"
created: 2026-10-28
---

# Phase 242: Benchmark baseline — Context

## Decisions

1. Use the existing `baseline` bench at `KUTHA_BENCH_N=10000` on this host; it is a regression baseline, not the B3 competitor gate.
2. Record `ingest_plain.n`, `as_of.n`, and the 2N/N, 4N/N ratios in `docs/architecture/benchmark-baseline.md` with the date and host note.
3. Do not compare absolute times across hosts; do not claim B3.
4. No product code, no thaw.

## Discretion Areas

Whether to include all metric keys or only the gate keys; include the gate keys and a pointer to the raw log.

## Deferred Ideas

The real B3 FalkorDB/Samyama benchmark remains a later milestone.
