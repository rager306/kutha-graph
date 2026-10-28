---
phase: 07-provenance-and-rule-version-check
verified: 2026-09-30T03:00:26Z
status: passed
score: 12/12 must-haves verified
covered_files:
  - .kutha/dictionaries/bridges.yaml
  - .kutha/dictionaries/checks.yaml
  - .kutha/dictionaries/fsm.yaml
  - .kutha/dictionaries/honeycomb.yaml
  - .planning/phases/07-provenance-and-rule-version-check/07-01-PLAN.md
  - .planning/phases/07-provenance-and-rule-version-check/07-01-SUMMARY.md
  - .planning/phases/07-provenance-and-rule-version-check/07-02-PLAN.md
  - .planning/phases/07-provenance-and-rule-version-check/07-02-SUMMARY.md
  - .planning/phases/07-provenance-and-rule-version-check/07-CONTEXT.md
  - CHANGELOG.md
  - crates/kutha-common/src/event.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/tests/m011_claim_supports.rs
  - crates/kutha-runtime/tests/m011_provenance.rs
covered_digest: "v2:sha256:6453c4cd4b8086d8ebe6bd814ebe45b041f1d91c829599887e3252c4071cdd35"
behavior_unverified: 0
overrides_applied: 0
decision_coverage:
  honored: 7
  total: 7
  not_honored: []
---

# Phase 7: Provenance and rule-version check Verification Report

**Phase Goal:** A developer can detect a change to only a causal reference or a pinned rule version even when the state fingerprint still matches, without execution replay

**Verified:** 2026-09-30T03:00:26Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | ------- | ---------- | -------------- |
| 1 | Verification detects a change to only a Behavior `caused_by` reference even when the state fingerprint still matches (PROV-01 / ROADMAP SC1) | ✓ VERIFIED | Named test `provenance_detects_caused_by_swap_when_state_fingerprint_matches` passed: clone-then-rebuild, patch only `caused_by` to a second valid prior; `fold().fingerprint()` equal; `provenance_fingerprint()` differs; both `replay_check` Ok |
| 2 | Verification detects a change to only a pinned rule version the same way; execution replay is not required (PROV-02 / ROADMAP SC2) | ✓ VERIFIED | Named test `provenance_detects_rule_version_change_when_state_fingerprint_matches` passed (`r1`→`r2` only). No CA-cache/execution-replay symbols in crate code. Oracles compare digests; they do not rerun behaviors |
| 3 | Named cargo tests are registered in the governor: `fsm.yaml` `observe_cargo.required`, `checks.yaml` `m011-provenance`, `bridges.yaml` `B-m011-provenance` (GATE-01 / D-P6) | ✓ VERIFIED | Exact fn strings in fsm lines 50–51; check needles `fn provenance_detects_…`; bridge cites `m011_provenance.rs` with `check: m011-provenance`. ci observe both names `=ok`; `uv run kutha-gov precommit --check m011-provenance` OK |
| 4 | `uv run kutha-gov ci` stays at 0 HIGH (GATE-01 / D-P5 / D-G2) | ✓ VERIFIED | Verifier re-ran ci: exit 0, `0 HIGH, 0 LOW, 31 checks`; `m011-provenance` OK; both PROV fns observed ok. 07-01 and 07-02 SUMMARYs have D-10 four-part Trajectory |
| 5 | `Op::Behavior` carries `rule_version: String` with serde default; fold Behavior arm still projects subject/relation/object/VT only (D-P2, D-P7) | ✓ VERIFIED | `event.rs` 48–60 `#[serde(default)] rule_version: String`. Optional test `behavior_without_rule_version_field_deserializes` passed (empty string). `fold.rs` Behavior arm uses `..` and does not name lineage fields; `GraphFold::fingerprint` hashes Fact rows only |
| 6 | `Runtime::replay_check` stays the state obligation (fingerprint + exists-earlier `caused_by` / `BrokenLineage`) and does not call the provenance surface (D-P3) | ✓ VERIFIED | Body at `quantum.rs` 497–515: `GraphFold::replay` + fingerprint compare + `seen` set. No `provenance_fingerprint`/`provenance_check` in that function. CBM outbound trace: `fingerprint`/`replay`/`iter`/`as_slice` only. Git diff vs `d347c2d` adds provenance *after* `replay_check`, does not rewrite it |
| 7 | `Runtime::provenance_fingerprint` returns `[u8; 32]` and `provenance_check(expected)` returns `ProvenanceMismatch { expected, actual }` when digests differ (D-P3) | ✓ VERIFIED | Signature `quantum.rs` 521; mix `kutha-prov-v1` + `(event.id, caused_by, name, rule_version)` length-prefixed. `provenance_check` 544–549 returns `RuntimeError::ProvenanceMismatch`. Display `ProvenanceMismatchError`. Named oracles exercise fingerprints; wrapper is a four-line compare |
| 8 | Named tests are exactly `provenance_detects_caused_by_swap_when_state_fingerprint_matches` and `provenance_detects_rule_version_change_when_state_fingerprint_matches` (D-P4) | ✓ VERIFIED | `cargo test --test m011_provenance -- --list` lists those two plus optional serde helper (not a GATE observe name) |
| 9 | Ghost `caused_by` still yields `BrokenLineage` via `replay_rejects_behavior_without_prior_cause` (D-P1 regression) | ✓ VERIFIED | Named test passed `--exact`; still matches `RuntimeError::BrokenLineage` for `EventId::nil()`. Empty `rule_version` on exhaustive constructors |
| 10 | Assert/Retract/Correct/CorrectInterval fold arms, typed/untyped CSR, and quantum outcomes sidecar stay unchanged (D-P7) | ✓ VERIFIED | `git diff --stat d347c2d -- fold.rs csr.rs store.rs receipt.rs materializer.rs leapfrog.rs` empty. Only additive `rule_version: String::new()` on `follow_ons` inverse_knows in `quantum.rs` |
| 11 | Phase executed while S07 is Active Slice; `.kutha/STATE.md` unedited; ROADMAP S07 checkbox stays unchecked (GATE-02 / D-P6) | ✓ VERIFIED | `**Active Slice:** S07`; `L_delivery=M011-S06-done` (slice not closed). `git diff --exit-code -- .kutha/STATE.md`. `git log d347c2d..HEAD -- .kutha/STATE.md` empty. `- [ ] **S07:` in `.kutha/ROADMAP.md` |
| 12 | Freeze items stay unstarted; honeycomb cells stay Proposed; ADR-060/011 evidence lists both oracle names; workspace members only `kutha-common` + `kutha-runtime` (GATE-03) | ✓ VERIFIED | `crates/` has those two only; `Cargo.toml` members line exact; no `rocksdb`/`cypher`/`hnsw` in crate sources; ci `freeze` OK. All honeycomb `map: Proposed` (zero `Accepted`). ADR-060 and ADR-011 evidence include both `provenance_detects_…` names. ADR markdown untouched (`git log` empty on those two files) |

**Score:** 12/12 truths verified (0 present, behavior-unverified)

### Required Artifacts

| Artifact | Expected | Status | Details |
| -------- | --------- | ------ | ------- |
| `crates/kutha-common/src/event.rs` | `Op::Behavior.rule_version` serde default; `digest_bytes` mixes field | ✓ VERIFIED | Substantive + wired; mix immediately after `caused_by.as_bytes()` |
| `crates/kutha-runtime/src/quantum.rs` | `provenance_fingerprint` / `provenance_check` / `ProvenanceMismatch` beside unchanged `replay_check` | ✓ VERIFIED | Additive API; `replay_check` body not rewritten |
| `crates/kutha-runtime/tests/m011_provenance.rs` | Named PROV-01 and PROV-02 oracles | ✓ VERIFIED | Exact fn names; 3 tests pass (2 GATE + serde helper) |
| `crates/kutha-runtime/tests/m011_claim_supports.rs` | Exhaustive Behavior ctors compile with empty `rule_version` | ✓ VERIFIED | Ghost + `derive_pq` emit; ghost oracle still `BrokenLineage` |
| `CHANGELOG.md` | Product provenance API + named tests; Process GATE-01; Trajectory Proposed | ✓ VERIFIED | 2026-09-30 Product H2; Process `m011-provenance` / `B-m011-provenance`; no closed-delivery S07 sentence |
| `.kutha/dictionaries/fsm.yaml` | Two observe names | ✓ VERIFIED | Both identifiers under `observe_cargo.required` |
| `.kutha/dictionaries/checks.yaml` | `m011-provenance` file_contains | ✓ VERIFIED | Test fn needles + extra `rule_version` / `provenance_fingerprint` |
| `.kutha/dictionaries/bridges.yaml` | `B-m011-provenance` | ✓ VERIFIED | cites test file; `check: m011-provenance` |
| `.kutha/dictionaries/honeycomb.yaml` | ADR-060 / ADR-011 evidence append; map Proposed | ✓ VERIFIED | Both cells `map: Proposed` + both oracle names |

`gsd_run query verify.artifacts` on 07-01 and 07-02: `all_passed: true` (5/5 and 5/5).

### Key Link Verification

| From | To | Via | Status | Details |
| ---- | -- | --- | ------ | ------- |
| EventLog Behavior rows | `Runtime::provenance_fingerprint` | Log-order mix of `event.id`, `caused_by`, `name`, `rule_version` | ✓ WIRED | `quantum.rs` 524–538; domain tag `kutha-prov-v1`; length-prefixed strings |
| `Runtime::replay_check` | `GraphFold::fingerprint` | State path plus exists-earlier `caused_by` | ✓ WIRED | `quantum.rs` 497–515; CBM outbound `GraphFold.fingerprint` / `replay`; inbound callers keep Ok meaning |
| cloned `Vec<Event>` | `Runtime::from_dict_and_events` | Stable `Event.id` so Fact.`claim_id` does not move | ✓ WIRED | `m011_provenance.rs` 55–73 patches `op` in cloned slice; does not call `Event::new` for the swap |
| `m011_provenance.rs` fn names | `fsm.yaml` `observe_cargo.required` | Identical strings; 07-02 registration | ✓ WIRED | fsm 50–51; ci evidence both `=ok` |
| `B-m011-provenance` | `m011_provenance.rs` | `bridges.yaml` `check` equals check id | ✓ WIRED | `check: m011-provenance`; cites test path |
| `kutha-gov ci` | SUMMARY Trajectory | D-10 recorded in 07-01 and 07-02 | ✓ WIRED | Both SUMMARYs: commands, HIGH 0, explain paraphrase, not-Accepted sentence |

Note: `gsd_run query verify.key-links` returned false because PLAN `from:` fields are symbols, not file paths — wiring verified manually against source + CBM + ci.

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `provenance_fingerprint` | SHA-256 digest | `EventLog` Behavior rows in iter order | Yes — live log fields, not a stub digest | ✓ FLOWING |
| `GraphFold::fingerprint` | state digest | live Fact rows (`seq`, S/R/O, VT, TT, `claim_id`) | Yes — lineage fields excluded | ✓ FLOWING |
| PROV clone logs | patched `caused_by` / `rule_version` | clone of `rt.log().as_slice()` then `from_dict_and_events` | Yes — same event ids | ✓ FLOWING |
| `replay_check` | rebuilt fold vs stored fold | `GraphFold::replay(self.log.as_slice())` | Yes — not hardcoded Ok | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| PROV-01 caused_by swap | `cargo test -p kutha-runtime --offline --test m011_provenance provenance_detects_caused_by_swap_when_state_fingerprint_matches -- --exact` (covered by full `--test m011_provenance` run) | 3 passed in file; both named oracles ok | ✓ PASS |
| PROV-02 rule_version change | same file: `provenance_detects_rule_version_change_when_state_fingerprint_matches` | ok | ✓ PASS |
| serde default empty pin | `behavior_without_rule_version_field_deserializes` | ok | ✓ PASS |
| Ghost BrokenLineage | `cargo test -p kutha-runtime --offline --test m011_claim_supports replay_rejects_behavior_without_prior_cause -- --exact` | 1 passed | ✓ PASS |
| GATE-01 governor | `uv run kutha-gov ci` | exit 0, HIGH 0, both PROV fns `=ok`, `m011-provenance` OK | ✓ PASS |
| GATE needles | `uv run kutha-gov precommit --check m011-provenance` | `OK m011-provenance high=0 low=0` | ✓ PASS |
| Test existence | `cargo test -p kutha-runtime --offline --test m011_provenance -- --list` | 3 tests listed, 0 ignored | ✓ PASS |

### Probe Execution

| Probe | Command | Result | Status |
| ----- | ------- | ------ | ------ |
| — | — | No phase-declared `scripts/*/tests/probe-*.sh` | SKIP |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| PROV-01 | 07-01, 07-02 | Detect caused_by-only change while state fingerprint matches | ✓ SATISFIED | Named PROV-01 test + GATE-01 registration |
| PROV-02 | 07-01, 07-02 | Detect rule-version-only change; no execution replay | ✓ SATISFIED | Named PROV-02 test; no execution-replay implementation |
| GATE-01 | ROADMAP SC3 (every slice) | Named cargo test in governor; ci HIGH 0 | ✓ SATISFIED | fsm/checks/bridge + verifier ci re-run |

No orphaned Phase 7 requirements: REQUIREMENTS.md maps PROV-01 and PROV-02 to Phase 7; both claimed by both plans. GATE-02/GATE-03 remain Phase 8 rows in REQUIREMENTS.md but are also ROADMAP Phase 7 SC4 and were verified as truths 11–12. REQUIREMENTS.md already checks PROV-01/PROV-02 `[x]` (executor/orchestrator); that checkbox is not extra code work.

### Decision Coverage

All trackable CONTEXT.md decisions are honored by shipped artifacts (7/7 honored, 0 not_honored). Non-blocking gate.

### Test Quality Audit

| Test File | Linked Req | Active | Skipped | Circular | Assertion Level | Verdict |
|-----------|-----------|--------|---------|----------|-----------------|---------|
| `m011_provenance.rs` | PROV-01 | 1 | 0 | 0 | Behavioral (state FP equal, provenance digest moves, both `replay_check` Ok) | OK |
| `m011_provenance.rs` | PROV-02 | 1 | 0 | 0 | Behavioral (same for `r1`→`r2`; no execution replay) | OK |
| `m011_claim_supports.rs` | PROV-01 regression (D-P1) | 1 | 0 | 0 | Value (`BrokenLineage` on ghost) | OK |

**Disabled tests on requirements:** 0
**Circular patterns detected:** 0
**Insufficient assertions:** 0 for PROV-01/PROV-02 (fingerprints + `replay_check`). `provenance_check` error variant is not called by a named test; it is a thin wrapper over fingerprints already asserted. Info, not a requirement gap.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| — | — | No TBD/FIXME/XXX/TODO/stub markers in phase impl/test files | — | None |
| `quantum.rs` | 544 | `provenance_check` unused by tests | ℹ️ Info | Wrapper proven by reading source; oracles use `provenance_fingerprint` directly |

### CBM / freeze notes

- `list_projects`: `kutha-graph` on `gsd/phase-07-provenance-and-rule-version-check` (17834 nodes). Did **not** `index_repository` (project present; coverage stale, not missing).
- `check_index_coverage`: `fold.rs` `metadata_match`; `event.rs` / `quantum.rs` / `m011_claim_supports.rs` / dictionaries `metadata_changed`; `m011_provenance.rs` `not_tracked`. Claims for those paths are from live source + cargo, not from a fresh graph generation (`indexed_at` 2026-09-30T01:45:29Z, before Wave 1).
- `get_code_snippet` `Op`: `rule_version` present. `replay_check` snippet matches disk. `search_graph` for `provenance_fingerprint` as a Method node returned empty (stale index); `search_code` + disk grep located it.
- Freeze: no Rocks/Cypher/HNSW crates or crate-source needles; workspace members unchanged.

### Human Verification Required

N/A — Infrastructure/foundation phase (runtime provenance digest + governor dictionaries). No user-facing UI. Behavior-dependent truths are exercised by named cargo tests that passed. Planner `<human-check>` in 07-02 duplicates automated PROV-01/PROV-02 fixtures plus ci/honeycomb checks already run here, and is absorbed under the infra auto-pass-UAT rule (not a PRESENT_BEHAVIOR_UNVERIFIED gap). ADR Accepted not required; Active Slice remains S07 without STATE edit.

### Gaps Summary

None. Phase goal achieved: a valid-prior `caused_by` swap or a `rule_version` pin change is detected while the state fingerprint still matches; `replay_check` stays the state path; GATE-01 holds; ADR-060/011 remain Proposed; harness lease S07 untouched. Execution replay (ADR-060 obligation 3) stays unimplemented as specified.

---

_Verified: 2026-09-30T03:00:26Z_
_Verifier: Claude (gsd-verifier)_
