# Kutha harness state

**Lease** of process intent + last fold — not process SoT. Process SoT (H0) is `.kutha/events.jsonl` (gitignored observations). Product SoT is the event log (D1): sealed `KUTHASEG` segments plus `manifest.json`, with legacy `events.jsonl` + `events.wal` until `store::migrate`; `crates/kutha-runtime` folds it. `events.jsonl` after migrate is an export. Not an ADR.

**STCA plane:** Space = check slices; Time = harness JSONL; Composition = `--budget`.

**Active Milestone:** None
**Active Slice:** None
**Phase:** H5
**Release:** 0.0.01 (`VERSION`, format `N.N.NN`). Packaging SemVer in Cargo and pyproject is `0.0.1` because Cargo rejects a leading zero. `publish` stays false.

## Lifecycles (do not collapse)

```text
L_map=honeycomb-proposed
L_delivery=M186-S01-closed
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M186-S01 closed (meta lease overhead A/B; GSD v1.79) | New capability / ADR Accepted |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

M186-S01 is closed under GSD **v1.79 meta lease overhead A/B**. Two interleaved same-host A/Bs (6 samples each, `KUTHA_BENCH_N=10000`, bench source unchanged, separate target dirs, base commit ids recorded) measured the v1.77/v1.78 meta leases. Against `4db8770` (v1.77 close, isolates v1.78): `ingest_plain.n` +2.7%, `as_of.n` +5.7%, one 253 µs base outlier named not dropped. Against `05dfea0` (v1.76 close, isolates v1.77+v1.78): `ingest_plain.n` −4.2% (faster), `as_of.n` +1.7%, `as_of.2n` +19.0% (reproduced at +16.3% in the other A/B), `ingest_keyed.n` +12.1%. The bench gate in AGENTS.md covers `ingest_plain.n` and `as_of.n` only: both stay inside the 10% stop line, so the M186 keep decision stands; `ingest_keyed.n` **+12.1%** is above that line and is recorded as a secondary signal, not a gate failure. Decision: keep the leases for the write path (they remove an O(n) lifecycle rebuild per meta write); the benefit is cited from the v1.77 install measurement, not measured here. `as_of.2n` +19.0% has no structural mechanism: `fold.rs`, `leapfrog.rs`, and `fold_query.rs` are unchanged from `05dfea0` to HEAD, and a same-binary toggle run (10+10 samples) moved `as_of.2n` by −2.2% while costing `ingest_plain.n` +11.4%; the toggle patch was not preserved, so that run is recorded evidence, not a reproducible procedure. Post-close audit (2026-10-31) found four of six A/B2 head medians had been computed from unsorted positional samples; the corrected values are above and the per-sample record is `.compound-engineering/artifacts/research/2026-10-31-m186-ab-samples.md`. The first verifier blocked a false scope claim (the initial A/B measured v1.78 only) and a deleted baseline worktree; both are fixed. M185-S01 remains closed (enrichment admission leases). M184/M183/M182/M181/M180/M179/M178/M177/M176/M175/M174/M173/M172 remain closed. M171/M170 residuals stay non-blocking. Residual stays separate: remote rocks observe skipped. Honeycomb stays Proposed; `I-legal-pack-not-next` stays.

## Thaw map

The freeze list came from agent-assisted commits (`1ee5942`, `85db1f7`, 2026-08-18, Cursor co-authored); the owner states he did not decide it. M002 is thawed (2026-10-03, owner's request). Every other item keeps its own named gate; nothing is thawed by "an M002 lease". No row below is active at HEAD (`Active Milestone: None`): each entry states the future gate, not a current lease.

| Item | Thaws when |
|------|------------|
| RocksDB crate | Thawed (Phase 51/55): optional dependency `rocksdb` in kutha-runtime only; `freeze-rocks` is a durable rule (optional dependencies only) |
| Cypher/GPML parser | Thawed (M003a leased 2026-10-30): read-only subset parser; M003b binder swap stays gated on C2 |
| HNSW | a milestone the owner names M005 is leased |
| ADR-050 six dictionaries | a milestone whose slice names SIX-01 is leased |
| ADR-080/081 | a security milestone is leased (after M003) |
| full ADR-090 ontology | STATE names a legal pack (`I-legal-pack-not-next`) |
| ADR-093 | a science pack is leased (M004) |
| Consensus Query 103+ | the owner lifts the literature bound |
