# Gate GB closure decision (2026-10-30)

Status: **Gate GB closed for M003a scope**. This is verification/scope closure, not ADR Accepted and not security implementation.

## Evidence

`crates/kutha-runtime/tests/m174_gate_gb.rs` proves GPML `AS OF` parity on the existing fixture corpus:

- **FF5 statute**: vt 2015 and 2020; parsed GPML equals typed `try_hops_named`; executed nodes equal the `as_of` facade.
- **B3 layered workload**: every layer-0 start, 1 and 2 hops; parsed GPML equals typed IR and execution is non-empty.
- **M167 eCFR PIT**: 3 sections x 7 published cuts; GPML nodes equal the `as_of` facade.
- Unknown relation/term fail closed.

Raw source fixtures are the existing eCFR files; no parser or QueryIr semantics changed.

## Decision

Gate GB is closed for the supported M003a subset. ADR-080/081 security is now technically eligible for an owner-leased security milestone because M003a exists and Gate GB parity passes.

## Boundary

- C4 (ADR-080 ABAC) and C6 (ADR-081 sandbox) are **not implemented** here.
- Security remains a separate owner-leased milestone.
- M003b pack-dictionary binder swap remains gated on C2.
- No HNSW/planner/legal-pack/Op/ADR/CI change; honeycomb stays Proposed.
