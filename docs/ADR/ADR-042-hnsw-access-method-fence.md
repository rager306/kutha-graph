# ADR-042: HNSW Access-Method Fence

## Status

**Proposed** (Data/Query — HNSW as a pack behind a fence; not SoT; not empty Cypher)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Data** (primary) · **Query**
- Depends on: ADR-000 (D5), ADR-040
- Anticipates: ADR-043 (scan vs ANN crossover), ADR-052 (enrichment pack; cosine/coherence still ≠ fact), ADR-071 (hybrid retrieve surface)

## Context

Vectors are an **access method** over interned node ids, not a memory product. RuVector supplies a real in-process HNSW (`confidence: code`) and a delete-repair crate that rewires neighbors. Filtered/predicate search has two literature poles: walk a predicate subgraph of HNSW (ACORN) or prefilter a subset then kNN (NaviX). SIEVE’s *collection* of indexes is a planner concern (043), not a reason to skip the fence.

Grounding cards:

- `ruvector-hnsw` — `.compound-engineering/artifacts/research/applicability/cards/ruvector-hnsw.md`
- `ruvector-hnsw-delete-repair` — `.compound-engineering/artifacts/research/applicability/cards/ruvector-hnsw-delete-repair.md`
- `paper-navix-filtered-hnsw` — `.compound-engineering/artifacts/research/applicability/cards/paper-navix-filtered-hnsw.md`
- `paper-acorn-predicate-subgraph` — `.compound-engineering/artifacts/research/applicability/cards/paper-acorn-predicate-subgraph.md`
- Adapter mapping (crate identifiers on top of those cards): `.compound-engineering/artifacts/research/ruvector-plugin-adaptation.md` (P-HNSW)

**Trap:** RuVector empty Cypher / fabricated success must not be cited as a capability. GNN facade is low/low. Hindsight four-network memory is not this index.

## Decision

### D042-1. HNSW is a droppable pack, never temporal truth

Insert/search/remove index vectors that *name* interned graph ids. Valid-time, invalidation, and AS OF live on the log (010/013). Cosine cannot supersede facts (MemStrata, ADR-013).

### D042-2. Delete-repair is neighbor rewiring, not constraint repair

Tombstones skip dead ids but stale edges drop recall. Repair strategies are **index maintenance** on the HNSW lease (`ruvector-hnsw-delete-repair`: TombstoneOnly / BatchRepair / EagerRepair), not PG-constraint repair and not “self-reconstructing agent memory” marketing. Maintenance is `apply(delta)` under ADR-040; it may fall back to rebuild. Crate identifier `ruvector-hnsw-repair` is the adapter target in the RuVector note, not an in-tree dependency.

### D042-3. Filtered kNN uses existing HNSW, not a new hybrid index in this cell

Predicate-filtered vector search over temporal entity subsets (`valid_to IS NULL`, jurisdiction, document category) can collapse standard HNSW beam search when only a small fraction of candidates match the predicate. Literature poles stay as already closed cards: ACORN walks a predicate subgraph of HNSW (`paper-acorn-predicate-subgraph`); NaviX prefilters then kNN (`paper-navix-filtered-hnsw`). This cell does **not** invent a specialized hybrid structure; Compass/SIEVE coordination is ADR-043. When STATE names HNSW, the ACORN-shaped adapter identifier is `ruvector-acorn`; NaviX-style prefiltering remains an alternative access path when the predicate strongly filters the candidate set. Numeric crossover thresholds stay in ADR-043.

### D042-4. Ports own the contract; RuVector is an adapter

Borrow REAL HNSW crates behind a port trait. Named identifiers (`rvf-index` / `ruvector-hnsw`, `ruvector-hnsw-repair`, `ruvector-acorn`) live in the adaptation note. Do not vendor the RuVector monorepo, Cypher stubs, or GNN-as-MATCH. Whole-product “agent brain” is rejected (STRATEGY). `.kutha/STATE.md` still forbids implementing this cell until HNSW is named.

**Hard separations:**

```text
HNSW graph             ≠  Event-log SoT
Tombstone / rewire     ≠  Fact invalidation
Filtered ANN           ≠  Exact MATCH (LFTJ)
Empty Cypher success   ≠  Capability
GNN facade             ≠  Hot path
Hindsight / Graphiti   ≠  This index
```

## Consequences

### Positive

- Hybrid retrieve (071) has a fenced ANN path.
- Legal embeddings cannot silently serve superseded statutes.

### Negative / risks

- Incremental HNSW IVM is weaker than CSR; planners must know rebuild cost.
- Adapter drift vs upstream ruvector.

### Non-goals (this ADR)

- Hybrid cost model (043).
- GenAI enrichment pack (052).
- GPU ANN.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| HNSW as SoT / memory DB | Violates D1/D5 |
| Always-hit embedding router | Topic table belongs in 050, not this index |
| Specialized hybrid index in 042 | Compass: coordinate existing structures |
| RuVector empty Cypher | Trap audit |

## Open Research Questions

1. Default repair strategy for engine-scale vs agent-memory-scale graphs.
2. Predicate pushdown: ACORN walk vs NaviX prefilter vs 043 collection (SIEVE).
3. Vector payload storage: separate column vs RVF (091) vs in-graph properties.
4. When exhaustive scan beats ANN (Sanca mixed-access — 043).

## Related Decisions

- ADR-013 — cosine ≠ stale-fact
- ADR-040 — plugin protocol
- ADR-043 — scan vs probe
- ADR-050 — must not use HNSW as the only tool gate
