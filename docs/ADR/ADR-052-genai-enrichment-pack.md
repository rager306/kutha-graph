# ADR-052: GenAI Enrichment as Optional Pack

## Status

**Proposed** (Agent — optional derived content; never on the trust core write path as SoT)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Agent** (primary) · **Composition**
- Depends on: ADR-010, ADR-050, ADR-040
- Anticipates: ADR-071 (retrieve of derived embeddings), ADR-081 (UDF sandbox)

## Context

STRATEGY: agents and GenAI live *inside* the runtime as packs, not bolted ETL. Samyama GAK and Raven in-DB agents are the pattern: enrichment is an **add-on**. LLM-compiler-not-executor: asserted results need an execution; enrichment writes *derived* events that remain invalidatable.

Grounding cards:

- `samyama-agentic-enrichment-gak` — `.compound-engineering/artifacts/research/applicability/cards/samyama-agentic-enrichment-gak.md`
- `raven-in-db-ai-agents` — `.compound-engineering/artifacts/research/applicability/cards/raven-in-db-ai-agents.md`
- `paper-llm-compiler-not-executor` — `.compound-engineering/artifacts/research/applicability/cards/paper-llm-compiler-not-executor.md`

Contrast: Dify/oxify DAG, Hindsight four-network, Graphiti memory, Harvey-as-SoT.

## Decision

### D052-1. Enrichment is a reversible pack

LLM extract/summarize/embed runs as ADR-021 pack. Outputs are events (`enrichment.proposed` / applied after validate). Content-addressed cache of model calls (D6) is mandatory for replay. Unload drops derived leases (HNSW of embeddings, etc.), not the log.

### D052-2. Dual-process ingest (ADR-090 TR-07 cousin)

Hot write of admitted evidence does **not** require LLM. Enrichment is async. Same-turn read of raw evidence must work.

### D052-3. Derived ≠ kernel fact

Enrichment cannot override L_KB / statutory facts. MemStrata still owns supersession. ULTRA/GNN scoring is a retrieve lease (071), not MATCH.

**Hard separations:**

```text
Enrichment event       ≠  Kernel assert
Optional pack          ≠  Mandatory cloud LLM
CA cache               ≠  Model as SoT
Raven/Samyama pattern  ≠  Vendor as Kutha core
Dify DAG               ≠  This pack
```

## Consequences

### Positive

- In-DB agents without making Claude the log.
- 071 can retrieve derived vectors as a lease.

### Negative / risks

- Sync extract on the write path (Memanto D6) kills OLTP profile A.

### Non-goals

- Shipping an LLM. GNN packs. Workflow-engine product.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Graphiti/Hindsight as enrichment SoT | Traps |
| Enrichment on every assert | Violates dual-process |
| External ETL+orchestrator as target | STRATEGY |

## Open Research Questions

1. Which derived types are first (embeddings vs proposed edges vs summaries).
2. How 014 receipts bind model+prompt cache keys.
3. Fail-closed schema of enrichment payloads vs free text nodes.

## Related Decisions

- ADR-050, ADR-021, ADR-042, ADR-071, ADR-081
