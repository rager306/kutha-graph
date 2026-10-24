---
ce-handoff/v1: true
title: "M158 closed (fork then query) — baseline still needs the owner"
date: 2026-10-24
branch: work/main-line
head: a8c23cd
resume_focus: "Ask the owner before the competitor baseline (docker, pause fd_api/9router). Otherwise the next lease is B6 Cypher AS OF (M003) or Arc C, each with a fresh RED. Do not bump VERSION unless the owner names the next N.N.NN. Harness moratorium applies."
---

# Handoff — M158 closed

## Closed

- **M158 / GSD v1.52 / Phase 232:** `Runtime::diff` is `diff_folds` of the two folds. Tests `b6_fork_query.rs` and `b6_fork_query_kill.rs` run fork, diff, and Scan together. Dead rows stay in the diff and drop out of Scan. A legal checkpoint cut matches. An interior cut stays `ForkCutUnavailable`.
- Release id stays `0.0.01`. No new Op.

## Next

1. Samyama and FalkorDB baseline: docker and the owner's OK to pause `fd_api` / `9router`. The B4/B5 kill rule needs those numbers.
2. Or B6 Cypher AS OF (M003) / Arc C, each with its own lease and a fresh RED.
3. Residuals needing a fired RED: StoreBlob digest on open; Horn blob fail-open; dangling lineage; vintage payload; `cause_of_behavior` last-wins; checkpoint operator surface; B1b vacuum; parallel `b1_checkpoint_kill3` WouldBlock.

## Notes

- Cargo rewrites `Cargo.lock` crate versions from `0.0.0` to `0.0.1` on the next `cargo test`. That sync belongs in git. Do not put `0.0.01` in Cargo.toml.
- rustfmt only on files without `mod` children. One `target/` directory. `cargo clean` at the end of a cycle.
