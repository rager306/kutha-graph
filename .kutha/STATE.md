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
L_delivery=M175-S01-closed
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M175-S01 closed (C4 path-policy foundation; GSD v1.68) | Full ABAC / tenant product |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

M175-S01 is closed under GSD **v1.68 C4 path-policy foundation**. Delivered `crates/kutha-runtime/src/security.rs`: closed-default `PathPolicy` over existing typed QueryIr. `Scan`, unlabeled walks, and zero-hop queries deny by default; labeled hops require every relation to resolve and be policy-allowed. `compile_path_policy` decides before `prepare`; `execute_authorized` preserves authorized GPML/typed results and denied queries return typed `SecurityError::PolicyDenied`, never empty rows. Post-policy query failures use distinct `SecurityError::QueryRejected`. Tests: policy 5/5, integration 4/4, workspace 753 passed / 0 failed; independent verifiers PASS. Security ADR-080/081 stay Proposed; no grants-as-facts, tenant slices, vacuum authority, capabilities, WASM, full ABAC, or ADR Accepted. M174-S01 remains closed (Gate GB; security eligibility recorded). M173-S01 remains closed (M003a parser delivered). M172-S01 remains closed (B3 no-gap; B4/B5 skipped). M171/M170 residuals stay non-blocking. Residuals stay separate: eCFR edition ambiguity, remote rocks observe skipped, GSD security audit warning. Honeycomb stays Proposed; `I-legal-pack-not-next` stays.

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
