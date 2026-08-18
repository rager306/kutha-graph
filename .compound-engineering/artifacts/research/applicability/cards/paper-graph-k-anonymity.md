---
id: paper-graph-k-anonymity
source: paper
axes: [Security, Data, Verify]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: Legal/enterprise: a published fold (opposing-party export, research dump) must not re-identify a party from degree or 1-hop structure — k-anonymity is a *publish transform*, not query-time DP noise and not a watermark
status: closed
channels_failed: []
---

# k-anonymity is a publish transform of the fold — not DP, not STE, not a watermark

Papers: [Liu & Terzi](https://consensus.app/papers/details/c032ced8cdad5477a5e3a4cd8e574524/?utm_source=cursor) (2008) k-degree anonymity: every degree appears ≥ k times; min edge edits from a realizable degree sequence — stripping IDs is not enough [5]; randomization vs k-degree (Ying 2009) [2]; MLDA multi-level k-degree on *directed* graphs with fake-node edge edits [1]; 1HIkDA 1-hop indistinguishability + k-degree [3]; sequential KG publish kw-tad so *w* consecutive releases still 1/k [6]; personalized k-attribute-degree on KGs [12]. Distinct from `paper-graph-differential-privacy` (query-time noise, graph unmodified), `paper-graph-structured-encryption` (ciphertext lease), `paper-graph-watermark-fingerprint` (ownership mark on a sealed copy). Tabular (α,k) and generic PPDP surveys stay cousins [15][19]. DP-gSpan stays queued (privacy×mining).

## 1. Raw idea

Before you *release* a graph, rewrite it so an adversary who knows a target’s degree (or 1-hop) cannot pick them out of k lookalikes [5]. The published graph is not the fold.

## 2. STCA applicability

Security: anonymization is a **pack on export**, fail-closed to “do not publish.” Data: the k-anonymous graph is a droppable lease; the event log stays unmarked (same rule as watermark). Time: sequential releases need a time-varying principle [6], not a one-shot rewrite. Verify: utility metrics (APL/ACC) are not quantum receipts. LLM does not choose k.

## 3. Quality / cost

Usefulness high wherever a fold leaves the box. Optimality med: NP-hard edits; greedy/degree-sequence heuristics. Cost: P0 = do not publish raw; honeycomb = optional k-degree export pack. Do not anonymize the live log. Do not answer MATCH from the published surrogate as SoT.

## 4. Demand

Counsel dumps, research mirrors, “share this matter graph.” Engine demand: an explicit publish transform with k, fail-closed to withhold.

## 5. Niche → effect

Legal/enterprise: a published fold (opposing-party export, research dump) must not re-identify a party from degree or 1-hop structure — k-anonymity is a *publish transform*, not query-time DP noise and not a watermark
