# ADR-010: Event Log & Reactive Runtime Quantum

## Status

**Proposed** (Time-band physics — log as SoT and one emit→idle quantum; not product runtime)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Time** (primary) · **Composition** · **Verify** · **Agent**
- Depends on: ADR-000 (D1, D2, D9), ADR-001, ADR-002
- Anticipates: ADR-011 (lean events), ADR-012 (snapshots/vacuum), ADR-014 (budgets & receipts), ADR-040 (leases of the fold), ADR-060/061 (replay/fork)

## Context

Kutha’s spine already locks **the append-only event log as source of truth** and **the graph as a deterministic fold** (ADR-000 D1/D2). Honeycomb still lacked a cell that names the *runtime quantum* and the crash cousin without turning vendor stores into SoT.

Grounding cards (SoT for evidence, not for decisions):

- `paper-activegraph-log-is-sot` — `.compound-engineering/artifacts/research/applicability/cards/paper-activegraph-log-is-sot.md`
- `paper-yankin-event-sourced-query` — `.compound-engineering/artifacts/research/applicability/cards/paper-yankin-event-sourced-query.md`
- `paper-tgms-operators` — `.compound-engineering/artifacts/research/applicability/cards/paper-tgms-operators.md`
- `rocksdb-wal-recovery` — `.compound-engineering/artifacts/research/applicability/cards/rocksdb-wal-recovery.md`

ActiveGraph inverts agent stacks: log first, working graph = deterministic projection, behaviors emit events. Yankin names query *mechanisms* over event-sourced state (reconstruction, temporal, cross-stream, retroactive replay) with cost envelopes — not a second SoT. TGMS supplies typed write operators with the LLM **outside** the trust boundary. RocksDB WAL is durability/crash recovery of *storage*, not the semantic log.

We need this cell so later ADRs can cite one quantum instead of restating D1.

## Decision

### D010-1. Log is SoT; fold is a picture

The append-only event log is the only source of temporal truth. The property graph, CSR, HNSW, interval indexes, named-graph slices, and receipts are **droppable leases** of a fold at a log offset. Unloading a lease must not change history. LLM compiles or proposes; it never writes truth.

### D010-2. Runtime quantum = emit → cascade → idle

One quantum is: admit an event (or a validated patch) → append to the log → project into subscribed leases → trigger relation/behavior reactions \(B\) / \(R_B\) → cascade until idle (or until a budget bound from ADR-014 fires). No component instructs another except **via** logged events. Direct mutate of the fold is forbidden.

### D010-3. Typed writes, not free MERGE-as-truth

Write surface is a closed operator family in the TGMS direction: **assert / retract / correct** (plus later schema/dict events). Invalidation is a behavior, not silent overwrite (deepened in ADR-013). Query-over-log uses Yankin mechanism groups; AS OF / replay / reconstruction are named cuts with cost envelopes, not marketing synonyms.

### D010-4. WAL is the crash cousin, not the semantic log

RocksDB (or equivalent) WAL + `WALRecoveryMode` is how bytes survive a crash. Semantic replay is fold-from-log. A recovered LSM is a materialization of storage WAL; it is not permission to treat LSM levels as SoT.

**Hard separations (must fail closed if collapsed):**

```text
Event log              ≠  Fold / working graph
Semantic replay        ≠  Storage WAL recovery
Typed assert/retract   ≠  LLM narrative write
Quantum (emit→idle)    ≠  Workflow-engine DAG
Lease at log offset    ≠  Second database
```

## Consequences

### Positive

- Replay, fork-at-offset, and lineage have a single spine.
- Materializer ADRs (040+) can mount without competing for SoT.
- Agent control (050) has a place to append *after* validate.

### Negative / risks

- Naive “replay from genesis every query” is too slow — needs ADR-012 snapshots as *placement*, not as truth.
- Unbounded cascades without ADR-014 budgets become storms.

### Non-goals (this ADR)

- Choosing a mandatory crate API or shipping `src/`.
- Defining lean event fields (ADR-011) or vacuum policy (ADR-012).
- Making Graphiti / ActiveGraph / RocksDB the product SoT.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Graphiti fact-edges as SoT | Trap cluster; LLM on write path; clocks collapse (`graphiti-bitemporal-fact-edges`) |
| Samyama / Neo4j / Falkor as SoT | Fast pictures, not the log; violates D1/D5 |
| CRDT merge as truth | Eventual merge drops losers; not audited SoT (`paper-crdt-graph-eventual`) |
| Geo-Raft / blockchain ledger as SoT | Replication or ADS anchor ≠ semantic log (`paper-geo-raft-wan`, `paper-blockchain-graph-ads`) |
| Hard FSM as the quantum | Rejected by D3; quantum is log+behaviors, not a per-vertical statechart |

## Open Research Questions

1. Bound of one quantum: event count vs wall time vs Cui envelope (ADR-014) — falsifiable spike, not a crate name.
2. Yankin cost envelopes for graph-shaped reconstruction vs snapshot-shortened replay (JOB-class later).
3. Operator IR for assert/retract/correct vs Cypher write subset (ADR-070).
4. Content-addressed cache keys for LLM/tool calls inside a quantum (D6) — required for cheap replay; encoding in ADR-011/014.

## Related Decisions

- ADR-000 — D1 log SoT, D2 reactive quantum, D9 tests = replay/fork
- ADR-001 — vision: log is truth; materializations are pictures
- ADR-002 — STCA time axis
- ADR-011 — lean event schema
- ADR-012 — snapshots vs vacuum
- ADR-014 — cascade budgets and quantum receipts
