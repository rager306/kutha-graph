---
id: paper-regular-path-queries
source: paper
axes: [Query, Data, Composition]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Variable-length paths are RPQs with a path mode — not WCOJ and not journeys

Papers: [Cypher/GQL vs RPQs](https://consensus.app/papers/details/f21b7cc2f07a54c08950310604226872/?utm_source=cursor) (Gheerbrant et al., 2025, ICDT) original Cypher cannot express *all* RPQs; GQL adds pattern features to close the gap [1]; [dl-CRPQ](https://consensus.app/papers/details/391bd06c00f05d62952787476b1d32b7/?utm_source=cursor) (Libkin et al., 2025, PODS) real languages add node+edge, path/list variables, **path modes**, data filters — automata should drive optimization [3]; [RPQ semantics](https://consensus.app/papers/details/aae9e1eb5092532487fe8eb926922e6b/?utm_source=cursor) (Marsault et al., 2026) walk vs trail vs simple vs shortest are *functions* to a finite set of walks; some properties mutually exclusive [6]; Liang et al. (VLDB 2024) **transitive restricted** regexes cover >99% of real RSPQs; simple-path RSPQ is NP-hard in general [8]; SPARQL-log corpus: 148.7M RPQs collapse to 572 syntactic shapes [5]; compact k²/ring evaluation of RPQs [4][15][16]. Distinct from `paper-lftj-wcoj` / `paper-subgraph-iso-vs-homomorphism` (conjunctive MATCH, fixed hop pattern), `paper-tvg-journeys-restless` (time-respecting *temporal* paths), SPARQL counting property-paths (queued skip).

## 1. Raw idea

`(p)-[:KNOWS*]->(q)` is not a join of `KNOWS` with itself. It is a **regular path query**: an automaton over edge labels, product with the graph [1][3]. You must pick a **path mode** (walk / trail / simple / shortest) or the answer set is infinite [6]. Most production regexes are easy (transitive restricted) [8]; arbitrary simple paths are not. Cypher historically under-expressed RPQs; GQL/SQL-PGQ closed some of that [1].

## 2. STCA applicability

Query 070: compile `*` / RPQ to an automaton + product traversal (or matrix algebra on labeled CSR), *beside* leapfrog for conjunctive cores. Data: k²/ring is an optional RPQ lease (cousin of succinct-graph card), not the hop SoT. Composition: path mode is a **query contract** in the dictionary, not an LLM default. Time: TVG journeys already closed — those add *time-respecting* constraints; RPQ is the untimed navigational noun. Do not enumerate infinite walks.

## 3. Quality / cost

Usefulness high: every Cypher surface has `*`. Optimality high: automata product is the right algorithm class; restricted regexes are the practical fragment. Cost: P0 = bounded `*1..k` as unrolled joins (sound for small k); honeycomb = NFA product + explicit path mode (default **trail**); skip query-by-example regex inference unless a pack.

## 4. Demand

Citation chains, `knows+`, and `subClassOf+` are RPQs. Engine demand: fail-closed path mode; do not pretend `*` is WCOJ.

## 5. Niche → effect

`no niche`
