# Phase 67: Process contract — Context

**Milestone:** v0.14 / harness M020 S05
**Requirements:** PC-01 lease file set, PC-02 bench record at product close, PC-03 re-verify after VERIFICATION edits, PC-04 close M020
**Depends on:** Phase 66 closed; lease S05 Active
**Run rules:** `docs/process/gsd-auto-contract.md`
**Gathered:** 2026-10-07
**Status:** Ready for planning
**Mode:** Controller + owner (GSD auto; discuss skipped)

## Phase Boundary

Codify process rules from the 2026-10-07 audit; close M020. No product crate behaviour. Honeycomb stays **Proposed**. Do not write `67-VERIFICATION.md` (executor).

## Locked decisions

- **D-01 (PC-01):** Lease commit = `.kutha/STATE.md` + `CHANGELOG.md` only (already in contract; wording tightened).
- **D-02 (PC-02):** Product close that edits `crates/` must record `ingest_plain.n` / `as_of.n` in CHANGELOG.
- **D-03 (PC-03):** Edits to a passed `NN-VERIFICATION.md` require a fresh independent verify before close.
- **D-04 (PC-04):** Close M020: `L_delivery=M020-closed`, S05 done, v0.14 shipped in id map; short milestone audit note. Crate SemVer stays 0.0.0. No GitHub Release / tag unless controller later asks.

## Verifier kill-tests

- `gsd-auto-contract.md` states lease file set, bench CHANGELOG record, re-verify after VERIFICATION edits.
- After close: STATE `L_delivery=M020-closed`, Active Slice None; ROADMAP S05 and M020 closed.
