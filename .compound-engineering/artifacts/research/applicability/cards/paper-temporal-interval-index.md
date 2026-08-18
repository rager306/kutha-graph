---
id: paper-temporal-interval-index
source: paper
axes: [Time, Query, Data]
usefulness: high
optimality: high
demand: high
confidence: paper
layer5: no niche
status: closed
channels_failed: []
---

# Validity intervals need an interval index — not a QL and not snapshot placement

Papers: [Timeline Index](https://consensus.app/papers/details/fd8a9b2d402159ec9512d0d69ff7f196/?utm_source=cursor) (Kaufmann et al., 2013, SAP HANA) unified structure for temporal aggregation, time travel, temporal joins; independent of physical order so any compression works [16]; [TIDE](https://consensus.app/papers/details/c16e3dfce0c15ccb8b12c9e63cd96c43/?utm_source=cursor) (Wang et al., 2025) duration×endpoint two-level append-only B+; up to 100× insert / 7000× query vs competitor [1]; [MAP21](https://consensus.app/papers/details/55f0e093c367520ba62104beb839113d/?utm_source=cursor) (Nascimento & Dunham, 1999, TKDE) map ranges to points on ordinary B+ trees [3]; [IB+tree time-splits](https://consensus.app/papers/details/18b80753a0b15ae495046d145dff80ec/?utm_source=cursor) (Bozkaya & Özsoyoğlu, 1998) [2]; [interval index for temporal subgraphs](https://consensus.app/papers/details/781761b7f2f854d0af84fc34325ec182/?utm_source=cursor) (Ouyang et al., 2026, VLDB) sub/super-valid window retrieval; linear build, near-optimal query, O(Δ) insert [10]. Distinct from `paper-tgql-intervals` (query *language* + path semantics), `paper-tvg-journeys-restless` (which journeys are computable), `paper-lsm-snapshot-compaction` (where to *place* snapshots), `paper-rost-bitemporal` (model). T-GQL Neo4j paper [13 in Q72] is already closed as the QL card.

## 1. Raw idea

A fact has a validity interval `[t0, t1)`. AS-OF / overlap / contain queries are interval range queries. Timeline Index is the modern column-store answer [16]. TIDE/CEB are disk B+ two-level (duration or center × endpoint) under append-only inserts [1][17]. MAP21/IB+ show you can stay on *ordinary* B+ trees [3][2]. Temporal-*subgraph* indexes answer “which stored subgraphs sit in this window” [10] — same interval noun, graph payload.

## 2. STCA applicability

Time 012/Query 070: the event log is ordered by transaction time; **valid-time** still needs an interval access path or every AS-OF scans the fold. Data: the index is a reversible lease (like CSR), keyed by interned ids (`paper-rdf-term-dictionary`). Do not store Neo4j interval properties as the index. Journeys remain algorithms on top of “which edges are live in `[t0,t1)`”.

## 3. Quality / cost

Usefulness high: every legal PIT query is an interval stab. Optimality high: Timeline Index / TIDE beat R-trees and naive B+ on this shape [16][1]. Cost: P0 = scan by log offset (transaction time only); honeycomb = Timeline-style or MAP21 B+ on `(valid_from, valid_to)` of events; vacuum old intervals to cold k²/HDT.

## 4. Demand

AS-OF without an interval index is a sequential fold. Engine demand: one access path for overlap/contain/timeslice on the same events leapfrog already joins.

## 5. Niche → effect

`no niche`
