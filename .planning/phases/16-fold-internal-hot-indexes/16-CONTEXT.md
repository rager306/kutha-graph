# Phase 16 Context: Fold-internal hot indexes

**Milestone:** v0.04 / harness M012a S06 (last M012a slice)
**Requirements:** HOT-01, HOT-02, HOT-03
**Depends on:** Phase 13 (stable refs); numeric order after Phase 15
**Source of truth for gaps:** `docs/architecture/semantic-gap-review.md` (F8)

## Locked decisions

- **D-01:** `claim_supported_at` / `as_of` at the leased N do not require a full linear scan of all facts (fold-internal index or equivalent) (HOT-01).
- **D-02:** CSR lease path remains a droppable lease; indexes are not a second SoT (HOT-02).
- **D-03:** Materializer vs `from_fold` is either wired on the hot path **or** a named guard documents the spike limit (HOT-03).
- **D-04:** Do not regress LOG/REF/ING/DUR/TIME oracles. Honeycomb stays Proposed. No M012 dicts-as-facts. No M002 Rocks. ADR-061 full GED-class diff stays out of M012a.

## Spike anchors

- `crates/kutha-runtime/src/fold.rs` — `as_of`, `claim_supported_at`, `facts()`
- `crates/kutha-runtime/src/csr.rs`, `materializer.rs`, `quantum.rs` (`csr_lease_at`)
- Bench note in semantic-gap-review F8 (linear µs growth)

## Non-goals

- M012 dictionaries-as-facts / rule registry
- M002 Rocks durability
- Full ADR-061 diff API

## Harness

- Active Milestone M012a, Active Slice **S06**, Phase H5, `L_delivery=M012a-S05-done`
- Do not clear S06 until Phase 16 verification passes
