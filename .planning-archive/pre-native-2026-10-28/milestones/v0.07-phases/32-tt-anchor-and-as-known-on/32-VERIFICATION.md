---
independent: true
verifier_model: composer-2.5
status: passed
score: 6/6
exit: pass
commands:
  - cmd: "cargo test -p kutha-runtime --offline --test m014_as_known_on"; exit: 0
  - cmd: "cargo test -p kutha-runtime --offline as_known_on"; exit: 0
  - cmd: "cargo test -p kutha-runtime --offline --test ff5_legal_pit --test ff6_allowlist"; exit: 0
  - cmd: "git diff --exit-code -- .kutha/STATE.md"; exit: 0
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/phases/32-tt-anchor-and-as-known-on/32-CONTEXT.md
  - crates/kutha-common/src/event.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/fold_query.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/src/hydrate.rs
  - crates/kutha-runtime/tests/m014_as_known_on.rs
  - .kutha/STATE.md
---

# Phase 32 — independent verification

**ROADMAP SC:** (1) log anchors map `ingested_at`→wall. (2) greatest wall ≤ D. (3) named `as_of` vs `as_known_on`; FF5 green.

| REQ | Status | Evidence |
|-----|--------|----------|
| ANK-01 | satisfied | `Op::TtAnchor { wall }` in `event.rs`; fold `tt_anchors.insert(wall, event.ingested_at)`; `emit_tt_anchor` → log; hydrate `rebuild_tt_anchors`; no Fact minted |
| ANK-02 | satisfied | `fold_query.rs` `tt_anchors.range(..=wall).next_back()` then `live_at(tt, MAX)`; tests `as_known_on_uses_greatest_anchor_at_or_before`, persist rebuild |
| ANK-03 | satisfied | `m014_as_known_on.rs#true_on_d_differs_from_known_on_d`; FF5/FF6 tests exit 0 |

**Kill-tests (32-CONTEXT):** miss → `None` (`as_known_on_miss_without_anchor`); W1<D≤W2 → A1 tt (`as_known_on_uses_greatest_anchor_at_or_before`); divergence oracle green.

**CBM:** `list_projects` kutha-graph indexed; `detect_changes` on anchor paths — inbound impact on `Runtime`/`GraphFold` fold path (truncated budget).

**Score:** 6/6. D-05: verifier ≠ executor; harness STATE untouched. Phase 33 (FIX-01) not started.
