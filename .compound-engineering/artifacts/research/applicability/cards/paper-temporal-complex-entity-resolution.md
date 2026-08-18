---
id: paper-temporal-complex-entity-resolution
source: paper
axes: [Data, Time, Verify]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Identity as a temporal fold (complex-entity resolution)

Papers: [Unsupervised Graph-Based Entity Resolution for Complex Entities](https://consensus.app/papers/details/97d76d7a5ab053e88a7f05427f348aaf/?utm_source=cursor) (Kirielle, Christen, Ranbaduge, 2022, ACM TKDD, DOI: 10.1145/3533016). Context: [entity alignment survey](https://consensus.app/papers/details/7e13fbaa4731591f819fbcae0572b92e/?utm_source=cursor) (Zeng et al., 2021); embedding EA benchmark (Sun et al., 2020). Production cousin already noted on `paper-az-projected-schema-cypher`: BIKG mapping/synonym index, not a second graph.

## 1. Raw idea

Most graph ER assumes **static** entities (bibliographic papers). **Complex entities** (people, orgs) change attribute values *and* relationships over time. Neither vanilla graph-ER nor temporal record linkage alone is enough. Kirielle’s unsupervised framework: (1) propagate **positive evidence** (changed attribute values) into later links; (2) **negative evidence** via temporal and link constraints on candidate pairs; (3) attribute ambiguity to split lookalikes; (4) adaptive use of relationship structure; (5) graph-measure refinement to drop likely-wrong links. Reported up to +25% precision / +29% recall vs SOTA on seven datasets.

## 2. STCA applicability

Data: canonical IDs are a **fold** of mentions/synonyms, not in-place merges that destroy lineage (contrast SAGE-class “no-merge + virtual links” still queued). Time: identity decisions are dated; a later name change must not rewrite the earlier observation. Verify: negative constraints and cluster refinement are receipts for “why these mentions are the same person.” Agent: AZ Mapping Agent is the product surface; the engine must ground mentions to canonical IDs before Cypher (already in the AZ compile card). Do not treat embedding similarity as SoT (MemStrata: cosine cannot tell contradiction from duplicate).

## 3. Quality / cost

Usefulness high: every NL→graph compile step needs grounding; BIKG already treats canonicalization as graph-build work. Optimality med: unsupervised graph ER is research-grade, not a WCOJ kernel. Cost: an identity pack that **appends** merge/split events to the log (User-as-Code / ActiveGraph style), never silent node collapse. Embedding EA (EAGER, GCN alignment) stays a ranking hint.

## 4. Demand

Without temporal identity, paralogs/synonyms (AZ admitted failure) and person/org rename break AS-OF and ABAC (policies attach to the wrong node). Engine demand: dated identity events + query-time grounding.

## 5. Niche → effect

`no niche`
