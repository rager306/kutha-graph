# Interim decision: D-06 ingest indexes as leases (2026-10-01)

**Status:** interim controller-facing note (not an Accepted ADR rewrite).  
**Lease:** M013 S03 — do **not** open M002 Rocks. RocksDB is cited as design analogy only.  
**Related:** `.compound-engineering/artifacts/handoffs/m013-s03-process-and-d06.md`, ADR-040 clarification (2026-10-01), `docs/architecture/benchmark-baseline.md` § Phase 24 / reserve spike.

## Plain-language problem

We added hot maps so ingest lookups stay O(1) (visit oracle green). Every assert now also updates several HashMaps. That costs wall time. Phase 22 baseline ~95 ms; after indexes still ~+21…+28% even with `reserve`. So D-06 (≤10% ingest regression) fails — not because the oracle is wrong, but because we tax the write path to keep indexes warm.

## Interim decision (locked for continuity)

1. **North star:** treat fold/runtime lookup indexes as **ADR-040 leases** (derived, droppable, rebuildable from the log/facts), not as SoT and not as mandatory synchronous work on every assert forever.
2. **Near-term experiment (optional, cheap):** `rustc_hash::FxHashMap` on trusted `EventId` / `TermId` maps; remeasure D-06. Do **not** expect this alone to guarantee ≤10%. Skip IdentityHasher for `Uuid` keys; skip `get_mut`-first on insert-heavy mint paths.
3. **If D-06 still fails after FxHash (or we skip the spike):** prefer honesty over micro-opts:
   - either **thin write path** (append + minimal fold; secondary maps lazy / rebuild / delta→compact later), or
   - **process:** keep visit-oracle as the index correctness gate; document write-cost tradeoff / soften absolute D-06 for S03 close.
4. **Out of S03:** VList, full Materializer/CSR delta lease as a shipping slice, M002 Rocks wiring.
5. **Uncommitted reserve spike** remains experimental; alone it does not clear D-06.

## Why RocksDB is a useful analogy (and what it is not)

Kutha’s product formula is already LSM-shaped at the idea level: **append log = SoT**, fold/views = droppable pictures. RocksDB is the frozen future durable engine (M002), not the thing we patch today.

What RocksDB (and LSM secondary-index literature) actually does that maps here:

| Rocks / LSM pattern | Meaning in plain language | Kutha mapping |
|---------------------|---------------------------|---------------|
| Write path = WAL + memtable; heavy work in flush/compaction | Writes stay cheap; indexes/files catch up in background | Assert should not pay full secondary-index tax if indexes are leases |
| Secondary indexes historically **app-maintained** via column families | Engine stores sorted KV; *you* write primary + index keys | Our hot HashMaps are the in-memory cousin of “extra CF writes” |
| Built-in secondary indices (TransactionDB, e.g. [PR #13180](https://github.com/facebook/rocksdb/pull/13180)) | On update: read old primary → delete old secondary → write primary → write new secondary **in one txn** | That is **eager** index maintenance — same class of tax we feel on ingest |
| Header comment in `secondary_index.h` | Secondary indices need txn semantics; maintained on insert/update/delete | Confirms: “always-fresh secondary” is not free |
| Community practice ([rocksdb.dev thread](https://www.facebook.com/groups/rocksdb.dev/posts/1237404363024742/)): composite keys / empty values vs posting lists | Prefer append-friendly index shapes; avoid RMW posting lists when possible | Prefer append/lazy merge over RMW `entry`+Vec on every assert when thinning |
| BlobDB / key-value separation ([wiki](https://github.com/facebook/rocksdb/wiki/BlobDB), [blog](https://rocksdb.org/blog/2021/05/26/integrated-blob-db.html)); lazy blob resolution ([#14386](https://github.com/facebook/rocksdb/issues/14386)) | Keep hot write/read path from touching fat payloads; resolve later | Separate “truth append” from “warm projection” |
| Deferred secondary-index updates in LSM research (hidden-read problem; see e.g. deferred-update LSM writeups) | Don’t Get-old-tuple on every write; reconcile in compaction | Lazy / delta→frozen leases instead of eager HashMap RMW |
| Eager vs Lazy secondary indexes ([Qader et al. ACM](https://dl.acm.org/doi/pdf/10.1145/3183713.3196900); Luo/DELI line) | Eager = fast lookup, slow write; Lazy = fast write, merge on read/compact | Our current Phase 24 maps ≈ Eager; lease north star ≈ Lazy + rebuild |

**Not implied:** import Rocks secondary-index APIs into S03, unfreeze M002, or copy BlobDB. Analogy only: Rocks pays for secondary freshness or defers it — we should name which bill S03 accepts.

## Next actions (controller picks)

1. Optional FxHash spike + rebench D-06.  
2. Else close S03 with visit-oracle gate + documented write-cost, **or** thin/lazy index write before close.  
3. Longer-term Materializer lease (delta→frozen / CSR) stays honeycomb Proposed under ADR-040 — not this interim lock.

## Sources touched this note

- In-repo: ADR-040, benchmark-baseline Phase 24, handoff `m013-s03-process-and-d06.md`, `fold.rs` / `quantum.rs` index leases.  
- Exa / Jina / Keen / GitHub: rustc-hash & FxHash benches; Aletheia IdentityHasher + incremental CSR; RocksDB BlobDB wiki/blog; RocksDB secondary index PR #13180; issue #14386 (lazy blob resolution); rocksdb.dev secondary-index maintenance discussion; LSM eager/lazy secondary index papers.
