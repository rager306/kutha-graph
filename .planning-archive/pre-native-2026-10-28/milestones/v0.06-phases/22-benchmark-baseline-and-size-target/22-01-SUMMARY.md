---
phase: 22-benchmark-baseline-and-size-target
plan: 01
status: complete
---

# Phase 22 Plan 01 Summary

## Done

- Added `crates/kutha-runtime/benches/baseline.rs` with `[[bench]] harness = false`.
- Workload: `relatedTo` only (no `knows` cascade). Validates live_count, fingerprint after open, claim_supported_at, late retract.
- Metrics: 27 sorted `key value unit` lines at N/2N/4N; `KUTHA_BENCH_SABOTAGE=drop_one` exits non-zero.
- D-07 base recorded in `tmp/p22-base.txt` before edits. No `src/` changes. No new Cargo deps.

## Commits

- `feat(22-01): add relatedTo baseline bench path`
- `feat(22-01): measure three sizes and remaining baseline ops`
- `feat(22-01): fail-closed bench sabotage`

## Verify

- `KUTHA_BENCH_N=20` → `validated yes`, 27 keys match, two-run key identity, sabotage exit ≠ 0.

## Tools

- CBM coverage on quantum/store/fold: `metadata_match`. GitNexus `impact` on `persist` (downstream, not edited). `rg` confirmed Assert/Retract API.
