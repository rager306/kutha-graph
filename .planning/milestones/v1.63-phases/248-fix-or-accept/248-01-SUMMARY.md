---
phase: 248-fix-or-accept
plan: 01
subsystem: runtime-hot-path
tags: [as-of, dictionary, performance]
requires:
  - phase: 247-asof-bisect
    provides: "first bad commit cec5b64"
provides:
  - "dense dictionary_rel_flags"
  - "as_of 123 us at N=10000"
affects: [ingest-bisect, b3]
actuals: {tokens: 900, tasks: 5, commits: 0}
tech-stack: {added: [], patterns: [dense-term-flags]}
key-files:
  created: []
  modified: [crates/kutha-runtime/src/fold.rs, docs/architecture/benchmark-baseline.md, CHANGELOG.md]
key-decisions:
  - "Vec<bool> indexed by TermId instead of HashSet per fact."
  - "1.35x old as_of is accepted as product-filter cost."
requirements-completed: [PF-04]
coverage:
  - id: D1
    description: "as_of returns to 123 us at N=10000, not 303 us."
    requirement: PF-04
    verification:
      - kind: integration
        ref: "tmp/bench-head-fix-10k.log"
        status: pass
    human_judgment: false
---

# Phase 248 summary — fix or accept

- `dictionary_rels: HashSet<TermId>` → `dictionary_rel_flags: Vec<bool>`.
- N=10000: as_of.n 123,019 ns vs old 91,335 and pre-fix 303,355; 2N 231,191, 4N 715,674.
- Ingest unchanged at 128.5 ms; separate follow-up.
- Semantics unchanged; no Op, no ADR.
