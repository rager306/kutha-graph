# Phase 20 Context: Thin action record

**Milestone:** v0.05 / harness M012 S04
**Requirements:** ACT-01, ACT-02
**Depends on:** Phase 19 (admission/policy meta-facts)
**Source of truth for gaps:** `docs/architecture/semantic-gap-review.md` (F5 / ADR-051)

## Locked decisions

- **D-01:** A thin Action record (or Op) binds resolved arguments to the admission decision and policy version (ACT-01).
- **D-02:** Named test shows Action + admission AS OF a prior cut after a later policy change (ACT-02).
- **D-03:** Do not implement ADR-050 six dictionaries. Do not regress ALL/RULE/ADM or M012a oracles.
- **D-04:** Honeycomb stays Proposed. No Rocks/Cypher/HNSW/legal pack / WASM UDF sandbox.

## Spike anchors

- Phase 19 `Op::PinPolicy` / `Op::RecordAdmission` / `check_admission` on product path
- `docs/ADR/051` action record clarification (Proposed)

## Non-goals

- Multi-hop derivation (Phase 21)
- Six dictionaries / ADR-081 sandbox

## Harness

- Active Milestone M012, Active Slice **S04**, Phase H5, `L_delivery=M012-S03-done`
- Do not clear S04 until Phase 20 verification passes
