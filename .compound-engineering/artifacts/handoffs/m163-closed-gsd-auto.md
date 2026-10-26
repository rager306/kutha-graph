---
ce-handoff/v1: true
title: "M163 closed (vintage checkpoint lineage)"
date: 2026-10-26
branch: work/main-line
head: 194a84d
resume_focus: "M163 is closed. Next product lease needs a fresh RED: dangling lineage after a checkpoint, checkpoint operator surface, or B1b vacuum. The remote baseline sample is not a B3 gate. B6 Cypher (M003) and HNSW (M005) stay frozen. Do not bump VERSION unless the owner names the next N.N.NN."
---

# Handoff — M163 closed

## Closed

- **M163 / GSD v1.57 / Phase 237:** a checkpoint `Snapshot` without `behaviors_exact: true` fails the receipt check. Writers set the mark, including a no-Horn checkpoint. Tests are in `crates/kutha-runtime/tests/b9_vintage_checkpoint.rs`.
- Close commit `194a84d`. Verification frontmatter follow-up `f1ef311`. Release id stays `0.0.01`.

## Baseline sample

`.compound-engineering/artifacts/research/2026-10-26-remote-baseline-sample.md` records one sitting on `bkk-edge01` (`43.152.237.209`, user `aleksei`). FalkorDB and Samyama ran there. Samyama OOM'd at the n=256 edge build. The numbers are not a regression gate.

## Next

Residuals that still need their own RED: dangling lineage after a checkpoint; checkpoint operator surface; B1b vacuum; parallel `b1_checkpoint_kill3` WouldBlock.

## Notes

- Remote checkout is machine-local: `/home/aleksei/kutha-graph` at `c21a9b3`, Rust 1.99.1, Docker installed. Containers were stopped. Images remain.
- Do not put `0.0.01` in Cargo.toml. One `target/` directory. `cargo clean` at the end of a cycle.
