# Roadmap: Kutha

## Overview

v1.78 closes the last recorded super-linear admission path: `Op::EnrichmentProposal` admission still ran `derive_pack_lifecycle` plus a full event-id set per proposal. The runtime already holds a lifecycle lease; this milestone gives the runtime a content-addressed payload set and a cache-key set, so enrichment admission is O(1) per proposal and identical to replay. No new Op, no feature, no thaw.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not restate the table here.

## Phases

- [ ] **Phase 273: Enrichment admission without log replay** - runtime payload/cache-key leases, O(1) proposal admission, replay parity

## Phase Details

### Phase 273: Enrichment admission without log replay
**Goal**: Make enrichment proposal admission O(1) and identical to replay.
**Depends on**: Phase 272
**Requirements**: ENR-01
**Success Criteria**:
  1. The runtime holds a droppable payload-digest set and cache-key set, rebuilt from the log.
  2. Proposal admission uses them and the lifecycle lease, with no log scan.
  3. Admission and replay accept/reject the same proposals; a many-proposal benchmark is flat.
**Plans**: 1 plan

Plans:
- [ ] 273-01-PLAN.md — runtime leases and parity tests

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 273. Enrichment admission without log replay | 0/1 | Pending | — |
