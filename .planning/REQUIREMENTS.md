# Requirements: Kutha

**Core Value:** Legal PIT stays falsifiable; governor CI stays honest; one Active Slice — not honeycomb delivery.
**State:** v0.10 shipped (archive `.planning/milestones/v0.10-REQUIREMENTS.md`). No live requirements until the controller leases the next milestone.

## Future Requirements (not leased)

### M002 — log durability protocol (frozen until leased; v0.11 slot)

- **ROCK-01**: Segmented append-only log, hash chain, atomic manifest; Rocks only for indexes
- **COMMIT-01**: Quantum outcome as segment-framing commit marker vs `Op::QuantumOutcome` in the log (ADR-010/014 F11) — one encoding decision
- **SNAP-01**: Sidecars (`outcomes`, `justifications`, `terms`) replaced via temp + sync + rename and parent-directory fsync; an unreadable or partial sidecar is treated as absent, not as a reason to reject an intact log; `wal::recover_events` propagates non-EOF I/O errors; `check_tt_anchors` tolerates a log that is ahead of the snapshot after a crash between renames
- **WRITE-01**: `intern` admit path; WAL vs JSONL open preference (v0.10 made jsonl the SoT and WAL a covering lease)
- **IO-01** (intake only): kernel flusher/dirty-throttle on cloud block storage — `docs/architecture/m002-cloud-kernel-io-intake.md` (arXiv:2609.12597)
- **R3 exclusion**: do not re-ship "lease identity verified on open" (v0.04) as M002 identity
- **PERF-GATE**: bench re-run (`ingest_plain.n`, `as_of.n`, same host, vs previous tag) at every product milestone close

### Candidate slots (need a controller decision)

- **CLOCK-01**: Type `Op` / `Event` / `Snapshot` clocks (`Vt`/`Tt`/`WallTime`); remove `u64::MAX` as "current TT" in facade and eval (F03, F14)
- **C01-01**: Application-end / sunset clause: pack stamp, fifth axis, or stop — ADR-013 D013-1 vs FIXG-01 verdict
- **HORN-KC-01**: Should Horn derivation follow the knowledge cut (F05)? Characterized today; decision pending

### ADR-050 six dictionaries

- **SIX-01**: Full six agent dictionaries — frozen until STATE thaws them

## Out of Scope

| Feature | Reason |
|---------|--------|
| Segments, hash chain, Rocks, Cypher, HNSW | Freeze; M002+ |
| Promoting honeycomb to Accepted | L_map stays Proposed |
