# Roadmap: Kutha

## Overview

v1.76 closes the two honest gaps v1.75 recorded instead of overclaiming: the manifest and enrichment digest checks were only "transitively covered" by the generic `StoreBlob` gate, and the manifest operator-collision scan was O(P²). This milestone gives each check a direct falsifier and makes the collision index linear. No new Op, no feature, no thaw.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not restate the table here.

## Phases

- [ ] **Phase 270: Direct falsifiers for manifest and payload digests** - prove the raw-SHA-256 checks fire on their own
- [ ] **Phase 271: Linear manifest operator collision index** - operator→pack index, linear replay

## Phase Details

### Phase 270: Direct falsifiers for manifest and payload digests
**Goal**: Make the manifest and enrichment digest checks independently falsifiable.
**Depends on**: Phase 269
**Requirements**: DIG-01
**Success Criteria**:
  1. A blob addressed by a policy/rule-body hash that passes the generic gate but fails raw SHA-256 is rejected as a manifest.
  2. The enrichment payload digest check fires on the same construction.
  3. Deleting the raw-SHA-256 check makes those tests fail.
**Plans**: 1 plan

Plans:
- [ ] 270-01-PLAN.md — adversarial digest fixtures

### Phase 271: Linear manifest operator collision index
**Goal**: Replace the per-install scan of all declared operators with an index.
**Depends on**: Phase 270
**Requirements**: DIG-02
**Success Criteria**:
  1. Collision detection is O(1) per operator, not O(P) per install.
  2. Retire still releases names; admission and replay agree.
  3. A many-pack benchmark shows near-linear replay.
**Plans**: 1 plan

Plans:
- [ ] 271-01-PLAN.md — operator index and benchmark

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 270. Direct falsifiers for manifest and payload digests | 0/1 | Pending | — |
| 271. Linear manifest operator collision index | 0/1 | Pending | — |
