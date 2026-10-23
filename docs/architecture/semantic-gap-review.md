# Semantic gap review

Date: 2026-09-30. **Proposed review, not an ADR, and not delivery authorization.**

This note traces product gaps F1–F8 onto the existing honeycomb. It is not another ADR, not a research wave, and not a milestone lease. Normative homes remain the linked ADRs. Live lease: `.kutha/STATE.md`. Honeycomb stays **Proposed**. Nothing here starts Rocks, Cypher, HNSW, a legal pack, or six dictionaries.

Register matches `docs/architecture/semantic-contract-validation.md`: a dated architecture note, not an Accepted cell.

## Evidence classes

| Class | Meaning in this document |
|-------|--------------------------|
| **read in source** | Named file and symbol were read on 2026-09-30 in this wave. No code-graph (CBM) verification is claimed. |
| **measured** | Copied from the 2026-09-30 `/tmp` bench in the Phase 10 context (D-R1). This wave did not re-run the bench. Persist/open were taken at one N only. |
| **not verified by a failing test** | Absence or unexercised-axis claims come from reading source (and a crate-wide symbol search). No failing cargo test asserts the gap. |

Source anchors: `crates/kutha-common/src/event.rs`; `crates/kutha-runtime/src/{fold,quantum,store,wal,allow,tenant,materializer,csr}.rs`; `crates/kutha-runtime/tests/m011_*.rs`.

## Findings F1–F8

| ID | Finding | Evidence | Class | Why it matters |
|----|---------|----------|-------|----------------|
| F1 | **Finding as of 2026-09-30 (historical):** Quantum outcomes and justifications are durable sidecars beside the log, not log records; `provenance_fingerprint` does not mix those sidecar bytes. **Status 2026-10-03:** log-native SoT is `Op::JustificationCite` / `Op::QuantumOutcome` (`store.rs` / `quantum.rs`); `events.jsonl` persists via `replace_via_tmp` (`store.rs`); fingerprint mix is `kutha-prov-log-native` (`provenance.rs`, also `Op::QuantumOutcome` / `Op::JustificationCite` / `Op::StoreBlob` digest bytes). **Status 2026-10-03 Phase 50:** SNAP-01 closed — those sidecars use `replace_via_tmp` + parent-dir fsync (`m002_snap01.rs`). **Status 2026-10-03 Phase 52:** incremental append shipped (`Persister`). | **As of 2026-09-30:** `store.rs` `OUTCOMES_REL` / `JUSTIFICATIONS_REL`; `persist` calls `write_outcomes` and `write_justifications` after `File::create` of `events.jsonl`. `quantum.rs` `Justification` documents SoT as `justifications.jsonl`; `attach_justifications` / `record_justification` mutate a RAM buffer never appended as `Op`. `provenance_fingerprint` hashes only in-log `Op::Behavior` rows (`kutha-prov-v1`). | read in source | Touches D1/D2 (event log = SoT) and ADR-010/014/060: a second durable file is an authoritative cousin of the log, so discarding the sidecar is not the same as discarding a lease. |
| F2 | **Finding as of 2026-09-30 (historical):** Retract/Correct addressed fold-local `fact_seq`. **Status 2026-10-08:** `Op::{Retract,Correct,CorrectInterval}` take `event_id: EventId` (`event.rs`); justifications cite `source_event_ids`; local `Fact.seq` remains fold-internal only. | Was: `Op::Retract { fact_seq }`. Now: `Op::Retract { event_id }` + M012a stable-ref oracles. | read in source (historical) | Closed by M012a; leave the row for register continuity. |
| F3 | `claim_id` is a support *slot*; conflict polarity is caller-supplied; `emit` is not idempotent. | `fold.rs` `Fact.claim_id` (multiple Facts may share one id). `quantum.rs` `conflict_report_at(claim, tt, vt, positive: TermId, negative: TermId)` partitions by those interned objects — no stored opposite-of relation. `emit` always `Event::new(op, next_tt())` (`Uuid::now_v7`); there is no duplicate-op gate (crate search: no `idempot`). Re-emitting an identical Assert appends another fact. | read in source | Touches ADR-013 conflict view and ADR-011: a claim is not a proposition identity, so ingest can mint extra supports without detecting a repeated delivery. |
| F4 | Derivation is single-parent, one-hop, with a hard-coded inverse and a free-string rule pin. | `event.rs` `Op::Behavior { caused_by: EventId, rule_version: String, .. }` — one parent. `quantum.rs` `derivation_eligible_at`: derived claim live **and** the `caused_by` event’s claim live (one hop; no walk). `follow_ons` emits `name: "inverse_knows"` when relation equals interned `knows`. `rule_version` is an unconstrained string (`#[serde(default)]` → `""`). | read in source; **not verified by a failing test** that a two-hop chain must be ineligible | Touches ADR-011 D011-3 and ADR-060: n-ary / transitive eligibility and a hashed rule registry are not this spike; claiming “no transitivity” is source reading, not an oracle that fails today. |
| F5 | Admission is a callable fail-closed helper with no non-test caller; the relation allowlist is ambient file/env state; there is no action entity. | `quantum.rs` `check_admission` (deny reasons `unknown_justification` / `stale_support` / `ineligible` / `rule_version`). Crate callers: `tests/m011_e2e_fixture.rs` only. `allow.rs` `ENV_PATH` (`KUTHA_RELATIONS_PATH`) and `relations_path()` → YAML file; `Runtime::admit` uses the in-memory `allowed` set loaded from that file, not a log fact. Crate search: no `Action` type. | read in source; **not verified by a failing test** for “no production caller” and “no action entity” | Touches ADR-050 admission-as-facts and ADR-051/081: policy version is not pinned in the log; “action” is not a product type. |
| F6 | **Finding as of 2026-09-30 (historical):** `persist` overwrites files in place; `open` trusts a snapshot without `replay_check`; each persist mints new `Define` event ids. **Status 2026-10-03:** `events.jsonl` / `snapshot.json` / `events.wal` replace via tmp+sync+rename (`store.rs` `replace_via_tmp` / `replace_events_jsonl` / `replace_snapshot` / `replace_events_wal`). Snapshot `open` path calls `replay_check()` (`store.rs` ~208; body in `provenance.rs`). **Status 2026-10-03 Phase 50:** SNAP-01 closed — sidecars tmp+sync+rename and parent-dir fsync; fail-closed WAL open (`m002_snap01.rs`). **Status 2026-10-03 Phase 52:** O(N) persist rewrite is closed for the `Persister` append path (`persist_append.rs`, `m002_append01.rs`); full `persist` still rewrites. | **As of 2026-09-30:** `store.rs` `persist`: `File::create` on `events.jsonl` and `snapshot.json` (truncating create, not rename-into-place). `open`: if `snapshot.json` exists, `Runtime::from_snapshot` then `attach_*`; **no** `replay_check` on that path (`replay_check` lives on `Runtime` in `quantum.rs`). `encoded_log` maps dictionary strings through `Event::new(Op::Define { name }, 0)` — `Event::new` assigns `Uuid::now_v7()`. **Measured (2026-09-30, `/tmp`, N≈121k log events / ≈40k asserts, one N):** tampered snapshot accepted by `open` (live 40199 vs truth 40200), caught only by `replay_check`; `Define` ids differed across persists; `persist` 19.5 s and `open` 8.2 s at that size (no growth-shape claim). | read in source + measured | Touches ADR-012 and D010-4: snapshot identity is not (log offset, fingerprint) verified on open; the log file is not an atomic append-only segment. |
| F7 | **Finding as of 2026-09-30 (historical):** VT/TT were bare `u64` aliases. **Status 2026-10-08:** newtypes + `TtAnchor` / `as_known_on` shipped (M014/M015). Remainder: `Op` storage still `u64`; TR-02 held by pins, not types (G-B02). | Was aliases; now newtypes + anchors. | read in source (historical) | Pack stamps / typed public API remain open (M025 candidate). |
| F8 | **Finding as of 2026-09-30 (historical):** CSR beside Materializer; no fork-diff. **Status 2026-10-08:** `csr_lease_at` builds via `CsrMaterializer`; `typed_csr_lease_at` still `from_fold`. `fork_at` exists; no public `diff`. Hot indexes exist (M012a/M013). **Status 2026-10-23:** public fact-set `diff_folds` / `GraphFold::diff` exists (`FoldDiff`); GED and named refs remain open. | Was: `CsrLease::from_fold` only. | read in source (historical) | Fact-set diff shipped; GED/named refs (ADR-061) still open. |

## Cell verdicts

Verdicts were set by reading each cited ADR’s current Decision and Clarification prose (2026-09-30), not by copying the Phase 10 D-R2 hypothesis. D-R2 remains the scout map. Where ADR text already states the target contract, the verdict is **no change** (the spike may still disagree). Where the cell lacks that contract, the verdict is **amend** or **open question**. Status stays **Proposed** everywhere. 10-02 writes dated additions; this review does not edit ADR bodies.

| Cell | F | Verdict | Evidence in the ADR text |
|------|---|---------|--------------------------|
| ADR-010 | F1, F6 | open question | D010-1: “The append-only event log is the only source of temporal truth” and “authoritative inputs needed to reconstruct meaning and quantum outcomes must survive dropping these leases.” Record *kinds* (outcomes / justifications / resume as log events vs retained objects) are unnamed. D010-4 (WAL is the crash cousin) already holds — F6’s 2026-09-30 `File::create` gap (historical; 2026-10-03 remaining SNAP-01 sidecars) is D010-1 append-only, not a D010-4 rewrite. |
| ADR-011 | F2, F3 | no change | Clarification (2026-09-13): “A local `fact_seq` is not a portable claim identity (ADR-061)”; “distinguish event ID, claim/version ID, support ID, and delivery/idempotency key”; “A retry of one delivery is not a second independent support.” The spike still uses `fact_seq` and non-idempotent `emit`; that is implementation vs this cell, not missing prose. |
| ADR-011 | F4 | open question | D011-3: how-provenance is a semiring polynomial; “the current P0 spike has neither a MATCH compiler nor this provenance evaluator.” Thin one-hop `derivation_eligible_at` is documented as not polynomials. N-ary derivation, transitive eligibility, and `rule_version` = definition hash are not decided here. |
| ADR-012 | F6 | amend | D012-1: “Snapshots are leases of the log… They are not SoT.” The cell does not require snapshot identity = (log offset, fingerprint) **verified on open**, nor an atomic append-only persist of the log file. |
| ADR-013 | F3 | amend | D013-3 and Clarification (2026-09-13) already keep independent supports and propose a `(positive_supports, negative_supports)` view. They do not define a proposition identity distinct from a support slot, nor a generic conflict operator that does not take caller-supplied polarity. |
| ADR-013 | F2, F7 | open question | Retract/correct targeting is “typed invalidation” (D013-2) without a stable id. D013-1 names VT×TT fields; it does not declare a time scale or TT↔wall map. |
| ADR-014 | F1 | amend | Context rejects a “syslog sidecar” as proof. Clarification (2026-09-13): completion evidence must be authoritative; “exact record encoding remains a future implementation choice.” That encoding choice is what F1 settles as log-native records; sidecars are leases. |
| ADR-040 | F8 | open question | Clarification (2026-09-13) already: “Current `CsrLease::from_fold` is an untyped neighbor set… `CsrMaterializer::build` records a supplied offset without verifying cut consistency. These are declared spike limits.” Whether the hot path must call `Materializer` (vs `from_fold` beside the trait) is not a new Decision. |
| ADR-041 | F8 | open question | D041-1: the hot hop picture is CSR over interned ids. The cell does not say `as_of` / `claim_supported_at` must use that picture instead of scanning `facts()`. |
| ADR-050 | F5 | amend | D050-2 is the fail-closed propose→validate→log cycle. It does not state that admission *status* is a bi-temporal fact or that the policy version is pinned in the log. |
| ADR-050 | F4 | open question | D050-1 Relation kind exists. A rule registry whose `rule_version` equals the definition hash is not a Decision. |
| ADR-051 | F5 | open question | Clarification (2026-09-13): “The action record binds the resolved arguments to their provenance… the admission/authorization decision, and the policy version.” That names an action record without a log schema. D051-3: P0 is an operator allow-list; honeycomb is grant events. |
| ADR-060 | F1 | amend | Clarification (2026-09-13) obligation 2: provenance/integrity must “validate event references, dependency links, input digests, and **quantum outcomes**.” The mix is not restricted to log records; P0 `provenance_fingerprint` omits sidecar bytes. Target: those outcomes are log records, not an extra SoT file. |
| ADR-060 | F4, F6 | no change | The cell does **not** state one-hop eligibility (D-R2’s amend-if). D060-2 (WAL recovery is a cousin) already matches F6’s durability cousin; verify-on-open belongs in ADR-012. |
| ADR-061 | F2 | no change | Clarification (2026-09-13): “Portable references must bind stable event/claim identity and branch ancestry, not a bare local `fact_seq`.” |
| ADR-061 | F8 | open question | D061-3: “Diff is GED-class… not cosine.” **Status 2026-10-23:** public fact-set `diff_folds` exists; named refs and GED remain open. |
| ADR-081 | F5 | no change | D081-1…3 isolate untrusted UDF code (WASM/SFI). The missing action *entity* is ADR-050/051’s noun, not this cell. |
| ADR-002 | F8 | no change | Time axis: “Event log = sole SoT.” Space and Composition are paradigm axes, not a delivery checklist. Unexercised Space/Composition is an evidence note in this review, not a Decision rewrite. |
| ADR-001 | F1 | no change | Vision one-liner already: append-only event log is SoT; views are reversible materializations. No honeycomb amendment. |
| ADR-000 | F7 | open question | **D4** names `valid_from` / `valid_to` / `ingested_at` / `invalidated_at` and native AS OF. It does not declare a calendar/epoch scale or TT↔wall mapping. Pointer only; D4 wording is not rewritten. |

### Hypothesis changes vs D-R2

| D-R2 guess | After reading the ADR | Why |
|------------|----------------------|-----|
| Amend ADR-011 for F2 (stable refs) | **no change** | 2026-09-13 already: `fact_seq` is not portable; identities and delivery keys are named. |
| Open question ADR-061 for F2 (Retract target / fork refs) | **no change** on that sentence | Same 2026-09-13 portable-reference paragraph. F8 still leaves D061-3 as **open question**. |
| Amend ADR-060 if it states one-hop (F4) | **no change** for F4 | Obligation 2/3 do not define one-hop eligibility. F1 still **amend**s fingerprint/outcome scope. |
| Open question ADR-081 for the action entity (F5) | **no change** | Action is D050-1 / ADR-051’s “action record”; D081 is sandbox isolation. |

## Lock assessment (D1–D10)

None of D1–D10 is rewritten. Honeycomb stays Proposed. This is a question list, not a lock edit.

| Lock | Holds? | Note |
|------|--------|------|
| D1 log = SoT; graph = fold | Holds | F1 is a spike sidecar vs this lock, not a reason to reopen D1. |
| D2 reactive quantum | Holds | F4’s inverse_knows cascade is a thin P0 behavior, not a second control plane. |
| D3 dict-first agents | Holds | F5 is missing admission-as-facts in the *honeycomb* (ADR-050), not a D3 rewrite. |
| D4 bi-temporal facts | Holds, with a **question** | Four stamps stay. **Question (not a wording change):** which integer scale is VT, and how TT maps to wall time. |
| D5 materializations as plugins | Holds | F8 is an unexercised plugin path (`Materializer` beside `from_fold`), not a revival of state-first SoT. |
| D6 Rust core | Holds | Untouched. |
| D7 load profiles | Holds | Untouched. |
| D8 antipatterns | Holds | F8’s linear scans are the D8 replay-tax shape at read time; snapshots (ADR-012) remain the named counter. |
| D9 replay / fork / diff | Holds | Replay and prefix fork exist; the **diff** half is unexercised (ADR-061 D061-3). |
| D10 positioning | Holds | Untouched. |

ADR-000 Open Research Questions may gain a one-line pointer to this review under time-scale (R3), written in 10-02. This plan does not edit ADR-000.

## Phase 8 D-F2 sidecar (superseded in intent)

Phase 8 **D-F2** chose justifications as a **durable sidecar** beside the log (`justifications.jsonl` / `JUSTIFICATIONS_REL`), mirroring Phase 5 **D-O1** quantum outcomes (`quantum_outcomes.jsonl` / `OUTCOMES_REL`): write on `store::persist`, load on `store::open`, not a droppable lease. That was a P0 shortcut so FIX-02 could fail-closed without minting new `Op` kinds.

**F1 supersedes that intent.** Authoritative records are log records; sidecars are leases. Outcomes, justifications, and resume belong on the event log (same vocabulary as ADR-010/014). The spike files remain until a lease names that encoding work; this review does not implement it and does not treat D-F2 as the target contract.

## Measured baselines (2026-09-30)

Copied from the Phase 10 context (D-R1). This wave did **not** re-run the bench. Class: **measured**.

- Host: `/tmp` bench. Log size **N≈121k** events for **≈40k** asserts.
- `claim_supported_at`: **15→32→65 µs** per query for N×1 / N×2 / N×4.
- `as_of`: **28→68→144 µs** per query for N×1 / N×2 / N×4.
- Ingest ≈ **O(1)** per emit.
- Late `Retract` ≈ **134 µs**.
- `persist` **19.5 s** and `open` **8.2 s** at that size. Persist and open were measured at **one N** only — **no growth-shape claim**.
- Tampered snapshot: `open` accepted live **40199** vs truth **40200**; caught only by `replay_check`.
- `Define` ids differed across persists.

Linear growth of the two read probes with N is a read-scan observation at three sizes. Persist/open must not be extrapolated from the single N. “No non-transitive eligibility” and “no action entity” remain **not verified by a failing test** (source reading; see F4 and F5).

## Addendum 2026-10-01: post-M012 review (F9–F16)

**Proposed review, not an ADR, and not delivery authorization.** F1–F8 above are closed or re-scoped by M011/M012a/M012 (v0.04–v0.05). This addendum records what the closure exposed. Live lease: `.kutha/STATE.md`. Honeycomb stays **Proposed**. Nothing here thaws Rocks, Cypher, HNSW, a legal pack, or six dictionaries.

Evidence classes as above, plus **graph-checked** (CBM project `root-kutha-graph` and GitNexus `kutha-graph`, reindexed 2026-10-01). Tool limits matter: GitNexus `impact` on `Runtime::emit` resolved 15 callers and none from `crates/*/tests/`; on enum `Op` it resolved 0 usage edges. CBM fan-in for `emit` is 76. Variant-level questions (`Op::RecordAction`) are answered by `search_code` / `rg`, not by either impact tool. Every finding below is still **not verified by a failing test**; each M013–M015 phase opens with a RED oracle for the finding it closes.

### Findings F9–F16

| ID | Finding | Evidence | Class | Why it matters |
|----|---------|----------|-------|----------------|
| F9 | “Dictionaries as facts” stores identity, not semantics. Policy and rule bodies are opaque strings, and `check_admission` runs four hard-coded checks. | `event.rs` `Op::PinPolicy { definition: String }` and `Op::RegisterRule { definition: String }`. `quantum.rs` `check_admission` never reads the pinned policy definition. | read in source; graph-checked | Changing a policy changes no decision. ADR-060 execution replay (re-derive and compare) is impossible. |
| F10 | YAML remains a root of trust for relation admission. | `quantum.rs` `admit`: allowed if `fold.relation_allowed_at(..)`, else allowed if `self.allowed` (YAML/env) contains the name. | read in source | Log facts can only widen the allowlist. A seeded relation cannot be retracted, which breaks D1 for admission. |
| F11 | Three write paths and a fourth for retraction of meta entries. | `emit` (cascade, receipt, `QuantumOutcome`); early-return branch for Allow/Rule/Policy; `append_meta` for justification/admission/action without `admit` or receipt; six `has_*_entry` scans for meta retract. | read in source; graph-checked | D2 (one quantum) holds for assert/retract only. Every new dictionary kind adds an `Op` variant, a side table, and a copy of `is_live_at` (five today in `fold.rs`). |
| F12 | The product has no consumer. | Inbound `trace_path` on `record_justification` and `record_action`: zero non-test callers. The only binary is `kutha-tenant` (harness). | graph-checked | “On the product path” currently means “reachable from the public API in tests”. No fixture drives the engine through one facade. |
| F13 | Clocks are untyped, and “as known on a date” is inexpressible. | `type ValidTime = u64`, `type TransactionTime = u64`; TT is a log sequence number (`next_tt`), VT is a year (`TimeScale::YearCe`). | read in source | ADR-090 TR-02 (no silent clock substitution) is not enforced by types. The legal wedge needs at least valid, transaction, wall, and effective-date clocks kept apart. |
| F14 | String references returned after the stable-reference phase. | `justification_id` is `format!("j:{claim}:{tt}:{vt}")`; `policy_version` and `RecordAdmission.justification_id` are `String`; a RAM buffer `self.justifications` is searched linearly. | read in source | A second source of truth beside the log (the F1 shape) and a regression from stable `EventId`. |
| F15 | Ingest is no longer O(1). | `delivery_key_retry` scans the whole log per keyed Assert; `admit_claim`, Retract, Correct, and `follow_ons` call `facts().iter().any(..)`. | read in source | The 2026-09-30 “ingest ≈ O(1)” baseline is stale. M012a S06 closed hot reads only. |
| F16 | Derived facts are asserted by the caller, not derived. | `Op::Behavior` is emitted by caller code. `derivation_eligible_at` walks one `caused_by` chain with a visited set. n-ary support is not implemented. | read in source | A trust chain, not derivation. Nothing re-computes a derived fact from its premises. |

### Roadmap findings R1–R6

| ID | Finding | Evidence |
|----|---------|----------|
| R1 | Risk inversion: durability (M002) and a real-text legal fixture were ordered before the cheapest check of the riskiest assumption (two axes plus typed clocks express real norms). | `.kutha/ROADMAP.md` “Later milestones” order 5 then 6. |
| R2 | The benchmark baseline is unscheduled and has no target N or memory budget. The fold is fully RAM-resident; no benches exist in `crates/`. | Measured baselines above (one N for persist/open). |
| R3 | M002 overlaps work already done: “lease identity verified on open” closed in DUR-01. What remains is segmentation, hash chain, manifest, and framing. | v0.04 Phase 15. |
| R4 | Verification is not independent: the executor writes the tests and the governor observations; `/gsd-verify-work` never ran. | `.planning/` has no UAT artifacts. |
| R5 | Planning artifacts outweigh the product: about 40.8k lines of `.planning` markdown, 12.3k of CE artifacts, 6.0k of harness Python, 2.1k of YAML against 9.2k of product Rust including tests. | `wc -l`, 2026-10-01. |
| R6 | Three id systems (GSD `v0.0x` and phases, harness `M0xx/S0x`, ADRs) with no single map. | `docs/process/gsd-auto-contract.md` now holds the map. |

### Cell verdicts (addendum)

Verdicts are amendments or open questions only. No cell is Accepted and no lock is rewritten.

| Cell | F | Verdict | Direction |
|------|---|---------|-----------|
| ADR-010 | F11 | amend | One write path for every record kind; quantum outcome targets a commit marker in log framing (M002), not an `Op`. |
| ADR-011 | F10, F11, F14, F15 | amend | Dictionary entries are bi-temporal facts in a reserved namespace with content-addressed bodies; references are `EventId`; EventId→fact and delivery-key→event indexes are fold-internal leases. |
| ADR-013 | F13 | amend, open question | Typed clocks; TT↔wall as `tt_anchor` facts; “as known on D” is an anchor lookup. Sufficiency of two axes for five legal clocks is decided at the golden fixture, not here. |
| ADR-014 | F11 | open question | Commit marker versus receipt versus cascade budget. |
| ADR-040 | F15 | amend | Indexes are leases; the source of truth stays the log. |
| ADR-041 | F9, F16 | open question | Leapfrog as the evaluator of rule bodies over the CSR lease. |
| ADR-050 | F9, F10 | amend, open question | Genesis events replace the YAML seed; policy and rule bodies are executable and hashed. Six dictionaries stay frozen. |
| ADR-051 | F9 | amend | `check_admission` evaluates the pinned policy definition. |
| ADR-060 | F9, F16 | amend | Execution replay is re-derivation by registered rules, compared with the log. |
| ADR-090 | F13 | amend | TR-02 is enforced by types; real-text fixtures come from the sister product (law-nexus). |

### Lock assessment (addendum)

| Lock | Holds? | Note |
|------|--------|------|
| D1 log = SoT | Holds, spike deviates | F10 (YAML root) and F14 (RAM buffer) are spike deviations, not a reason to reopen D1. |
| D2 reactive quantum | Holds, spike deviates | F11: four write paths. |
| D3 dict-first agents | Holds | F9: dictionaries carry identity but no semantics yet. |
| D4 bi-temporal | Holds, with a question | F13: which clocks are typed, and whether two axes plus anchors suffice. |

### Proposed remedies (not Accepted)

Each remedy is falsifiable by a named oracle in `.kutha/ROADMAP.md` M013–M015.

1. **Unified fact substrate** (F10, F11, F14): dictionary entries become ordinary bi-temporal facts in a reserved namespace; rule and policy bodies are content-addressed blobs referenced by hash; the YAML seed becomes genesis events at the head of a new log.
2. **Executable rules** (F9, F16): a canonical rule AST hashed into `rule_version`, bodies evaluated by the existing leapfrog over the CSR lease, support counting as the existing `claim_id` multiplicity, execution replay as re-derivation.
3. **Commit marker in framing** (F11): the quantum outcome moves out of `Op` into segment framing at M002 (the log is about 3× the number of asserts today: 121k events for 40k asserts).
4. **Typed clocks** (F13): newtypes for valid time, transaction time, and stamps per clock; TT↔wall as `tt_anchor` facts.
5. **Indexes and one facade** (F12, F15): EventId→fact and delivery-key→event fold indexes; one public write path (propose, admit, commit, receipt) consumed by one example and one integration test.

Sequencing and gates: `.kutha/ROADMAP.md` (M013 → M014 → M015 → M002) and `docs/process/gsd-auto-contract.md`.

## Addendum 2026-10-02: post-M013 S01–S07 (honest close-out)

**Proposed review, not an ADR.** After M013 refactor slices S03–S07 (Phases 24–27) and Phase 28 verification. Honeycomb stays **Proposed**. Evidence: named cargo tests (`rg 'fn <name>' crates/`) and source reads on 2026-10-02.

### Closed (named oracles)

| ID | Oracle / evidence |
|----|-------------------|
| F10 | `m013_genesis_log.rs` (`retract_seeded_related_to_fails_closed_after`, genesis persist/replay tests) |
| F11 | Single `pipeline` path; `append_meta` absent from `crates/` |
| F12 | `statute_as_of_via_facade` (`facade_statute.rs`; `examples/statute_as_of.rs`) |
| F14 | Retract/Correct/justification use `EventId` (`m012a_stable_refs.rs`, `m013_write_path.rs`) |
| F15 | `green_ingest_visits_constant_in_n` (`m013_fold_indexes.rs`); lazy/eager parity: `lazy_as_of_matches_eager_after_same_ingest` |

### Still open

| ID | Note |
|----|------|
| F1 | **2026-10-03 Phase 50:** SNAP-01 sidecar atomicity closed (`replace_via_tmp` + `sync_dir`). See addendum F1/F6. Historical 2026-10-02 note: `events.jsonl` rename-atomic; `snapshot.json` truncating create. |
| F7 remainder | Five legal clocks / calendar scale beyond YearCe + LogSequence + Wall (see post-M014 addendum) |
| F8 remainder | Typed CSR via `from_fold`; public fact-set `diff_folds` shipped; GED/named refs remain open |
| F9 | Policy/rule bodies not evaluated on admit (M016) |
| F13 remainder | Sufficiency of two axes + typed annotations for transitional rules — **DECIDED 2026-10-06:** pack stamp (ADR-013 Verdict Phase 46); crate unchanged. See addendum 2026-10-06. |
| F16 | Caller-emitted `Behavior`; no execution replay re-derive (M016) |

### FF5 scope (not “as known on D”)

FF5 compares **two valid-time windows** on the same log using the fold at **transaction-time max** (`as_of(vt)` → `live_at(u64::MAX, vt)` in `fold_query.rs`). It is not “as known on calendar date D”. That query is `as_known_on` via `tt_anchor` (M014; Phase 34 made the cut TT-only).

## Addendum 2026-10-02: post-M014 + Phase 34 (known-on residual)

**Proposed review, not an ADR.** After M014 (v0.07 typed clocks / `as_known_on`) and Phase 34 (M015 S01 TT-only known-on + fail-closed anchors). Honeycomb stays **Proposed**. Evidence: named cargo tests and `event.rs` newtypes, read 2026-10-02.

### Closed (named oracles)

| ID | Oracle / evidence |
|----|-------------------|
| F7 alias half | `ValidTime` / `TransactionTime` / `WallTime` newtypes in `event.rs`; compile_fail mix |
| F13 clock + known-on | `Op::TtAnchor`; `as_known_on`; `true_on_d_differs_from_known_on_d`; Phase 34 `closed_vt_interval_survives_as_known_on`; `duplicate_tt_anchor_wall_is_rejected` |

### Still open (this addendum)

| ID | Note |
|----|------|
| F7 remainder | Promulgation / effective / application / edition are not distinct core types |
| F13 remainder | Whether two axes plus typed annotations suffice (`tempus regit actum`) — **DECIDED 2026-10-06:** pack stamp (ADR-013 Verdict Phase 46); crate unchanged. See addendum 2026-10-06. |
| F9, F16 | Executable rules — M016 |

## Addendum 2026-10-03: post-M016 (executable rules + FIXG-01)

**Proposed review, not an ADR.** After M016 (v0.09 `RuleAst` / evaluator / policy eval / execution replay) and FIXG-01. Honeycomb stays **Proposed**. Evidence: named cargo tests and source, read 2026-10-03 (CBM `root-kutha-graph` + `rg`).

**Finding registers:** F1–F16 = this review; A-F01–A-F14 = 2026-10-02 audit.

### Closed (named oracles)

| ID | Oracle / evidence |
|----|-------------------|
| F9 | Thin `deny` policy body eval in `admission.rs` `check_admission_row` (live blob `deny` → `AdmissionDenied { reason: "policy" }`); `deny_policy_fails_admission_leased_policy_admits` |
| F16 | `horn_behavior_reproducible` in `eval.rs`; called from `provenance.rs` `replay_check`; `empty_pin_inverse_fails_replay_fingerprint_unchanged`. **2026-10-03:** `live_horns` is lease-first via `GraphFold.horns_by_premise` (scan fallback `live_horns_scan` when the lease is not ready). Remaining-premise walk still uses `search_prems(..., u64::MAX)` / `live_facts_at(u64::MAX, vt)` (`eval.rs`). Ingest scaling: `extra_relatedto_horn_visits_constant_in_n` |

### Still open (this addendum)

| ID | Note |
|----|------|
| F13 remainder | Clock half closed v0.07 (`ValidTime` / `TransactionTime` / `WallTime`, `Op::TtAnchor`). Remainder = FIXG-01: C01 sunset is not expressible as Horn. **DECIDED 2026-10-06:** pack stamp (a); not fifth axis, not Horn, not `valid_to`-as-application. Crate unchanged. Legal pack out of wedge. |
| F1 / F6 remainder | **2026-10-04 Phase 53:** COMMIT-01 option A closed the crash-between-payload-and-outcome visibility gap on `open` (`committed_prefix`, `m002_commit01.rs`): an uncommitted User suffix is hidden; `Op::QuantumOutcome` stays the marker. Full `persist` still rewrites. Residual: Typed/Dictionary multi-event groups have no outcome (partial group can still be visible). B/C wait for S06. |
| F8 remainder | Fact-set `diff_folds` shipped; GED/named refs remain open (cosmetic vs D061-3) |

### Audit code findings (2026-10-02), restaged 2026-10-03

| ID | Residual (as audited) | Status 2026-10-03 |
|----|-----------------------|-------------------|
| A-F01 | Historical audit: `snapshot.json` `File::create` (not atomic replace) | Closed: `abort_snapshot_replace_keeps_previous_bytes` (`m017_replay_integrity.rs`) |
| A-F02 | Historical audit: `events.wal` truncate-in-place | Closed: `abort_wal_replace_keeps_previous_bytes` (`m017_replay_integrity.rs`) |
| A-F04 | Tampered persisted `Op::TtAnchor` not fail-closed on `replay_check` / `open` | Closed: `tampered_tt_anchor_wall_fails_replay_check` (`m017_replay_integrity.rs`) |
| A-F06 | Sidecars still written; `open` can attach outcomes when the log has none | Closed for disagreeing sidecar: `mismatched_outcomes_sidecar_fails_open`. Phase 50: in-place `File::create` closed (`sidecar_*` / `persist_leaves_no_tmp_files` in `m002_snap01.rs`). |
| A-F07 | `FoldIndexMode::from_env` on new/open; snapshot does not pin mode | Closed: `snapshot_index_mode_survives_env_flip` (`m017_replay_integrity.rs`) |
| A-F08 | WAL preferred whenever the file exists (empty/magic-only can beat intact `events.jsonl`) | Closed: `wal_header_only_falls_back_to_jsonl`, `wal_magic_only_falls_back_to_jsonl` (`m017_replay_integrity.rs`) |
| A-F10 | `as_known_on` rustdoc/oracle honesty (TT-only vs `live_at(tt, MAX)` on a closed interval) | Closed: `as_known_on_differs_from_live_at_max_on_closed_vt` (`m017_replay_integrity.rs`) |
| A-F05 | Horn remaining-premise lookup is `TT = u64::MAX`, not the knowledge cut | **DECIDED 2026-10-06.** Ingest-at-head (D-HORN-KC). Killed pin: `known04_horn_fires_when_premise_hidden_on_early_cut` (`m017_known_on_horn.rs`; ingest mint + `as_known_on` TT hide). Crate behaviour unchanged. Do not invert. |

DET-01 / KNOWN-04 characterized A-F05; Phase 46 promoted `known04` to the killed pin of ingest-at-head (crate unchanged). A-F09 / A-F11 / A-F12 of the same audit wave are not in the closed set above.

## Addendum 2026-10-06: Phase 46 / M018 S02 (C01 / HORN-KC / ASOF)

**Proposed review, not an ADR.** Crate behaviour unchanged (`eval.rs` / fold / provenance not edited). Honeycomb stays **Proposed**.

### Decided (crate unchanged)

| ID | Decision |
|----|----------|
| F13 remainder | C01 is pack stamp (a). Not fifth axis, not Horn, not `valid_to`-as-application. Oracle `fixg01_c01_sunset_inexpressible_as_horn`. Legal pack out of wedge. |
| A-F05 | Ingest-at-head (D-HORN-KC). Killed pin `known04_horn_fires_when_premise_hidden_on_early_cut` (ingest mint + `as_known_on` TT hide). Do not invert. |

`as_of` stays support-live (`asof_elig01_as_of_contains_derived_when_ineligible`). Eligibility is `derivation_eligible_at`.
