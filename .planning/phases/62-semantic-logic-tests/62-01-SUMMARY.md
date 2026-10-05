# Phase 62-01 SUMMARY

Start `da655d8`. Commits: `5f96184` test(62-01) LCG helper; `3285865` test(62-02) model; `d71790c` test(62-03) CSR/fold/persist diff; `b2a0bbb` test(62-04) cascade/meta/Horn char; `2ca6644` docs(62-05) matrix/ADR/CHANGELOG; test(62-06) `assert_seed_budget` for governor rust_test_asserts. No `src/` change. No STATE/ROADMAP edit.

**Oracles:** `SEM-csr-fold-diff` DIFF-01/02 — 64 seeds behavior-free then Horn; live_at/as_of/as_known_on; typed/untyped CSR; persist+open legacy, segmented+rollover (≥2 sealed), migrate, export_jsonl import. `SEM-bitemporal-model` naive list-scan (no fold internals) vs `GraphFold` + FF5. `SEM-cascade-order` Eager/Lazy inverse_knows+alsoKnows name order. Metamorphic: disjoint-subject permute, two fresh replays + `replay_check`, persist layouts, tamper flip/swap/drop (named; never silently equal). `SEM-horn-correct-char` HORN-CORR-01: Correct/CorrectInterval of `knows` leaves derived inverse live, does not mint replacement inverse.

**Seeds:** `BASE_SEEDS[64]`; `KUTHA_SEEDS` may raise (headers + matrix). Persist/tamper use first `min(n,12)`. Failures print seed, case, prefix-shrunk ops. Compare `(s,r,o,stamps)` not EventIds. Default added runtime ~24 s (diff 12.6 + meta 10.9 + model 0.2).

**Kills** (scratch `tmp/p62-*`, fresh `CARGO_TARGET_DIR`, then deleted): CSR `relation=0` → typed CSR ≠ live_at rc=101. `is_live_at` skip tt → model mid-tt rc=101. Lazy reverse follow-ons → Behavior name order rc=101. `push_fact` global LWW → order-independence rc=101. `GraphFold::replay` skip Correct → `replay_check` rc=101. `encoded_log` pop last → persist unwrap/stamps rc=101. Assert `valid_from=2010` → tamper flip silently equal rc=101. `rule_follow_ons` fires on Correct → new inverse rc=101.

**Matrix:** before killed 48 / char 5 / gap 3 / blocked 6 / spec-only 4 / gate 1 (67). After killed 55 / char 6 / gap 0 / blocked 6 / spec-only 4 / gate 1 (72).

**Graph:** CBM `list_projects` `root-kutha-graph`; coverage src match, new tests `not_tracked`. No `src/` so no GitNexus impact / no bench gate. Did not reindex.

**Gates:** fmt/clippy/workspace tests/rocks/`kutha-gov` py precommit ci selftest/pytest (see executor reply). **Open:** HORN-KC-01, ASOF-ELIG-01, C01, DET-02, TR-02, HORN-CORR-01 (blocked/char). ADRs Proposed. No product bug found.
