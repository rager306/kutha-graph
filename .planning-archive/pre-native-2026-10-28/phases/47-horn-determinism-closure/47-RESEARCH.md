# Phase 47: Horn determinism and closure — Research

**Researched:** 2026-10-06 | **Domain:** Horn cascade / derived EventId | **Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-DET-02 hash-stable derived ids:** User-emitted events stay UUID v7 (`Event::new`). Derived cascade events (`Op::Behavior`, follow-on `Retract`) mint id as first 16 bytes of SHA-256 over a Kutha prefix plus rule pin, `caused_by`, binding TermIds, and op discriminant — same pattern as `Event::stable_define`. Two identical ingest sequences yield the same derived ids. Do not change persisted user v7 ids.
- **D-HORN-CORR re-derive:** `rule_follow_ons` treats `Correct` and `CorrectInterval` like a premise change: `retract_derived` of the targeted event_id, then `horn_fires` on the corrected triple. Stale derived Behavior must not stay live at the post-correct cut. Invert `horn_correct_char_*` (characterization → killed of the new rule). Keep FF5 / M010–M017 named oracles except those characterization pair.
- **D-CASC admit hops:** every event committed in `commit_user_cascade` (and Lazy equivalent) passes `admit` before commit, not only the first event’s follow-ons. Unknown head or premise relation on a firing Horn fails closed (`RuntimeError`), not `continue`. Keep existing LIFO pending order unless SEM-cascade-order forces a documented swap.
- **D-SCOPE:** honeycomb **Proposed**. Controller owns `.kutha/STATE.md`. Verifier writes `47-VERIFICATION.md`. TR-02 / C01 pack stay out.

### Claude's Discretion
None — CONTEXT.md has no such section.

### Deferred Ideas (OUT OF SCOPE)
Phase 48 ORACLE-01 / GATE-01. UUID v7 for derived events (rejected). Fifth clocks.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| DET-02 | derived-event identity is deterministic from premise ids, rule pin and binding | Clone `Event::stable_define`; keep `Event::new` = `Uuid::now_v7()` |
| HORN-CORR-01 | `Correct`/`CorrectInterval` re-derive; stale derived rows do not stay live | Extend `rule_follow_ons` `_ => vec![]`; invert `horn_correct_char_*` after RED |
| CASC-01 | every hop `admit`; stable order; unknown Horn relation fail-closed | Admit before `pending.push`; `UnknownRelation` on firing head; intern-later `continue` |
</phase_requirements>

## Summary
HEAD mints derived ids with `Event::new` / `Uuid::now_v7()`; `rule_follow_ons` ignores Correct (`_ => vec![]`); `commit_user_cascade` admits only the first event’s follow-ons; `dict.id` None is `continue`/`?`. **Primary recommendation:** add `Event::stable_derived` next to `stable_define`; Correct arm = `retract_derived` then `horn_fires` on fold subject/relation + new object/interval; admit every follow-on before push; fail-closed only on a **matched firing** Horn whose head is missing/not allowed.

## Architectural Responsibility Map
| Capability | Primary Tier | Secondary | Rationale |
|------------|--------------|-----------|-----------|
| Derived EventId | API / Backend (`kutha-common` Event) | — | Hash at mint, not persist |
| Correct re-derive | API / Backend (`eval.rs`) | Storage fold apply already invalidates | Follow-ons before commit |
| Hop admit | API / Backend (`pipeline.rs` + `admit`) | — | Same User path for Eager/Lazy |
| Dictionary intern | API / Backend (`Runtime::intern`) | — | Eval is `&self`; no silent intern |

## Project Constraints (from .cursor/rules/)
CBM graph-first (`root-kutha-graph`); never `delete_project` / `index_repository` / `manage_adr`. Chat RU / docs EN. Honeycomb Proposed ≠ delivery. `cargo test --workspace`. No LifeOS. GitNexus not this phase. Nyquist off (`workflow.nyquist_validation: false`).

## Standard Stack
No new crates. Reuse workspace `sha2 = "0.10"` lock **0.10.9**, `uuid` **1.24.1** features `v7` (`Cargo.toml` 12–13; `Cargo.lock`). Analog: `h.update(b"kutha-define-id");` … `id_bytes.copy_from_slice(&digest[..16]);` `Uuid::from_bytes` [VERIFIED: crates/kutha-common/src/event.rs:288-304]. User path stays `id: Uuid::now_v7()` [VERIFIED: event.rs:278-279]. Tests: `cargo test -p kutha-runtime --offline -- m019_sem_meta m017_determinism m016_rule_eval m013_write_path`.

## Package Legitimacy Audit
No installs. None removed/SUS.

## Architecture Patterns
**Eager vs Lazy:** User writes always `return self.commit_user_cascade(op);` [VERIFIED: pipeline.rs:197-202]. Lazy is fold leases only — do not fork a second cascade.

**DET-02:** New constructor (do not change `Event::new`). Preimage: distinct prefix (not `kutha-define-id`) ‖ pin ‖ `caused_by` bytes ‖ TermId bindings (Behavior: s,r,o; Retract: retracted id) ‖ op tag. Call from `head_event` and `retract_derived` instead of `Event::new`. `commit_user_cascade` first event stays `Event::new`.

**HORN-CORR:** Match `Op::Correct { event_id, object, valid_from, valid_to }` / `Op::CorrectInterval { event_id, object, patch_from, patch_to }` [VERIFIED: event.rs:184-196]. Lookup s,r from fold by `event_id` (pre-commit old row). `retract_derived(trigger, *event_id)` then `horn_fires(trigger.id, ingested_at, s, r, *object, vf, vt)`. Fold already invalidates on apply [VERIFIED: fold.rs:703-733].

**CASC admit:** Today only `for follow in self.rule_follow_ons(&first) { self.admit(&follow.op, ReceiptMode::User)?; }` then LIFO `pending.pop()` without hop admit [VERIFIED: pipeline.rs:243-266]. Copy `pipeline_events` pre-commit admit: after `rule_follow_ons(&event)`, `admit` each follow **before** `pending.push`. Keep LIFO unless `cascade_order_eager_lazy_inverse_knows` fails.

**Fail-closed vs intern-later:** `nary_horn_fires_when_both_premises_live` does `emit_rule_ast` **then** `intern("relatedTo")` [VERIFIED: m016_rule_eval.rs:36-40]. `live_horns_scan` can see that Horn while `dict.id(&prem.relation)` is None. Keep `continue` on None for **unmatched** premises (`horn_fires` 110-112, `search_at` 329). Error `RuntimeError::UnknownRelation` only when a premise **matched** the trigger (`prem_rel == relation`) and `head.relation` is missing or `admit` would deny. Do not `intern_silent` in eval (breaks `intern_appends_define_for_new_terms_only`). Do not fail `emit_rule_ast` for uninterned AST names.

## Don't Hand-Roll
| Problem | Don't | Use |
|---------|-------|-----|
| 16-byte id | New hash crate / UUID v5 | Existing `Sha256` + `Uuid::from_bytes` |
| Allow check | Parallel eval allowlist | `admit` + `UnknownRelation { name }` [VERIFIED: admission.rs:151-157; quantum.rs:55-57] |
| Cascade queue | Extra Lazy pipeline | Same `commit_user_cascade` |

## Common Pitfalls
1. **Invert char before RED** — keep `horn_correct_char_*` failing on HEAD, then flip asserts (old inverse dead; new object derived or none; superseded premise not live stays). Matrix `SEM-horn-correct-char` `status: characterization` → `killed`; fill `SEM-horn-correct` / `SEM-det02-ids` oracles (today `oracle: []` `blocked`).
2. **Admit after `commit_event`** — failing hop must never hit the log (verifier).
3. **Fingerprint vs derived ids** — `m017_determinism.rs` still compares sorted triples; user v7 ids still differ.
4. **`u64::MAX` in `search_prems`** — CLOCK-01; do not touch.
5. **Partial quantum** — admit-before-push; no rollback of earlier hops unless tests demand it.

## Code Examples
`stable_define` pattern above. `rule_follow_ons` Retract arm: `Op::Retract { event_id } => self.retract_derived(event, *event_id)` [VERIFIED: eval.rs:54]. Char tests: `horn_correct_char_derived_survives_premise_correct` 429, `horn_correct_interval_char_derived_survives_patch` 486 [VERIFIED: m019_sem_meta.rs].

## Assumptions Log
| # | Claim | Risk |
|---|-------|------|
| A1 | Prefix bytes `kutha-derived-id` (CONTEXT did not pin the exact string) | Collision with `kutha-define-id` if reused |
| A2 | Op tag is one ASCII byte `B`/`R` | Cross-run mismatch if planner picks another encoding |

## Open Questions
None blocking. Exact prefix/tag = A1/A2; default as assumed.

## Security Domain
V5 Input: `admit` + `UnknownRelation` (no new parse). V2/V3/V6 n/a. Threat: skip-admit forges derived triples — mitigate hop `admit`. Do not intern untrusted names in eval.

## Sources
HIGH: CBM `root-kutha-graph` snippets + Read eval.rs/pipeline.rs/event.rs/admission.rs/fold.rs/quantum.rs/m019_sem_meta.rs/m016_rule_eval.rs/test-matrix.yaml. Coverage: tests `m019_sem_meta.rs` `not_tracked`; `test-matrix.yaml` `metadata_changed` (no reindex). MEDIUM: 47-PATTERNS.md. Graphify disabled.

## Metadata
Stack/architecture/pitfalls: HIGH (source Read). Valid 30d. Nyquist skipped.
