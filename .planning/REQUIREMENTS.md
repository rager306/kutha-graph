# Requirements: Kutha

**Milestones:** v0.19 / harness M025 shipped 2026-10-09. **v0.20 / harness M026** active. Lease: `.kutha/STATE.md`.

## v0.20 Requirements (M026)

### Generators (Phase 85, S01)

- [x] **GEN-01**: `MixStyle::Coverage` reaches Retract-before-Assert, overlap, keyed retry, derived Correct/Retract, empty ops; oracles G-C04/C05/C11 with kill_mutation

### Matrix honesty (Phase 86, S02)

- [ ] **MAT-01**: Honor-system and tautology rows re-labelled or given real mutants

## Out of Scope (this milestone)

| Feature | Reason |
|---------|--------|
| Product fold changes | Only if oracle exposes a bug |
| Cypher / HNSW / ADR-090 pack | Thaw map |
| cargo-mutants as CI gate | Verifier evidence only |
