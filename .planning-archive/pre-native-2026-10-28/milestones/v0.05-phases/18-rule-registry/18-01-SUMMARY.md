---
phase: 18-rule-registry
plan: 01
subsystem: runtime
tags: [rule-registry, event-log, behavior, RULE-01, RULE-02, RULE-03]

requires:
  - phase: 17-allowlist-as-log-facts
    provides: Op::AllowRelation log-native dictionary facts and skip-serialized fold entries
provides:
  - "Op::RegisterRule log facts that do not mint graph Facts"
  - "rule_definition_hash and GraphFold::rule_hash_live_at"
  - "User emit(Behavior) and derivation_eligible_at fail closed unless the pin is a live hash"
affects: [18-02, 18-03, emit, eligibility]

actuals:
  tokens: 7310
  tasks: 3
  commits: 2
  plan_head_before: 7c78391143afc59c9e36e4fcfc26a4f49baa54f5
  plan_head_after: 7e00bc741b20f3c9e362098d26f3200465da8914

tech-stack:
  added: []
  patterns:
    - "RegisterRule mirrors AllowRelation: dedicated Op, skip-serialized fold entries, emit without QuantumOutcome sidecar"
    - "User Behavior.rule_version must equal a live registry hash; follow_ons inverse_knows keeps an empty pin"

key-files:
  created:
    - crates/kutha-runtime/tests/m012_rule_registry.rs
  modified:
    - crates/kutha-common/src/event.rs
    - crates/kutha-common/src/lib.rs
    - crates/kutha-runtime/src/fold.rs
    - crates/kutha-runtime/src/quantum.rs
    - crates/kutha-runtime/tests/m011_provenance.rs
    - crates/kutha-runtime/tests/m011_e2e_fixture.rs
    - crates/kutha-runtime/tests/m011_claim_supports.rs
    - crates/kutha-runtime/tests/m012a_log_native.rs
    - crates/kutha-runtime/tests/m012a_stable_refs.rs

key-decisions:
  - "RegisterRule is a dedicated Op, not Assert/Define/Behavior, so the registry is a log fact"
  - "rule_definition_hash is SHA-256 of kutha-rule-def plus UTF-8 definition bytes; the Op does not store a precomputed hash"
  - "User emit(Behavior) pin check does not run on follow_ons inverse_knows"

patterns-established:
  - "Rule-entries skip-serialize; hydrate_from_log rebuilds them from the log walk"
  - "UnknownRuleVersion fails closed and does not grow the log"

requirements-completed: [RULE-01, RULE-02, RULE-03]

coverage:
  - id: D1
    description: "Op::RegisterRule appends as a log fact; Behavior.rule_version equals the definition hash"
    requirement: RULE-01
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_rule_registry.rs#register_rule_pins_behavior_to_definition_hash
        status: pass
    human_judgment: false
  - id: D2
    description: "Unknown or mismatched rule_version fails closed on emit"
    requirement: RULE-02
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_rule_registry.rs#unknown_or_mismatched_rule_version_fails_closed
        status: pass
    human_judgment: false
  - id: D3
    description: "Free-string and empty pins fail closed; empty RegisterRule definition does not append"
    requirement: RULE-03
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_rule_registry.rs#free_string_rule_version_does_not_append
        status: pass
    human_judgment: false
  - id: D4
    description: "M011 provenance/e2e/claim-supports and M012a LOG/REF Behavior fixtures pin derive_pq; ALL/FF6/FF5 stay green"
    requirement: RULE-03
    verification:
      - kind: unit
        ref: crates/kutha-runtime/tests/m011_provenance.rs#provenance_detects_caused_by_swap_when_state_fingerprint_matches
        status: pass
      - kind: unit
        ref: crates/kutha-runtime/tests/m012_allowlist_facts.rs#allow_relation_appends_as_log_fact
        status: pass
    human_judgment: false

duration: 7min
completed: 2026-10-01
status: complete
---

# Phase 18 Plan 01: Rule registry tracer Summary

**Log-native RegisterRule pins Behavior.rule_version to rule_definition_hash; unknown, mismatched, and free-string pins fail closed**

## Performance

- **Duration:** 7 min
- **Started:** 2026-10-01T02:17:39Z
- **Completed:** 2026-10-01T02:24:17Z
- **Tasks:** 3
- **Files modified:** 10

## Accomplishments

- `Op::RegisterRule { definition, valid_from, valid_to }` hashes tag `register-rule`; `rule_definition_hash` is SHA-256 of `kutha-rule-def` plus UTF-8 bytes.
- `GraphFold` skip-serializes rule-entries; `rule_hash_live_at` uses the AllowEntry liveness shape; `hydrate_from_log` rebuilds entries from the log.
- User `emit(Behavior)` and `derivation_eligible_at` require a live hash. Empty RegisterRule definitions return `UnknownRuleVersion` with an empty pin and do not append.

## Task Commits

1. **Task 1–2: RegisterRule tracer plus fail-closed pins** - `9bb8335` (feat)
2. **Task 3: Prior Behavior fixtures pin rule_definition_hash of derive_pq** - `7e00bc7` (test)

**Plan metadata:** pending docs commit

_Note: TDD_MODE was false; exhaustive `Op` matches make an isolated RED uncompilable. Fail-closed tests landed with the tracer feat commit._

## Files Created/Modified

- `crates/kutha-common/src/event.rs` - `Op::RegisterRule`, `rule_definition_hash`, `new` / `digest_bytes` arms
- `crates/kutha-common/src/lib.rs` - export `rule_definition_hash`
- `crates/kutha-runtime/src/fold.rs` - skip-serialized rule-entries, `rule_hash_live_at`, hydrate rebuild
- `crates/kutha-runtime/src/quantum.rs` - public emit of RegisterRule, user pin check, eligibility clause, `UnknownRuleVersion`
- `crates/kutha-runtime/tests/m012_rule_registry.rs` - RULE-01/02/03 named oracles
- `crates/kutha-runtime/tests/m011_provenance.rs` - RegisterRule then hashed pin
- `crates/kutha-runtime/tests/m011_e2e_fixture.rs` - RegisterRule then hashed pin and justification
- `crates/kutha-runtime/tests/m011_claim_supports.rs` - RegisterRule on emit path only
- `crates/kutha-runtime/tests/m012a_log_native.rs` - RegisterRule then hashed pin
- `crates/kutha-runtime/tests/m012a_stable_refs.rs` - RegisterRule then hashed pin

## Decisions Made

- Dedicated `Op::RegisterRule` rather than `Assert`/`Define`/`Behavior` so registry rows are log facts, not graph Facts.
- `emit(RegisterRule)` does not append a `QuantumOutcome` sidecar, matching the tracer contract that log length grows by 1.
- Pin check is on the user `emit(Behavior)` op only; `follow_ons` inverse_knows still uses an empty pin (Phase 21).

## Deviations from Plan

### Auto-fixed Issues

None - plan executed as written except commit batching below.

### Commit batching

**1. [Process] Tasks 1 and 2 share one feat commit**
- **Found during:** Task 1 (tracer pin check)
- **Issue:** Tracer eligibility requires the live-hash gate; fail-closed tests belong in the same file.
- **Fix:** Landed all three named tests with the RegisterRule implementation in `9bb8335`; Task 3 fixtures are a separate test commit.
- **Files modified:** `crates/kutha-runtime/tests/m012_rule_registry.rs`
- **Commit:** `9bb8335`

---

**Total deviations:** 1 process (commit batching)
**Impact on plan:** No behavior change. Retract UnknownFact gate and persist/open remain Plan 18-02.

## Issues Encountered

`cargo fmt --all` reformatted `ff6_allowlist.rs` and `m010_semantic_open.rs`; those files were restored and not committed.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Tracer green: RegisterRule is a log fact; hashed Behavior is eligible; unknown/mismatched/free-string pins fail closed; M011/M012a/ALL/FF6/FF5 named tests pass.
- Plan 18-02 should version rule-entries via Retract EventId and persist/open reconstruction.
- Do not register fsm.yaml names until Plan 18-03. Do not clear the S02 lease.

## Self-Check: PASSED

- FOUND: crates/kutha-common/src/event.rs
- FOUND: crates/kutha-common/src/lib.rs
- FOUND: crates/kutha-runtime/src/fold.rs
- FOUND: crates/kutha-runtime/src/quantum.rs
- FOUND: crates/kutha-runtime/tests/m012_rule_registry.rs
- FOUND: 9bb8335
- FOUND: 7e00bc7
