# Phase 36 Research: Test hardening T2–T6

**Date:** 2026-10-02. **CBM:** `root-kutha-graph` ready, indexed 2026-10-02T09:06:03Z. `event.rs` / `fold.rs` / `fold_query.rs` / `quantum.rs` freshness `metadata_changed` — source + `rg` arbiter (D-06). GitNexus unused (outside M013). `search_code` missed `JustificationCite` / `TtAnchor` (enum variants).

<user_constraints>
## Locked decisions (CONTEXT, verbatim intent)

- **D-01:** trybuild crate (dev-dep) or in-tree `tests/trybuild` for E0308 clock mix; weaken rustdoc `compile_fail` only if trybuild covers it.
- **D-02:** TH-02 may add a test without changing Op field types to newtypes; assert cite stores distinct tt/vt and a swapped construction fails the oracle.
- **D-03:** TH-04/05 may fix `GraphFold::replay` / hydrate tail to apply `TtAnchor` — correctness, not a new feature.
- **D-04:** No Rule AST (Phase 37+); five clocks; intern/WAL; honeycomb Accepted.
- **D-05:** Verifier ≠ executor. Do not write `36-VERIFICATION.md`.
- **D-06:** CBM graph-first; GitNexus secondary; `rg` arbiter.
</user_constraints>

## Current surface (rg)

| Hole | Evidence |
|------|----------|
| T2 / TH-01 | `event.rs` rustdoc `compile_fail` mixes `TransactionTime`→`ValidTime` only. No `trybuild` in `Cargo.lock`. Doctest can fail for syntax, not E0308. No WallTime mix. |
| T3 / TH-02 | `Op::JustificationCite` `tt`/`vt` are `u64` (`event.rs`). `record_justification` copies Into clocks then stores u64. No named swap oracle. |
| T4 / TH-03 | `m014_as_known_on.rs` persist/open + greatest-anchor. No Eager vs Lazy pair. `fork_at` tests do not assert `as_known_on`. |
| T5 / TH-04 | `snapshot_plus_tail_matches_full_replay` snapshots after 10 Asserts, restores **full** log (empty tail), compares **fingerprint**. `tt_anchors` is `#[serde(skip)]`; fingerprint is facts-only (Phase 32 lock). |
| T6 / TH-05 | `GraphFold::apply` inserts `TtAnchor` (`fold.rs` ~676). `replay` calls `apply` on every event. `op_affects_fold` is false (offset skip for prefix anchors). `from_snapshot` applies non-affecting ops only in tail, then `hydrate_from_log` → `rebuild_tt_anchors`. `replay_check` cannot see anchors (fingerprint). Missing `replayed.as_known_on == live`. |

CBM: `GraphFold.replay` inbound 78 (mostly `replay_check`). `GraphFold.as_known_on` inbound 0 in graph (stale); `rg` finds `m014` / `m015` / invariants.

## Recommendations

1. **TH-01:** `trybuild = "1"` dev-dep on `kutha-common` ([crates.io/crates/trybuild](https://crates.io/crates/trybuild), dtolnay). UI crate mixes Tt and WallTime into `ValidTime`; committed `.stderr` must contain `E0308`. Keep rustdoc `compile_fail`.
2. **TH-02:** Integration test: `record_justification` with `tt != vt`; stored `Justification` matches those fields; a swapped-field clone `assert_ne!` the stored row (no newtypes).
3. **TH-03:** Same seed on `FoldIndexMode::Eager` and `Lazy`; `as_known_on` equal. `fork_at` before later `TtAnchor`; fork vs live `as_known_on` differ; fork persist/open agrees with fork.
4. **TH-04/05:** Named tests: mid-cut `snapshot()` + tail Assert + `TtAnchor`; restored `as_known_on` equals live. `GraphFold::replay(log).as_known_on == fold.as_known_on`. Do **not** put anchors in fingerprint (Phase 32). RED first; production edit only if those oracles fail (D-03). Strengthen `snapshot_plus_tail_matches_full_replay` to non-empty tail + `as_known_on`.
5. **Non-goals:** `op_affects_fold` true for `TtAnchor` is Rule 4 unless RED proves offset bug. No `kutha-gov ci`. No `.kutha/STATE.md`.

**Primary recommendation:** Two plans — clocks+cite tests; then known-on/snapshot/replay oracles with RED-then-GREEN.
