---
id: paper-provenance-semirings
source: paper
axes: [Verify, Query, Data]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: "Legal/science: how-polynomials explain a MATCH as +/× of events — not why-not absence, not PACT tool args, not a crypto receipt"
status: closed
channels_failed: []
---

# How-provenance is a semiring polynomial — not why-not and not a hash

Papers: [Green/Karvounarakis/Tannen](https://consensus.app/papers/details/5728ebae04d75009a670bf632c0a7d37/?utm_source=cursor) (2007) provenance semirings: why, bag, incomplete, probabilistic as instances of the same polynomial algorithm; Datalog via formal power series [2][9]; graph/RPQ provenance via specialized graph algorithms per semiring class (shortest, top-k, Boolean, security) [1][3]; SPARQLprov: rewrite SPARQL to how-polynomials on deployed engines, spm-semirings for non-monotone SPARQL [10][20]; NPCS native SPARQL how-provenance including RDF-star [17]; dual-indeterminate polynomials for negation and reverse analysis (find models / missing answers) [5][11]; query-rewriting polynomials for SPJUA without forking the DBMS [4]; HAPPI: commutative semiring for probabilistic KG result *and* incremental maintenance [6]. Distinct from `paper-why-not-query-provenance` (absence/missing hops), `paper-pact-argument-provenance` (tool-call arguments), `paper-constant-size-evidence` (crypto receipt size), `paper-memlineage-memory-custody` (agent memory chain). OBDA/ELHr provenance stays a cousin of the OBDA card. ProvSQL is a Postgres module, not Kutha core.

## 1. Raw idea

Annotate each event/triple with a token. The query result carries a **polynomial**: `+` is alternative derivations, `×` is joint use [2]. Evaluating the polynomial in a semiring yields why, how, confidence, or cost [1][9]. That is algebraic lineage, not a boolean “missing” and not a Merkle path.

## 2. STCA applicability

Verify: the polynomial is the explainable fold of a compiled query — a receipt *of derivation*, not of the log’s integrity. Query: MATCH/RPQ compile to a provenance-aware evaluator (or a rewrite that returns polynomials) [1][10]. Data: tokens intern with the dictionary; dropping the polynomial is dropping a lease. Time: as-of means polynomials over the slice, not a new clock. Agent: LLM does not invent coefficients. Negation needs dual indeterminates [5] — keep P0 to positive conjunctive/RPQ.

## 3. Quality / cost

Usefulness high: “which events produced this hop.” Optimality high: the semiring abstraction is the standard. Cost: P0 = why-tokens on conjunctive MATCH (lineage tuples); honeycomb = polynomial rewrite / graph semiring algorithms. Do not vendor ProvSQL. Do not confuse with quantum/constant-size evidence.

## 4. Demand

Counsel asks *how* the graph entailed the answer. Engine demand: optional how-polynomials on compiled queries, fail-closed to “result without lineage.”

## 5. Niche → effect

Legal/science: how-polynomials explain a MATCH as +/× of events — not why-not absence, not PACT tool args, not a crypto receipt
