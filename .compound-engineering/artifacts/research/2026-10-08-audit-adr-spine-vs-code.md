# Audit: ADR spine 000–002, 010–014 vs code (2026-10-08)

Read-only. CBM `kutha-graph` indexed `2026-10-08T07:14:05Z`; cited paths `metadata_match`. Cargo/uv not run. Cells stay **Proposed**. Honeycomb `delivery: spike` ≠ Accepted.

Status: **DEL** delivered (symbol + named oracle) · **PAR** partial · **NS** not started · **CON** contradicted by code · **STL** ADR lock body stale vs HEAD (clarifications may already say so).

## 1. Decision → status → evidence

| ID | Status | Evidence |
|----|--------|----------|
| D1 log=SoT / fold | PAR | `store::open`+`commit::committed_prefix` 56; SEG-01. Cypher/HNSW/Rocks **store** absent. Oracle: `persist_empty_dir_writes_segments_not_jsonl` |
| D2 quantum + UUID v7 | PAR | `pipeline::commit_user_cascade` 243; `Event::new` v7 255. Derived: `Event::stable_derived` 312 (not v7). Oracle: `det02_two_ingest_derived_event_ids_equal` |
| D3 dict-first agents | PAR | 5 reserved rels `event.rs` 144. Six dicts frozen (ADR-050). Oracle: `allow_relation_appends_as_log_fact` |
| D4 VT×TT AS OF | PAR | `Fact.is_live_at` 143; `as_of`=`live_at(MAX,vt)` 82. `Op` clocks still `u64`. Oracle: `ff5_as_of_t1_differs_from_as_of_t2_on_statute_log` |
| D5 reversible plugins | PAR | `csr_lease_at` via `CsrMaterializer` 414; `typed_csr_lease_at` still `from_fold` 432. HNSW NS |
| D6 Rust+Tokio+Rocks store | CON/STL | No `tokio` in Cargo.toml (`rg`). Rocks = unwired `rocks_index`. Clarification 2026-10-04; lock table L182 stale |
| D7 profiles A/B/C | NS | No profile switch in runtime |
| D8 snapshots/segments; writer | PAR/STL | `segment::persist_log`; `kutha.lock` (ADR-010 WRITER-01). D8.2 “no try_lock” (ADR-000 L211) stale. Oracle: `two_writer_attach_fails_closed` |
| D9 replay/fork/diff | PAR | `replay_check` 19; `fork_at` 155; **no** `pub fn diff` (`rg`). Oracle: `fork_at_isolates_prefix` |
| D10 legal wedge | PAR | FF5 + TR-02 pins; no legal pack. Oracle: `tr02_force_vt_publication_anchor_decision_as_of` |
| ADR-001 vision | PAR | One-liner matches spike core; packs/HNSW/agents not product |
| ADR-002 space | PAR | Crates, not vertical-slice ports as SoT |
| ADR-002 time | PAR | Log+fold+Horn; see D2 id split |
| ADR-002 Cui convolution | NS | `rg` max_convolution empty; `max_cascade` int only |
| D010-1 log≠lease | PAR | Log SoT + droppable CSR/snapshot. Resume encoding still thin |
| D010-2 emit→idle | PAR | `pipeline_kind` 185. Residuals: intern Define side-path; GROUP-01. Oracle: `casc01_disallowed_second_hop_absent_from_log` |
| D010-3 typed writes | DEL | `Op::{Assert,Retract,Correct,CorrectInterval}` 168. Oracle: `retract_by_event_id_invalidates_support` |
| D010-4 Rocks WAL cousin | PAR/STL | Cousin = `KUTHASEG`/`KUTHAWAL`. Lock title L105 stale |
| D011-1 lean + v7 + N-ary | PAR | Ternary SRO; no n-ary schema. Taxonomy create/patch/invalidate **STL** vs `Op` names |
| D011-2 intern lease | DEL | `TermDictionary` intern.rs 6; `Runtime::intern` logs `Define` 335. Oracle: `intern_appends_define_for_new_terms_only` |
| D011-3 how-polynomials | NS | Comment provenance.rs 121; no MATCH compiler |
| D011-4 caused_by | PAR | Behavior `caused_by: EventId`. PROV activity types NS |
| D011 ids/delivery_key | PAR | claim/support/key on Assert. **G-A01** vs Correct. Oracle: `identical_assert_same_delivery_key_does_not_mint_second_support`; M022 `keyed_assert_retry_*` **not** in honeycomb |
| D012-1 snapshot lease | DEL | `open`→`replay_check` store.rs 292. Oracle: `open_rejects_tampered_snapshot` |
| D012-2 BACH aging | NS | CSR lease ≠ LSM aging |
| D012-3 SEM tiers | NS | RAM-full fold |
| D012-4 vacuum+hold | NS | `rg` vacuum empty in crates |
| D012-5 vector tier | NS | Frozen HNSW |
| D013-1 two axes | PAR | `live_at` both; `as_known_on` TT-only 106. Oracle: `true_on_d_differs_from_known_on_d` |
| D013-2 typed invalidation | PAR | Retract-all-live `fold.apply` 698. Missing id = silent no-op. Oracle: `retract_correct_interval_event_id_clears_all_residuals` |
| D013-3 no cosine supersession | PAR | Correct/Retract deterministic. Independent supports: `claim_id_distinct_from_support_slot` |
| D013-4 interval index | NS | `live_facts_at` scans `facts()` |
| HORN-KC / ASOF / C01 / TR-02 | DEL* | Decided, crate pins: `known04_*`, `asof_elig01_*`, `c01_applies_to_assert_encodes_application_window`, `tr02_*`. *semantics ≠ 2026-09-13 contract fixture |
| D014-1 const-size receipt | PAR | `Op::QuantumOutcome` + SHA digest. `event_ids: Vec` leaks set/size (`receipt.rs` 6) |
| D014-2 max-convolution | NS | `max_cascade` abort only. Oracle: `budgets_0_1_2_distinguish_zero_partial_full_after_persist_open` |
| D014-3 anchor≠SoT | PAR | No external Merkle. GROUP-01 visible prefix. Oracle: `mixed_typed_dict_partial_group_is_visible` |
| D014-4 prompt versions | PAR | `rule_version` pin; no meta-prompt id on receipt |
| D014-5 read receipts | NS | No query surface |

Honeycomb.yaml: ADR-011/013 `capability: named`; 000/001/002/010/012/014 `capability: none` despite evidence lists.

## 2. Logical / semantic gaps

**G-A01 high · graph-checked** (cargo not run). Keyed retry live-gate vs Correct-copied key.  
Inputs: Assert(s,r,o,key=k) → Correct(id,o2) → Assert(s,r,o,key=k).  
ADR-011: retry of one delivery ≠ second support (`keyed_correct_preserves_delivery_key_and_retry` m013:178).  
Code: `by_delivery_key` indexes **Assert only** (`append_indexed` 44); `delivery_key_retry` 243 treats invalidated mint as not live → remint. After remint, Correct residual **and** new Assert can both carry k. Oracle: assert log_len unchanged after Correct+retry **and** `live_support_count==1`; if M022 remints, that oracle must fail or be withdrawn.

**G-A02 high · graph-checked.** After CorrectInterval, 3 facts share CI `event.id`. Retract clears all (`apply` 698). Correct/CI targeting uses **first-live** (`fact_index_by_event_id` 159).  
Inputs: CI residuals then `Correct{event_id: ci, object: X, vt: whole}`.  
ADR: typed invalidation of the targeted version; 2026-09-13 interval residuals must be explicit.  
Code: only first residual patched; others stay. Oracle: after Correct(ci), `as_of` prefix **and** suffix must match declared policy (none today).

**G-A03 high · graph-checked.** `as_of` is support-live (`fold_query` 82); `derivation_eligible_at` is separate (provenance 122).  
Inputs: Horn Q from P; Retract last P support; query `as_of(vt)`.  
ADR-013 2026-09-13 + contract-validation fixture obs.2: Q loses eligibility. Phase 46 **rejects** hiding.  
Code: Q still in `as_of`. Oracle already: `asof_elig01_as_of_contains_derived_when_ineligible`. Gap is **ADR vs ADR**, not missing test.

**G-A04 med · graph-checked.** Horn ingest-at-head: `search_prems(..., u64::MAX)` eval.rs 250; `live_horns` `is_live_at(MAX,vt)` 274; `as_known_on` never re-derives.  
Inputs: premises ingested after wall-2014; `as_known_on(2014)` vs current fold.  
D013-1 `live_at(tt,vt)` = known-at-tt-about-vt; Horn does not evaluate at historical tt. Oracle: `known04_horn_fires_when_premise_hidden_on_early_cut`.

**G-A05 med · read-in-source.** Retract/Correct missing or already-invalidated `event_id`: `apply` no-ops (Correct 730 `if invalidated skip`). D013-2 “typed”; 2026-10-03 says fail-closed not claimed.  
Oracle: emit Retract(unknown) → `Err`, not Ok+unchanged fold.

**G-A06 med · graph-checked.** D2/D011-1 UUID v7 vs `stable_derived` SHA-16 (event.rs 312) and `stable_define` (290). User ingest still v7. Documented DET-02; lock title still “UUID v7”.

**G-A07 med · graph-checked.** Retract(**original** Assert id) after CI: residuals minted with **CI** `event.id` (`apply` 788). Original already invalidated. Residuals stay. No oracle. Expect-per-“retract the claim”: all windows gone. Actual: only original row dead.

**G-A08 med · graph-checked.** `as_known_on` → `known_at(tt)` ignores VT (`is_known_at` 164). Closed future-VT intervals appear. Oracle: `closed_vt_interval_survives_as_known_on`. D4 “AS OF” does not name this cut.

**G-A09 med · read-in-source.** `Runtime::intern` `apply`+`append_indexed` Define with no admit/QO (quantum.rs 335). F11 “one path” residual. `committed_prefix` keeps SelfCommit. Oracle: intern-only crash vs `pipeline` Assert atomicity.

**G-A10 med · graph-checked.** D014-1 constant-size, size must not leak payload. `QuantumReceipt.event_ids: Vec` + `Op::QuantumOutcome.event_ids`. Oracle: receipt byte length independent of cascade width (would fail today).

**G-A11 low · read-in-source.** GROUP-01: Typed/Dictionary prefix visible (`mixed_typed_dict_partial_group_is_visible`). D010-2 one quantum. Characterized, not closed.

**G-A12 low · read-in-source.** D011-3 polynomials NS; D012-4 vacuum NS (AS-OF-after-GC undefined); D014-2 convolution NS; interval index NS (linear scan).

## 3. Stale docs (file:line)

- `docs/ADR/ADR-000-…md:9` “u64 type aliases” vs `:13` newtypes (historical stack).
- `ADR-000:110` Storage RocksDB/WAL; `:182` Tokio+Rocks event store; `:211` no `try_lock` (lock shipped).
- `ADR-010:105` RocksDB `WALRecoveryMode` as cousin (superseded in later blocks).
- `ADR-011:36` P0 taxonomy create/patch/invalidate/`llm.*` vs live `Op`.
- `docs/architecture/semantic-gap-review.md:24` F2 `Op::Retract { fact_seq }` — HEAD is `event_id` (event.rs 181).
- `semantic-gap-review.md:29` F7 aliases (clocks newtyped; remainder is pack stamps).
- `semantic-gap-review.md:30` F8 CSR beside Materializer — `csr_lease_at` now builds via trait (quantum.rs 421); `typed_csr_lease_at` still `from_fold`.
- `semantic-contract-validation.md:29` receipt not persisted — SoT is `Op::QuantumOutcome`.
- `semantic-contract-validation.md:30` execution replay absent — `horn_behavior_reproducible` in `replay_check` 36.
- `p0-spike-inventory.md:9` still leads with jsonl+WAL SoT beside SEG-01.
- `.kutha/dictionaries/honeycomb.yaml` no `keyed_assert_retry_*` (M022); ADR-010 `:115` `capability: none` with a long evidence list.

Method: CBM `list_projects` / `search_graph` / `trace_path` / `get_code_snippet` / `search_code` then `rg` arbiter. `check_index_coverage` on cited crate+ADR paths: `no_recorded_issue`. Negative NS claims are `rg` over `crates/`.
