# Phase 19 Context: Admission and policy meta-facts

**Milestone:** v0.05 / harness M012 S03
**Requirements:** ADM-01, ADM-02, ADM-03
**Depends on:** Phase 17 (allowlist as log facts)
**Source of truth for gaps:** `docs/architecture/semantic-gap-review.md` (F5)

## Locked decisions

- **D-01:** Admission status is recorded as a bi-temporal meta-fact (or equivalent log record) queryable AS OF a cut (ADM-01).
- **D-02:** Policy version is pinned in the log and cited by admission (ADM-02).
- **D-03:** `check_admission` (or successor) is invoked on the product path for the leased fixture, not only from tests (ADM-03).
- **D-04:** Do not implement ADR-050 six dictionaries. Do not regress ALL/RULE or M012a oracles.
- **D-05:** Honeycomb stays Proposed. No Rocks/Cypher/HNSW/legal pack.

## Spike anchors

- `crates/kutha-runtime/src/quantum.rs` — `check_admission`, admit path
- Phase 17 `Op::AllowRelation` / Phase 18 `Op::RegisterRule` patterns for log-native meta ops
- `crates/kutha-runtime/tests/m011_e2e_fixture.rs` — current test-only admission caller

## Non-goals

- Thin action record (Phase 20)
- Multi-hop derivation (Phase 21)
- Six dictionaries / WASM UDF sandbox

## Harness

- Active Milestone M012, Active Slice **S03**, Phase H5, `L_delivery=M012-S02-done`
- Do not clear S03 until Phase 19 verification passes
