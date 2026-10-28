---
phase: 22-benchmark-baseline-and-size-target
plan: 02
status: complete
---

# Phase 22 Plan 02 Summary

## Done

- Wrote `docs/architecture/benchmark-baseline.md` from a default-N=10000 measured run.
- Documented ratios, three-size persist/open, `bytes_per_fact`, target N 1000000, memory budget 13674000000.
- CHANGELOG Product + Process + Trajectory coupled. D-07 committed path set exact vs `tmp/p22-base.txt`.

## Commits

- `docs(22-02): record measured benchmark baseline note`

## Verify

- Note greps for ratios, persist/open, target N, memory budget, re-run command.
- Committed path set: bench, Cargo.toml, note, CHANGELOG, `.planning/*` only.
