---
id: paper-keyword-search-graphs
source: paper
axes: [Query, Agent, Data]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Legal/journalism: schema-agnostic keywords over the fold return a connecting subtree — not Cypher MATCH and not BM25-on-chunks"
status: closed
channels_failed: []
---

# Keywords compile to a connecting subtree — not MATCH and not chunk RAG

Papers: [BANKS](https://consensus.app/papers/details/163aadd06b755309952343b019e67737/?utm_source=cursor) (Bhalotia et al., 2002) tuples as graph nodes, FK edges; answers = rooted trees covering keywords, ranked by proximity + prestige [1][5][8]; [BLINKS](https://consensus.app/papers/details/0c8d084f3eb45331b5ad25247142b9bd/?utm_source=cursor) (He/Wang et al., 2007) bi-level block index for top-k keyword search on general graphs; orders-of-magnitude over naive expansion [4]; survey: schema-based SQL CN vs materialized directed data graph (Steiner / distinct-root / r-radius) [3]; VLDB tutorial: continuum between SPARQL/GPML and keyword search — structured when you know the schema, keywords when you do not [7]; Elas4RDF: schema-agnostic triple retrieval via Elasticsearch, explainable triples not just entities [2]; ConnectionLens: investigative journalism over heterogeneous schema-less sources as one keyword-searchable graph [11]. Distinct from `ruvector-hybrid-bm25-dense` (lexical+vector on *documents*, no connecting tree), `paper-mixed-vector-relational-access` (FVS/access paths), `paper-iso-gql-gpml` (structured GPML), `paper-az-projected-schema-cypher` (LLM compiles to Cypher, not keywords-as-query). Distributed DKWS and data-lake UnifySea stay queued.

## 1. Raw idea

The user types words, not Cypher. Hits live on different nodes; the *answer* is the small connecting subgraph (Steiner tree / rooted tree) that ties them, ranked [1][4]. This is IR over a graph, not IR over a corpus then hope the LLM joins.

## 2. STCA applicability

Query 071 hybrid retrieve: keyword → posting lists on interned text → Steiner/expansion with a hop budget → optional Cypher refine. Agent: keywords are the *unskilled* surface; LLM may rewrite keywords, never walk the graph. Data: BLINKS-style block index is a droppable lease, not SoT. Time: AS-OF means search the fold at offset T, not a third IR index as truth. Verify: returned tree is the why (which hops connected the hits).

## 3. Quality / cost

Usefulness high: nobody writes Cypher on first contact with a dump. Optimality med: BANKS/BLINKS are classic; schema-agnostic ES over RDF is a portable pole [2]. Cost: P0 = inverted index on node/edge strings + hop-bounded Steiner; honeycomb = BLINKS block index. Do not replace MATCH with Elasticsearch as SoT.

## 4. Demand

“Find the second-request memo about Acme” is keywords plus a connecting path, not glob and not a 12-join Cypher. Engine demand: keyword surface that returns a subtree of the fold.

## 5. Niche → effect

Legal/journalism: schema-agnostic keywords over the fold return a connecting subtree — not Cypher MATCH and not BM25-on-chunks
