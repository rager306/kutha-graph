# ADR-042: HNSW Access-Method Fence

## Status

**Proposed** (Data/Query — HNSW as a pack behind a fence; not SoT; not empty Cypher)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Data** (primary) · **Query**
- Depends on: ADR-000 (D5), ADR-040
- Anticipates: ADR-043 (scan vs ANN crossover), ADR-052 (enrichment — **not opened**), ADR-071 (hybrid retrieve surface)

## Context

Vectors are an **access method** over interned node ids, not a memory product. RuVector supplies a real in-process HNSW (`confidence: code`) and a delete-repair crate that rewires neighbors. Filtered/predicate search has two literature poles: walk a predicate subgraph of HNSW (ACORN) or prefilter a subset then kNN (NaviX). SIEVE’s *collection* of indexes is a planner concern (043), not a reason to skip the fence.

Grounding cards:

- `ruvector-hnsw` — `.compound-engineering/artifacts/research/applicability/cards/ruvector-hnsw.md`
- `ruvector-hnsw-delete-repair` — `.compound-engineering/artifacts/research/applicability/cards/ruvector-hnsw-delete-repair.md`
- `paper-navix-filtered-hnsw` — `.compound-engineering/artifacts/research/applicability/cards/paper-navix-filtered-hnsw.md`
- `paper-acorn-predicate-subgraph` — `.compound-engineering/artifacts/research/applicability/cards/paper-acorn-predicate-subgraph.md`

**Trap:** RuVector empty Cypher / fabricated success must not be cited as a capability. GNN facade is low/low. Hindsight four-network memory is not this index.

## Decision

### D042-1. HNSW is a droppable pack, never temporal truth

Insert/search/remove index vectors that *name* interned graph ids. Valid-time, invalidation, and AS OF live on the log (010/013). Cosine cannot supersede facts (MemStrata, ADR-013).

### D042-2. Delete-repair is neighbor rewiring, not constraint repair

Tombstones skip dead ids but stale edges drop recall. Repair strategies are **index maintenance**, borrowed directly from `ruvector-hnsw-repair` (`TombstoneOnly` for fast deletes, `BatchRepair` for amortised sweeps, `EagerRepair` for maximum recall preservation), not PG-constraint repair and not “self-reconstructing agent memory” marketing. Maintenance is `apply(delta)` under ADR-040; it may fall back to rebuild.

### D042-3. Filtered kNN via predicate-agnostic traversal (ruvector-acorn adapter)

Predicate-filtered vector search over temporal entity subsets (`valid_to IS NULL`, jurisdiction, document category) collapses standard HNSW beam search at low selectivity. Adopt `ruvector-acorn` (`AcornIndex1`, `AcornIndexGamma` implementing Patel et al., SIGMOD 2024): denser graphs ($\gamma \cdot M$) with predicate-agnostic traversal to explore neighbor topology through failing nodes without returning them. NaviX-style prefiltering remains an alternative access path for high-selectivity ranges.

### D042-4. Ports own the contract; RuVector is an adapter

Borrow REAL HNSW crates (`rvf-index`, `ruvector-hnsw-repair`, `ruvector-acorn`) behind a port trait. Do not vendor the RuVector monorepo, Cypher stubs, or GNN-as-MATCH. Whole-product “agent brain” is rejected (STRATEGY).

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
