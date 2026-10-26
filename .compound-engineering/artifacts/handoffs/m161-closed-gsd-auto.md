---
ce-handoff/v1: true
title: "M161 closed (every cause of a shared head)"
date: 2026-10-26
branch: work/main-line
head: 66a713d
resume_focus: "Ask the owner before the competitor baseline (docker, pause fd_api/9router). Otherwise lease B6 Cypher AS OF (M003) or Arc C with a fresh RED. Do not bump VERSION unless the owner names the next N.N.NN. Cargo clean after the cycle. Harness moratorium applies."
---

# Handoff — M161 closed

## Closed

- **M161 / GSD v1.55 / Phase 235:** `cause_of_behavior` keeps every distinct `caused_by` for one Behavior EventId. `unindex_behavior` clears `derived_by_cause` under each of them. Tests live in `crates/kutha-runtime/src/quantum_invariants.rs` because the indexes are `pub(crate)`.
- Release id stays `0.0.01`. No new Op. Fingerprint unchanged.

## Next

1. Samyama and FalkorDB baseline: docker and the owner's OK to pause `fd_api` / `9router`.
2. Or B6 Cypher AS OF (M003) / Arc C, each with its own lease and a fresh RED.
3. Residuals: dangling lineage after a checkpoint; vintage checkpoint payload; checkpoint operator surface; B1b vacuum; parallel `b1_checkpoint_kill3` WouldBlock. `derived_by_cause` can still push the same head twice for one cause; unindex removes every copy.

## Notes

- Do not put `0.0.01` in Cargo.toml.
- One `target/` directory. `cargo clean` at the end of a cycle.
- The earlier note `m161-shared-cause-uncommitted.md` described the uncommitted tree. It is superseded by this file.
