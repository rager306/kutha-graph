# M022 / v0.16 — Audit residual honesty

**Date:** 2026-10-08  
**Source:** whole-repo audit “not re-verified” + stale M020 residual after M021  
**Principle:** correct docs that lie; fix confirmed product/harness bugs with RED first; leave frozen/ontology items alone.

## Triage

| Finding | Action |
|---------|--------|
| Retract CI 1-of-3 | **Docs only** — fixed M021 S02; strike from v0.14 audit residuals |
| Keyed Assert retry after retract | **Fix** — `delivery_key_retry` must require a live fact |
| `kutha:admission` delivery_key vs jid | **Characterize** — EventId path works; fallback dead; pin + ADR note, no behaviour change |
| Rollover seal/manifest window | **Defer** — documented heal; own high-risk slice later |
| jobs>1 observe without `overlap_with` | **Fail-closed** — HIGH if jobs>1 and row lacks overlap |
| pytest writes live `.kutha/` | **Fix** — `Context.skip_emit` for FSM unit tests |
| SPIKE-LAW / Pi replay / Cypher | Out of scope |

## Slices

| Slice | Phase | After this |
|-------|-------|------------|
| S01 | 71 | Stale audit claims corrected; triage recorded |
| S02 | 72 | Keyed-retry-after-retract RED→GREEN; bench ≤10%; admission pin |
| S03 | 73 | skip_emit + overlap_with fail-closed; pytest pin |
| S04 | 74 | M022 closed |
