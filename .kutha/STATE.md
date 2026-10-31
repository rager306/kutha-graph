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

M186-S01 is closed under GSD **v1.79 meta lease overhead A/B**. Two interleaved same-host A/Bs (6 samples each, `KUTHA_BENCH_N=10000`, bench source unchanged, separate target dirs, base commit ids recorded) measured the v1.77/v1.78 meta leases. Against `4db8770` (v1.77 close, isolates v1.78): `ingest_plain.n` +2.7%, `as_of.n` +5.7%, one 253 µs base outlier named not dropped. Against `05dfea0` (v1.76 close, isolates v1.77+v1.78): `ingest_plain.n` −6.2% (faster), `as_of.n` −7.7%, but **`as_of.2n` +19.0%**, reproduced at +16.3% in the other A/B, above the 10% gate. Decision: keep the leases for the write path (they remove an O(n) lifecycle rebuild per meta write) and record `as_of.2n` as an **open, unexplained residual** — the bench exercises no meta write, so the benefit is cited from the v1.77 install measurement, not measured here. The first verifier blocked a false scope claim (the initial A/B measured v1.78 only) and a deleted baseline worktree; both are fixed. M185-S01 remains closed (enrichment admission leases). M184/M183/M182/M181/M180/M179/M178/M177/M176/M175/M174/M173/M172 remain closed. M171/M170 residuals stay non-blocking. Residual stays separate: remote rocks observe skipped. Honeycomb stays Proposed; `I-legal-pack-not-next` stays.

## Thaw map

The freeze list came from agent-assisted commits (`1ee5942`, `85db1f7`, 2026-08-18, Cursor co-authored); the owner states he did not decide it. M002 is thawed (2026-10-03, owner's request). Every other item keeps its own named gate; nothing is thawed by "an M002 lease".

| Item | Thaws when |
|------|------------|
| RocksDB crate | Thawed (Phase 51/55): optional dependency `rocksdb` in kutha-runtime only; `freeze-rocks` is a durable rule (optional dependencies only) |
| Cypher/GPML parser | Thawed (M003a leased 2026-10-30): read-only subset parser; M003b binder swap stays gated on C2 |
| HNSW | M005 is leased |
| ADR-050 six dictionaries | SIX-01 is leased |
| ADR-080/081 | a security milestone is leased (after M003) |
| full ADR-090 ontology | STATE names a legal pack (`I-legal-pack-not-next`) |
| ADR-093 | a science pack is leased (M004) |
| Consensus Query 103+ | the owner lifts the literature bound |
