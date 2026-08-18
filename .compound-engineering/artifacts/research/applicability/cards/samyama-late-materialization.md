---
id: samyama-late-materialization
source: samyama
axes: [Query, Data]
usefulness: high
optimality: high
demand: high
confidence: code
layer5: no niche
status: closed
channels_failed: []
---

# Late materialization traversal (ids first, properties later)

Evidence: `benches/late_materialization_bench.rs`; store comments on stub nodes + ColumnStore phase B; CLAUDE.md documents raw vs lazy vs Cypher.

## 1. Raw idea

Keep hops on compact ids/CSR; pull property maps only when the plan needs them.

## 2. STCA applicability

Query+Data: matches “joins sized by output.” Packs can still attach fat properties without poisoning the hop path.

## 3. Quality / cost

High fit to STCA materialization. Cost: two-phase load and column layout in the hot store.

## 4. Demand

Wide nodes (legal/science) would otherwise make every hop a hashmap copy.

## 5. Niche → effect

`no niche`
