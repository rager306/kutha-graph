---
phase: 08-end-to-end-candidate-fixture
verified: 2026-09-30T04:31:46Z
status: passed
score: 10/10 must-haves verified
covered_files:
  - .kutha/dictionaries/bridges.yaml
  - .kutha/dictionaries/checks.yaml
  - .kutha/dictionaries/fsm.yaml
  - .kutha/dictionaries/honeycomb.yaml
  - .planning/phases/08-end-to-end-candidate-fixture/08-01-PLAN.md
  - .planning/phases/08-end-to-end-candidate-fixture/08-01-SUMMARY.md
  - .planning/phases/08-end-to-end-candidate-fixture/08-02-PLAN.md
  - .planning/phases/08-end-to-end-candidate-fixture/08-02-SUMMARY.md
  - .planning/phases/08-end-to-end-candidate-fixture/08-CONTEXT.md
  - CHANGELOG.md
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/tests/m011_e2e_fixture.rs
covered_digest: "v2:sha256:e5f7a0d0d78ac739fd35aae9d2fbe426c54cae65b9569747778cc72b6e932252"
behavior_unverified: 0
overrides_applied: 0
decision_coverage:
  honored: 7
  total: 7
  not_honored: []
---

# Phase 8: End-to-end candidate fixture Verification Report

**Phase Goal:** A developer can run one end-to-end fixture that distinguishes preserved history, current evidence, and allowed action at named cuts, with incremental maintenance and clean reconstruction agreeing

**Verified:** 2026-09-30T04:31:46Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | ------- | ---------- | -------------- |
| 1 | Independent supports and last-support withdrawal hold at named (TT, VT) cuts; the conflict variant preserves both sides and reports conflict instead of superseding (FIX-01 / ROADMAP SC1) | ✓ VERIFIED | Named test `e2e_fixture_supports_and_conflict_at_named_cuts` passed. At (t1,2017) two live supports, eligibility true, `conflict_report_at` positive-only. After t2 CorrectInterval `not-P` on `a`, `live_at` still has `(b, relatedTo, P)` and `(a, relatedTo, not-P)`; both report sides non-empty; residuals at VT 2012/2021 keep `(a, relatedTo, P)`. After t3 Retract `b`, `positive_supports` empty; `claim_supported_at` for Q still true; `replay_check` Ok. `ConflictReport` has only `positive_supports` / `negative_supports` — no winner enum |
| 2 | Summary and action records cite exact source revisions and rule version; a changed dependency forces re-evaluation; a stale cached output cannot renew its own admission (FIX-02 / ROADMAP SC2) | ✓ VERIFIED | Named test `e2e_justification_cites_sources_and_rejects_stale_admission` passed. t1 row cites `source_fact_seqs` of `a`/`b` and `rule_version` `r1`; `check_admission` Ok; persist, delete `snapshot.json`, open still Ok. After t2 original id is `AdmissionDenied { reason: "stale_support" }`; a new row after reevaluation admits; after t3 both old ids still deny; unknown id is `unknown_justification`. Production path uses `Fact::is_live_at(u64::MAX, row.vt)` so CorrectInterval/Retract can stale a t1 row (historical `row.tt` would never stale). `ineligible` / `rule_version` deny arms exist in `check_admission` and are not the named-oracle proof — see Test Quality |
| 3 | Incremental maintenance and clean reconstruction agree on values, active supports, and completeness; discarding CSR and snapshots changes no answer (FIX-03 / ROADMAP SC3) | ✓ VERIFIED | Named test `e2e_incremental_matches_reconstruct_after_discarding_leases` passed. `GraphFold::replay(log)` fingerprint equals incremental; named cuts including residual VT 2012/2021 match `live_at` / support counts / `conflict_report_at` / `derivation_eligible_at`. persist, delete `snapshot.json`, open: fold fingerprint and justification rows match; stale ids still deny. Drop typed + untyped CSR then rebuild: `neighbors` / `edges_out` equal; fold fingerprint unchanged |
| 4 | One shared fixture builder encodes a/b/P/Q plus t1 Assert pair, t2 CorrectInterval on `a`, t3 Retract of `b`, and the FIX-01 conflict branch with object `not-P`; named tests are exactly the three locked fn names (D-F1, D-F6) | ✓ VERIFIED | `build_through_t1` / `apply_t2_conflict` / `apply_t3_withdraw_b` in `m011_e2e_fixture.rs`. `cargo test --test m011_e2e_fixture -- --list` prints exactly those three tests (0 ignored). Interns `relatedTo` (not `knows`); Behavior `derive_pq` `rule_version` `r1`; VT constants 2010/2015/2020/2012/2017/2021 |
| 5 | Assert/Retract/Correct/CorrectInterval fold arms, `derivation_eligible_at` mix, quantum outcome disposition, typed/untyped CSR `from_fold`, and `replay_check` / `provenance_fingerprint` mix stay unchanged (D-F7) | ✓ VERIFIED | `git diff 0126b32..HEAD` empty for `fold.rs`, `csr.rs`, `snapshot.rs`, `event.rs`. `quantum.rs` hunks are additive (`Justification` / `AdmissionDenied` / buffer / APIs / empty vec in four constructors). `emit` still auto-pushes outcomes only (lines 595–603); `record_justification` is never called from `emit`. `derivation_eligible_at` (disk 708–729) still keys eligibility on live derived + live premise claim. `provenance_fingerprint` mix is still `kutha-prov-v1` + `(id, caused_by, name, rule_version)` — no justification bytes. Comment on `Justification` forbids mixing into that digest |
| 6 | Named tests are registered in the governor: `fsm.yaml` `observe_cargo.required`, check `m011-e2e`, bridge `B-m011-e2e`; `uv run kutha-gov ci` stays at 0 HIGH (GATE-01 / D-F6 / ROADMAP SC4) | ✓ VERIFIED | fsm lines 52–54 exact fn identifiers (S07 provenance names retained). `checks.yaml` id `m011-e2e` needles `fn e2e_…` plus `JUSTIFICATIONS_REL` / `conflict_report_at` / `check_admission`. `B-m011-e2e` cites `m011_e2e_fixture.rs` with `check: m011-e2e`. Verifier re-ran `uv run kutha-gov ci`: exit 0, **0 HIGH, 0 LOW, 32 checks**; `m011-e2e` OK; all three FIX names observed `=ok`. `precommit --check m011-e2e` and `observe-required-fn` OK |
| 7 | This phase executed only while S08 is the Active Slice; `.kutha/STATE.md` unedited; harness ROADMAP S08 stays unchecked (GATE-02 / D-F6 / ROADMAP SC4) | ✓ VERIFIED | `**Active Slice:** S08`; `L_delivery=M011-S07-done` (slice not closed). `git diff --exit-code -- .kutha/STATE.md`. `git log 0126b32..HEAD -- .kutha/STATE.md` empty. `- [ ] **S08:` in `.kutha/ROADMAP.md`. CHANGELOG Process: “Not a closed S08 delivery lease.” Lease H2 for S08 remains a lease sentence, not closed-delivery |
| 8 | Freeze items stay unstarted and honeycomb cells stay Proposed; workspace members only `kutha-common` + `kutha-runtime`; ADR-013/011/012/040 evidence lists the assigned FIX names (GATE-03 / ROADMAP SC5) | ✓ VERIFIED | `Cargo.toml` `members = ["crates/kutha-common", "crates/kutha-runtime"]`; `crates/` has those two only. No `rocksdb`/`cypher`/`hnsw` in crate `Cargo.toml` or crate sources. ci `freeze` OK. honeycomb `map: Proposed` count 31, `map: Accepted` count 0. ADR-011 evidence includes both FIX-01 and FIX-02 names; ADR-012/040 include FIX-03; ADR-013 includes FIX-01. ADR markdown `git diff --exit-code` clean on those four files |
| 9 | Justification rows persist as `justifications.jsonl` (`JUSTIFICATIONS_REL`) after events/outcomes, load on every `store::open` path, missing file is empty vec (cannot admit) (D-F2) | ✓ VERIFIED | `JUSTIFICATIONS_REL = "justifications.jsonl"` (`store.rs` 13). `persist` writes justifications last after outcomes (32–34). `load_justifications` missing path → `Vec::new()` (168–170). `attach_justifications` beside all four `attach_outcomes` returns (snapshot, Define dict, terms file, empty-events default: 69, 77, 85, 91). `Snapshot` has no justification fields. FIX-01/02 tests persist, delete `snapshot.json`, open, rows present. Disk `persist` body includes `write_justifications`; CBM `get_code_snippet` for `persist` is stale (ends at `write_outcomes`) — source Read is the evidence |
| 10 | Wave SUMMARYs carry D-10 four-part Trajectory; green governor is not ADR Accepted and not L_capability (D-F5) | ✓ VERIFIED | 08-01 and 08-02 SUMMARYs list commands `uv run kutha-gov ci` / `explain trajectory`; HIGH 0; explain paraphrase; orthogonality sentence. CHANGELOG Trajectory names Proposed / not four-valued logic |

**Score:** 10/10 truths verified (0 present, behavior-unverified)

### Required Artifacts

| Artifact | Expected | Status | Details |
| -------- | --------- | ------ | ------- |
| `crates/kutha-runtime/src/store.rs` | `JUSTIFICATIONS_REL`; write after outcomes; load empty if missing; attach on all four open returns | ✓ VERIFIED | Substantive + wired; crash-order comment present |
| `crates/kutha-runtime/src/quantum.rs` | `Justification`, `ConflictReport`, `AdmissionDenied`, `record_justification`, `check_admission`, `conflict_report_at` | ✓ VERIFIED | Buffer beside outcomes; four constructors init empty; `fork_at` starts empty (documented) |
| `crates/kutha-runtime/src/lib.rs` | Re-export `Justification` and `ConflictReport` | ✓ VERIFIED | `pub use quantum::{… ConflictReport, Justification, …}` |
| `crates/kutha-runtime/tests/m011_e2e_fixture.rs` | Shared builder + three named FIX oracles | ✓ VERIFIED | 423 lines; three tests, 0 ignored; 3 passed |
| `CHANGELOG.md` | Product sidecar + three fn names; Process GATE-01; Trajectory Proposed | ✓ VERIFIED | 2026-09-30 Product H2; Process `m011-e2e` / `B-m011-e2e`; no closed-delivery S08 sentence |
| `.kutha/dictionaries/fsm.yaml` | Three observe names | ✓ VERIFIED | Exact identifiers under `observe_cargo.required` |
| `.kutha/dictionaries/checks.yaml` | `m011-e2e` file_contains | ✓ VERIFIED | Test fn needles + product-symbol extra steps |
| `.kutha/dictionaries/bridges.yaml` | `B-m011-e2e` | ✓ VERIFIED | cites test file; `check: m011-e2e` |
| `.kutha/dictionaries/honeycomb.yaml` | ADR-013/011/012/040 evidence append; map Proposed | ✓ VERIFIED | All four `map: Proposed` + assigned FIX names |

`gsd_run query verify.artifacts` on 08-01 and 08-02: `all_passed: true` (5/5 and 5/5).

### Key Link Verification

`gsd_run query verify.key-links` returned `verified: false` because PLAN `from:` values are symbols, not file paths. Manual Level-3/4 wiring:

| From | To | Via | Status | Details |
| ---- | -- | --- | ------ | ------- |
| `store::persist` | `justifications.jsonl` | Write after events, WAL, terms, snapshot, outcomes | ✓ WIRED | `store.rs` 30–34 `write_justifications` last |
| `store::open` | Runtime justifications buffer | `load_justifications` + `attach_justifications` on every success return | ✓ WIRED | Four attach sites; open never infers cites from the log |
| `check_admission` | `Fact::is_live_at` on cited `source_fact_seqs` | Fail-closed `stale_support`; claim_id liveness alone is not enough | ✓ WIRED | Loops `source_fact_seqs`; `is_live_at(u64::MAX, row.vt)` (current picture × row VT). Named test after t2 asserts `stale_support` |
| `conflict_report_at` | `GraphFold::live_supports` | Partition live Facts by interned P versus not-P; report only | ✓ WIRED | `quantum.rs` 346–351; CBM `live_supports` (fold.rs 146–151, coverage `metadata_match`) filters `claim_id` + `is_live_at`. No winner field |
| `m011_e2e_fixture.rs` fn names | `fsm.yaml` `observe_cargo.required` | GATE-01 observe-required-fn (08-02) | ✓ WIRED | Names identical; ci observe all three `=ok` |
| `B-m011-e2e` | `m011_e2e_fixture.rs` | `bridges.yaml` `check: m011-e2e` | ✓ WIRED | Bridge `check` equals checks.yaml id |
| `uv run kutha-gov ci` | 08-02-SUMMARY Trajectory | D-G1 / D-10 / D-F5 | ✓ WIRED | SUMMARY has four D-10 parts; verifier re-ran ci HIGH 0 |

**Wiring:** 7/7 connections verified (manual; gsd path-heuristic N/A)

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `justifications.jsonl` | `Justification` rows | `record_justification` → `justification_records()` → `write_justifications` | Yes — serde JSONL of recorded cites; open reloads same rows | ✓ FLOWING |
| `check_admission` | `source_fact_seqs` liveness | `fold.facts()` + `Fact::is_live_at` | Yes — deny after CorrectInterval/Retract in named test | ✓ FLOWING |
| `conflict_report_at` | `positive_supports` / `negative_supports` | `fold.live_supports` partitioned by object TermId | Yes — t1 positive-only; t2 both sides; t3 positive empty | ✓ FLOWING |
| FIX-03 reconstruct | fold fingerprint / reports | `GraphFold::replay` + `store::open` without snapshot | Yes — compared to incremental Runtime, not hardcoded | ✓ FLOWING |
| CSR drop-rebuild | `neighbors` / `edges_out` | `csr_lease_at` / `typed_csr_lease_at` `from_fold` | Yes — rebuild after drop equals pre-drop | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Three named tests exist | `cargo test -p kutha-runtime --offline --test m011_e2e_fixture -- --list` | exactly three `e2e_*` tests, 0 ignored | ✓ PASS |
| FIX-01/02/03 oracles pass | `cargo test -p kutha-runtime --offline --test m011_e2e_fixture` | `3 passed; 0 failed; 0 ignored` | ✓ PASS |
| GATE-01 needles | `uv run kutha-gov precommit --check m011-e2e` | `OK high=0` | ✓ PASS |
| Observe required fn | `uv run kutha-gov precommit --check observe-required-fn` | `OK high=0` | ✓ PASS |
| Freeze unthawed | `uv run kutha-gov precommit --check freeze` | `OK high=0` | ✓ PASS |
| Honeycomb map | `uv run kutha-gov precommit --check honeycomb-map` | `OK high=0` | ✓ PASS |
| Changelog planes / docs-coupling | `precommit --check changelog-planes` / `docs-coupling` | both OK | ✓ PASS |
| Full GATE-01 ci | `uv run kutha-gov ci` | exit 0; `0 HIGH, 0 LOW, 32 checks`; three FIX names `=ok` | ✓ PASS |
| STATE unedited | `git diff --exit-code -- .kutha/STATE.md` | exit 0 | ✓ PASS |

### Probe Execution

No phase-declared `scripts/*/tests/probe-*.sh`. Step 7c skipped.

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| FIX-01 | 08-01 | Independent supports, last-support withdrawal, conflict report not supersession | ✓ SATISFIED | Named FIX-01 test pass |
| FIX-02 | 08-01 | Cites + stale cannot renew admission | ✓ SATISFIED | Named FIX-02 test pass; sidecar persist/open |
| FIX-03 | 08-01 | Incremental matches reconstruct after discarding leases | ✓ SATISFIED | Named FIX-03 test pass |
| GATE-01 | 08-02 truths / ROADMAP SC4 | Named tests in FSM + check + bridge; ci HIGH 0 | ✓ SATISFIED | Dictionaries + live ci |
| GATE-02 | 08-02 | Execute only while S08 Active Slice; STATE unedited | ✓ SATISFIED | STATE S08; no diff; ROADMAP unchecked |
| GATE-03 | 08-02 | Freeze unstarted; honeycomb Proposed | ✓ SATISFIED | freeze OK; 0 Accepted; two workspace members |

No orphaned REQUIREMENTS.md IDs for Phase 8. GATE-01 is not in 08-02 `requirements:` frontmatter but is in ROADMAP SC4 and 08-02 truths — verified.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| `crates/kutha-runtime/src/quantum.rs` | 313–330 | `ineligible` and `rule_version` deny arms have no named e2e assertion | ℹ️ Info | FIX-02 oracle proves `stale_support` + `unknown_justification` + snapshot-survive. Those two extra reasons are implemented and changelog-claimed; not required as the sole stale proof (Pattern 4: t3 may keep `claim_p` via `not-P`) |
| `crates/kutha-runtime/src/quantum.rs` | 305 | `is_live_at(u64::MAX, row.vt)` vs PLAN “row tt vt” | ℹ️ Info | Intentional: historical `row.tt` would never stale after later invalidation. Documented in source and 08-01-SUMMARY. Named FIX-02 test would fail without this |

No `TBD` / `FIXME` / `XXX` in phase-touched crate files. No `#\[ignore\]` on the three oracles.

### Test Quality Audit

| Test File | Linked Req | Active | Skipped | Circular | Assertion Level | Verdict |
|-----------|-----------|--------|---------|----------|-----------------|---------|
| `m011_e2e_fixture.rs` `e2e_fixture_supports_and_conflict_at_named_cuts` | FIX-01 | 1 | 0 | no | Behavioral (supports, both polarities, residuals, last positive empty) | PASS |
| `m011_e2e_fixture.rs` `e2e_justification_cites_sources_and_rejects_stale_admission` | FIX-02 | 1 | 0 | no | Behavioral (cites, persist/open, `stale_support`, reevaluation, unknown id) | PASS |
| `m011_e2e_fixture.rs` `e2e_incremental_matches_reconstruct_after_discarding_leases` | FIX-03 | 1 | 0 | no | Behavioral (replay fingerprint, open-without-snapshot, CSR drop-rebuild) | PASS |

**Disabled tests on requirements:** 0
**Circular patterns detected:** 0 — reconstruct path uses `GraphFold::replay` on the log, not captured SUT output files
**Insufficient assertions:** 0 blockers. Advisory: `ineligible` / `rule_version` deny reasons lack a dedicated named assertion (does not unseat FIX-02)

### CBM evidence plane

- `list_projects`: `kutha-graph` present (`gsd/phase-08-end-to-end-candidate-fixture`, 17834 nodes).
- `check_index_coverage`: `quantum.rs` / `store.rs` / `lib.rs` **metadata_changed**; `m011_e2e_fixture.rs` **not_tracked**; `fold.rs` / `csr.rs` **metadata_match**. Did **not** call `index_repository`.
- `search_graph` for `check_admission` / `record_justification` / `conflict_report_at`: 0 hits (index stale). Disk Grep + Read used.
- `get_code_snippet` `persist`: stale body (no `write_justifications`). Disk `store.rs` 17–35 is authoritative.
- `get_code_snippet` `derivation_eligible_at`: **wrong symbol** (returned `admit_claim`/`emit` at stale lines 515–537). Disk `quantum.rs` 708–729 is authoritative.
- `get_code_snippet` `GraphFold.live_supports`: matches disk (fold.rs 146–151).
- `detect_changes` vs `main`: 826 files (whole v0.02 branch); not used as S08 blast radius. Phase-local evidence is `git diff 0126b32..HEAD` on crate files.
- `trace_path` inbound `persist`: prior tests + tenant; e2e fixture callers missing because the test file is `not_tracked`.

### Decision Coverage

All trackable CONTEXT.md decisions are honored by shipped artifacts. `honored: 7 / total: 7`. Non-blocking gate.

### Human Verification Required

N/A — Infrastructure/foundation phase with no user-facing elements.
All acceptance criteria are verifiable programmatically.

The 08-02 PLAN `<human-check>` restates the named cargo tests, ci HIGH-free, honeycomb Proposed, and STATE S08 — all exercised above. Not a separate UAT item.

### Gaps Summary

**No gaps found.** Phase goal achieved. Ready to proceed.

Harness lease remains **S08** (`L_delivery=M011-S07-done`). This report does not close the S08 lease.

---

## Verification Metadata

**Verification approach:** Goal-backward (ROADMAP success criteria + PLAN must_haves; SUMMARY claims not taken as evidence)
**Must-haves source:** `.planning/ROADMAP.md` Phase 8 SCs + 08-01/08-02 PLAN frontmatter (deduped)
**Automated checks:** e2e suite 3/3 pass; ci 32/32 HIGH 0; precommit m011-e2e / freeze / observe-required-fn / honeycomb-map / changelog-planes / docs-coupling OK
**Human checks required:** 0
**CBM:** project present; coverage stale on new S08 symbols — source Read used; no `index_repository` / `delete_project` / `manage_adr`

---

_Verified: 2026-09-30T04:31:46Z_
_Verifier: Claude (gsd-verifier)_
