# Roadmap: Kutha

## Overview

GSD overlay tracks the steel thread already in crates — not honeycomb waterfall. Harness delivery stays in `.kutha/STATE.md` / `.kutha/ROADMAP.md`.

v0.01–v0.10 are shipped. v0.11 (harness M018) is paused after Phase 45 at a decision gate. v0.12 (harness M002, thawed) is planned; one slice is leased at a time.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not copy the table onto this ROADMAP.

## Milestones

- ✅ **v0.01–v0.10** — shipped — see `./milestones/`
- 🚧 **v0.11 Wedge unblock** — Phases 45–48 (harness M018) — Phase 45 shipped; paused at the Phase 46 decision gate
- 📋 **v0.12 Log durability protocol** — Phases 49–55 (harness M002, thawed) — planned

## Phases

- [x] **Phase 45: Doc truth round 2** — ADR Evidence, F-register prefix, English spine, hygiene (S01, docs only)
- [ ] **Phase 46: C01 and knowledge-cut decision** — ADR-013 verdict; Horn vs `as_known_on`; `as_of` vs eligibility (S02, decision gate)
- [ ] **Phase 47: Horn determinism and closure** — derived identity, Correct re-derives, cascade admit and order (S03)
- [ ] **Phase 48: Differential tests and oracle tightening** — fold/CSR/rebuild property, oracle tightening, bench gate (S04)
- [x] **Phase 49: Thaw map and freeze governor honesty** — M002 S01, docs and YAML only
- [x] **Phase 50: SNAP-01 and fail-closed WAL open** — sidecar rename, dir fsync, crash matrix (M002 S02)
- [x] **Phase 51: RocksDB build spike** — isolated module, offline build, EventId to offset (M002 S03)
- [x] **Phase 52: Incremental framed append** — no O(N) rewrite per persist (M002 S04)
- [x] **Phase 53: Commit marker decision (COMMIT-01)** — one encoding of the quantum outcome (M002 S05, decision gate)
- [ ] **Phase 54: Segments, manifest and hash chain** — (M002 S06)
- [ ] **Phase 55: Droppable RocksDB index lease** — (M002 S07)

## Progress

| Milestone | Phases | Status |
|-----------|--------|--------|
| v0.01–v0.10 | 1–44 | shipped |
| v0.11 | 45–48 | Phase 45 shipped; paused at decision gate |
| v0.12 | 49–55 | Phases 49–53 shipped; Phase 54 next |
