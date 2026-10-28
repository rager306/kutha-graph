---
phase: 151-observe-dual-bin
---

# Phase 151 — observe build both operator bins

After M078, `kutha-migrate` exists but `observe_cargo` build only compiles
`kutha-tenant`. Thin slice: one `cargo build` with both `--bin` flags; extend
`tenant-bin`. Honeycomb stays Proposed.
