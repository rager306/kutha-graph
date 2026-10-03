---
independent: true
verifier_model: cursor-grok-4.6-xhigh
status: passed
score: 6/6
exit: pass
commands:
  - cmd: "cargo test --offline -p kutha-runtime --test m017_verify_killtests -- --test-threads=1"
    exit: 0
  - cmd: "cargo test --offline -p kutha-runtime --test ff5_legal_pit"
    exit: 0
  - cmd: "cargo test --offline -p kutha-runtime --test ff6_allowlist"
    exit: 0
  - cmd: "git diff --stat 428f3ff..HEAD -- Cargo.toml Cargo.lock crates/*/Cargo.toml"
    exit: 0
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/src/store.rs
  - crates/kutha-runtime/src/wal.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/fold_query.rs
  - crates/kutha-runtime/src/provenance.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/src/hydrate.rs
  - docs/ADR/ADR-010-event-log-runtime-quantum.md
  - crates/kutha-runtime/tests/m012a_verify_persist.rs
  - crates/kutha-runtime/tests/m015_known_on.rs
  - crates/kutha-runtime/tests/ff5_legal_pit.rs
---

| REQ | status | evidence |
|-----|--------|----------|
| REPL-01 | pass | kill `a_*`: snap wall/position → `open`/`replay_check` Err; jsonl+snap wall vs intact WAL Err |
| REPL-02 | pass | `abort_*` dest byte-identical; WAL 0/8/12/len-3 → full jsonl (not shorter); id disagree Err |
| REPL-03 | pass | spurious outcomes/justifications sidecar → Err; absent outcomes → Ok |
| REPL-04 | pass | persist eager, open under `lazy` env: `index_mode` Eager; `as_of`/`as_known_on`/fingerprint match |
| KNOWN-03 | pass | rustdoc TT-only; `as_known_on` ≠ `live_at(tt, MAX)` closed VT; `known_at` = `is_known_at` brute |
| REG+FP | pass | FF5/FF6 ok; Cargo.toml/lock empty vs 428f3ff; `h_*` facts+fingerprint ignore TtAnchor |

## Residual

CBM stale (`metadata_changed` on store/wal/fold/provenance/quantum/hydrate; kill-test `not_tracked`). GitNexus 9 commits behind. M010–M016 binaries not re-run (controller).

Mutation (h): `h_tt_anchor_does_not_enter_facts_fingerprint` fails if TtAnchor minted facts or fingerprint hashed `tt_anchors`. Changing only `op_affects_fold` (graph_len/log_offset) would not fail it: fingerprint still walks `facts`; `apply` does not `push_fact` for TtAnchor.
