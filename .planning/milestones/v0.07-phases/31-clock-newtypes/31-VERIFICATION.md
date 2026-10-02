---
independent: true
verifier_model: composer-2.5
status: passed
score: 6/6
exit: pass
commands:
  - cmd: "cargo test -p kutha-common --doc ValidTime"; exit: 0
  - cmd: "cargo test -p kutha-runtime --offline --test ff5_legal_pit --test ff6_allowlist"; exit: 0
  - cmd: "cargo test -p kutha-runtime --offline --test m010_semantic_open --test m011_typed_csr --test m011_e2e_fixture --test m012_admission_facts --test m012a_hot_indexes --test m012a_time_scale --test m013_write_path --test m013_dual_mode"; exit: 0
  - cmd: "cargo test --workspace --offline"; exit: 0
  - cmd: "git diff HEAD~8 -- crates/kutha-runtime/tests/ff5_legal_pit.rs crates/kutha-runtime/tests/ff6_allowlist.rs"; exit: 0
  - cmd: "git diff HEAD~8 -- crates/kutha-runtime/tests/m012a_time_scale.rs | rg 'ValidTime::from|assert_eq!'"; exit: 0
  - cmd: "git diff --exit-code -- .kutha/STATE.md"; exit: 0
read_only_inputs:
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/phases/31-clock-newtypes/31-CONTEXT.md
  - crates/kutha-common/src/event.rs
  - crates/kutha-common/src/lib.rs
  - crates/kutha-runtime/src/fold_query.rs
  - crates/kutha-runtime/src/quantum.rs
  - crates/kutha-runtime/src/csr.rs
  - crates/kutha-runtime/tests/m012a_time_scale.rs
  - .kutha/STATE.md
---

# Phase 31 — independent verification

**ROADMAP SC:** (1) `Vt`/`Tt`/wall distinct newtypes. (2) compile_fail mix rejected. (3) FF5–M013 green; renames only.

| REQ | Status | Evidence |
|-----|--------|----------|
| CLK-01 | satisfied | `event.rs` `define_clock!` → `ValidTime`/`TransactionTime`/`WallTime` tuple structs; `Vt`/`Tt` aliases; no cross-clock `From`; query cuts `impl Into` then `as_u64`; storage `Op`/`Event`/`Fact` stay `u64` |
| CLK-02 | satisfied | rustdoc `compile_fail` on `ValidTime` (mix `TransactionTime`); `cargo test -p kutha-common --doc ValidTime` exit 0 |
| CLK-03 | satisfied | FF5/FF6 diff empty vs HEAD~8; TIME-01 only `ValidTime::from` lets in `m012a_time_scale.rs`; `assert_eq!` bodies unchanged; workspace `--offline` green |

**Score:** 6/6 must-haves. D-04: no `tt_anchor` / `as_known_on`. D-05: verifier ≠ executor; harness STATE untouched during verify. Residual: two raw `u64` args can still swap tt/vt (planned). FIX-01 field wraps → Phase 33.
