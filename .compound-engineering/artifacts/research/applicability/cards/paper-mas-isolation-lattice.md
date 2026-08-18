---
id: paper-mas-isolation-lattice
source: paper
axes: [Agent, Composition, Verify]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Verified isolation lattice for multi-agent LLM runtimes

Paper: [Verified Detection and Prevention of Concurrency Anomalies in Multi-Agent Large Language Model Systems](https://arxiv.org/abs/2606.17182) (Khan, 2026). Consensus: https://consensus.app/papers/details/383d7859b43b551087d195d14e29a0f8/?utm_source=cursor

## 1. Raw idea

Shared memory / vector indices / tool registries as long-running read-generate-write. Four anomalies (stale-generation, phantom-tool, causal-cascade, tool-effect reordering) in TLA+; a mechanically checked chain \(L_0 \subsetneq \cdots \subsetneq L_4\). Realized in Rust runtimes; reproduces a silent lost update in deer-flow and tool-effect reordering in LangGraph ToolNode.

## 2. STCA applicability

Agent/Composition: concurrent writers on the same graph are isolation, not “prompt harder.” Complements TOKI (write-time *belief* operators) with *runtime* anomalies when several agents share Kutha. Verify: detectors proved against specs. Event log remains SoT; the lattice is the contract on concurrent folds/tools.

## 3. Quality / cost

High: machine-checked, deployed L0–L1, live L2. Cost: Verus/TLA+ is not P0 engine work; do not pull GPU or a second consensus. TOKI already closed the *fact* contradiction story — this card is the *agent-runtime* story.

## 4. Demand

Any multi-agent product on one store will hit stale-generation / phantom-tool without named levels.

## 5. Niche → effect

`no niche`
