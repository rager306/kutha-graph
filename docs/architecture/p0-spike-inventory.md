# P0 spike inventory

> **Status 2026-10-04:** Inventory of the crates P0 spike, not Accepted product. Live lease and freeze: `.kutha/STATE.md`.

This is the **crates** P0 spike, **not** Accepted product. Live lease and freeze live in `.kutha/STATE.md`; this file does not name them.

## In spike

log, fold, quantum emit→idle, receipt, snapshot, WAL-cousin, CSR lease, leapfrog intersect, `Materializer` trait, `fork_at`, named `as_of`/`live_at`, FF6 relation allowlist, typed clocks (`ValidTime` / `TransactionTime` / `WallTime`), `Op::TtAnchor`, `as_known_on`, `known_at`, `RuleAst` / evaluator (`eval.rs` `live_horns`), genesis log (`kutha:allows`), public `Runtime` facade, portable `claim_id`, `Op::Define`. **2026-10-03:** `events.jsonl` SoT + covering `KUTHAWAL` WAL (`load_events_d04`); snapshot and WAL replace via tmp+sync+rename (`store.rs` `replace_via_tmp`); Horn lookup lease-first via `GraphFold.horns_by_premise`. **2026-10-04:** `open` can read `manifest.json` + `KUTHASEG` v1 (plan 54-01); empty/segmented dirs persist that layout and `Persister` appends to the active segment (plan 54-02); legacy jsonl+WAL dirs keep the previous writer.

## Not in spike

RocksDB crate, Cypher, HNSW, ABAC, legal corpus, ADR-050 six dictionaries.
