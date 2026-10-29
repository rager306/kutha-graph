---
phase: "244"
name: "pit-probe"
created: 2026-10-29
---

# Phase 244: PIT probe — Context

## Decisions

1. The probe uses day-scale valid time (`YYYYMMDD`) as a test-only clock so 2017-01-23 and 2017-03-14 are distinct. This is not a product calendar claim.
2. `as_of` is force-only; `as_known_on` is TT-only and lists superseded editions by design; `norm_view` is force ∩ knowledge.
3. Subject is `1 CFR <section>`; relation is the existing `inForceAs` allowlist entry; object is the full extracted text.
4. The checkpoint round-trip uses `common::PersistScratch` and tiny segment limits; no new Op.
5. No legal-pack ontology, no Cypher, no thaw.

## Discretion Areas

Whether to keep the fixture text as the object or a digest; the probe needs text equality, so full text is used.

## Deferred Ideas

Applicability/CTV, more sections, and a real corpus remain later work.
