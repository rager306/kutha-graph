---
phase: 271-linear-manifest-operator-collision-index
plan: 01
status: complete
requirements: [DIG-02]
---

# Summary — Linear manifest operator collision index

- Added `OperatorIndex` (operator→pack) and `validate_install_indexed`; collision detection is one lookup per name.
- Re-declare replaces the previous set and retire releases names, so ownership cannot go stale (verifier finding fixed).
- Removed the unused `validate_install_against` path so there is one collision rule.
- 400→1600 pack scaling test guards linearity; `OperatorIndex` has a unit test for collision, re-declare, and retire.

- Tests: digest falsifiers 5/5; workspace 816 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0.
