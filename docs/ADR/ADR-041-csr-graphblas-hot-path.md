# ADR-041: CSR / GraphBLAS Hot Path

## Status

**Proposed** (Data/Query kernel — Kutha-owned frozen adjacency + WCOJ; vendor trees are evidence, not SoT)

### Evidence (2026-10-02; Proposed)

`leapfrog_intersect` is the sorted-key multiway intersect helper (`leapfrog.rs` unit tests). Hot CSR cut still builds via `CsrLease::from_fold` beside the `Materializer` trait (F8 remainder).

### Evidence (2026-10-03; Proposed)

M016's evaluator (`crates/kutha-runtime/src/eval.rs`) runs Horn bodies, not over the CSR lease. **2026-10-03:** `live_horns` is lease-first via `GraphFold.horns_by_premise`, falling back to `live_horns_scan` (`fold.facts()`) when the lease is not ready (`horn_lease_ready` false). Remaining-premise walk `search_at` / `search_prems` still scans `live_facts_at(tt, vt)` at `tt = u64::MAX`. `leapfrog_intersect` is not used by rules. Oracle: `extra_relatedto_horn_visits_constant_in_n` (`m017_ingest_scaling.rs`). The 2026-10-01 gap-review F9/F16 evaluator question is answered for the shipped Horn path. This subsection does not rewrite D041-1…D041-4.

### Evidence (2026-10-05, Phase 62 / M019 S07; Proposed)

Seeded differential (`SEM-csr-fold-diff`, `m019_sem_diff.rs`): typed and untyped CSR leases at random `(tt, vt)` cuts match `live_at`/`as_of`/`as_known_on` on behavior-free and Horn mixes; persist+open rebuild agrees for legacy, segmented (rollover seam), and migrate. Evaluator still does not join over the CSR lease. Horn@TT=`MAX`, `as_of` vs eligibility, and C01 stay open. This subsection does not rewrite D041-1…D041-4. Status stays **Proposed**.

### Evidence (2026-10-06, Phase 48 / M018 S04; Proposed)

I-F15 visit bound remains `extra_relatedto_horn_visits_constant_in_n` (`m017_ingest_scaling.rs`; GREEN after M017 `horns_by_premise`). This subsection does not rewrite D041-1…D041-4. Status stays **Proposed**.

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

### Open question (2026-09-30)

Open, from docs/architecture/semantic-gap-review.md (F8): D041-1 names the hot hop picture as CSR over interned ids. This cell does not yet say that temporal cuts (AS OF / supported-at) must use that picture rather than scanning the fold's fact table. This question does not rewrite D041-1…D041-4 and does not mark this cell Accepted.

### Open question (2026-10-01)

Open, from docs/architecture/semantic-gap-review.md (F9, F16): whether rule bodies (multi-hop, n-ary, inverse, admission) are evaluated by the existing leapfrog join over the CSR lease, with support counts as semiring multiplicities and delete/rederive maintenance on retraction. This is a candidate evaluator for a small positive rule language (M015), not a commitment to a Datalog engine or a MATCH compiler. This question does not rewrite D041-1…D041-4 and does not mark this cell Accepted.

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
