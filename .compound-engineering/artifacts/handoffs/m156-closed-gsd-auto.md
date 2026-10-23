---
ce-handoff/v1: true
title: "M156 closed (fork diff, B2) — Arc B continues"
date: 2026-10-23
branch: work/main-line
head: c8b4dff
resume_focus: "After the user asks to continue: pick the next Arc B slice (B3 needs the owner's baseline environment; B4 HNSW / B6 Cypher AS OF need their own lease with a fresh RED), bundle STATE+ROADMAP+CHANGELOG+README, run through subagents under the GSD-auto controller protocol. Standing owner orders: do not ask permission to commit; clean temp files and target after each cycle; update docs; look for semantic gaps and glue results into one workflow. Harness moratorium applies."
---

# Handoff — M156 closed

## Closed

- **M156 / GSD v1.50 / Phase 230:** `diff_folds(a, b) -> FoldDiff { added, removed, changed }` and `GraphFold::diff` in `crates/kutha-runtime/src/fold_diff.rs` (own module so hot `fold_query.rs` codegen is untouched). Identity is (event id, valid_from, valid_to as `Option`), rank among equal keys by a canonical total order, never `seq`; domain is `facts()` incl. dead rows.
- Flow: lease -> context/plan -> executor (7 tests) -> bench gate tripped on single samples -> alternating A/B (PASS, ratios 0.969 / 1.012) with isolation refactor -> independent verifier (2 HIGH: `None` vs `Some(u64::MAX)` sentinel) -> fix -> gates re-run. Tests: `b4_fork_diff.rs`, `b4_fork_diff_kill.rs`.
- Closeout: workspace tests 0, `kutha-gov ci --jobs 1` `run_high=0`.

## Next

1. Arc B: B3 CSR benchmark vs Samyama/FalkorDB (owner baseline env; pausing `fd_api` / `9router` needs the owner's OK), or B4 HNSW (M005) / B6 Cypher AS OF (M003) with a fresh RED each.
2. Residuals needing a fired RED: StoreBlob digest on open; `horn_behavior_reproducible` fail-open; dangling lineage ids; vintage payload; `cause_of_behavior` last-wins; checkpoint operator surface; B1b vacuum.

## Notes

- Bench lesson: single-sample and non-alternating pairs are order-biased on this host (foreign `/root/prime-agent` cargo). Use alternating A/B, `taskset -c 0`, median of 9.
- rustfmt follows `mod` declarations: format only files without `mod` children.
