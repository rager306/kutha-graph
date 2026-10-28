# Phase 3: Lease-gated next slice - Pattern Map

**Mapped:** 2026-09-29
**Files analyzed:** 8 (GSD evidence + optional ROADMAP D-G3 captions; **no product crates**)
**Analogs found:** 8 / 8

## Product-code verdict

**No new or modified Rust product files for phase success.** CONTEXT D-L1…D-L6 and RESEARCH: GOV-03 / NEXT-01 / NEXT-02 are proven by **governor CI + path/git probes + evidence tables**. Plans must not start a named Active Slice, close M011 in harness STATE, thaw freeze, start a legal pack, assume M002, or implement honeycomb. Pattern map is **verification / process-docs / optional Overview caption clarity** only.

Optional D-G3 (only if probes or `ci` HIGH expose caption drift): `.planning/ROADMAP.md` Overview lines 5 and 9 so they match D-L1 (verification-only close under Active Slice None; product slice delivery waits for a **new** leased phase). Do **not** rewrite `.kutha/STATE.md`. Do not add Python Check subclasses. Do not add crate deps.

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `.planning/phases/03-lease-gated-next-slice/03-VERIFICATION.md` | config (GSD evidence SoT) | batch / transform | `.planning/phases/02-honest-harness-and-freeze/02-VERIFICATION.md` + `03-RESEARCH.md` probe catalog | exact (phase-local evolve) |
| `.planning/phases/03-lease-gated-next-slice/03-VALIDATION.md` | config (Nyquist) | batch | `.planning/phases/02-honest-harness-and-freeze/02-VALIDATION.md` + `03-RESEARCH.md` § Validation Architecture | exact |
| `.planning/phases/03-lease-gated-next-slice/03-*-PLAN.md` | config (GSD plan) | request-response | `02-01-PLAN.md` / `02-02-PLAN.md` / `02-03-PLAN.md` + ROADMAP Phase 3 block | exact (wave shape) |
| `.planning/phases/03-lease-gated-next-slice/03-*-SUMMARY.md` | config (GSD summary) | batch | `02-01-SUMMARY.md` § Trajectory (D-10) + D-L4 lease snapshot | exact + D-L4 extension |
| `.planning/REQUIREMENTS.md` | config | batch (checkbox flip) | self — GOV-03 / NEXT-01 / NEXT-02 (lines 24, 38–39, 104–106) + `02-03-PLAN.md` batch hygiene | exact |
| `.planning/STATE.md` | config | event-driven | self — frontmatter + cite-only harness lease (lines 29–31) | exact |
| `.planning/ROADMAP.md` | config | batch | self — Overview (lines 5, 9) D-G3 + Phase 3 Plans/Progress (lines 74–95) | exact |
| `.kutha/dictionaries/{checks,invariants,bridges,fsm,honeycomb}.yaml` | config (harness) | event-driven (policy) | `docs/process/governor-intake.md` + existing rows | role-match (edit-only if HIGH/drift) |

**Do not create/modify (read-only evidence / cite targets):**

| Path | Role | Note |
|------|------|------|
| `.kutha/STATE.md` | config (harness lease) | Cite `**Active Slice:** None`; Next action legal-pack forbid; **never overwrite from GSD** (D-L5/D-L6) |
| `crates/kutha-*` | product | Quiescence probes only (D-L2); no slice delivery |
| `scripts/kutha_gov/*.py` | utility | Observe/decide already correct; Wave 0 needs no new tests |
| FIT / GOV-01/02 / PLANE / FREEZE / MAP REQUIREMENTS | config | Leave Phase 1–2 `[x]`; flip **only** GOV-03 / NEXT-01 / NEXT-02 at closeout |

## Pattern Assignments

### `.planning/phases/03-lease-gated-next-slice/03-VERIFICATION.md` (config, batch)

**Analog:** `02-VERIFICATION.md` (structure + hard-gate + Trajectory + WARN + closeout) + RESEARCH § Concrete probe catalog (replace GOV/PLANE/FREEZE/MAP rows)

**Frontmatter / status tokens** (mirror `02-VERIFICATION.md` lines 1–30; adapt phase ids):

```yaml
---
phase: 03-lease-gated-next-slice
verified: <ISO-UTC>
status: draft   # → passed | gaps_found
# covered_files: phase plans/summaries + REQUIREMENTS + STATE + ROADMAP (cite .kutha/STATE.md; do not list crate feature files)
---
```

**Hard-gate block** — inherit Phase 2 primary gate (`02-VERIFICATION.md` lines 55–66). Do not weaken to `precommit` (D-L3 / D-G1).

| Field | Value |
|-------|-------|
| **Command** | `uv run kutha-gov ci` |
| **Exit code** | `0` (HIGH-free) |
| **Summary line** | `harness: N HIGH, M LOW, … checks` (from `cmd_ci`) |
| **Trajectory** | `uv run kutha-gov explain trajectory` — ≤8 lines + authority sentence |
| **WARN ledger** | Required iff LOW > 0 (D-11): `check_id \| category \| one-line note` |
| **Cargo smoke (D-15)** | Explicit `cargo test --workspace --offline` at tracer + pre-verify |
| **Harness cite** | Active Slice **None**; M011; freeze until M002 (`.kutha/STATE.md` — not edited) |
| **D-L4** | Wave-open + wave-close Active Slice still `None`; if named `S##` → status blocker, **do not** implement |

**ci print / exit semantics** (`scripts/kutha_gov/__main__.py` lines 174–192):

```python
print(
    f"\nharness: {outcome.high} HIGH, {outcome.low} LOW, "
    f"{len(outcome.results)} checks  ({rung} dogfood)"
)
if outcome.terminal != "ok":
    return 1
return 0
```

Do **not** enable `--fail-on-warn` as the Phase 3 gate (D-11 inherit).

**explain authority** (`scripts/kutha_gov/__main__.py` lines 123–131):

```python
print("authority: none — harness does not accept ADRs or claim product readiness")
```

**Probe evidence table** — copy column shape from `02-VERIFICATION.md` lines 107–112 (`Req | Probe | Exit | Result | pass/fail`). Seed rows from `03-RESEARCH.md` lines 238–261 (do not reuse Phase 2 GOV-01/PLANE rows as success criteria):

| Req | Probe | Pass criterion |
|-----|-------|----------------|
| GOV-03 | `rg -n '^\*\*Active Slice:\*\*' .kutha/STATE.md` | Line is `**Active Slice:** None` |
| GOV-03 | `uv run kutha-gov precommit --check trajectory` | Exit 0; output includes `no Active Slice` |
| GOV-03 | `git status --porcelain -- crates/` | Empty during Phase 3 waves |
| GOV-03 | `git log --oneline e77132d9275bd36ea766b8bef9cff28128dfc636..HEAD -- crates/` | Empty (S03 tip); if non-empty, re-baseline hash per RESEARCH A1 + still require None + D-L2 intent |
| GOV-03 | VERIFICATION prose | No named Active Slice delivery; GSD/docs/harness-only diffs OK (D-L2) |
| NEXT-01 | Cite `.kutha/STATE.md` Next action | Contains `Do **not** start a legal pack` |
| NEXT-01 | absent pack trees | `packs/legal`, `legal-corpus`, `corpus`, `crates/kutha-legal`, `crates/kutha-pack` absent |
| NEXT-01 | `precommit --check h4-lease` | Exit 0; `high=0` |
| NEXT-01 | honeycomb ADR-090 | `map: Proposed`, `delivery: frozen`, `freeze_as: "full ADR-090 ontology"` |
| NEXT-01 | ADR-090 Status | `**Proposed**` |
| NEXT-01 | Cargo freeze + `precommit --check freeze` | No rocksdb/hnsw/cypher in workspace Cargo.toml; freeze OK |
| NEXT-01 | `rg -n 'Do not plan ADR-010' .planning/ROADMAP.md` | Hit (no phase-per-cell) |
| NEXT-02 | Active Milestone / L_delivery | `M011`; `L_delivery=M011-S03-done` (not `*-closed`) |
| NEXT-02 | `rg -n 'not assumed to be M002' .planning/ROADMAP.md` | Hit |
| NEXT-02 | anti-implication rg | No prose that next product work **is** M002 / “implement honeycomb” (fencing sentences OK) |
| NEXT-02 | `precommit --check honeycomb-map` | Exit 0 |
| NEXT-02 | SUMMARY/VERIFICATION sentence | Next milestone = whatever STATE names after M011 close |
| D-L4 | Active Slice rg open+close | Both `None`; named → hard stop recorded |
| D-L3 | `uv run kutha-gov ci` | Exit 0; `0 HIGH`; green≠Accepted≠L_capability |
| D-10 | `explain trajectory` | Exit 0; authority none |
| D-15 | `cargo test --workspace --offline` | Exit 0 at tracer + pre-verify |

**Trajectory excerpt seed** (reuse live shape from `02-VERIFICATION.md` lines 74–85):

```text
check: trajectory
purpose: Active Milestone/Slice are None or exist on ROADMAP
authority: none — harness does not accept ADRs or claim product readiness
```

**WARN (LOW) ledger** — copy `02-VERIFICATION.md` lines 99–105. Empty table when LOW=0.

**Closeout table analog** (`02-VERIFICATION.md` lines 135–156): swap Phase 2 checkbox list for GOV-03/NEXT `[x]`; keep `.kutha/STATE.md` untouched + Active Slice None; add D-L4 still-None row.

**Anti-patterns:** Do not wait for a future lease (D-L1); do not treat any `crates/` whitespace as GOV-03 fail without D-L2 intent; do not close M011 or scaffold Rocks to “satisfy” NEXT-02; do not start legal-pack trees as docs; do not implement a mid-flight named slice under Phase 3 plan IDs (D-L4).

---

### `.planning/phases/03-lease-gated-next-slice/03-VALIDATION.md` (config, batch)

**Analog:** `02-VALIDATION.md` (frontmatter + sampling + per-task map) seeded from `03-RESEARCH.md` § Validation Architecture (lines 449–485)

**Frontmatter** (mirror `02-VALIDATION.md` lines 1–8):

```yaml
---
phase: "03"
slug: "lease-gated-next-slice"
status: draft
nyquist_compliant: false   # → true after wave_0 + maps filled
wave_0_complete: false
created: "2026-09-29"
---
```

**Test infrastructure** — dual plane + shell absence probes (`02-VALIDATION.md` lines 19–24; RESEARCH adds git/path):

| Property | Value |
|----------|-------|
| Framework | (A) `kutha-gov` FSM + pytest harness; (B) cargo libtest smoke; (C) shell path/git probes for lease negative proof |
| Quick run | `uv run kutha-gov precommit` |
| Full suite | `uv run kutha-gov ci` (+ explicit cargo at tracer/pre-verify) |

**Sampling** (`02-VALIDATION.md` lines 31–33) plus D-L4:

```markdown
- **After every task commit (YAML/docs touched):** `uv run kutha-gov precommit`; re-check Active Slice if `.kutha/STATE.md` could change
- **After every plan wave:** `uv run kutha-gov ci` (D-G1) + D-10 SUMMARY + D-L4 lease snapshot pair
- **Before `/gsd-verify-work`:** `ci` HIGH-free + D-11 WARN ledger + D-15 cargo + GOV-03/NEXT tables + D-L4 still None
```

**Per-task map** — copy table header from `02-VALIDATION.md` lines 39–47; replace GOV/PLANE rows with GOV-03 / NEXT-01 / NEXT-02 / D-L4 Task IDs filled by tracer (`03-01-01`…). No new pytest/cargo test files (RESEARCH Open Question 6).

**Wave 0 gaps** (RESEARCH 479–483): VERIFICATION skeleton; VALIDATION Task IDs; optional ROADMAP Overview D-G3.

---

### `.planning/phases/03-lease-gated-next-slice/03-*-PLAN.md` (config, request-response)

**Analog:** `02-01-PLAN.md` / `02-02-PLAN.md` / `02-03-PLAN.md` YAML frontmatter + ROADMAP Phase 3 (`ROADMAP.md` lines 74–84) + TESTING.md tooling matrix (lines 253–261)

**Frontmatter shape** (`02-01-PLAN.md` lines 1–72): `must_haves.truths`, `artifacts`, `key_links`, `prohibitions`. Untracked `.cursor/gsd-core/templates/**` are **not** analogs (tracked-source gate).

**Must_haves** (map to CONTEXT D-L* + ROADMAP success criteria 1–3, interpreted as **negative proof** while lease is None):

1. Wave close: `uv run kutha-gov ci` — exit 0 / HIGH-free (D-L3 → D-G1/D-G2)
2. SUMMARY § Trajectory per D-10
3. Probe tables in VERIFICATION for GOV-03 / NEXT-01 / NEXT-02 + D-L4
4. WARN ledger iff LOW > 0 (D-11)
5. Cargo: tracer + pre-verify (D-15); re-run if a wave unexpectedly touches `crates/`
6. D-L4: Active Slice snapshot at wave open **and** close; named slice → stop, do not implement
7. **Not** must_have: product slice, legal pack, M002, M011-closed in harness STATE, `--fail-on-warn`, phase-per-cell ROADMAP

**Recommended wave split** (RESEARCH primary recommendation + Phase 2 01/02/03):

| Wave | Analog | Focus |
|------|--------|--------|
| Tracer (`03-01`) | `02-01-PLAN.md` | `ci` + `explain trajectory` + cargo + VERIFICATION/VALIDATION skeletons + D-L4 snapshot |
| Mid (`03-02`) | `02-02-PLAN.md` | Paint GOV-03/NEXT-01/NEXT-02 + D-L4 rows; optional ROADMAP Overview D-G3 |
| Closeout (`03-03`) | `02-03-PLAN.md` | Batch REQUIREMENTS GOV-03/NEXT `[x]` + GSD STATE/ROADMAP; **never** `.kutha/STATE.md` |

**Closeout prohibitions analog** (`02-03-PLAN.md` lines 52–67) — invert Phase 2 “do not check GOV-03”:

- Flip GOV-03 / NEXT-01 / NEXT-02 **only** after `03-VERIFICATION.md` `status: passed`
- Do **not** uncheck Phase 2 IDs
- Do not overwrite harness STATE; do not lease a slice; do not treat green as next-milestone grant

**Tooling gate matrix** (`TESTING.md` 255–261) — Phase 3 uses **ci row** as hard gate (same as Phase 2):

| Command | Cargo | Dict checks | JSONL emit |
|---------|-------|-------------|------------|
| `uv run kutha-gov ci` | observe | yes | yes | ← Phase 3 hard gate |
| `uv run kutha-gov precommit` | no | yes | no | ← task-level / single-check probes |
| `cargo test --workspace --offline` | yes | no | no | ← D-15 only |

---

### `.planning/phases/03-lease-gated-next-slice/03-*-SUMMARY.md` (config, batch)

**Analog:** `02-01-SUMMARY.md` frontmatter (`provides`, `requirements-completed: []` until closeout) + § Trajectory lines 110–129

**§ Trajectory template** (D-10 — English; every wave):

```markdown
## Trajectory

**Commands:**
- `uv run kutha-gov ci` → exit <N>; harness: <H> HIGH, <L> LOW, <C> checks
- `uv run kutha-gov explain trajectory` → exit 0

**Excerpt (≤8 lines):**
```text
<paste or paraphrase explain output>
```

**Authority:** green governor ≠ ADR Accepted ≠ L_capability.
```

**D-L4 extension** (no Phase 2 equivalent — add a short subsection after Trajectory):

```markdown
## Lease snapshot (D-L4)

- Wave open: `**Active Slice:** None` (`.kutha/STATE.md`)
- Wave close: `**Active Slice:** None`
- If named S##: HARD STOP — do not implement under Phase 3 plans
```

**WARN:** if LOW > 0, table here or point to `03-VERIFICATION.md`. **Next Step** analog (`02-01-SUMMARY.md` lines 131–133): tracer defers REQUIREMENTS; closeout names “next GSD work is a **new** discuss/plan if STATE later names a slice” — not M002.

**Closeout `requirements-completed`:** `02-03` left GOV-03/NEXT empty; Phase 3 closeout lists GOV-03, NEXT-01, NEXT-02 only.

---

### `.planning/REQUIREMENTS.md` (config, batch)

**Analog:** self lines 20–43 and 104–106; batch hygiene from `02-03-PLAN.md` lines 31–34 (same “one batch after VERIFICATION passed”)

**Checkbox hygiene:**

- Flip in **one batch at verify/complete only**: GOV-03, NEXT-01, NEXT-02 → `[x]`
- Leave FIT-01…05 and Phase 2 GOV/PLANE/FREEZE/MAP as `[x]`
- Update traceability Status column lines 104–106: Phase 3 → Complete
- Do not edit mid-task

```markdown
- [x] **GOV-03**: …   # after VERIFICATION passed
### Delivery next
- [x] **NEXT-01**: …
- [x] **NEXT-02**: …
```

NEXT-02 text still says “After M011 is closed…” — evidence is **negative proof while M011 open** (D-L5). Do not rewrite REQUIREMENTS to require `M011-closed` as a Phase 3 delivery.

---

### `.planning/STATE.md` (config, event-driven)

**Analog:** self — YAML frontmatter (lines 1–18) + Current Position; Phase 2 HIGH-stop pattern from `02-PATTERNS.md` STATE section

**Progress update on green closeout:**

```yaml
status: complete          # or verifying during phase-verify
current_phase: 3
completed_phases: 3       # when phase closed
last_activity_desc: Phase 3 VERIFICATION green; GOV-03/NEXT negative-proof filled
```

**Blocker on HIGH** (D-G2) or **D-L4 named slice:**

```markdown
- Phase 3 governor HIGH: `<check_id>` from `uv run kutha-gov ci`. Wave stopped. Fix under D-G3 captions/YAML — do not thaw freeze or start a slice.
- Phase 3 D-L4: Active Slice became `<id>`. Hard stop. Human starts discuss/plan or inserted phase. Do not implement under 03-* plans.
```

**Guard** (lines 29–31): cite `.kutha/STATE.md`; **never overwrite** harness STATE from GSD. Do not set harness `L_delivery` to `M011-closed`.

---

### `.planning/ROADMAP.md` (config, batch)

**Analog:** self — Overview + Phase 3 block + Progress; Phase 2 plan-line `[x]` pattern

**D-G3 caption clarity (optional, likely required):** RESEARCH pitfall 1 — lines 5 and 9 currently mix “only *new* product work” and “blocked until STATE names a slice,” which **conflicts with D-L1**. Edit Overview so Phase 3 is **verification-only / negative proof under Active Slice None**; product slice delivery waits for a **named lease in a later phase**. Keep line 7 fence: `Do not plan ADR-010–093 as sequential GSD phases` (NEXT-01 / MAP probe). Keep line 14: `not assumed to be M002`.

**Success criteria** (lines 80–82): keep as observable truths; executor proves them negatively while lease is None (criterion 2 “when STATE names a slice” is **out of default plans** — D-L4).

**Closeout:** fill `**Plans**:` with `03-01`… like Phase 2; Progress row `3. Lease-gated… | N/N | Complete | date`.

---

### `.kutha/dictionaries/*` (config, event-driven) — D-G3 only

**Analog:** `docs/process/governor-intake.md` (intake order) + `02-PATTERNS.md` dictionary section

Do **not** promote `I-legal-pack-not-next` (`invariants.yaml` deferred) into a new HIGH check unless D-G3 explicitly adds one (D-L6). Prefer path probes + `h4-lease`.

**Trajectory regex already sufficient** (`checks.yaml` lines 24–32):

```yaml
pattern: '^\*\*Active Slice:\*\*\s*(?P<id>S\d+|None)\s*$'
skip_values: ["None"]
```

After any YAML edit: `uv run kutha-gov precommit --check <id>` then full `ci`. Never add `scripts/kutha_gov/checks/*.py`.

---

## Shared Patterns

### Governor wave gate (D-L3 → D-G1 / D-G2)
**Source:** `scripts/kutha_gov/__main__.py` `cmd_ci` (lines 144–192); CONTEXT D-G1/D-G2 via D-L3  
**Apply to:** every PLAN must_have, SUMMARY, VERIFICATION hard-gate, VALIDATION sampling  

```bash
uv run kutha-gov ci          # HIGH > 0 → stop wave
uv run kutha-gov explain trajectory
```

### Trajectory SUMMARY block (D-10)
**Source:** `02-01-SUMMARY.md` lines 110–129; RESEARCH Pattern 1  
**Apply to:** every wave SUMMARY + phase VERIFICATION  
Four required parts: commands · ci exit+HIGH/LOW · ≤8-line excerpt · green≠Accepted≠L_capability.

### WARN ≡ LOW ledger (D-11)
**Source:** `02-VERIFICATION.md` lines 99–105  
**Apply to:** VERIFICATION + SUMMARY  
`uv run kutha-gov json` only if LOW>0. Never `--fail-on-warn` as gate.

### Evidence map = probe table
**Source:** `02-VERIFICATION.md` lines 107–133  
**Apply to:** `03-VERIFICATION.md`  
Columns: Req | Probe command | Exit | Result one-liner | pass/fail. Commands from RESEARCH catalog, not Phase 2 GOV/PLANE set.

### Active Slice None + crate quiescence (GOV-03 / D-L2)
**Source:** `.kutha/STATE.md` lines 7–9; `checks.yaml` trajectory step; RESEARCH Pattern 2  
**Apply to:** every wave probe table + phase-verify  

```7:9:.kutha/STATE.md
**Active Milestone:** M011
**Active Slice:** None
**Phase:** H4
```

Violation = **new leased product capability** in `crates/kutha-*`, not GSD/docs/D-G3 YAML.

### Mid-phase lease hard stop (D-L4)
**Source:** RESEARCH Pattern 3; no prior GSD section — add VERIFICATION + SUMMARY snapshot pair  
**Apply to:** every PLAN execute wave and phase-verify  

```bash
rg -n '^\*\*Active Slice:\*\*\s*' .kutha/STATE.md
```

Pass: `None`. Named `S\d+` → record blocker; stop; human discuss/plan.

### Legal-pack / honeycomb non-substitute (NEXT-01 / D-L6)
**Source:** Phase 2 FREEZE/MAP spirit (`02-VERIFICATION.md` FREEZE-01/MAP-01 rows 126–132); `.kutha/STATE.md` Next action lines 25–27  
**Apply to:** NEXT-01 probe cluster  
Path absence + `h4-lease` + honeycomb ADR-090 frozen + ADR Proposed. Do not implement pack.

### NEXT-02 negative captions (D-L5)
**Source:** `ROADMAP.md` line 14; RESEARCH Pattern 4  
**Apply to:** NEXT-02 rows + SUMMARY sentence  
Do not require `M011-closed`. Do not start M002. Cite “whatever STATE names after M011 close.”

### Batch REQUIREMENTS hygiene
**Source:** `02-03-PLAN.md` lines 31–34, 52–57  
**Apply to:** closeout wave only  
Single batch GOV-03 + NEXT-01 + NEXT-02 after VERIFICATION passed.

### Two-plane + harness cite-only
**Source:** `.planning/STATE.md` lines 29–31; `.kutha/STATE.md`  
**Apply to:** all plans  
GSD STATE cites lease; never edits `.kutha/STATE.md`.

### Cargo beside ci (D-15)
**Source:** CONTEXT D-15; `TESTING.md` ci vs cargo rows  
**Apply to:** PLAN sampling  
Explicit cargo at tracer + pre-verify (+ crates-touching wave). `observe_cargo` inside `ci` is not a substitute for those two standalone runs.

### Dictionary intake (D-G3)
**Source:** `docs/process/governor-intake.md`  
**Apply to:** HIGH fixes only  
invariants → checks; honeycomb ≠ check ≠ Accepted; no Python Check subclass.

## No Analog Found

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| — | — | — | All Phase 3 GSD artifacts have tracked analogs in Phase 2 + current REQUIREMENTS/ROADMAP/STATE. **D-L4 lease snapshot** is a new *section*, not a new file type — template is RESEARCH Pattern 3. Untracked `.cursor/gsd-core/templates/**` excluded. |

No new product/harness test files. Optional future check for novel legal-pack paths (RESEARCH A2): if added, intake → YAML kind, not a new class.

## Metadata

**Analog search scope:** `.planning/phases/02-honest-harness-and-freeze/{02-VERIFICATION,02-VALIDATION,02-01-PLAN,02-02-PLAN,02-03-PLAN,02-01-SUMMARY,02-PATTERNS}.md`, `.planning/{REQUIREMENTS,STATE,ROADMAP,codebase/TESTING}.md`, `.kutha/{STATE.md,dictionaries/checks.yaml}`, `scripts/kutha_gov/__main__.py`, `docs/process/governor-intake.md`
**Files scanned:** ~20 candidate paths; 14 tracked analogs excerpted
**Tracked-source gate:** `git ls-files` non-empty for all named analogs; rejected untracked gsd-core templates
**Pattern extraction date:** 2026-09-29
**CBM:** `list_projects` → `kutha-graph` present (`/root/kutha-graph`, branch `feat/m010-s01-semantic-recovery`). Structural graph optional for YAML/STATE — Grep/Read used for lease/probe literals (RESEARCH: `metadata_changed` on YAML/STATE)
