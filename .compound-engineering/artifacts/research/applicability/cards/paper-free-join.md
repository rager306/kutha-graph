---
id: paper-free-join
source: paper
axes: [Query]
usefulness: high
optimality: high
demand: med
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Free Join: unify WCOJ and binary joins

Paper: [Free Join](https://doi.org/10.1145/3589295) (Wang et al., 2023). Consensus: https://consensus.app/papers/details/82bccd7bd2555ec9842b46e309ec78ed/?utm_source=cursor

## 1. Raw idea

One plan/data structure instead of “WCOJ only on cyclic fragments, binary elsewhere.” Rust implementation matches or beats both paradigms on standard benchmarks.

## 2. STCA applicability

Query: hybrid planner (ADR-043). Acyclic legal/science patterns should not pay full LTJ; cycles should not explode pairwise.

## 3. Quality / cost

High planner-fit. Cost: new physical algebra in Kutha; Samyama already has leapfrog — Free Join is the *unification* story.

## 4. Demand

Real Cypher mixes stars and cycles.

## 5. Niche → effect

`no niche`
