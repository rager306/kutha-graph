# Phase 100 CONTEXT — vacuous planning gates

**Source:** CE full-branch review P1 #16 (G3 governor silent-pass).  
**Lease:** M040 S01 / GSD v0.34.

## Goal

`verification-independent` used `allow_empty: true` so a live CONTEXT without any VERIFICATION file still passed. Drop `allow_empty`. Gate content steps with `when_glob_exists` on CONTEXT: idle (no CONTEXT) skips; live CONTEXT without VERIFICATION is HIGH. Unit oracles in `VerificationIndependentTests`. Honeycomb stays Proposed.

## Oracles

- `test_verification_context_without_verification_is_high`
- `test_verification_idle_without_context_skips`
- `test_frontmatter_context_without_verification_is_high`
