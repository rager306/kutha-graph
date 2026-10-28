# Phase 173 Summary

RED left `seg-00000001-*.log` after a failed verify. `migrate_until` now deletes every `seg-*.log` once the manifest is gone, then syncs the directory. Legacy jsonl/WAL still open. Existing unflip oracles in `m002_seg01` and `m056_persist_heal` stayed green.
