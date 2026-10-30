---
phase: 265-gate-gc-toy-pack-zero-core-edit-probe
plan: 01
status: complete
requirements: [GC-01]
---

# Summary — Gate GC toy-pack probe

- Toy non-legal pack installs with a manifest and activates; manifest blob and operator registry are genuinely loaded.
- GPML `AS OF 2015` returns `50-kmh`, `AS OF 2020` returns `30-kmh` (half-open boundary), with the pack installed and after retire.
- Retire changes lifecycle and registry only; facts and fingerprint unchanged.
- Reopen parity on a real persisted store: facts, fingerprint, lifecycle, registry, and both cuts.
- Zero production edits in this slice: no `crates/*/src`, Cargo, build.rs, or dictionary change.
- Scope honesty: the query path does not consult pack state, so the probe evidences pack neutrality, not query sufficiency.
- Tests: gate 2/2; workspace 790 passed / 0 failed; `kutha-gov ci --jobs 1` exit 0.
