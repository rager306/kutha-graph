---
id: open-ontologies-mcp-govern
source: open-ontologies
axes: [Agent, Verify, Query, Space]
usefulness: high
optimality: med
demand: high
confidence: code
layer5: "Legal/science: MCP validate/diff/certify AI-generated OWL; Oxigraph is working memory — not the event-log SoT"
status: closed
channels_failed: []
---

# Ontology MCP is a generate–validate loop — Oxigraph is not Kutha SoT

User URL: https://github.com/fabio-rovai/open-ontologies (cloned `/tmp/user-url-scout/open-ontologies`, not CBM-indexed). Rust MCP + Studio; v1.2.0; `oxigraph = 0.5`. Read: `src/graph.rs` — in-memory (or Rocks-backed) Oxigraph `Store` + SPARQL; `src/reason.rs` — OWL2-RL fixpoint on interned `u32` triples (`rdfs` / `owl-rl` / `owl-rl-ext`; OWL-DL delegated to tableaux); `src/shacl.rs` — shapes Turtle → SPARQL (`minCount`/`maxCount`/`datatype`/`pattern`/`hasValue`); `src/kgcl.rs` — drift → KGCL CNL (`create`/`obsolete`); `src/projection_check.rs` — GraphRAG slice vs full neighbourhood coverage. README: LLM does intelligence; server validates. Distinct from `paper-obda-ontology-compile` (virtual rewrite to SQL), `paper-ontology-temporal-versioning` (τOWL version algebra; KGCL already named there), `paper-shacl-sparql-compile` (complexity trap of recursive SHACL), `oxify-dag-llm-orchestration` (workflow DAG, not TBox tools), `oxixml-xml-rdf-stack` (XML/RDF parse, SPARQL **syntax**).

## 1. Raw idea

A **Terraform for ontologies**: 70+ `onto_*` tools let an LLM build/validate/query/diff/lint/align/plan OWL, while the engine holds an Oxigraph working copy and fail-closes on SHACL/OWL-RL/CQ verdicts. Projection-loss audit pairs with TBox-slice RAG. Optional Dynamics/Causal/Planner (PDDL, PyWhy) and WASM plugins exist behind features — not this card’s noun.

## 2. STCA applicability

Agent: LLM is **compiler/proposer**, MCP tools are the verified operators (AZ fence). Verify: OWL-RL materialization and SHACL reports are **receipts on a pack**, not quantum receipts on the event log. Space: marketplace ontologies are named-graph packs. Query: SPARQL against Oxigraph is a *remote/working-memory* dialect; Kutha MATCH stays leapfrog. Data: interned u32 reasoner is a dictionary cousin; do not make Oxigraph/Rocks the SoT (D1/D2). KGCL CNL is a payload type for dict-drift events already closed as versioning.

## 3. Quality / cost

Usefulness high: legal/science packs will be LLM-drafted OWL. Optimality med: real RL reasoner + fragment SHACL; not full Pellet; SHACL here is a *bounded* SPARQL translation (avoid recursive shapes). Cost: call as an **ontology workbench pack** (validate before dict events); do not embed Oxigraph as the fold; skip PDDL/PyWhy/WASM plugins unless a later spike.

## 4. Demand

Unreviewed LLM ontologies will ship. Engine demand: generate → MCP validate/certify → log schema events, fail-closed.

## 5. Niche → effect

Legal/science: MCP validate/diff/certify AI-generated OWL; Oxigraph is working memory — not the event-log SoT
