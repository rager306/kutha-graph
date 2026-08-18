---
id: paper-az-projected-schema-cypher
source: paper
axes: [Query, Agent, Verify]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Biomedical/clinical R&D: NL over a KG scientists cannot Cypher; citation-tagged Observations consumed by chat and by patient-safety tools"
status: closed
channels_failed: []
---

# Projected-schema Cypher with deterministic provenance rules

Paper: Grabowski et al., *Research Assistant: AstraZeneca’s Agentic System for R&D* (arXiv:2608.12395v1). HTML: https://arxiv.org/html/2608.12395v1 PDF: https://arxiv.org/pdf/2608.12395. Consensus does not index this preprint yet. Underlying graph: [Biological Insights Knowledge Graph](https://consensus.app/papers/details/485261f969ba5a9ea8b3c3c63326656e/?utm_source=cursor) (Geleta et al., 2021, bioRxiv, DOI: 10.1101/2021.10.28.466262). Sibling AZ agentic product (chemistry, not this KG QA stack): [ChatInvent](https://consensus.app/papers/details/4dc8cd75fd9d59a5943e992c0a05ae28/?utm_source=cursor) (He et al., 2026).

## 1. Raw idea

AstraZeneca’s internal Research Assistant is a chat + MCP/REST surface over literature, chemistry, clinical, safety, expression, and the Biological Insights Knowledge Graph (BIKG). Domain experts previously needed REST or Cypher. The Knowledge Graph Agent does **not** let an LLM emit Cypher/SPARQL against the full schema. Observed failure modes of unconstrained NL→Cypher on a large integrated graph: non-deterministic path choice (direct `ASSOCIATES` vs inferred pathway hops for the same question); inability to apply meta-level confidence and source trust consistently; factually legal but too-complex queries that time out.

Production workflow: (1) ground mentions to canonical IDs (KAZU NER + BIKG mapping); (2) build a **prototype MATCH** from a **projected schema** (QA-relevant node/rel/attr subset); (3) **deterministic rules** rewrite the prototype (allowed `r.prov` sources, categorical edge semantics, required return attributes); (4) execute on Neo4j; (5) return a typed **Observation** (id, source URL, payload of sentences/tables/triples, citation string). Observation IDs are injected into the synthesis LLM so statements can be tagged back to retrieved evidence.

Adjacent product mechanisms (same paper, not this card’s SoT claim): Scientific Mode = single-pass parallel tool agents + large-model synthesis; Deep Research Mode = judge-accepted DAG of questions, topological execution, dependent-question rewrite; tool routing by topic modeling over real queries (embedding-NN over exemplars was dropped: cosine always hits); sentence-level literature RelEx index (~3.8B sentences) rather than abstract RAG; STaRK/PrimeKG mapped onto BIKG for KG-agent regression; BioASQ yes/no did not track real user needs.

## 2. STCA applicability

Query: NL is compiled into a constrained graph pattern, not executed as free Cypher. Matches `paper-llm-compiler-not-executor` in production on a biomedical KG.

Agent: LLM maps text → structured tool calls (Instructor/Pydantic) and synthesizes; it does not become the graph. Apache Burr is a product-layer state machine (loops, SSE), not Kutha Control-as-SoT. Deep Research DAG is a plan over questions, each answered by the same grounded Scientific Mode.

Verify: Observation envelope + citation injection is the product cousin of `paper-proof-carrying-llm-envelope` / PACT: users (and downstream tools like CRAM combination-risk) inspect URLs and tags. Edge `prov` filters are source-trust, not temporal SoT. Residual admitted failures: hallucinations; weak sensitivity to gene paralogs.

Time/Data: BIKG is a Neo4j materialization with source provenance on edges, not an event-log SoT. Literature `date` and OpenAlex impact scores are ranking heuristics. Do not import Neo4j or the LLM as Kutha truth.

## 3. Quality / cost

Usefulness is high because it is a large-scale (claimed 15k unique internal users in one year) confirmation that unconstrained LLM-Cypher is unsafe on a real integrated KG, and that **projected schema + deterministic provenance rewrite** is the workable split. Optimality is med: execution is still Neo4j Cypher (timeouts remain a stated risk); no WCOJ/CSR story; tool parallelism and 10–30s / ~$0.16-per-query economics are product, not engine. Cost to Kutha: borrow the **compile contract** (ground → prototype from a QA projection → rule-seal sources → execute → Observation ids), not Burr/FastAPI/Gemini. Mapping-agent synonym/canonical index is a Data-axis identity fold, not a second graph.

## 4. Demand

Scientists and clinicians will not write Cypher; they still need pairwise gene–disease / compound–target facts with inspectable sources. Patient-safety combination-risk (CRAM) already consumes the same grounded endpoint programmatically. Engine demand: fail-closed NL→pattern compilation, canonical ID grounding, and an Observation/receipt type that synthesis cannot invent. Contrast: Graphiti LLM-extracts into an external GDBMS; this paper keeps the KG as a queried service and the LLM as router/synthesizer.

## 5. Niche → effect

Biomedical/clinical R&D: NL over a KG scientists cannot Cypher; citation-tagged Observations consumed by chat and by patient-safety tools. Pattern generalizes; the wedge is drug-discovery KG QA, not a research intake filter.
