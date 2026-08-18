---
id: paper-worlddb-worlds-edge-programs
source: paper
axes: [Space, Time, Composition]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Nested worlds + write-time edge programs (WorldDB)

Paper: [WorldDB](https://arxiv.org/abs/2604.18478) (Ganesan, 2026). Consensus: https://consensus.app/papers/details/651dffe80bf55eb8a38900889353c97f/?utm_source=cursor

## 1. Raw idea

Node = recursive world (interior subgraph + ontology + composed embedding). Nodes content-addressed/immutable (Merkle ancestors). Edges are programs (`on_insert`/`on_delete`/`on_query_rewrite`): supersede closes VT; contradict keeps both; no raw append.

## 2. STCA applicability

Space: nested packs/ports. Time: Relation Behaviors as write-time programs (already Kutha-shaped). Merkle worlds must be a *materialization*, not a second SoT (event log still wins).

## 3. Quality / cost

Strong critique of flat Graphiti-class graphs. Cost: world hashing vs log replay; cascade storms without budgets.

## 4. Demand

Case files, jurisdictions, agent sandboxes that must not leak without `REFERS_TO`.

## 5. Niche → effect

`no niche` — engine composition; legal case-files would use it later.
