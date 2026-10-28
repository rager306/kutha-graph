---
phase: 227-verus-on-ramp
independent: true
status: passed
verifier_model: parent-session
commands:
  - cmd: sha256sum -c tools/verus/SHA256SUMS
    exit: 0
  - cmd: RUSTUP_TOOLCHAIN=1.99.1 $KUTHA_VERUS_BIN verus/vt_intersect.rs
    exit: 0
  - cmd: uv run kutha-gov selftest --check verus-proof-sync
    exit: 0
  - cmd: uv run kutha-gov precommit --check verus-proof-sync,docs-coupling,budget-covers-checks,budget-template-align
    exit: 0
  - cmd: uv run kutha-gov ci --jobs 1
    exit: 0
read_only_inputs:
  - crates/kutha-runtime/src/fold.rs
  - verus/vt_intersect.rs
  - tools/verus/VERSION
  - .kutha/dictionaries/checks.yaml
  - docs/architecture/test-matrix.yaml
---

# Phase 227 — independent verification

**Verdict:** passed. Verus release `0.2026.10.18.7d82994` (rustc 1.99.1, checksum matches the GitHub release digest) verifies `verus/vt_intersect.rs`: `2 verified, 0 errors`. The proof body is the production `vt_intersect` body line for line.

**Contract proven:** for every point `x < u64::MAX`, `x` is in the result iff it is in both inputs (`None` result: no common point); a `Some` result is non-empty and never carries `Some(u64::MAX)` as its end.

**Negative controls (Verus must fail):** `a_from.max(b_from)` changed to `min` gives `1 verified, 1 errors`; the sentinel test `to_raw == u64::MAX` changed to `to_raw == 0` gives `1 verified, 1 errors`.

**Not claimed:** callers (`conjunct_vt`, `windows_cover`), serde, the fold, the sentinel point `u64::MAX` itself (it is the open-end encoding). `verus-proof-sync` pins sync of the body and the pin; it does not run Verus, and CI has no Verus dependency. Budget 104 to 105. No thaw; honeycomb stays Proposed.
