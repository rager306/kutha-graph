# Kutha execution roadmap

Non-authoritative delivery pyramid. Architecture SoT remains `docs/ADR/`. Honeycomb is a **map**, not this list.

**Vision:** Event-sourced bi-temporal graph engine; LLM proposes; log and dictionaries own audited truth. Wedge: legal temporal agents.

## M001: Legal PIT steel thread — CLOSED

**Success:** same log, two valid-times, different live triples; replay bit-stable; CSR cut agrees with fold. Cosine is not the oracle.

**Closed:** S01–S03 delivered. Historical lease was `L_delivery=M001-closed` before M010. Not product Accepted; not M002.

- [x] **S01: Named AS OF cut + fixture** `risk:high` `depends:[]`
  > After this: `as_of(T1) ≠ as_of(T2)` on a three-fact statute-shaped log; FF5 test exists.
- [x] **S02: CSR agrees with the cut** `risk:medium` `depends:[S01]`
  > After this: rebuilding the CSR lease at T_old vs T_new matches the fold.
- [x] **S03: Relation allowlist fail-closed** `risk:low` `depends:[S01]`
  > After this: unknown relation does not append (FF6 stub, not six dictionaries).

## M010: Semantic recovery without snapshot lease — CLOSED

**Success:** discarding `snapshot.json` still restores term meanings and fold fingerprint from retained history; missing authoritative terms fail explicitly. Live intern appends `Op::Define`. Not M002 Rocks; not ADR Accepted.

**Closed:** S01–S03 delivered. Historical lease was `L_delivery=M010-closed`. Not product Accepted; not M002.

- [x] **S01: Open recovers intern meanings** `risk:high` `depends:[]`
  > After this: persist writes `terms.jsonl`; `open` without snapshot replays events with restored dictionary; named test green.
- [x] **S02: Logged term-definition ops** `risk:medium` `depends:[S01]`
  > After this: durable `Op::Define` prefix in events.wal / events.jsonl restores intern meanings without `terms.jsonl`; sidecar remains a derived picture.
- [x] **S03: Live intern appends Define** `risk:medium` `depends:[S02]`
  > After this: `Runtime::intern` logs `Op::Define` for new terms; bootstrap stays silent; graph-oriented snapshot offset; named test green.

## M011: Claim / support identity — ACTIVE

**Success:** two independent supports for one claim remain distinguishable; withdrawing one support does not erase the other at the same cut. Thin P→Q: a Behavior-derived claim stays eligible while its premise claim has a live support. Not full provenance polynomials; not M002; not ADR Accepted.

- [x] **S01: Portable claim_id on facts** `risk:high` `depends:[]`
  > After this: Assert may attach to an existing claim; retracting one support leaves the claim supported when another support is live; named test green.
- [x] **S02: Unknown claim fail-closed + replay lineage** `risk:medium` `depends:[S01]`
  > After this: Assert onto a never-seen claim does not append; `replay_check` rejects Behavior whose `caused_by` is not an earlier log event.
- [x] **S03: Thin P→Q derivation eligibility** `risk:medium` `depends:[S02]`
  > After this: Behavior-derived `Q` is eligible at a cut iff its premise claim still has a live support; retracting one of two supports leaves `Q` eligible; retracting the last drops eligibility without erasing historical `Q`; named test green.

## Boundary map (M001)

### M001 S01 → S02

Produces: named `live_at`/`as_of` on `GraphFold`; golden fixture under `crates/kutha-runtime`.  
Consumes: existing `Fact::is_live_at`.

### M001 S02 → S03

Produces: CSR rebuild at a cut.  
Consumes: S01 fixture.

### M010 S01 → S02

Produces: open without snapshot from `terms.jsonl` + event replay.  
Consumes: persist path; ADR-011 semantic recovery contract.

### M010 S02

Produces: `Op::Define` prefix on the durable event stream; open without snapshot or terms sidecar.  
Consumes: S01 persist/open; intern order = Define order.

### M010 S02 → S03

Produces: live in-memory log carries Define for new interns (SoT before persist).  
Consumes: S02 Op::Define schema; fold no-op; encoded_log still synthesizes durable Define prefix.

### M011 S01

Produces: `Fact.claim_id` + optional Assert.claim; live support queries.  
Consumes: M010 recovery; existing Retract + `is_live_at`.

### M011 S01 → S02

Produces: fail-closed unknown claim; replay checks Behavior `caused_by` against prior event ids.  
Consumes: S01 claim_id; ADR-060 obligation 2 (causal refs only, not execution replay).

### M011 S02 → S03

Produces: `derivation_eligible_at` oracle for Behavior-derived claims keyed on premise `claim_id`.  
Consumes: S01 supports; S02 `caused_by` lineage; candidate fixture in `docs/architecture/semantic-contract-validation.md` (thinned).

## Harness dogfood (process plane, not M00x)

- [x] **H0** — dictionary FSM + file trajectory.
- [x] **H1** — `kutha-gov ci` observes named FF tests via `cargo test` (evidence, not SoT).
- [x] **H2** — harness JSONL ingested onto the Kutha log; query AS OF last process status.
- [x] **H3** — fail-closed process writes through a relation allowlist.
- [x] **H4** — process-relation membership editions logged as snapshots and queryable AS OF a prior tenant cut (tip YAML stays the admit lease; full ADR-090 ontology stays frozen).

## Later milestones (not active)

Do not start until STATE names them. M011 is active (S03 done; further slices need lease); M002+ stay frozen until leased.

- **M002** — Rocks adapter behind the same events (persistence, not a second SoT).
- **M003** — Cypher skin over the already-correct AS OF cut.
- **M004** — Science fixture (second vertical → then pack lifecycle has a reason).
- **M005** — HNSW fence as retrieve-not-truth.

## Freeze until explicit M002 lease

RocksDB crate, Cypher parser, HNSW, ABAC rewrite, RVF as hot store, bulk honeycomb Accepted, Consensus 103+. M001 is closed; freeze still holds.
