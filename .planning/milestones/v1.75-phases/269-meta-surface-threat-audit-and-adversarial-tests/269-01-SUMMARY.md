---
phase: 269-meta-surface-threat-audit-and-adversarial-tests
plan: 01
status: complete
requirements: [SEC-269-01]
---

# Summary — meta-surface threat audit

- Register: 10 threats across four trust boundaries; at close 6 had direct adversarial tests, 2 were transitively covered by the generic `StoreBlob` digest gate (direct falsifiers added in M183), and 2 are accepted with re-open triggers.
- **Defect found and fixed:** meta replay was O(n²) — 2000 pack installs took ~2.18 s on open. A single-pass tracker plus incremental manifest checks bring 2000 ops to ~13 ms (4000 ops ~0.08 s in the guard test).
- Admission and replay now share `PackLifecycleTracker` and `check_proposal`, so the two gates cannot drift apart.
- **Regression caught by the verifier and fixed:** the first unified replay did not release operator names on retire, so a log admission accepted could fail to reopen. Fixed, with `admission_accepted_log_reopens_after_retire` as the guard.
- Two threats are recorded as transitively covered by the generic `StoreBlob` digest gate rather than as direct tests, instead of overstating coverage.
- Tests: threats 10/10, workspace 811 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0.
