---
phase: 246-bytes-honesty
independent: true
status: passed
verifier_model: genspark/deep-seek-v4.1-flash
commands:
  - cmd: git diff HEAD --stat -- crates/
    exit: 0
  - cmd: uv run kutha-gov precommit
    exit: 0
  - cmd: grep -rn bytes_per_fact --include=*.rs --include=*.py --include=*.yaml .
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/benches/baseline.rs
  - .kutha/STATE.md
---

# Phase 246 independent verification — bytes honesty

Fresh verifier context; the executor did not verify its own edit.

## Claim 1 — `n/a` on a zero RSS delta or zero live facts: PASS

- Guard is `(Some(before), Some(after)) if after >= before && live > 0`; a zero live count falls through
  to the `_` arm, which writes `bytes_per_fact n/a n/a`.
- Inside the arm, `delta == 0` writes `n/a` with unit `n/a` instead of `0 B`; `delta > 0` keeps numeric
  `delta / live.max(1)` in `B`. Unmeasurable RSS still writes `n/a`.
- The `expected` key list still requires `bytes_per_fact`, so a missing metric still fails the bench.
- Nothing parses the value: outside the bench it appears only in doc comments (`tests/m054_scaling_bench.rs:55`,
  `tests/m019_dur_b.rs:132`) and `test-matrix.yaml` invariant text.

## Claim 2 — fresh log shows `n/a n/a`: PASS

`tmp/bench-head-4.log` (mtime 2026-10-29 20:29, one minute after the bench edit at 20:28) records
`bytes_per_fact n/a n/a`, `validated yes`, `EXIT=0`.

## Claim 3 — recorded values match the log: PASS

| key | log | note |
|-----|-----|------|
| ingest_plain.n | 107315884 ns | matches |
| as_of.n | 224278 ns | matches |
| open.n | 2838457903 ns | 2.84 s |
| persist.n | 2272972968 ns | 2.27 s |
| replay_check.n | 14300653 ns | 14.3 ms |
| bytes_per_fact | n/a | n/a |

## Claim 4 — no product code outside the bench: PASS

`git diff HEAD --stat -- crates/` lists only `crates/kutha-runtime/benches/baseline.rs` (8 insertions,
3 deletions), all inside the `bytes_per_fact` match block. Other modified files are docs and `.planning`.

## Gate

`uv run kutha-gov precommit` exit 0, all checks OK. No bench rerun and no cargo run by the verifier; the
recorded log already shows a clean compile and exit 0.
