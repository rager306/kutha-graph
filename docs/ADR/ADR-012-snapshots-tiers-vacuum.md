# ADR-012: Snapshots, Tiers, and Vacuum

## Status

**Proposed** (Time/Data placement vs policy GC of the log; not product storage runtime)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Time** (primary) · **Data** · **Packaging** · **Verify**
- Depends on: ADR-000 (D1, D5), ADR-010
- Anticipates: ADR-041 (CSR as aged picture), ADR-080 (who may vacuum), ADR-091 (cold capsules)

## Context

Replay-from-genesis is correct and too slow. This cell splits three nouns that papers keep mixing:

1. **Where a snapshot lives** (compaction / query-driven placement).
2. **How a hot picture is laid out** (adj-list → CSR aging; RAM vertices vs SSD edges).
3. **Whether history may be physically removed** (vacuum + legal hold).

Grounding cards:

- `paper-lsm-snapshot-compaction` — `.compound-engineering/artifacts/research/applicability/cards/paper-lsm-snapshot-compaction.md`
- `paper-bach-lsm-csr-bridge` — `.compound-engineering/artifacts/research/applicability/cards/paper-bach-lsm-csr-bridge.md`
- `paper-semi-external-graph` — `.compound-engineering/artifacts/research/applicability/cards/paper-semi-external-graph.md`
- `paper-event-log-vacuum-legal-hold` — `.compound-engineering/artifacts/research/applicability/cards/paper-event-log-vacuum-legal-hold.md`

LSM snapshots are a **compaction policy**: place them where queries land, not on a uniform wall clock. BACH ages adjacency lists into CSR inside LSM levels for mixed TP/AP. Semi-external graphs keep vertex state in RAM and stream edges from SSD — still a lease. Vacuum is **policy GC of the SoT**; legal hold pins ranges so vacuum cannot touch them. After vacuum, AS-OF of a removed state is a **defined failure**, not a silent hole.

## Decision

### D012-1. Snapshots are leases of the log, placed by query load

Snapshots shorten Yankin reconstruction. They are not SoT. Placement is a Control/pack policy (query-distribution clustering, time-aware compaction knobs). Compaction that **reorganizes layout** is allowed. Compaction that **drops losers silently** is TOKI-illegal (ADR-013).

### D012-2. Aging pipeline: mutable adj → frozen CSR (BACH-shaped)

Hot write-friendly adjacency at the top of the pipeline; AP-friendly CSR after aging (BACH). Samyama frozen CSR (ADR-041) is the in-process cousin of the same noun. LSM levels are not truth.

### D012-3. Tiers: in-RAM fold, SEM edge shards, cold archive

When edges do not fit RAM, **semi-external** placement (vertex state in RAM, edges on SSD) is a materialization tier, not a second log. Cold archive (later RVF/pack) is packaging. Crash recovery rebuilds vertex state from the **log**, not from a dirty edge cache.

### D012-4. Vacuum ≠ compaction; hold pins SoT

Vacuum physically removes expired transaction-time history under a **declared policy**, and logs that the drop happened. A **legal hold** is a grant-shaped fact: vacuum must fail closed on held ranges. LLM does not choose what to forget. P0 may be append-only-forever; honeycomb owns vacuum+hold as a pack.

**Hard separations:**

```text
Snapshot placement     ≠  Vacuum (physical erase)
Layout compaction      ≠  Deleting losers
SEM / SSD edge shards  ≠  Event log
Legal hold             ≠  LSM tombstone
Defined AS-OF failure  ≠  Silent hole after GC
```

## Consequences

### Positive

- Legal PIT can stay cheap without Raphtory-in-RAM history.
- CSR aging has a named pipeline instead of “just compact Rocks.”
- GDPR/e-discovery have a place that is not “delete the fold.”

### Negative / risks

- Query-driven snapshot placement assumes a stable mix.
- Vacuum without 080/policy graph will be ad hoc.

### Non-goals (this ADR)

- Implementing LSM or GraphChi.
- ABAC of *who* may vacuum (ADR-080).
- Treating blockchain ADS as the archive (anchor only, ADR-014).

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Uniform wall-clock snapshots | Wastes cold time; starves hot AS-OF |
| Compaction deletes history | Violates vacuum card + TOKI losers |
| SSD as SoT | SEM is a lease; log remains SoT |
| Agent “what to remember” as GC | LLM must not choose forget |

## Open Research Questions

1. P0: mmap/CSR-if-fits vs when to cut SEM shards.
2. Snapshot cadence as a pack policy vs engine default.
3. Vacuum query semantics: typed `Vacuumed` / `Held` outcomes (cousin of ADR-090 TR-06).
4. Schema vacuuming when historical schemata still interpret remaining facts.

## Related Decisions

- ADR-010 — log quantum
- ADR-013 — losers stay until vacuum policy says otherwise
- ADR-040/041 — leases and CSR
- ADR-080 — who may vacuum (later)
