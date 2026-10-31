# Roadmap: Kutha

## Overview

v1.73 hardens **replay honesty** for the C1/C1b/C5 meta surfaces. Admission already fails closed at write time, but `replay_check` and `Runtime::open` accepted a hand-edited log whose pack lifecycle, manifests, or enrichment proposals were inconsistent. This milestone revalidates those surfaces on replay and rejects the store instead of deriving a silently wrong lease. No new Op, no new feature, no thaw.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not restate the table here.

## Phases

- [ ] **Phase 266: Replay-side pack lifecycle revalidation** - lifecycle ordering and manifest identity rechecked on replay
- [ ] **Phase 267: Replay-side enrichment validation** - proposal identity, sources, payload, and cache key rechecked on replay

## Phase Details

### Phase 266: Replay-side pack lifecycle revalidation
**Goal**: Reject a log whose pack lifecycle or manifest is inconsistent, instead of deriving a partial lease.
**Depends on**: Phase 265
**Requirements**: RPL-01
**Success Criteria**:
  1. `replay_check` and `open` reject activate-before-install, duplicate install, retire-of-unknown, and version mismatch.
  2. A cited manifest must exist, match its digest, and match the install identity.
  3. A valid store still opens and derives the same lifecycle/registry as before.
**Plans**: 1 plan

Plans:
- [ ] 266-01-PLAN.md — replay validation for lifecycle and manifests

### Phase 267: Replay-side enrichment validation
**Goal**: Reject a log whose enrichment proposals are inconsistent.
**Depends on**: Phase 266
**Requirements**: RPL-02
**Success Criteria**:
  1. Replay rejects unknown/retired pack, wrong version, empty pins, missing source, missing/mismatched payload, duplicate cache key.
  2. A valid store still opens and derives the same ledger.
  3. Graph facts and fingerprint are unchanged by the new checks.
**Plans**: 1 plan

Plans:
- [ ] 267-01-PLAN.md — replay validation for enrichment proposals

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 266. Replay-side pack lifecycle revalidation | 0/1 | Pending | — |
| 267. Replay-side enrichment validation | 0/1 | Pending | — |
