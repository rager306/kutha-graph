---
module: kutha-runtime
tags: [m011, claim, support, retract]
problem_type: architecture-pattern
---

# Portable claim_id with independent supports

## Problem

ADR-011 requires claim identity distinct from local `fact_seq`. Two sources supporting the same proposition must remain distinguishable; withdrawing one must not erase the other.

## Solution

- `Op::Assert.claim: Option<EventId>` (serde default). Absent → `claim_id = event.id`. Present → attach support to that claim.
- `Fact.claim_id` on the fold; Correct inherits it; Behavior uses `event.id`.
- Oracles: `live_support_count` / `claim_supported_at` at a TT×VT cut.

## Do not

- Treat duplicate triples as one Fact.
- Build P→Q derivation or provenance polynomials in this slice.
- Start M002.
