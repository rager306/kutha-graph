---
title: Whole-repo audit — verified findings and correction slices
date: 2026-10-07
status: adopted as M020 / GSD v0.14 (phases 63–67)
source: .compound-engineering/artifacts/reviews/2026-10-07-whole-repo-audit/report.md
---

# Verification result (parent re-read, `rg` + CBM, 2026-10-07)

Re-read from source; no tree mutations.

| Finding | Verdict | Note |
|---|---|---|
| P1-1 `live_triples` overwrite + `invalidate_fact_at` | Confirmed mechanism; **severity P2** | `fold.rs:500` inserts per triple (last wins); `fold_query.rs:219` drops the key only when it equals the idx. Effect: duplicate derived head, not data loss. |
| P1-2 `persist_log` truncates in place | Confirmed; stays **P1** | `segment.rs:185` `File::create` on the same `seg-N` names, manifest swapped last (`:610`). Callers `store.rs:237,354`. Crash window untested. Same hazard in `rewrite_segment_with_seal_trailer` (`:302`, test-only helper). |
| CorrectInterval re-derives only patch window | Confirmed, **my Phase 47 code** | `eval.rs:78-96`: `retract_derived` kills all derived for the premise, `horn_fires` runs on `patch_from..patch_to` only. Prefix/suffix residuals (old object) lose derivations. HORN-CORR tests assert retract only. |
| Retract by CorrectInterval event id kills 1 of 3 rows | Confirmed | `fact_index_by_event_id` returns the first live row; residual rows share `event_id`. |
| User `emit(Retract)` un-gated | Confirmed | `op_relation` returns `None` for Retract/Correct*, so `admit` returns `Ok`; only `Correct*` has a `MetaOpRejected` pre-check. |
| Oracle stub `open_retract_reemit…` | Confirmed | `m013_fold_indexes.rs:111` `let _ = (fp, rebuilt)`. |
| Harness silent passes (nameless row, missing cite, bad JSONL line, `min_phase`) | Confirmed (shape read) | `dictionary.py:91-92`, `kinds.py:1480`, `time_log.py:159`, `kinds.py:1759-1761`. Empty-glob heading check not traced to the end. |
| Process: lease commit not alone | Confirmed (813421e, 8 files) | |
| Process: M018 closed without bench | Confirmed | Phase 47 edited `crates/`; `BENCH-controller` is `gate: controller`, `oracle: []`, no run recorded. |
| Eager/Lazy order (#4) | Refuted by audit; not re-run | |
| Not re-verified | `admission.rs:239-247` keyed retry after retract, `kutha:admission` key mismatch, rollover window `persist_append.rs`, `fsm.py` jobs>1 skip, M019-S07 stale verification, pytest writing live `.kutha/` | Taken from the report; confirm inside the slice. |

New, found here: `cargo fmt --all -- --check` fails on `crates/kutha-runtime/src/eval.rs` (Phase 47 indentation in `retract_derived` / `horn_fires`). No governor check covers fmt.

# Dead code

- Strictly dead: `rust_source.line_at` (`rust_source.py:406`, 0 callers by CBM and `rg`). Delete.
- Pub API with no caller: `AdjacencyIter::at_end`, `Materializer::name` (`rg` finds none). Not dead in the trait-contract sense; keep unless the owner wants pruning.
- CBM zero-degree `ExamineCounter::bump` and `load_dotenv` are false positives (same-name method collapse; `__main__.py:362` calls `load_dotenv`).
- `rewrite_segment_with_seal_trailer`: `#[doc(hidden)] pub`, used only by `m002_seg01.rs`. Test hook, not dead.
- Conclusion: dead code is about 1 function. The audit's weight is on silent-pass paths and missing oracles, not on unused code. No sequence "gluing" is needed in code.

# Is sequence stitching needed

Only in process, not in code. Fixes must go in this order because later ones assume earlier ones:

1. Hygiene (no product risk): `cargo fmt`, delete `line_at`, stale pointers (`.kutha/STATE.md` Next action, ROADMAP/`.planning`), commit or drop the dirty `ce-skills-ru.mdc`.
2. Harness fail-closed (one slice): nameless dictionary row = load error, missing cite file = finding, `fold_log` skips/flags bad line, `min_phase` dirs without `NN-` = finding, empty-glob = finding. Each needs a RED selftest mutation first.
3. Product P1-2 (own slice, bench gate): write to `*.tmp` + fsync + rename, then manifest last; failpoint test across the crash window.
4. Product Horn/fold slice (extends M018-S03): CorrectInterval residual re-derive (prefix/suffix with old object), co-live-safe `live_triples` (count or multimap), Retract admit gate for `kutha:*`, replace the oracle stub with a real fingerprint assert, add pins for each. Bench gate required.
5. Process contract: lease commit alone vs STATE+docs (decide: amend the rule or split the commit), record bench at product close (`BENCH-controller` run), re-verify after post-verify edits.

# Proposed leases (owner decides; nothing started)

| Slice | Plane | Scope | Gate |
|---|---|---|---|
| A hygiene | harness/docs | step 1 | `precommit` |
| B harness fail-closed | harness | step 2 | `selftest` + `ci` |
| C persist atomic | product | step 3 | `cargo test`, bench ≤10% |
| D Horn/fold | product | step 4 | `cargo test`, `ci`, bench ≤10% |
| E process | docs | step 5 | `ci` |

Honeycomb ADRs stay Proposed. ADR edits need `honeycomb.yaml` in the same commit. CHANGELOG with plane-bearing paths.
