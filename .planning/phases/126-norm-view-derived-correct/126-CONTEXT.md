---
phase: 126-norm-view-derived-correct
---

# Phase 126 — Context

CE #28: NormView keys eligibility off Behavior claim lineage +
`derivation_eligible_at` but pushes Correct/CI replacement triples unchecked.
A Correct of derived Q(a,b) to Q(a,c) inherits rule eligibility. Fix: require
the live row to match the Behavior head proposition. Reuse M037 claim-lineage
oracles. Honeycomb stays Proposed.
