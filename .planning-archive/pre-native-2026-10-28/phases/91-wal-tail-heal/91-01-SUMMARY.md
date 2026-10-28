# Phase 91-01 SUMMARY

`open_for_append` heals when legacy jsonl tip is ahead of WAL (`legacy_attach_needs_rewrite`), not only when `dropped_uncommitted>0`. Rewrite reuses `persist_unlocked`. Honeycomb Proposed. Oracles: `m031_wal_tail_heal.rs`.
