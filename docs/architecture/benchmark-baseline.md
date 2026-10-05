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

## Phase 42 (M017 S02) — ingest regression found, not caused here

Same host, same day (2026-10-03), `KUTHA_BENCH_N=10000`, Eager, single runs of `cargo bench -p kutha-runtime --bench baseline`:

| commit | ingest_plain.n (ns) | as_of.n (ns) |
|--------|--------------------:|-------------:|
| v0.06 tag | 106001520 | 240538 |
| v0.08 tag | 107897439 | 304056 |
| `428f3ff` (post-M016, pre-42) | 300881900 | 298766 |
| HEAD after Phase 42 (two runs) | 303357414 / 280920210 | 322652 / 269763 |

Phase 42 does not move ingest (within run noise of `428f3ff`). Between v0.08 and `428f3ff` ingest roughly tripled (+179%): M016 (v0.09) never re-ran this bench (the contract names phases 24, 27, 30 only). This is an open P1 finding against the 10 percent gate, scheduled as M017 S04 (Phase 44); it is not an M002 input until understood.

## Phase 44 (M017 S04) — Horn premise-relation ingest lease (2026-10-03)

Same host as 44-01, Eager, `KUTHA_BENCH_N=10000`, two runs of `cargo bench -p kutha-runtime --offline --bench baseline`. Culprit was `live_horns` scanning `fold.facts()` twice per extra `relatedTo` (genesis `inverse_knows`). Fix: droppable `horns_by_premise` lease keyed by Horn premise relation.

44-01 same-host v0.08 (`1fee7a7`): ingest_plain.n **115322845** ns, as_of.n **252890** ns. Cap +10% ingest = 126855129 ns. Phase 42 table v0.08 was 107897439 ns (cap 118687182). HEAD-before 44-01 (`4d669ca`) ingest **296189698** ns. `428f3ff` ingest **300881900** ns.

| run | ingest_plain.n | vs 44-01 v0.08 | vs `428f3ff` | as_of.n |
|-----|---------------:|---------------:|-------------:|--------:|
| a | 111283177 | −3.5% | −63.0% | 401528 |
| b | 105460301 | −8.6% | −65.0% | 290851 |

D-06 ≤10% ingest vs same-host v0.08: **met** (both runs under cap; faster than 115.3M). as_of.n stays in Phase 42 host noise (~0.25–0.40 ms). Per-fact ingest ~11.1 / 10.5 µs vs v0.08 ~11.5 µs. No ADR-010/013 edit.

## Phase 50 (M002 S02) — SNAP-01 / fail-closed WAL open (2026-10-03)

Same host, Eager, `KUTHA_BENCH_N=10000`, isolated `CARGO_TARGET_DIR`. Pre-phase `7e31784` vs HEAD after SNAP-01 (sidecar `replace_via_tmp`, `sync_dir`, `check_tt_anchors_range`, fail-closed WAL header). Persist is not the D-06 pair (Phase 52). Two consecutive runs:

| run | 7e31784 ingest_plain.n | HEAD ingest_plain.n | Δ ingest | 7e31784 as_of.n | HEAD as_of.n | Δ as_of |
|-----|-----------------------:|--------------------:|---------:|----------------:|-------------:|--------:|
| a | 88490215 | 109468422 | +23.7% | 227344 | 252921 | +11.3% |
| b | 101718878 | 117933202 | +15.9% | 284070 | 291312 | +2.5% |

Runs a and b above were back-to-back, not interleaved, and the baseline commit itself moved 88.5M to 101.7M between them (about 15% host drift; load average 6+ on 12 cores). They cannot decide the gate. Controller re-measure: six interleaved base/head pairs of the prebuilt release binaries, median of six.

| metric | median 7e31784 | median HEAD | Δ median |
|--------|---------------:|------------:|---------:|
| ingest_plain.n | 116422669 | 116580979 | +0.1% |
| as_of.n | 282387 | 286284 | +1.4% |

D-06 ≤10% vs `7e31784`: **met** on medians (ingest raw range 100.5M–120.9M base, 112.0M–142.5M head). Ingest and `as_of` sources were not edited. `bytes_per_fact` (peak RSS over the whole bench including persist) rose 17218 B to 25200 B in the first pair; this is outside the D-06 pair and is carried as a residual for Phase 52 (persist buffering).

## Phase 51 (M002 S03) — ROCK-00 optional RocksDB spike (2026-10-03)

Same host, Eager, `KUTHA_BENCH_N=10000`. Isolated targets (`tmp/p51-bench-target` for HEAD, `tmp/p51-base-target` for `24e2306`). Default bench binary (feature off). Binaries run from each worktree's `crates/kutha-runtime`. Interleaved `base, head, base, head`. Host noise is about 15–18% between back-to-back runs of the same binary (base ingest 154.8M then 131.0M). Compare interleaved medians; do not treat a single pair as a gate.

| run | 24e2306 ingest_plain.n | HEAD ingest_plain.n | Δ ingest | 24e2306 as_of.n | HEAD as_of.n | Δ as_of |
|-----|-----------------------:|--------------------:|---------:|----------------:|-------------:|--------:|
| 1 | 154822225 | 109161244 | −29.5% | 312963 | 291503 | −6.9% |
| 2 | 131002508 | 111652833 | −14.8% | 367955 | 280913 | −23.7% |

| metric | median 24e2306 | median HEAD | Δ median |
|--------|---------------:|------------:|---------:|
| ingest_plain.n | 142912366 | 110407038 | −22.7% |
| as_of.n | 340459 | 286208 | −15.9% |

D-07 ≤10% vs `24e2306`: **met** (no median regression; HEAD was faster). The apparent speedup is inside host noise (base alone moved ~18%). Default ingest/`as_of` sources were not edited. `bytes_per_fact` stayed 17216–17269 B. RocksDB is not on the default bench compile path.

## Phase 52 (M002 S04) — incremental framed append (2026-10-03)

Same host, Eager, `KUTHA_BENCH_N=10000`. Isolated targets (`tmp/p52-head-target` for HEAD, `tmp/p52-base-target` for `bd5fa4b`). Binaries run from each worktree's `crates/kutha-runtime`. Interleaved `base, head` × 4 pairs. Host noise: base `ingest_plain.n` alone moved 106.5M–158.4M. Compare interleaved medians; do not treat a single pair as a gate.

| run | bd5fa4b ingest_plain.n | HEAD ingest_plain.n | Δ ingest | bd5fa4b as_of.n | HEAD as_of.n | Δ as_of |
|-----|-----------------------:|--------------------:|---------:|----------------:|-------------:|--------:|
| 1 | 109823212 | 111289559 | +1.3% | 284942 | 298546 | +4.8% |
| 2 | 158424111 | 118883628 | −25.0% | 278469 | 298668 | +7.3% |
| 3 | 106469742 | 111237889 | +4.5% | 325617 | 302714 | −7.0% |
| 4 | 129561191 | 112846280 | −12.9% | 409252 | 322081 | −21.3% |

| metric | median bd5fa4b | median HEAD | Δ median |
|--------|---------------:|------------:|---------:|
| ingest_plain.n | 119692202 | 112067920 | −6.4% |
| as_of.n | 305280 | 300691 | −1.5% |

D-10 ≤10% vs `bd5fa4b`: **met** (no median regression). Ingest/`as_of` sources were not edited. New metric `persist_append.n` (after `persist.n`, append 1% more events via `Persister`): HEAD median persist.n 5940191937 ns vs persist_append.n 48907222 ns. Mechanical: `expected` key list gained `persist_append.n`; factor-1 `events_per_assert` now includes that 1% tail.

### Phase 52 follow-up — Define-on-open (2026-10-03)

Same host, Eager, `KUTHA_BENCH_N=10000`. Isolated targets (`tmp/p52-head-target-gap` for HEAD, `tmp/p52-base-target-gap` for `bd5fa4b` in `tmp/p52-base-wt`). Binaries run from each worktree's `crates/kutha-runtime`. Interleaved `base, head` × 4 pairs after the snapshot-open Define intern change. Host noise: pair 1 HEAD ingest 158.1M vs later HEAD 101.7–109.3M. Compare interleaved medians.

| run | bd5fa4b ingest_plain.n | HEAD ingest_plain.n | Δ ingest | bd5fa4b as_of.n | HEAD as_of.n | Δ as_of |
|-----|-----------------------:|--------------------:|---------:|----------------:|-------------:|--------:|
| 1 | 108684580 | 158095140 | +45.5% | 229858 | 267960 | +16.6% |
| 2 | 105347002 | 105975292 | +0.6% | 312712 | 282697 | −9.6% |
| 3 | 114167702 | 109254023 | −4.3% | 280424 | 295943 | +5.5% |
| 4 | 135904907 | 101747410 | −25.1% | 295953 | 285363 | −3.6% |

| metric | median bd5fa4b | median HEAD | Δ median |
|--------|---------------:|------------:|---------:|
| ingest_plain.n | 111426141 | 107614657 | −3.4% |
| as_of.n | 288188 | 284030 | −1.4% |

D-10 ≤10% vs `bd5fa4b` after the open-path fix: **met**. HEAD persist.n median 5957761678 ns vs persist_append.n 48742410 ns.

## Phase 53 (M002 S05) — COMMIT-01 option A (2026-10-04)

Same host, Eager, `KUTHA_BENCH_N=10000`. Isolated targets (`tmp/p53-head-target` for HEAD, `tmp/p53-base-target` for `187d4c7`). Binaries run from each worktree's `crates/kutha-runtime`. Interleaved `base, head` × 4 pairs. Writer path unchanged; `open` gains one linear walk. Compare interleaved medians.

| run | 187d4c7 ingest_plain.n | HEAD ingest_plain.n | Δ ingest | 187d4c7 as_of.n | HEAD as_of.n | Δ as_of |
|-----|-----------------------:|--------------------:|---------:|----------------:|-------------:|--------:|
| 1 | 107339657 | 116227652 | +8.3% | 344151 | 318886 | −7.3% |
| 2 | 112106349 | 115561119 | +3.1% | 295121 | 298308 | +1.1% |
| 3 | 115031987 | 108937665 | −5.3% | 321430 | 277467 | −13.7% |
| 4 | 101286323 | 119271174 | +17.8% | 227234 | 297054 | +30.7% |

| metric | median 187d4c7 | median HEAD | Δ median |
|--------|---------------:|------------:|---------:|
| ingest_plain.n | 109723003 | 115894386 | +5.6% |
| as_of.n | 308276 | 297681 | −3.4% |
| open.n | 2774322253 | 2882886421 | +3.9% |

D-10 ≤10% vs `187d4c7`: **met**. Pair 4 as_of is host noise (base 227k vs 295–344k). Worktrees and target dirs removed after the run.

## Phase 54-01 (M002 S06) — KUTHASEG reader (2026-10-04)

Same host, Eager, `KUTHA_BENCH_N=10000`. Isolated targets (`tmp/p54-head-target` for HEAD, `tmp/p54-base-target` for `3c469fe`). Binaries run from each worktree's `crates/kutha-runtime`. Interleaved `base, head` × 4 pairs. Writer path unchanged; `open` branches only when `manifest.json` is present. Compare interleaved medians.

| run | 3c469fe ingest_plain.n | HEAD ingest_plain.n | Δ ingest | 3c469fe as_of.n | HEAD as_of.n | Δ as_of |
|-----|-----------------------:|--------------------:|---------:|----------------:|-------------:|--------:|
| 1 | 104288907 | 101075166 | −3.1% | 298487 | 276234 | −7.5% |
| 2 | 108934689 | 113534153 | +4.2% | 292855 | 289940 | −1.0% |
| 3 | 108809710 | 105623220 | −2.9% | 231812 | 388745 | +67.7% |
| 4 | 112671069 | 120560577 | +7.0% | 267878 | 288396 | +7.7% |

| metric | median 3c469fe | median HEAD | Δ median |
|--------|---------------:|------------:|---------:|
| ingest_plain.n | 108872200 | 109578686 | +0.6% |
| as_of.n | 280366 | 289168 | +3.1% |
| open.n | 3010978112 | 2853330595 | −5.2% |

Segmented vs legacy `open` of the same ~10k-assert persist (encoded 30025 events, 4 timed opens, HEAD binary): legacy median 3085905705 ns, segmented median 3053446425 ns (−1.1%). D-09 ≤10% vs `3c469fe`: **met**. Pair 3 as_of is host noise (base 232k vs 268–298k). Worktrees and target dirs removed after the run.

## Phase 54-02 (M002 S06) — segmented persist and Persister (2026-10-04)

Same host, Eager, `KUTHA_BENCH_N=10000`. Isolated targets (`tmp/p54-02-head-target` for HEAD `4f8bdc2`, `tmp/p54-02-base-target` for `39af84b`). Binaries run from each worktree's `crates/kutha-runtime`. Interleaved `base, head` × 4 pairs. Empty-dir `persist` is now segmented (HEAD) vs legacy jsonl+WAL (base). Compare interleaved medians.

| run | 39af84b ingest_plain.n | HEAD ingest_plain.n | Δ ingest | 39af84b as_of.n | HEAD as_of.n | Δ as_of |
|-----|-----------------------:|--------------------:|---------:|----------------:|-------------:|--------:|
| 1 | 122549779 | 107042103 | −12.7% | 330287 | 309587 | −6.3% |
| 2 | 123113123 | 99948338 | −18.8% | 288427 | 319134 | +10.6% |
| 3 | 98714729 | 116010969 | +17.5% | 302173 | 298857 | −1.1% |
| 4 | 114485023 | 164994449 | +44.1% | 306843 | 299559 | −2.4% |

| metric | median 39af84b | median HEAD | Δ median |
|--------|---------------:|------------:|---------:|
| ingest_plain.n | 118517401 | 111526536 | −5.9% |
| as_of.n | 304508 | 304573 | +0.0% |
| open.n | 2957055471 | 2726309838 | −7.8% |
| persist.n | 5839573510 | 2372222778 | −59.4% |
| persist_append.n | 51141140 | 14035558 | −72.6% |

D-09 ≤10% vs `39af84b` on ingest/`as_of`/`open.n`: **met**. persist.n is empty-dir segmented vs legacy. persist_append.n dropped (no jsonl). Pair 4 ingest is host noise (HEAD 165M vs later/other HEAD 100–116M). Worktrees and target dirs removed after the run.

## Phase 54-03 (M002 S06) — migrate, export, sealed tamper (2026-10-04)

Same host, Eager, `KUTHA_BENCH_N=10000`. Isolated targets (`tmp/p54-03-head-target` for HEAD `e695d98`, `tmp/p54-03-base-target` for `be57466`). Binaries run from each worktree's `crates/kutha-runtime`. Interleaved `base, head` × 4 pairs. Ingest/`as_of`/`open`/`persist` paths unchanged vs 54-02 (migrate/export are explicit APIs). Compare interleaved medians. `open.n` vs 54-02 HEAD median 2726309838.

| run | be57466 ingest_plain.n | HEAD ingest_plain.n | Δ ingest | be57466 as_of.n | HEAD as_of.n | Δ as_of |
|-----|-----------------------:|--------------------:|---------:|----------------:|-------------:|--------:|
| 1 | 107644870 | 139934250 | +30.0% | 287526 | 356765 | +24.1% |
| 2 | 104112838 | 118099611 | +13.4% | 304898 | 267679 | −12.2% |
| 3 | 109735737 | 103697224 | −5.5% | 289429 | 330405 | +14.2% |
| 4 | 118564645 | 114679176 | −3.3% | 309687 | 315057 | +1.7% |

| metric | median be57466 | median HEAD | Δ median |
|--------|---------------:|------------:|---------:|
| ingest_plain.n | 108690304 | 116389394 | +7.1% |
| as_of.n | 297164 | 322731 | +8.6% |
| open.n | 2759184936 | 2766741940 | +0.3% |
| persist.n | 2346903716 | 2334421877 | −0.5% |
| persist_append.n | 15918808 | 15560351 | −2.3% |

D-09 ≤10% vs `be57466` on ingest/`as_of`: **met**. `open.n` vs 54-02 HEAD +1.5% (within 10%). Pair 1 HEAD ingest 140M vs later HEAD 104–118M is host noise. Worktrees and target dirs removed after the run.

## Phase 54 second round (M002 S06) — cross-segment chain stitch (2026-10-04)

Same host, Eager, `KUTHA_BENCH_N=10000`. Isolated targets (`tmp/p54-r2-head-target` for HEAD `2cc3639`, `tmp/p54-r2-base-target` for `424bda6`). Binaries run from each worktree. Interleaved `base, head` × 2 pairs. Open path gained a 44-byte header prev tip and chain-link checks. Compare interleaved medians (two values: arithmetic mean of the pair).

| run | 424bda6 ingest_plain.n | HEAD ingest_plain.n | Δ ingest | 424bda6 as_of.n | HEAD as_of.n | Δ as_of |
|-----|-----------------------:|--------------------:|---------:|----------------:|-------------:|--------:|
| 1 | 110506624 | 107082382 | −3.1% | 326930 | 290302 | −11.2% |
| 2 | 152350002 | 131048571 | −14.0% | 316209 | 290361 | −8.2% |

| metric | median 424bda6 | median HEAD | Δ median |
|--------|---------------:|------------:|---------:|
| ingest_plain.n | 131428313 | 119065476 | −9.4% |
| as_of.n | 321570 | 290332 | −9.7% |
| open.n | 2606879562 | 2649238906 | +1.6% |

D-09 ≤10% vs `424bda6` on ingest/`as_of`/`open.n`: **met**. Pair 2 base ingest 152M vs pair 1 110M is host noise. Worktrees and target dirs removed after the run.

## Phase 55 (M002 S07) — droppable Rocks locator lease (2026-10-04)

Same host, Eager, `KUTHA_BENCH_N=10000`. Isolated targets (`tmp/p55-head-target` for HEAD `a5b244c`, `tmp/p55-base-target` for `0245bbd`). Binaries run from each worktree's `crates/kutha-runtime`. Interleaved `base, head` × 2 pairs. Default paths unchanged (feature-gated `rocks-index` only). First pair-set ran during a parallel `librocksdb-sys` release compile and was discarded. Numbers below are the idle-host re-run. Compare interleaved medians (two values: arithmetic mean of the pair).

| run | 0245bbd ingest_plain.n | HEAD ingest_plain.n | Δ ingest | 0245bbd as_of.n | HEAD as_of.n | Δ as_of |
|-----|-----------------------:|--------------------:|---------:|----------------:|-------------:|--------:|
| 1 | 143956965 | 114409735 | −20.5% | 314757 | 231110 | −26.6% |
| 2 | 128122974 | 109185233 | −14.8% | 247381 | 282087 | +14.0% |

| metric | median 0245bbd | median HEAD | Δ median |
|--------|---------------:|------------:|---------:|
| ingest_plain.n | 136039970 | 111797484 | −17.8% |
| as_of.n | 281069 | 256599 | −8.7% |
| open.n | 2710537108 | 2705649871 | −0.2% |

D-10 ≤10% regression vs `0245bbd` on ingest/`as_of`/`open.n`: **met** (HEAD not slower on medians). Pair 2 as_of +14% is host noise (HEAD 282k vs pair 1 HEAD 231k). Worktrees and target dirs removed after the run.

## Phase 60 (M019 S05): writer lock

Same host, Eager, `KUTHA_BENCH_N=10000`. Isolated worktrees and absolute target dirs. Binaries pinned with `taskset -c 10`. Interleaved `base, head` × 6 pairs after a discarded warm-up. Base `1f9fb14` vs head `642ddd3`. Host load 5–9 from other agents. Compare interleaved medians; do not treat a single pair as a gate.

| metric | median 1f9fb14 | median 642ddd3 | Δ median |
|--------|---------------:|---------------:|---------:|
| ingest_plain.n | 112518252 | 108327822 | −3.72% |
| as_of.n | 295375 | 280262 | −5.12% |
| open.n | 2813254980 | 2754772929 | −2.08% |

A first 2-pair run showed a one-off 3× `as_of` outlier (892233 ns) that did not reproduce in the 6-pair run.

D-10 ≤10% vs `1f9fb14` on ingest/`as_of`/`open.n`: **PASS**.



