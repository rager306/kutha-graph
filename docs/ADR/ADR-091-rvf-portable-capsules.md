# ADR-091: RVF Portable Capsules

## Status

**Proposed** (Packaging — sealed export/import; **not** hot SoT; card is med/med/med)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Packaging** (primary) · **Time** · **Verify**
- Depends on: ADR-000 (D5), ADR-012, ADR-014, ADR-061
- Anticipates: ADR-090/093 export of edition slices

## Context

ADR-000/001 already reject RVF as primary storage. RuVector CowEngine freeze + WitnessChain is `confidence: code` with marketing-heavy surface. Constant-size evidence (014) is the *engine* receipt; RVF is a **portable container** that may carry those hashes.

Grounding cards:

- `ruvector-rvf-cow-seal` — `.compound-engineering/artifacts/research/applicability/cards/ruvector-rvf-cow-seal.md`
- `paper-constant-size-evidence` — `.compound-engineering/artifacts/research/applicability/cards/paper-constant-size-evidence.md`

ADR-090 already: RVF capsule ≠ hot SoT; packaging ≠ storage.

## Decision

### D091-1. RVF is transport / freeze, not the log

Hot path remains log + Rocks + leases. Capsules export a fold slice (named graph, branch ref, snapshot epoch) with optional witness chain. Import is ingest-as-events (or attach as a read-only lease), never “the capsule is now SoT.”

### D091-2. Witness in RVF ≠ quantum receipt algebra

014 owns per-quantum constant-size tuples. RVF may embed those tuples / Merkle roots. Do not invent a second receipt system.

### D091-3. Subset of segments is still open

ADR-000 R8: which of WITNESS, VEC, GRAPH, META, WASM in v1. This cell does not freeze the subset; Open Research Questions own it. Do not vendor the whole RuVector monorepo (STRATEGY).

**Hard separations:**

```text
RVF capsule            ≠  Event-log SoT
COW freeze             ≠  Vacuum
WitnessChain           ≠  014 algebra (may embed it)
Fork export            ≠  CRDT merge
```

## Consequences

### Positive

- Legal/science sealed export has a pack.
- 061 forks can leave the box.

### Negative / risks

- med/med/med card — do not over-invest before P0 physics.
- Marketing “cognitive container” language.

### Non-goals

- RVF as database. Full segment matrix.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| RVF-primary store | D5 / STRATEGY |
| Blockchain capsule as SoT | Anchor only |
| Skip ADR (leave planned) | User asked remaining cells opened |

## Open Research Questions

1. v1 segment subset (R8).
2. Mapping: graph fork ↔ RVCOW; event segment ↔ file.
3. Schema bridge: dict@version + log offset inside META.

## Related Decisions

- ADR-012, ADR-014, ADR-061, ADR-090, ADR-093
