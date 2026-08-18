# ADR-022: Vertical-Slice Module Conventions

## Status

**Proposed** (Space — cohesion inside a pack crate; engineering convention)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Space** (primary)
- Depends on: ADR-002, ADR-020
- Anticipates: ADR-021 (what a slice *installs*), ADR-090/093 (example slices)

## Context

ADR-020 names workspace laws. This cell names **what lives inside one vertical slice** so legal, science, and later finance packs look the same. WorldDB nested worlds are the *product* metaphor (case file / jurisdiction); named graphs are the *data* metaphor. Neither replaces crate conventions.

Grounding cards:

- `paper-worlddb-worlds-edge-programs` — `.compound-engineering/artifacts/research/applicability/cards/paper-worlddb-worlds-edge-programs.md`
- `paper-named-graphs-rdf-dataset` — `.compound-engineering/artifacts/research/applicability/cards/paper-named-graphs-rdf-dataset.md`

## Decision

### D022-1. Slice = one module crate, hexagon inside

Each vertical slice owns: domain types, relation behaviors, dictionary facets, optional materializers, Ports. Hexagon (ports/adapters) lives **inside** the slice, not as a repo-global layer.

### D022-2. Slices communicate only via Ports + the log

No slice instructs another except by logged events (ADR-010). Shared kernel types live in `common`. Embeddings/world hashes are leases.

### D022-3. Default graph vs named graph is compile scope

A slice typically owns one or more named-graph IRIs. Compiled queries must not silently union every pack (named-graphs card). Tenant quotas stay 080.

**Hard separations:**

```text
Vertical slice crate   ≠  Named graph IRI
World / case file      ≠  Event log
Slice-local hexagon    ≠  Global ports/ tree
```

## Consequences

### Positive

- 090 and 093 can copy one skeleton.
- AI compilers have a checklist (STCA isolation laws).

### Negative / risks

- Conventions without a first slice remain slogans until 090/093 spike.

### Non-goals

- IndoorGML. Federated SPARQL as a slice (that's a port, 070 cousin).

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Technical layers as top-level crates | STRATEGY rejection |
| One world = one RocksDB | Second SoT |

## Open Research Questions

1. Shared `kutha-pack` crate vs copy-paste skeleton.
2. How WorldDB `on_insert` maps onto relation behaviors without Merkle-SoT.

## Related Decisions

- ADR-020, ADR-021, ADR-090, ADR-093
