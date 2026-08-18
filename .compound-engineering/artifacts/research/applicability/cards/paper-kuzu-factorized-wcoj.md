---
id: paper-kuzu-factorized-wcoj
source: paper
axes: [Query, Data]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Kùzu: factorized processor with binary and multiway WCOJ

Paper: [KÙZU Graph Database Management System](https://consensus.app/papers/details/5f24e277251d5b038572b8abdf7342bd/?utm_source=cursor) (Jin et al., 2023).

## 1. Raw idea

A GDBMS aimed at m-n, cyclic, and recursive joins plus semi-structured storage. Factorized query processor with two (sometimes conflicting) goals: good factorization under many-to-many joins, and sequential scans that skip whole columns/join indices. Core operators: binary joins *and* multiway WCOJ that try to satisfy both.

## 2. STCA applicability

Query: production-shaped successor of Graphflow, complementary to Samyama leapfrog (tries) and Free Join (unification algebra). Factorized intermediates are a *layout* of a projection, not a SoT. Recursive-join morsel policies later measured inside Kuzu.

## 3. Quality / cost

High: open-source GDBMS, not a demo. Cost: copying Kuzu as Kutha core would fight event-log SoT; borrow operator mix, not the product.

## 4. Demand

Cypher-like hops with fat m-n joins: pairwise plans explode; naive WCOJ can destroy sequential scan locality.

## 5. Niche → effect

`no niche`
