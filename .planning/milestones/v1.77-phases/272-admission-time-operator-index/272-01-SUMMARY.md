---
phase: 272-admission-time-operator-index
plan: 01
status: complete
requirements: [OPS-01]
---

# Summary — admission-time operator index

- `Runtime` holds a droppable `OperatorIndex` and `PackLifecycleTracker`, rebuilt from the log on hydrate/open/fork and updated on commit.
- Pack install/activate/retire admission is O(1) per pack write: measured per-install cost is flat (400 → 3200 installs: 255 µs → 287 µs); before, it grew 213 µs → 2.33 ms.
- **Verifier blocker, fixed:** the first rebuild applied `PackRetire` only after the install loop, so a legal install-after-retire was rebuilt as a collision and a store the runtime had accepted could fail to reopen. Retire is now applied inline in log order, guarded by `retired_name_reuse_survives_every_reopen_path` (live, open, fork, checkpoint).
- **Second verifier finding, fixed:** a constructor that skips `replay_check` now marks the leases untrusted, and admission fails closed with `meta_leases_untrusted` instead of trusting a short index.
- Documented that `operator_count()` (live names) and `pack_operators()` (audit rows incl. retired) differ by design, with a pinning test.
- Honest scope: `Op::EnrichmentProposal` still replays the lifecycle per proposal; this milestone is about pack writes.
- Tests: admission index 10/10, workspace 826 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0.
