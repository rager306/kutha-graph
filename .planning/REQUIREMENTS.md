# Requirements: Kutha

**Milestones:** v0.21 / harness M027 shipped 2026-10-09. Next lease: controller decision. Lease: `.kutha/STATE.md`.

## v0.21 Requirements (M027)

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
| Product fold changes | Only if oracle exposes a bug |
| Cypher / HNSW / ADR-090 pack | Thaw map |
| cargo-mutants as CI gate | Verifier evidence only |
