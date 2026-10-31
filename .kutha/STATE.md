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
L_delivery=M184-S01-closed
L_capability=ff5-green
```

| Lifecycle | Current | Must not read as |
|-----------|---------|------------------|
| L_map | ADR-000–002 and 010–093 are **Proposed** | Product ready / Accepted |
| L_delivery | M184-S01 closed (admission operator index; GSD v1.77) | New capability / ADR Accepted |
| L_capability | FF5 green (`as_of(2015) ≠ as_of(2021)` on statute fixture) | Governor CI green |

## Next action

M184-S01 is closed under GSD **v1.77 admission operator index**. `Runtime` now holds droppable `OperatorIndex` and `PackLifecycleTracker` leases, rebuilt from the log on hydrate/open/fork and updated on commit, so pack install/activate/retire admission is O(1) per pack write (measured per-install cost flat: 255 µs → 287 µs from 400 → 3200 installs; before, 213 µs → 2.33 ms). The first independent verifier blocked the slice: `hydrate` applied `PackRetire` after the install loop, so a legal install-after-retire rebuilt as a collision and an accepted store could fail to reopen on open/fork/append/checkpoint. Retire is now applied inline in log order, guarded by `retired_name_reuse_survives_every_reopen_path`. A constructor that skips `replay_check` now marks the leases untrusted and admission fails closed (`meta_leases_untrusted`). `operator_count()` (live names) and `pack_operators()` (audit rows incl. retired) are documented and pinned by a test. Honest scope: `Op::EnrichmentProposal` admission still replays the lifecycle; the linearity guard is a source-shape assertion plus the recorded measurement, not a wall-clock assertion. Tests: admission index 10/10, workspace 826 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0 with `run_high=0 run_low=0`. No new Op, feature, operator execution, legal pack, grants/capabilities/WASM, or ADR Accepted. M183-S01 remains closed (digest falsifiers + linear index). M182/M181/M180/M179/M178/M177/M176/M175/M174/M173/M172 remain closed. M171/M170 residuals stay non-blocking. Residual stays separate: remote rocks observe skipped. Honeycomb stays Proposed; `I-legal-pack-not-next` stays.

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
