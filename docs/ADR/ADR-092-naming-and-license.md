# ADR-092: Naming & License

## Status

**Proposed** (Packaging — brand and license questions; not a graph capability; **does not rename or file trademarks**)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Packaging** (primary)
- Depends on: ADR-000 (R10), ADR-001, `STRATEGY.md`
- Anticipates: none

## Context

research-boundary: ADR-092 is **not a Consensus noun**. ADR-000 R10 lists kutkha vs kutha and license for core vs packs. STRATEGY already ships the product name **Kutha**. This cell records the research questions so they are not silent.

No applicability card is required (KD3 exception in the remaining-graduation plan).

## Decision

### D092-1. Working product name remains Kutha

`STRATEGY.md` and ADR-001 use **Kutha**. This ADR does not execute a rename. Historical “kutkha” remains a dialogue leftover, not an instruction to retitle the repo.

### D092-2. License is undecided (core vs packs vs ontologies)

Split under consideration (research only): engine core vs agent packs vs legal/science ontology packs. No SPDX choice is Accepted here. Secrets and third-party code (Samyama/RuVector adapters) must not be relicensed by this cell.

### D092-3. Trademark / clearance is a later human process

Not an agent task. Do not treat this ADR as clearance.

**Hard separations:**

```text
Working name Kutha     ≠  Registered mark
License research       ≠  Chosen SPDX
Pack license           ≠  Event-log SoT
```

## Consequences

### Positive

- R10 has a honeycomb home.
- Agents stop inventing Consensus queries for naming.

### Negative / risks

- Leaving license open can block downstream distribution.

### Non-goals

- Filing trademarks. Changing `kutha-graph` remote. Dual-license bikeshed in this session.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Rename to kutkha in this cell | STRATEGY/ADR-001 already say Kutha; rename is a product decision |
| Copy AGPL “because graphs” | No evidence; wait for human counsel |
| Skip the ADR | Planned title would remain a ghost |

## Open Research Questions

1. SPDX for core vs vertical packs.
2. Whether ontology packs (090/093) need a different license than the engine.
3. Trademark search owner and timing (human).

## Related Decisions

- ADR-000 R10
- ADR-001
- `STRATEGY.md`
