---
phase: 26-dictionary-entries-as-facts
plan: 02
subsystem: runtime
tags: [dictionary-facts, StoreBlob, kutha-namespace]
requires: [{ phase: 26-dictionary-entries-as-facts, provides: D-08 RED as_of oracles }]
provides: [kutha: facts, StoreBlob, dictionary_as_of, EventId admissions]
affects: [M014]
actuals: { tokens: 32199, tasks: 2, commits: 2 }
plan_head_before: 7e7991273fa90222d8272c03834d7872192e05a4
plan_head_after: 1307ffef54e5a62e6004c41015c15db05037b4fb
tech-stack: { added: [], patterns: [reserved kutha: facts; StoreBlob store-once] }
key-files: { created: [], modified: [crates/kutha-common/src/event.rs, crates/kutha-runtime/src/fold.rs, crates/kutha-runtime/src/quantum.rs, crates/kutha-runtime/tests/m013_dictionary_facts.rs] }
key-decisions: ["D-02 StoreBlob only; five meta Ops removed", "D-05 product as_of excludes kutha:", "D-04 justification_id is admission EventId"]
patterns-established: ["emit_allow/emit_rule/emit_policy; never edit .kutha/STATE.md"]
requirements-completed: [SUB-01, SUB-02, SUB-03, SUB-04]
coverage:
  - { id: D1, description: "kutha: facts + StoreBlob; five meta Ops gone", requirement: SUB-01, verification: [{ kind: unit, ref: "rg five-meta-Op crates = 0", status: pass }], human_judgment: false }
  - { id: D2, description: "dictionary_as_of GREEN; named oracles keep expects", requirement: SUB-04, verification: [{ kind: unit, ref: "cargo test m013_dictionary_facts ff5 ff6 m012_*", status: pass }], human_judgment: false }
duration: 90min
completed: 2026-10-01
status: complete
---

# Phase 26 Plan 02: Dictionary entries as kutha facts Summary

**Five dictionary meta-ops are gone: reserved `kutha:` facts plus `StoreBlob`; product `as_of` excludes them, `dictionary_as_of` returns them; `justification_id` is the admission EventId.**

## Rename table (old → new)

| Old | New |
|-----|-----|
| `as_of` (26-01 policy/admission/allows) | `dictionary_as_of` |
| `emit(Op::AllowRelation {..})` | `emit_allow(name, valid_from, valid_to)` |
| `emit(Op::RegisterRule {..})` | `emit_rule(definition, valid_from, valid_to)` |
| `emit(Op::PinPolicy {..})` | `emit_policy(definition, valid_from, valid_to)` |
| `Op::RecordAdmission` / RAM buffer | `kutha:admission` Assert; jid = EventId |
| `Op::RecordAction` public emit | `kutha:action` Assert (`MetaOpRejected`) |
| `facts().len()==0` after default | product `live_count` / filtered `as_of` |

## Task commits

1. `cec5b64` feat(26-02): dictionary entries as kutha facts and StoreBlob
2. `1307ffe` feat(26-02): EventId admissions and green dictionary oracles

**Deviations:** provenance mix hashes dictionary Asserts. GitNexus 21 behind (no reindex). CBM `metadata_changed` (no `index_repository`).

## Self-Check: PASSED
