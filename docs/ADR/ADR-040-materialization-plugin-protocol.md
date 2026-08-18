# ADR-040: Materialization Plugin Protocol

## Status

**Proposed** (Data-band protocol — reversible views/leases; CSR/HNSW are instances, not this cell’s kernels)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Data** (primary) · **Composition** · **Time** · **Query**
- Depends on: ADR-000 (D5), ADR-010, ADR-014
- Anticipates: ADR-021 (pack install ≠ view contract), ADR-041 (CSR), ADR-042 (HNSW), ADR-043 (planner over leases)

## Context

D5 already says CSR/HNSW/temporal slices are **reversible plugins**. This cell is the **view contract**: mount / apply_delta / snapshot / unload / rollback — distinct from packing a module into the host.

Grounding cards:

- `paper-pg-materialized-views` — `.compound-engineering/artifacts/research/applicability/cards/paper-pg-materialized-views.md`
- `paper-dbsp-ivm` — `.compound-engineering/artifacts/research/applicability/cards/paper-dbsp-ivm.md`
- `paper-graph-pack-plugin-lifecycle` — `.compound-engineering/artifacts/research/applicability/cards/paper-graph-pack-plugin-lifecycle.md`

PG views: virtual rewrite vs stored materialization; DRed/B-F when recursive. DBSP: IVM as a stream calculus so log deltas become view deltas without per-language heuristics. Pack lifecycle: install/activate/retire of a **versioned module** (operators + dictionary + leases) is not the same noun as `build`/`drop` of one view. DuckPGQ injects planner hooks — do not vendor DuckDB. WASM sandbox is how untrusted *code* runs (051/081), not how a view is maintained.

## Decision

### D040-1. Every hot picture is a lease of the fold

CSR, HNSW, interval indexes, named-graph slices, Cypher views, and process-mining graphs are the same protocol noun: derived structures keyed by log offset. Reversible = drop and fold again. Incremental = apply a changeset. Both must be valid. Competing projections of the same type are allowed; routing is ADR-043.

### D040-2. Plugin lifecycle ≠ pack lifecycle

View contract: `build` / `apply(delta)` / `snapshot` / `unload` / `rollback`. Pack lifecycle (ADR-021, not opened here): install/activate/retire of a versioned artifact that *may register* plugins. Activating a pack is not silently mutating SoT.

### D040-3. IVM is the maintenance algebra, not a second log

DBSP-shaped incrementalization is the target for rich views. DRed-style delete/rederive remains the conservative cousin for recursive views. The event log stays SoT; view deltas are not events unless explicitly logged (e.g. schema/pack events).

### D040-4. Mandatory external graph DBs are forbidden as materializers-of-truth

Neo4j / Graphiti / Falkor / Samyama may inform **kernels** (041/042) but must not be required for assert → invalidate → AS OF.

**Hard separations:**

```text
View / lease           ≠  Event log SoT
apply(delta)           ≠  Append to SoT
Pack install           ≠  View contract
IVM stream             ≠  Second database
Unload                 ≠  Vacuum of history
WASM sandbox           ≠  Materializer plugin
```

## Consequences

### Positive

- 041/042 can be instances of one protocol.
- Planner (043) has a catalog of droppable access paths.
- Vertical packs (090/093) attach AST materializers without forking the engine.

### Negative / risks

- Dual-writing to an external graph “for speed” will recreate a second SoT.
- IVM of HNSW is harder than CSR; 042 must not pretend otherwise.

### Non-goals (this ADR)

- Specifying CSR layout (041) or HNSW repair (042).
- Opening ADR-021.
- Mandatory Graphiti/Neo4j.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Periodic full rebuild only | Too slow after appends; DBSP exists |
| Pack marketplace as the view | Wrong noun |
| Graphiti/Neo4j as required materializer | Violates D1/D5 and STRATEGY |

## Open Research Questions

1. Trait shape (`MaterializerPort`) vs per-kind plugins — spike, not this ADR’s crate.
2. When apply(delta) must fall back to rebuild (HNSW delete-repair is 042).
3. Versioning leases when a dictionary/ontology pack changes (Bellini lifecycle).
4. Query rewrite through views (Han) vs always hitting base fold.

## Related Decisions

- ADR-000 — D5
- ADR-010 — fold at offset
- ADR-014 — materializers consume cascade budget
- ADR-041, ADR-042, ADR-043 — instances and compiler
