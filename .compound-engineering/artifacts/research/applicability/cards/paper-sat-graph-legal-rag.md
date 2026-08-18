---
id: paper-sat-graph-legal-rag
source: paper
axes: [Time, Query, Verify]
usefulness: high
optimality: med
demand: med
confidence: paper
layer5: "Legal/constitutional retrieval: hierarchical Works vs versioned Expressions; point-in-time + provenance"
status: closed
channels_failed: []
---

# SAT-Graph RAG: hierarchical, temporal, deterministic legal norms

Paper: [An Ontology-Driven Graph RAG for Legal Norms](https://doi.org/10.3233/faia251598) (de Martim, 2025). Consensus: https://consensus.app/papers/details/0d100fa6848453b880bdbde97a690cb0/?utm_source=cursor

Companion argument (same author): [Beyond Probabilistic Similarity](https://arxiv.org/abs/2606.09724) — mereological / diachronic / causal blindness of vanilla RAG; commitments: ontological primacy, event reification, bitemporal correctness, deterministic protocols. Consensus: https://consensus.app/papers/details/c43768fbf9fc57f688a9d3e9a7cf3fde/?utm_source=cursor

## 1. Raw idea

Flat-text RAG is blind to hierarchy, diachrony, and causation of law. SAT-Graph: LRMoo-inspired Works vs versioned Expressions; temporal states reuse unchanged component versions; legislative events as Action nodes. Planner policies for point-in-time retrieve, hierarchical impact, auditable provenance. Case study: Brazilian Constitution.

## 2. STCA applicability

Time/Verify: graph is a *versioned fold* of norms, not an embedding store. Query: as-of and impact analysis are graph cuts, not cosine. LLM still outside trust (TGMS). Distinct from MemStrata ((s,r,o) supersession) and statutory QA (LLM failure modes): this is the *schema* those probes need.

## 3. Quality / cost

Strong shape for a legal honeycomb later; not P0 core. Cost: ontology-heavy; Brazilian Constitution case is not a join benchmark.

## 4. Demand

Anachronistic citations and “which article applied on date D” cannot be answered by chunk RAG.

## 5. Niche → effect

Legal/constitutional retrieval: hierarchical Works vs versioned Expressions; point-in-time + provenance. Wedge, not intake filter.
