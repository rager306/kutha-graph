# ADR-071: Hybrid Retrieval Composition

## Status

**Proposed** (Query — approximate/keyword/vector retrieve as leases beside exact MATCH)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Query** (primary) · **Data** · **Agent**
- Depends on: ADR-042, ADR-043, ADR-070
- Anticipates: ADR-052 (derived embeddings)

## Context

Exact MATCH is 041/070. Hybrid retrieve is BM25+dense, keyword connecting trees (BANKS), and mixed scan-vs-ANN (Sanca, already in 043). Community detection is a **droppable lease**, not MATCH.

Grounding cards:

- `ruvector-hybrid-bm25-dense` — `.compound-engineering/artifacts/research/applicability/cards/ruvector-hybrid-bm25-dense.md`
- `paper-keyword-search-graphs` — `.compound-engineering/artifacts/research/applicability/cards/paper-keyword-search-graphs.md`
- `paper-mixed-vector-relational-access` — `.compound-engineering/artifacts/research/applicability/cards/paper-mixed-vector-relational-access.md`

RuVector hybrid is `observed` — adapt behind a port, do not vendor empty Cypher. GNN facade is low/low.

## Decision

### D071-1. Approximate operators are explicit

Hybrid retrieve is a **named operator family** (keyword tree, BM25+dense fusion, ANN kNN), never a silent substitute for exact MATCH. Fail closed if the caller asked for exact.

### D071-2. Compose through the same planner

043 chooses scan vs HNSW vs fusion vs BANKS-style Steiner/connecting subtree. 042 remains the HNSW fence. Fusion crates are adapters.

### D071-3. Keyword ≠ Cypher ≠ BM25-on-chunks

BANKS-class: schema-agnostic keywords over the fold return a connecting subtree. Not MATCH, not chunk RAG. Legal/journalism Layer 5 is GTM color.

**Hard separations:**

```text
Hybrid retrieve        ≠  Exact MATCH
BM25+dense fusion      ≠  SoT
Keyword subtree        ≠  Chunk RAG
Community lease        ≠  This operator (optional pack)
Empty Cypher           ≠  Retrieve success
```

## Consequences

### Positive

- Agent chat can retrieve without lying about MATCH.
- 052 embeddings have a consume path.

### Negative / risks

- `observed` RuVector hybrid needs a tree before `code`.

### Non-goals

- ULTRA as MATCH. GNN rerank as P0.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Always ANN then Cypher | 043/Sanca crossover |
| Graphiti retrieve as engine | Trap |
| Collapse into 070 | Mixing exact and approx in one grammar without names |

## Open Research Questions

1. Surface syntax for approximate vs exact.
2. Tenant vector silos (080 cousin) vs one HNSW with predicate.
3. When community-detection lease (R6 card) is invoked from 071 vs a pack.

## Related Decisions

- ADR-042, ADR-043, ADR-070, ADR-052, ADR-080
