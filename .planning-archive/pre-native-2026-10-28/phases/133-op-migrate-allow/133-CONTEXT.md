---
phase: 133-op-migrate-allow
---

# Phase 133 — Context

CE #1 / `op-compat.md` follow-on: owner authorized migrate off main-era dictionary meta-ops. Full five-variant backfill is multi-slice; S01 is AllowRelation only, mirroring `Runtime::emit_allow` TermId shape (`subject=object=name`, `relation=kutha:allows`, `delivery_key=name`).

Forbidden: silent serde aliases on `Op` (would launder TermIds and break m050).
