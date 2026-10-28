---
phase: 129-disk-free
plan: 01
---

# Phase 129-01 — Summary

- Kind `disk_free` (`min_free_gib` in YAML); check `disk-free`; `I-disk-free`.
- Selftest derives `min_free_gib: 999999`. Pytest `DiskFreePreflightTests`.
- FSM / `.env.example` budget 78→79.
- Bench markers for Phases 65/66 moved to tracked phase paths.
- Matrix `SEM-m058-disk-free` killed. No crates.
