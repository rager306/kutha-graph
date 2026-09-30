# Changelog

All notable changes to **this repository** are recorded here. This is project history, not GitHub Releases and not a Compound Engineering skill catalog.

Keep **product** (`crates/`) and **process** (harness) distinct. Dated entries may also use a **Trajectory** subsection so L_map / L_delivery / L_capability are not collapsed. New entries prefer Keep a Changelog groups (`Added` / `Changed` / `Fixed`) inside those plane headings.

## 2026-10-01 — Process: governor observes HOT-01..03

### Process

### Added

- Governor bridge `B-m012a-s06` / `m012a-s06-hot-indexes` and three `observe_cargo.required` names for HOT-01..03 (`as_of_and_claim_supported_at_skip_non_overlapping_facts`, `discard_csr_lease_and_snapshot_rebuild_keeps_as_of`, `csr_lease_at_builds_via_materializer`). LOG, REF, ING, DUR, and TIME names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-040 and ADR-041 evidence lists include the HOT names. Active Slice remains **S06**. Freeze until M002 unchanged. Not a GED-class fork-diff API.

## 2026-10-01 — Product: droppable CSR via Materializer

### Product

### Changed

- Untyped `csr_lease_at` builds through `CsrMaterializer` then unloads. `from_fold` consumes the fold live-fact iterator. Dropping the lease does not change `as_of` or fingerprint. Indexes and CSR stay in-memory leases, not snapshot SoT. Typed CSR remains `TypedCsrLease::from_fold` (named spike limit).

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S06**. Freeze until M002 unchanged. Not M012 dictionaries-as-facts.

## 2026-10-01 — Product: fold-internal hot as_of and claim_supported_at

### Product

### Added

- Fold-internal VT and claim maps so `as_of` / `claim_supported_at` skip non-overlapping facts. Answers match a brute-force `is_live_at` filter of `facts()`. Maps skip-serialize, rebuild from facts after snapshot open, and are not fingerprint input.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S06**. Freeze until M002 unchanged. Not M012 dictionaries-as-facts.

## 2026-10-01 — Process: lease M012a S06 for Phase 16

### Process

- Active Slice **S06** (fold-internal hot indexes for `as_of` / `claim_supported_at`). Active Milestone stays **M012a**; `L_delivery=M012a-S05-done` until S06 closes.

### Trajectory

- GSD Phase 16 may edit product crates for HOT-01..03. Last M012a slice. Freeze until M002 unchanged. Honeycomb Proposed.

## 2026-10-01 — Product: M012a S04+S05 verify, persist, time scale

### Product

- `open` verifies snapshot via `replay_check` (tampered snapshot → `InvalidData`).
- `persist` replaces `events.jsonl` atomically (temp + rename). `Define` ids are name-stable (`Event::stable_define`).
- `TimeScale::{YearCe, LogSequence}` declared; fixture oracles map years and TT as log sequence.

### Process

- Governor `m012a-s04-verify-persist` / `m012a-s05-time-scale` + FSM observe for five DUR/TIME tests. GSD Phase 15 closed.

### Trajectory

- `L_delivery=M012a-S05-done`; Active Slice **None** until S06 leased. Active Milestone stays M012a. Freeze until M002 unchanged.

## 2026-10-01 — Process: governor observes DUR-01..03 and TIME-01..02

### Process

### Added

- Governor bridges `B-m012a-s04` / `m012a-s04-verify-persist` and `B-m012a-s05` / `m012a-s05-time-scale`, plus five `observe_cargo.required` names for the DUR and TIME oracles. LOG, REF, and ING names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-012 and ADR-013 evidence lists include the new names. Active Slice remains **S04**. Freeze until M002 unchanged.

## 2026-10-01 — Product: declared YearCe valid-time and log-sequence transaction-time

### Product

### Added

- `TimeScale::{YearCe, LogSequence}` with `VALID_TIME_SCALE` and `TRANSACTION_TIME_SCALE`. Fixture integers 2015/2017/2021 are Gregorian years CE; `Event.ingested_at` is log sequence, not wall-clock. Named tests in `m012a_time_scale`. No TT-to-wall calendar map.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S04**. Freeze until M002 unchanged.

## 2026-10-01 — Product: verify-on-open, atomic jsonl persist, stable Define ids

### Product

### Added

- `open` verifies snapshot identity against the log (`replay_check`) and rejects a tampered snapshot as `InvalidData` / `ReplayDivergenceError`.
- Dictionary `Op::Define` ids are name-stable (SHA-256 of the ASCII prefix `kutha-define-id` plus the term bytes) across persist/open/persist. Live `intern()` still mints UUID v7.

### Changed

- `persist` replaces `events.jsonl` by writing a same-directory temp file, syncing, then renaming. A failed temp write leaves the previous complete jsonl in place.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S04**. Freeze until M002 unchanged. Not M012 dictionaries-as-facts.

## 2026-10-01 — Process: lease M012a S04 for Phase 15

### Process

- Active Slice **S04** (verify-on-open, atomic persist, stable Define). Phase 15 GSD wave also covers S05 declared time scale. Active Milestone stays **M012a**; `L_delivery=M012a-S03-done` until Phase 15 closes.

### Trajectory

- GSD Phase 15 may edit product crates for DUR-01..03 and TIME-01..02. Freeze until M002 unchanged. Honeycomb Proposed.

## 2026-10-01 — Product: M012a S03 idempotent ingest

### Product

- Assert carries optional `delivery_key` and `polarity`; identical keyed Assert does not mint a second support.
- `conflict_report_at` partitions by stored polarity. Named oracles in `m012a_idempotent_ingest`.

### Process

- Governor `m012a-s03-idempotent-ingest` + FSM observe for three ING tests. GSD Phase 14 closed.

### Trajectory

- `L_delivery=M012a-S03-done`; Active Slice **None** until S04 leased. Active Milestone stays M012a. Freeze until M002 unchanged.

## 2026-10-01 — Process: governor observes ING-01..03

### Process

### Added

- Governor bridge `B-m012a-s03` / check `m012a-s03-idempotent-ingest` and three `observe_cargo.required` names for the ING-01..03 oracles. LOG and REF names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-011 and ADR-013 evidence lists include the new names. Active Slice remains **S03**. Freeze until M002 unchanged.

## 2026-10-01 — Product: claim vs support identities

### Product

### Added

- Named ING-02 oracle `claim_id_distinct_from_support_slot`. `Op::Assert` and `Fact` rustdoc name claim (proposition), minting EventId (support slot), and `delivery_key` (retry) as three identities.

### Changed

- `conflict_report_at` takes claim, transaction-time, and valid-time only and partitions live supports by stored `Fact.polarity`. Correct/CorrectInterval residuals copy polarity; an object-changing patch row flips Positive/Negative. Leased e2e fixture Asserts store Positive.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S03**. Freeze until M002 unchanged.

## 2026-10-01 — Product: delivery-key Assert ingest

### Product

### Added

- `Op::Assert` carries an optional `delivery_key` and `SupportPolarity` (default none). Re-emitting an identical Assert with the same non-empty key returns the original `EventId` and does not append a second support.
- Same delivery key with a different Assert payload fails closed (`DeliveryKeyConflict`) and does not append. Persist/open still honors the durable key.

### Changed

- Fold `Fact` copies `delivery_key` and polarity; both mix into the event digest and fold fingerprint.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S03**. Freeze until M002 unchanged. Not M012 dictionaries-as-facts.

## 2026-10-01 — Process: lease M012a S03

### Process

- Active Slice **S03** (idempotent ingest / delivery keys; claim ≠ support slot). Active Milestone stays **M012a**; `L_delivery=M012a-S02-done` until S03 closes.

### Trajectory

- GSD Phase 14 may edit product crates for ING-01..03. Freeze until M002 unchanged. Honeycomb Proposed.

## 2026-10-01 — Product: M012a S02 stable references

### Product

- Retract, Correct, and CorrectInterval target minting `EventId` (`Fact.event_id`); fold-local `seq` stays a lease.
- Justification cites and admission key on EventId across fork/rebuild. Named oracles in `m012a_stable_refs`.

### Process

- Governor `m012a-s02-stable-refs` + FSM observe for three REF tests. GSD Phase 13 closed.

### Trajectory

- `L_delivery=M012a-S02-done`; Active Slice **None** until S03 leased. Active Milestone stays M012a. Freeze until M002 unchanged.

## 2026-10-01 — Product + Process: rebuilt-fold EventId oracle

### Product

### Added

- Named REF-03 oracle `rebuilt_fold_renumbered_seqs_apply_same_retract_and_cite_payloads`: dropping a filler Assert renumbers fold-local seqs; the same Retract and JustificationCite Events still bind.

### Process

### Added

- Governor bridge `B-m012a-s02` / check `m012a-s02-stable-refs` and three `observe_cargo.required` names for the REF-01..03 oracles. LOG names stay required.

### Trajectory

- Honeycomb stays **Proposed**. ADR-011 and ADR-061 evidence lists include the new names. Active Slice remains **S02**. Freeze until M002 unchanged.

## 2026-10-01 — Product: EventId justification cites

### Product

### Changed

- `Op::JustificationCite` and `check_admission` name source supports by minting `EventId` only. Fold-local seq is no longer in the durable cite payload.
- `fork_at` of a prefix that contains the cite still admits after the parent retracts that EventId.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S02**. Freeze until M002 unchanged.

## 2026-10-01 — Product: minting EventId retract and correct

### Product

### Changed

- `Op::Retract`, `Op::Correct`, and `Op::CorrectInterval` target the minting `EventId` stored on `Fact.event_id`. Fold-local `seq` remains a lease index (CSR / conflict report), not the operator key.
- `Runtime::emit` returns `UnknownFact { event_id }` and does not append when no Fact was minted by that id. Interval patches that do not intersect still fail closed as `IntervalPatchRejected { event_id }`.

### Trajectory

- Honeycomb stays **Proposed**. Active Slice remains **S02**. Freeze until M002 unchanged. Not ADR-011 Accepted.

## 2026-10-01 — Process: lease M012a S02

### Process

- Active Slice **S02** (stable EventId / proposition refs for Retract, Correct, and justification cites). Active Milestone stays **M012a**; `L_delivery=M012a-S01-done` until S02 closes.

### Trajectory

- GSD Phase 13 may edit product crates for REF-01..03. Freeze until M002 unchanged. Honeycomb Proposed.

## 2026-09-30 — Product: M012a S01 log-native SoT

### Product

- Quantum outcomes, justification cites, and resume persist as log records (`Op::QuantumOutcome`, `Op::JustificationCite`); sidecars are droppable leases.
- `provenance_fingerprint` mixes those log-native bytes (`kutha-prov-log-native`). Named oracles in `m012a_log_native`.

### Process

- Governor check `m012a-s01-log-native` + FSM observe names for the three LOG tests. GSD Phase 12 closed.

### Trajectory

- `L_delivery=M012a-S01-done`; Active Slice **None** until S02 leased. Active Milestone stays M012a. Freeze until M002 unchanged. Honeycomb Proposed.

## 2026-10-01 — Log-native SoT

### Product

### Added

- `Op::QuantumOutcome` fold-noop events on the product log. After persist, deleting `quantum_outcomes.jsonl` still reconstructs Zero / Partial / Full (`discard_outcomes_sidecar_keeps_reconstructible_disposition`).
- `Op::JustificationCite` fold-noop events. After persist, deleting `justifications.jsonl` still reconstructs cites and `check_admission` (`discard_justifications_sidecar_keeps_admission_and_resume`). Resume is a log-native `Op::QuantumOutcome` with `disposition: resume`.

### Changed

- Quantum outcome and justification SoT is the event log. Sidecar JSONL files remain written leases; open hydrates from log Events when they exist and does not let an empty sidecar wipe them.
- `provenance_fingerprint` domain tag is `kutha-prov-log-native`; the mix includes Behavior plus log-native outcome and justification Event bytes (`provenance_fingerprint_moves_when_log_native_record_bytes_change`).

### Process

### Added

- Governor bridge `B-m012a-s01` / check `m012a-s01-log-native` and three `observe_cargo.required` names for the LOG-01..03 oracles.

### Trajectory

- Honeycomb stays **Proposed**. Harness lease remains M012a / S01. Freeze until explicit M002 is unchanged. Not ADR-014 Accepted.

## 2026-09-30 — Process: lease M012a

### Process

- Harness lease: Active Milestone **M012a**; Active Slice **S01** (log-native outcomes/justifications); `L_delivery=M012a-leased`. Phase stays **H5**. Freeze until explicit M002 unchanged.
- Governor Active Milestone patterns accept letter-suffix ids (`M012a`). `.kutha/ROADMAP.md` marks M011 CLOSED and M012a ACTIVE with S01–S06 checkboxes.

### Trajectory

- Product plane may land single-log SoT + stable-reference work (F1–F3, F6 verify/atomic/Define, F7, F8). F4/F5 wait for M012. Rocks / Cypher / HNSW / legal pack stay frozen. Honeycomb stays Proposed.

## 2026-09-30 — Process: semantic governor

### Process

### Added

- `kutha-gov selftest` copies the tree once, mutates only that copy, and fails closed on UNPROVEN or VACUOUS. FSM `run_selftest` records `h5_selftest`; `ci` labels H5 from that evidence.
- Kinds `rust_test_asserts`, `cite_equals`, `refs_resolve`, and `file_max_lines`. Named `fn`-needle checks and honeycomb evidence resolve through asserting `#[test]` bodies. `cite-lease` also matches `L_map=`, `L_capability=`, and Active Milestone M### phrasing to STATE.
- Intake section “What a green check proves”: presence is not meaning; every check is mutation-provable or carries a skip reason. META allowlists the new kinds and `run_selftest`. `AGENTS.md` Commands includes `uv run kutha-gov selftest` (line budget ≤110, no lease tokens).

### Changed

- `run_checks` default budget and `.env.example` are 48 so `ci` cannot truncate Cui slices (32 would drop checks).
- Control-loop rows `I-F1-outcomes` … `I-F8-hot-reads` record F1–F8 as `disposition: deferred` (`until` names M012a / M012 / M002). Recorded, not enforced.
- ROADMAP H5 checkbox is checked; `h4-lease` requires the done form without promoting ADR-090. Dogfood ladder marks H5 **Done**. It remains the current rung (no H6).

### Fixed

- Live-repo selftest found no vacuous named test and no vacuous YAML row (`docs-coupling` skips: depends on the live git diff).
- Harness tests no longer leak `/tmp/kutha-selftest-mini-*` and `/tmp/kutha-rust-test-*` directories (cleanup registered with `atexit`).
- Harness scratch is repo-local: `scripts/tests/conftest.py` and `kutha-gov selftest` create temp trees under gitignored `tmp/` instead of system `/tmp`; selftest's tree copy skips root `tmp/`.

### Trajectory

- Operator Phase stays **H5**, now closed on the dogfood ladder. Active Milestone stays M011; Active Slice None; freeze and honeycomb **Proposed** unchanged. Governor green is not product correctness, not ADR Accepted, and not L_capability. F1–F8 are product gaps deferred to unleased M012a (F4/F5 until STATE names M012; F6 durability protocol until STATE names M002).

## 2026-09-30 — Process: semantic-gap review and Proposed honeycomb notes

### Process

- Architecture note `docs/architecture/semantic-gap-review.md` traces F1–F8 onto the honeycomb (per-cell verdicts, D1–D10 lock assessment). It is not an ADR and not a lease.
- Twelve honeycomb ADRs received dated 2026-09-30 Proposed clarifications and/or open questions. Status stays Proposed. ADR README points at the review.
- Later-milestones proposed order (non-authoritative): M012a (single-log SoT and stable refs) → M012 (dictionaries as facts) → M002 (log durability protocol first). Not leased.

### Trajectory

- Honeycomb stays Proposed. No cell promoted. Green governor is not ADR Accepted and not L_capability.

## 2026-09-30 — Process: H5 operator lease and AGENTS diet

### Process

- Operator lease Phase named **H5** (semantic governor + lean agent context). Dogfood ladder and `h4-lease` / `dogfood` needles name H5 without promoting ADR-090.
- `AGENTS.md` diet: 194 lines down to about 100 durable operating rules. Volatile notes relocated (P0 spike inventory, Leapfrog, applicability Agent notes, README Layout). Stale S03 live-lease caption replaced with a STATE pointer.

### Trajectory

- Operator Phase is **H5**; Active Milestone stays M011; Active Slice None; freeze and honeycomb Proposed unchanged. Green governor is not ADR Accepted and not L_capability.

## 2026-09-30 — Product: end-to-end candidate fixture

### Product

### Added

- Durable `justifications.jsonl` sidecar (`JUSTIFICATIONS_REL`) beside outcomes; `record_justification` / `check_admission` fail-closed on stale `source_fact_seqs`, ineligible derivation, or `rule_version` mismatch. Missing sidecar loads empty (cannot admit).
- `conflict_report_at` reports `positive_supports` and `negative_supports` without picking a winner.
- Named tests `e2e_fixture_supports_and_conflict_at_named_cuts`, `e2e_justification_cites_sources_and_rejects_stale_admission`, `e2e_incremental_matches_reconstruct_after_discarding_leases` (FIX-01, FIX-02, FIX-03).

### Process

- FSM `observe_cargo.required` plus `m011-e2e` / `B-m011-e2e` needles for the three named FIX tests (GATE-01).
- Harness lease advanced: **S08** delivered (`L_delivery=M011-S08-done`); Active Slice **None**. M011 tail (S04–S08) closed. Not ADR Accepted; not closed product delivery.

### Trajectory

- **S08 delivered** (`L_delivery=M011-S08-done`); Active Slice cleared. ADR-013, ADR-011, ADR-012, and ADR-040 remain Proposed (not Accepted, not L_capability). Evidence names the FIX oracles `e2e_fixture_supports_and_conflict_at_named_cuts`, `e2e_justification_cites_sources_and_rejects_stale_admission`, and `e2e_incremental_matches_reconstruct_after_discarding_leases`. Green governor is not ADR Accepted and not L_capability. Conflict report is not four-valued logic.

## 2026-09-30 — Process: lease Active Slice S08 after S07 delivery

### Process

- Harness lease advanced: **S07** remains delivered (`L_delivery=M011-S07-done`); Active Slice **S08** (end-to-end candidate fixture — semantic-contract observations 1–5 at named cuts). Not ADR Accepted; not closed product delivery.

### Trajectory

- Active Slice is **S08**; honeycomb cells stay Proposed (not Accepted, not L_capability). S07 provenance tests stay registered. Green governor is not ADR Accepted and not L_capability.

## 2026-09-30 — Product: provenance check apart from state replay

### Product

### Added

- `Op::Behavior.rule_version` (`#[serde(default)]`); fold still ignores lineage (`name` / `caused_by` / `rule_version`). `Runtime::provenance_fingerprint` / `provenance_check` sit beside unchanged `replay_check` (ADR-060 obligation 2; state replay stays obligation 1).
- Named test `provenance_detects_caused_by_swap_when_state_fingerprint_matches` (PROV-01): swap `caused_by` among valid priors; state fingerprint matches; provenance digest moves; both `replay_check` Ok.
- Named test `provenance_detects_rule_version_change_when_state_fingerprint_matches` (PROV-02): change only `rule_version` (`r1` → `r2`); state fingerprint matches; provenance digest moves; no execution replay.

### Process

- FSM `observe_cargo.required` plus `m011-provenance` / `B-m011-provenance` needles for named PROV-01 and PROV-02 tests (GATE-01).
- Harness lease advanced: **S07** delivered (`L_delivery=M011-S07-done`); Active Slice **None**. S08 remains unleased. Not ADR Accepted; not closed product delivery.

### Trajectory

- **S07 delivered** (`L_delivery=M011-S07-done`); Active Slice cleared until an explicit S08 lease. ADR-060 and ADR-011 honeycomb evidence names `provenance_detects_caused_by_swap_when_state_fingerprint_matches` and `provenance_detects_rule_version_change_when_state_fingerprint_matches`; map stays Proposed on both (not Accepted, not L_capability). Green governor is not ADR Accepted and not L_capability. `replay_check` Ok is not execution replay.

## 2026-09-30 — Process: lease Active Slice S07 after S06 delivery

### Process

- Harness lease advanced: **S06** remains delivered (`L_delivery=M011-S06-done`); Active Slice **S07** (provenance and rule-version check apart from state replay). S08 remains unleased. Not ADR Accepted; not closed product delivery.

### Trajectory

- Active Slice is **S07**; ADR-040 and ADR-060 remain Proposed (not Accepted, not L_capability). S06 typed-CSR tests stay registered. Green governor is not ADR Accepted and not L_capability.

## 2026-09-30 — Product: typed CSR lease (labels + support multiplicity)

### Product

- `TypedEdge` / `TypedCsrLease` beside untyped `CsrLease`; `Runtime::typed_csr_lease_at` + `edges_out` project one edge per live Fact (`relation`, `object`, `claim_id`, `fact_seq`). Untyped `CsrLease::from_fold` object-only dedup unchanged.
- Named test `typed_csr_preserves_relation_labels_and_support_multiplicity` (CSR-01): ≥2 relations and ≥2 supports visible on typed edges; untyped neighbors stay a singleton object.
- Named test `untyped_csr_neighbor_set_and_ff5_still_hold` (CSR-02): untyped neighbor-set + FF3-style drop/rebuild; existing FF5 statute observe stays green.

### Process

- FSM `observe_cargo.required` plus `m011-typed-csr` / `B-m011-typed-csr` needles for named CSR-01 and CSR-02 tests (GATE-01).
- Harness lease advanced: **S06** delivered (`L_delivery=M011-S06-done`); Active Slice **None**. S07–S08 remain unleased. Not ADR Accepted; not closed product delivery.

### Trajectory

- **S06 delivered** (`L_delivery=M011-S06-done`); Active Slice cleared until an explicit S07+ lease. ADR-040 and ADR-041 honeycomb evidence names `typed_csr_preserves_relation_labels_and_support_multiplicity` and `untyped_csr_neighbor_set_and_ff5_still_hold`; map stays Proposed on both (not Accepted, not L_capability). Green governor is not ADR Accepted and not L_capability.

## 2026-09-30 — Process: lease Active Slice S06 after S05 delivery

### Process

- Harness lease advanced: **S05** remains delivered (`L_delivery=M011-S05-done`); Active Slice **S06** (typed CSR lease keeps labels and support multiplicity). S07–S08 remain unleased. Not ADR Accepted; not closed product delivery.

### Trajectory

- Active Slice is **S06**; ADR-014 remains Proposed (not Accepted, not L_capability). S05 outcome tests stay registered. Green governor is not ADR Accepted and not L_capability.

## 2026-09-29 — Product: persisted quantum outcome sidecar

### Product

- Authoritative `quantum_outcomes.jsonl` sidecar beside the log (written by `store::persist` after events; loaded on every `store::open` path). Not a droppable lease — discarding `snapshot.json` keeps outcome history (D-O1 / ADR-010).
- Dispositions `Zero` / `Partial` / `Full` from abort × `events_in_quantum`; `Runtime::record_resume` appends an explicit `Resume` row (open never auto-resumes). Call `Ok(QuantumOutcome)` remains not completion proof (ADR-014).
- Named test `budgets_0_1_2_distinguish_zero_partial_full_after_persist_open` (OUT-01): budgets 0/1/2 via `Runtime::new(0|1|default)` survive persist→open as Zero/Partial/Full.
- Named test `crash_after_prefix_has_no_terminal_success_until_explicit_resume` (OUT-02): after a committed prefix without a Full row, open invents no terminal success until `record_resume`.

### Process

- FSM `observe_cargo.required` plus `m011-quantum-outcome` / `B-m011-quantum-outcome` needles for named OUT-01 and OUT-02 tests (GATE-01).
- Harness lease advanced: **S05** delivered (`L_delivery=M011-S05-done`); Active Slice cleared. S06–S08 remain unleased. Not ADR Accepted; not closed product delivery.

### Trajectory

- **S05 delivered** (`L_delivery=M011-S05-done`); Active Slice cleared until an explicit S06+ lease. ADR-014 honeycomb evidence names the OUT-01/OUT-02 tests; map stays Proposed (not Accepted, not L_capability). Green governor is not ADR Accepted and not L_capability.

### Product

- `Op::CorrectInterval { fact_seq, object, patch_from, patch_to }`: explicit interval patch on a live fact. Fold invalidates the original row (VT bounds unchanged) and pushes prefix residual, clipped replacement, and suffix residual sharing `claim_id`.
- Named test `interval_patch_leaves_vt_2012_and_2021_residuals` (CORR-01): after patch `[2015, 2020)` on `[2010, ∞)`, `as_of(2012)` and `as_of(2021)` keep source `a` as `P`; interior `as_of(2017)` is `P-prime`.
- Named test `whole_version_correct_does_not_invent_residuals` (CORR-02): whole-version `Op::Correct` with narrower VT `[2015, 2020)` on `[2010, ∞)` invents no live residual of `P` at VT 2012 or 2021; interior `as_of(2017)` is the replacement only.
- Fail-closed interval-patch writes: unknown `fact_seq` (`UnknownFact`); non-intersecting (including half-open touching endpoints), inverted, and not-live targets (`IntervalPatchRejected`) do not append.

### Process

- FSM `observe_cargo.required` plus `m011-partial-correction` / `B-m011-partial-correction` needles for named CORR-01 and CORR-02 tests (GATE-01).
- Harness lease advanced: **S04** delivered (`L_delivery=M011-S04-done`); Active Slice **S05** (persisted quantum outcome and continuation). S06–S08 remain unleased. Not ADR Accepted; not closed delivery.

### Trajectory

- Active Slice is **S05**; ADR-013 remains Proposed (not Accepted, not L_capability). S04 residual tests stay registered. Green governor is not ADR Accepted and not L_capability.

## 2026-09-16 — Product + Process: M011 S03 thin P→Q eligibility

### Product

- `Runtime::derivation_eligible_at`: Behavior-derived claim `Q` is eligible at a TT×VT cut iff `Q` is still live and its premise claim (from `caused_by`) still has a live support.
- Named test `derived_q_loses_eligibility_when_last_premise_support_withdrawn`: two supports for `P`; retract one → `Q` stays eligible; retract last → eligibility drops; historical `Q` and earlier TT cut remain.

### Process

- Lease `L_delivery=M011-S03-done`. FSM observe + `m011-claim-supports` needles cover S03.

### Trajectory

- ADR-011 / semantic-contract-validation / honeycomb evidence record the thin oracle. Cells remain Proposed; full provenance polynomials and M002 freeze unchanged.

## 2026-09-16 — Process: session handoff after M011 S02

### Process

- Durable handoff at `.compound-engineering/artifacts/handoffs/m011-s02-cbm-mcp.md`. Project `.cursor/mcp.json` points Cursor at local `codebase-memory-mcp` stdio (agent sessions were missing user MCP servers).

## 2026-09-16 — Product + Process: M011 S02 unknown claim and replay lineage

### Product

- `emit` rejects `Assert.claim` that never appeared as `Fact.claim_id` (`UnknownClaim`; log does not grow).
- `replay_check` rejects `Behavior.caused_by` that is not an earlier event id (`BrokenLineage`). Named tests `unknown_claim_does_not_append`, `replay_rejects_behavior_without_prior_cause`.

### Process

- Lease `L_delivery=M011-S02-done`. FSM observe + `m011-claim-supports` needles cover S02.

### Trajectory

- ADR-060 / semantic-contract-validation record causal-ref check. Cells remain Proposed; M002 freeze unchanged.

## 2026-09-16 — Product + Process: M011 S01 independent claim supports

### Product

- `Op::Assert.claim` / `Fact.claim_id`: absent claim opens a new claim (`event.id`); `Some(id)` attaches another support. Fold helpers `live_support_count` / `claim_supported_at`. Correct inherits `claim_id`.
- Named test `retracting_one_support_leaves_claim_supported`.

### Process

- Leased M011; S01 delivered (`L_delivery=M011-S01-done`). Governor `m011-claim-supports` + FSM observe required.

### Trajectory

- ADR-011 / semantic-contract-validation / honeycomb evidence record S01. Cells remain Proposed; M002 freeze unchanged.

## 2026-09-16 — Product + Process: M010 S03 live intern Define; M010 closed

### Product

- `Runtime::intern` appends `Op::Define` for newly created terms; bootstrap `knows`/`knownBy` stay silent. `graph_len()` and graph-oriented `snapshot.log_offset`; `from_snapshot` strips Define before applying the tail.
- Named test `intern_appends_define_for_new_terms_only`. Allowlist tests assert graph growth via `graph_len`.

### Process

- M010 S03 delivered then milestone closed (`L_delivery=M010-closed`). FSM observe requires the live-intern test; `m010-semantic-open` cites S03 needles.
- `pointer_in_other_file` supports `require_template` so an Active Slice must remain unchecked on ROADMAP (blocks done-but-leased drift).

### Trajectory

- ADR-011 / semantic-contract-validation / honeycomb evidence record S03. Cells remain Proposed; M002 freeze unchanged.

## 2026-09-15 — Product + Process: M010 S02 logged term definitions

### Product

- Persist encodes `Op::Define` (dictionary order) on the durable event stream. `open` recovers intern meanings without `snapshot.json` and without `terms.jsonl`. Snapshot open strips Define so log_offset still matches the in-memory graph log.
- Named test `open_without_snapshot_or_terms_file_recovers_from_define_ops`.

### Process

- Lease moved to M010 S02. Governor observe accepts `module::tests::name` cargo lines. `m010-semantic-open` cites the Define recovery test.

### Trajectory

- ADR-011 / semantic-contract-validation record S02. ADR-011 honeycomb evidence names the new test. Cells remain Proposed; M002 freeze unchanged.

## 2026-09-15 — Product + Process: lease M010 S01 semantic recovery open

### Product

- `persist` retains authoritative `terms.jsonl`; `open` recovers fold + intern meanings without `snapshot.json`. Named test `open_without_snapshot_recovers_intern_meanings`.

### Process

- STATE/ROADMAP/README lease M010 S01. Governor `m010-semantic-open` + FSM observe required. Bridge cites store.rs.

### Trajectory

- ADR-011 and semantic-contract-validation note M010 S01 limit (companion terms file; event-logged term ops deferred to S02). Cells remain Proposed; M002 freeze unchanged.

## 2026-09-15 — Process: live H4 membership emit on CI quanta

### Process

- `emit_log` syncs tip process-relation membership into `.kutha/events.jsonl` when the tip set differs from the last logged `allows` edition (H4 dogfood no longer test-only).
- kutha-changelog skill example lease text updated past H3→H4.

### Trajectory

- Cells and freeze unchanged. Active Milestone remains None; no M002 lease.

## 2026-09-15 — Process: post-M001 lease debt wave 1

### Process

- Added governor kind `when_match_then_match` and check `idle-delivery-closed` so Active Milestone None requires `L_delivery=*-closed`.
- Retargeted freeze finding text to the live M002 lease gate; extended `docs-coupling` so STATE and architecture-note diffs require CHANGELOG.
- Indexed `kutha-graph` in GitNexus for session blast-radius queries.

### Trajectory

- Refreshed `docs/architecture/semantic-contract-validation.md` lease wording to Phase H4 / M001-closed (no Active Milestone). Cells and freeze unchanged.

## 2026-09-15 — Process: close M001 milestone

### Process

- Cleared Active Milestone after S01–S03 delivery (`Active Milestone: None`). ROADMAP marks M001 **CLOSED**. Freeze and H4 phase unchanged.

### Trajectory

- `L_delivery=M001-closed`. L_map stays Proposed; L_capability stays ff5-green. No M002 lease.

## 2026-09-15 — Product + Process: H4 process overlay dogfood

### Product

- Tenant ingest maps `process.relations` / `allows` snapshot editions onto `processAllows` and chains intervals. AS OF a prior cut still sees a membership set that later editions dropped. One FF6 name; not ADR-050's six kinds.

### Process

- Tip YAML stays the admit lease. Reserved process relation `allows` logs one sorted membership snapshot per edition. Pytest proves tip-without-`status` rejects `status` without editing the repo tip. Governor `h4-lease` / `h4-membership-as-of` and FSM `observe_cargo.required` name the H4 evidence.
- CodeRabbit follow-ups: reject comma-bearing process relation names (CSV membership ambiguity); `pointer_in_other_file` names the real `other` path; `load_map` rejects non-mapping cells; ROADMAP H4 wording matches shipped overlay scope.

### Trajectory

- ADR-042/060 wording clarified (predicate filter language; `replay_check` unavailable outcome marked proposed). Lease still Phase H4; freeze unchanged.

### Trajectory

- Cells remain **Proposed**. ADR-090 ontology stays frozen. Delivery lease is **H4** overlay dogfood, not a legal pack.

## 2026-09-15 — Process: H4 session handoff after architecture POV

### Process

- Session continuity: `.compound-engineering/artifacts/handoffs/h4-after-architecture-pov.md` records the architecture POV (accept lifecycle self-correction; reject Stages 1–4 as delivery order) and the next CE path: H4 overlay brainstorm/plan, not a legal pack.

### Trajectory

- Cells remain **Proposed**. Delivery lease remains H3; next thin slice is still H4 (ADR-090 overlay). Product crates untouched.

## 2026-09-14 — Process: durable CE artifacts stay under docs_root

### Process

- Standing rule: session-surviving Compound Engineering writes (plans, handoffs, sweep state) go under `.compound-engineering/artifacts/`, not `/tmp`. `ce-handoff` must use `.compound-engineering/artifacts/handoffs/<topic>.md`; the skill's default managed store is OS-evictable.

### Trajectory

- Cells remain **Proposed**. Delivery lease remains H3; next thin slice is still H4 (ADR-090 overlay). Product crates untouched.

## 2026-09-14 — Process: CE skill routing for Russian chat

### Process

- Always-on Cursor rule `.cursor/rules/ce-skills-ru.mdc` maps Russian verbs (коммит, changelog, PR, план, …) onto Compound Engineering skills and `kutha-changelog`. Plugin-cache `description` fields stay English; do not patch them. Git user-rule remains safety-only; commit workflow is `ce-commit`.

### Trajectory

- Cells remain **Proposed**. Delivery lease remains H3; next thin slice is still H4 (ADR-090 overlay). Product crates untouched.

## 2026-09-14 — Process: changelog skill (not a release bumper)

### Process

- Adapted baoyu `release-skills` into `.cursor/skills/kutha-changelog/`: dated `CHANGELOG.md` on product / process / Trajectory planes. Versions stay `0.0.0`. Commit handoff is **ce-commit**; this skill does not tag, `gh release`, or push.

### Trajectory

- Cells remain **Proposed**. Delivery lease remains H3; next thin slice is still H4 (ADR-090 overlay). Product crates untouched.

## 2026-09-14 — Process: L_map compact honeycomb index

### Added

- Compact L_map index `.kutha/dictionaries/honeycomb.yaml` (`kutha-map-honeycomb/v1`): D1–D10 plus all opened ADR cells, with orthogonal `map` / `delivery` / `capability` stages and `depends_on` / `locks` / `evidence` links.
- `kutha-gov map` (optional cell id, neighborhood dump, `--format json`) so agents can load the graph without a second markdown coverage table.
- Governor check `honeycomb-ledger`, FSM state `load_honeycomb`, kind `glob_paths_in_file`, and `yaml_map_list` list/path/embed/glob steps. ADR diffs must include the index (`docs-coupling`).

### Process

- The map dictionary is a fourth intake surface: governor validates shape and links; it does not accept cells, enforce HNSW/Cypher, or treat the index as a delivery backlog.
- Control-loop claims stay in `invariants.yaml`; product freeze/tests stay in `bridges.yaml`. Honeycomb rows are not YAML checks.

### Trajectory

- Cells remain **Proposed**. Delivery annotations (`spike` / `frozen` / `map-only`) and named FF evidence do not promote L_map or L_capability. Lease stays H3; next thin slice is still H4 (ADR-090 overlay). Product crates untouched.

## 2026-09-14 — Process: governor intake split and docs entry

### Added

- Root `README.md` and `CLAUDE.md` (`@AGENTS.md`) as the human / Claude Code entry points.
- Harness `kutha-gov precommit` (dictionary checks only: no cargo quantum, no JSONL) plus `--check ID`, `git_path_implies` coupling, and `.pre-commit-config.yaml`. Neighbor surfaces: daily-archive `--check-only`, law-nexus `--check`; not a baoyu release bumper.
- Control-loop ledger `.kutha/dictionaries/invariants.yaml` and bridge ledger `.kutha/dictionaries/bridges.yaml` (`docs/process/governor-intake.md`). Kind `yaml_map_list` keeps each check id in exactly one ledger.

### Changed

- RuVector adapter citations in ADR-012/014/042/052 and the CE adaptation note: in-repo cards + `.compound-engineering/artifacts/research/ruvector-plugin-adaptation.md`; no machine-local vendor paths. Mapping does not lift the HNSW freeze.

### Process

- Pre-commit profile is FAST-ONLY: ruff on `scripts/` and `kutha-gov precommit`. Product `cargo test` remains `kutha-gov ci`. Versions stay `0.0.0`. CHANGELOG remains plane-dated history, not GitHub Releases.
- Control loop and Kutha requirements stay partitioned: honeycomb / fitness live in ADRs, STATE, and crates tests; `map-only` and `capability` are not process dispositions. Bridges may cite product freeze or named tests; they do not copy L_map or L_capability.

### Trajectory

- Cells remain **Proposed**. Product crates untouched. Delivery lease remains H3; next thin slice is still H4 (ADR-090 overlay).

## 2026-09-13 — Architecture: ruVector adapter mapping (L_map)

### Research & ADRs

- Mapped five named adapter crates onto already-open honeycomb cells via `.compound-engineering/artifacts/research/ruvector-plugin-adaptation.md`. Literature bound stays **163 cards**; no new matrix rows.
- **P-Temporal-Tensor** → ADR-012 D012-5 (vector tiering lease ≠ fact history).
- **P-Retrieval-Receipt** → ADR-014 D014-5 (`paper-constant-size-evidence`; write quantum receipt ≠ read evidence). `ruvector-proof-gate` remains a D014-2 cousin, not a new decision.
- **P-HNSW** → ADR-042 D042-2/3 citing existing cards `ruvector-hnsw-delete-repair`, `paper-acorn-predicate-subgraph`, `paper-navix-filtered-hnsw`. Mapping does not lift the HNSW freeze.
- **P-Agent-Memory** → ADR-052 D052-4 (coherence score ≠ fact validity).
- ADRs cite in-repo cards and the adaptation note. Machine-local vendor paths are not architecture SoT.

### Trajectory

- Cells remain **Proposed**. Product crates untouched. Delivery lease remains H3; next thin slice is still H4 (ADR-090 overlay). This mapping does not authorize HNSW, Cypher, or M002.

## 2026-08-18 — Wave 6: same-second tenant AS OF + one cargo compile per quantum

### Product

- Tenant ingest reports `last_valid_from` as the **emitted** cut after monotonic chaining. Two status rows with the same unix second no longer query AS OF a closed window (`as_of_match=0`).
- Test `h2_same_second_status_as_of_uses_emitted_cut` (TDD).

### Process

- `observe_cargo` optional `build:` compiles `kutha-tenant` once after `cargo test`. `emit_tenant` executes that binary (`KUTHA_TENANT_BIN`, else `$CARGO_TARGET_DIR/debug/kutha-tenant`). No second `cargo run`.
- Checks `plane-mix-dicts` and `tenant-bin`: process/product relation schemas stay distinct; FSM must not `cargo run` the tenant.
- `kutha-gov py` treats ty warnings as errors (`--error-on-warning`).

### Trajectory

- Phase **H3**. Next: H4 waits on ADR-090 overlay (do not start a legal pack). M002 Rocks stays frozen until STATE names it.

## 2026-08-18 — Wave 5: H3 process allowlist + one source for FF names

### Process

- Fail-closed process writes: `.kutha/dictionaries/relations.yaml` (`schema: kutha-harness-relations/v1`). Unknown relation does not append to `.kutha/events.jsonl`.
- FSM loads `relations.yaml` before checks. Override: `KUTHA_HARNESS_RELATIONS_PATH`.
- Named FF tests are CLI **evidence**, not process relations (aligned with H2 tenant mapping of `cargo` only).
- Kind `yaml_needles_in_glob`: FSM `observe_cargo.required` must appear as `fn` in `crates/**/*.rs` (one list, not a copy in checks.yaml).
- `.cbmignore` excludes ideation `mermaid.min.js` from the code index.

### Trajectory

- Phase **H3**. Next: H4 waits on ADR-090 overlay (do not start a legal pack). M002 Rocks stays frozen until STATE names it.

## 2026-08-18 — Wave 4: H2 tenant ingest + unnamed CSR cut

### Product

- Removed `Runtime::csr_lease()` (silent valid-time `0`). Callers name a cut with `csr_lease_at(tt, vt)`.
- H2 tenant: `ingest_harness_jsonl` maps process `harness.run`/`status` → `runStatus` and `harness.observe`/`cargo` → `observed`, with monotonic `valid_to` windows. Same log, two process times, different live status (fixture).
- Binary `kutha-tenant` persists the tenant picture under `KUTHA_TENANT_DIR` (default `.kutha/tenant`, gitignored).
- Allowlist rows `runStatus` and `observed` are delivery facts on the FF6 list, not a second dictionary kind (ADR-050 note).

### Process

- FSM kind `emit_tenant` after `emit_log`: run `kutha-tenant`, require `as_of(last)` to match last status.
- Fold pictures `last_cargo` and `last_tenant`.
- Check `unnamed-csr` forbids bringing `csr_lease()` back.

### Trajectory

- Phase **H2**. Next: H3 (process-plane allowlist). M002 Rocks stays frozen until STATE names it.

## 2026-08-18 — Wave 3: H1 cargo observation as evidence

### Process (harness)

- FSM kind `observe_cargo`: after dictionary checks, run `cargo test --workspace --offline` and require named FF tests (`ff5_…`, `ff6_…`) to appear as `... ok`.
- Cargo output is **evidence**, not product SoT. Success is recorded as `harness.observe` triples; missing/FAILED tests are HIGH and still walk emit/fold so the log exists.
- `emit_log` counts FSM HIGH findings (not only dictionary check highs).
- Timeout from `KUTHA_GOV_CARGO_TIMEOUT_SEC` then `fsm.yaml` `timeout_sec`.
- Static check `observe-required-fn` keeps required names aligned with Rust `fn` lines.
- Split glued FSM tests (`glob_none` vs budget-from-env).

### Trajectory

- Phase **H1**. Next: H2 (harness facts on the Kutha log). M002 Rocks stays frozen until STATE names it.

## 2026-08-18 — Wave 2: FF6 relation allowlist + harness hygiene

### Product

- Fail-closed writes: `Runtime::emit` admits only relations listed in `crates/kutha-runtime/dictionaries/relations.yaml` (`schema: kutha-relations/v1`). Unknown relation → `RuntimeError::UnknownRelation`; log does not grow.
- Path override: `KUTHA_RELATIONS_PATH`. Cascade default: `KUTHA_MAX_CASCADE` (also from repo `.env`).
- FF6 tests: `crates/kutha-runtime/tests/ff6_allowlist.rs`.

### Process

- Kind `glob_none`: forbidden globs (setuptools `*.egg-info` must not live in the tree).
- Check `relation-allowlist` guards the product dictionary.
- FSM transitions must use `event:` (test rejects YAML 1.1 boolean keys).

### Trajectory

- M001 S03 done. Active slice **None**. Next: H1 (cargo test as harness evidence). M002 Rocks stays frozen until STATE names it.

## 2026-08-18 — Wave 1: meta-prompt FSM + Legal PIT FF5

### Process (harness)

- `kutha-gov ci` walks a dictionary FSM (`.kutha/dictionaries/fsm.yaml`): constitution → checks → emit → fold → `ok`|`fail`.
- Unknown FSM kind or missing transition fails closed (`unknown-fsm-kind` / `unknown-transition`).
- Budget and warn-fail come from `.env` (`KUTHA_GOV_BUDGET`, `KUTHA_GOV_FAIL_ON_WARN`), then `fsm.yaml` defaults; CLI flags win. Template: `.env.example`.
- Trajectory also requires **Active Slice** to exist on ROADMAP.
- YAML transition field is `event:` (not `on:` — YAML 1.1 would coerce `on` to boolean `true`).

### Product (kutha-runtime)

- Named cuts: `GraphFold::live_at(tt, vt)`, `GraphFold::as_of(vt)` (no “now” default), `Runtime::csr_lease_at(tt, vt)`.
- FF5: same statute-shaped log, `as_of(2015) ≠ as_of(2021)`; CSR neighbors disagree the same way.
- FF3: dropping a CSR lease does not change log length or fold fingerprint.
- Fixture note: `crates/kutha-runtime/tests/fixtures/legal_pit.md`.

### Trajectory

- M001 S01 and S02 done. Active slice is S03 (FF6 relation-allowlist stub). Honeycomb ADRs stay Proposed.

