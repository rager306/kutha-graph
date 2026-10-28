---
phase: 242-benchmark-baseline
independent: true
status: passed
verifier_model: glm-5.3
commands:
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: python offline parse/arithmetic over tmp/bench-m166-clean.log (no bench rerun)
    exit: 0
read_only_inputs:
  - docs/architecture/benchmark-baseline.md
  - tmp/bench-m166-clean.log
  - .kutha/STATE.md
---

# Phase 242 verification — benchmark baseline (M166-S01)

Fresh verifier context. Did not read the plan or summary artifacts. Did not rerun the bench.

## Claim checks

| # | claim in the 2026-10-28 section | evidence | verdict |
|---|--------------------------------|----------|---------|
| 1 | `ingest_plain.n` 115733914 ns | raw log `ingest_plain.n 115733914 ns` | match |
| 2 | `as_of.n` 334032 ns | raw log `as_of.n 334032 ns` | match |
| 3 | `open.n` 2761849414, `persist.n` 2421032562, `replay_check.n` 8983909, `bytes_per_fact` 0 B | same keys in raw log, digit for digit | match |
| 4 | ingest ratios 2N/N 1.611, 4N/N 3.479 | 186502584/115733914 = 1.6115; 402669954/115733914 = 3.4793 | match |
| 5 | as_of ratios 2N/N 1.595, 4N/N 4.320 | 532913/334032 = 1.5954; 1443078/334032 = 4.3202 | match |
| 6 | vs 2026-10-01 table: ingest +21.3%, as_of 4.0x, open +2.4%, persist -58.7%, replay_check -6.5% | 1.2134, 4.015, 1.0244, 0.4132, 0.9348 against that table (same file, lines 30-55); cited 95,383,904 / 83,195 / 13,674 B present there | match |
| 7 | not the B3 FalkorDB/Samyama gate | explicit sentence, plus "not evidence for B3" and "not a D-06 pass/fail claim" | match |
| 8 | `bytes_per_fact` 0 B | recorded as RSS sampling failure; old 13,674 B declared not comparable | recorded honestly |
| 9 | first discarded sample | explained: full harness pytest ran concurrently, inflated `as_of.n` and `ingest_plain.n`, not recorded | explained |
| 10 | run conditions | log: `KUTHA_BENCH_N=10000` implied by `base_n 10000`, `fold_index_mode eager`, `validated yes`, `EXIT=0`; section states no pytest or second cargo process | consistent |

## Observation (non-blocking)

The 0 B `bytes_per_fact` is stated inside the section but is not added to the residual list in
`.kutha/STATE.md` ("Residuals that stay open until their own RED"). The measurement failure is not
hidden; only its tracking lives in prose rather than in the lease.

## Gate

`uv run kutha-gov precommit` exit 0, every row OK, including `trajectory`, `verification-independent`,
`verification-uv-only`, `version-freeze`. Full `ci` is not claimed and was not run for this phase.

## Scope boundary

Numbers are taken as recorded; no bench was rerun and no other file was read for product claims.
This section is a same-host regression reference, not a competitor benchmark.

Verdict: **PASS**.
