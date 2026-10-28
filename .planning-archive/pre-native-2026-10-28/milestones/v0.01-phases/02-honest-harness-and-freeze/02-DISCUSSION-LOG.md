# Phase 2 — Discussion Log

**Date:** 2026-09-29
**Mode:** Seeded D-G* then Claude-decided remaining gray areas (user: «реши эти вопросы разумно»)

## Locked by user

| ID | Decision |
|----|----------|
| D-G1 | `ci` after each execute-wave + before phase-verify; brief `explain trajectory` in SUMMARY |
| D-G2 | HIGH stops wave; WARN recorded, never masked |
| D-G3 | Improvements = harness YAML/docs + ADR/roadmap clarity; not M002 / thaw / legal pack |

## Claude decided (gray areas 1–6)

| Area | ID | Choice |
|------|-----|--------|
| SUMMARY trajectory template | D-10 | Structured short block: cmds + HIGH/WARN counts + ≤8 lines excerpt + green≠Accepted sentence |
| WARN at phase-verify | D-11 | WARN does not block closeout if fully ledgered; unlogged WARN = failure; HIGH blocks |
| PLANE evidence | D-12 | Path/schema/source probes only — no new product features |
| FREEZE evidence | D-13 | Absence + lease cite; never “fix” by implementing frozen surfaces |
| MAP / honeycomb | D-14 | Proposed map + no phase-per-cell backlog + `kutha-gov map` smoke |
| Cargo beside ci | D-15 | Cargo offline at tracer wave + before phase-verify (and any wave that touches `crates/`); not every wave |

## Skipped / deferred

- GOV-03 → Phase 3
- Escalating WARN to HIGH-equivalent → deferred
- Full discuss of every probe regex → planner may tighten strings

## Status

**Ready for planning** — `/gsd-plan-phase 2`
