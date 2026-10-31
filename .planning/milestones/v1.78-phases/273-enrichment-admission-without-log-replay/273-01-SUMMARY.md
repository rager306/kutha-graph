---
phase: 273-enrichment-admission-without-log-replay
plan: 01
status: complete
requirements: [ENR-01]
---

# Summary — enrichment admission leases

- Enrichment proposal admission and the emit pre-check use runtime leases (`PackLifecycleTracker`, visible-log id set, blob map, borrowed cache-key set) instead of `derive_pack_lifecycle` plus a full event-id set.
- `check_proposal` is read-only; the caller inserts the key after commit, so a rejected proposal cannot poison the lease and admission does not clone the key set.
- The cache-key and visible-log leases are rebuilt on hydrate/open/fork and updated on every append (`append_indexed`), including `intern`'s `Op::Define`.
- Admission and replay share one `check_proposal`; the log-scanning wrappers were removed.
- **Two verifier blockers fixed:** the first attempt never updated the key lease on commit, cloned the set per write, and used a source predicate broader than replay's; the second round found `intern` bypassing the append choke point and a replay carve-out that was broader than needed.
- Tests: enrichment admission 6/6, workspace 832 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0.
