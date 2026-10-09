# Requirements: Kutha

**Milestones:** v0.23 / harness M029 shipped 2026-10-09. Next lease: controller decision. Lease: `.kutha/STATE.md`.

## v0.23 Requirements (M029) — complete

### Interrupted rollover seal heal (Phase 89, S01)

- [x] **HEAL-01**: after RolloverTrailer / RolloverNewHeader crash, `open_for_append` finishes the seal and can append

## v0.22 Requirements (M028) — complete

### Dictionary retract on open (Phase 88, S01)

- [x] **REV-01**: trailing `emit_retract_dict` of a `kutha:*` Assert survives persist/open (revoked allow stays revoked)

## v0.21 Requirements (M027) — complete

### Trajectory honesty (Phase 87, S01)

- [x] **TRAJ-01**: `killed` rows with characterization-shaped `kill_mutation` HIGH; ROADMAP `— ACTIVE` forbidden while Active Milestone is None

## v0.20 Requirements (M026) — complete

### Generators (Phase 85, S01)

- [x] **GEN-01**: `MixStyle::Coverage` reaches Retract-before-Assert, overlap, keyed retry, derived Correct/Retract, empty ops; oracles G-C04/C05/C11 with kill_mutation

### Matrix honesty (Phase 86, S02)

- [x] **MAT-01**: Honor-system and tautology rows re-labelled or given real mutants

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| Persister lock-on-error (#2) | Next G1 residual |
| WAL-tail heal (#30) | Next G1 residual |
| Product fold / Cypher / HNSW | Thaw map |
