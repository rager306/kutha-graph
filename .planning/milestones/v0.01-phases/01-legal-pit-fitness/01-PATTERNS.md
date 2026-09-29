# Phase 1: Legal PIT fitness - Pattern Map

**Mapped:** 2026-09-29
**Files analyzed:** 6 (create/modify GSD artifacts only)
**Analogs found:** 6 / 6

## Product-code verdict

**No new or modified Rust/Python product files in this phase.** CONTEXT D-08/D-09 and RESEARCH Wave 0: FIT-01…05 are already covered by existing integration tests under `crates/kutha-runtime/tests/`. Plans must not create crates, fixtures, harness assertions, or dictionary edits for Phase 1 success. Pattern map below is **documentation / verification only**.

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `.planning/phases/01-legal-pit-fitness/01-VERIFICATION.md` | config (GSD evidence SoT) | batch / transform | `.planning/phases/01-legal-pit-fitness/01-VALIDATION.md` + RESEARCH evidence table + `.kutha/dictionaries/fsm.yaml` `required` | exact (phase-local) |
| `.planning/phases/01-legal-pit-fitness/01-*-PLAN.md` | config (GSD plan) | request-response (task→must_haves) | `.planning/ROADMAP.md` Phase 1 block + `01-VALIDATION.md` sampling | role-match |
| `.planning/phases/01-legal-pit-fitness/01-*-SUMMARY.md` | config (GSD summary) | batch | `.planning/onboarding/SUMMARY.md` + VERIFICATION evidence reuse | role-match |
| `.planning/REQUIREMENTS.md` | config | batch (checkbox flip) | `.planning/REQUIREMENTS.md` itself (FIT-01…05 list) | exact |
| `.planning/STATE.md` | config | event-driven (progress / blocker) | `.planning/STATE.md` frontmatter + body | exact |
| `.planning/phases/01-legal-pit-fitness/01-VALIDATION.md` | config | batch | already present; fill Task IDs from plan | exact (self) |

**Do not create/modify (read-only evidence targets):**

| Path | Role | Note |
|------|------|------|
| `crates/kutha-runtime/tests/{ff5,ff6,m010,m011,h2,h4}_*.rs` | test | Existing FIT falsifiers — observe only |
| `scripts/kutha_gov/observe.py` | utility | Arg parity + name matching — Phase 2 owns `ci` |
| `.kutha/dictionaries/fsm.yaml` | config | Twelve `required` names — align map; do not edit for Phase 1 |
| `.kutha/STATE.md` | config | Cite freeze / Active Slice None — do not overwrite from GSD |

## Pattern Assignments

### `.planning/phases/01-legal-pit-fitness/01-VERIFICATION.md` (config, batch)

**Analog:** `.planning/phases/01-legal-pit-fitness/01-VALIDATION.md` (structure) + RESEARCH Pattern 1 twelve-row map + `.kutha/dictionaries/fsm.yaml` lines 31–43 (canonical fn names)

**Note on GSD template:** `.cursor/gsd-core/templates/verification-report.md` exists on disk but is **not git-tracked** (gsd-core install tree, 0 files under `git ls-files '.cursor/gsd-core/**'`). Do **not** cite that path as an analog in plans. Prefer tracked phase docs + `fsm.yaml` name list. Optional shape ideas (frontmatter `status` / Observable Truths) may be adapted, but Phase 1 SoT columns are CONTEXT D-04: **FIT-id | test file | test fn | pass/fail**.

**Core evidence-table pattern** (from RESEARCH / CONTEXT specifics — copy these twelve rows):

| FIT | File | Test fn | pass/fail |
|-----|------|---------|-----------|
| FIT-01 | `crates/kutha-runtime/tests/ff5_legal_pit.rs` | `ff5_as_of_t1_differs_from_as_of_t2_on_statute_log` | |
| FIT-02 | `crates/kutha-runtime/tests/ff6_allowlist.rs` | `ff6_unknown_relation_does_not_append` | |
| FIT-03 | `…/m010_semantic_open.rs` | `open_without_snapshot_recovers_intern_meanings` | |
| FIT-03 | `…/m010_semantic_open.rs` | `open_without_snapshot_or_terms_file_recovers_from_define_ops` | |
| FIT-03 | `…/m010_semantic_open.rs` | `intern_appends_define_for_new_terms_only` | |
| FIT-04 | `…/m011_claim_supports.rs` | `retracting_one_support_leaves_claim_supported` | |
| FIT-04 | `…/m011_claim_supports.rs` | `unknown_claim_does_not_append` | |
| FIT-04 | `…/m011_claim_supports.rs` | `replay_rejects_behavior_without_prior_cause` | |
| FIT-04 | `…/m011_claim_supports.rs` | `derived_q_loses_eligibility_when_last_premise_support_withdrawn` | |
| FIT-05 | `…/h2_harness_tenant.rs` | `h2_harness_status_as_of_t1_differs_from_as_of_t2` | |
| FIT-05 | `…/h2_harness_tenant.rs` | `h2_same_second_status_as_of_uses_emitted_cut` | |
| FIT-05 | `…/h4_process_allows.rs` | `h4_prior_cut_keeps_status_membership_after_later_edition_drops_it` | |

**Canonical name list** (`.kutha/dictionaries/fsm.yaml` lines 31–43) — VERIFICATION rows must match exactly:

```yaml
required:
  - ff5_as_of_t1_differs_from_as_of_t2_on_statute_log
  - ff6_unknown_relation_does_not_append
  - h2_harness_status_as_of_t1_differs_from_as_of_t2
  - h2_same_second_status_as_of_uses_emitted_cut
  - h4_prior_cut_keeps_status_membership_after_later_edition_drops_it
  - open_without_snapshot_recovers_intern_meanings
  - open_without_snapshot_or_terms_file_recovers_from_define_ops
  - intern_appends_define_for_new_terms_only
  - retracting_one_support_leaves_claim_supported
  - unknown_claim_does_not_append
  - replay_rejects_behavior_without_prior_cause
  - derived_q_loses_eligibility_when_last_premise_support_withdrawn
```

**Hard-gate command block** (also record exit code + timestamp in VERIFICATION):

```bash
cargo test --workspace --offline
```

**Pass/fail interpretation pattern** (`scripts/kutha_gov/observe.py` lines 16–42) — Phase 1 humans/agents apply the same semantics without running `kutha-gov ci`:

```python
ok_pat = re.compile(rf"^test (?:[\w:]+::)?{escaped} \.\.\. ok\b", re.MULTILINE)
fail_pat = re.compile(rf"^test (?:[\w:]+::)?{escaped} \.\.\. FAILED\b", re.MULTILINE)
# ok → pass; FAILED → fail (D-06/D-07 stop); else → missing (treat as red / blocker)
```

**VALIDATION frontmatter / status tokens to mirror** (`01-VALIDATION.md` lines 1–8, 48):

```yaml
---
phase: "01"
slug: "legal-pit-fitness"
status: draft   # after verify: passed | gaps_found (or align VERIFICATION status)
---
# Status cells: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky
```

**Anti-pattern:** Do not treat filtered `cargo test --test ff5_…` as the gate (D-01). Optional diagnostic only:

```bash
cargo test --workspace --offline -- --list | rg '<exact_fn_name>'
```

---

### `.planning/phases/01-legal-pit-fitness/01-*-PLAN.md` (config, request-response)

**Analog:** `.planning/ROADMAP.md` Phase 1 goal/requirements/success criteria (lines 28–36) + `01-VALIDATION.md` sampling rates (lines 29–34) + `.planning/codebase/TESTING.md` tooling gate matrix (lines 253–262)

**Must_haves pattern** (copy into plan frontmatter / tasks):

1. Hard gate command string exactly: `cargo test --workspace --offline` (D-01)
2. Evidence map of twelve FIT fns in VERIFICATION/SUMMARY (D-02) — not cargo `--exact`
3. Hard stop on any red named FIT (D-06/D-07) — record failing fn in `.planning/STATE.md`
4. **Not** must_have: `uv run kutha-gov ci` / `precommit` (D-03 → Phase 2)
5. **Not** must_have: product code, new H2/H4 assertions, REQUIREMENTS mid-task edits

**ROADMAP Phase 1 success criteria** (lines 28–36) — plan truths should map 1:1 to FIT-01…05:

```markdown
### Phase 1: Legal PIT fitness
**Goal**: A developer can run the named product fitness suite and observe legal point-in-time, …
**Requirements**: FIT-01, FIT-02, FIT-03, FIT-04, FIT-05
**Success Criteria** (what must be TRUE):
  1. … as_of(2015) and as_of(2021) … (FF5)
  2. … unknown product relation … (FF6)
  …
```

**Sampling pattern** (`01-VALIDATION.md` lines 29–34):

```markdown
- **After every task commit:** `cargo test --workspace --offline -q`
- **After every plan wave:** `cargo test --workspace --offline`
- **Before `/gsd-verify-work`:** Full suite green + FIT evidence map complete
```

**Tooling gate matrix** (`.planning/codebase/TESTING.md` lines 255–261) — Phase 1 uses **cargo row only**:

| Command | Cargo | Dict checks | JSONL emit |
|---------|-------|-------------|------------|
| `cargo test --workspace` | yes | no | no |
| `uv run kutha-gov ci` | observe | yes | yes | ← Phase 2 |

---

### `.planning/phases/01-legal-pit-fitness/01-*-SUMMARY.md` (config, batch)

**Analog:** `.planning/onboarding/SUMMARY.md` (short GSD summary shape) + same FIT evidence table as VERIFICATION

**Core pattern** (`onboarding/SUMMARY.md` lines 1–26) — keep terse, English, cite harness lease:

```markdown
# … Summary
## Project State
- …
## Recommended Next Step
- Practical steel thread: …
- Do not thaw freeze. …
```

**Coverage / requirements-completed:** Prefer listing `FIT-01`…`FIT-05` with verification refs pointing at the twelve fns (D-02). Reuse VERIFICATION table rather than inventing a second map.

**Untracked template note:** `.cursor/gsd-core/templates/summary.md` is also untracked gsd-core — same rule as verification-report; do not hard-depend on that path. If a SUMMARY needs frontmatter, keep fields minimal: `phase`, `requirements-completed: [FIT-01…FIT-05]`, `status`, `completed`.

---

### `.planning/REQUIREMENTS.md` (config, batch)

**Analog:** self — Fitness section lines 12–18

**Checkbox pattern** (D-05 — **one batch at verify/complete only**):

```markdown
### Fitness (L_capability)

- [ ] **FIT-01**: … (`ff5_as_of_t1_differs_from_as_of_t2_on_statute_log`)
- [ ] **FIT-02**: … (`ff6_unknown_relation_does_not_append`)
- [ ] **FIT-03**: …
- [ ] **FIT-04**: …
- [ ] **FIT-05**: …
```

After VERIFICATION all twelve rows pass → flip FIT-01…05 to `[x]` in one edit. Do **not** flip GOV-* / PLANE-* / FREEZE-* (Phase 2+). Do not edit mid-task (D-04).

---

### `.planning/STATE.md` (config, event-driven)

**Analog:** self — YAML frontmatter lines 1–17 + body Current Position / Blockers

**Progress update pattern** (on green complete):

```yaml
status: …          # e.g. verifying → complete for phase 1
stopped_at: …
last_activity_desc: Phase 1 VERIFICATION green; FIT-01…05 evidence map filled
progress:
  completed_phases: 1   # when phase closed
  …
```

**Blocker pattern on red** (D-07) — under `### Blockers/Concerns`:

```markdown
- Phase 1 hard gate red: `<exact_failing_test_fn>` under `cargo test --workspace --offline`. Stopped per D-06/D-07. No quarantine/`#[ignore]`.
```

**Guard:** Cite `.kutha/STATE.md` lease; **never overwrite** harness STATE from GSD (`.planning/STATE.md` lines 28–30).

---

### `.planning/phases/01-legal-pit-fitness/01-VALIDATION.md` (config, batch)

**Analog:** self (already seeded from RESEARCH)

**Planner fill pattern** (lines 40–46): replace `01-*-*` Task IDs once PLAN exists; leave Status ⬜ until execute/verify paints ✅/❌. Do not invent new test files in Wave 0 (lines 52–54).

## Shared Patterns

### Hard gate = observe arg parity
**Source:** `scripts/kutha_gov/observe.py` lines 133–141; `.kutha/dictionaries/fsm.yaml` lines 23–26  
**Apply to:** PLAN must_haves, VALIDATION sampling, VERIFICATION command record

```python
default_args=["test", "--workspace", "--offline"],
```

```yaml
args: ["test", "--workspace", "--offline"]
```

### Evidence map ≠ CLI filter
**Source:** CONTEXT D-02; RESEARCH Pattern 1; `fsm.yaml` `required`  
**Apply to:** VERIFICATION, SUMMARY coverage, PLAN acceptance  
Named FIT selection is a **table of twelve fns**, not `cargo test --exact` / `--test <binary>`.

### Hard-fail / no quarantine
**Source:** CONTEXT D-06/D-07  
**Apply to:** PLAN, VERIFICATION Gaps, STATE blockers  
Any FAILED or missing named FIT → phase incomplete; record fn name; no `#[ignore]`.

### Batch REQUIREMENTS hygiene
**Source:** CONTEXT D-04/D-05  
**Apply to:** execute vs verify steps  
No mid-task `[x]` on FIT; single batch after VERIFICATION green.

### Freeze / no product expansion
**Source:** `.kutha/STATE.md` lines 7–31; CONTEXT D-08/D-09  
**Apply to:** all plans  
Active Slice None; no legal pack, Rocks, Cypher, HNSW, new H2/H4 assertions.

### Tooling surfaces (Phase 1 vs 2)
**Source:** `.planning/codebase/TESTING.md` lines 253–262  
**Apply to:** PLAN must_haves  
Phase 1 = cargo workspace offline. Phase 2 = `kutha-gov ci` / observe-required.

## No Analog Found

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| — | — | — | No product files; all GSD artifacts have tracked analogs. Untracked gsd-core templates explicitly excluded per tracked-source gate. |

## Metadata

**Analog search scope:** `.planning/` (phases, REQUIREMENTS, STATE, ROADMAP, codebase/TESTING, onboarding), `scripts/kutha_gov/observe.py`, `.kutha/dictionaries/fsm.yaml`, `.kutha/STATE.md`; Glob `*VERIFICATION*` / `*TESTING*`; CBM `list_projects` (kutha-graph indexed — not required for docs mapping)
**Files scanned:** ~25 candidate paths; 8 tracked analogs read
**Tracked-source gate:** Confirmed `git ls-files` for TESTING.md, observe.py, fsm.yaml, REQUIREMENTS.md, STATE.md, ROADMAP.md, onboarding/SUMMARY.md, 01-VALIDATION.md; rejected `.cursor/gsd-core/**` (0 tracked files)
**Pattern extraction date:** 2026-09-29
**CBM:** `list_projects` → `kutha-graph` present; structural graph optional for this docs-only phase
