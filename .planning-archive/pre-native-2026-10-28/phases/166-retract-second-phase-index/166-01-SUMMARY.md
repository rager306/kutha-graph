---
phase: 166-retract-second-phase-index
plan: 01
---

# Summary

`retract_derived` second phase now walks `behaviors_by_premise_rel` for the
retracted fact's relation instead of cloning all `live_behaviors`. Append /
rebuild / unindex maintain the lease. Oracles in `m095_retract_second_phase.rs`.
