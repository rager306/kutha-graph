# ADR-062: Regimes Gated Loop (Optional)

## Status

**Proposed** (Verify/Agent — **optional, default off**; not P0 physics)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Verify** · **Agent** · **Composition**
- Depends on: ADR-050, ADR-060
- Anticipates: none required

## Context

STRATEGY: “Regimes gated loop optional later.” research-boundary: not a Consensus noun. This cell exists so the title is not a ghost. It must not become a hard FSM (D3) or a Dify DAG.

Grounding cards (cousins, not a dedicated regimes paper):

- `paper-cordon-semantic-tx` — `.compound-engineering/artifacts/research/applicability/cards/paper-cordon-semantic-tx.md`
- `paper-kumiho-agm-revision` — `.compound-engineering/artifacts/research/applicability/cards/paper-kumiho-agm-revision.md`

Cordon: semantic transaction for tool effects (commit/rollback of *effects*). Kumiho: AGM-style revision — cousin of belief change, not a product loop.

## Decision

### D062-1. Default off

P0–P2 do not require a regimes loop. If enabled later, it is a **pack** that gates “enter regime R” using dictionaries (050) + receipts (014), not a second runtime.

### D062-2. Derived from dictionaries, not hard-coded

Regimes, if any, are snapshots of State/mode + Policy dictionaries (050 derived FSM). Tool effects still commit only via the log (Cordon-shaped: effects that bypass the log cannot roll back).

### D062-3. Not an orchestrator

No Airflow/Dify/oxify DAG as the loop. Cui budgets (030/031) still apply.

**Hard separations:**

```text
Optional regimes pack  ≠  Core quantum (010)
Derived FSM            ≠  Hard FSM sole control
Semantic tx of effects ≠  Workflow engine
AGM revision           ≠  This product loop
```

## Consequences

### Positive

- Title is closed; agents will not reopen it as a missing paper.
- Default-off protects P0 scope.

### Negative / risks

- Vague “regimes” language in marketing.

### Non-goals

- Implementing a loop. Consensus queries.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Hard FSM per vertical | D3 |
| Make 062 mandatory for agents | STRATEGY says optional later |
| Skip the ADR entirely | User asked to open remaining planned cells |

## Open Research Questions

1. Whether Cordon-style effect brackets are enough and 062 never ships.
2. Regime vs 050 mode dictionary — possible collapse later.

## Related Decisions

- ADR-050, ADR-014, ADR-010
