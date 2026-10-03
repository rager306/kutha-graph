# Roadmap: Kutha

## Overview

GSD overlay tracks the steel thread already in crates — not honeycomb waterfall. Harness delivery stays in `.kutha/STATE.md` / `.kutha/ROADMAP.md`.

v0.01–v0.09 are shipped. v0.10 (harness M017) is authored; only the slice named in `.kutha/STATE.md` runs.

## Id map

See `docs/process/gsd-auto-contract.md` § Id map. Do not copy the table onto this ROADMAP.

## Milestones

- ✅ **v0.01–v0.08** — shipped — see `./milestones/`
- ✅ **v0.09 Test hardening + executable rules** — Phases 36–40 (harness M016) — [archive](./milestones/v0.09-ROADMAP.md)
- 🚧 **v0.10 Docs honesty and replay integrity** — Phases 41–44 (harness M017)

## Phases

- [x] **Phase 41: Living docs and governor honesty** — DOC-02..05, GOV-02 (docs only; no `crates/`)
- [x] **Phase 42: Replay integrity** — REPL-01..04, KNOWN-03 (product code; CBM + GitNexus pair)
- [ ] **Phase 43: Determinism and knowledge-cut characterization** — DET-01, KNOWN-04 (tests first)
- [ ] **Phase 44: Ingest regression repair** — PERF-01, PERF-02 (product code; found by Phase 42 bench bisect)

### Phase 41: Living docs and governor honesty
**Goal**: Living docs, ADR Evidence and the governor agree with the code at `c702d71`.
**Depends on**: —
**Requirements**: DOC-02, DOC-03, DOC-04, DOC-05, GOV-02
**Success criteria**: PROJECT has no stale lease tokens and `cite-lease` fails on a mutated one; gap-review addendum and invariants ledger match M014–M016; ADR clarifications and README name three AS OF cuts; `uv run kutha-gov precommit` green; independent verification artifact.

### Phase 42: Replay integrity
**Goal**: Knowledge-cut anchors, snapshot and WAL cannot be silently corrupted or overridden by leases.
**Depends on**: Phase 41
**Requirements**: REPL-01, REPL-02, REPL-03, REPL-04, KNOWN-03
**Success criteria**: Each REPL oracle is RED on `c702d71` and GREEN after; FF5/FF6/M010–M016 assertions unchanged; baseline `as_of` / ingest within 10 percent.

### Phase 43: Determinism and knowledge-cut characterization
**Goal**: Replay determinism and the Horn-vs-known-on behaviour are named oracles.
**Depends on**: Phase 42
**Requirements**: DET-01, KNOWN-04
**Success criteria**: Seeded shuffle + persist/open properties green with no new dependency; characterization test recorded in ADR-013 Evidence; no behaviour change.

### Phase 44: Ingest regression repair
**Goal**: Ingest cost returns to the v0.08 level without changing behaviour.
**Depends on**: Phase 43
**Requirements**: PERF-01, PERF-02
**Success criteria**: bisect names the M016 commit(s); scaling oracle RED then GREEN; named oracles unchanged; `ingest_plain.n` within 10 percent of v0.08 on the same host or controller-accepted residual.

## Progress

| Milestone | Phases | Status |
|-----------|--------|--------|
| v0.01–v0.09 | 1–40 | shipped |
| v0.10 | 41–44 | 2/4 |

**Next:** Phase 43 (lease S03). Phase 44 stays authored, unleased.
