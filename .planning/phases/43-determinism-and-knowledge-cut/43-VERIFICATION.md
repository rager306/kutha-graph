---
independent: true
verifier_model: cursor-grok-4.6-xhigh
status: passed
score: 4/4
exit: pass
commands:
  - cmd: "cargo test --offline -p kutha-runtime --test m017_verify43_killtests -- --test-threads=1"
    exit: 0
  - cmd: "cargo test --offline -p kutha-runtime --test m017_determinism --test m017_known_on_horn -- --test-threads=1"
    exit: 0
  - cmd: "cargo test --offline -p kutha-runtime --test ff5_legal_pit -- --test-threads=1"
    exit: 0
  - cmd: "git diff c702d71..HEAD -- Cargo.toml Cargo.lock crates/*/Cargo.toml"
    exit: 0
  - cmd: "git diff c702d71..HEAD -- crates/kutha-runtime/src/eval.rs"
    exit: 0
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - crates/kutha-runtime/src/eval.rs
  - crates/kutha-runtime/src/fold.rs
  - crates/kutha-runtime/src/fold_query.rs
  - crates/kutha-runtime/src/lib.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/src/pipeline.rs
  - crates/kutha-common/src/rule.rs
  - docs/ADR/ADR-013-bitemporal-facts-invalidation.md
  - crates/kutha-runtime/tests/m013_fold_index_mode.rs
  - crates/kutha-runtime/tests/m016_rule_eval.rs
  - crates/kutha-runtime/tests/m011_partial_correction.rs
  - crates/kutha-runtime/tests/m016_known_on_hardening.rs
---

| REQ | status | evidence |
|-----|--------|----------|
| DET-01 | pass | Mix64 seeds `0x43A1…`; replay fp==fold; Eager==Lazy `as_of`/`as_known_on`/`live_at` after persist/open `tmp/p43-vfy-*`; Cargo.toml/lock empty vs `c702d71`; no proptest/bolero |
| KNOWN-04 | pass | ADR-013 Evidence names `known04_horn_fires_when_premise_hidden_on_early_cut`; `eval.rs` diff empty; `horn_fires`→`search_prems(..., u64::MAX)`; `live_horns` `is_live_at(u64::MAX, vt)` |
| D013-1 | pass | Status + D013-1 body byte-identical to `c702d71`; additions only |
| oracles | pass | FF5/FF6; M010–M016 binaries; executor `det01_*` + `known04_*` (unread) |

Kill-tests (all ok): `v43_shuffled_mix_replay_fp_eager_lazy_persist_open`, `v43_horn_two_prem_as_known_on_characterization`, `v43_horn_retract_premise_drops_derived`.

Mutation (no src edit): ignoring `CorrectInterval` in `GraphFold::replay` fails `v43_shuffled_mix_*` (fp) and executor `det01_*`. Horn kill-tests still pass. persist/open views may still match via snapshot.

Residual: CBM stale (`fold.rs`/`quantum.rs`/`ADR-013` metadata_changed; kill-test `not_tracked`). GitNexus 17 behind `c702d71`. Disk used for Phase 42+ src. Phase 42 src diffs expected.
