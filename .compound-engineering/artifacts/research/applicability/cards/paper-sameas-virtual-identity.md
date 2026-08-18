---
id: paper-sameas-virtual-identity
source: paper
axes: [Data, Verify, Space]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Identity as dated links, not owl:sameAs collapse

Papers: [SAGE](https://consensus.app/papers/details/11c30e1788c659b48fbcd505aca3e426/?utm_source=cursor) (Arjarapu et al., 2026, SIGMOD Companion, DOI: 10.1145/3788853.3801599); [When owl:sameAs isn't the Same](https://consensus.app/papers/details/b69036544207504088bede88036eb42a/?utm_source=cursor) (Halpin et al., 2010); survey [The sameAs Problem](https://consensus.app/papers/details/58135a57d66d51898a188381e0f5ab80/?utm_source=cursor) (Raad et al., 2019, DOI: 10.48550/arxiv.1907.10528); [identiConTo / DECIDE](https://consensus.app/papers/details/c27ac9ee4bb65e93b79fbb0dba074f5d/?utm_source=cursor) (Raad et al., 2017). Complements `paper-temporal-complex-entity-resolution` (temporal *fold* of mentions) with the **no-merge** pole.

## 1. Raw idea

SAGE: heterogeneous sources (CSV, JSON, PDF, RDBMS) queried in NL **without merging entities**. Aggressive merge and LLM-hallucinated joins cause semantic errors. Architecture: no-merge KG + LLM concept classification + **validated virtual links** (every join shown with evidence) + GAT fingerprints. Halpin: `owl:sameAs` on the Web is often the wrong identity link; four weaker readings; named graphs as context. identiConTo: contextual identity that could replace blanket sameAs. Universal KG embeddings that fuse on sameAs (Kouagou 2023) are the opposite pole — unique identity by collapse.

## 2. STCA applicability

Data: canonical identity in Kutha is an **event** (`SameAs`/`Identifies` with valid-time and provenance), not an in-place node merge that destroys lineage (Kirielle already: dated fold). Space: virtual links are query-time joins / leases across packs, not a second SoT. Verify: SAGE’s “every join validated and exposed” is the Observation/receipt pattern (AZ, PACT). Agent: LLM classifies concepts; it does not silently fuse nodes. Do not treat `owl:sameAs` inference as SoT — it is a view that can be wrong in a context.

## 3. Quality / cost

Usefulness high: names the merge vs link fork the identity pack must pick. Optimality med: SAGE is a SIGMOD demo; Halpin is diagnosis. Cost: store mentions + link events; compile virtual links into the query (AZ projected schema), never collapse WAL history. Embedding alignment stays a ranking hint (MemStrata).

## 4. Demand

BIKG mapping, legal party identity, and multi-source enterprise graphs all hit “same person, two URIs.” Silent merge breaks AS-OF and ABAC. Engine demand: identity links with context and receipts.

## 5. Niche → effect

`no niche`
