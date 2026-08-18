---
id: paper-iso-gql-gpml
source: paper
axes: [Query, Space, Data]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# ISO GQL / SQL/PGQ share GPML — Cypher is the dialect, not the calculus

Papers: [Graph Pattern Matching in GQL and SQL/PGQ](https://consensus.app/papers/details/6146441f8e1d5d068411e0685f54604f/?utm_source=cursor) (Deutsch et al., 2022, SIGMOD, DOI: 10.1145/3514221.3526057) — identical **GPML** core; PGQ = graph views over SQL tables, GQL = standalone [1]; [Core PGQ vs Core GQL](https://consensus.app/papers/details/1d2ec7de402354a99c2b1c4dedb538a2/?utm_source=cursor) (Gheerbrant et al., 2024) bottom-up PGQ vs linear/pipelined GQL; v1 expressiveness gaps [2]; [GPC](https://consensus.app/papers/details/607a2b3e84435df3b4367824ca88c089/?utm_source=cursor) (Francis et al., PODS 2023) pattern calculus beyond RPQ/CRPQ [4]; [Cypher cannot express all RPQs](https://consensus.app/papers/details/f21b7cc2f07a54c08950310604226872/?utm_source=cursor) — why GQL added path features [6]; [path-based algebra](https://consensus.app/papers/details/fd2d6ee720cd56d4a72d6e1dc969bee3/?utm_source=cursor) (Angles et al., 2024) paths as first-class in the plan [14]; [RPQ walk semantics](https://consensus.app/papers/details/aae9e1eb5092532487fe8eb926922e6b/?utm_source=cursor) trail vs shortest vs GQL-selectable modes [12]; ISO/IEC 39075:2024 [5]. Distinct from `paper-gql-rules-materialization` (MERGE/ENRICH fixpoint) and `paper-az-projected-schema-cypher` (NL→MATCH compile). PG-Keys already closed.

## 1. Raw idea

ISO standardized **one pattern-matching sublanguage** used by GQL (graph DB) and SQL/PGQ (relational graph views). Industry Cypher is trail-biased and historically not all RPQs [6]. GPC/GPML is the thing to implement, then surface Cypher *or* GQL syntax. PGQ evaluates bottom-up from tables; GQL pipelines. Transformations that *output graphs* (GENERATE / DTGraph) sit outside tuple-set semantics [8][16].

## 2. STCA applicability

Query 070: compile GPML (with an explicit path mode) onto WCOJ + time-respecting journeys — not “whatever Neo4j does.” Space: SQL/PGQ is a **view pack** over tabular leases, not a second SoT. Data: PG-Schema/PG-Keys already name types; this card is the **pattern calculus**. Increasing-edge constraints that GQL cannot check can be compiled *into the graph* [13] — a rewrite pack, cousin of SHACL2SPARQL.

## 3. Quality / cost

Usefulness high: buyers will ask “GQL-compatible?” Optimality med: v1 has holes [2]; trail vs walk is a footgun next to restless journeys. Cost: one GPML IR; Cypher and GQL as skins; PGQ only if a SQL twin is needed.

## 4. Demand

ISO 39075 is the Cypher successor story. Engine demand: pattern IR + selectable path mode (do not default to counting walks).

## 5. Niche → effect

`no niche`
