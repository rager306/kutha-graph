---
id: paper-query-admission-control
source: paper
axes: [Query, Composition, Security]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Admission is a per-query budget — not a tenant slice and not cardinality

Papers: [CASA](https://consensus.app/papers/details/6f7ae997037853ce815706ecfeac16fe/?utm_source=cursor) (Zeyl et al., 2024) map predicted CPU/memory from query *text* to slots; throttle admission; +48% throughput, −50.6% tail latency [1]; [SafeLoad](https://consensus.app/papers/details/0bd88af2667d5de08be78b4fef9de0ef/?utm_source=cursor) (Wu et al., 2025, VLDB) admit/reject **memory-overloading** warehouse queries before they waste CPU; +66% precision vs baselines [2]; [Bouncer](https://consensus.app/papers/details/dd2150b4f3525b0a951d128c2b423b9b/?utm_source=cursor) (Xu et al., 2024, SIGMOD) percentile-SLO admission from cheap latency estimates; class-specific SLOs; starvation-avoidance variants [4][5]; workload-management taxonomy (admission, scheduling, execution control) [18]; [Banyan](https://consensus.app/papers/details/6daaa6c85a6f56b1981409437c5fd6b5/?utm_source=cursor) (Su et al., 2022) scoped dataflow so graph query *service* can isolate and schedule at subquery grain [20]. Distinct from `paper-graph-tenant-isolation` (slice/quota of *data* and noisy-neighbor CPU), `paper-graph-cardinality-estimation` (planner statistic, not admit/reject), `paper-wasm-udf-sandbox` (isolate a procedure).

## 1. Raw idea

A query may be *correct* and still *unaffordable*. Admission control decides accept / queue / reject **before** leapfrog runs, using predicted slots, memory, or percentile SLO [1][2][4]. Workload classes get different budgets [18]. Graph services need this at *subquery* grain or one MATCH starves the rest [20]. Cloud workflow-deadline papers and SAP HANA vendor guides are demand, not this card. SQLVM (tenant card) reserves *tenant* resources; this card budgets *this query*.

## 2. STCA applicability

Composition 014/030: STCA cascade / max-convolution is exactly a **query budget** V. Query: compile emits a cost envelope (cardinality card feeds it); admission compares envelope to remaining V. Security: a tenant slice without admission still lets one AS-OF burn the box. Time: budgets are per request, not valid-time of a grant. Do not train an LLM as the governor.

## 3. Quality / cost

Usefulness high: honeycomb 014 is a slogan without an admit operator. Optimality med: CASA/Bouncer/SafeLoad are warehouse-shaped; Banyan is the graph-service cousin. Cost: P0 = unlimited queries; honeycomb = slot/SLO check after compile, before execute; reject is an event (why-not for “query not run”).

## 4. Demand

Legal AS-OF multi-hop will be issued by agents in bursts. Engine demand: fail-closed admission with a budget receipt, not OS OOM.

## 5. Niche → effect

`no niche`
