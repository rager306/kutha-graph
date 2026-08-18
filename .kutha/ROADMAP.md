# Kutha execution roadmap

Non-authoritative delivery pyramid. Architecture SoT remains `docs/ADR/`. Honeycomb is a **map**, not this list.

**Vision:** Event-sourced bi-temporal graph engine; LLM proposes; log and dictionaries own audited truth. Wedge: legal temporal agents.

## M001: Legal PIT steel thread

**Success:** same log, two valid-times, different live triples; replay bit-stable; CSR cut agrees with fold. Cosine is not the oracle.

- [x] **S01: Named AS OF cut + fixture** `risk:high` `depends:[]`
  > After this: `as_of(T1) ≠ as_of(T2)` on a three-fact statute-shaped log; FF5 test exists.
- [x] **S02: CSR agrees with the cut** `risk:medium` `depends:[S01]`
  > After this: rebuilding the CSR lease at T_old vs T_new matches the fold.
- [ ] **S03: Relation allowlist fail-closed** `risk:low` `depends:[S01]`
  > After this: unknown relation does not append (FF6 stub, not six dictionaries).

## Boundary map

### S01 → S02

Produces: named `live_at`/`as_of` on `GraphFold`; golden fixture under `crates/kutha-runtime`.  
Consumes: existing `Fact::is_live_at`.

### S02 → S03

Produces: CSR rebuild at a cut.  
Consumes: S01 fixture.

## Later milestones (not active)

Do not start until M001 capability is green.

- **M002** — Rocks adapter behind the same events (persistence, not a second SoT).
- **M003** — Cypher skin over the already-correct AS OF cut.
- **M004** — Science fixture (second vertical → then pack lifecycle has a reason).
- **M005** — HNSW fence as retrieve-not-truth.

## Freeze until M001 S03 (FF6 stub)

RocksDB crate, Cypher parser, HNSW, ABAC rewrite, RVF as hot store, bulk honeycomb Accepted, Consensus 103+. S01/S02 (named `as_of` + CSR cut) are done.
