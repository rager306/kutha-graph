# ADR-041: CSR / GraphBLAS Hot Path

## Status

**Proposed** (Data/Query kernel — Kutha-owned frozen adjacency + WCOJ; vendor trees are evidence, not SoT)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Data** (primary) · **Query** · **Composition**
- Depends on: ADR-000 (D5, D6), ADR-011 (interned ids), ADR-040
- Anticipates: ADR-043 (planner chooses LFTJ vs scan), ADR-070 (Cypher surface)

## Context

STRATEGY track 2: speed is a **removable picture** of the log. Leapfrog Triejoin is already the theoretical baseline in `AGENTS.md`. This cell names the hot adjacency picture and forbids treating Samyama or Falkor as Kutha’s event log.

Grounding cards:

- `samyama-csr-frozen-adjacency` — `.compound-engineering/artifacts/research/applicability/cards/samyama-csr-frozen-adjacency.md` (`confidence: code`)
- `samyama-leapfrog-triejoin` — `.compound-engineering/artifacts/research/applicability/cards/samyama-leapfrog-triejoin.md` (`confidence: code`)
- `samyama-late-materialization` — `.compound-engineering/artifacts/research/applicability/cards/samyama-late-materialization.md` (`confidence: code`)
- `paper-lftj-wcoj` — `.compound-engineering/artifacts/research/applicability/cards/paper-lftj-wcoj.md`
- `falkor-graphblas-sparse-adj` — `.compound-engineering/artifacts/research/applicability/cards/falkor-graphblas-sparse-adj.md` (`confidence: spec`)

Samyama: `FrozenAdjacency` CSR segments + write-buffer compact; leapfrog on sorted neighbors; ids-first late materialization. Veldhuizen LFTJ: multiway join work tracks AGM bound (up to log). Falkor GraphBLAS/CSC is a **spec-class** cousin: graph as sparse matrix — do not raise to `code` without a kernel tree. BACH (ADR-012) is how adj ages into CSR on disk.

## Decision

### D041-1. Kutha owns CSR; vendors are kernel evidence

Hot hop picture is compressed sparse row (or GraphBLAS-equivalent sparse adj) over **interned ids**. Compact write buffer into immutable CSR tiers is the in-process aging step (Samyama-shaped). Samyama is **not** Kutha SoT. TypeScript is not the graph core (D6).

### D041-2. Conjunctive MATCH uses LFTJ-class WCOJ

Variable-ordered leapfrog over sorted trie/CSR iterators; no mandatory pairwise intermediates. This cell does **not** invent another WCOJ algorithm. Hash-table variant (Ross) is an optional later pack. Temporal BGP / CompactLTJ stay cousins.

### D041-3. Late materialization

Hops stay on compact ids/CSR; property maps pull only when the plan needs them. Planner (043) must know the cost of early vs late property access.

### D041-4. GraphBLAS is an API shape, not a second SoT

Sparse-matrix kernels may implement some analytics/hops. They remain leases of the fold. Falkor remains `spec` until a tree is read.

**Hard separations:**

```text
CSR / sparse adj       ≠  Event log
Samyama store          ≠  Kutha SoT
LFTJ                   ≠  Pairwise Cypher intermediates
Late ids               ≠  Fat property maps on every hop
GraphBLAS API          ≠  Required Falkor runtime
TypeScript core        ≠  Allowed (D6)
```

## Consequences

### Positive

- P0 hop path has a named kernel and a theoretical bound.
- 043 can treat LFTJ as one access path among leases.

### Negative / risks

- Copying Samyama without an event-log fold recreates a state-first product (rejected).
- Unsorted CSR breaks leapfrog.

### Non-goals (this ADR)

- New WCOJ paper or GPU.
- GNN-as-MATCH (low/low trap).
- Hybrid planner cost model (043).

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Samyama as product SoT | Abandoned log/agents; STRATEGY |
| Pairwise hash joins as default | Worse than LFTJ on cyclic patterns |
| Another WCOJ algorithm this cell | Bound already closed |
| ULTRA / GNN MATCH substitute | Trap / low optimality |

## Open Research Questions

1. Segmented CSR vs single array; compact trigger vs BACH disk aging.
2. Seekable iterator trait sufficient for LFTJ without copying Samyama files.
3. When GraphBLAS kernels beat pointer CSR for Kutha analytics.
4. JOB-class envelope once 043 exists.

## Related Decisions

- ADR-011 — interned ids
- ADR-012 — adj→CSR aging
- ADR-040 — plugin protocol
- ADR-043 — chooses this path
