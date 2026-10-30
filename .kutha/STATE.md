# Kutha harness state

**Lease** of process intent + last fold — not process SoT. Process SoT (H0) is `.kutha/events.jsonl` (gitignored observations). Product SoT is the event log (D1): sealed `KUTHASEG` segments plus `manifest.json`, with legacy `events.jsonl` + `events.wal` until `store::migrate`; `crates/kutha-runtime` folds it. `events.jsonl` after migrate is an export. Not an ADR.

**STCA plane:** Space = check slices; Time = harness JSONL; Composition = `--budget`.

**Active Milestone:** M171
**Active Slice:** S01
**Phase:** H5
**Release:** 0.0.01 (`VERSION`, format `N.N.NN`). Packaging SemVer in Cargo and pyproject is `0.0.1` because Cargo rejects a leading zero. `publish` stays false.

## Lifecycles (do not collapse)

```text
L_map=honeycomb-proposed
L_delivery=M171-S01-open
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M171-S01 active (ingest bisect; GSD v1.64) | Capability proven / Cypher started |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

M171-S01 is leased under GSD **v1.64 ingest regression**: phase 249 names the first bad commit for `ingest_plain.n` versus `cdf1942` on a same-host bisect; phase 250 fixes it or records the tradeoff. **Phase 249 is blocked**: bisect candidate `199807d` (append_indexed maps) is supported by the Phase 24 +16.9/+28.5% note, but the independent verifier rejected the n=2 confirmation (1.68x outlier) and the required artifact is missing. Unblock with >=5 interleaved N=10000 samples for `cdf1942`, `c26bf35`, `199807d`, `929bbc6` on an idle host, reporting min and median; then record the table in `docs/architecture/benchmark-baseline.md` and CHANGELOG. Phase 250 should accept the known per-assert index cost unless that table separates a new cost. No product fix in phase 249. M170-S01 remains closed (independent PASS, 730 tests, run_high=0) and its follow-ups stay non-blocking: shared scan helper, capacity pre-pass/over-reservation on heavily retracted cuts, and the m019 store-lock flake. Residuals stay separate: ingest cause, eCFR edition ambiguity, remote rocks observe skipped, GSD security audit warning. Honeycomb stays Proposed; `I-legal-pack-not-next` stays.

## Thaw map

The freeze list came from agent-assisted commits (`1ee5942`, `85db1f7`, 2026-08-18, Cursor co-authored); the owner states he did not decide it. M002 is thawed (2026-10-03, owner's request). Every other item keeps its own named gate; nothing is thawed by "an M002 lease".

| Item | Thaws when |
|------|------------|
| RocksDB crate | Thawed (Phase 51/55): optional dependency `rocksdb` in kutha-runtime only; `freeze-rocks` is a durable rule (optional dependencies only) |
| Cypher/GPML parser | M003 is leased |
| HNSW | M005 is leased |
| ADR-050 six dictionaries | SIX-01 is leased |
| ADR-080/081 | a security milestone is leased (after M003) |
| full ADR-090 ontology | STATE names a legal pack (`I-legal-pack-not-next`) |
| ADR-093 | a science pack is leased (M004) |
| Consensus Query 103+ | the owner lifts the literature bound |
