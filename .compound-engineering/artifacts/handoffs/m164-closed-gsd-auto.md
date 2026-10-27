---
ce-handoff/v1: true
title: "M164 closed (cited checkpoint lineage)"
date: 2026-10-27
branch: work/main-line
head: db48cf2
resume_focus: "M164 is closed. Next product lease needs a fresh RED. Named residuals without one: checkpoint operator surface, B1b vacuum, parallel b1_checkpoint_kill3 WouldBlock. Dangling log lineage is already BrokenLineage. The remote baseline sample is not a B3 gate. B6 Cypher (M003) and HNSW (M005) stay frozen. Do not bump VERSION unless the owner names the next N.N.NN."
---

# Handoff — M164 closed

## Closed

- **M164 / GSD v1.58 / Phase 238:** `checkpoint_prefix_behaviors` copies `Snapshot.behaviors` only from a `StoreBlob` whose digest an `Op::Checkpoint` in that prefix cites, and only when `behaviors_exact` is true. Prefix `Op::Behavior` rows are still copied. Tests are in `crates/kutha-runtime/tests/b10_cited_lineage.rs`.
- Close commit `db48cf2`. Product fix `844fbbd`. Kill-tests `606d852`. Release id stays `0.0.01`.

## Also closed this sitting

- **M163 / GSD v1.57 / Phase 237:** a checkpoint blob without `behaviors_exact: true` fails the receipt check. Close `194a84d`.

## Baseline sample

`.compound-engineering/artifacts/research/2026-10-26-remote-baseline-sample.md` records one sitting on `bkk-edge01` (`43.152.237.209`, user `aleksei`). FalkorDB and Samyama ran there. Samyama OOM'd at the n=256 edge build. The numbers are not a regression gate.

## Next

Residuals that still need their own RED: checkpoint operator surface; B1b vacuum; parallel `b1_checkpoint_kill3` WouldBlock. Dangling log lineage already returns `BrokenLineage`.

## Notes

- Remote checkout is machine-local: `/home/aleksei/kutha-graph` at `c21a9b3`, Rust 1.99.1, Docker installed. Containers were stopped. Images remain.
- Do not put `0.0.01` in Cargo.toml. One `target/` directory. `cargo clean` at the end of a cycle.
