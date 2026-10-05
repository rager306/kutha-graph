# Phase 62-01 SUMMARY

Start `da655d8`. Through re-verify `96be8a9`, then `419c397` test(62-21) helper/oracles + CHANGELOG; docs(62-22) matrix/SUMMARY. No STATE/ROADMAP. No product `src/` this round.

**MUST FIX:** tamper at `KUTHA_SEEDS=256` seed=79 had no non-commuting swap. Generator mints a dependent Retract when every in-log swap commutes; remaining equal-fold pairs must be disjoint (s,r) (`commutes`, not silently equal). `KUTHA_SEEDS=256` and `512` green.

**Pins:** `export_jsonl` persist-encoded event-id order (middle swap dies). Model/fold triples are `(s,r,o)`. `Fact::is_live_at` vs naive at EDGE_VT (skip `valid_from`; Eager `live_at` via `vt_by_from` would mask it). `GraphFold::replay` stamps vs ingest (Retract-first dies). CRC: fail-closed InvalidData **and** detector pin `CRC mismatch`. Two persist passes per index: legacy, rollover, migrated, export-imported; pass 1 WithHorn on segmented/migrated.

**Coverage (default 64):** Retract 49/64, Correct 47/64, CorrectInterval 38/64, Horn 50/64, VT-inside 128/256 (50%).

**Runtime (isolated `--test-threads=1`):** diff 5.66 + model 0.09 + meta 4.79 = **10.54 s** (≤30; verifier 9.7 s before this round).

**Kills** (scratch `tmp/p62r3-mut-*`, fresh `CARGO_TARGET_DIR`, deleted): 8 claimed + 3 former survivors + export-middle-swap + Assert s/o + skip valid_from + Retract-first replay, all rc=101. No equivalent-mutant residual.

**Matrix:** killed 55 / char 6 / gap 0 / blocked 6 / spec-only 4 / gate 1 (72).

**Open:** C01, HORN-KC-01, ASOF-ELIG-01, DET-02, TR-02, HORN-CORR-01. ADRs Proposed.
