# Roadmap: Kutha

## Overview

v1.61 Legal PIT probe closed on 2026-10-29. Milestone **v1.62 Performance truth** (M168) **closed** on 2026-10-29: the A/B confirms a real 3.3× `as_of` regression and `bytes_per_fact` now prints `n/a` on a zero RSS delta. No product semantics, no thaw, no next milestone leased.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not restate the table here.

## Phases

- [x] **Phase 245: Bench A/B** - interleaved HEAD vs `cdf1942` medians on this host (completed 2026-10-29)
- [x] **Phase 246: Bytes honesty** - zero RSS delta reports `n/a`, not `0 B` (completed 2026-10-29)

## Phase Details

### Phase 245: Bench A/B
**Goal**: Run old and HEAD benches interleaved on the same idle host and record medians.
**Depends on**: Nothing
**Requirements**: PF-01
**Success Criteria**:
  1. `cdf1942` and HEAD each run at least twice, interleaved.
  2. `ingest_plain.n` and `as_of.n` medians are recorded with host/method.
  3. The old commit's numbers are compared to the 2026-10-01 table.
**Plans**: 1/1 plans complete

### Phase 246: Bytes honesty
**Goal**: Stop printing `0 B` when RSS did not move.
**Depends on**: Phase 245
**Requirements**: PF-02
**Success Criteria**:
  1. Zero delta or zero live facts prints `n/a`.
  2. A fresh HEAD bench records the new key honestly.
**Plans**: 1/1 plans complete

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 245. Bench A/B | 1/1 | Complete    | 2026-10-29 |
| 246. Bytes honesty | 1/1 | Complete    | 2026-10-29 |
