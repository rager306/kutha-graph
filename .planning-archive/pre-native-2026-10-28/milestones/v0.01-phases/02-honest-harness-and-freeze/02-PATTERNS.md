# Phase 2: Honest harness and freeze - Pattern Map

**Mapped:** 2026-09-29
**Files analyzed:** 9 (GSD artifacts + optional D-G3 YAML/docs; **no product crates**)
**Analogs found:** 9 / 9

## Product-code verdict

**No new or modified Rust product files for phase success.** CONTEXT D-G3 / D-12…D-14 and RESEARCH: GOV/PLANE/FREEZE/MAP are proven by **governor CI + path/schema probes + evidence tables**. Plans must not thaw freeze, start M002, add Rocks/Cypher/HNSW, legal pack, or promote honeycomb to Accepted. Pattern map below is **verification / process-docs / optional dictionary clarity** only.

Optional D-G3 edits (only if probes or `ci` HIGH expose drift): harness YAML under `.kutha/dictionaries/`, `docs/process/*`, ADR caption clarity, `.planning/ROADMAP.md` / GSD STATE captions — never Python Check subclasses, never crate deps.

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `.planning/phases/02-honest-harness-and-freeze/02-VERIFICATION.md` | config (GSD evidence SoT) | batch / transform | `.planning/phases/01-legal-pit-fitness/01-VERIFICATION.md` + RESEARCH probe catalog | exact (phase-local evolve) |
| `.planning/phases/02-honest-harness-and-freeze/02-VALIDATION.md` | config (Nyquist) | batch | `.planning/phases/01-legal-pit-fitness/01-VALIDATION.md` + `02-RESEARCH.md` § Validation Architecture | exact |
| `.planning/phases/02-honest-harness-and-freeze/02-*-PLAN.md` | config (GSD plan) | request-response | `.planning/ROADMAP.md` Phase 2 block + `01-*-PLAN.md` wave shape + TESTING gate matrix | role-match |
| `.planning/phases/02-honest-harness-and-freeze/02-*-SUMMARY.md` | config (GSD summary) | batch | `01-*-SUMMARY.md` + **new** § Trajectory (D-10) from RESEARCH Pattern 1 | role-match + extension |
| `.planning/REQUIREMENTS.md` | config | batch (checkbox flip) | self — Governor/Planes/Freeze/Map sections (lines 20–43) | exact |
| `.planning/STATE.md` | config | event-driven | self — frontmatter + cite-only harness lease | exact |
| `.planning/ROADMAP.md` | config | batch | self — Phase 2 Plans/Progress (lines 51–86) | exact |
| `.kutha/dictionaries/{checks,invariants,bridges,fsm,honeycomb}.yaml` | config (harness) | event-driven (policy) | `docs/process/governor-intake.md` + existing rows | exact (edit-only if HIGH/drift) |
| `docs/process/{kutha-harness,governor-intake}.md` | config (process doc) | file-I/O | self — clarity under D-G3 | role-match |

**Do not create/modify (read-only evidence / cite targets):**

| Path | Role | Note |
|------|------|------|
| `.kutha/STATE.md` | config (harness lease) | Cite Freeze / Active Slice None / L_*; **never overwrite from GSD** |
| `crates/kutha-*` | product | Probe Op / absence only; no feature work |
| `scripts/kutha_gov/*.py` | utility | Observe/decide already correct; edit only if D-G3 truly needs kernel (last resort → META + test) |
| FIT / GOV-03 / NEXT-* REQUIREMENTS | config | Leave FIT `[x]`; do not flip GOV-03 / NEXT-* |

## Pattern Assignments

### `.planning/phases/02-honest-harness-and-freeze/02-VERIFICATION.md` (config, batch)

**Analog:** `.planning/phases/01-legal-pit-fitness/01-VERIFICATION.md` (structure + frontmatter + Gaps) + RESEARCH § Concrete probe catalog (replace FIT twelve-row map)

**Frontmatter / status tokens** (mirror `01-VERIFICATION.md` lines 1–31; adapt phase ids):

```yaml
---
phase: 02-honest-harness-and-freeze
verified: <ISO-UTC>
status: draft   # → passed | gaps_found
# covered_files: phase plans/summaries + REQUIREMENTS + STATE + ROADMAP (+ any D-G3 YAML/docs)
---
```

**Hard-gate block** — Phase 2 primary gate is governor CI (not cargo-only). Copy shape from `01-VERIFICATION.md` lines 149–157; change command:

| Field | Value |
|-------|-------|
| **Command** | `uv run kutha-gov ci` |
| **Exit code** | `0` (HIGH-free) |
| **Summary line** | `harness: N HIGH, M LOW, … checks` (from `cmd_ci`) |
| **Trajectory** | `uv run kutha-gov explain trajectory` — ≤8 lines or paraphrase + authority sentence |
| **WARN ledger** | Required iff LOW > 0 (D-11): `check_id \| category \| one-line note` |
| **Cargo smoke (D-15)** | Explicit `cargo test --workspace --offline` at tracer + pre-verify (exit + timestamp) |
| **Harness cite** | Active Slice None; freeze until M002 (`.kutha/STATE.md` — not edited) |

**ci print / exit semantics** (`scripts/kutha_gov/__main__.py` lines 144–192):

```python
# Prints: "harness: {high} HIGH, {low} LOW, {n} checks  ({rung} dogfood)"
# Finding text for LOW suppressed unless fail_on_warn — use `kutha-gov json` for D-11 messages
if outcome.terminal != "ok":
    return 1
return 0
```

**decide HIGH-stop** (`scripts/kutha_gov/fsm.py` lines 291–302) — D-G2:

```python
if high:
    return "fail"
if ctx.fail_on_warn and low:
    return "fail"
return "ok"
```

Do **not** enable `--fail-on-warn` / `KUTHA_GOV_FAIL_ON_WARN` as the Phase 2 gate (rejects D-11).

**Probe evidence table** — replace FIT columns with RESEARCH catalog shape (`Req | Probe | Exit | Result | pass/fail`). Seed rows from RESEARCH lines 450–472:

| Req | Probe | Pass criterion |
|-----|-------|----------------|
| GOV-01 | `uv run kutha-gov ci` | Exit 0; `0 HIGH`; SUMMARY green≠Accepted≠L_capability |
| GOV-01 | `uv run kutha-gov explain trajectory` | Contains `authority: none — harness does not accept ADRs or claim product readiness` |
| GOV-02 | `rg -n 'L_map=\|L_delivery=\|L_capability=' .kutha/STATE.md` | All three needles, distinct rows |
| GOV-02 | `uv run kutha-gov precommit --check lifecycles` | Exit 0; high=0 |
| PLANE-01 | path probes (`ls crates/`, no `*.py` in runtime, no root `ports/`/`adapters/`/`domain/`) | All ok |
| PLANE-02 | `rg` Op enum + no merge-patch / `object.created` in crates | Op present; no STCA §5 write API |
| PLANE-03 | `head -1` both relations.yaml + `precommit --check plane-mix-dicts` | Distinct schemas |
| FREEZE-01 | STATE Freeze cite + Cargo.toml absences + `precommit --check freeze` | Absence + lease |
| MAP-01 | `kutha-gov map` + ROADMAP “Do not plan ADR-010” + `honeycomb-map` / `adr-status` | Proposed / map |
| D-15 | `cargo test --workspace --offline` | Exit 0 at tracer + pre-verify |

**WARN (LOW) ledger subsection** (D-11) — new vs Phase 1; when `LOW > 0`:

| check_id | category | note |
|----------|----------|------|
| … | … | one line |

Unlogged LOW = process failure. Severity mapping: CONTEXT “WARN” ≡ harness `Severity.LOW` / summary `LOW`.

**Anti-patterns:** Do not treat `precommit` alone as GOV-01; do not paste full FSM / events.jsonl; do not “fix” FREEZE by implementing frozen surfaces; do not flip GOV-03.

---

### `.planning/phases/02-honest-harness-and-freeze/02-VALIDATION.md` (config, batch)

**Analog:** `01-VALIDATION.md` (frontmatter + sampling + per-task map) seeded from `02-RESEARCH.md` § Validation Architecture

**Frontmatter** (mirror `01-VALIDATION.md` lines 1–8):

```yaml
---
phase: "02"
slug: "honest-harness-and-freeze"
status: draft
nyquist_compliant: false   # → true after wave_0 + maps filled
wave_0_complete: false
created: "2026-09-29"
---
```

**Test infrastructure row** — dual plane (from RESEARCH):

| Property | Value |
|----------|-------|
| Framework | (A) `kutha-gov` FSM + pytest harness; (B) cargo libtest smoke |
| Quick run | `uv run kutha-gov precommit` |
| Full suite | `uv run kutha-gov ci` (+ explicit cargo at tracer/pre-verify) |

**Sampling pattern** (replace Phase 1 cargo-every-commit):

```markdown
- **After every task commit (YAML/docs touched):** `uv run kutha-gov precommit`
- **After every plan wave:** `uv run kutha-gov ci` (D-G1) + D-10 SUMMARY § Trajectory
- **Before `/gsd-verify-work`:** `ci` HIGH-free + D-11 WARN ledger + D-15 cargo smoke + completed probe tables
```

**Wave 0 gaps** (RESEARCH lines 568–572): create VERIFICATION skeleton; seed VALIDATION; Trajectory fragment lives in PLAN/SUMMARY — not a new code file.

---

### `.planning/phases/02-honest-harness-and-freeze/02-*-PLAN.md` (config, request-response)

**Analog:** `.planning/ROADMAP.md` Phase 2 (lines 51–61) + Phase 1 wave cadence (`01-01` tracer → `01-02` paint → `01-03` batch) + `.planning/codebase/TESTING.md` tooling matrix (lines 253–262)

**Must_haves** (map 1:1 to CONTEXT + ROADMAP success criteria):

1. Wave close: `uv run kutha-gov ci` — exit 0 / HIGH-free (D-G1/D-G2)
2. SUMMARY § Trajectory per D-10 (commands, exit+HIGH/LOW counts, ≤8-line explain excerpt, green≠Accepted≠L_capability)
3. Probe tables in VERIFICATION for GOV/PLANE/FREEZE/MAP (D-12…D-14)
4. WARN ledger iff LOW > 0 (D-11)
5. Cargo: tracer + pre-verify only (D-15); re-run if wave touches `crates/`
6. **Not** must_have: product features, freeze thaw, GOV-03, `--fail-on-warn`, phase-per-cell ROADMAP

**ROADMAP truths** (lines 56–61) → plan acceptance:

```markdown
1. kutha-gov ci fail-closed on HIGH; green ≠ ADR Accepted ≠ capability
2. STATE still lists L_map / L_delivery / L_capability separately
3. Planes / schemas / no root hexagon
4. Typed Op; no stca-guide §5 write API
5. Frozen surfaces absent; honeycomb ADRs remain Proposed
```

**Recommended wave split** (RESEARCH primary recommendation):

| Wave | Focus |
|------|--------|
| Tracer | `ci` + `explain trajectory` + cargo smoke + VERIFICATION skeleton |
| Mid | Complete PLANE/FREEZE/MAP probe paint + WARN ledger rule |
| Closeout | REQUIREMENTS GOV/PLANE/FREEZE/MAP `[x]` batch + GSD STATE/ROADMAP |

**Tooling gate matrix** (TESTING.md 255–261) — Phase 2 uses **ci row** as hard gate; cargo as D-15 smoke:

| Command | Cargo | Dict checks | JSONL emit |
|---------|-------|-------------|------------|
| `uv run kutha-gov ci` | observe | yes | yes | ← Phase 2 hard gate |
| `uv run kutha-gov precommit` | no | yes | no | ← task-level |
| `cargo test --workspace --offline` | yes | no | no | ← D-15 only |

---

### `.planning/phases/02-honest-harness-and-freeze/02-*-SUMMARY.md` (config, batch)

**Analog:** `01-01-SUMMARY.md` frontmatter/provides + onboarding SUMMARY terseness; **extend** with mandatory § Trajectory (no Phase 1 equivalent)

**Frontmatter reuse** (`01-01-SUMMARY.md` lines 1–43): `phase`, `plan`, `provides`, `key-files`, `requirements-completed` (defer GOV/PLANE/FREEZE/MAP `[x]` to closeout wave).

**§ Trajectory template** (D-10 — English; copy into every wave SUMMARY):

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

Authority needle (must appear in explain or be stated explicitly):

```text
authority: none — harness does not accept ADRs or claim product readiness
```

**WARN ledger** (D-11): if LOW > 0, include table here **or** point to `02-VERIFICATION.md` ledger — never silence.

---

### `.planning/REQUIREMENTS.md` (config, batch)

**Analog:** self — lines 20–43; Phase 1 FIT batch pattern (`01-PATTERNS.md` REQUIREMENTS section)

**Checkbox hygiene:**

- Flip in **one batch at verify/complete only**: GOV-01, GOV-02, PLANE-01, PLANE-02, PLANE-03, FREEZE-01, MAP-01 → `[x]`
- Leave **GOV-03**, **NEXT-01**, **NEXT-02** unchecked (Phase 3)
- Leave FIT-01…05 as `[x]` (already done)
- Update traceability table Status column for Phase 2 rows (lines 97–103) when closing
- Do not edit mid-task (same D-04/D-05 spirit as Phase 1)

```markdown
### Governor honesty
- [x] **GOV-01**: …
- [x] **GOV-02**: …
- [ ] **GOV-03**: …   # Phase 3 — leave open

### Planes and write surface
- [x] **PLANE-01** … **PLANE-03**

### Freeze
- [x] **FREEZE-01**: …

### Map discipline
- [x] **MAP-01**: …
```

---

### `.planning/STATE.md` (config, event-driven)

**Analog:** self — YAML frontmatter (lines 1–17) + Current Position; Phase 1 blocker pattern for HIGH-stop

**Progress update on green closeout:**

```yaml
status: complete          # or verifying during phase-verify
current_phase: 2
completed_phases: 2       # when phase closed
last_activity_desc: Phase 2 VERIFICATION green; GOV/PLANE/FREEZE/MAP evidence map filled
```

**Blocker on HIGH** (D-G2) — under Blockers/Concerns:

```markdown
- Phase 2 governor HIGH: `<check_id>` from `uv run kutha-gov ci`. Wave stopped per D-G2. Fix under D-G3 (YAML/docs/captions) or explicit lease — do not thaw freeze.
```

**Guard** (lines 29–31): cite `.kutha/STATE.md`; **never overwrite** harness STATE from GSD.

---

### `.planning/ROADMAP.md` (config, batch)

**Analog:** self — Phase 2 block + Progress table; Phase 1 plan-line `[x]` pattern (lines 40–48)

**Edit pattern at closeout:**

- Fill `**Plans**:` with `N/N` and wave plan checkboxes (`02-01`… like `01-01`…)
- Progress row: `2. Honest harness… | N/N | Complete | date`
- Keep line 7 fence: `Do not plan ADR-010–093 as sequential GSD phases` (MAP-01 probe)
- D-G3 caption clarity OK; do not schedule phase-per-cell

---

### `.kutha/dictionaries/*` (config, event-driven) — D-G3 only

**Analog:** `docs/process/governor-intake.md` (full file) + existing ledger/check rows

**Intake order** (governor-intake.md lines 21–35, 57–63) — copy exactly when fixing HIGH:

```text
control-loop claim → invariants.yaml (disposition) → checks.yaml (allowed kind)
bridge cite product → bridges.yaml + checks.yaml
map stage/edge → honeycomb.yaml only (not a check; not Accepted)
```

**Invariant row shape** (`.kutha/dictionaries/invariants.yaml` lines 9–20):

```yaml
  - id: I-lifecycles
    claim: "L_map, L_delivery, and L_capability stay named and uncollapsed"
    source: ".kutha/STATE.md"
    disposition: check
    check: lifecycles
```

**Bridge row shape** (`bridges.yaml` lines 8–12):

```yaml
  - id: B-freeze-deps
    claim: "Cargo members do not add frozen product deps before STATE names them"
    cites: ".kutha/STATE.md"
    check: freeze
```

**Check row examples to mirror** (`checks.yaml`):

- `lifecycles` — `file_contains` needles `L_map=`, `L_delivery=`, `L_capability=` (lines 37–42 in RESEARCH / checks.yaml)
- `freeze` — `glob_absent` frozen deps (lines 60–74)
- `plane-mix-dicts` — schema fence (lines 149–161)
- `honeycomb-map` / `adr-status` — map discipline (lines 50–58, 87–107); note `severity: low` legend steps → D-11 ledger candidates

**After YAML edit:** `uv run kutha-gov precommit --check <id>` then full `ci`. Never add `scripts/kutha_gov/checks/*.py`.

**fsm.yaml:** append state/transition only with allowed kind; `observe_cargo.args` stay `["test", "--workspace", "--offline"]` (lines 23–31) — do not remove cargo observe to “speed up” Phase 2.

---

### `docs/process/{kutha-harness,governor-intake}.md` (config, file-I/O) — D-G3 only

**Analog:** self. Clarity edits when docs-coupling / dogfood needles drift.

**CLI contract cite** (`kutha-harness.md` ~158): `HIGH findings → exit 1. LOW → exit 0 unless --fail-on-warn.` Align SUMMARY wording with this; do not flip default fail-on-warn.

## Shared Patterns

### Governor wave gate (D-G1 / D-G2)
**Source:** `scripts/kutha_gov/__main__.py` `cmd_ci`; `fsm.py` `decide`; CONTEXT D-G1/D-G2  
**Apply to:** every PLAN must_have, SUMMARY, VERIFICATION hard-gate, VALIDATION sampling  

```bash
uv run kutha-gov ci          # HIGH > 0 → stop wave
uv run kutha-gov explain trajectory
```

### Trajectory SUMMARY block (D-10)
**Source:** CONTEXT D-10; RESEARCH Pattern 1  
**Apply to:** every wave SUMMARY + phase VERIFICATION  
Four required parts: commands · ci exit+HIGH/(LOW) · ≤8-line excerpt · green≠Accepted≠L_capability.

### WARN ≡ LOW ledger (D-11)
**Source:** CONTEXT D-11; `protocol.py` Severity; `__main__.py` print suppression  
**Apply to:** VERIFICATION + SUMMARY  
Use `uv run kutha-gov json` when LOW > 0 and finding text missing from `ci` stdout. Never `--fail-on-warn` as gate.

### Evidence map = probe table (not cargo --exact)
**Source:** `01-VERIFICATION.md` FIT map evolved; RESEARCH probe catalog  
**Apply to:** VERIFICATION SoT  
Columns: Req | Probe command | Exit | Result one-liner | pass/fail.

### Batch REQUIREMENTS hygiene
**Source:** Phase 1 D-04/D-05 pattern; REQUIREMENTS lines 20–43  
**Apply to:** closeout wave only  
Single batch GOV-01/02 + PLANE-01…03 + FREEZE-01 + MAP-01; leave GOV-03/NEXT.

### Two-plane + freeze cite-only
**Source:** `.kutha/STATE.md` lines 7–31; AGENTS two planes; D-12/D-13  
**Apply to:** all plans  
GSD STATE cites harness lease; never edits `.kutha/STATE.md`. Freeze proven by absence + check `freeze`, not by implementing Rocks.

### Dictionary intake (D-G3)
**Source:** `docs/process/governor-intake.md`  
**Apply to:** any HIGH fix  
invariants → checks; bridges → checks; honeycomb index ≠ check ≠ Accepted; no Python Check subclass.

### Cargo beside ci (D-15)
**Source:** CONTEXT D-15; `fsm.yaml` observe_cargo already runs cargo inside `ci`  
**Apply to:** PLAN sampling  
Do not require a second standalone cargo on intermediate waves; do require explicit cargo at tracer + pre-verify (+ any crates-touching wave).

## No Analog Found

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| — | — | — | All Phase 2 artifacts have tracked analogs. § Trajectory is an **extension** of SUMMARY (no prior section) — template lives in RESEARCH/CONTEXT, not a missing file. Untracked `.cursor/gsd-core/templates/**` excluded per tracked-source gate. |

Optional future YAML check for root hexagon dirs (`ports/`/`adapters/`/`domain/`): no existing check — path probe sufficient (RESEARCH Open Question 2); if planner adds one, follow intake → `glob_absent`/`file_absent` kind, not a new class.

## Metadata

**Analog search scope:** `.planning/phases/01-*`, `.planning/{REQUIREMENTS,STATE,ROADMAP,codebase/TESTING,onboarding}`, `docs/process/{kutha-harness,governor-intake}.md`, `.kutha/dictionaries/{checks,invariants,bridges,fsm,honeycomb}.yaml`, `.kutha/STATE.md`, `scripts/kutha_gov/{__main__,fsm,protocol}.py`
**Files scanned:** ~30 candidate paths; 12 tracked analogs read/excerpted
**Tracked-source gate:** Confirmed via `git ls-files` for all named analogs; rejected untracked gsd-core templates
**Pattern extraction date:** 2026-09-29
**CBM:** `list_projects` → `kutha-graph` present (`/root/kutha-graph`); structural graph optional — Grep/Read preferred for YAML/docs (per RESEARCH coverage note)
