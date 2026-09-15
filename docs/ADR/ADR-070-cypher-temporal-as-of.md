# ADR-070: Cypher + Temporal AS OF Surface

## Status

**Proposed** (Query — GPML calculus with Cypher/GQL skins and native AS OF; not the planner)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Query** (primary) · **Time**
- Depends on: ADR-013, ADR-041, ADR-043, ADR-050
- Anticipates: ADR-071 (approximate/hybrid retrieve), ADR-080 (rewrite into the same IR)

## Context

ADR-043 is the compiler over leases. This cell is the **language surface**. ISO GQL and SQL/PGQ share GPML; industry Cypher is a dialect, historically not all RPQs. AZ projected-schema Cypher: NL compiles to MATCH over a projected schema — LLM is compiler, not executor. T-GQL: interval/path semantics; Neo4j is a host. RPQ: automata/product or matrix on labeled CSR, beside LFTJ for conjunctive cores.

Grounding cards:

- `paper-iso-gql-gpml` — `.compound-engineering/artifacts/research/applicability/cards/paper-iso-gql-gpml.md`
- `paper-az-projected-schema-cypher` — `.compound-engineering/artifacts/research/applicability/cards/paper-az-projected-schema-cypher.md`
- `paper-regular-path-queries` — `.compound-engineering/artifacts/research/applicability/cards/paper-regular-path-queries.md`
- `paper-tgql-intervals` — `.compound-engineering/artifacts/research/applicability/cards/paper-tgql-intervals.md`

Empty Cypher success (RuVector) is a trap. SHACL compile is verify, not this surface.

## Decision

### D070-1. One GPML IR; Cypher and GQL as skins

Implement pattern calculus (GPML/GPC) onto LFTJ plus explicit walk/trail/simple uniqueness and separate shortest/foremost selection objectives; restless journeys are 013/TVG cousins. SQL/PGQ is an optional **view pack** over tabular leases, not a second SoT.

### D070-2. Temporal AS OF is native

Surface binds valid-time and transaction-time cuts (013). Interval index is an access path, not the language. Compiled plans must not silently use “now” when AS OF was requested (MemStrata / statutory QA).

Native fold cut (not the Cypher skin): `GraphFold::as_of(vt)` and `Runtime::csr_lease_at(tt, vt)` exist in P0. Cypher/GPML grammar remains Proposed and frozen under the current `.kutha/STATE.md`; completion of S03 did not authorize it.

### D070-3. Writes are typed operators, not free MERGE-as-truth

Assert/retract/correct (010/TGMS) may have Cypher-shaped syntax later; they still validate through dictionaries (050) and grants (080). LLM-produced Cypher is a proposal.

### D070-4. Named-graph / USE scope is explicit

Compiled patterns do not union every pack by default (020/named graphs).

### Clarification (2026-09-13): choose semantics before an access path

The proposed query contract distinguishes:

- **Snapshot path:** every edge is live at the same `(tt, vt)` cut.
- **Co-temporal interval path:** edges share a nonempty validity intersection within the requested window, at a fixed knowledge cut. Alternative validity windows compose by union; simultaneous premises by intersection.
- **Journey:** traversal times advance along edges, with explicit waiting/latency constraints. For `A->B [1,2)` and `B->C [3,4)`, waiting may permit a journey, but there is no simultaneous snapshot path. A journey is a separate operator, not the default meaning of AS OF.

Specify node uniqueness, relationship uniqueness, walk/trail/simple mode, endpoint answers versus path enumeration, and set/bag multiplicity independently. Shortest/foremost are selection objectives, not substitutes for uniqueness. Cypher relationship uniqueness is not vertex-injective subgraph isomorphism; dialect defaults must be pinned by a compatibility contract (see the corrected `paper-subgraph-iso-vs-homomorphism` card).

LFTJ's conjunctive-query guarantee does not automatically cover simple-path enumeration, restless journeys, or all GPML. P0 implements sorted-row `leapfrog_intersect`, not full variable-ordered MATCH. Budget-limited results must retain completeness metadata (ADR-014); an empty partial result is not proof of absence.

**Hard separations:**

```text
GPML IR                ≠  Neo4j compatibility product
Cypher skin            ≠  Calculus
AS OF                  ≠  Latest-state RAG
Exact MATCH            ≠  Hybrid retrieve (071)
NL→Cypher              ≠  LLM as executor
Empty success          ≠  Capability
```

## Consequences

### Positive

- Buyers can hear “Cypher-class” without forking the planner.
- 080 can rewrite the same IR.

### Negative / risks

- GQL v1 holes; trail vs walk footguns next to journeys.

### Non-goals

- Full ISO GQL. Streaming RPQ. PathDB algebra.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Cypher-as-Neo4j clone | Host ≠ SoT; missing RPQ |
| SPARQL as core | Export dialect; MATCH stays leapfrog |
| Skip AS OF in v1 surface | Violates D4 / legal wedge |

## Open Research Questions

1. Minimum Cypher subset for P1 (node/edge MATCH + AS OF + interned props).
2. Path mode default (fail-closed vs trail).
3. How 050 dictionaries bind labels at compile time.

## Related Decisions

- ADR-043, ADR-013, ADR-041, ADR-050, ADR-071, ADR-080
