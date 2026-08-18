# ADR-002: Spatiotemporal Compositional Architecture (STCA)

## Status

**Proposed** (paradigm ADR — normative frame for honeycomb cells; not a full runtime implementation spec)

## Date

2026-08-16

## Context

Kutha’s hybrid thesis (ADR-000) already cites ActiveGraph (temporal), Cordis (spatiotemporal packs), and Cui (composition). The STCA manifesto synthesizes these into one named paradigm so Cargo layout, behaviors, budgets, and tests share vocabulary.

ADR-001 places STCA at the **top of the idea stack**. This ADR defines STCA itself.

## Decision

Adopt **Spatiotemporal Compositional Architecture (STCA)** as Kutha’s architectural paradigm:

```
[ SPACE AXIS ]                         [ TIME AXIS ]
Vertical domain slices + Ports         Append-only Event Log (SoT)
(Cockburn / onion / clean +            ActiveGraph-style fold
 Drotbohm vertical slices)             Behaviors / Relation Behaviors
         \                                    /
          \                                  /
           ---- Spatiotemporal Context ----
                         ^
                         |
              [ COMPOSITION BRIDGE ]
         Packs as Cui generalized items
              max-convolution of budgets
```

### Space axis

- Decompose by **vertical slices** (feature/domain modules), not by technical layer folders alone.
- Hexagonal / onion structure lives **inside** a module; modules talk only through **Ports**.
- Packs register into a shared context; effects should be **reversible** where they touch projections or agent side effects (Cordis-aligned).

### Time axis

- **Event log = sole SoT.** Graph state is \(G_t = \mathrm{foldl}(\mathrm{apply}, G_0, L_t)\).
- No direct graph mutate; only patch → validate → append.
- **Behaviors** and **Relation Behaviors** react and emit further events until idle or budget exhausted.
- Principle (Nakajima): graph = world; behaviors = physics; log = proof.
- Identifiers: **UUID v7** for monotonic event ordering.

### Composition bridge (Cui)

- Each pack \(M_i\) exposes a local value function \(h_i(v)\) under resource budget \(v\) (cascade depth, CPU, agent tokens, etc.).
- Global allocation uses **max-convolution** (Cui knapsack lecture 8) rather than a hard-coded workflow engine.
- Maps to Kutha ideas: cascade budgets, competing materializer packs, agent tool budgets.

### Verification stance (paradigm-level)

STCA expects deterministic verification styles:

- **Strict replay** of historical logs (external I/O via content-addressed cache).
- **Fork-and-diff** from an event id for counterfactuals.
- Optional **Regimes** gated loops for agent seam updates (held-out must not regress).

Detail algorithms and Cargo isolation laws: `docs/architecture/stca-guide.md`.

### Kutha deltas that STCA does *not* replace

STCA alone is not the full product. Honeycomb cells still must specify:

- Hot reversible materializations (CSR/HNSW/…) — ADR-000 D5 / cell ADR-040+
- Native bi-temporal fact semantics (Graphiti *reference*, native Rust) — cell ADR-013 class
- Dict-first agent control — ADR-000 D3 / cell ADR-050+
- Self-contained core (no mandatory external graph/LLM for temporal truth) — vision ADR-001

## Consequences

### Positive

- Shared language for humans and coding agents.
- Compile-time isolation story (workspace modules + ports).
- Budgets and packs become first-class, not afterthoughts.

### Negative / risks

- Over-formalizing Cui before P0 runtime exists — keep convolution as *allocation idiom*, implement fully when packs compete.
- Confusing STCA with “must ship ActiveGraph code” — we reimplement the *model*, not vendor runtimes.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Clean/hexagonal only | Missing event-log SoT and reactive behaviors |
| ActiveGraph only | Missing vertical-slice product modularity and hot data plane |
| Workflow-engine orchestration | Fights pack composability and reversible effects |

## Open Research Questions

- Formal API for pack budget functions \(h_i\) in Rust traits.
- Default cascade budget policy for P0 (serial single-writer first?).
- How relation-behavior incidence checks generalize beyond task_id examples in the manifesto sketch.

## Related Decisions

- ADR-000 D1–D2, Cordis/Cui/ActiveGraph citations
- ADR-001 overall vision (STCA on top of idea stack)
- Honeycomb cells ADR-010+ (runtime, schema, temporal, materialization, …)
- `docs/architecture/stca-guide.md`
