---
phase: 274-meta-lease-overhead-a-b-evidence
plan: 01
status: complete
requirements: [PERF-274-01]
---

# Summary — meta lease overhead A/B

- Two interleaved same-host A/Bs, 6 samples each, `KUTHA_BENCH_N=10000`, bench source unchanged between arms, separate target dirs, base commit ids recorded.
- Base `4db8770` (v1.77 close) isolates v1.78: `ingest_plain.n` +2.7%, `as_of.n` +5.7%.
- Base `05dfea0` (v1.76 close) isolates v1.77 + v1.78: `ingest_plain.n` −6.2%, `as_of.n` −7.7%, **`as_of.2n` +19.0%**.
- The `as_of.2n` move reproduces (+16.3% in the other A/B), so it is recorded as an **open, unexplained residual** above the 10% gate, not waved away as noise.
- Decision: keep the leases for the write path; this bench exercises no meta write, so the benefit is cited from the v1.77 install measurement, not measured here.
- Host noise (load 7.6–9.9) recorded as an `uptime` observation, not as log evidence.
- Gates: `cargo test --workspace` green; `kutha-gov ci --jobs 1` exit 0.
