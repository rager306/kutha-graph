---
id: paper-graph-structured-encryption
source: paper
axes: [Security, Query, Space]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Legal/enterprise: outsource a *lease* (shortest-path / neighbor index) to an untrusted cloud; the event log never leaves the fold; STE is not DP noise and not an ADS receipt"
status: closed
channels_failed: []
---

# Structured encryption queries ciphertext — the log stays home

Papers: [Structured Encryption and Controlled Disclosure](https://consensus.app/papers/details/e29a9fb283495ac08ce4716e2787dfc1/?utm_source=cursor) (Chase & Kamara, 2010, DOI: 10.1007/978-3-642-17373-8_33) — STE generalizes SSE from keyword files to arbitrary structure (web/social graphs) [12]; [GraphShield](https://consensus.app/papers/details/0cc34d834bb058af981b5e8cb105e6fe/?utm_source=cursor) (Du et al., 2020, TKDE) encrypted graphs: shortest path, max-flow, PageRank; updates with **forward privacy** [1]; [STE for knowledge graphs](https://consensus.app/papers/details/c1dd47faed805929a9599cc5cb1b233b/?utm_source=cursor) (Xue et al., 2022) first CQA2-secure STE for multi-relational and property graphs [2]; [GES shortest path](https://consensus.app/papers/details/472523b0fe005eabb174c19f634b041c/?utm_source=cursor) (Ghosh, Kamara, Tamassia, 2021) recursive SPSP with optimal overhead vs plaintext search structure [17]; [Spidey](https://consensus.app/papers/details/33c57d97234a5ae3b8960ab218e0ac2d/?utm_source=cursor) (Wu et al., 2025) dynamic encrypted *property* graphs + lightweight RBAC [15]. Distinct from `paper-graph-differential-privacy` (noise on *aggregates*, plaintext store), `paper-blockchain-graph-ads` (proofs of an answer, not hiding the graph), `paper-wasm-udf-sandbox` (isolate a procedure), `oxify-zanzibar-rebac` (who may Check).

## 1. Raw idea

SSE retrieves documents by keyword. Graphs need **structured encryption**: an encrypted index that answers neighbor / shortest-path / flow queries with a defined leakage profile (search pattern, access pattern, updates) [12][1]. Forward privacy: an update does not reveal it matches a past query [1][3]. Property-graph STE is strictly richer than adjacency-only [2][15]. Keyword-document SSE and “encrypt then linear scan” are not this card. Leakage is the product: schemes that hide search pattern still leak *something* [7].

## 2. STCA applicability

Security 080: STE is a **ciphertext lease** of a projection (CSR / neighbor lists / SP matrix), not a second SoT. The event log and dictionaries stay on trusted Space. Query: only the operators the scheme actually supports (SPSP, adjacency, PageRank) — not general Cypher/WCOJ over ciphertext. Verify: STE leakage ≠ quantum receipt; ADS/blockchain proves an answer, STE hides the graph. Cloud is untrusted compute, like federation’s remote, but encrypted.

## 3. Quality / cost

Usefulness high: STRATEGY already assumes untrusted infra. Optimality med: GraphShield is practical on real graphs [1]; general MATCH over STE is not in this batch. Cost: P0 = encrypt at rest (disk); honeycomb = STE neighbor/SPSP pack for *outsourced analytics leases*; do not encrypt the WAL as the query surface.

## 4. Demand

Counsel graphs and citation graphs will be asked to live in someone else’s cloud. Engine demand: an explicit leakage-bounded neighbor operator, not “put the fold in S3 and hope”.

## 5. Niche → effect

Legal/enterprise: outsource a *lease* (shortest-path / neighbor index) to an untrusted cloud; the event log never leaves the fold; STE is not DP noise and not an ADS receipt.
