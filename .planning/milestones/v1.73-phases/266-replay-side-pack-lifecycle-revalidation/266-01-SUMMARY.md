---
phase: 266-replay-side-pack-lifecycle-revalidation
plan: 01
status: complete
requirements: [RPL-01]
---

# Summary — Replay-side pack lifecycle revalidation

- `Runtime::check_meta_replay` walks the log in order and validates each meta op against the exact prefix before it, reusing the writer-side rules.
- Pack lifecycle: activate-before-install, duplicate install, retire-unknown, wrong version, double activate/retire, and missing/mismatched manifest all fail replay.
- Enrichment: unknown/retired/inactive pack, wrong version, empty pins, missing source, missing/mismatched payload, duplicate cache key all fail replay.
- Source existence is required for every proposal after the first checkpoint; the exemption applies only to proposals a later checkpoint compacts away.
- Valid stores still open and derive the same lifecycle, registry, and ledger; no graph fact or fingerprint change.
- Tests: replay meta 10/10 plus a unit test for the tail rule; workspace 801 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0.
