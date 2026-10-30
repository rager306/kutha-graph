# Project Milestones: Kutha

## v1.64 ingest regression (Shipped: 2026-10-30)

**Delivered:** M171. The first index commit `199807d` is named; the per-assert map write cost is accepted as the O(1) lookup tradeoff.

**Phases completed:** 249–250 (2 plans)

**Key accomplishments:**
- Five interleaved N=10000 samples: parent `c26bf35` 93.5 ms -> first bad `199807d` 130.4 ms (+39.6%); old `cdf1942` 96.3 ms -> HEAD 114.1 ms (+18.5%).
- Cause: `Runtime::append_indexed` (`by_event_id`, `by_delivery_key`) and GraphFold `fact_by_event` / `live_triples`; per-assert write cost for O(1) lookup.
- `027a2ce` capacity reserve lowers HEAD but does not clear D-06; `KUTHA_FOLD_INDEX_MODE=lazy|thin` is the bounded opt-out.
- No product change, no Op/ADR/CI change; phase 250 accepts the tradeoff.

---

## v1.63 as_of regression (Shipped: 2026-10-29)

**Delivered:** M169. The 3.32× as_of gap is traced to `cec5b64` and reduced to 1.35×.

**Phases completed:** 247–248 (2 plans)

**Key accomplishments:**
- First bad commit `cec5b64` added a per-fact HashSet `is_dictionary_rel` probe to `LiveFactFilter::Product`.
- Dense `dictionary_rel_flags: Vec<bool>` replaces the HashSet; semantics unchanged.
- N=10000 `as_of.n` 303 µs → 123 µs (old 91 µs); residual 1.35× is the accepted product-filter cost.
- `kutha-gov ci` green; cargo observe budget raised to 600 s for cold builds.

---

## v1.62 Performance truth (Shipped: 2026-10-29)

**Delivered:** M168. Interleaved old-vs-HEAD bench and `bytes_per_fact` honesty.

**Phases completed:** 245–246 (2 plans)

**Key accomplishments:**
- Old `cdf1942` reproduces the 2026-10-01 `as_of` (84–99 µs) and `bytes_per_fact` (13.7 KB).
- HEAD medians: ingest +12.6%, as_of **3.32×**, open −3.0%, persist −67.0%, replay +1.1%.
- The as_of gap is a real regression; next step is a bisect, not another benchmark reshuffle.
- `bytes_per_fact` is `n/a` on a zero RSS delta.

---

## v1.61 Legal PIT probe (Shipped: 2026-10-29)

**Delivered:** M167. A public-domain eCFR fixture (3 sections × 3 editions) and a probe proving `as_of`, TT-only `as_known_on`, `norm_view`, and checkpoint/replay parity on real text.

**Phases completed:** 243–244 (2 plans)

**Key accomplishments:**
- 1 CFR 304.1/304.2/304.10 at 2015-12-18, 2017-01-23, 2017-03-14; fetch tool and licence note.
- 7 day-scale cuts; 2017-03-13 vs 2017-03-14 is the discriminating boundary.
- `cargo test --workspace` 730 passed; `kutha-gov ci --jobs 1` `run_high=0`.
- No new Op, no core schema change, no thaw.

---

## v1.60 Operational honesty (Shipped: 2026-10-28)

**Delivered:** M166. A cargo timeout or missing binary no longer retries; GitHub CI installs Rust 1.99.0 and runs pytest plus the governor gate; a fresh same-host benchmark baseline is recorded.

**Phases completed:** 240–242 (3 plans)

**Key accomplishments:**
- `run_cargo_observation` breaks on 124/127; new retries-env regression.
- CI action parse fixed; pytest and governor gates wired.
- Baseline: `ingest_plain.n` 115,733,914 ns; `as_of.n` 334,032 ns; +21.3% / 4.0× vs 2026-10-01; `bytes_per_fact` 0 B is a follow-up RED.

**What's next:** no milestone is leased.

---

## v1.59 Compacted Horn cause (Shipped: 2026-10-27)

**Delivered:** Checkpoint and replay lineage hardening (M154–M165): checkpoint receipt, checkpoint-is-a-cut, fork diff, hop workload, missing Horn blob, StoreBlob digest, shared-head causes, one head per cause, vintage and cited lineage, compacted Horn cause.

**Phases completed:** 228–239 across M154–M165 (12 plans total)

**Git range:** `v1.48..30deaa9`

**What's next:** GSD native reset on 2026-10-28; next milestone unleased.

---

## Pre-native history

GSD milestones v0.01–v1.58 and their phase artifacts are preserved in `.planning-archive/pre-native-2026-10-28/`, on the git branch `archive/pre-gsd-native-2026-10-28`, and in `tmp/backups/pre-gsd-1.19-2026-10-28.tar.gz`. They are not active GSD artifacts after the reset.
