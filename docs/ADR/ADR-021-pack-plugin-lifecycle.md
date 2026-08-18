# ADR-021: Pack / Plugin Lifecycle

## Status

**Proposed** (Space/Composition — versioned module install/activate/retire; not the view contract)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Space** (primary) · **Composition** · **Data**
- Depends on: ADR-000 (D5), ADR-010, ADR-020, ADR-040
- Anticipates: ADR-050 (pack may carry dictionaries), ADR-090/093 (vertical packs)

## Context

ADR-040 owns `build` / `apply(delta)` / `drop` of a **lease**. This cell owns **install / activate / retire** of a versioned artifact that *registers* operators, types, dictionaries, and leases.

Grounding cards:

- `paper-graph-pack-plugin-lifecycle` — `.compound-engineering/artifacts/research/applicability/cards/paper-graph-pack-plugin-lifecycle.md`
- `paper-online-pg-schema-evolution` — `.compound-engineering/artifacts/research/applicability/cards/paper-online-pg-schema-evolution.md`

DuckPGQ: inject parser/optimizer into a host without forking it. Bellini: index/store versioning when ontology packs change. Living Databases: dependent objects evolve with schema. Schema SMO is a **logged rewrite**; pack activate *may require* an SMO, but they are different nouns. WASM is how untrusted *code* runs (081), not this lifecycle.

## Decision

### D021-1. A pack is a versioned artifact on the log

A pack = operators + optional dictionary facets + lease registrations. Lifecycle events: `PackInstall` / `PackActivate` / `PackRetire` (names are design-level). Activate does not mutate SoT except by appending those events. Rollback = retire + drop leases (040).

### D021-2. Pack ≠ view; pack ≠ schema SMO

View contract remains ADR-040. Domain ontology/schema change remains dictionary/`SchemaModify` (050). Activating legal-reference (090) may require both a pack event *and* a schema event.

### D021-3. Host extension, not a second engine

Packs register into Kutha; they do not vendor DuckDB/Neo4j/Graphiti as SoT. Cordis-aligned: reversible effects, talk through Ports (020).

**Hard separations:**

```text
Pack activate          ≠  View build (040)
Pack version           ≠  Schema version (050 SMO)
WASM sandbox           ≠  Pack lifecycle (081)
Marketplace UI         ≠  This cell
```

## Consequences

### Positive

- 090/093 have an install story.
- Dictionary drift can version dependent CSR/HNSW leases.

### Negative / risks

- Cascade storms if activate rebuilds every lease without 014/031 budgets.

### Non-goals

- Plugin marketplace product. WASM. Opening a Cargo registry.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Treat 040 as pack install | Wrong noun (already fenced) |
| oxify DAG as pack runtime | Trap orchestration |
| Fork the engine per vertical | Defeats STCA packs |

## Open Research Questions

1. Semver vs valid-time on pack manifests.
2. Which leases must rebuild on dictionary pack change.
3. Compile-hook injection (DuckPGQ shape) vs explicit Port registration.

## Related Decisions

- ADR-020, ADR-040, ADR-050, ADR-014, ADR-090, ADR-093
