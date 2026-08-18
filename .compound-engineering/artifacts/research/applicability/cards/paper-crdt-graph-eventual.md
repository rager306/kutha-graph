---
id: paper-crdt-graph-eventual
source: paper
axes: [Time, Composition, Space]
usefulness: med
optimality: low
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# CRDT graphs: strong eventual consistency is not an event-log SoT

Papers: [Conflict-Free Replicated Data Types](https://consensus.app/papers/details/7f88eff91e1a5133844c2229ab446cd8/?utm_source=cursor) (Shapiro et al., 2011, SSS, DOI: 10.1007/978-3-642-24550-3_29) — includes a Graph CRDT; [A comprehensive study of CvRDTs](https://consensus.app/papers/details/f1d8ce4587bd5e38b579ed8644bf3a4d/?utm_source=cursor) (Shapiro et al., 2011 INRIA RR-7687); overview [Conflict-free Replicated Data Types: An Overview](https://consensus.app/papers/details/e46ce1818efd50fc8e5563c154842cb9/?utm_source=cursor) (Preguiça, 2018, arXiv:1806.10254, DOI: 10.48550/arxiv.1806.10254). Graph-shaped: [DAG CRDTs](https://consensus.app/papers/details/03ac4f6ed6075277918ed40a68a6c44f/?utm_source=cursor) (Borth et al., 2025); [hypergraph CRDTs](https://consensus.app/papers/details/ed0d9076d20e530291b07df87d8f41c8/?utm_source=cursor) (Bansal, 2022). Cousin that keeps a log: [Log-Based CRDT for Edge Applications](https://consensus.app/papers/details/1995eb9113ad5a10a8a72c5e1e93243e/?utm_source=cursor) (Saquib et al., 2022). Distinct from `paper-toki-contradiction-ops` (isolation-typed merge + audit losers), `samyama-raft-ha` (linearizable quorum), `paper-mas-isolation-lattice`.

## 1. Raw idea

Replicas accept updates without coordination; when they have seen the same set of updates they converge (Strong Eventual Consistency). State-based: join-semilattice + inflate + merge=LUB. Operation-based: concurrent effectors commute (causal delivery). Shapiro studies a **Graph** CRDT (add/remove vertices and edges with clean set semantics). Preguiça: add-wins vs remove-wins vs LWW sets; arbitration can be **unstable** (a discarded write becomes live again); Bounded Counter escrow for numeric invariants; Antidote/SwiftCloud wrap CRDTs in highly available transactions. CALM: queries over CRDT state are unconstrained and unsafe unless monotonic. DAG CRDTs add compensation when cycles appear.

## 2. STCA applicability

Time: CRDT merge **throws away losers** (or resurrects them unpredictably). Kutha SoT is the event log; TOKI keeps losers as audit rows with a declared isolation operator. A CRDT replica state is at best a **fold/view**, never the quantum. Composition: op-based CRDTs need exactly-once causal delivery — that is a replication pack, not Control SoT. Space: geo/edge availability is real; Raft (already closed) is the linearizable pole; CRDT is the AP pole. Log-based CRDT (undo, no exactly-once, version history) is the only cousin that rhymes: **log first, merge as a view**. Do not adopt add-wins as legal invalidation.

## 3. Quality / cost

Usefulness med: collab/edge apps will ask for “offline sync.” Optimality low for temporal truth: SEC ≠ PIT; LWW is the silent overwrite TOKI forbids; Byzantine-ready “reliable CRDTs” (Janus) reintroduce consensus and kill the performance story. Cost: if Kutha ever fans out, replicate the **log** (Rocks WAL family) and materialize per-replica folds; CRDT graph objects are a warning label, not a kernel type.

## 4. Demand

Yjs/Automerge/AntidoteDB mindshare is high. Engine demand: explain why eventual graph merge is not AS-OF. Product demand: optional AP replica pack with explicit “not auditable until quorum/log catch-up.”

## 5. Niche → effect

`no niche`
