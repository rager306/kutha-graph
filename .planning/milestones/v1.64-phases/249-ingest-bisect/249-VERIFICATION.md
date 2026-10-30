---
phase: 249-ingest-bisect
independent: true
status: passed
verifier_model: genspark/deep-seek-v4.1-flash
commands:
  - cmd: git status --short
    exit: 0
  - cmd: git diff --stat
    exit: 0
  - cmd: taskset -c 0 env KUTHA_BENCH_N=10000 KUTHA_BENCH_INGEST_ONLY=1 tmp/ingest-only-bins/cdf1942
    exit: 0
  - cmd: taskset -c 0 env KUTHA_BENCH_N=10000 KUTHA_BENCH_INGEST_ONLY=1 tmp/ingest-only-bins/199807d
    exit: 0
read_only_inputs:
  - docs/architecture/benchmark-baseline.md
  - tmp/ingest-only-cdf1942-1.log
  - tmp/ingest-only-c26bf35-1.log
  - tmp/ingest-only-199807d-1.log
  - tmp/ingest-only-929bbc6-1.log
  - .planning/STATE.md
  - crates/kutha-runtime/src/quantum.rs
---

# Phase 249 verification — ingest bisect

Independent verifier #2 (fresh context, read-only) PASSed the five-sample table against all 20 logs and confirmed the first bad commit by the parent/child diff. It reproduced one sample per commit with the prebuilt early-exit binaries and confirmed the timed `fill_plain` source is byte-identical across the four commits.

Four documentation corrections were required and are in `docs/architecture/benchmark-baseline.md`: std-only bench wording, skipped as_of/claim/live_count validation, +21.5% min, and the 929bbc6 setup/027a2ce non-monotonic explanation. The early-exit patch is retained at `tmp/patch_bench.py`.

Verdict: PASS. No ADR, Op, or CI change.
