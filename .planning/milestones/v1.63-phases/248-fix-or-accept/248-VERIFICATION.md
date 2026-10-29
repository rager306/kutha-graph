---
phase: 248-fix-or-accept
independent: true
status: passed
verifier_model: glm-5.3
commands:
  - cmd: git diff HEAD --stat -- crates/
    exit: 0
  - cmd: cargo test -p kutha-runtime --test ff5_legal_pit
    exit: 0
  - cmd: cargo check -p kutha-runtime --all-targets
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/fold.rs
  - tmp/bench-head-fix-10k.log
  - .kutha/STATE.md
---

# Phase 248 independent verification — M169-S01 as_of fix

Fresh verifier context; the executor did not verify its own edit; the plan and summary artifacts were unread.
Lease (`.kutha/STATE.md`): M169-S01 "as_of regression (phases 247–248): fix or record an explicit tradeoff" — this phase sits inside it; no thaw claimed.

## 1 — `dictionary_rel_flags` is `Vec<bool>` indexed by `TermId`: PASS

`fold.rs:315-319` declares `Vec<bool>` with `#[serde(skip)]`; `fold.rs:535-540` builds `vec![false; strings().len()]` and sets `true` where the string starts with `kutha:`. `TermId = u32` (`kutha-common/src/event.rs:13`) and dictionary ids are dense indices into `to_str` (`intern.rs:16`), so the flag index is the `TermId`.

## 2 — `is_dictionary_rel` is bounds-checked: PASS

`fold.rs:563-568`: `.get(rel as usize).copied().unwrap_or(false` — an unbound id answers `false`, no panic.

## 3 — `bind_reserved` fills it correctly: PASS

`fold.rs:534-540` rebuilds the vec from the live dictionary every call. Sites: `quantum.rs:390` (genesis), `quantum.rs:448/454` (each `kutha:*` intern re-binds, so the vec never lags a new reserved rel), `hydrate.rs:18` via `hydrate_from_log` (reached by `from_snapshot` and `from_dict_and_events`), `fold.rs:1087` (`replay_with_dict`). Flags stay `#[serde(skip)]` like the old `HashSet`, so a deserialized fold still needs the same hydrate bind.

## 4 — dictionary relations stay excluded from `as_of`: PASS

`as_of` → `live_at` → `live_facts_at` → `LiveFactFilter::Product` (`fold_query.rs:146,130,13,239`). `tests/m012_allowlist_facts.rs:41` asserts product `as_of(2017)` is empty on a log holding only genesis `kutha:allows`; dictionary facts stay readable via `LiveFactFilter::Dictionary` (`tests/m013_dictionary_facts.rs`).

## 5 — N=10000 log: `as_of.n 123019 ns`, `bytes_per_fact n/a`: PASS

`tmp/bench-head-fix-10k.log`: `as_of.n 123019 ns`, `bytes_per_fact n/a n/a`, `base_n 10000 1`, `validated yes`, `EXIT=0`. Bench not rerun (by instruction); ~91 µs pre-regression is not re-measured, so this claims the cited point, not a restored baseline.

## 6 — no other semantics changed: PASS

Member set identical: old `HashSet::contains` inserted `i as TermId` for the same `kutha:` strings and answered `false` for ids it never held; the vec answers `true` on the same indices, `false` out of range. `LiveFactFilter::keep` (`fold.rs:236-243`), `as_of`, `known_at`, `live_at`, fingerprint and replay paths untouched. `git diff HEAD -- crates/` is one file, `+11 −6`, all in `fold.rs`; `cargo check --all-targets` has no warnings; `ff5_legal_pit` is 2 passed / 0 failed and its binary is newer than `fold.rs`.

## Note (non-blocking)

`fold.rs:1074` and `tests/m035_replay_dict_bind.rs:2` still say `dictionary_rels` in prose; no code reference remains. Governor green, remote CI, ADR status and capability are separate claims.
