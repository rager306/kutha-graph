# Benchmark baseline (2026-10-01)

Dated measurement note for M013 S01 / GSD Phase 22. Not an ADR. Workload is `relatedTo` (no `knows` inverse cascade). `fork_at` is not measured.

## Re-run

```text
cargo bench -p kutha-runtime --offline --bench baseline
```

Override size with `KUTHA_BENCH_N` (default `10000`). Fail-closed sabotage: `KUTHA_BENCH_SABOTAGE=drop_one` must exit non-zero.

### Fold index mode (env)

Repo-root `.env` / `.env.example` key **`KUTHA_FOLD_INDEX_MODE`** (loaded by `apply_dotenv` before `Runtime` / `GraphFold` default):

| Value | Mode | Meaning |
|-------|------|---------|
| `eager` (default), unset, other | Eager | Accept write-cost — warm secondary leases on every fact push |
| `lazy` or `thin` | Lazy | Thin write — append facts; rebuild via `ensure_fold_indexes` or scan on cold reads |

API override: `Runtime::with_index_mode` / `set_index_mode`. Bench prints metric `fold_index_mode`. D-06 absolute comparisons use Eager unless otherwise noted. See interim note `.compound-engineering/artifacts/research/notes/2026-10-01-d06-index-lease-interim-decision.md`.

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
## Phase 24 delta (2026-10-01)

After fold ingest indexes (M013 S03).

### Visit oracle (IDX-02)

`green_ingest_visits_constant_in_n` passes: ingest examine count at N=256 ≤ N=64 + 8.

### Absolute wall times vs Phase 22 default-N run (D-06)

Phase 22 reference: ingest_plain.n 95383904 ns, as_of.n 83195 ns.

| run | ingest_plain.n | vs Phase 22 | as_of.n | vs Phase 22 |
|-----|---------------:|------------:|--------:|------------:|
| a | 111500502 | +16.9% | 130742 | +57.2% |
| b | 122563370 | +28.5% | 90177 | +8.4% |

D-06 absolute gate (≤10% regression): **not met** for ingest_plain on either run. as_of is within 10% on run b (noise at ~0.1 ms). Within-run scaling still ~linear (ingest 2N/N≈2.01, 4N/N≈3.82).

Interpretation: write-path index maintenance (fact_by_event, live_triples, by_event_id, by_delivery_key) adds per-assert work; lookup paths are O(1). Controller must accept the write-cost tradeoff or request a follow-up to thin the indexes.

### Hybrid spike (2026-10-01): FxHash + SmallVec + reserve

Inspired by Samyama (`rustc-hash` on hot maps), Pogocache (cheap per-entry storage), and the lease interim note — **not** a Samyama-style full index deferral (ingest still needs live leases for delivery-key / fact lookup):

- `FxHashMap` for ingest index leases (`claim_facts`, `fact_by_event`, `live_triples`, `by_event_id`, `by_delivery_key`)
- `SmallVec<[usize; 1]>` for claim/event posting lists (typical one fact per key)
- Keep `reserve_ingest_indexes` pre-size

Phase 22 reference: ingest_plain.n 95383904 ns.

| run | ingest_plain.n | vs Phase 22 | as_of.n | vs Phase 22 |
|-----|---------------:|------------:|--------:|------------:|
| a | 115105156 | +20.7% | 84829 | +2.0% |
| b | 121900974 | +27.8% | 89577 | +7.7% |

Visit oracle still green (`m013_fold_indexes`). D-06 ≤10% still **not met**. Hybrid lowers hasher/alloc noise but does not remove per-assert multi-map maintenance.

### FoldIndexMode (2026-10-01): Eager + Lazy as config

Controller accepted both policies as a setting (not a forced single path):

| Mode | Env | Write path | Lookup |
|------|-----|------------|--------|
| **Eager** (default) | unset / other | accept write-cost — index on every fact push | O(1) leases |
| **Lazy** | `KUTHA_FOLD_INDEX_MODE=lazy` or `thin` | thin write — append facts only | scan until `ensure_fold_indexes` / indexed op |

Runtime delivery-key / event-id maps stay eager either way. Bench prints `fold_index_mode`. Default D-06 comparisons remain Eager (accept tax); Lazy is for thin-write experiments and later Materializer work.

### Reserve spike (superseded by hybrid; kept for history)

Earlier: `Runtime::reserve_ingest_indexes` + `Vec::with_capacity(1)` alone — still +21…+28% ingest.

| run | ingest_plain.n | vs Phase 22 | as_of.n | vs Phase 22 |
|-----|---------------:|------------:|--------:|------------:|
| a | 115079705 | +20.6% | 92101 | +10.7% |
| b | 122023995 | +27.9% | 87774 | +5.5% |

Still above the 10% ingest gate. Capacity hints remove rehash noise but do not erase per-assert map inserts. Next levers: thinner map shapes, or accept write-cost / soften D-06 to the visit oracle.

regression within 10%: no (ingest); as_of run b yes

## Phase 27 delta (2026-10-02)

One write pipeline + facade (M013 S06 / 27-02). Eager. `KUTHA_BENCH_N=10000`. Host date: 2026-10-02.

Phase 22 reference: ingest_plain.n **95383904** ns, as_of.n **83195** ns.

### First run (STOP)

| key | Phase 27 | vs Phase 22 |
|-----|--------:|------------:|
| ingest_plain.n | 2163469973 | +2168% |
| as_of.n | 1386193 | +1566% |
| ingest_plain.2n / n | 3.34 | (Phase 22 1.55) |
| ingest_plain.4n / n | 13.38 | (Phase 22 3.29) |

D-07 ≤10% gate: **STOP — not met.** Admit called `fold.relation_allowed_at`, which collected every live fact before the allow hit (dictionary SoT from Phase 26). O(live facts) per Assert; N=10000 ingest superlinear vs the Phase 22 HashSet allowlist.

### After `kutha:allows` lease (same day)

Droppable ADR-040 lease `allows_by_name` (delivery_key → fact indices), maintained on every fact push even under Lazy product maps; rebuilt in `rebuild_hot_maps` / `bind_reserved`. `relation_allowed_at` is O(allows). Product `as_of` still excludes `kutha:`. `dictionary_as_of` unchanged. `Op::AllowRelation` not revived. Named D-07 expects unchanged.

| key | after lease | vs Phase 22 | vs STOP |
|-----|--------:|------------:|--------:|
| ingest_plain.n | 120267790 | +26.1% | was +2168% |
| as_of.n | 511233 | +514% | was +1566% |
| ingest_plain.2n / n | 1.69 | (Phase 22 1.55) | |
| ingest_plain.4n / n | 3.37 | (Phase 22 3.29) | |

Ingest is back in the Phase 24 hybrid band (+20–28% vs Phase 22; write-cost accepted there). as_of is a single-shot fold walk (2N/N=1.28, 4N/N=2.93, not 4N-scaled); still above +10% vs Phase 22, 63% below the STOP figure.

Full bench: `tmp/p27-bench-lease.txt` (`fold_index_mode eager`, `events_per_assert` 3.000800, `bytes_per_fact` 16710).

## Phase 30 delta (2026-10-02)

Write-path integrity hardening (M013 S09 / 30-02). Eager. `KUTHA_BENCH_N=10000`. Host date: 2026-10-02.

Phase 27 after-lease reference: ingest_plain.n **120267790** ns, as_of.n **511233** ns. Gate: ≤10% regression vs Phase 27 (not Phase 22).

| key | Phase 30 | vs Phase 27 | vs Phase 27 +10% cap |
|-----|--------:|------------:|---------------------:|
| ingest_plain.n | 126962516 | +5.6% | 132294569 |
| as_of.n | 288999 | −43.5% (faster) | 562356 |

Single run (`tmp/p30-bench.txt`). Workspace tests and facade/dual-mode oracles added; no ingest path change intended in this plan slice.

regression within 10% vs Phase 27: yes
