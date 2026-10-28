---
phase: 66-horn-fold-residuals
verified: 2026-10-07T08:45:00Z
independent: true
verifier_model: composer-2.5-fast
status: passed
read_only_inputs:
  - .planning/phases/66-horn-fold-residuals/66-CONTEXT.md
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/fold_query.rs
  - crates/kutha-runtime/src/eval.rs
  - crates/kutha-runtime/src/admission.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-runtime/tests/m019_sem_meta.rs
  - crates/kutha-runtime/tests/m013_write_path.rs
  - crates/kutha-runtime/tests/m013_fold_indexes.rs
  - .planning/phases/66-horn-fold-residuals/bench-pre.txt
  - .planning/phases/66-horn-fold-residuals/bench-post.txt
  - .kutha/STATE.md
commands:
  - { cmd: "set -a && source .env && set +a && unset CARGO_TARGET_DIR && cargo test -p kutha-runtime --test m019_sem_meta horn_correct_interval_prefix_suffix_derived_remain_live -- --exact", exit: 0 }
  - { cmd: "set -a && source .env && set +a && unset CARGO_TARGET_DIR && cargo test -p kutha-runtime --test m013_write_path public_emit_retract_kutha_allow_is_refused -- --exact", exit: 0 }
  - { cmd: "set -a && source .env && set +a && unset CARGO_TARGET_DIR && cargo test -p kutha-runtime --test m013_fold_indexes open_retract_reemit_indexes_match_fingerprint -- --exact", exit: 0 }
  - { cmd: "git diff --exit-code -- .kutha/STATE.md", exit: 0 }
cbm: list_projects kutha-graph; search_graph leftover_halves / has_live_triple / emit_retract_dict; check_index_coverage fold.rs fold_query.rs pipeline.rs m019_sem_meta.rs m013_write_path.rs m013_fold_indexes.rs (metadata_changed — evidence from source read)
---

# Phase 66 Horn/fold residuals — Independent Verification

**Goal (CONTEXT):** CorrectInterval residual re-derive; co-live-safe `live_triples`; user `Retract` of reserved/`kutha:*` → `MetaOpRejected`; real fingerprint oracle for `open_retract_reemit`; bench ≤10% on `ingest_plain.n` / `as_of.n`; honeycomb **Proposed**; implementation commits must not edit `.kutha/STATE.md`.

| Check | Evidence |
|-------|----------|
| HF-02 `live_triples` multimap | `type FactIdxList = SmallVec<[usize; 1]>` (`fold.rs` L13); `live_triples: IngestMap<(TermId, TermId, TermId), FactIdxList>` (L262); index pushes idx (L499–503); `invalidate_fact_at` retains other idxs, removes key when empty (`fold_query.rs` L229–240) |
| HF-02 `has_live_triple` / co-live | `has_live_triple_except` scans list for any non-invalidated idx not in `except` (L203–226); Horn uses `has_live_triple_except` in `eval.rs` |
| HF-01 CorrectInterval Horn | Fold splits via `vt_intersect` + `leftover_halves`, prefix/patch/suffix rows (`fold.rs` L739–791); `rule_follow_ons` retracts derived, fires prefix/patch/suffix with `horn_fires_except` + same-batch `except` set, LIFO order fires then retracts (`eval.rs` L79–144) |
| HF-01 pin | `horn_correct_interval_prefix_suffix_derived_remain_live` — cargo test exit 0 |
| HF-03 Retract `kutha:*` gate | User mode: `admit` rejects `Retract` when fact relation is reserved (`admission.rs` L116–125); dictionary path `emit_retract_dict` uses `ReceiptMode::Dictionary` (`pipeline.rs` L300–306) |
| HF-03 pin | `public_emit_retract_kutha_allow_is_refused` — cargo test exit 0 |
| HF-04 fingerprint oracle | `open_retract_reemit_indexes_match_fingerprint` persist-after-mutation; asserts in-memory, post-persist open, and `replay_check` fingerprints (`m013_fold_indexes.rs` L82–125) — cargo test exit 0 |
| HF-05 bench ≤10% | Pre: `ingest_plain.n` 137712385, `as_of.n` 419823 (`.planning/phases/66-horn-fold-residuals/bench-pre.txt`); post: 132950275, 315267 (`.planning/phases/66-horn-fold-residuals/bench-post.txt`); Δ −3.4% / −24.9% (faster; no >10% regression) |
| Lease / STATE | `git diff --exit-code -- .kutha/STATE.md` clean; STATE **M020 S04** leased; fix `ec62ca5` does not touch `.kutha/STATE.md` (lease-only `7e3d753`) |

**Verdict:** PASS — CONTEXT kill-tests satisfied; no PLAN/SUMMARY read.
