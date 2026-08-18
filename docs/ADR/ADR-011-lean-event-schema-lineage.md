# ADR-011: Lean Event Schema & Lineage

## Status

**Proposed** (Time/Data encoding cell — Kutha record shape, intern map, how-provenance; not a new literature noun)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Time** (primary) · **Data** · **Verify**
- Depends on: ADR-000 (D1, D2), ADR-010
- Anticipates: ADR-013 (VT×TT on facts), ADR-050 (agent control dictionaries ≠ intern map), ADR-014 (receipts vs polynomials)

## Context

ADR-010 locks the log. This cell is the **encoding choice**: what an event record contains, how identifiers stay dense on the hop path, and how “how did this hop arise” is algebraic — not LLM narrative.

This is **not** a missing arXiv noun. Surrounding cards already name the cousins:

- `paper-ocpm-multi-object-events` — `.compound-engineering/artifacts/research/applicability/cards/paper-ocpm-multi-object-events.md`
- `paper-rdf-term-dictionary` — `.compound-engineering/artifacts/research/applicability/cards/paper-rdf-term-dictionary.md`
- `paper-provenance-semirings` — `.compound-engineering/artifacts/research/applicability/cards/paper-provenance-semirings.md`

OCPM: one event may point at **many typed objects**; flattening to a single `case_id` causes convergence/divergence. RDF term dictionaries: the hot path runs on interned IDs; the string↔id map is a **reversible materialization**, not the graph. Provenance semirings: how-provenance is a polynomial (`+` alternatives, `×` joint use), not a Merkle receipt and not why-not absence.

**Two dictionary nouns (KD7):** intern/term map lives **here**. Agent control dictionaries (Action, Policy, …) live in **ADR-050**. Collapsing them is a fail.

## Decision

### D011-1. Lean events; objects are not flattened

P0 event taxonomy stays lean (create / patch / invalidate / `behavior.*` / `llm.*` as named in ADR-000 R5). An event may reference **N objects** (OCPM). Do not require a single process-mining `case_id`. Process variants and conformance, if any, are leases over the log — not a second process DB.

Identifiers for new events: **UUID v7** (ADR-000 D2) for log order friendliness. Graph-internal hop keys are interned integers (D011-2).

### D011-2. Intern map is a lease, not SoT

Before joins, IRIs/literals/labels become dense integer IDs (`paper-rdf-term-dictionary`). CSR/HNSW/interval indexes store IDs. The string arena is droppable and rebuildable. LiteMat-style hierarchy bits are an **optional inference lease**, not OWL materialization as truth.

This intern map is **not** the D3 Action/Relation/Policy dictionaries.

### D011-3. How-provenance is a semiring polynomial

Query results may carry how-provenance polynomials over event tokens (Green–Karvounarakis–Tannen). P0: why-tokens on positive conjunctive MATCH. Polynomials are a **derivation lease**, not log integrity (that is ADR-014 receipts). LLM does not invent coefficients. Negation/dual indeterminates stay out of P0.

### D011-4. Lineage ≠ agent narrative

`caused_by` / witness links point at prior events or content-addressed tool/LLM cache keys. They do not substitute dictionaries (050) or ABAC (080).

**Hard separations:**

```text
Lean event record     ≠  Graphiti episode blob
Intern / term map     ≠  Agent control dictionaries (ADR-050)
How-polynomial        ≠  Quantum receipt (ADR-014)
N-ary object links    ≠  Single case_id flatten
SchemaModify payload  ≠  Silent ALTER of SoT
```

## Consequences

### Positive

- WCOJ/CSR can run on integers (ADR-041).
- Legal/science IRIs stay off the hop path.
- Explainable MATCH without treating the LLM as lineage.

### Negative / risks

- Canonical field list is still a spike (Open Research Questions).
- Temptation to store IRIs inside CSR “for convenience.”

### Non-goals (this ADR)

- Shipping Rust structs or a crate.
- Ontology/schema evolution protocol (cousins in 050; SMO on the log).
- Why-not provenance, PACT argument provenance, or crypto receipts.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| One case id per event | OCPM flattening loss |
| IRIs in CSR adjacency | Destroys hop density; intern map exists |
| RDF dictionary as second SoT | Map is a lease; log remains SoT |
| ProvSQL / LLM story as lineage | Wrong algebra; LLM not coefficients |
| Put Action/Policy dicts in this cell | KD7; those are ADR-050 |

## Open Research Questions

1. Canonical event record fields (minimum viable: type, uuid v7, object-id set, interned payload refs, `caused_by`, VT/TT stamps as deepened in 013).
2. Whether LiteMat hierarchy-in-ID is a P1 inference pack or never.
3. How-polynomial storage: per-result lease vs on-demand rewrite.
4. Mapping of OCEL/EKG interchange onto lean events without importing ProM.

## Related Decisions

- ADR-000 — D1/D2, R5 event schema
- ADR-010 — log quantum
- ADR-013 — bi-temporal stamps on facts
- ADR-014 — receipts (integrity) vs this cell’s polynomials (derivation)
- ADR-050 — control dictionaries
