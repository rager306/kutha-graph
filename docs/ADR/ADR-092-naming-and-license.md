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

### D092-2. This repository is Apache-2.0

Owner choice, 2026-10-24. The engine and the harness in this tree are Apache License 2.0 (`LICENSE`, SPDX `Apache-2.0`). Cargo workspace metadata and `pyproject.toml` carry the same SPDX id.

Pack and ontology licenses stay open until a pack exists. They are not a second license on this tree. Secrets and third-party code must not be relicensed by this cell. Status stays **Proposed**: choosing SPDX is not an engine acceptance.

### D092-3. Trademark / clearance is a later human process

Not an agent task. Do not treat this ADR as clearance.

**Hard separations:**

```text
Working name Kutha        ≠  Registered mark
Apache-2.0 on this tree  ≠  A pack or ontology license
Pack license             ≠  Event-log SoT
```

## Consequences

### Positive

- R10 has a honeycomb home.
- Agents stop inventing Consensus queries for naming.

### Negative / risks

- Pack and ontology SPDX ids are still open, so a future pack is not automatically Apache-2.0.

### Non-goals

- Filing trademarks. Changing `kutha-graph` remote. Dual-license bikeshed in this session.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Rename to kutkha in this cell | STRATEGY/ADR-001 already say Kutha; rename is a product decision |
| Copy AGPL “because graphs” | No evidence; wait for human counsel |
| Skip the ADR | Planned title would remain a ghost |

## Open Research Questions

1. SPDX for a future pack, if it is not Apache-2.0.
2. Whether ontology packs (090/093) need a different license than the engine.
3. Trademark search owner and timing (human).

## Evidence (2026-10-24)

The owner set the repository license to Apache-2.0. `LICENSE` is the Apache License 2.0 text. The cell stays **Proposed**.

## Related Decisions

- ADR-000 R10
- ADR-001
- `STRATEGY.md`
