# M021 / v0.15 — Wedge decisions close

**Status:** controller plan (owner: decide elegantly + GSD auto)
**Date:** 2026-10-07
**Lease:** harness M021; GSD phases 68–70

## Verdicts (elegant principle)

**No fifth clock. Everything else is a fact or a query.**

| ID | Decision |
|----|----------|
| C01 | Pack stamp via Assert (`appliesTo` / application VT). Not Horn, not fifth axis, not `valid_to`-as-application for text force. FIXG-01 stays. |
| HORN-KC | Confirm ingest-at-head; `as_known_on` TT-hides only. |
| ASOF-ELIG | Confirm `as_of` support-live; eligibility separate API. |
| TR-02 | Force=VT; publication=`TtAnchor`/`as_known_on`; case decision=`as_of` arg. |
| RETRACT-CI | Retract(event_id) invalidates **all** live residual rows for that id (Phase 69). |
| Pi / agent resume | Out of scope — harness honesty later; not product SoT. |

## Slices

| Slice | Phase | After this |
|-------|-------|------------|
| S01 | 68 | TR-02 + C01 appliesTo pins; matrix killed; ADR Evidence; no fold change |
| S02 | 69 | RED Retract-all-live; fold fix; bench ≤10% |
| S03 | 70 | M021 close; audit note; Active None |

## Non-goals

Cypher, HNSW, legal pack crate, fifth clocks, invert Phase 46 pins, Pi Durable clone.
