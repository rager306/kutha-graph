---
ce-handoff/v1: true
title: "M159 closed (missing Horn blob fails closed)"
date: 2026-10-24
branch: work/main-line
head: 668a7b6
resume_focus: "Ask the owner before the competitor baseline (docker, pause fd_api/9router). Otherwise lease B6 Cypher AS OF (M003) or Arc C with a fresh RED. Do not bump VERSION unless the owner names the next N.N.NN. Disk was near full; cargo clean after the cycle. Harness moratorium applies."
---

# Handoff — M159 closed

## Closed

- **M159 / GSD v1.53 / Phase 233:** `horn_behavior_reproducible` returns false when the rule blob is missing. `replay_check` returns `ExecutionReplayFailed`. Tests `b7_horn_blob.rs` and `b7_horn_blob_kill.rs`. Nine older fixtures that omitted the blob now include a named body so their pin/cite oracles still run.
- Release id stays `0.0.01`. No new Op.

## Next

1. Samyama and FalkorDB baseline: docker and the owner's OK to pause `fd_api` / `9router`.
2. Or B6 Cypher AS OF (M003) / Arc C, each with its own lease and a fresh RED.
3. Residuals: StoreBlob digest on open; dangling lineage; vintage checkpoint payload; `cause_of_behavior` last-wins; checkpoint operator surface; B1b vacuum; parallel `b1_checkpoint_kill3` WouldBlock.

## Notes

- `disk-free` minimum is 2 GiB. A full disk fails `kutha-gov ci` before the product tests. Clean `target/` at the end of a cycle.
- Do not put `0.0.01` in Cargo.toml. Cargo rejects the leading zero.
