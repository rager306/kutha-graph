# Project Milestones: Kutha

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
