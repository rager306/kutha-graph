---
ce-handoff/v1: true
title: "M160 closed (StoreBlob digest on open)"
date: 2026-10-25
branch: work/main-line
head: b063f20
resume_focus: "Ask the owner before the competitor baseline (docker, pause fd_api/9router). Otherwise lease B6 Cypher AS OF (M003) or Arc C with a fresh RED. Do not bump VERSION unless the owner names the next N.N.NN. Cargo clean after the cycle. Harness moratorium applies."
---

# Handoff — M160 closed

## Closed

- **M160 / GSD v1.54 / Phase 234:** `replay_check` rejects an ordinary `StoreBlob` whose bytes fail `blob_digest_matches` (`UnknownRuleVersion`). The Behavior walk still runs first, so a missing Horn blob stays `ExecutionReplayFailed`. Tests `b8_blob_digest.rs` and `b8_blob_digest_kill.rs`.
- Migrate fixtures no longer open a pre-migrate log whose digest does not match. Pin and cite oracles stayed.
- Release id stays `0.0.01`. No new Op.

## Next

1. Samyama and FalkorDB baseline: docker and the owner's OK to pause `fd_api` / `9router`.
2. Or B6 Cypher AS OF (M003) / Arc C, each with its own lease and a fresh RED.
3. Residuals: dangling lineage after a checkpoint; vintage checkpoint payload; `cause_of_behavior` last-wins; checkpoint operator surface; B1b vacuum; parallel `b1_checkpoint_kill3` WouldBlock.

## Notes

- Do not put `0.0.01` in Cargo.toml. Cargo rejects the leading zero.
- One `target/` directory. `cargo clean` at the end of a cycle.
