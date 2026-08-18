# ADR-031: Pack Scheduling Under V

## Status

**Proposed** (Composition — competition among packs for a *fixed* envelope; not admission-as-tenancy)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Composition** (primary) · **Query** · **Security**
- Depends on: ADR-014, ADR-030
- Anticipates: ADR-043 (cost envelopes), ADR-080 (tenant quotas ≠ this scheduler)

## Context

Synthesis hole: admission (CASA-class) ≠ competition among materializers/agents for the same cascade budget. 030 *builds* envelope \(V\). This cell **allocates leftover V** and **admits or rejects** a request before leapfrog.

Grounding cards:

- `paper-query-admission-control` — `.compound-engineering/artifacts/research/applicability/cards/paper-query-admission-control.md`
- `paper-max-convolution-budgets` — `.compound-engineering/artifacts/research/applicability/cards/paper-max-convolution-budgets.md`

CASA/SafeLoad/Bouncer: accept/queue/reject from predicted slots, memory, or SLO — before execution. Banyan: subquery grain. Tenant isolation (080) is data slice + noisy-neighbor quota, not “this MATCH.” CardEst (043) is a statistic feeding the envelope, not the gate.

## Decision

### D031-1. Admit is a typed outcome, not a silent drop

After compile, compare cost envelope to remaining \(V\). Outcomes: run / queue / reject. Reject is a **logged why-not** (query not run), not a hole in the fold. LLM is not the governor.

### D031-2. Scheduling under V ≠ building V

030/014 compose \(h_i\). 031 assigns concurrent CSR rebuild vs HNSW vs agent vs MATCH a share of leftover V (preemptive or queued). Starvation-avoidance is in-scope as research; hard FSM workflows are not.

### D031-3. P0 may be unlimited

Honeycomb: slot/SLO check. Do not train a learned governor as P0.

**Hard separations:**

```text
Admit this query       ≠  Tenant slice (080)
Schedule under V       ≠  Construct V (030)
Queue/reject           ≠  Vacuum of SoT
Subquery isolation     ≠  WASM sandbox (081)
```

## Consequences

### Positive

- Legal PIT cannot let one AS-OF burn the box.
- Materializer storms (021 activate) have a gate.

### Negative / risks

- Warehouse papers are not graph-shaped; Banyan is the cousin.

### Non-goals

- Per-tenant Postgres. LLM SLO picker.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Global SERIALIZABLE on the log as the scheduler | Wrong noun (Ddi is per-op on compiled query) |
| Dify queue as STCA | Trap |
| Collapse into 014 | 014 is receipt+envelope, not competition |

## Open Research Questions

1. Fair share among packs vs priority classes (agent vs hop vs compact).
2. Subquery grain vs whole-statement admit.
3. Interaction with 080 tenant CPU reservations.

## Related Decisions

- ADR-014, ADR-030, ADR-043, ADR-080
