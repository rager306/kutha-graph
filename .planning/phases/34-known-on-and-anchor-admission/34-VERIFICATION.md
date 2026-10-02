---
independent: true
verifier_model: composer-2.5
status: passed
score: 6/6
exit: pass
covered_files:
  - crates/kutha-runtime/src/admission.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/fold_query.rs
  - crates/kutha-runtime/tests/m015_anchor_admit.rs
  - crates/kutha-runtime/tests/m015_known_on.rs
covered_digest: "v2:sha256:57503866c7d5a53ebd247bd310c0951934d08ec9ac11698e2c5501e762850451"
commands:
  - cmd: "cargo test -p kutha-runtime --offline --test m015_known_on --test m015_anchor_admit --test m014_as_known_on --test ff5_legal_pit"; exit: 0
  - cmd: "rg 'live_at\\(tt, u64::MAX\\)' crates/kutha-runtime/src/fold_query.rs"; exit: 1
  - cmd: "git diff --exit-code -- .kutha/STATE.md crates/kutha-runtime/tests/ff5_legal_pit.rs"; exit: 0
read_only_inputs:
  - .planning/phases/34-known-on-and-anchor-admission/34-CONTEXT.md
  - crates/kutha-runtime/src/fold_query.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/admission.rs
  - crates/kutha-runtime/tests/m015_known_on.rs
  - crates/kutha-runtime/tests/m015_anchor_admit.rs
cbm: "list_projects kutha-graph; detect_changes kutha-runtime (inbound impacted fold_query/admission/quantum + m015 tests)"
---

# Phase 34 — independent verification

**Goal:** KNOWN-01/02 closed VT survives `as_known_on`; ANK-ADM-01/02 monotone unique `TtAnchor` walls at admit.

| Kill-test / guard | Status | Evidence |
|-------------------|--------|----------|
| Closed VT + anchors: `as_known_on` keeps speed fact `as_of` sees | pass | `closed_vt_interval_survives_as_known_on` green |
| Backwards wall → reject; fold/log unchanged | pass | `backwards_tt_anchor_wall_is_rejected` + `log().len()` |
| Duplicate wall → reject | pass | `duplicate_tt_anchor_wall_is_rejected` + `TtAnchorRejected` in `admission.rs` |
| Persist/open agrees | pass | `closed_vt_known_on_persist_open`, `monotone_unique_anchors_persist_open` |
| D-01: no `live_at(tt, u64::MAX)` in `as_known_on` | pass | `as_known_on` → `known_at(tt)` (`fold_query.rs`); rg no match |
| D-02: FF5 `as_of` bodies untouched | pass | `ff5_legal_pit` green; git diff empty vs HEAD |

| REQ | Status | Evidence |
|-----|--------|----------|
| KNOWN-01 | satisfied | `Fact::is_known_at` + `GraphFold::known_at` TT-only product filter |
| KNOWN-02 | satisfied | RED oracle + persist test; FF5 byte-stable |
| ANK-ADM-01 | satisfied | `wall <= max_wall` → `TtAnchorRejected` before append |
| ANK-ADM-02 | satisfied | monotone persist/open + `m014_as_known_on` (4/4) green |

**Score:** 6/6. D-05: verifier ≠ executor; `.kutha/STATE.md` untouched during verify. CBM `detect_changes` shows inbound impact on runtime fold/admit path (expected).
