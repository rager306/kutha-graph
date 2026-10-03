# 52-01 SUMMARY — Incremental framed append

Executor Phase 52 / M002 S04. Not verification. Honeycomb stays Proposed. Did not edit STATE/README/REQUIREMENTS/ROADMAP or write VERIFICATION.

## Commits

- `c1d5379` test(52-01): RED oracles for incremental append
- `758ba43` fix(52-02): drop torn jsonl tails without a newline
- `3d01a1f` feat(52-02): Persister framed append with tail guard and checkpoint
- `eb314b6` feat(52-02): add persist_append.n to the baseline bench
- `7840062` docs(52): APPEND-01 evidence, bench table, and changelog
- `df8da47` test(52-01): RED oracles for intern-after-append and tail-guard mutations
- `f08ac15` fix(52-02): restore interned terms from log Defines on snapshot open
- this commit: Define-on-open evidence, mutation oracles, CHANGELOG, bench re-run

## RED (`m002_append01`)

Wave 1 failed 10 / preserved 2. Wave 2 failed 4: intern-between-appends, intern-only reattach, twice-then-open dict parity, checkpoint abort-before-rename. Guard length/id and checkpoint dir-fsync already green.

## Blast radius (CBM `root-kutha-graph`; rg arbiter)

`open` inbound ~69 (M010–M017, benches). Coverage: store/hydrate `metadata_changed`; `persist_append.rs` / `m002_append01.rs` `not_tracked` (no `index_repository`). M010/M012 Define oracles unchanged.

## old → new

`from_snapshot` interns log `Op::Define` names not in `snap.dict_strings` (log order) then still drops Defines from the fold log. `open` passes unfiltered events. `attach` last `EventId`/frame_len come from the on-disk jsonl tail (Define-safe). Checkpoint abort: thread-local `replace_snapshot(..., false)`. **D-01 deviation:** `hydrate.rs` `from_snapshot` only, for this gap.

## Bench (D-10)

Original vs `bd5fa4b`: ingest −6.4%, as_of −1.5%. Re-run after open-path fix (4 interleaved pairs, `tmp/p52-*-target-gap`): ingest 111.4M → 107.6M (−3.4%); as_of 288188 → 284030 (−1.4%). **No ≥10% median regression.** HEAD persist.n median 5.96 s vs persist_append.n 48.7 ms.

## Residual (D-12) + mutations

WAL duplicates jsonl (S06). Single-writer. `durable_graph_count` tracks encoded non-Define prefix; file guard is lengths + last on-disk `EventId`. Mutations (c)(d)(i)(f) killed in-tree then `git checkout -- persist_append.rs` (clean).
