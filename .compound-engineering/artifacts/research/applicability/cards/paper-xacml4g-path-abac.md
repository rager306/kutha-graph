---
id: paper-xacml4g-path-abac
source: paper
axes: [Security, Query, Space]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Legal/patent knowledge graphs: authorization as path constraints (subject–task–object), not table ACLs; query rewrite so unauthorized subgraphs never return"
status: closed
channels_failed: []
---

# Path-shaped ABAC via query rewrite (not app-layer filters)

Papers: [XACML Extension for Graphs](https://arxiv.org/pdf/2306.12819) (Mohamed, Auer, Hofer, Küng, arXiv:2306.12819); survey [Comparison of Access Control Approaches for Graph-Structured Data](https://consensus.app/papers/details/385ebdfcde2053a6ac6840422ed5bb00/?utm_source=cursor) (Mohamed et al., 2024, DOI: 10.48550/arxiv.2405.20762). Companion rewrite family: [ABAC for Neo4j](https://consensus.app/papers/details/5c1f288af18450da8e94e8ed5ff2376f/?utm_source=cursor) (Bereksi Reguig et al., 2023); [Rewriting Graph-DB Queries to Enforce ABAC](https://consensus.app/papers/details/5d21a8521d3f5ecf9494a9c1c99ff796/?utm_source=cursor) (Hofer et al., 2023); [AReBAC / Nano-Cypher](https://consensus.app/papers/details/180f1b9f9cd45f2aaed1913b8422bd8b/?utm_source=cursor) (Rizvi et al., 2020).

## 1. Raw idea

Property-graph authorization is not row-level SQL. Rights can depend on **vertices and edges along a path** from subject to resource (e.g. a task node with `typeCode` must exist on the path). XACML4G extends XACML with flexible path patterns (not every hop spelled out), treats **edges as resources**, and enforces datastore-independently by intersecting request path with policy path (Cypher template + source-subset graph). Sibling line: intercept Cypher and **rewrite** to a safe query that returns only authorized data (AST rewrite / extra filters) — ACaaS on top of stores that lack fine-grained AC. Survey comparison axes: base model, open/closed policy, negative permissions, datastore-independent enforcement. Origin scenario: patent-law knowledge graph (process + data).

## 2. STCA applicability

Security: ABAC/path policy is a **Port**, not a second SoT. Query: enforcement by **compiling policy into the query** (rewrite / pattern intersect) matches LLM-compiler-not-executor and AZ projected-schema Cypher: unauthorized data must not appear in the result set. Space: policies are a vertical pack (legal/patent vs biomedical). Time: these papers are not temporal ABAC (valid-time of the *permission*); Kutha still needs AS-OF on both facts and grants. Do not put policy evaluation inside the LLM.

## 3. Quality / cost

Usefulness high: STRATEGY already names ABAC/temporal policies as an enterprise buyer constraint; this is the graph-shaped form. Optimality med: Neo4j/Cypher prototypes, performance still “outlook”; rewrite cost grows with policy complexity. Cost: a Kutha security pack that **rewrites compiled graph patterns** (including `prov`/label filters) rather than post-filtering rows in the app. Distinct from PACT (tool-argument provenance) and MemLineage (memory custody).

## 4. Demand

Without path-aware AC, multi-tenant or counsel/client graphs leak along neighbor hops. Engine demand: fail-closed query rewrite from a policy fold. App-embedded `IF role` is the failure mode these papers reject.

## 5. Niche → effect

Legal/patent knowledge graphs: authorization as path constraints (subject–task–object), not table ACLs; query rewrite so unauthorized subgraphs never return.
