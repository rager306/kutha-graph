---
id: paper-semantic-layer-smq
source: paper
axes: [Query, Agent, Space]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Semantic layer as compile IR (SMQ), not LLM-over-schema

Papers: [A Semantic-Layer-Mediated Agent for NL2SQL](https://consensus.app/papers/details/d8b879deaea55d08a8ec53b4e14c2c1f/?utm_source=cursor) (Kim, Khoeurn, Yoon, 2026, arXiv:2606.31041, DOI: 10.48550/arxiv.2606.31041). Related: [LangChain Iceberg Toolkit](https://consensus.app/papers/details/b7da6cd637265ca991d46c1f9dcb624d/?utm_source=cursor) (Kataria et al., 2026) YAML metrics + type-safe API vs text-to-SQL injection; [SemanticQuark](https://consensus.app/papers/details/bb655655b771566fbe04f75a27328db6/?utm_source=cursor) (Kataria, 2026) Python-native metric graph; [CoeusBI](https://consensus.app/papers/details/283bd121b81459d697f4eefb3e8b0357/?utm_source=cursor) (Lian et al., 2026) dual-agent + deterministic SQL compiler. YOTG Vol. 31 named Cube/dbt semantic layers as the BI cousin of context graphs.

## 1. Raw idea

Enterprise NL2SQL fails when the LLM sees hundreds of cryptic tables. Kim et al.: agent reasons over a **curated semantic layer** via a compact IR (**Semantic Model Query**); a **deterministic compiler** turns SMQ into dialect SQL (SQLite/BigQuery/Snowflake). Agent may compose around compiled blocks, not invent joins from raw schema. 94.15% execution accuracy on Spider2-snow (547 tasks) vs schema-only baselines. Iceberg toolkit: YAML business terms; 100% vs 67% schema-aware text-to-SQL; SQL-injection success 0% vs ~99% in prior NL2SQL. CoeusBI: offline view-generation agent flattens JOINs into single views; routing agent; dialect-agnostic compiler. SemanticQuark: one metric definition graph, BFS join paths, RLS at the semantic boundary.

## 2. STCA applicability

Query/Agent: this is the **SQL-side twin** of `paper-az-projected-schema-cypher` and `paper-llm-compiler-not-executor`. Meaning lives in a portable pack (YAML/metrics/dimensions), not in the warehouse and not in the LLM. Space: semantic layer is the BI dictionary; Kutha’s graph dictionaries should compile the same way (SMQ ≈ prototype MATCH). Verify: compiled SQL/Cypher is the receipt; do not log “the model wrote a query.” Distinct from OCPM (process objects) and from formal OWL ontology (Figay): this is a **metrics/meaning IR**, often not a description logic.

## 3. Quality / cost

Usefulness high: production-shaped compile contract with numbers. Optimality med: still SQL warehouses, not WCOJ; Kim notes overfitting tension of maintaining the layer as LLM context. Cost: treat metric/dimension defs as versioned dictionary events; compiler in-process; refuse schema-dump prompting. Do not make Cube/dbt the graph SoT.

## 4. Demand

“What was revenue last quarter?” must mean one definition across agents. Without a compile IR, every copilot invents a JOIN. Engine demand: dict → IR → executed query, same as Cypher guardrails.

## 5. Niche → effect

`no niche`
