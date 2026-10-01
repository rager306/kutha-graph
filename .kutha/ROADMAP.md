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

## M011: Claim / support identity — CLOSED

**Success:** two independent supports for one claim remain distinguishable; withdrawing one support does not erase the other at the same cut. Thin P→Q: a Behavior-derived claim stays eligible while its premise claim has a live support. Not full provenance polynomials; not M002; not ADR Accepted.

- [x] **S01: Portable claim_id on facts** `risk:high` `depends:[]`
  > After this: Assert may attach to an existing claim; retracting one support leaves the claim supported when another support is live; named test green.
- [x] **S02: Unknown claim fail-closed + replay lineage** `risk:medium` `depends:[S01]`
  > After this: Assert onto a never-seen claim does not append; `replay_check` rejects Behavior whose `caused_by` is not an earlier log event.
- [x] **S03: Thin P→Q derivation eligibility** `risk:medium` `depends:[S02]`
  > After this: Behavior-derived `Q` is eligible at a cut iff its premise claim still has a live support; retracting one of two supports leaves `Q` eligible; retracting the last drops eligibility without erasing historical `Q`; named test green.

### M011 tail — S04–S08 done

Proposed close-out of the semantic core (source: `docs/architecture/semantic-contract-validation.md` "Remaining decisions"). Each slice needs an explicit Active Slice lease in `.kutha/STATE.md` before any code. Order is by risk; stop when one fixture separates preserved history, current evidence, and allowed action.

- [x] **S04: Partial correction with residual intervals** `risk:high` `depends:[S03]`
  > After this: an explicit interval-patch correction leaves residual versions on both sides of the corrected VT range while whole-version `Correct` is unchanged; named test on the fixture's VT 2012 / 2021 residuals.
- [x] **S05: Persisted quantum outcome and continuation** `risk:medium` `depends:[S03]`
  > After this: budgets 0/1/2 and a crash after a committed prefix are distinguishable; missing terminal evidence is never read as success; resume is an explicit record; named test green.
- [x] **S06: Typed CSR lease keeps labels and support multiplicity** `risk:medium` `depends:[S03]`
  > After this: same endpoints with different relations or supports survive the lease; the untyped neighbor-set path stays available; named test green.
- [x] **S07: Provenance and rule-version check apart from state replay** `risk:high` `depends:[S04]`
  > After this: changing only a causal reference or a pinned rule version is detected even when the state fingerprint still matches; execution replay stays out of scope.
- [x] **S08: End-to-end candidate fixture** `risk:medium` `depends:[S04,S05,S06,S07]`
  > After this: observations 1–5 of the semantic-contract fixture hold at named cuts; incremental maintenance and clean reconstruction agree; discarded leases change no answer.

## M012a: Single-log SoT and stable references — CLOSED

**Success:** Outcomes, justifications, and resume survive as log records (sidecars are leases); Retract/Correct/justification cites use stable EventId / proposition ids; identical re-delivery does not mint a second support; `open` verifies snapshot identity against the log; persist is atomic for the log file; `Define` ids are stable across persist; VT/TT carry a declared scale; fold-hot reads use fold-internal indexes. Not M012 dictionaries-as-facts; not M002 Rocks; not ADR Accepted.

**Leased:** Active Milestone M012a; Active Slice None until a GSD phase leases one. Closing F1–F3, F6 (verify/atomic/stable Define), F7, F8 from `docs/architecture/semantic-gap-review.md`. F4/F5 wait for M012. Durability protocol waits for M002.

- [x] **S01: Log-native outcomes and justifications** `risk:high` `depends:[]`
  > After this: quantum outcomes / justifications / resume are appendable log records; sidecars are droppable leases; provenance includes those records.
- [x] **S02: Stable references for retract and cites** `risk:high` `depends:[S01]`
  > After this: Retract/Correct/justification target EventId (or proposition id), not fold-local fact_seq.
- [x] **S03: Idempotent ingest and proposition polarity** `risk:medium` `depends:[S02]`
  > After this: re-emitting an identical Assert is a no-op or keyed delivery; conflict polarity is not caller-only free strings.
- [x] **S04: Verify-on-open, atomic persist, stable Define** `risk:high` `depends:[S01]`
  > After this: `open` runs replay_check (or equivalent) against snapshot identity; persist is rename-into-place; Define ids survive re-persist.
- [x] **S05: Declared time scale** `risk:low` `depends:[]`
  > After this: VT/TT document a scale (and optional TT↔wall map); fixtures use that scale explicitly.
- [x] **S06: Fold-internal indexes for hot reads** `risk:medium` `depends:[S02]`
  > After this: `as_of` / `claim_supported_at` do not scan the full fact vector at the leased N; CSR path stays a lease.

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
- [x] **H5** — semantic governor + lean agent context (harness/docs only)

## M012: Dictionaries as facts — CLOSED

**Success:** Relation allowlist entries are versioned log facts (not ambient file-only SoT); `rule_version` equals a definition hash from a rule registry; admission status and policy version are bi-temporal meta-facts on the log; a thin action record binds resolved arguments to the admission decision; derivation eligibility is not limited to hard-coded one-hop + free-string `rule_version`. LLM output remains a proposal. Not ADR-050 six dictionaries; not M002 Rocks; not ADR Accepted.

**Closed:** Active Milestone M012 S01–S05 done. Closing F4/F5 from `docs/architecture/semantic-gap-review.md`. Freeze still blocks six dictionaries, Cypher, HNSW, legal pack, M002.

- [x] **S01: Relation allowlist as log facts** `risk:high` `depends:[]`
  > After this: allowlisted relations are appendable/versioned facts; admit consults the fold cut, not only tip YAML/env.
- [x] **S02: Rule registry (`rule_version` = definition hash)** `risk:high` `depends:[S01]`
  > After this: Behavior pins a registry hash, not a free string; unknown/mismatched rule fails closed.
- [x] **S03: Admission status and policy version as meta-facts** `risk:high` `depends:[S01]`
  > After this: admission decisions and the policy version are queryable AS OF a cut; `check_admission` is on the product path, not tests-only.
- [x] **S04: Thin action record** `risk:medium` `depends:[S03]`
  > After this: an Action (or equivalent Op) binds resolved arguments to admission + policy version; named test green.
- [x] **S05: Multi-hop / n-ary derivation eligibility** `risk:medium` `depends:[S02]`
  > After this: eligibility is not only one-hop `caused_by`; a two-hop (or n-ary) fixture is named and green — without claiming full provenance polynomials.

## Later milestones (not active)


Do not start until STATE names them. **M012 is CLOSED** (S01–S05 done). M012a and M011 are CLOSED. M002+ stay frozen until leased.

Proposed order (long horizon, non-authoritative; sequence not calendar; re-derive each step with `/gsd-new-milestone` when leased). Ids are given only to the nearest steps; later waypoints stay unnumbered until a lease creates them (inflation guard).

1. **M011** — CLOSED (S01–S08). Historical record above.
2. **M012a** — CLOSED (S01–S06). Detail above.
3. *(unnumbered)* Benchmark baseline — immediately after M012a indexes; not a new three-digit id.
4. **M012** — CLOSED (S01–S05). Detail above.
5. **M002** — Log durability protocol first: segmented append-only log, hash chain, atomic manifest, lease identity verified on open, replay parity across stores. Rocks only behind that protocol, for indexes — not a second SoT. Vacuum policy and litigation hold stay in this horizon, not as the milestone identity.
6. *(unnumbered)* Thin real-text legal golden fixture — test-only, under a lease, before Cypher.
7. **M003** — Cypher skin over the already-correct AS OF cut.
8. **M004** — Science fixture (second vertical → then pack lifecycle has a reason).
9. *(unnumbered)* Pack lifecycle and Cui allocation — only after two verticals run (rule of three).
10. **M005** — HNSW fence as retrieve-not-truth; exact AS OF must not be fakeable by ANN.
11. *(unnumbered)* Security — ABAC rewrite, grants @T, agent sandbox; required before any enterprise ship.
12. *(unnumbered)* Packaging, naming and license decision (ADR-092) before any publish.

P4 (Raft/multi-node, learned indexes) is not planned until single-node hybrid evidence exists.

## Freeze until explicit M002 lease

RocksDB crate, Cypher parser, HNSW, ABAC rewrite, RVF as hot store, bulk honeycomb Accepted, Consensus 103+. M001 is closed; freeze still holds.
