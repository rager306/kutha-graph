---
phase: "248"
name: "fix-or-accept"
created: 2026-10-29
---

# Phase 248: Fix or accept — Context

## Decisions

1. Fix the per-fact HashSet probe by replacing `dictionary_rels: HashSet<TermId>` with a dense `Vec<bool>` indexed by TermId.
2. Keep the product filter semantics; only the lookup changes.
3. Measure post-fix at N=10000; 1.35x old is acceptable if stable, 3.3x is not.
4. No ADR or Op change.

## Discretion Areas

If the vec is not enough, consider a product-only `vt_by_from` index, but do not widen the slice.

## Deferred Ideas

Ingest delta and B3 remain later work.
