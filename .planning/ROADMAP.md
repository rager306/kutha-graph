# Roadmap: Kutha

## Overview

v1.72 answers **Gate GC** with a read-only probe: a toy (non-legal) pack installs, is queried through GPML `AS OF`, retires, and replays with zero core edits. The probe uses only the C1/C1b surfaces and the existing query IR; it does not use C5. No operator execution, no dictionary facets, no HNSW, no WASM, no legal pack, no ADR Accepted.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not restate the table here.

## Phases

- [x] **Phase 265: Gate GC toy-pack zero-core-edit probe** - install, GPML AS OF query, retire, replay parity; zero core edits (completed 2026-10-31)

## Phase Details

### Phase 265: Gate GC toy-pack zero-core-edit probe
**Goal**: Prove a toy pack installs, queries through GPML AS OF, and retires with zero core edits.
**Depends on**: Phase 264
**Requirements**: GC-01
**Success Criteria**:
  1. A toy pack installs with a manifest and active lifecycle; product facts remain pack-agnostic.
  2. GPML `AS OF` returns the expected cut before and after retire; no core file changes for the probe.
  3. Retire + replay is parity-equal on facts, fingerprint, lifecycle, and operator registry.
**Plans**: 1/1 plans complete

Plans:
- [x] 265-01-PLAN.md — Gate GC probe and recorded evidence

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 265. Gate GC toy-pack zero-core-edit probe | 1/1 | Complete    | 2026-10-31 |
