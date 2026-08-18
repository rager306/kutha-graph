# ADR-030: Cui Budget Traits / Max-Convolution Allocation

## Status

**Proposed** (Composition — trait-level envelope; 014 names the quantum receipt)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Composition** (primary) · **Time** · **Query**
- Depends on: ADR-002, ADR-014
- Anticipates: ADR-031 (who runs *under* a fixed V)

## Context

ADR-014 already decided: cascade budgets compose by **max-convolution**; one receipt per quantum. This cell is the **trait/algebra home**: what \(h_i(v)\) is, who implements it, how MQO shares work across queries without becoming a workflow engine.

Grounding cards:

- `paper-max-convolution-budgets` — `.compound-engineering/artifacts/research/applicability/cards/paper-max-convolution-budgets.md`
- `paper-sparql-multi-query-opt` — `.compound-engineering/artifacts/research/applicability/cards/paper-sparql-multi-query-opt.md`

Admission of *one* query (CASA) is **031/gate**, not this trait. Cardinality (043) estimates cost of one plan; convolution allocates **across concurrent packs**.

## Decision

### D030-1. Packs expose \(h_i(v)\) as a Port

Each materializer/agent/pack is a generalized knapsack item: local value at budget \(v\). Global envelope is tropical max-convolution (Cui / STCA guide Algorithm 3). LLM does not pick \(v\). Hard FSM is not the allocator.

### D030-2. P0 may use a concave/greedy envelope

Full convolution is hard (quadratic barrier). Research cell allows greedy/concave approximation until a spike falsifies it. Do not pretend CNN “convolution.”

### D030-3. Multi-query sharing is composition, not a DAG product

SPARQL MQO-class sharing of common subpatterns is allowed as *work sharing under V*, not Dify/oxify orchestration.

**Hard separations:**

```text
Envelope V             ≠  Per-query admit (031)
h_i(v) trait           ≠  Quantum receipt layout (014)
MQO work-share         ≠  Workflow engine
Max-convolution        ≠  Join semiring (LFTJ)
```

## Consequences

### Positive

- 014 receipts have an algebra home.
- 031 can compete for a *named* leftover budget.

### Negative / risks

- Traits without measured \(h_i\) stay fiction.

### Non-goals

- Implementing DP knapsack. Opening 031’s scheduler.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| YAML quotas as STCA | Not Cui |
| Airflow/Dify as allocator | STRATEGY |
| LLM governor | D3 |

## Open Research Questions

1. Units of \(v\): events, CPU-ms, RAM, hop-count.
2. How 043 cardinality feeds \(h_i\).
3. Concave restriction vs piecewise-linear tables.

## Related Decisions

- ADR-002, ADR-014, ADR-031, ADR-043
