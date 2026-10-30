# Roadmap: Kutha

## Overview

v1.70 extends the C1 lifecycle foundation with a bounded **pack manifest and operator registry** (C1b). A pack install may pin a content-addressed manifest blob that declares read-only operator names; admission validates identity, schema, and cross-pack collisions, and the runtime derives an operator registry from the log. No operator execution, dictionary facets, lease rebuild, WASM, marketplace, schema SMO, or ADR Accepted.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not restate the table here.

## Phases

- [x] **Phase 261: C1b pack manifest and operator registry** - manifest schema, blob pin, admission validation (planned) (completed 2026-10-31)
- [x] **Phase 262: C1b pack registry runtime parity** - replay-derived registry, persist/open/fork/checkpoint parity (planned) (completed 2026-10-31)

## Phase Details

### Phase 261: C1b pack manifest and operator registry
**Goal**: Add a content-addressed pack manifest and fail-closed registration metadata without executing pack code.
**Depends on**: Phase 260
**Requirements**: C1B-01
**Success Criteria**:
  1. `PackInstall` can pin a manifest digest; legacy manifest-less installs still parse.
  2. Manifest identity matches the install and declares unique, valid, read-only operator names.
  3. Missing, malformed, mismatched, reserved, duplicate, or cross-pack-colliding operators are rejected at admission.
**Plans**: 1/1 plans complete

Plans:
- [x] 261-01-PLAN.md — manifest schema, blob pin, admission validation

### Phase 262: C1b pack registry runtime parity
**Goal**: Derive a deterministic operator registry from admitted manifests and prove persistence parity.
**Depends on**: Phase 261
**Requirements**: C1B-02
**Success Criteria**:
  1. `Runtime::pack_operators()` lists declared operators with pack lifecycle state.
  2. Registry survives persist/open, fork prefix cuts, and checkpoint_sealed_prefix.
  3. Graph facts and fold fingerprint remain unchanged; no operator is executed.
**Plans**: 1/1 plans complete

Plans:
- [x] 262-01-PLAN.md — replay-derived registry and parity tests

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 261. C1b pack manifest and operator registry | 1/1 | Complete    | 2026-10-31 |
| 262. C1b pack registry runtime parity | 1/1 | Complete    | 2026-10-31 |
