# Roadmap: Kutha

## Overview

v1.80 is test honesty: it closes the five test defects found by the 2026-10-31 audits and the
`QueryRejected` error-taxonomy claim that 257/258-VERIFICATION record as `gaps_found`. No new `Op`,
no graph or log behaviour change, no thaw. The work is in `crates/kutha-runtime/tests/*.rs` plus one
named mapping in `security.rs`.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not restate the table here.

## Phases

- [ ] **Phase 275: Replay oracles and the eCFR falsifier** - real enrichment fixtures, three lifecycle replay tests, an eCFR assert that cannot lie
- [ ] **Phase 276: Scratch isolation, clock-free oracles, error taxonomy** - unique tmp dirs, structural scaling oracles, QueryRejected made true or removed

## Phase Details

### Phase 275: Replay oracles and the eCFR falsifier

**Goal**: The replay gates are tested by tests that fail for the reason they name.
**Depends on**: Phase 274
**Requirements**: TEST-275-01, TEST-275-02, TEST-275-03
**Success Criteria**:
  1. `enrichment_for_unknown_pack_fails_replay` and `duplicate_enrichment_cache_key_fails_replay` pass a real source event and assert "not installed" / "cache key".
  2. Replay rejects version-mismatch activate, second activate and second retire, each with a named test.
  3. The eCFR probe fails on a lost or gained edition, and no assert is claimed to catch a direction it cannot.
**Plans**: 1/1 plans

Plans:
- [ ] 275-01-PLAN.md — fixtures, lifecycle replay tests, eCFR assert

### Phase 276: Scratch isolation, clock-free oracles, error taxonomy

**Goal**: Concurrent runs cannot collide, scaling oracles assert structure, and the security error taxonomy matches its documentation.
**Depends on**: Phase 274 (file-disjoint from Phase 275; both run in wave 1)
**Requirements**: TEST-276-01, TEST-276-02, TEST-276-03
**Success Criteria**:
  1. No test under `crates/` writes a fixed relative `tmp/` path; two concurrent runs of each affected binary both pass.
  2. `replay_meta_is_not_quadratic` and `manifest_replay_scales_linearly_with_packs` assert counters or op counts, not elapsed time.
  3. `SecurityError::QueryRejected` is constructed by `from_query_error` with a test, or removed; the 257/258 records match.
**Plans**: 1/1 plans

Plans:
- [ ] 276-01-PLAN.md — tmp isolation, structural oracles, taxonomy

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 275. Replay oracles and the eCFR falsifier | 0/1 | Planned | — |
| 276. Scratch isolation, clock-free oracles, error taxonomy | 0/1 | Planned | — |
