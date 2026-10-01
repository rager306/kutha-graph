---
phase: 24-fold-indexes-for-ingest
plan: 02
status: complete
---

# Phase 24 Plan 02 Summary

## Done

- Wired D-03 sites to indexes; `green_ingest_visits_constant_in_n` passes.
- rg arbiter: D-03 `facts().iter().any/find` and `log.iter().find` removed from production paths (one `assert!` remains in unit test).
- rg count quantum.rs scan sites: 14 → 5 (remaining hydrate/graph_len full scans, intentional).
- Bench delta recorded; **D-06 absolute ingest ≤10% not met** under Eager (+17…+28%).

## Controller closeout (2026-10-01)

Accepted **both** policies as config (not a forced single path):

- **Eager** (default / `KUTHA_FOLD_INDEX_MODE=eager`): accept write-cost; visit-oracle gate.
- **Lazy** (`lazy`|`thin`): thin write / lazy lease; `ensure_fold_indexes` / scan.

Shipped: `FoldIndexMode`, Runtime APIs, `.env.example` + local `.env`, `m013_fold_index_mode`, bench `fold_index_mode` metric. See interim note and `benchmark-baseline.md` § Fold index mode.

## Open (out of S03)

Materializer / CSR delta lease remains honeycomb Proposed (ADR-040). Do not unfreeze M002.
