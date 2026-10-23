# P0 spike inventory

> **Status 2026-10-04:** Inventory of the crates P0 spike, not Accepted product. Live lease and freeze: `.kutha/STATE.md`.

This is the **crates** P0 spike, **not** Accepted product. Live lease and freeze live in `.kutha/STATE.md`; this file does not name them.

## In spike

log, fold, quantum emit→idle, receipt (`Op::QuantumOutcome`), snapshot, WAL-cousin, CSR lease, leapfrog intersect, `Materializer` trait, `fork_at`, named `as_of`/`live_at`, FF6 relation allowlist, typed clocks (`ValidTime` / `TransactionTime` / `WallTime`), `Op::TtAnchor`, `as_known_on`, `known_at`, `RuleAst` / evaluator (`eval.rs` `live_horns`), genesis log (`kutha:allows`), public `Runtime` facade, portable `claim_id`, `Op::Define`, Retract-all-live, keyed retry with Correct lineage (M022/M023), TR-02 / C01 pack-stamp pins (M021). **Physical SoT (HEAD):** sealed `KUTHASEG` + `manifest.json` for new/migrated dirs; legacy `events.jsonl` + `KUTHAWAL` until `store::migrate`; jsonl after migrate is an unread export. Snapshot/WAL/segment replace via tmp+sync+rename. Horn lookup lease-first via `GraphFold.horns_by_premise`. **Checkpoint (HEAD):** `store::checkpoint_sealed_prefix` / `Op::Checkpoint` — library API only, no operator CLI, not vacuum. **In spike (feature-gated, not wired):** optional `rocksdb` behind `rocks-index`; `rocks_index::SegmentIndex` locators; default `cargo test` does not compile RocksDB.
## Not in spike

Cypher, HNSW, ABAC, legal corpus, ADR-050 six dictionaries. Historical (superseded 2026-10-04): "Not in spike: RocksDB crate".
