# Requirements: Kutha

## v1.80 Requirements

- [ ] **TEST-275-01**: The two enrichment replay tests use a real source event and assert the intended rejection reason.
- [ ] **TEST-275-02**: Replay rejects pack version-mismatch activate, second activate and second retire (three named tests).
- [ ] **TEST-275-03**: The eCFR probe's assertion catches a lost or gained edition; no redundant assert is claimed as an independent falsifier.
- [ ] **TEST-276-01**: No test uses a fixed relative `tmp/` directory; concurrent runs cannot delete a peer's store.
- [ ] **TEST-276-02**: The two scaling oracles assert structure (counters or op counts), not wall-clock.
- [ ] **TEST-276-03**: `SecurityError::QueryRejected` is either constructed by `from_query_error` with a test, or removed; 257/258 record the outcome.

## Explicit non-goals

- No new `Op`, no graph/log/serialization change, no thaw, no ADR Accepted.
- No change to enrichment, pack or eCFR product behaviour beyond the error taxonomy named in TEST-276-03.

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| TEST-275-01 | Phase 275 | Pending |
| TEST-275-02 | Phase 275 | Pending |
| TEST-275-03 | Phase 275 | Pending |
| TEST-276-01 | Phase 276 | Pending |
| TEST-276-02 | Phase 276 | Pending |
| TEST-276-03 | Phase 276 | Pending |
