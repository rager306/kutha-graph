# Phase 94 CONTEXT — Correct Horn re-fire

**Source:** CE full-branch review P1 #9.  
**Lease:** M034 S01 / GSD v0.28.

## Goal

`Op::Correct` follow-ons must not lose a same-triple derived fact: use retract except-set and fires-then-retracts like CorrectInterval.

## Oracles

- `correct_same_triple_keeps_derived_inverse_live`
- `correct_object_change_re_fires_new_inverse`
