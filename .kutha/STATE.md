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
L_delivery=M182-S01-closed
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M182-S01 closed (meta-surface threat audit; GSD v1.75) | Security implementation / ADR Accepted |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

M182-S01 is closed under GSD **v1.75 meta-surface threat audit**. Delivered the phase security register (`269-SECURITY.md`): 10 threats across four trust boundaries, 8 mitigated with adversarial tests, 2 accepted with re-open triggers. The audit found and fixed a real defect: meta replay was O(n²) (2000 pack installs ~2.18 s on open; now ~13 ms via a single-pass `PackLifecycleTracker` plus incremental manifest checks). Admission and replay now share `PackLifecycleTracker` and `check_proposal`; the independent verifier caught that the first unification dropped retired-pack operator release on replay, and the fix plus `admission_accepted_log_reopens_after_retire` are in. `declared_operators_for` errors are propagated, not swallowed. Two threats are recorded as transitively covered by the generic `StoreBlob` digest gate instead of overclaimed. Tests: threats 10/10, workspace 811 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0. No new Op, feature, grants/capabilities/WASM/ABAC implementation, legal pack, or ADR Accepted. M181-S01 remains closed (eCFR ambiguity explained). M180/M179/M178/M177/M176/M175/M174/M173/M172 remain closed. M171/M170 residuals stay non-blocking. Residual stays separate: remote rocks observe skipped. Honeycomb stays Proposed; `I-legal-pack-not-next` stays.

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
