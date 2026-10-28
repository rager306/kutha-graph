# Phase 83-01 — Legal answer envelope summary

**Slice:** M025 S03 / GSD Phase 83  
**Status:** implemented

## Delivered

- `NormView` / `NormTriple`; `Runtime::norm_view(ValidTime, WallTime)`.
- Reuses `knowledge_tt_at`, `live_facts_at`, `derivation_eligible_at`, `conflict_report_at`.
- Oracles: `m025_norm_view.rs` (MissingAnchor, Abstain/retracted-premise, Ok, Unknown, Conflict).
- ADR-013 Clarification Phase 83; ADR-090 TR-06 Clarification; matrix `SEM-m025-norm-view`.

## Deferred

- Demo bin + real-text fixtures (S04); Incomplete outcome; pack thaw.
