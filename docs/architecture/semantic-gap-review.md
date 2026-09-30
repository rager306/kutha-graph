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
| F1 | Quantum outcomes and justifications are durable sidecars beside the log, not log records; `provenance_fingerprint` does not mix those sidecar bytes. | `store.rs` `OUTCOMES_REL` / `JUSTIFICATIONS_REL`; `persist` calls `write_outcomes` and `write_justifications` after `File::create` of `events.jsonl`. `quantum.rs` `Justification` documents SoT as `justifications.jsonl`; `attach_justifications` / `record_justification` mutate a RAM buffer never appended as `Op`. `provenance_fingerprint` hashes only in-log `Op::Behavior` rows (`kutha-prov-v1`). | read in source | Touches D1/D2 (event log = SoT) and ADR-010/014/060: a second durable file is an authoritative cousin of the log, so discarding the sidecar is not the same as discarding a lease. |
| F2 | Retract, interval patch, and justification cites address fold-local `fact_seq`, not a stable event or proposition id. | `event.rs` `Op::Retract { fact_seq: u64 }` (same field on `Correct` / `CorrectInterval`). `fold.rs` `Fact.seq` assigned from `GraphFold.next_seq` in `apply`. `quantum.rs` `Justification.source_fact_seqs: Vec<u64>`; `check_admission` matches those seqs against `fold.facts()`. `EventId` exists and is used for `claim_id` / `caused_by`, but Retract does not take it. | read in source | Touches ADR-011 identity and ADR-013 invalidation: a fork or a rebuilt fold that numbers facts differently cannot name the same retract target. |
| F3 | `claim_id` is a support *slot*; conflict polarity is caller-supplied; `emit` is not idempotent. | `fold.rs` `Fact.claim_id` (multiple Facts may share one id). `quantum.rs` `conflict_report_at(claim, tt, vt, positive: TermId, negative: TermId)` partitions by those interned objects — no stored opposite-of relation. `emit` always `Event::new(op, next_tt())` (`Uuid::now_v7`); there is no duplicate-op gate (crate search: no `idempot`). Re-emitting an identical Assert appends another fact. | read in source | Touches ADR-013 conflict view and ADR-011: a claim is not a proposition identity, so ingest can mint extra supports without detecting a repeated delivery. |
| F4 | Derivation is single-parent, one-hop, with a hard-coded inverse and a free-string rule pin. | `event.rs` `Op::Behavior { caused_by: EventId, rule_version: String, .. }` — one parent. `quantum.rs` `derivation_eligible_at`: derived claim live **and** the `caused_by` event’s claim live (one hop; no walk). `follow_ons` emits `name: "inverse_knows"` when relation equals interned `knows`. `rule_version` is an unconstrained string (`#[serde(default)]` → `""`). | read in source; **not verified by a failing test** that a two-hop chain must be ineligible | Touches ADR-011 D011-3 and ADR-060: n-ary / transitive eligibility and a hashed rule registry are not this spike; claiming “no transitivity” is source reading, not an oracle that fails today. |
| F5 | Admission is a callable fail-closed helper with no non-test caller; the relation allowlist is ambient file/env state; there is no action entity. | `quantum.rs` `check_admission` (deny reasons `unknown_justification` / `stale_support` / `ineligible` / `rule_version`). Crate callers: `tests/m011_e2e_fixture.rs` only. `allow.rs` `ENV_PATH` (`KUTHA_RELATIONS_PATH`) and `relations_path()` → YAML file; `Runtime::admit` uses the in-memory `allowed` set loaded from that file, not a log fact. Crate search: no `Action` type. | read in source; **not verified by a failing test** for “no production caller” and “no action entity” | Touches ADR-050 admission-as-facts and ADR-051/081: policy version is not pinned in the log; “action” is not a product type. |
| F6 | `persist` overwrites files in place; `open` trusts a snapshot without `replay_check`; each persist mints new `Define` event ids. | `store.rs` `persist`: `File::create` on `events.jsonl` and `snapshot.json` (truncating create, not rename-into-place). `open`: if `snapshot.json` exists, `Runtime::from_snapshot` then `attach_*`; **no** `replay_check` on that path (`replay_check` lives on `Runtime` in `quantum.rs`). `encoded_log` maps dictionary strings through `Event::new(Op::Define { name }, 0)` — `Event::new` assigns `Uuid::now_v7()`. **Measured (2026-09-30, `/tmp`, N≈121k log events / ≈40k asserts, one N):** tampered snapshot accepted by `open` (live 40199 vs truth 40200), caught only by `replay_check`; `Define` ids differed across persists; `persist` 19.5 s and `open` 8.2 s at that size (no growth-shape claim). | read in source + measured | Touches ADR-012 and D010-4: snapshot identity is not (log offset, fingerprint) verified on open; the log file is not an atomic append-only segment. |
| F7 | Valid-time and transaction-time are bare `u64` aliases; there is no declared scale and no TT↔wall map. | `event.rs` `type ValidTime = u64` (“opaque integer clock”); `type TransactionTime = u64` (“log sequence”); `Event.ingested_at: TransactionTime`. Crate search: no `TimeScale` type. Fixture years such as `2017` in `m011_*.rs` are integers, not a calendar type. | read in source | Touches ADR-000 **D4** and ADR-013 D013-1 as an **open question** only. This review does not rewrite D4 wording. |
| F8 | Hot reads scan the fact vector; CSR is built beside the `Materializer` trait, not through it; STCA Space/Composition and the ADR-061 diff half are unexercised. | `fold.rs` `as_of` / `claim_supported_at` iterate `facts()`. **Measured (same bench):** `claim_supported_at` 15→32→65 µs and `as_of` 28→68→144 µs per query for N×1/2/4 (linear in N). `quantum.rs` `csr_lease_at` → `CsrLease::from_fold` (`csr.rs`); `Materializer` / `CsrMaterializer` live in `materializer.rs` and are used from that file’s unit test, not from `csr_lease_at`. `fork_at` exists; crate search: no `diff` API. | read in source + measured; **not verified by a failing test** that Materializer must sit on the CSR path or that a fork-diff must exist | Touches ADR-040/041 (leases vs SoT) and ADR-061 (fork is present, diff half is not). ADR-002 is the STCA paradigm note — unexercised axes are an evidence note, not a lock rewrite. |

## Cell verdicts

Verdicts were set by reading each cited ADR’s current Decision and Clarification prose (2026-09-30), not by copying the Phase 10 D-R2 hypothesis. D-R2 remains the scout map. Where ADR text already states the target contract, the verdict is **no change** (the spike may still disagree). Where the cell lacks that contract, the verdict is **amend** or **open question**. Status stays **Proposed** everywhere. 10-02 writes dated additions; this review does not edit ADR bodies.

| Cell | F | Verdict | Evidence in the ADR text |
|------|---|---------|--------------------------|
| ADR-010 | F1, F6 | open question | D010-1: “The append-only event log is the only source of temporal truth” and “authoritative inputs needed to reconstruct meaning and quantum outcomes must survive dropping these leases.” Record *kinds* (outcomes / justifications / resume as log events vs retained objects) are unnamed. D010-4 (WAL is the crash cousin) already holds — F6’s `File::create` gap is D010-1 append-only, not a D010-4 rewrite. |
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
| ADR-061 | F8 | open question | D061-3: “Diff is GED-class… not cosine.” P0 has `fork_at` (prefix); crate search found no diff API. Diff half unexercised — **not verified by a failing test**. |
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
