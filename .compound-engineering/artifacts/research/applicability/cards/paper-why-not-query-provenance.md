---
id: paper-why-not-query-provenance
source: paper
axes: [Verify, Query, Agent]
usefulness: high
optimality: med
demand: high
confidence: paper
layer5: "Legal AS-OF: missing statute hops explained as why-not on the compiled query, not as an LLM apology"
status: closed
channels_failed: []
---

# Why / why-not of a query answer is a compiled explanation — not a chat rationale

Papers: [PUG](https://consensus.app/papers/details/500a412f6a4c51a59361bd5c93784415/?utm_source=cursor) (Lee et al., 2018, VLDB J., DOI: 10.1007/s00778-018-0518-5) why *and* why-not for FO queries with negation; Datalog firing rules; reverse reasoning [1][8]; [why-not polynomials](https://consensus.app/papers/details/a46f4a4a19dd5212ad6b10fefeb078e5/?utm_source=cursor) / [NedExplain](https://consensus.app/papers/details/6e357c6395e558009608a25721b291f1/?utm_source=cursor) query-tree-independent explanations [2][9]; [SPARQLprov](https://consensus.app/papers/details/e30b7f19ca5a5adb81acc53e0cbf02ff/?utm_source=cursor) (Hernández et al., 2021) how-provenance polynomials via SPARQL rewrite, engine-agnostic [4]; [HUKA](https://consensus.app/papers/details/cfa36c0c9cba52f09a1cd89eda175fca/?utm_source=cursor) (Gaur et al., 2020) *maintain* polynomials on dynamic KGs instead of recompute [13]; [Erebus](https://consensus.app/papers/details/1b6a75fe838f5667a393c5495411a8a6/?utm_source=cursor) streaming missing-answer expectations [14]; [Meliou causality/responsibility](https://consensus.app/papers/details/868a1a72047650f7ab936f8489ab0efb/?utm_source=cursor) (2010) causes ⊆ lineage; responsibility ranks them; CQ causality in PTIME, responsibility dichotomy [18]; [why-provenance for Datalog](https://consensus.app/papers/details/5621ee37dabf5938870ef3a4a9628b99/?utm_source=cursor) (Calautti et al., 2023) recursive why-provenance intractable even for linear recursion [20]. Distinct from `paper-pact-argument-provenance` (LLM tool-argument contracts), `paper-memlineage-memory-custody` (tainted agent memory), `paper-constant-size-evidence` (crypto receipts).

## 1. Raw idea

**Why**: which input tuples/edges derived this answer (lineage / how-polynomial / semiring). **Why-not**: where in the query (or data) the expected tuple died — frontier-picky vs polynomial explanations [6][17]. Negation needs failed derivations, not only successful ones [1]. Full provenance explodes; capture only what the question names, then summarize with patterns [3][16]. Rewrite-based capture (SPARQLprov, DataProv) rides an existing engine [4][7]. On a mutating KG, maintain the polynomial [13]. Streaming: declare *expectations* and explain divergence [14].

## 2. STCA applicability

Verify/Query: an AS-OF Cypher result must be able to answer “why is article X not here?” as **why-not on the compiled plan** (dictionary + time-respecting hops + grants), logged as facts. Agent: the LLM may *ask* the why-not port; it must not invent the polynomial. Time: HUKA-style incremental provenance is a lease over the log, cousin of DBSP — recompute-from-scratch on every invalidation is the failure mode. Distinct from PACT (capability envelope of a *tool call*) and MemLineage (whether a *memory* may act).

## 3. Quality / cost

Usefulness high: missing legal hops are the product question. Optimality med: recursive Datalog why is hard [20]; CQ causality is cheap [18]. Cost: why-not as a Query/Verify pack over compiled Cypher (rewrite like SPARQLprov); polynomials on the fold; no provenance SoT separate from the event log.

## 4. Demand

Users will not trust “no rows.” Engine demand: explanation port. NL highlighting of the guilty clause [6] is UX over the same polynomial.

## 5. Niche → effect

Legal AS-OF: missing statute hops explained as why-not on the compiled query, not as an LLM apology.
