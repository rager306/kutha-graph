# Roadmap: Kutha

## Overview

v1.64 Ingest regression is active. M170 closed the `as_of` allocation residual; M168 recorded `ingest_plain.n` +12.6% versus the controlled `cdf1942` baseline. Bisect `cdf1942..HEAD` on a same-host bench, name the first bad commit, then fix or record the tradeoff. No product feature, no thaw.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not restate the table here.

## Phases

- [ ] **Phase 249: Ingest bisect** - name the first bad commit for ingest_plain.n
- [ ] **Phase 250: Ingest fix or accept** - fix the cost or record the tradeoff

## Phase Details

### Phase 249: Ingest bisect
**Goal**: Name the commit that took `ingest_plain.n` from the controlled `cdf1942` baseline to HEAD.
**Depends on**: Nothing
**Requirements**: PF-05
**Success Criteria**:
  1. Old and HEAD are measured on the same host at a stable N.
  2. `git bisect` yields the first bad commit.
  3. The cause is stated in code terms; no product fix is attempted here.
**Plans**: 0 plans

Plans:
- [ ] TBD (run /gsd-plan-phase 249 to break down)

### Phase 250: Ingest fix or accept
**Goal**: Remove the ingest cost or record why it is the accepted tradeoff.
**Depends on**: Phase 249
**Requirements**: PF-06
**Success Criteria**:
  1. A fix lands, or the tradeoff and its cause are explicit.
  2. A post-change interleaved bench records `ingest_plain.n`.
  3. No semantics change beyond the ingest path.
**Plans**: 0 plans

Plans:
- [ ] TBD (run /gsd-plan-phase 250 to break down)

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 249. Ingest bisect | 0/1 | Pending | — |
| 250. Ingest fix or accept | 0/1 | Pending | — |
