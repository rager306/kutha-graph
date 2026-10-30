# Roadmap: Kutha

## Overview

v1.67 closes **Gate GB** for M003a: prove GPML `AS OF` parity across the existing fixture corpus (FF5, B3, M167 eCFR), record scope/caveats, and state security eligibility without implementing C4/C6. No thaw beyond M003a; no product semantics change.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not restate the table here.

## Phases

- [x] **Phase 255: Gate GB fixture parity** - GPML vs typed facade on FF5/B3/eCFR (completed 2026-10-30)
- [x] **Phase 256: Gate GB closure decision** - record Gate GB result and security eligibility (completed 2026-10-30)

## Phase Details

### Phase 255: Gate GB fixture parity
**Goal**: Prove parsed GPML `AS OF` results equal the typed facade across FF5, B3, and M167 eCFR fixtures.
**Depends on**: Nothing
**Requirements**: GB-01
**Success Criteria**:
  1. FF5 2015/2020 parity is covered.
  2. B3 layered 1/2-hop parity is covered.
  3. eCFR 3 sections x 7 published cuts are covered; unknown vocabulary fails closed.
**Plans**: 1/1 plans complete

Plans:
- [ ] TBD (run /gsd-plan-phase 255 to break down)

### Phase 256: Gate GB closure decision
**Goal**: Record Gate GB closure, explicit caveats, and the security eligibility boundary.
**Depends on**: Phase 255
**Requirements**: GB-02
**Success Criteria**:
  1. Gate GB evidence is cited.
  2. Scope is limited to read-only supported GPML; no C4/C6 implementation.
  3. Security remains a separate owner-leased milestone.
**Plans**: 1/1 plans complete

Plans:
- [ ] TBD (run /gsd-plan-phase 256 to break down)

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 255. Gate GB fixture parity | 1/1 | Complete    | 2026-10-30 |
| 256. Gate GB closure decision | 1/1 | Complete    | 2026-10-30 |
