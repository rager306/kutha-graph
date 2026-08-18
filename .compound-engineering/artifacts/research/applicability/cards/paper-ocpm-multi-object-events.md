---
id: paper-ocpm-multi-object-events
source: paper
axes: [Time, Data, Query]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Object-centric events: one event, many objects (do not flatten)

Papers: [Object-Centric Process Mining: Unraveling the Fabric of Real Processes](https://consensus.app/papers/details/0626e2d220cc5159a6b5524f7c681d82/?utm_source=cursor) (van der Aalst, 2023, Mathematics, DOI: 10.3390/math11122691); flattening gap [Dealing with Divergence and Convergence](https://consensus.app/papers/details/c2fb4177602c5eaaa951855dd9062592/?utm_source=cursor) (van der Aalst, 2019, DOI: 10.1007/978-3-030-30446-1_1). Graph case notion: [Defining Cases and Variants for Object-Centric Event Data](https://consensus.app/papers/details/c493b0af1a8c54e0a3d11808d031de1e/?utm_source=cursor) (Adams et al., 2022). Event KG ↔ OCEL: [Transforming Event Knowledge Graph to Object-Centric Event Logs](https://consensus.app/papers/details/c88f7e929fab510981c30c8a7a27f768/?utm_source=cursor) (Khayatbashi et al., 2023). Distinct from ActiveGraph/CTEG (agent *execution* traces) and from YOTG “decision traces” as a product noun.

## 1. Raw idea

Traditional process mining forces **one case id per event**. Real ERP/CRM events involve many objects (order + customer + ten items + shipments + invoice). Flattening causes **convergence** (one event counted in many cases) and **divergence** (repeated activity inside a case). Object-centric event data (OCED/OCEL): an event relates to a set of typed objects. Process executions are **graphs**, not sequences; variants via graph isomorphism. Event knowledge graphs (EKG) store event–object correlations and directly-follows per object; converting EKG↔OCEL changes how directly-follows is computed on filtered logs. This is McCreary’s process-mining leg of “context graphs,” with a formal log model.

## 2. STCA applicability

Time/Data: Kutha log events must be allowed to **point at many objects** (lean event + relation facts), not a single `case_id`. Flattening to Graphiti-style “one episode → one subgraph” is the same information loss. Query: object-centric variants and conformance are folds/views over the log (leases), not a second process DB. Distinct from TARIS (time-respecting hops on a temporal graph) and from agent-trace CTEGs (causal trees of tool calls). Expertise in the YOTG split lives here: how work actually ran, multi-object.

## 3. Quality / cost

Usefulness high: names the exact bug of case-centric logs. Optimality med: discovery/conformance lifted from Petri/DFG; not WCOJ. Cost: event schema with N-ary object links; keep process models as reversible views. Do not import ProM as the engine.

## 4. Demand

Legal/ops packs will ask “which order, which filing, which agent session produced this?” without picking one case notion. Engine demand: multi-object events native on the log.

## 5. Niche → effect

`no niche`
