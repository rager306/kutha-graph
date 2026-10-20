# Roadmap: Kutha

## Overview

v0.01–v1.12 shipped. v1.13 gives anchor-admit persist/open a per-call directory. Honeycomb stays Proposed.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. v1.13 closed as M119 (`L_delivery=M119-closed`).

## Milestones

- ✅ **v0.01–v0.98** — shipped (through unindex cause reverse pointer)
- ✅ **v0.99 residual correctness and hot-path honesty** — phases 176–179 closed (M105)
- ✅ **v1.00 open restores fold-index lease** — phase 180 closed (M106)
- ✅ **v1.01 unindex skips the live-behavior vector** — phase 181 closed (M107)
- ✅ **v1.02 live behaviors are a set** — phase 182 closed (M108)
- ✅ **v1.03 known_at ingest index** — phase 183 closed (M109)
- ✅ **v1.04 rebuild indexes only live Behaviors** — phase 184 closed (M110)
- ✅ **v1.05 hot-path lease dogfood** — phase 185 closed (M111)
- ✅ **v1.06 selftest substring trap** — phase 186 closed (M112)
- ✅ **v1.07 selftest mutation static** — phase 187 closed (M113)
- ✅ **v1.08 cargo observe flock** — phase 188 closed (M114)
- ✅ **v1.09 empty --check on ci** — phase 189 closed (M115)
- ✅ **v1.10 unique known-on persist dir** — phase 190 closed (M116)
- ✅ **v1.11 unique quantum known-on persist dir** — phase 191 closed (M117)
- ✅ **v1.12 unique closed-VT known-on persist dir** — phase 192 closed (M118)
- ✅ **v1.13 unique anchor-admit persist dir** — phase 193 closed (M119)

## Phases (v1.13)

- [x] **Phase 193:** anchor-admit persist/open uses a per-call store directory

## Phases (v1.12)

- [x] **Phase 192:** closed-VT known-on persist/open uses a per-call store directory

## Phases (v1.11)

- [x] **Phase 191:** quantum known-on persist/open uses a per-call store directory

## Phases (v1.10)

- [x] **Phase 190:** as_known_on persist/open uses a per-call store directory

## Phases (v1.09)

- [x] **Phase 189:** ci exits 2 when --check is present but empty

## Phases (v1.08)

- [x] **Phase 188:** observe_cargo exclusive flock on Cargo target dir

## Phases (v1.07)

- [x] **Phase 187:** get_checks LoadError on substring-preserving declared selftest replace_regex

## Phases (v1.06)

- [x] **Phase 186:** replace_regex with that still contains pattern is UNPROVEN

## Phases (v1.05)

- [x] **Phase 185:** governor pins hot-path needles; unindex has no full-bucket fallback

## Phases (v1.04)

- [x] **Phase 184:** rebuild keeps dead Behavior heads out of derived_by_cause

## Phases (v1.03)

- [x] **Phase 183:** warm known_at skips facts ingested after the cut

## Phases (v1.02)

- [x] **Phase 182:** one Behavior removal does not walk every live id

## Phases (v1.01)

- [x] **Phase 181:** non-behavior unindex returns before walking `live_behaviors`

## Phases (v1.00)

- [x] **Phase 180:** open calls `set_index_mode` so snapshot lease warmth survives the env

## Phases (v0.99)

- [x] **Phase 176:** newest-first policy pin election (`policy_by_ingest`)
- [x] **Phase 177:** premise-rel reverse pointer on unindex
- [x] **Phase 178:** Op-wire skips a duplicate modern Define
- [x] **Phase 179:** all Horn combinations; replay VT must be covered; open keeps the replay fold; Correct walks every live residual

## Still open

- `FoldIndexMode::Lazy` still full-scans until hot maps rebuild (intentional thin write)
- Cypher, HNSW, legal pack, ADR-050 six kinds stay frozen
- ADR status stays Proposed (`docs/ADR/README.md` engine-evidence table, 2026-10-18)

## Progress

See `.planning/STATE.md` and `.kutha/STATE.md`.
