# ADR-020: Cargo Workspace & Port Isolation

## Status

**Proposed** (Space — compile-time isolation; not a graph capability noun)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Space** (primary) · **Composition**
- Depends on: ADR-000 (D6), ADR-001, ADR-002
- Anticipates: ADR-021 (runtime pack lifecycle), ADR-022 (slice conventions), ADR-080 (tenant slice ≠ crate)

## Context

STCA Space is vertical isolation: modules talk only through **Ports**. Cargo workspaces are the Rust encoding of that law (`docs/architecture/stca-guide.md`). Named graphs, WorldDB nested worlds, and tenant slices are *runtime* Space nouns; this cell is **how crates depend**.

Grounding cards:

- `paper-named-graphs-rdf-dataset` — `.compound-engineering/artifacts/research/applicability/cards/paper-named-graphs-rdf-dataset.md`
- `paper-worlddb-worlds-edge-programs` — `.compound-engineering/artifacts/research/applicability/cards/paper-worlddb-worlds-edge-programs.md`
- `paper-graph-tenant-isolation` — `.compound-engineering/artifacts/research/applicability/cards/paper-graph-tenant-isolation.md`

WorldDB: nested worlds + write-time edge programs — Merkle worlds are a **materialization**, not a second SoT. Named graphs: slice id on one fold. Tenant isolation: quotas/noisy-neighbor, not crate layout.

## Decision

### D020-1. Workspace = compile-time Space

Target shape (STCA guide): `app-shell` (sole composition root) + `common` + `modules/<slice>` with `domain/` / `internal/` / `adapters/` / Ports in `lib.rs`. Hot path has **no mandatory network adapters**. TypeScript/Cordis-like SDK is agent I/O only (D6).

### D020-2. Isolation laws (AI-compiler gates)

1. **Domain ignorance** — `domain/` without sqlx/ORM/HTTP.
2. **Private-by-default** — only Ports leave the crate (`pub(crate)` otherwise).
3. **Single composition root** — only `app-shell` wires concrete adapters.

Repo-wide `ports/` / `adapters/` folders as the *primary* structure are rejected (STRATEGY).

### D020-3. Runtime Space is not this crate map

Named-graph IRIs, tenant banks, and WorldDB-style nested packs are **data slices of the log** (080 / 021). A crate is not a tenant. Geo-Raft is not crate isolation.

**Hard separations:**

```text
Cargo crate            ≠  Named graph / tenant
Port trait             ≠  Event log
app-shell wiring       ≠  Pack activate (ADR-021)
Merkle world hash      ≠  SoT
Hexagon-inside-slice   ≠  Repo-wide ports/ folder
```

## Consequences

### Positive

- Agents cannot import materializer internals as “just a helper.”
- Legal/science packs become crates without forking the runtime.

### Negative / risks

- Over-slicing before P0 spike. P0 may be `common` + one runtime crate.

### Non-goals

- Shipping the workspace. Runtime pack install (021). ABAC (080).

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Repo-wide hexagonal folders | Low cohesion (Drotbohm / STRATEGY) |
| TypeScript as graph core | D6 |
| One crate per tenant | Tenancy is a fold slice (080) |
| WorldDB Merkle as SoT | Log wins (D1) |

## Open Research Questions

1. P0 crate split vs monolith-until-spike.
2. How named-graph IRIs appear in Port methods without leaking storage.
3. Whether Cordis-like pack SDK lives in `app-shell` or a `sdk/` crate.

## Related Decisions

- ADR-002 — STCA Space
- ADR-021, ADR-022, ADR-080
