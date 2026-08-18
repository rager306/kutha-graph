---
id: paper-az-research-plan-dag
source: paper
axes: [Agent, Composition, Verify]
usefulness: med
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Research-plan DAG of questions (not a second SoT)

Paper: Grabowski et al., *Research Assistant: AstraZeneca’s Agentic System for R&D* (arXiv:2608.12395v1). HTML: https://arxiv.org/html/2608.12395v1. Same technical note as `paper-az-projected-schema-cypher`; this card is the **Control/plan** capability, not the Cypher compile contract.

## 1. Raw idea

Deep Research Mode models a complex user question as a **DAG of sub-questions**. A planner proposes the DAG; a judge agent accepts or requests revision (bounded loop). The accepted DAG is topologically sorted: independent questions run first; dependents are **rewritten** with earlier answers (e.g. a later drug query uses a synonym found upstream). Users may supply an explicit plan and skip auto-generation. Plan size is fixed up front so wall-clock and token budget are bounded. Each node is answered by the same grounded Scientific Mode (parallel tools + Observation envelopes), not by an unconstrained LLM loop. Contrast with open-ended co-scientist systems they cite (CoScientist, Robin): this is day-to-day grounded retrieval, not autonomous discovery.

## 2. STCA applicability

Agent/Composition: the DAG is a **plan over queries**, not over graph mutations. Matches dict-first agents: LLM proposes structure; execution is typed tool calls. Do not promote Apache Burr or the judge LLM to Kutha Control SoT (ADR-000 rejected hard FSM as sole agent control). Verify: judge-accept is advisory; truth still comes from Observations / executed graph patterns. Time: dependent rewrite is session-scoped, not bi-temporal fact invalidation.

## 3. Quality / cost

Usefulness med: production pattern for multi-hop *research* (not multi-hop *join*). Optimality med: topo + rewrite is cheap; quality hangs on the planner/judge, which they do not bound formally. Cost: a Kutha-side analogue is a versioned query pack (compiler-not-executor) whose nodes are sealed graph/vector views — not Burr graphs. Distinct from CoAgent MTPO (runtime CC) and Cordon (tool-effect txs).

## 4. Demand

Scientists ask multi-entity, multi-step questions (“repeat this analysis across genes/trials”) and need a **fixed-size** plan rather than an unbounded agent loop. Engine demand: bounded DAG of compiled queries with rewrite of later nodes from earlier receipts. Product chat is optional.

## 5. Niche → effect

`no niche`
