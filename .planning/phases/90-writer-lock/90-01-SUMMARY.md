# Phase 90-01 SUMMARY

`release_lock_if_poisoned` keeps `kutha.lock` on non-poison `Other` append/checkpoint errors; fail-closed `InvalidData` still poisons and releases. Heal oracles `drop` the crashed writer before `open_for_append` / `migrate`. Honeycomb Proposed. Oracles: `m030_writer_lock.rs`.
