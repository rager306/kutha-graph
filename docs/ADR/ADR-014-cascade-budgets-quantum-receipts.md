# ADR-014: Cascade Budgets & Quantum Receipts

## Status

**Proposed** (Time/Composition/Verify — bound the quantum; constant-size evidence per emit→idle)

## Date

2026-08-18

## Honeycomb coordinates

- Axes: **Time** (primary) · **Composition** · **Verify**
- Depends on: ADR-000 (D2, D3, D9), ADR-002 (Cui), ADR-010
- Anticipates: ADR-030 (budget traits), ADR-031 (scheduling under V — **not opened here**), ADR-050 (meta-prompt version on receipts), ADR-091 (portable witness)

## Context

ADR-010’s cascade can storm. STCA already names modules as generalized knapsack items whose feasible envelope is **max-convolution**. Separately, regulated workflows need **one evidence item per quantum**, not a syslog sidecar and not a public chain as SoT.

Grounding cards:

- `paper-max-convolution-budgets` — `.compound-engineering/artifacts/research/applicability/cards/paper-max-convolution-budgets.md`
- `paper-constant-size-evidence` — `.compound-engineering/artifacts/research/applicability/cards/paper-constant-size-evidence.md`
- `ruvector-proof-gate` — `/root/vendor-source/ruvector/crates/ruvector-proof-gate`
- `ruvector-retrieval-receipt` — `/root/vendor-source/ruvector/crates/ruvector-retrieval-receipt`

Max-convolution / tropical `(max,+)` composes pack budgets: CSR rebuild vs HNSW vs agent vs materializer share envelope \(V\). Admission of *one* query is a different noun (gate vs envelope). Constant-size cryptographic evidence (Kao; Crosby–Wallach log ADS) is the literature shape of **quantum receipts**: fixed-size tuple, hash/Merkle link, optional epoch anchor. Graph ADS / blockchain papers are **anchors**, not SoT (`paper-blockchain-graph-ads` contrast).

## Decision

### D014-1. One receipt per quantum, not per vendor span

Each completed (or budget-aborted) emit→idle quantum emits a **constant-size** evidence item: commitments to admitted events, I/O, environment digest, link into the log’s hash/Merkle structure, authenticator. Size does not leak payload. This certifies **that the engine quantum existed**. It is not how-provenance (ADR-011) and not an LLM envelope certificate.

### D014-2. Budgets compose by max-convolution, not a workflow engine

Cascade/agent/materializer local functions \(h_i(v)\) compose by tropical max-convolution into a global envelope. The LLM does not pick \(v\). Hard FSM is not the allocator (D3). Pack scheduling *under* a fixed \(V\) (competition among materializers) is **ADR-031**, out of this cell.

### D014-3. Anchor ≠ SoT

Optional epoch root-of-roots (Merkle, transparency log, even a chain) may **anchor** receipts. The semantic event log remains SoT; Rocks WAL is its storage-durability cousin (ADR-010), not another source of semantic truth. Public blockchain as graph store is rejected.

### D014-4. Receipt may bind control-plane versions

A receipt may include `meta_prompt_version` and dictionary snapshot ids (ADR-050). This cell does not define those entities. Content-addressed LLM/tool cache keys (D6) belong in the quantum’s evidence so replay is cheap.

### D014-5. Read provenance receipts (ruvector-retrieval-receipt adapter)

While D014-1 certifies write quanta (emit→idle cascades), regulated agent reasoning also requires tamper-evident read commitments: binding the exact evidence subset handed to the agent during a query. Borrow `ruvector-retrieval-receipt` Merkle receipts (`RetrievalReceipt::Merkle(query_hash, index_root, results)`). Read receipts detect post-issuance evidence mutation in agent workflows and link to the action justification (ADR-052) without making retrieval a secondary write SoT.

### Clarification (2026-09-13): partial progress and recoverable outcomes

The proposed contract permits prefix commit; it does not promise rollback on budget exhaustion. An outcome must distinguish completed, budget-stopped, and failed execution, and bind the input cut, committed range, rule/environment versions, consumed budget, and continuation disposition. A constant-size commitment may reference larger evidence; it does not make that evidence optional or constant-size. The P0 receipt is not yet the full cryptographic contract above.

Completion evidence and enough information to reconstruct or explicitly reject continuation must be authoritative, not available only in a dropped receipt/cache. If a crash leaves no terminal evidence, recovery reports incomplete/unknown, never inferred success. Resumption must identify the original quantum and prevent duplicate delivery/effects (ADR-011/062); exact record encoding remains a future implementation choice.

Current `Runtime::emit` returns `Ok(QuantumOutcome)` even when its receipt says budget-aborted, may retain a committed prefix, and `store::persist` does not persist that receipt. Call success is therefore not quantum completion. Future budget-0/1/2 and crash-boundary fixtures must test this distinction before stronger guarantees are claimed.

**Hard separations:**

```text
Quantum receipt (write) ≠  Retrieval receipt (read evidence)
Quantum receipt        ≠  How-polynomial (ADR-011)
Envelope V             ≠  Per-query admission gate
Max-convolution        ≠  Airflow / Dify DAG
Merkle / ADS anchor    ≠  Event-log SoT
Budget abort           ≠  Silent truncate of truth
Hard FSM allocator     ≠  Cui envelope
```

## Consequences

### Positive

- Replay/fork (D9) has a sealed per-quantum handle.
- Materializers (040) can be scored as knapsack items.
- Legal/clinical/finance Layer 5 on the evidence card is GTM color, not a requirement to implement 37 niches.

### Negative / risks

- Convolution is hard in general; P0 may use a concave/greedy envelope, not the full algorithm.
- Binding meta-prompt versions before 050 exists needs a placeholder field.

### Non-goals (this ADR)

- Implementing PQ signatures or TEE.
- Opening ADR-030/031 files.
- Blockchain as database.

## Alternatives Considered

| Alternative | Why not |
|-------------|---------|
| Blockchain graph ADS as SoT | Anchor only |
| Workflow-engine orchestration | Fights reversible packs (STRATEGY) |
| Syslog sidecar as proof | Not constant-size, not bound to log offset |
| LLM picks budgets | Violates dict-first / Cui |

## Open Research Questions

1. P0 receipt layout: which hashes are mandatory vs optional authenticator.
2. Greedy/concave envelope vs full max-convolution for P0.
3. How ADR-050 versions appear on the tuple without circular crate deps.
4. Relation to RVF WitnessChain (ADR-091) — packaging of the same hashes, not a second log.

## Related Decisions

- ADR-002 — Cui composition
- ADR-010 — quantum definition
- ADR-011 — polynomials are different evidence
- ADR-040 — materializers consume budget
- ADR-050 — versions bound into receipts
