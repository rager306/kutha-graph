# Phase 96 CONTEXT — VT-aware Horn dedup

**Source:** CE full-branch review P1 #11.  
**Lease:** M036 S01 / GSD v0.30.

## Goal

`head_event` duplicate suppression must require VT overlap with a live residual, so disjoint VT windows of the same triple each mint a derived fact.

## Oracles

- `disjoint_vt_windows_mint_separate_derived_inverses`
- `overlapping_vt_windows_still_suppress_duplicate_derived`
