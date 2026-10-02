# Kutha execution roadmap

Non-authoritative delivery pyramid. Architecture SoT remains `docs/ADR/`. Honeycomb is a **map**, not this list.

**Vision:** Event-sourced bi-temporal graph engine; LLM proposes; log and dictionaries own audited truth. Wedge: legal temporal agents.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not copy the table onto this ROADMAP. Controller protocol lives in the same file.

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

## M013: Honest baseline and one write path — CLOSED (GSD v0.06)

**Success:** a declared target N and a re-runnable measured baseline exist; a real-text legal fixture has been mapped on paper against the current model; ingest, retract, correct, and claim-admit use fold indexes, not log scans; the initial relation allowance is genesis events in the log and a seeded relation can be retracted; dictionary entries are ordinary facts in a reserved namespace (the five meta side tables are gone); every record kind enters through one propose, admit, commit, receipt path and is consumed through a public facade by one example and one integration test; references are `EventId` or hash; the milestone closes only with an independent verification artifact and a planning-artifact budget check. Not rules, not typed clocks, not M002, not ADR Accepted.

**Closed:** Active Milestone M013 delivered S01–S09 (2026-10-02); GSD tag `v0.06`; archive `.planning/milestones/v0.06-*`. Closes F10, F11, F12, F14, F15 and R2, R4, R5, R6 from `docs/architecture/semantic-gap-review.md` (addendum 2026-10-01). No new features in S03–S06: refactors under named oracles (FF5, FF6, M010–M012). Thawed subset was scoped and freeze restored. Still frozen: RocksDB, Cypher, HNSW, six dictionaries, legal pack, M002.

- [x] **S01: Benchmark baseline and size target** `risk:medium` `depends:[]`
  > After this: a bench harness with no new dependencies measures ingest, `as_of`, retract, persist, open, and replay at three sizes, reports bytes per fact, and states a target N; one command re-runs it; numbers land in a dated note, not in an ADR.
- [x] **S02: Legal fixture reconnaissance** `risk:high` `depends:[]`
  > After this: a note lists real-text cases from law-nexus (FZ-44 snippets, consultant sources with editions, the synthetic sunset clause), maps each against the two-axis model, and names the cases the current model cannot express. No product code.
- [x] **S03: Fold indexes for ingest** `risk:medium` `depends:[S01]`
  > After this: delivery-key retry, claim admit, retract, correct, and follow-ons do no whole-log or whole-fact scans; a scaling oracle is RED before the change and GREEN after; the baseline is re-measured.
- [x] **S04: Genesis log** `risk:high` `depends:[S03]`
  > After this: relation allowance enters through genesis events; the YAML file is read only when a new log is created; retracting a seeded relation makes admit fail closed; FF6 stays green.
- [x] **S05: Dictionary entries as facts** `risk:high` `depends:[S04]`
  > After this: relation, rule, policy, admission, and action records are reserved-namespace facts with content-addressed bodies; the five meta `Op` variants and side tables are gone; the M012 oracles keep their assertions.
- [x] **S06: One write path and facade** `risk:high` `depends:[S05]`
  > After this: every record kind goes through one propose, admit, commit, receipt pipeline; justification and policy references are `EventId` or hash; the runtime is split by concern; a public facade is driven by one example and one integration test that use nothing else.
- [x] **S07: Independent verification and artifact budget** `risk:medium` `depends:[S06]`
  > After this: a governor check enforces the planning-artifact budget; milestone close requires a verification artifact from a verifier that did not write the code; the id map (GSD version, harness milestone, ADR) is one table.

### M013 tail — S08–S09 (review follow-ups)

Added 2026-10-02 after the v0.06 milestone audit (`gaps_found`) and the project review of roadmap, ADR, code, tests, and harness. M013 CLOSED after S09; process tag `v0.06`; no product SemVer. Each later slice needs its own lease.

- [x] **S08: Close-out honesty — documents and governor** `risk:medium` `depends:[S07]`
  > After this: stale M013/v0.06 bookkeeping is consistent across ROADMAP, PROJECT, REQUIREMENTS, id map, and the milestone audit; a post-M013 addendum records which semantic-gap findings are closed and which stay open; Proposed ADR cells carry dated Evidence notes that match the code; the governor fails closed on FSM budget truncation and on an empty budget glob, and the independence check covers every phase VERIFICATION. No file under `crates/` changes.
- [x] **S09: Write-path integrity hardening** `risk:high` `depends:[S08]`
  > After this: dictionary-namespace writes fail closed outside the typed constructors; `correct` keeps `delivery_key`; one oracle runs FF5, retract, correct, and claim support under both Eager and Lazy fold index modes; the facade test asserts exact live triples after assert, correct, retract, persist/open, and fork. Named oracles keep their assertions; no typed clocks, no rules.

### Process gates (apply from S01, harness plane)

- **Independent verify:** each phase ends with a verifier that receives requirements and the public API only, writes kill-tests, and does not read the executor's plan or summary. `/gsd-verify-work` is the human-facing form. Executor-written tests and governor green are not verification.
- **Artifact budget:** per phase at most two PLAN files of 150 lines or fewer, one SUMMARY of 60 lines or fewer, one VERIFICATION of 60 lines or fewer. No per-plan evidence JSON. A phase that exceeds the budget stops for a decision.
- **Tool pair:** CBM and GitNexus are both used on every phase that edits product code (`docs/process/gsd-auto-contract.md`).
- **RED first:** each slice that closes a finding starts with an oracle that fails on current code, recorded before the change.

## M014: Typed clocks and as-known-on — CLOSED (GSD v0.07)

**Closed:** 2026-10-02. Process tag `v0.07`. S01–S03 complete.

**Success:** valid time, transaction time, and wall time are distinct types; mixing them without a conversion does not compile; `tt_anchor` facts map transaction time to wall time; “as known on date D” is an anchor lookup; FF5 stays green with explicit clocks. Closes F13 (clock/anchor ship). Residual known-on cut → M015 / v0.08.

- [x] **S01: Clock newtypes** `risk:high` `depends:[]`
  > After this: `Vt`, `Tt`, and a wall clock are separate types; a `compile_fail` doctest proves a silent mix is rejected.
- [x] **S02: `tt_anchor` facts and `as_known_on`** `risk:medium` `depends:[S01]`
  > After this: anchors are `Op::TtAnchor` log records (fold lease `tt_anchors`); `as_known_on(date)` resolves through the greatest anchor at or before the date; a named test separates “true on D” from “known on D”.
- [x] **S03: Fixture migration** `risk:low` `depends:[S02]`
  > After this: FF5 and the M011/M012 fixtures use explicit clocks with unchanged assertions.

## M015: Known-on honesty and M014 residuals — CLOSED (GSD v0.08)

**Closed:** 2026-10-02. Process tag `v0.08`. S01–S02 complete.

**Success:** `as_known_on(D)` does not drop closed VT intervals; duplicate/backwards `TtAnchor` walls fail closed; living docs and governor match post-M014 (ANK required; budget min_hits when phases live). Closes audit H1/H2.

- [x] **S01: Known-on cut and anchor admission** `risk:high` `depends:[]`
  > After this: FF5-shaped closed-interval + anchors RED→GREEN; monotone unique walls only; FF5 `as_of` unchanged.
- [x] **S02: Living docs and governor honesty** `risk:medium` `depends:[S01]`
  > After this: living docs/PROJECT/semantic-gap/ADR-013 Evidence aligned; `true_on_d_differs_from_known_on_d` in observe_cargo; artifact-budget `min_hits: 1` when live phases exist.

## M016: Test hardening + executable rules — CLOSED (GSD v0.09)

**Closed:** Active Milestone M016; S01–S05 done. Process tag `v0.09`. Crate SemVer stays 0.0.0. Honeycomb stays Proposed.

**Success:** T2–T6 test holes closed; canonical rule AST hashed into `rule_version`; rule bodies evaluated over the CSR lease with support counting; admission evaluates the pinned policy; execution replay re-derives; FIXG-01 fail-closed (C01 sunset is not a Horn; ADR-013 Evidence dated). Closes T2–T6, F9, F16.

- [x] **S01: Test hardening T2–T6** `risk:medium` `depends:[]`
  > After this: trybuild E0308 clock mix; cite tt/vt oracle; Eager/Lazy/fork known-on; snapshot+tail anchors; `GraphFold::replay` applies TtAnchor.
- [x] **S02: Rule AST and hash** `risk:high` `depends:[S01]`
  > After this: a positive, bounded rule language has a canonical encoding; `rule_version` is its hash; unknown or non-canonical bodies fail closed.
- [x] **S03: Evaluator and support counting** `risk:high` `depends:[S02]`
  > After this: multi-hop, n-ary, and `inverse_knows` are rules; retraction of a premise maintains derived supports; the hard-coded cascade is deleted.
- [x] **S04: Policy evaluation and execution replay** `risk:high` `depends:[S03]`
  > After this: admission runs the pinned policy body; changing the policy changes a decision; replay re-derives derived facts and fails on any it cannot reproduce.
- [x] **S05: Legal golden fixture and verdict** `risk:high` `depends:[S04]`
  > After this: a real-text fixture with one transitional rule and an as-known-on query is green, or the failing case is recorded and the plan stops before M002. The verdict is written into ADR-013 as a dated clarification.

## Later milestones (not active)

Do not start until STATE names them. M011, M012a, and M012 are CLOSED. M013 is CLOSED. M014 is CLOSED. M015 is CLOSED. M016 is CLOSED. M002+ stay frozen until leased.

Proposed order (long horizon, non-authoritative; sequence not calendar; re-derive each step with `/gsd-new-milestone` when leased). Ids are given only to the nearest steps; later waypoints stay unnumbered until a lease creates them (inflation guard).

1. **M011** — CLOSED (S01–S08). Historical record above.
2. **M012a** — CLOSED (S01–S06). Detail above.
3. **M012** — CLOSED (S01–S05). Detail above.
4. **M013** — CLOSED (S01–S09, GSD v0.06 tagged). Detail above.
5. **M014** — CLOSED. Typed clocks and as-known-on. Detail above.
6. **M015** — CLOSED. Known-on honesty and M014 residuals (v0.08). Detail above.
7. **M016** — CLOSED (S01–S05, GSD v0.09 tagged). Detail above.
8. **M002** — Log durability protocol: segmented append-only log, hash chain, atomic manifest, replay parity across stores, with the quantum outcome as a commit marker in segment framing (ADR-010/014 open question). “Lease identity verified on open” is already done (v0.04 Phase 15). Rocks only behind that protocol, for indexes — not a second SoT. Vacuum policy and litigation hold stay in this horizon, not as the milestone identity.
9. **M003** — Cypher skin over the already-correct AS OF cut.
10. **M004** — Science fixture (second vertical → then pack lifecycle has a reason).
11. *(unnumbered)* Pack lifecycle and Cui allocation — only after two verticals run (rule of three).
12. **M005** — HNSW fence as retrieve-not-truth; exact AS OF must not be fakeable by ANN.
13. *(unnumbered)* Security — ABAC rewrite, grants @T, agent sandbox; required before any enterprise ship.
14. *(unnumbered)* Packaging, naming and license decision (ADR-092) before any publish.

P4 (Raft/multi-node, learned indexes) is not planned until single-node hybrid evidence exists.

## Freeze until explicit M002 lease

RocksDB crate, Cypher parser, HNSW, ABAC rewrite, RVF as hot store, bulk honeycomb Accepted, Consensus 103+. M001 is closed; freeze still holds. M013 thaw was scoped and freeze restored; do not treat M013 as a live thaw.
