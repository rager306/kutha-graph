# Benchmark baseline (2026-10-01)

Dated measurement note for M013 S01 / GSD Phase 22. Not an ADR. Workload is `relatedTo` (no `knows` inverse cascade). `fork_at` is not measured.

## Re-run

```text
cargo bench -p kutha-runtime --offline --bench baseline
```

Override size with `KUTHA_BENCH_N` (default `10000`). Fail-closed sabotage: `KUTHA_BENCH_SABOTAGE=drop_one` must exit non-zero.

## Measured run (default N = 10000)

Host date: 2026-10-01. Absolute times are host-local; compare ratios across N, not wall times across machines.

| key | value | unit |
|-----|------:|------|
| as_of.2n | 170438 | ns |
| as_of.4n | 446704 | ns |
| as_of.n | 83195 | ns |
| base_n | 10000 | 1 |
| bytes_per_fact | 13674 | B |
| claim_supported_at.2n | 1473 | ns |
| claim_supported_at.4n | 2184 | ns |
| claim_supported_at.n | 1062 | ns |
| events_per_assert | 3.000200 | 1 |
| ingest_keyed.2n | 3224002282 | ns |
| ingest_keyed.4n | 52277431815 | ns |
| ingest_keyed.n | 810193778 | ns |
| ingest_plain.2n | 147556928 | ns |
| ingest_plain.4n | 313682549 | ns |
| ingest_plain.n | 95383904 | ns |
| open.2n | 5659419861 | ns |
| open.4n | 11536141572 | ns |
| open.n | 2696143193 | ns |
| persist.2n | 11387076025 | ns |
| persist.4n | 22919632591 | ns |
| persist.n | 5859564924 | ns |
| replay_check.2n | 20193373 | ns |
| replay_check.4n | 36947283 | ns |
| replay_check.n | 9610665 | ns |
| retract_late.2n | 457082 | ns |
| retract_late.4n | 842811 | ns |
| retract_late.n | 224999 | ns |

## Ratios (D-04)

ingest_plain 2N/N 1.54705
ingest_plain 4N/N 3.28840
as_of 2N/N 2.04842
as_of 4N/N 5.36924

## Persist / open by size

persist.n 5859564924
persist.2n 11387076025
persist.4n 22919632591
open.n 2696143193
open.2n 5659419861
open.4n 11536141572

## Target N and memory budget (D-06 / BASE-02)

bytes_per_fact 13674
target N 1000000
memory budget 13674000000

Budget is `bytes_per_fact × target N` on Linux VmRSS delta from this run. Independent verification must still approve the pair. No optimization in this phase.
