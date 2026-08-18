---
id: claim-yotg-context-graph-layers
source: paper
axes: [Agent, Space, Verify, Data]
usefulness: high
optimality: low
demand: high
confidence: claim
layer5: no niche
status: closed
channels_failed: []
---

# Context graphs as a market mash-up (not a new SoT)

Source: Anadiotis, *Layers of Meaning: Context Graphs, Graph Memory, and Ontologies for AI* ([Year of the Graph Vol. 31, Summer 2026](https://yearofthegraph.xyz/newsletter/2026/06/layers-of-meaning-context-graphs-graph-memory-and-ontologies-for-ai-the-year-of-the-graph-newsletter-vol-31-summer-2026/)). Industry synthesis (Foundation Capital, Forrester, Prukalpa, McCreary, Figay, Verhelst, Murty, Ciocanu), not a kernel paper. Confidence `claim`.

## 1. Raw idea

“Context graph / context layer / ontology for AI” is a **convergence brand**, not one data structure. The newsletter splits it three ways that vendors flatten:

**Three *sources* of context** (Bou-Balust & Arias market map): (1) agent-internal memory and operating manual; (2) institutional knowledge (docs, chats, tickets); (3) systems of record (human business records + machine telemetry), plus a public verified-facts layer. Surface-owners vs neutral indexers; governance/trust as a tax.

**Three *kinds* of context the layer must encode** (Prukalpa): Knowledge (map of the business), Expertise (how work is done), Norms (rules of acceptable action). McCreary: a context graph is a persistent record of product/customer data, ontologies, **and decision traces** (what happened, why, who approved, which precedents) — sitting between knowledge graphs, RAG, and process mining.

**Meaning vs graph:** graph structure is necessary but not sufficient. Who authors and **owns** meaning, and whether meaning is portable across vendors, is the differentiator. Figay: most “ontologies” shipping from Databricks/Snowflake/Neo4j/etc. are governed property graphs or glossaries, not formal ontologies — the gap appears when an agent is asked to *reason*. Verhelst’s tests: can you move the meaning; can the system **prove an action is allowed before** the agent acts, or only log after; did you author the meaning or did the platform learn and hold it.

**Graph memory ≠ RAG:** RAG reads a fixed corpus; agent memory is bidirectional (write/read/update). Murty: STM / LTM / semantic (what is true) / episodic (what happened, when). Ciocanu: agent memory should be precise, versioned, consistent — the opposite of reconstructive human memory; library science + ontology engineering, not a bigger vector store. Gupta: pluggable multi-backend (episodic + graph + session cache) is a consistency trap; unified graph engines win at org scope.

**LLM wiki pattern:** humans own judgment; LLM does bookkeeping in a compounding store (Karpathy: git of markdown). At scale, nodes+links want a graph, not naive chunk RAG.

**Graph DB → graph engine:** query data in place (Memgraph Zero / federated GQL) vs ETL into a graph store. Arango thesis quoted: modeling relationships ≠ managing business context.

## 2. STCA applicability

Do **not** adopt “context graph” as Kutha’s SoT. Split the mash-up onto existing locks:

| Newsletter layer | Kutha |
|------------------|--------|
| Bucket 3 systems of record + McCreary decision traces | Event log (D1/D2); ActiveGraph / CTEG / agent-trace cards |
| Knowledge (business map) | Folded property graph + identity pack (`paper-temporal-complex-entity-resolution`) |
| Expertise (how work is done) | Behaviors + dicts; not Graphiti extraction as write-path SoT |
| Norms (allowed action) | Temporal grants + path-ABAC compile-before-act (`paper-temporal-grants-as-facts`, `paper-xacml4g-path-abac`, Verhelst “prove before”) |
| Ontology / semantic layer | Space/Agent **dictionaries and packs** (portable meaning); not a vendor-held learned “Genie Ontology” |
| Agent-internal memory | Lease/view over the log (Engram/TOKI/Graphiti *contrast*); STM is session, LTM is fold |
| Institutional RAG | Query leases (HNSW/hybrid); LLM compiler-not-executor |
| One shared brain, many agents | Shared log + worlds/forks (WorldDB, Omnigraph-ad branches) — context is global, execution paths local |
| Unified engine vs multi-backend | One Rust core; CSR/HNSW/ontology views are reversible pictures, not three SoTs |

Inmon/Talisman: context is a **human-led social agreement**. That is pack authorship (legal/science dictionaries), not something the LLM silently learns into the graph.

## 3. Quality / cost

Usefulness high as a **demand and vocabulary map** (every major platform now ships the word). Optimality low: no join bound, no receipt calculus, no kernel. Cost of swallowing the brand: collapsing ontology, KG, process-mining traces, and agent memory into one mutable graph — exactly Graphiti/Kumiho-host-DB failure modes. Cost of ignoring it: Kutha will be asked “where is the context graph?” and must answer with the split above, not a new product noun.

## 4. Demand

Builders want agents that know what the business *means*, remember what happened, and do not act outside norms. That demand is real. The engine answer is log + dated grants + portable dictionaries + hot views — not a trillion-dollar “context graph” SKU.

## 5. Niche → effect

`no niche` — category critique and mapping, not a vertical pack.
