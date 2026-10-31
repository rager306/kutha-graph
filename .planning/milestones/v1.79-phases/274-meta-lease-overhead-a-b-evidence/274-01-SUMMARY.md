---
phase: 274-meta-lease-overhead-a-b-evidence
plan: 01
status: complete
requirements: [PERF-274-01]
---

# Summary — meta lease overhead A/B

- Two interleaved same-host A/Bs, 6 samples each, `KUTHA_BENCH_N=10000`, bench source unchanged between arms, separate target dirs, base commit ids recorded.
- Base `4db8770` (v1.77 close) isolates v1.78: `ingest_plain.n` +2.7%, `as_of.n` +5.7%.
- Base `05dfea0` (v1.76 close) isolates v1.77 + v1.78: `ingest_plain.n` −4.2%, `as_of.n` +1.7%, **`as_of.2n` +19.0%**, `ingest_keyed.n` +12.1% (corrected 2026-10-31 from sorted samples; four head medians had been computed from unsorted positional samples).
- The `as_of.2n` move reproduces (+16.3% in the other A/B) but has no structural mechanism: `fold.rs`, `leapfrog.rs`, and `fold_query.rs` are unchanged between the bases. The gate metrics (`ingest_plain.n`, `as_of.n`) stay inside 10%; `as_of.2n` and `ingest_keyed.n` are recorded as secondary, unexplained moves.
- Decision: keep the leases for the write path; this bench exercises no meta write, so the benefit is cited from the v1.77 install measurement, not measured here.
- Host noise (load 7.6–9.9) recorded as an `uptime` observation, not as log evidence.
- Gates: `cargo test --workspace` green; `kutha-gov ci --jobs 1` exit 0.

## Falsification follow-up (one binary hash, toggle-only, 10+10 interleaved samples)

- `ingest_plain.n` +11.4%, `as_of.n` −4.9%, `as_of.2n` **−2.2%**.
- Corrects the first read: the `as_of.2n` moves in the A/Bs were host-load drift between interleaved runs, not a lease cost — the read path is unchanged between the bases.
- The toggle patch was not preserved: the tree has no such env var and the probe target directory is gone, so this is recorded evidence, not a reproducible procedure.
- The real lease cost is ingest-only and small; accepted.
- Host spread is ±20% run-to-run, wider than every effect measured here.
