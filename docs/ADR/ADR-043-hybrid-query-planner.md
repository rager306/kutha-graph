# ADR-043: Hybrid Query Planner

## Status

**Proposed** (Query/Data compiler over leases — not a new join algorithm)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Query** (primary) · **Data** · **Composition**
- Depends on: ADR-013 (interval index), ADR-040, ADR-041, ADR-042
- Anticipates: ADR-050 (vocabulary bind, not cost-based planner), ADR-070/071 (surface)

## Context

The matrix closed many **access paths** and almost no Kutha **cost model**. This cell is one compiler that chooses among leases. It does not invent another WCOJ (041), another HNSW (042), or NL→MATCH (050/070).

Grounding cards:

- `paper-mixed-vector-relational-access` — `.compound-engineering/artifacts/research/applicability/cards/paper-mixed-vector-relational-access.md`
- `paper-compass-cooperative-hybrid` — `.compound-engineering/artifacts/research/applicability/cards/paper-compass-cooperative-hybrid.md`
- `paper-graph-cardinality-estimation` — `.compound-engineering/artifacts/research/applicability/cards/paper-graph-cardinality-estimation.md`
- `paper-sieve-index-collection` — `.compound-engineering/artifacts/research/applicability/cards/paper-sieve-index-collection.md`

Sanca: at low relational selectivity, batched scan can beat ANN probe; crossover depends on dim, HNSW params, concurrency. Compass: coordinate existing HNSW/IVF and B+ rather than a new hybrid index. CardEst: WCOJ bounds worst-case work; the planner still needs \(|Q|\) for *this* instance (OmniSketch, CEG optimistic vs pessimistic, COLOR, JOB). SIEVE: a **collection** of indexes plus a 3-D (size, time, recall) picker — constrained HNSW walks lose few-hop properties under hard predicates.

## Decision

### D043-1. Planner is a pack over leases, not an LLM

The compiler chooses among: fold scan, CSR/LFTJ, interval index, HNSW probe, view rewrite, (later) other leases. LLM may **propose** a plan (050 compiler-not-executor); the engine **validates and costs** it. LLM is not the cost-based planner.

### D043-2. No new join algorithm in this cell

Conjunctive exact MATCH stays LFTJ-class (041). Fail-closed: if the plan cannot guarantee exact MATCH, it must not return “success” (empty-Cypher trap). ULTRA/GNN are not MATCH substitutes.

### D043-3. Hybrid = cooperative existing indexes

Compass-shaped coordination: vector and structured predicates cooperate. Mixed-access crossover (scan vs probe) is a statistic, not a slogan. SIEVE-shaped collections are allowed as a **catalog policy** under 040, picked at query time by the same compiler.

### D043-4. Cardinality is the missing kernel

P0 may ship naive/AGM-pessimistic estimates. Honeycomb: CardEst that does not assume uniformity/independence on many-to-many graph joins (JOB lesson). Learned GNN CardEst is demand, not P0.

**Hard separations:**

```text
Planner                ≠  LLM
Cardinality statistic  ≠  Join algorithm
Scan vs ANN crossover  ≠  “Always HNSW”
Index collection       ≠  New hybrid structure as SoT
Exact MATCH            ≠  Approximate retrieve (071)
NL bind / dictionaries ≠  This compiler (ADR-050 / 070)
```

## Consequences

### Positive

- Access-path cards have a home without minting Query 103+.
- 041 and 042 stop competing as “the” engine.

### Negative / risks

- Without JOB-class traces, the planner will be vibes (Yankin envelopes apply).
- Over-collecting indexes (SIEVE) fights Cui budgets (014).

### Non-goals (this ADR)

- Cypher grammar (070).
- New WCOJ.
- Agent dictionaries (050) as the optimizer.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| LLM as planner | Nondeterminism; D3 validate fail-closed |
| Specialized hybrid index as SoT | Compass: coordinate existing |
| Always ANN then filter | Sanca crossover; ACORN/NaviX already fenced in 042 |
| Another WCOJ | Closed in 041 / LFTJ paper |

## Open Research Questions

1. JOB-class (or graph-JOB) cost envelope for Kutha — falsifiable spike.
2. Default estimator: pessimistic AGM vs OmniSketch-class interpolation.
3. How 014 cascade budget V appears as a planner constraint vs admission.
4. Fail-closed exact MATCH vs explicit approximate operator in 071.

## Related Decisions

- ADR-013 — interval path
- ADR-040–042 — leases
- ADR-050 — proposal/validate, not cost model
- ADR-070/071 — surface
