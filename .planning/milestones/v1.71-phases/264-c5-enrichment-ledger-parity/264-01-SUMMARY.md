---
phase: 264-c5-enrichment-ledger-parity
plan: 01
status: complete
requirements: [C5-02]
---

# Summary — C5 ledger parity

- Typed `Op::EnrichmentProposal`: pack id/version, source event ids, model/prompt/cache pins, raw-SHA-256 payload digest.
- Admission: active pack only, exact version, non-empty pins, existing sources, blob present and matching, unique cache key. Validation runs before the payload append, so a rejected proposal leaves no orphan `StoreBlob`.
- `Runtime::enrichment_ledger()` is replay-derived and lifecycle-aware; no model call and no derived lease.
- Parity: persist/open, fork cuts, and checkpoint_sealed_prefix with real multi-segment geometry.
- Graph facts and fold fingerprint unchanged; user `emit` remains `MetaOpRejected`.
- Tests: enrichment 10/10; workspace 788 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0.
