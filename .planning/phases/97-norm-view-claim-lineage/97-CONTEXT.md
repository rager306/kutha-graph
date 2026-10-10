# Phase 97 CONTEXT — norm_view claim-lineage eligibility

**Source:** CE full-branch review P1 #13.  
**Lease:** M037 S01 / GSD v0.31.

## Goal

`norm_view` must gate eligibility on claim lineage (`event_by_id(f.claim_id)` is `Op::Behavior`), not on the minting event type. Correct/CorrectInterval replacements of derived claims keep `claim_id` but mint under Correct ids; minting-event checks skip `derivation_eligible_at` and launder ineligible derived facts into `Ok`.

## Oracles

- `correct_of_ineligible_derived_still_abstains_in_norm_view`
- `correct_interval_of_ineligible_derived_still_abstains_in_norm_view`
- `correct_of_eligible_derived_still_ok_in_norm_view`
