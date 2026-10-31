# Roadmap: Kutha

## Overview

v1.77 closes the last recorded complexity gap: v1.76 made **replay** collision detection linear, but **admission** still rebuilt every declared operator set per install. This milestone keeps an operator index on the runtime as a droppable lease, rebuilt from the log on hydrate, so admission is O(1) per operator too. No new Op, no feature, no thaw.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not restate the table here.

## Phases

- [x] **Phase 272: Admission-time operator index** - runtime-held index, hydrate rebuild, parity with replay (completed 2026-10-31)

## Phase Details

### Phase 272: Admission-time operator index
**Goal**: Make admission-time collision detection O(1) per operator.
**Depends on**: Phase 271
**Requirements**: OPS-01
**Success Criteria**:
  1. The runtime holds an `OperatorIndex` rebuilt from the log on hydrate/open/fork.
  2. Admission uses it and rejects the same collisions replay rejects.
  3. A many-install benchmark shows admission is not super-linear.
**Plans**: 1/1 plans complete

Plans:
- [x] 272-01-PLAN.md — runtime index and parity tests

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 272. Admission-time operator index | 1/1 | Complete    | 2026-10-31 |
