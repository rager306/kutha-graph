# Phase 21 Context: Multi-hop derivation

**Milestone:** v0.05 / harness M012 S05 (last M012 slice)
**Requirements:** DER-01, DER-02, DER-03
**Depends on:** Phase 18 (rule registry)
**Source of truth for gaps:** `docs/architecture/semantic-gap-review.md` (F4)

## Locked decisions

- **D-01:** Derivation eligibility is not limited to one-hop `caused_by` for the leased fixture (DER-01).
- **D-02:** A two-hop (or n-ary) named test is green without claiming full provenance polynomials (DER-02).
- **D-03:** Hard-coded `inverse_knows`-only cascade is not the sole follow-on mechanism, or is a named residual spike guard (DER-03).
- **D-04:** Do not implement ADR-050 six dictionaries or MATCH compiler. Do not regress ALL/RULE/ADM/ACT or M012a oracles.
- **D-05:** Honeycomb stays Proposed. No Rocks/Cypher/HNSW/legal pack.

## Spike anchors

- `crates/kutha-runtime/src/quantum.rs` — `derivation_eligible_at`, `follow_ons`, Behavior
- Phase 18 rule registry hash pins
- ADR-011 D011-3 open question (polynomials out of scope)

## Non-goals

- Full provenance polynomials / MATCH compiler
- Six dictionaries / M002

## Harness

- Active Milestone M012, Active Slice **S05**, Phase H5, `L_delivery=M012-S04-done`
- Do not clear S05 until Phase 21 verification passes
