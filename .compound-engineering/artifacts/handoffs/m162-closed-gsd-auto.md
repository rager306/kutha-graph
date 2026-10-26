---
ce-handoff/v1: true
title: "M162 closed (one head per cause)"
date: 2026-10-26
branch: work/main-line
head: 0577c7f
resume_focus: "The remote baseline sample is not a B3 gate. Next product lease needs a fresh RED: dangling lineage after checkpoint, vintage checkpoint payload, checkpoint operator surface, or B1b vacuum. B6 Cypher (M003) and HNSW (M005) stay frozen. Do not bump VERSION unless the owner names the next N.N.NN."
---

# Handoff — M162 closed

## Closed

- **M162 / GSD v1.56 / Phase 236:** `derived_by_cause` stores a Behavior head once per `caused_by`. Both `append_indexed` and `index_live_behavior_event` use `record_derived_by_cause`. Tests are in `crates/kutha-runtime/src/quantum_invariants.rs`.
- Release id stays `0.0.01`.

## Baseline sample

`.compound-engineering/artifacts/research/2026-10-26-remote-baseline-sample.md` records one sitting on `bkk-edge01` (`43.152.237.209`, user `aleksei`). FalkorDB and Samyama ran there. Samyama OOM'd at the n=256 edge build. The numbers are not a regression gate.

## Next

Residuals that still need their own RED: dangling lineage after a checkpoint; vintage checkpoint payload; checkpoint operator surface; B1b vacuum; parallel `b1_checkpoint_kill3` WouldBlock.

## Notes

- Remote checkout is machine-local: `/home/aleksei/kutha-graph` at `c21a9b3`, Rust 1.99.1, Docker installed. Containers were stopped. Images remain.
- Do not put `0.0.01` in Cargo.toml. One `target/` directory. `cargo clean` at the end of a cycle.
