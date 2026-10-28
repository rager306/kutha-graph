# Phase 93 CONTEXT — CI admission VT

**Source:** CE full-branch review P1 #8.  
**Lease:** M033 S01 / GSD v0.27.

## Goal

`check_admission_row` must not false-deny a justification that cites a CorrectInterval-split `event_id` when `row.vt` falls in a live patch/suffix residual.

## Oracles

- `admission_admits_when_cited_ci_residual_covers_patch_vt`
- `admission_still_denies_when_no_ci_residual_covers_vt`
