# Honeycomb capability backfill — proposal

Date: 2026-10-31
Branch: `work/main-line`, HEAD `2680cd9`
Scope: propose `capability: named` + `evidence` for cells whose implemented core is pinned by named tests.
Constraints honoured: repository read-only; no `honeycomb.yaml` edit; no `graph index`; no `cargo`; lease **Active = None**; the only file created is this report. Test claims are source claims about the test file and `fn` (no test was executed), so "fails if removed" means the assertion text pins that behaviour, not an observed green run.

Inputs read: `.kutha/dictionaries/honeycomb.yaml`; `docs/ADR/ADR-010…093`; the 2026-10-31 ADR-conformance audit and the 2026-10-31 architecture gap register (used as leads, both re-derived against source where cited).

---

## 0. Verification of the current evidence set (independent measurement)

| Measure | Value | Method |
|---|---|---|
| Cells | 32 | `honeycomb.yaml` parse |
| `capability: named` today | 2 (ADR-011, ADR-013) | yaml |
| Cells with non-empty evidence and `capability: none` | 11 (010, 012, 014, 023, 040, 041, 050, 051, 060, 061, 090) | yaml |
| Evidence names recorded | 204 over 13 cells | yaml |
| Names that resolve to a function | 204/204 | regex scan of `crates/**/*.rs` for `fn <name>` |
| Names that resolve to an **asserting `#[test]`** | 204/204 | `#[test]`/`#[tokio::test]` attribute within 16 lines above the `fn` |
| Duplicate fn names among evidence entries | 0 | same scan |
| Newly proposed evidence names (this report) | 28 | each checked: unique fn name, `#[test]`, body contains `assert`/`expect_err`, not `#[ignore]` |

Two caveats that the governor does not model, and that a reader of `capability: named` should know:

1. **Feature-gated evidence.** `crates/kutha-runtime/tests/m002_rock01.rs:1` and `crates/kutha-runtime/tests/m019_rocks_b.rs:1` are `#![cfg(feature = "rocks-index")]`. Every test in them is skipped by default `cargo test --workspace`. Three ADR-010 evidence names live there (`index_answers_equal_the_log`, `verify_full_rejects_poisoned_middle_locator`, `open_or_rebuild_rebuilds_when_sealed_list_binding_differs`). The governor's `rust_test_asserts` (`scripts/kutha_gov/kinds.py:1290` region) resolves the `fn` and does not read `cfg`, so a feature-gated name counts as evidence even when the default gate never runs it. This report keeps those three names (they are honest evidence when the feature is on) and does not let a promotion rest on them alone.
2. **Source-text evidence is weak.** `admission_does_not_scan_the_log_for_proposals` (`crates/kutha-runtime/tests/m185_enrichment_admission.rs:127`) asserts `source.contains("self.log_ids.contains")` at `:146`. It passes on a comment, fails on a rename, and proves nothing about behaviour. It is **not** used as evidence anywhere in this proposal.

---

## 1. The rule applied (stated so it can be checked)

A cell may be `capability: named` when all three hold:

- **R1 — the cell's `must` names a surface that exists in code** and is reachable from the crate's public API (`crates/kutha-runtime/src/lib.rs` re-exports or a public method), not only a private helper.
- **R2 — removing the implemented core makes named tests fail.** For each promotion the report names the test and the assertion line that would break.
- **R3 — the unimplemented part is a named residual inside the same cell**, not a silent collapse of "some of the cell ships" into "the cell ships".

`capability: named` is **not** `map: Accepted`. Per `docs/ADR/README.md:28` the field is "`none` / `named` test evidence"; `docs/process/kutha-harness.md:37` defines `L_capability` as "product proof (fitness functions) — named test green". A promotion therefore says: named tests pin this implemented core. It does not accept the cell, and it does not authorise product work.

Under R1–R3 the same logic is applied to every cell in the requested set, including the three that many readers would promote out of habit (020, 023, 043) and the frozen one (070).

---

## 2. Result table

`cur → prop` is `capability`. `ev` is the current evidence count → proposed count. Confidence is my confidence that a mutation of the implemented core turns a named test red.

| Cell | cur → prop | ev | Load-bearing evidence (file::fn) | Conf. | Residual that stays unproven |
|---|---|---|---|---|---|
| ADR-010 | none → **named** | 17 → 17 | `tests/m002_commit01.rs::payload_without_outcome_is_not_visible`; `tests/m002_append01.rs::append_keeps_prefix_bytes_of_jsonl_and_wal`; `tests/m002_snap01.rs::sidecar_truncated_mid_line_opens_from_log`; `tests/m019_dur_a.rs::two_writer_attach_fails_closed` | high | 3 of 17 names are `rocks-index`-gated; no test for `kutha-tenant` under the lock |
| ADR-012 | none → **named** | 17 → 18 | `tests/m011_e2e_fixture.rs::e2e_incremental_matches_reconstruct_after_discarding_leases`; `tests/m002_commit01.rs::snapshot_ahead_of_committed_prefix_fails_closed`; `tests/m002_rock01.rs::dropping_rocks_dir_changes_no_answer` | medium | **aging pipeline (D012-2) and vacuum/legal hold (D012-4) have no test at all** |
| ADR-014 | none → **named** | 16 → 17 | `tests/m011_quantum_outcome.rs::budgets_0_1_2_distinguish_zero_partial_full_after_persist_open`; `tests/m011_quantum_outcome.rs::crash_after_prefix_has_no_terminal_success_until_explicit_resume`; `tests/m024_write_path.rs::receipt_digest_len_constant_across_cascade_widths` | high | "constant-size" holds for `digest`, not for the persisted `QuantumOutcome.event_ids` |
| ADR-021 | none → **named** | 0 → 8 | `tests/m179_gate_gc.rs::toy_pack_installs_queries_retires_and_replays_without_core_edits`; `tests/m176_pack_lifecycle_runtime.rs::lifecycle_state_is_replay_derived_and_graph_neutral`; `tests/m177_pack_manifest.rs::retired_pack_releases_operator_names` | high | no operator *execution* test (a registered operator is a name, not a runnable extension) |
| ADR-040 | none → **named** | 8 → 9 | `src/materializer.rs::csr_materializer_build_unload`; `tests/m012a_hot_indexes.rs::csr_lease_at_builds_via_materializer`; `tests/m012a_hot_indexes.rs::discard_csr_lease_and_snapshot_rebuild_keeps_as_of` | medium | `apply(delta)` / `snapshot` / `rollback` absent; `build` does not verify the supplied cut |
| ADR-041 | none → **named** | 9 → 12 | `tests/m012a_hot_indexes.rs::csr_lease_at_builds_via_materializer`; `src/leapfrog.rs::leapfrog_three_lists`; `src/leapfrog.rs::leapfrog_common_neighbors_on_csr` | medium | leapfrog is not called by query execution; D041-2 "MATCH uses LFTJ" and D041-3 late materialization absent |
| ADR-052 | none → **named** | 0 → 6 | `tests/m178_enrichment.rs::proposal_is_typed_meta_and_does_not_change_facts`; `tests/m178_enrichment.rs::retired_or_inactive_pack_fails_closed`; `tests/m183_digest_falsifiers.rs::policy_hash_addressed_enrichment_payload_is_rejected` | high | D052-1 "explicit admission decision" is replaced by structural rejection; D052-4 coherence gate absent |
| ADR-060 | none → **named** | 12 → 12 | `tests/m019_sem_meta.rs::replay_tamper_flip_swap_drop_never_silently_equal`; `tests/m019_sem_meta.rs::replay_idempotence_fresh_runtime_twice`; `tests/m035_replay_dict_bind.rs::replay_check_returns_bound_fold` | high | execution replay of non-Horn behaviours (ADR-060:100 says it does not rerun them) |
| ADR-061 | none → **named** | 4 → 8 | `tests/b6_fork_query.rs::runtime_diff_explains_scan_on_fork`; `src/quantum_invariants.rs::fork_at_isolates_prefix`; `tests/b4_fork_diff.rs::diff_fold_vs_fork_at_matches_post_cut_mints` | medium | named branches (`main` only, ADR-061:48); GED-class diff open (shipped diff is a fact-set edit script) |
| ADR-070 | none → **named** | 0 → 5 | `tests/m003a_gpml.rs::parses_one_hop_to_query_ir_and_executes_as_of`; `tests/m003a_gpml_differential.rs::ff5_gpml_as_of_matches_typed_facade`; `tests/m003a_gpml_differential.rs::typed_tt_and_limit_are_preserved` | medium | no Cypher/GQL skin, no named-graph scope (D070-4), no RPQ/journey, not full LFTJ MATCH; ADR-070:44 is stale |

Totals if applied unchanged: `capability: named` 2 → 12; `none` 30 → 20; `map` and `delivery` untouched (map stays Proposed for all 32; delivery stays spike 13 / map-only 12 / frozen 7).

---

## 3. Per-cell reasoning (what breaks, and at which line)

### ADR-010 — log is SoT; emit→idle quantum (promote, high)

- `payload_without_outcome_is_not_visible` (`tests/m002_commit01.rs:110`) commits a prefix, emits a product assert, crashes before the outcome (`:119` `crash_omit_last`), reopens, and asserts the fold fingerprint is the committed one (`:122-126`) and the log ids are unchanged (`:128`). The gate is `Op::QuantumOutcome` (`crates/kutha-common/src/event.rs:220`) applied by `commit::committed_prefix` (`crates/kutha-runtime/src/store.rs:1095`); remove it and the uncommitted assert folds, so this test fails.
- `append_keeps_prefix_bytes_of_jsonl_and_wal` (`tests/m002_append01.rs:97`) asserts the pre-append prefix bytes of `events.jsonl` and `events.wal` are unchanged (`:116-125`) — append-only log, no silent rewrite.
- `sidecar_truncated_mid_line_opens_from_log` (`tests/m002_snap01.rs:115`) tears `quantum_outcomes.jsonl` (`store::OUTCOMES_REL`, `store.rs:18`) mid-line and still opens; ids equal the log's (`:136-138`). Derived sidecar loses, log wins.
- `two_writer_attach_fails_closed` (`tests/m019_dur_a.rs:387`) — one writer per store (`kutha.lock`, `store.rs:37`, `try_lock` at `store.rs:73`).
- Residual: see §0.1 (three feature-gated names). Promotion rests on 14 default-run tests.

### ADR-012 — snapshots and tiers are leases; vacuum is policy (promote, medium)

- `e2e_incremental_matches_reconstruct_after_discarding_leases` (`tests/m011_e2e_fixture.rs:395`) persists, **deletes `snapshot.json`** (`:445`), reopens, and asserts fingerprint and justification records equal the live runtime (`:446-453`). Delete the lease and this fails; keep the snapshot as SoT and this cannot pass.
- `snapshot_ahead_of_committed_prefix_fails_closed` (`tests/m002_commit01.rs:273`) bumps `snapshot.log_offset` (`:279`) and requires `store::open` to fail `InvalidData` (`:284-287`); the guard is `store.rs:1188`. This is the "lease identity is verified, not trusted" half, and it is **not currently listed** in the cell's evidence — proposed addition.
- `dropping_rocks_dir_changes_no_answer` (`tests/m002_rock01.rs:269`) deletes the locator tier and answers are unchanged (`:279-286`). Feature-gated; cited but not load-bearing alone.
- **Vacuum and aging: no test, and no code.** `rg -ni vacuum crates/` returns two doc comments that exclude the feature (`crates/kutha-runtime/src/security.rs:6`, `crates/kutha-runtime/tests/m175_security_policy.rs:3`). A checkpoint is not vacuum: `checkpoint_sealed_prefix` (`store.rs:177`) keeps dead facts in the fingerprint (ADR-012:69: "Fingerprint still hashes all facts **including dead ones** (not vacuum, D012-4)"). D012-2's aging pipeline is also absent: `CsrLease::from_fold` (`src/csr.rs:13`, typed at `:79`) rebuilds from the fold every time, and `CsrMaterializer::build` just calls it (`src/materializer.rs:41-53`) — there is no mutable head tier and no `rg -ni "aging|write buffer"` hit in `crates/`. Promotion therefore speaks only to D012-1 (leases) and the locator tier; it must not be read as vacuum or aging.

### ADR-014 — cascade budgets and quantum receipts (promote, high)

- `budgets_0_1_2_distinguish_zero_partial_full_after_persist_open` (`tests/m011_quantum_outcome.rs:7`): budgets 0/1/default yield Zero/Partial/Full dispositions that survive persist+open (`:13-24`). Remove the budget and Zero/Partial collapse into Full.
- `crash_after_prefix_has_no_terminal_success_until_explicit_resume` (`tests/m011_quantum_outcome.rs:28`): after deleting the outcomes sidecar, open reconstructs `Partial` (`:63-67`) and must not invent `Full` (`:68-74`) or `Resume` (`:75-81`); `record_resume` is explicit (`:83`).
- `receipt_digest_len_constant_across_cascade_widths` (`tests/m024_write_path.rs:83`): 32 bytes for a narrow and a wide cascade (`:118-123`), with the width actually differing (`:125-127`). This is the D014-1 test (ADR-014:48) and is **missing from the current evidence** — proposed addition.
- Residual: the persisted record is not constant-size — `Op::QuantumOutcome { … events_in_quantum, event_ids: Vec<EventId> … }` (`event.rs:220-228`) grows with width. ADR-014:49 already records the width leak, and ADR-014:109 says the P0 receipt is not the full contract. `digest` is `[u8; 32]` at `src/receipt.rs:13`.

### ADR-021 — pack / plugin lifecycle (promote, high)

Currently `evidence: []` although the lifecycle is implemented (`pipeline.rs:587` `pack_lifecycle`, `:605` `pack_operators`, `:682/:692` install, `:741` activate, `:756` retire).

- `lifecycle_state_is_replay_derived_and_graph_neutral` (`tests/m176_pack_lifecycle_runtime.rs:15`) — lifecycle state is a fold of the log and does not change graph facts.
- `lifecycle_state_survives_persist_open` (`tests/m176_pack_lifecycle_runtime.rs:35`) — replay parity.
- `lifecycle_ops_are_meta_and_do_not_create_graph_facts` (`tests/m176_pack_lifecycle_schema.rs:30`) — the ops are meta.
- `manifest_is_content_addressed_and_roundtrips` (`tests/m177_pack_manifest.rs:25`) and `cross_pack_operator_collision_fails_closed` (`:116`) / `retired_pack_releases_operator_names` (`:128`) — the registry laws.
- `toy_pack_installs_queries_retires_and_replays_without_core_edits` (`tests/m179_gate_gc.rs:69`): install → active (`:74`), one operator (`:75`), AS OF answers from the pack's rule (`:80-84`), retire is graph-neutral (`:94-100`), reopen reproduces lifecycle and registry (`:105-114`). `pack_retire_then_replay_is_parity_equal_to_reopen` (`:119`) is the same claim for the retired state.
- Residual: a registered operator is a declared name; nothing executes pack code (no dynamic loading), which ADR-021 does not require but a reader may assume. `delivery: map-only` is left alone per contract — note that `map-only` + `capability: named` is an unusual pair and is a deliberate owner decision, not mine.

### ADR-040 — materializer protocol; CSR is a named lease (promote, medium)

- `csr_materializer_build_unload` (`src/materializer.rs:72`, inline `#[cfg(test)]`): build → `is_mounted` (`:97`), neighbors (`:98`), unload → not mounted (`:99-100`).
- `csr_lease_at_builds_via_materializer` (`tests/m012a_hot_indexes.rs:317`): materializer build == `Runtime::csr_lease_at` == `CsrLease::from_fold` (`:332-339`), typed lease likewise (`:347-354`), and `Runtime` retains nothing (`:341-345`).
- `discard_csr_lease_and_snapshot_rebuild_keeps_as_of` (`tests/m012a_hot_indexes.rs:260`): drop both leases; `as_of` and fingerprint unchanged (`:276-279`).
- Residual: the trait is `build` / `unload` / `is_mounted` only (`src/materializer.rs:6-18`); ADR-040:45 names five verbs, so `apply(delta)`, `snapshot`, `rollback` do not exist, and `build` records the caller's `log_offset` without verifying that it describes the same cut (`materializer.rs:41-53`). Promotion proves "derived, named, droppable, rebuild-equal", not the full view contract. This is the main reason confidence is medium.

### ADR-041 — CSR / GraphBLAS hot path; leapfrog intersect (promote, medium)

- CSR on the read path: `query_ir.rs:88-91` (`HopCsr::Untyped/Typed`) is built by `prepare` (`query_ir.rs:300`) and executed by `execute` (`:345`); `Runtime::csr_lease_at` at `quantum.rs:591`, typed at `:609`.
- `typed_csr_preserves_relation_labels_and_support_multiplicity` (`tests/m011_typed_csr.rs:7`) — labels and multiplicity survive, which is the D041-1 claim beyond the untyped spike limit.
- `src/leapfrog.rs::leapfrog_three_lists` (`:73`), `leapfrog_common_neighbors_on_csr` (`:81`: builds a `CsrLease` and intersects two neighbor lists, `:105-107`), `leapfrog_empty_short_circuits` (`:111`). These are **missing from the current evidence** even though "leapfrog intersect" is in the cell's `must` — proposed addition.
- Residual and honesty flag: leapfrog has no caller outside its own tests. `rg -n leapfrog crates/` matches only `src/leapfrog.rs` and the two re-export lines (`lib.rs:15` `mod`, `lib.rs:44` `pub use`). So D041-2 "Conjunctive MATCH uses LFTJ-class WCOJ" (ADR-041:57) is **not** implemented; the kernel exists, the query path does not use it. D041-3 late materialization has no code (`rg -ni "late material" crates/` → 0). ADR-070:66 fixes the expectation: "P0 implements sorted-row `leapfrog_intersect`, not full variable-ordered MATCH."

### ADR-052 — GenAI enrichment is an optional pack (promote, high)

Currently `evidence: []` although `Op::EnrichmentProposal` (`event.rs:271`), the ledger (`pipeline.rs:610`) and emission (`pipeline.rs:618`) exist.

- `proposal_is_typed_meta_and_does_not_change_facts` (`tests/m178_enrichment.rs:50`): fingerprint and fact count unchanged (`:56-57`), ledger row present (`:58-62`) — D052-3.
- `retired_or_inactive_pack_fails_closed` (`tests/m178_enrichment.rs:66`): retired (`:70-74`) and installed-but-not-active (`:76-95`) packs are refused — D052-1 pack gate.
- `unknown_source_and_duplicate_key_fail_closed` (`tests/m178_enrichment.rs:99`): missing source (`:103-117`) and duplicate cache key (`:119-…`).
- `enrichment_missing_source_fails_replay` (`tests/m180_replay_meta.rs:230`): replay validates the source event (`:273-276`).
- `enrichment_payload_tamper_fails_on_replay` (`tests/m182_meta_threats.rs:193`): the CA cache identity is checked on replay.
- `policy_hash_addressed_enrichment_payload_is_rejected` (`tests/m183_digest_falsifiers.rs:67`): a payload cannot borrow policy-hash identity (`:72-74`).
- Residual: ADR-052's wording "applying an output requires an explicit admission decision in addition to structural validation" overstates what ships — the proposal is graph-neutral by construction (`fold.rs:1076` no-op arm) and user `emit` of it is `MetaOpRejected` (`pipeline.rs:338-349`). That is stricter than "needs admission" for kernel facts, but there is no per-output admission record. D052-4 temporal coherence is absent (`rg -ni "coherence|temporal decay" crates/kutha-runtime/src` → 0).

### ADR-060 — strict replay of the event log (promote, high)

- `replay_tamper_flip_swap_drop_never_silently_equal` (`tests/m019_sem_meta.rs:325`): dropping a fold-affecting event (`:339-347`), swapping causes (`:349`), and flipping `valid_from` (`:351-371`) each make the stamps differ; a tampered on-disk log fails closed (`:373-…`).
- `replay_idempotence_fresh_runtime_twice` (`tests/m019_sem_meta.rs:206`): two fresh replays agree and equal the live fold (`:212-241`).
- `replay_check_returns_bound_fold` (`tests/m035_replay_dict_bind.rs:68`): the rebuilt fold is query-ready (`:85-90`).
- Residual: execution replay of user behaviours is not implemented; ADR-060:100 says "it still does not rerun behaviors or compare receipts" (only Horn-pinned behaviors are re-derived). The honeycomb `must` already scopes this ("unavailable outcome proposed beyond fingerprint"), so the promotion is within the cell's own text.

### ADR-061 — fork-and-diff at a log cut (promote, medium)

- `fork_at_isolates_prefix` (`src/quantum_invariants.rs:274`): fork length equals the cut, fewer live facts, `replay_check` passes (`:305-308`); `fork_at` is `src/hydrate.rs:257`.
- `runtime_diff_explains_scan_on_fork` (`tests/b6_fork_query.rs:202`): `Runtime::diff` equals `diff_folds` (`:218-223`, code `src/fold_diff.rs:163/:170/:122`), `apply_diff` reconstructs the live facts (`:246-251`), and Scan on the fork equals the live product triples (`:257-265`).
- `diff_fold_vs_fork_at_matches_post_cut_mints` (`tests/b4_fork_diff.rs:477`) and `kill_runtime_diff_equals_diff_folds_aa_empty_and_mirror` (`tests/b6_fork_query_kill.rs:252`).
- Residual: named branches are `main`-only by decision (ADR-061:48); D061-3 GED-class diff is open — the shipped diff is a fact-set edit script, which the ADR permits as "a cheaper restriction" but has not recorded as the chosen restriction.
- Evidence honesty flag: of the four current names, `retract_by_event_id_invalidates_support` (`tests/m012a_stable_refs.rs:7`) and `rebuilt_fold_renumbered_seqs_apply_same_retract_and_cite_payloads` (`:157`) pin stable event identity, not fork or diff — they are ancestry evidence. `justification_cites_source_event_ids_survive_fork` (`:76`) does fork (`rt.fork_at` at `:131`, admission on the fork at `:144-153`). The proposal is additive; if evidence must be strictly load-bearing, drop the first and third and keep the fork one.

### ADR-070 — Cypher/GPML + temporal AS OF surface (promote, medium) — the explicit question

**Decision: `capability: named`**, with the scoping statement recorded here and an owner action recommended (a one-line ADR-070 clarification; `docs/ADR/` is read-only for this task).

Defence against the rule "a named capability must be a test that fails if the cell's core claim is removed":

1. The cell's `must` is "Cypher/GPML + temporal AS OF surface". Both halves exist as public API: `parse_gpml` (`crates/kutha-runtime/src/gpml.rs:114`) binds labels through `VocabBinder` (`gpml.rs:37`) and returns `QueryIr` (`query_ir.rs:24`), which `execute` runs (`query_ir.rs:345`). Native AS OF is `Runtime::as_of` (`quantum.rs:747`) and `GraphFold::live_at` (`quantum.rs:846`), not a Cypher skin.
2. Delete `parse_gpml` and named tests fail: `parses_one_hop_to_query_ir_and_executes_as_of` (`tests/m003a_gpml.rs:43`), `typed_cut_is_not_stringly_and_prepare_matches_handwritten_ir` (`tests/m003a_gpml.rs:157`), `b3_workload_render_parse_parity` (`tests/m003a_gpml_differential.rs:107`), `ff5_gpml_as_of_matches_typed_facade` (`:124`), `typed_tt_and_limit_are_preserved` (`:176`). If AS OF stopped binding VT/TT natively, `ff5_gpml_as_of_matches_typed_facade` and `typed_tt_and_limit_are_preserved` fail. So R2 holds for the shipped surface.
3. The absent parts are other decisions, not this `must`: one GPML syntax rather than Cypher and GQL skins (D070-1 second sentence), writes through the surface (D070-3 says "may … later"), named-graph scope (D070-4 — `rg -ni "named.?graph|\biri\b" crates/` → 0), RPQ/journeys, and full variable-ordered MATCH (ADR-070:66 assigns P0 the sorted-row `leapfrog_intersect`). None of them can be "removed" to test this cell because none exists.
4. The honest cost of `named` here: the map moves **ahead of the ADR text**. ADR-070:44 still says "No Cypher/GPML parser exists (frozen until M003 is leased)", while `.kutha/STATE.md:37` records the M003a thaw ("read-only subset parser; M003b binder swap stays gated on C2"). Recommend adding one Evidence block to ADR-070 naming `gpml.rs:114` and these five tests. **If the owner requires "map never exceeds ADR text", keep `capability: none` and add the evidence anyway** — that is the only reason I would accept the alternative, and it is a documentation ordering rule, not a capability fact. Against the stated rule (a test that fails on removal), `named` is the correct answer.

The alternative "none + note" is weaker on the rule and stronger on map-ADR agreement; I am choosing the rule, and making the ADR fix explicit rather than silent.

---

## 4. Refused promotions (and the test that is missing)

| Cell | Why refused | Missing test |
|---|---|---|
| **ADR-020** (spike / none / 0) | `must` = "Cargo workspace; hexagon stays inside a slice". The workspace half exists (`Cargo.toml:3`, two members) but the slice half does not (`crates/` holds no slice crate), and no `#[test]` can pin workspace shape: `rg -n "Cargo.toml" crates/*/tests/` → 0. A test that reads `Cargo.toml` would be source-text evidence of the m185 kind. The isolation laws are also not enforced: 69 re-exports from `lib.rs:39-70` plus `pub mod store` (`lib.rs:30`). | `crate_dependency_direction_is_one_way_and_only_ports_leave_the_crate` — a gate (`cargo metadata` / compile-fail), not a `#[test]`. Without a gate, this cell has no honest named-test evidence. |
| **ADR-023** (map-only / none / 19) | The ADR forbids it in its own decision: "Honeycomb cell 023 is **Proposed**, `delivery: map-only`, `capability: none`" (ADR-023:29). Its 19 evidence names are other cells' oracles (`det02_*`, `horn_correct_*`, `casc01_*`, `extra_relatedto_horn_*`, …) — promoting would make a test-strategy map "proven" by the tests it maps, which is circular. Its own rule is "empty until a named product test is this cell's own" (`ADR-023:29`). | None can exist by construction; the governor's `test-matrix` check is the correct evidence plane. Separately: the current 19-name list is itself inconsistent with ADR-023:29 (a finding, not a promotion). |
| **ADR-043** (map-only / none / 0) | No plan choice and no cost model. `QueryIr` is an explicit two-variant enum (`Scan` / `Hops`, `query_ir.rs:24`) chosen by the caller/parser; `prepare` (`:300`) builds the requested picture rather than choosing among fold scan / CSR / interval / HNSW (D043-1). `rg -ni "cardinalit|selectivity|estimate" crates/kutha-runtime/src/` → 0, so D043-4's "naive/AGM-pessimistic estimates" do not exist either. The existing tests (`b5_hop_workload.rs:579` prepare+run equals execute, `:634` prepared cut is a snapshot) would still pass with no planner, so they are not R2 evidence. | `planner_chooses_scan_or_csr_by_estimate_and_fails_closed_on_inexact` — would fail today. |
| **ADR-062** (map-only / none / 0) | Nothing to promote: `rg -ni regime crates/` → 0. The cell is "regimes gated loop stays optional/off"; there is no loop and no gate. | `fork_and_replay_never_reexecutes_external_effects` (the ADR's own off-by-default gate). |
| **ADR-071** (map-only / none / 0) | Absent: `rg -ni "bm25|knn|cosine|hnsw|embedding|fulltext" crates/kutha-runtime/src` → 0 matches. No approximate operator family exists, so the exact-versus-approximate fail-closed distinction (D071-1) cannot be tested. | `approximate_operator_is_named_and_never_substitutes_for_exact_match`. |
| ADR-050, 051, 090 (frozen, evidence 5/2/3, none) | **Out of scope** of this task's candidate list. Their evidence names are real tests and their `must` lines are partly shipped, but a frozen cell's capability is an owner judgement about what "frozen" means for the map; not assessed here. | n/a |

---

## 5. Exact YAML diff (proposal — not applied)

Only `capability` and `evidence` lines change. `map: Proposed`, `delivery`, `depends_on`, `locks`, `must`, `path`, `axis` are untouched. The context lines below (id, delivery, depends_on, locks) were copied from the current file rather than typed from memory. Additions are appended at the end of each existing evidence list; order is not semantic.

```diff
@@ .kutha/dictionaries/honeycomb.yaml — ADR-010 (map: Proposed, delivery: spike) @@
   - id: ADR-010
     delivery: spike
-    capability: none
+    capability: named
     depends_on: [ADR-000, ADR-001, ADR-002]
     locks: [D1, D2, D9]
     evidence:  # unchanged — 17 names

@@ .kutha/dictionaries/honeycomb.yaml — ADR-012 (map: Proposed, delivery: spike) @@
   - id: ADR-012
     delivery: spike
-    capability: none
+    capability: named
     depends_on: [ADR-000, ADR-010]
     locks: [D1, D5]
     evidence:  # existing 17 names unchanged; append:
+      - snapshot_ahead_of_committed_prefix_fails_closed

@@ .kutha/dictionaries/honeycomb.yaml — ADR-014 (map: Proposed, delivery: spike) @@
   - id: ADR-014
     delivery: spike
-    capability: none
+    capability: named
     depends_on: [ADR-000, ADR-002, ADR-010]
     locks: [D2, D3, D9]
     evidence:  # existing 16 names unchanged; append:
+      - receipt_digest_len_constant_across_cascade_widths

@@ .kutha/dictionaries/honeycomb.yaml — ADR-021 (map: Proposed, delivery: map-only) @@
   - id: ADR-021
     delivery: map-only
-    capability: none
+    capability: named
     depends_on: [ADR-000, ADR-010, ADR-020, ADR-040]
     locks: [D5]
-    evidence: []
+    evidence:
+      - lifecycle_state_is_replay_derived_and_graph_neutral
+      - lifecycle_state_survives_persist_open
+      - lifecycle_ops_are_meta_and_do_not_create_graph_facts
+      - manifest_is_content_addressed_and_roundtrips
+      - cross_pack_operator_collision_fails_closed
+      - retired_pack_releases_operator_names
+      - toy_pack_installs_queries_retires_and_replays_without_core_edits
+      - pack_retire_then_replay_is_parity_equal_to_reopen

@@ .kutha/dictionaries/honeycomb.yaml — ADR-040 (map: Proposed, delivery: spike) @@
   - id: ADR-040
     delivery: spike
-    capability: none
+    capability: named
     depends_on: [ADR-000, ADR-010, ADR-014]
     locks: [D5]
     evidence:  # existing 8 names unchanged; append:
+      - csr_materializer_build_unload

@@ .kutha/dictionaries/honeycomb.yaml — ADR-041 (map: Proposed, delivery: spike) @@
   - id: ADR-041
     delivery: spike
-    capability: none
+    capability: named
     depends_on: [ADR-000, ADR-011, ADR-040]
     locks: [D5, D6]
     evidence:  # existing 9 names unchanged; append:
+      - leapfrog_three_lists
+      - leapfrog_common_neighbors_on_csr
+      - leapfrog_empty_short_circuits

@@ .kutha/dictionaries/honeycomb.yaml — ADR-052 (map: Proposed, delivery: map-only) @@
   - id: ADR-052
     delivery: map-only
-    capability: none
+    capability: named
     depends_on: [ADR-010, ADR-050, ADR-040]
     locks: []
-    evidence: []
+    evidence:
+      - proposal_is_typed_meta_and_does_not_change_facts
+      - retired_or_inactive_pack_fails_closed
+      - unknown_source_and_duplicate_key_fail_closed
+      - enrichment_missing_source_fails_replay
+      - enrichment_payload_tamper_fails_on_replay
+      - policy_hash_addressed_enrichment_payload_is_rejected

@@ .kutha/dictionaries/honeycomb.yaml — ADR-060 (map: Proposed, delivery: spike) @@
   - id: ADR-060
     delivery: spike
-    capability: none
+    capability: named
     depends_on: [ADR-000, ADR-010, ADR-014]
     locks: [D9]
     evidence:  # unchanged — 12 names

@@ .kutha/dictionaries/honeycomb.yaml — ADR-061 (map: Proposed, delivery: spike) @@
   - id: ADR-061
     delivery: spike
-    capability: none
+    capability: named
     depends_on: [ADR-000, ADR-010, ADR-060]
     locks: [D9]
     evidence:  # existing 4 names unchanged; append:
+      - fork_at_isolates_prefix
+      - r1_fork_cut_bounds_and_no_interior_reconstruction
+      - diff_fold_vs_fork_at_matches_post_cut_mints
+      - kill_runtime_diff_equals_diff_folds_aa_empty_and_mirror

@@ .kutha/dictionaries/honeycomb.yaml — ADR-070 (map: Proposed, delivery: frozen) @@
   - id: ADR-070
     delivery: frozen
-    capability: none
+    capability: named
     depends_on: [ADR-013, ADR-041, ADR-043, ADR-050]
     locks: []
-    evidence: []
+    evidence:
+      - parses_one_hop_to_query_ir_and_executes_as_of
+      - typed_cut_is_not_stringly_and_prepare_matches_handwritten_ir
+      - b3_workload_render_parse_parity
+      - ff5_gpml_as_of_matches_typed_facade
+      - typed_tt_and_limit_are_preserved
```

Names by file for the additions (for a reviewer who wants to click through):

| Test fn | File:line |
|---|---|
| `snapshot_ahead_of_committed_prefix_fails_closed` | `crates/kutha-runtime/tests/m002_commit01.rs:273` |
| `receipt_digest_len_constant_across_cascade_widths` | `crates/kutha-runtime/tests/m024_write_path.rs:83` |
| `lifecycle_state_is_replay_derived_and_graph_neutral` | `crates/kutha-runtime/tests/m176_pack_lifecycle_runtime.rs:15` |
| `lifecycle_state_survives_persist_open` | `crates/kutha-runtime/tests/m176_pack_lifecycle_runtime.rs:35` |
| `lifecycle_ops_are_meta_and_do_not_create_graph_facts` | `crates/kutha-runtime/tests/m176_pack_lifecycle_schema.rs:30` |
| `manifest_is_content_addressed_and_roundtrips` | `crates/kutha-runtime/tests/m177_pack_manifest.rs:25` |
| `cross_pack_operator_collision_fails_closed` | `crates/kutha-runtime/tests/m177_pack_manifest.rs:116` |
| `retired_pack_releases_operator_names` | `crates/kutha-runtime/tests/m177_pack_manifest.rs:128` |
| `toy_pack_installs_queries_retires_and_replays_without_core_edits` | `crates/kutha-runtime/tests/m179_gate_gc.rs:69` |
| `pack_retire_then_replay_is_parity_equal_to_reopen` | `crates/kutha-runtime/tests/m179_gate_gc.rs:119` |
| `csr_materializer_build_unload` | `crates/kutha-runtime/src/materializer.rs:72` |
| `leapfrog_three_lists` | `crates/kutha-runtime/src/leapfrog.rs:73` |
| `leapfrog_common_neighbors_on_csr` | `crates/kutha-runtime/src/leapfrog.rs:81` |
| `leapfrog_empty_short_circuits` | `crates/kutha-runtime/src/leapfrog.rs:111` |
| `proposal_is_typed_meta_and_does_not_change_facts` | `crates/kutha-runtime/tests/m178_enrichment.rs:50` |
| `retired_or_inactive_pack_fails_closed` | `crates/kutha-runtime/tests/m178_enrichment.rs:66` |
| `unknown_source_and_duplicate_key_fail_closed` | `crates/kutha-runtime/tests/m178_enrichment.rs:99` |
| `enrichment_missing_source_fails_replay` | `crates/kutha-runtime/tests/m180_replay_meta.rs:230` |
| `enrichment_payload_tamper_fails_on_replay` | `crates/kutha-runtime/tests/m182_meta_threats.rs:193` |
| `policy_hash_addressed_enrichment_payload_is_rejected` | `crates/kutha-runtime/tests/m183_digest_falsifiers.rs:67` |
| `fork_at_isolates_prefix` | `crates/kutha-runtime/src/quantum_invariants.rs:274` |
| `r1_fork_cut_bounds_and_no_interior_reconstruction` | `crates/kutha-runtime/tests/b3_checkpoint_cut_kill.rs:400` |
| `diff_fold_vs_fork_at_matches_post_cut_mints` | `crates/kutha-runtime/tests/b4_fork_diff.rs:477` |
| `kill_runtime_diff_equals_diff_folds_aa_empty_and_mirror` | `crates/kutha-runtime/tests/b6_fork_query_kill.rs:252` |
| `parses_one_hop_to_query_ir_and_executes_as_of` | `crates/kutha-runtime/tests/m003a_gpml.rs:43` |
| `typed_cut_is_not_stringly_and_prepare_matches_handwritten_ir` | `crates/kutha-runtime/tests/m003a_gpml.rs:157` |
| `b3_workload_render_parse_parity` | `crates/kutha-runtime/tests/m003a_gpml_differential.rs:107` |
| `ff5_gpml_as_of_matches_typed_facade` | `crates/kutha-runtime/tests/m003a_gpml_differential.rs:124` |
| `typed_tt_and_limit_are_preserved` | `crates/kutha-runtime/tests/m003a_gpml_differential.rs:176` |

---

## 6. The three tests that would most improve the map

Each is chosen because a **promoted** cell currently rests on a weaker claim than its `must` line, so writing the test converts an honest residual into evidence (or into a red gate).

1. **ADR-041 — `conjunctive_hops_use_leapfrog_and_equal_scan`** (`crates/kutha-runtime/tests/`). Assert that a two-start, two-hop conjunctive workload through `query_ir::execute` reaches `leapfrog_intersect` and returns the same rows as the per-hop nested walk. Today leapfrog has no caller outside `src/leapfrog.rs` tests (`leapfrog.rs:106` is the only call site), so D041-2 (`ADR-041:57`) is unproven while `capability: named` is proposed for the CSR half. This test is the difference between "a primitive exists" and "the hot path uses it". *Needs product work (wiring), so it needs a lease.*

2. **ADR-040 — `apply_delta_matches_full_rebuild_on_assert_retract_correct`**. Build a lease, apply a changeset for one assert, one retract, one correct, and assert `answer(apply_delta(view, tail)) == answer(rebuild(history, cut))` (the clarification formula at ADR-040:59). Nothing can pass today: `Materializer` has no `apply(delta)` (`src/materializer.rs:6-18`), and `build` does not verify the cut. This is the single test that would let 040 claim the protocol rather than the lease. *Needs product work.*

3. **ADR-012 — `vacuum_refuses_held_range_and_records_the_drop`**. With a declared policy, vacuum a range that intersects a legal-hold fact and require a fail-closed error plus no unreported physical drop; then vacuum an unheld range and require the log to record the drop. Nothing today: `rg -ni vacuum crates/` returns two doc comments that exclude it, and the checkpoint receipt deliberately keeps dead facts (`ADR-012:69`). This is the only test for D012-4, and its absence is why ADR-012 must be promoted with a stated residual. *Needs product work (vacuum pack).*

Runner-up: **ADR-043 — `planner_chooses_scan_or_csr_by_estimate_and_fails_closed_on_inexact`**. It would be the first test for a cell that currently has zero capability and only execution-equivalence evidence (`b5_hop_workload.rs:579`). It is fourth only because 043's cell is not proposed for promotion, so no badge depends on it.

---

## 7. What this report did not verify

- **No test was executed** (contract: no `cargo`). "Would fail if removed" is a reading of the assertion, the code path, and the mutation target, not a run.
- **No mutation run.** No `cargo-mutants` invocation, so R2 confidence is textual.
- **Governor not run.** `uv run kutha-gov ci` was not executed; the checks described (`capability=named` non-empty evidence, `rust_test_asserts`) are read from `.kutha/dictionaries/checks.yaml:1156-1173` and `scripts/kutha_gov/kinds.py`.
- **No coverage tooling**, and no reading of `benches/`. Bench-gate claims are out of scope.
- **ADR-050 / 051 / 090** were not assessed (out of scope, frozen).
- **Cross-reference**: the 2026-10-31 gap register independently reports 204/204 evidence names resolving; my scan reproduces that count and adds the check that all 204 are `#[test]` fns, plus the `cfg(feature)` caveat it does not state.
