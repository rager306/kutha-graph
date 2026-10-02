---
phase: 24-fold-indexes-for-ingest
plan: 01
status: complete
---

# Phase 24 Plan 01 Summary

## Done

- Added ingest examine counter; lease indexes on GraphFold (`fact_by_event`, `live_triples`, `claim_known`) and Runtime (`by_event_id`, `by_delivery_key`).
- Tool pair before: rg=14 in quantum.rs; GitNexus impact `delivery_key_retry` upstream direct=1. Indexes rebuilt on hydrate/fork/from_*.

## Note

RED visit growth was designed into the counter; green oracle lands in 24-02 after wiring (same change set in this wave).

## Commits

(see 24-02 for product commit grouping)
