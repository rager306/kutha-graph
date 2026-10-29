---
phase: 244-pit-probe
independent: true
status: passed
verifier_model: glm-5.3
commands:
  - cmd: cargo test -p kutha-runtime --test m167_ecfr_pit
    exit: 0
  - cmd: git diff -- crates/kutha-common/src/event.rs
    exit: 0
  - cmd: git diff --stat HEAD -- crates/
    exit: 0
  - cmd: rg -n 'pub fn (as_of|as_known_on|norm_view)' crates/kutha-runtime/src/quantum.rs
    exit: 0
  - cmd: python sha256 over the 9 crates/kutha-runtime/tests/fixtures/ecfr/*.txt
    exit: 0
  - cmd: mtime of target/debug/deps/m167_ecfr_pit-* vs crates/kutha-runtime/tests/m167_ecfr_pit.rs
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/tests/m167_ecfr_pit.rs
  - crates/kutha-runtime/tests/fixtures/ecfr/README.md
  - crates/kutha-common/src/event.rs
  - .kutha/STATE.md
---

# Phase 244 verification — eCFR PIT probe (M167-S01)

Independent verifier context. PLAN and SUMMARY were not read. Evidence: the probe test,
the 9 fixtures, the runtime query functions it calls, the lease, and executed commands.

## Checked

| Claim | Result |
|-------|---------|
| `as_of` exact at every edition boundary (7 cuts) | Pass, all 3 sections: 20151217 → none, 20151218 → ed0, 20170122 → ed0, 20170123 → ed1, 20170313 → ed1, 20170314 → ed2, 20180101 → ed2. 20170313 vs 20170314 is the discriminating pair (texts differ) |
| Half-open VT convention | Pinned: at the boundary date the next edition answers, so `valid_to` is exclusive; an inclusive convention would return ed0 at 20170123 |
| Fixture equal/different facts | sha256: editions 0 and 1 byte-identical, edition 2 differs, for 304.1 / 304.2 / 304.10 |
| `as_known_on` is TT-only, no future edition | `knowledge_tt_at(wall)` → `known_at(tt)`, no ValidTime cut. 2017-03 text absent at walls 20151218 and 20170123, present at 20170314 |
| `norm_view` is force ∩ knowledge | `knowledge_tt_at(wall)` then `live_facts_at(tt, vt)`. Early pair (force 20151218, wall 20151218) returns ed0 exactly; late force (20170314) with early wall does not return ed2 |
| persist + checkpoint + open parity | Opened runtime re-answers the same 7 cuts × 3 sections; both runtimes pass `replay_check` |
| Checkpoint did real work | `checkpoint_sealed_prefix` errors on an empty sealed prefix; the test unwraps it, so a sealed prefix existed (limits 4096 B / 2 records) |
| No new `Op`, no core change | `event.rs` diff empty; `git diff --stat HEAD -- crates/` empty. Probe uses existing `Op::Assert`, `emit_tt_anchor`, `ValidTime`, `WallTime` |
| Run is not stale | Binary 15:09:40 is newer than the test source 15:09:34 and embeds the new `opened ... is_none()` assertion; cargo reported no recompile needed |

## Caveats (do not read as more than proven)

- Editions 0 and 1 are textually identical, so 20170122 vs 20170123 proves liveness and
  attachment, not which row answers; only 20170313 vs 20170314 separates editions.
- 20151217 → none means "no fixture edition in force", not that the regulation was absent;
  and the late-force/early-wall `norm_view` assertion is a negative (`!= ed2`), so it
  proves non-leakage, not which edition is shown.
- Day-scale VT (`YYYYMMDD` as `u64`) is a probe encoding relying on integer ordering; no
  calendar VT type exists, and mixing VT scales in one runtime would break it.
- Fixtures and the test are untracked at verification time; new files stay probe-only.

## Verdict

**PASS.** Boundary exactness is now asserted at every amendment date and the opened
replay matches. `cargo test -p kutha-runtime --test m167_ecfr_pit` exits 0 (1 passed), and
no core `Op` or crate source changed. Governor CI, ADR status, capability not claimed.
