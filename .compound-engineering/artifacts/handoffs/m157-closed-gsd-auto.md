---
ce-handoff/v1: true
title: "M157 closed (B3-lite hop workload) — owner decision on the baseline"
date: 2026-10-23
branch: work/main-line
head: pending
resume_focus: "Ask the owner: set up the Samyama/FalkorDB baseline (docker, >=20 GB disk, pause fd_api/9router for runs) or reorder to B6 Cypher AS OF (M003) / Arc C. Then lease the chosen slice with a fresh RED and run it through subagents under the GSD-auto controller protocol. Standing owner orders: do not ask permission to commit; clean temp files and target after each cycle (cargo clean, empty tmp/); economical cargo builds (filtered -p/--test runs, one target dir, no stray worktrees); update docs; look for semantic gaps and glue results into one workflow. Harness moratorium applies."
---

# Handoff — M157 closed

## Closed

- **M157 / GSD v1.51 / Phase 231:** `crates/kutha-runtime/src/query_ir.rs` (`QueryIr` Scan/Hops, `execute`, `prepare` -> `Prepared::run` / `PreparedHops::run_from`, fail-closed `try_hops_named`, `QueryError`), `benches/hop_k.rs` (self-relative band), tests `b5_hop_workload.rs`, kill tests `b5_hop_workload_kill.rs`, `b5_hop_prepared_kill.rs`. Hops = sorted set of end nodes; Scan = sorted bag equal to `as_of`.
- Flow: scout (B3 doable part, glue audit) -> ADR-061/PROJECT hygiene -> lease -> planner -> executor 231-01 -> executor 231-02 (A/B 9 alternating pairs PASS) -> verifier (PASS-WITH-FINDINGS: bench measured n x CSR rebuild) -> fix round (prepare/run split, `m019_dur_a` isolation) -> re-verification PASS.
- Closeout: workspace tests 0, `kutha-gov ci --jobs 1` `run_high=0`.

## Next (owner decision)

1. Baseline environment for B3: Samyama + FalkorDB via docker, 20 GB free disk (root has ~40 GB), OK to pause `fd_api` / `9router` during a run. The B4/B5 kill rule needs it.
2. Or reorder: B6 Cypher AS OF (M003) or Arc C, each with its own lease and fresh RED.
3. Residuals needing a fired RED: StoreBlob digest on open; `horn_behavior_reproducible` fail-open; dangling lineage ids; vintage payload; `cause_of_behavior` last-wins; checkpoint operator surface; B1b vacuum; older non-RAII test scratch dirs.

## Notes

- Bench lessons: alternating A/B, `taskset -c 0`, median of 9; a foreign `/root/prime-agent` cargo perturbs timings (load up to 19 during `hop_k`), so the recorded `hop_k` band is wide; re-record on a quiet host before gating on it.
- Shared `CARGO_TARGET_DIR` makes BASE/HEAD fingerprints collapse when worktree paths differ only by checkout; build each arm separately and copy binaries.
- rustfmt follows `mod` declarations: format only files without `mod` children.
