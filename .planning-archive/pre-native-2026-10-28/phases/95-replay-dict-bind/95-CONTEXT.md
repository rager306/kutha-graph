# Phase 95 CONTEXT — Replay dict bind

**Source:** CE full-branch review P1 #10.  
**Lease:** M035 S01 / GSD v0.29.

## Goal

`replay_check` / query-ready rebuild must `bind_reserved` from the live TermDictionary so product `as_of` excludes genesis `kutha:*`. Bare `replay` stays fingerprint-only.

## Oracles

- `bare_replay_as_of_includes_dictionary_facts`
- `replay_with_dict_as_of_matches_live`
- `replay_check_returns_bound_fold`
