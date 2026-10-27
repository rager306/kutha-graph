---
ce-handoff/v1: true
title: "M165 closed (compacted Horn cause)"
date: 2026-10-27
branch: work/main-line
head: 0dd053c
resume_focus: "M165 is closed. Next product lease needs a fresh RED. Named residuals without one: checkpoint operator surface, B1b vacuum, parallel b1_checkpoint_kill3 WouldBlock. The remote baseline sample is not a B3 gate. B6 Cypher (M003) and HNSW (M005) stay frozen. Do not bump VERSION unless the owner names the next N.N.NN."
---

# Handoff — M165 closed

## Closed

- **M165 / GSD v1.59 / Phase 239:** `replay_check` requires each compacted Horn behavior to have a `search_all` hit that covers the head window and whose `premise_ids` contain `caused_by`. A missing blob or no covering hit is `ExecutionReplayFailed`. A covering hit without that cause is `BrokenLineage`. Log-row `horn_behavior_reproducible` is unchanged. Tests are in `crates/kutha-runtime/tests/b11_compacted_cause.rs`.
- Close commit `0dd053c`. Product fix `ca8289a`. Kill-tests `af307b6`. Release id stays `0.0.01`.

## Also closed this sitting

- **M164 / GSD v1.58 / Phase 238:** checkpoint harvest copies behaviors only from a blob `Op::Checkpoint` cites. Close `db48cf2`.
- **M163 / GSD v1.57 / Phase 237:** a checkpoint blob without `behaviors_exact: true` fails the receipt check. Close `194a84d`.

## Baseline sample

`.compound-engineering/artifacts/research/2026-10-26-remote-baseline-sample.md` records one sitting on `bkk-edge01` (`43.152.237.209`, user `aleksei`). FalkorDB and Samyama ran there. Samyama OOM'd at the n=256 edge build. The numbers are not a regression gate.

## Next

Residuals that still need their own RED: checkpoint operator surface; B1b vacuum; parallel `b1_checkpoint_kill3` WouldBlock.

## Notes

- Remote checkout is machine-local: `/home/aleksei/kutha-graph` at `c21a9b3`, Rust 1.99.1, Docker installed. Containers were stopped. Images remain.
- Do not put `0.0.01` in Cargo.toml. One `target/` directory. `cargo clean` at the end of a cycle.
