---
id: paper-temporal-wcoj-bgp
source: paper
axes: [Query, Time, Data]
usefulness: high
optimality: high
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Worst-case optimal BGPs on temporal graphs

Paper: [Worst-Case Optimal BGPs on Temporal Graphs](https://arxiv.org/abs/2607.20356) (Arroyuelo et al., 2026). Jina: `https://arxiv.org/html/2607.20356`.

## 1. Raw idea

BGPs as quads (s,p,o,time) with order constraints on time; WCOJ-style index in O(N) space; snapshot/version queries as related types.

## 2. STCA applicability

Query+Time: AS OF multi-hop cost model (ADR-000 R3/R6). Data: temporal index as materialization, not SoT.

## 3. Quality / cost

Directly probes whether LFTJ-class joins survive bi-temporal. Implementation cost is a dedicated temporal index pack (P1–P2).

## 4. Demand

Legal PIT + multi-hop together — the expensive case Kutha must not hand-wave.

## 5. Niche → effect

`no niche` — query-engine optimality; legal PIT uses it but does not own it.
