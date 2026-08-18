---
id: ruvector-hnsw-delete-repair
source: ruvector
axes: [Data, Query]
usefulness: high
optimality: med
demand: high
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# Heal the HNSW proximity graph after deletes — not PG-Constraint repair

CBM `root-vendor-source-ruvector` (coverage: `crates/ruvector-hnsw-repair/src/lib.rs`, `strategy.rs`, `graph.rs` — `no_recorded_issue`, best-effort). Crate crate-docs three deletion strategies. Cousin (ops, not this card’s SoT): `crates/ruvector-postgres/src/healing/engine.rs` `RemediationEngine::heal` (compact/rebalance/throttle with cooldown, approval, rollback). Distinct from `paper-pg-constraint-repair` (semantic denial-constraint deletes of *property-graph* nodes/edges/labels) and from `ruvector-hnsw` (insert/search/remove without neighbor rewiring).

## 1. Raw idea

Deletes punch holes in the HNSW *neighbor graph*. Tombstones skip deleted ids at visit time but leave stale edges, so recall drops (~10–25pp at 20% deleted, crate table). `repair_one` scans live nodes that still point at the dead id, drops the edge, and fills the slot from the dead node’s remaining live neighbours (closest L2, degree cap `m`/`m0`). Strategies: `TombstoneOnly` (O(1), no rewiring); `BatchRepair` (queue + periodic sweep — production default in the crate comments); `EagerRepair` (rewire on every delete; O(deg × n); meant for small agent memories). Tests: tombstone recall does not improve; eager/batch maintain recall vs brute-force live kNN. README “self reconstructing graph memory / shortcut edges” is a *different* claim (MRAgent); not this crate.

## 2. STCA applicability

Data/Query: this is **index hygiene on a lease**. The event log remains SoT; HNSW is a reversible projection (ADR-042). Healing restores ANN recall after invalidation, it does **not** restore PG-Keys/PG-Constraints. Do not confuse “graph repair” here with Fan/Grafixer/Spinrath: those mutate the *fact graph*; this rewires *approximate neighbor lists*. Postgres `RemediationEngine` is an ops FSM over the same family (detect → strategy → verify → maybe rollback) — still not semantic repair. Shortcut-edge “self reconstructing memory” would be a write to the fold if it ever becomes real; keep it a claim until a kernel path is read.

## 3. Quality / cost

Usefulness high: every HNSW with deletes needs a story or recall silently dies. Optimality med: `repair_one` is a linear scan of all live nodes per deleted id — fine for the research crate, not a production WAL-applied algorithm; BatchRepair is the right *shape*. Cost: borrow the **strategy split** (tombstone vs amortised vs eager) for Kutha HNSW leases; log delete+repair as projection maintenance, not as constraint fixes. Do not auto-apply PG-Constraint ILP deletes because RuVector “heals graphs.”

## 4. Demand

Agent memory and RAG indexes delete constantly. Operators will say “heal the graph” and mean this, or mean Grafixer. Answer: two verbs — **rewire the lease** vs **compensate a constraint violation on the log**.

## 5. Niche → effect

`no niche`
