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

**Closed.** Historical lease was M012a. Closing F1–F3, F6 (verify/atomic/stable Define), F7, F8 from `docs/architecture/semantic-gap-review.md`. F4/F5 wait for M012. Durability protocol waits for M002.

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

**Success:** T2–T6 test holes closed; canonical rule AST hashed into `rule_version`; rule bodies evaluated over live facts (`fold.facts()`) with support counting; eager path uses `horns_by_premise` since M017 S04; CSR is not used by rules; admission evaluates the pinned policy; execution replay re-derives; FIXG-01 fail-closed (C01 sunset is not a Horn; ADR-013 Evidence dated). Closes T2–T6, F9, F16.

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

## M017: Docs honesty and replay integrity — CLOSED (GSD v0.10)

**Closed:** 2026-10-03. Process tag `v0.10`. S01–S04 complete; four independent VERIFICATIONs. Crate SemVer stays 0.0.0. Residuals in `.planning/milestones/v0.10-MILESTONE-AUDIT.md`.

**Source:** 2026-10-02 audit at `c702d71` (ADR/roadmap, code/tests, process docs). Closes the doc drift and the replay/durability holes that M002 would otherwise inherit. Not M002 segments or Rocks; not typed `Op` clocks; not a C01 decision; not ADR Accepted.

**Success:** living docs, ADR Evidence and the governor agree with the code (restaged 2026-10-03, Phase 45); a tampered `TtAnchor`, a truncated WAL or snapshot, a disagreeing sidecar, or an environment-dependent index mode cannot silently change an answer; replay determinism and the Horn-vs-known-on behaviour are named oracles. Closes audit findings G02–G12, A-F01, A-F02, A-F04, A-F06–A-F10 and the ingest regression found by the Phase 42 bisect (S04).

- [x] **S01: Living docs and governor honesty** `risk:low` `depends:[]`
  > After this: PROJECT carries no stale lease and `cite-lease` covers it; gap-review addendum and invariants ledger match M014–M016; ADR-000/013/014/041/050/060/070 and README carry dated clarifications (three AS OF cuts, Horn over live facts, C01 open with options); no file under `crates/` changes.
- [x] **S02: Replay integrity** `risk:high` `depends:[S01]`
  > After this: tampered `TtAnchor` fails replay; `snapshot.json` and `events.wal` replace atomically; open never lets a sidecar or an empty WAL override the log; snapshot pins the fold index mode; `as_known_on` rustdoc and `known_at` oracle are honest.
- [x] **S03: Determinism and knowledge-cut characterization** `risk:medium` `depends:[S02]`
  > After this: seeded replay==fold and Eager==Lazy (persist/open) properties are green with no new dependency; Horn-vs-known-on behaviour is recorded in a named test and ADR-013 Evidence without changing behaviour.
- [x] **S04: Ingest regression repair** `risk:high` `depends:[S03]`
  > After this: the M016 commit that tripled `ingest_plain.n` (108M to 301M ns at N=10000, found by the Phase 42 bisect) is named; a scaling oracle is RED then GREEN; ingest is within 10 percent of v0.08 on the same host or the residual is explained; named oracles unchanged.

## M018: Wedge unblock — doc truth, C01 and knowledge cut, Horn determinism (GSD v0.11)

**Source:** 2026-10-03 post-v0.10 review at `d230ab2` (five read-only audits, CBM + GitNexus, `rg` arbiter): trajectory, ADR vs code, tests, docs, code logic. The wedge (norm AS OF a date) is blocked by an undecided C01 and by Horn semantics, not by Rocks. Not M002 segments or Rocks; not typed `Op` clocks; not ADR Accepted.

**Success:** living docs and ADR Evidence agree with the code (one F-register, English spine); ADR-013 records one C01 option and the Horn-vs-knowledge-cut rule, with a named oracle; derived facts have deterministic identity and follow `Correct`; every cascade hop is admitted; fold, CSR and rebuild agree on seeded random logs.

**Paused 2026-10-03** after S01 at the S02 decision gate (needs the user's C01 / Horn choice). M002 (below) is independent of S02–S04 and is leased first; M018 resumes at S02 after M002 S01. Failed-WAL-open (OPEN-01) and crash-seam work moved to M002 S02.

- [x] **S01: Doc truth round 2** `risk:low` `depends:[]`
  > After this: ADR-010/012/014/041/060 Evidence and `semantic-gap-review.md` match v0.10 code (snapshot and WAL atomic, `horns_by_premise`, sidecars still in place); audit ids carry the `A-F` prefix; ADR-000 D6 and ADR-010 D010-4 no longer name Rocks WAL as shipped; ADR-013 states `as_of` is support-live, not eligibility; ADR-000 and `stca-guide.md` spine is English; CHANGELOG head, `PROJECT.md`, `.kutha/ROADMAP.md` closed-milestone wording and `.gitignore` hygiene are fixed; no file under `crates/` changes.
- [ ] **S02: C01 and knowledge-cut decision** `risk:high` `depends:[S01]` `gate:controller+user decision`
  > After this: ADR-013 records one C01 option (pack stamp, fifth axis, or stop) with a named oracle or an explicit out-of-wedge note; the Horn-vs-`as_known_on` rule (HORN-KC-01) and the `as_of` vs derivation-eligibility rule are decided; if behaviour changes, the oracle is RED first and `m017_known_on_horn.rs` is inverted or re-pinned by the decision.
- [ ] **S03: Horn determinism and closure** `risk:high` `depends:[S02]`
  > After this: two ingests of the same ops give the same derived-event identity (or the decision to keep UUID v7 is recorded with its replay consequence); `Correct` and `CorrectInterval` re-derive; every cascade hop passes `admit` and commits in a stable order; the unknown head or premise relation in a rule fails closed; named oracles unchanged.
- [ ] **S04: Differential tests and oracle tightening** `risk:medium` `depends:[S03]`
  > After this: fold, CSR and clean rebuild agree on seeded random logs (no new dependency); I-F15 and I-F16 oracles measure what they claim; the bench gate is a recorded controller step or a governor row. OPEN-01 (failed WAL magic read, empty WAL, WAL error hidden behind jsonl) and the crash seam between jsonl and sidecars moved to M002 S02.

## M002: Log durability protocol (GSD v0.12) — THAWED, planned

**Thawed 2026-10-03.** The freeze came from agent-assisted commits (`1ee5942`, `85db1f7`, 2026-08-18, Cursor co-authored); the owner states he did not decide it; the owner asked for M002 to be re-verified and run under GSD. Provenance and the per-item thaw map: `.kutha/STATE.md` § Thaw map. ADR-000 D1–D10 are not a lease gate: D6 names a Rust+RocksDB event store, D8.1 asks for log segmentation; the 2026-10-03 clarification keeps `events.jsonl` as SoT and Rocks as an index store.

**Source:** 2026-10-03 re-verification at `f4af544` (provenance audit, governor freeze map, durability design research; CBM + GitNexus, `rg` arbiter). Current defects: `persist` rewrites the whole log each call (O(N), `store.rs` `encoded_log`, `wal::write_wal`); sidecars `terms`/`quantum_outcomes`/`justifications` use in-place `File::create` without sync; no parent-directory fsync after any rename; `check_tt_anchors` demands exact equality so a log ahead of the snapshot fails open; `recover_events` returns an empty log on a failed magic read; a WAL error is hidden behind jsonl. Not M003 Cypher; not M005 HNSW; not ADR Accepted.

**Success:** a crash at any persist step reopens to the last durable log prefix or fails closed with a named error; persist cost is proportional to the new events, not to N; the quantum outcome has one durable encoding; the log is segmented with a manifest and a hash chain; RocksDB, if used, holds only droppable indexes and never owns an event, a commit or a disposition. The event log stays SoT (D1). Each slice keeps FF5, M010–M017 oracles unchanged and re-runs the bench gate when it edits `crates/`.

- [x] **S01: Thaw map and freeze governor honesty** `risk:low` `depends:[]`
  > After this: the blanket "freeze until M002 lease" is replaced by a per-item thaw map (RocksDB crate: S03; Cypher: M003; HNSW: M005; six dictionaries: SIX-01; ADR-080/081, ADR-090, ADR-093, Consensus 103+: their own named gate); the `freeze` governor check no longer claims to read STATE (split per dependency, each message names its real thaw gate) and its selftest and pytest prove it; no file under `crates/` changes.
- [x] **S02: SNAP-01 and fail-closed WAL open** `risk:medium` `depends:[S01]`
  > After this: sidecars replace via tmp+sync+rename and every rename is followed by a parent-directory fsync; an unreadable or partial sidecar is treated as absent while an intact log still opens; `check_tt_anchors` tolerates a log ahead of the snapshot; a failed WAL magic read or an empty WAL without jsonl never yields an empty log; a WAL error is not hidden behind jsonl; a crash-matrix test (failpoint after each persist step) reopens to the last durable prefix or fails closed.
- [x] **S03: RocksDB build spike** `risk:low` `depends:[S01]`
  > After this: a thin isolated module compiles `rocksdb` offline on this host and round-trips `EventId` to file offset; it is not wired into `Runtime`; the governor allows the dependency only in the leased slice; build prerequisites are recorded; if it cannot build, the failure and the fallback (pure-Rust index) are written down.
- [x] **S04: Incremental framed append** `risk:high` `depends:[S02]`
  > After this: a second `persist` appends length+CRC framed records and does not rewrite the prefix bytes; one `fdatasync` per quantum; a crash mid-append drops only the incomplete tail; replay parity with the jsonl path; bench gate (`ingest_plain.n`, `as_of.n`) within 10 percent.
- [ ] **S05: Commit marker decision (COMMIT-01)** `risk:high` `depends:[S04]` `gate:controller+user decision`
  > After this: one durable encoding of the quantum outcome is chosen (commit frame in the segment, or `Op::QuantumOutcome` kept) and recorded in ADR-010/014; a crash between payload events and the commit leaves no uncommitted prefix visible; named oracles unchanged.
- [ ] **S06: Segments, manifest and hash chain** `risk:high` `depends:[S04,S05]`
  > After this: sealed segments with an atomically replaced manifest; recovery from manifest plus the active segment; a flipped byte or swapped records in a sealed segment fails open with `InvalidData`; the jsonl export is no longer the SoT.
- [ ] **S07: Droppable RocksDB index lease** `risk:medium` `depends:[S03,S06]`
  > After this: `EventId` to offset (and `delivery_key`) indexes live in RocksDB as a lease rebuilt from the log; deleting the Rocks directory changes no answer; Rocks never stores an event payload, a commit frame, a disposition or a term meaning.

IO-01 (cloud kernel writeback soak) stays an intake note; vacuum and litigation hold stay outside M002.

## Later milestones (not active)

Do not start until STATE names them. M011, M012a, and M012 are CLOSED. M013 is CLOSED. M014 is CLOSED. M015 is CLOSED. M016 is CLOSED. M017 is CLOSED. M018 is paused after S01 at the S02 decision gate. M002 is thawed and planned (S01–S07); one lease at a time. M003+ stay unleased.

Proposed order (long horizon, non-authoritative; sequence not calendar; re-derive each step with `/gsd-new-milestone` when leased). Ids are given only to the nearest steps; later waypoints stay unnumbered until a lease creates them (inflation guard).

1. **M011** — CLOSED (S01–S08). Historical record above.
2. **M012a** — CLOSED (S01–S06). Detail above.
3. **M012** — CLOSED (S01–S05). Detail above.
4. **M013** — CLOSED (S01–S09, GSD v0.06 tagged). Detail above.
5. **M014** — CLOSED. Typed clocks and as-known-on. Detail above.
6. **M015** — CLOSED. Known-on honesty and M014 residuals (v0.08). Detail above.
7. **M016** — CLOSED (S01–S05, GSD v0.09 tagged). Detail above.
8. **M017** — CLOSED (S01–S04, GSD v0.10 tagged). Detail above.
9. **M018** — Wedge unblock (S01–S04, GSD v0.11). S01 done; S02 decision gate. Detail above.
10. **M002** — THAWED (S01–S07, GSD v0.12). Detail above. Log durability protocol: segmented append-only log, hash chain, atomic manifest, replay parity across stores, with the quantum outcome as a commit marker in segment framing (ADR-010/014 open question). “Lease identity verified on open” is already done (v0.04 Phase 15). Rocks only behind that protocol, for indexes — not a second SoT. Vacuum policy and litigation hold stay in this horizon, not as the milestone identity. **Intake (2026-10-02):** cloud kernel writeback / dirty-throttle stalls — `docs/architecture/m002-cloud-kernel-io-intake.md` (arXiv:2609.12597 / SteelDB companion); multi-drain placement beats single-volume bandwidth when buffered I/O is on distributed block storage.
11. **M003** — Cypher skin over the already-correct AS OF cut.
12. **M004** — Science fixture (second vertical → then pack lifecycle has a reason).
13. *(unnumbered)* Pack lifecycle and Cui allocation — only after two verticals run (rule of three).
14. **M005** — HNSW fence as retrieve-not-truth; exact AS OF must not be fakeable by ANN.
15. *(unnumbered)* Security — ABAC rewrite, grants @T, agent sandbox; required before any enterprise ship.
16. *(unnumbered)* Packaging, naming and license decision (ADR-092) before any publish.

P4 (Raft/multi-node, learned indexes) is not planned until single-node hybrid evidence exists.

## Thaw map

The per-item thaw map lives in `.kutha/STATE.md` § Thaw map. M002 is thawed; the other frozen items keep their own named gates. M013 thaw was scoped and freeze restored; do not treat it as a precedent.
