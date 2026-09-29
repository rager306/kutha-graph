# Phase 2: Honest harness and freeze - Research

**Researched:** 2026-09-29
**Domain:** Harness governor CI / trajectory honesty / two-plane freeze verification (Python `kutha-gov`, `.kutha/` dictionaries)
**Confidence:** HIGH (in-repo FSM + CLI + probes); MEDIUM on ASVS 5.0 chapter renumbering vs GSD template labels

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-G1:** After every execute-wave and before phase-verify, run `uv run kutha-gov ci` as a required automated gate. Each wave-closing SUMMARY (and phase closeout VERIFICATION) includes a brief trajectory reading from `uv run kutha-gov explain trajectory` (see **D-10**). — **Reversibility:** reversible — process gate only
- **D-G2:** Any **HIGH** from `kutha-gov ci` **stops the wave** and blocks phase completion until cleared or explicitly leased outside this CONTEXT. **WARN** findings are recorded — never silenced or “green with caveat” (severity at closeout: **D-11**). — **Reversibility:** reversible — policy only
- **D-G3:** In-scope improvements when trajectory/semantics drift: harness YAML (checks, bridges, invariants, fsm, honeycomb index), `docs/process/*`, clarity edits to ADR wording / `.planning/ROADMAP.md` / GSD STATE captions. **Out of scope:** new product milestone, assumed M002, freeze thaw, legal pack, collapsing L_map / L_delivery / L_capability, treating governor green as ADR Accepted. — **Reversibility:** reversible — scope fence
- **D-10:** Wave SUMMARY § Trajectory (English) must contain: (1) commands run — at least `uv run kutha-gov ci` and `uv run kutha-gov explain trajectory`; (2) `ci` outcome — exit code + HIGH count (and WARN count if non-zero); (3) **≤8 lines** excerpt from `explain trajectory` (or a one-paragraph paraphrase of the same facts); (4) one explicit sentence: green governor ≠ ADR Accepted ≠ L_capability. Do not paste full FSM dumps or entire events.jsonl. — **Reversibility:** reversible
- **D-11:** Phase-verify / phase complete may proceed when `ci` is **HIGH-free** even if WARNs remain, **iff** every WARN id is listed in `02-VERIFICATION.md` (or wave SUMMARY ledger) with check id + one-line note. Unlogged WARN = treat as process failure (fix ledger or escalate). HIGH always blocks. — **Reversibility:** reversible
- **D-12:** PLANE-01…03 are proven by **path/schema/source probes**, not new product features: (PLANE-01) product under `crates/kutha-*`, harness under `scripts/kutha_gov` + `.kutha/`, no Python inside `kutha-runtime`, no repo-root `ports/` / `adapters/` / `domain/`; (PLANE-02) product write surface remains typed `Op` in crates — no STCA-guide §5 JSON merge-patch / `object.created` as product write API; (PLANE-03) product `kutha-relations/v1` and harness `kutha-harness-relations/v1` remain distinct files/schemas. Evidence lives in VERIFICATION table; D-G3 clarity edits allowed if probes fail for doc drift. — **Reversibility:** reversible
- **D-13:** FREEZE-01 proven by **absence + lease cite**: `.kutha/STATE.md` still documents freeze / Active Slice None / no M002 lease; workspace manifests do not add RocksDB/Cypher/HNSW (or equivalent) product deps; no legal-pack / ADR-050 six-dict delivery under crates. Do not “fix” freeze by implementing frozen surfaces. — **Reversibility:** reversible
- **D-14:** MAP-01 proven by: honeycomb ADRs 010–093 remain Proposed (not Accepted as shipped product); `.planning/ROADMAP.md` does not schedule one GSD phase per honeycomb cell; `uv run kutha-gov map` (or dictionary index read) still presents L_map as compact map. Clarity edits to captions OK under D-G3. — **Reversibility:** reversible
- **D-15:** Do **not** require full `cargo test --workspace --offline` after every Phase 2 wave. Run it **once in the tracer / first wave** and again **before phase-verify** as product regression smoke (Phase 1 hard gate still valid). Intermediate waves rely on `kutha-gov ci` (which already observes cargo per fsm). If a wave modifies `crates/`, that wave must re-run cargo before closing. — **Reversibility:** reversible

### Claude's Discretion
User said “реши эти вопросы разумно” for gray areas 1–6 above → D-10…D-15. Planner may tighten probe command strings; must not weaken HIGH-stop or D-G3 fence.

### Deferred Ideas (OUT OF SCOPE)
- GOV-03 / Active Slice enforcement as delivery gate — Phase 3
- Making WARN fail-closed like HIGH — rejected for Phase 2 (D-11)
- Requiring cargo on every intermediate wave — rejected (D-15)
- M002 / Rocks / Cypher / HNSW / legal pack — freeze
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| GOV-01 | `kutha-gov ci` fail-closed on HIGH; green ≠ ADR Accepted ≠ L_capability | FSM `decide` fails on HIGH; CLI exit 1 when terminal ≠ `ok`; D-10 SUMMARY sentence + `explain trajectory` authority line |
| GOV-02 | STATE keeps `L_map` / `L_delivery` / `L_capability` separate | Check `lifecycles` needles; STATE lifecycle block; `precommit --check lifecycles` |
| PLANE-01 | Product crates vs harness; no Python in runtime; no root hexagon dirs | Path probes + existing `dogfood` / plane checks; absence of `ports/` `adapters/` `domain/` |
| PLANE-02 | Typed `Op`; not stca-guide §5 merge-patch write API | `Op` enum in `kutha-common`; no `object.created` / merge-patch in crates |
| PLANE-03 | Distinct relation schemas | `kutha-relations/v1` vs `kutha-harness-relations/v1`; check `plane-mix-dicts` |
| FREEZE-01 | Frozen surfaces unstarted until STATE leases | STATE freeze text; check `freeze`; Cargo.toml absence probes |
| MAP-01 | Honeycomb stays map / Proposed; no phase-per-cell | `kutha-gov map`; `adr-status`; ROADMAP wording; check `honeycomb-map` |
</phase_requirements>

## Summary

Phase 2 is **verification-first on the harness plane**. The governor quantum already exists: `uv run kutha-gov ci` walks `.kutha/dictionaries/fsm.yaml` (load dictionaries → `run_checks` → `observe_cargo` → emit → tenant → fold → `decide`). Baseline this session: **exit 0**, `harness: 0 HIGH, 0 LOW, 27 checks (H4 dogfood)`, terminal `ok`. Planning must wire D-G1/D-G2 gates into every execute wave, fill a Phase-1-style evidence map for GOV/PLANE/FREEZE/MAP probes, and keep all fixes inside the D-G3 fence (YAML/docs/captions only).

**WARN ↔ LOW:** CONTEXT uses “WARN”; the harness protocol uses `Severity.LOW` printed as `low` / counted as `LOW` on the `ci` summary line. Treat them as the same severity class for D-11. Do **not** enable `--fail-on-warn` / `KUTHA_GOV_FAIL_ON_WARN` as the Phase 2 gate (that would reject D-11).

**Cargo vs D-15:** Every `ci` already runs `cargo test --workspace --offline` inside `observe_cargo`. D-15 means: do not add a *second* mandatory standalone cargo gate on intermediate waves; still run explicit cargo once in the tracer and again before phase-verify (and whenever a wave touches `crates/`).

**Primary recommendation:** Plan 2–3 verify-only waves: (1) tracer — `ci` + `explain trajectory` + cargo smoke + VERIFICATION skeleton; (2) complete probe tables for PLANE/FREEZE/MAP + WARN ledger rule; (3) batch REQUIREMENTS GOV/PLANE/FREEZE/MAP checkboxes + GSD STATE — zero product features, zero freeze thaw.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Governor CI / FSM quantum | Harness CLI (`scripts/kutha_gov`) | `.kutha/dictionaries/*` | Process trajectory honesty; not product SoT |
| Trajectory / lifecycle / freeze checks | Harness dictionaries + filesystem adapters | — | YAML kinds interpret STATE/ROADMAP/Cargo.toml |
| Cargo observe inside `ci` | Developer workstation / CI shell | Product crates tests | Evidence observation only (`observe.py`) |
| Typed Op write surface | Product (`crates/kutha-common`) | — | Event log SoT; Phase 2 only probes |
| Relation allowlists | Product YAML + harness YAML (separate schemas) | — | plane-mix fail-closed |
| Honeycomb L_map index | `.kutha/dictionaries/honeycomb.yaml` + `map` CLI | ADR Status narrative | Map ≠ backlog ≠ Accepted |
| GSD wave SUMMARY / VERIFICATION | `.planning/phases/02-*` | — | D-10/D-11 evidence lease |
| Freeze lease | `.kutha/STATE.md` | check `freeze` | Absence + cite; do not implement frozen surfaces |

## Standard Stack

### Core

| Library / Tool | Version | Purpose | Why Standard |
|----------------|---------|---------|--------------|
| `uv run kutha-gov` | uv 0.12.13 `[VERIFIED: shell uv --version]`; entry `kutha-gov = "kutha_gov.__main__:main"` `[VERIFIED: pyproject.toml:10-11]` | Wave gate + map + explain | Composition root for harness STCA plane |
| Python | `requires-python = ">=3.13"` `[VERIFIED: pyproject.toml:5]`; pin `.python-version` = `3.13` `[VERIFIED: .python-version]` | Harness runtime | AGENTS / docs forbid system 3.12 |
| `.kutha/dictionaries/fsm.yaml` | schema `kutha-harness-fsm/v1` `[VERIFIED: .kutha/dictionaries/fsm.yaml:1]` | CI quantum machine | Append-only phases; unknown kind HIGH |
| `scripts/kutha_gov/{__main__,fsm,protocol,observe}.py` | in-tree | CLI + decide + severity | Already tested (`scripts/tests`) |
| `cargo test --workspace --offline` | cargo/rustc 1.98.1 `[VERIFIED: shell]` | Product smoke (D-15) + observe inside ci | Same args as `fsm.yaml` observe_cargo |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|----------------|---------|---------|-------------|
| `uv run kutha-gov explain trajectory` | in-tree | D-10 trajectory reading | Explains check id `trajectory` (not a free-form dump) |
| `uv run kutha-gov map` | in-tree | MAP-01 compact L_map | Text table or `--format json` |
| `uv run kutha-gov precommit [--check ID]` | in-tree | Fast dictionary-only probes | No cargo / no JSONL write |
| `uv run kutha-gov json` | in-tree | List LOW finding details for D-11 | When summary `LOW > 0` |
| `uv run pytest` | pytest≥8 (dev group) | Harness regression | Wave touching `scripts/kutha_gov` |
| PyYAML | `pyyaml>=6` `[VERIFIED: pyproject.toml:6-8]` | Dictionary load | Already installed; **do not add packages** |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `ci` as wave gate | `precommit` only | Faster but skips observe/emit/tenant — **rejects GOV-01** |
| WARN fail-closed (`--fail-on-warn`) | D-11 ledger | Rejected by CONTEXT |
| Standalone cargo every wave | Rely on `observe_cargo` in `ci` | D-15: cargo smoke only tracer + pre-verify (+ crates edits) |
| New Python Check subclasses | Append YAML rows | Forbidden by intake / META |

**Installation:** None — no new crates or PyPI packages for Phase 2.

**Version verification:** `uv 0.12.13`, `cargo 1.98.1`, `rustc 1.98.1`, Python pin `3.13` probed this session.

## Package Legitimacy Audit

> Phase installs **no** external packages.

| Package | Registry | Age | Downloads | Source Repo | Verdict | Disposition |
|---------|----------|-----|-----------|-------------|---------|-------------|
| — | — | — | — | — | N/A | No install |

**Packages removed due to [SLOP] verdict:** none
**Packages flagged as suspicious [SUS]:** none

## Architecture Patterns

### System Architecture Diagram

```text
Execute wave close / phase-verify
        │
        ├─► uv run kutha-gov explain trajectory     ◄── D-10 excerpt (≤8 lines)
        │         (check id: trajectory)
        │
        └─► uv run kutha-gov ci                     ◄── D-G1 HARD GATE
                  │
                  ▼
         fsm.yaml quantum
         idle → load_* (META, checks, fsm, relations,
                invariants, bridges, honeycomb)
              → run_checks  (dictionary kinds; budget V)
              → observe_cargo  (cargo test --workspace --offline
                                + build kutha-tenant)
              → emit_log → emit_tenant → fold_log
              → decide
                  │
         high > 0 ──► terminal fail → exit 1 ──► STOP wave (D-G2)
         high == 0 && (!fail_on_warn || low==0) ──► ok → exit 0
                  │
                  ▼
         SUMMARY § Trajectory (D-10) + WARN ledger if LOW>0 (D-11)
                  │
                  ▼
         Path/schema probes (D-12…D-14) → 02-VERIFICATION.md tables
                  │
         Explicit cargo smoke (D-15): tracer + pre-verify only
```

### Recommended Project Structure (unchanged)

```text
scripts/kutha_gov/          # harness CLI + FSM interpreter
.kutha/
  STATE.md                  # lease: lifecycles, freeze, Active Slice
  dictionaries/
    fsm.yaml                # ci quantum
    checks.yaml             # check rows (trajectory, lifecycles, freeze, …)
    invariants.yaml         # control-loop ledger
    bridges.yaml            # cite product without copying FSMs
    honeycomb.yaml          # L_map compact index
    relations.yaml          # kutha-harness-relations/v1
crates/
  kutha-common/             # Op enum
  kutha-runtime/
    dictionaries/relations.yaml  # kutha-relations/v1
.planning/phases/02-honest-harness-and-freeze/
  02-VERIFICATION.md        # evidence SoT (create in execute)
  02-VALIDATION.md          # Nyquist seed from this research
```

### Pattern 1: Wave-close governor loop (D-G1 / D-10)

**What:** Required automated gate + short trajectory reading.
**When to use:** End of every execute wave and before phase-verify.
**Example:**

```bash
uv run kutha-gov ci
# Capture: exit code; line "harness: N HIGH, M LOW, …"
uv run kutha-gov explain trajectory
# Paste ≤8 lines into SUMMARY § Trajectory + green≠Accepted≠L_capability sentence
```

Authority line from explain (verbatim this session):

```text
authority: none — harness does not accept ADRs or claim product readiness
```

`[VERIFIED: shell uv run kutha-gov explain trajectory]`

### Pattern 2: D-G3 fix without thaw

**What:** Fix HIGH by editing harness YAML / process docs / captions only.
**When to use:** `ci` exit 1 or probe doc-drift.
**How:**

1. Identify check id from `[HIGH] … (check_id/category) …`
2. Control-loop claim → append/edit `.kutha/dictionaries/invariants.yaml` then `checks.yaml`
3. Bridge cite → `.kutha/dictionaries/bridges.yaml` + matching check
4. Map stage/edge → `.kutha/dictionaries/honeycomb.yaml` (not a check; not Accepted)
5. FSM phase → append `fsm.yaml` transition/state with allowed kind
6. Re-run `uv run kutha-gov precommit --check <id>` then full `ci`
7. **Never:** add Rocks/Cypher/HNSW deps, legal pack, M002 milestone, Accepted honeycomb, Python under `kutha-runtime`, repo-root `ports/`

### Pattern 3: WARN (LOW) ledger (D-11)

**What:** HIGH-free closeout with logged LOWs.
**When to use:** Summary shows `LOW > 0`.
**How:**

1. Per-check lines already show `low=N` even when finding text is suppressed
2. For messages: `uv run kutha-gov json` and filter `"severity": "low"` (ci prints LOW details only if `--fail-on-warn`)
3. Ledger row: `check_id | category | one-line note` in SUMMARY or `02-VERIFICATION.md`
4. Unlogged LOW → process failure

### Anti-Patterns to Avoid

- **Green = Accepted:** Violates GOV-01 / AGENTS; SUMMARY must deny it
- **`--fail-on-warn` as Phase 2 gate:** Collapses D-11
- **Implementing frozen surfaces to “satisfy” FREEZE-01:** Freeze is absence + lease cite
- **Phase-per-cell ROADMAP:** Collapses L_map into L_delivery
- **Hand-rolling Python Check classes:** Intake forbids; unknown kind → HIGH
- **Treating `explain trajectory` as FSM dump:** It explains one check; use `fsm` / fold separately if needed (do not paste into SUMMARY)

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Wave honesty gate | Custom shell checklist | `uv run kutha-gov ci` | FSM already fail-closed on HIGH |
| Trajectory reading | Paste events.jsonl | `explain trajectory` + ≤8 lines | D-10; authority none |
| Lifecycle non-collapse | New status enum | Needles + check `lifecycles` | Already encoded |
| Freeze enforcement | Implement Rocks “behind flag” | Absence probes + check `freeze` | Implementing thaws |
| Plane mix | Shared relations file | Two schemas + `plane-mix-dicts` | Mixing is HIGH |
| New check logic | `scripts/kutha_gov/checks/*.py` | YAML row + allowed kind | META / intake |
| WARN inventory | Guess from memory | `ci` LOW count + `json` | Print suppression without fail_on_warn |

**Key insight:** Phase 2 ships evidence and process gates, not new product capability. The harness already enforces most fences; GSD must *observe and record* them honestly.

## Common Pitfalls

### Pitfall 1: Confusing WARN with `--fail-on-warn`

**What goes wrong:** Enabling `KUTHA_GOV_FAIL_ON_WARN` makes LOW fail `decide` → exit 1.
**Why it happens:** Env/docs mention the flag next to HIGH.
**How to avoid:** Gate on HIGH only; ledger LOWs per D-11.
**Warning signs:** `ci` red with `0 HIGH` but nonzero LOW and fail_on_warn set.

### Pitfall 2: Double-counting cargo / skipping D-15 smoke

**What goes wrong:** Intermediate waves either skip product smoke entirely *or* demand a third cargo ritual beyond `ci`.
**Why it happens:** Misreading D-15 vs `observe_cargo`.
**How to avoid:** Intermediate: `ci` only (observe included). Tracer + pre-verify: explicit `cargo test --workspace --offline`. Crates edits: cargo that wave.
**Warning signs:** Wave SUMMARY with no cargo mention after editing `crates/`.

### Pitfall 3: Fixing HIGH by thawing freeze

**What goes wrong:** Adding rocksdb to Cargo.toml to “make freeze check happy” or starting legal pack.
**Why it happens:** Misreading absence probes as implementation tasks.
**How to avoid:** D-G3 + D-13 — fix docs/YAML or stop; never implement frozen surfaces.
**Warning signs:** Diff touches `crates/*/Cargo.toml` deps or new pack crates.

### Pitfall 4: LOW findings invisible in `ci` text

**What goes wrong:** Summary says LOW>0 but no finding lines → empty D-11 ledger.
**Why it happens:** `cmd_ci` prints finding text only for HIGH unless `fail_on_warn`.
**How to avoid:** Use per-check `low=` counts + `kutha-gov json` for messages.
**Warning signs:** `harness: 0 HIGH, 2 LOW` with no `[low]` lines.

### Pitfall 5: Collapsing three lifecycles in captions

**What goes wrong:** GSD STATE or ROADMAP says “green = shipped”.
**Why it happens:** Convenience wording.
**How to avoid:** Keep L_* distinct; check `lifecycles` forbids `governor green = product`.
**Warning signs:** HIGH from `lifecycles` / `collapse` category.

## Code Examples

### Decide / exit semantics

```291:302:scripts/kutha_gov/fsm.py
    if kind == "decide":
        high = sum(r.high_count for r in outcome.results) + sum(
            1 for f in outcome.findings if f.severity is Severity.HIGH
        )
        low = sum(r.low_count for r in outcome.results) + sum(
            1 for f in outcome.findings if f.severity is Severity.LOW
        )
        if high:
            return "fail"
        if ctx.fail_on_warn and low:
            return "fail"
        return "ok"
```

`[VERIFIED: scripts/kutha_gov/fsm.py:291-302]`

```190:192:scripts/kutha_gov/__main__.py
    if outcome.terminal != "ok":
        return 1
    return 0
```

`[VERIFIED: scripts/kutha_gov/__main__.py:190-192]`

Severity values:

```10:12:scripts/kutha_gov/protocol.py
class Severity(StrEnum):
    HIGH = "high"
    LOW = "low"
```

`[VERIFIED: scripts/kutha_gov/protocol.py:10-12]`

### observe_cargo placement (D-15)

```23:31:.kutha/dictionaries/fsm.yaml
  observe_cargo:
    kind: observe_cargo
    bin: cargo
    args: ["test", "--workspace", "--offline"]
    timeout_sec: 180
    build:
      bin: cargo
      args: ["build", "--offline", "-p", "kutha-runtime", "--bin", "kutha-tenant"]
```

`[VERIFIED: .kutha/dictionaries/fsm.yaml:23-31]`

Transition order: `run_checks` → `observe_cargo` → `emit` → `emit_tenant` → `fold` → `decide`.

```70:78:.kutha/dictionaries/fsm.yaml
  - { from: run_checks, event: done, to: observe_cargo }
  - { from: observe_cargo, event: done, to: emit }
  - { from: emit, event: ok, to: emit_tenant }
  - { from: emit, event: fail, to: fail }
  - { from: emit_tenant, event: done, to: fold }
  - { from: fold, event: ok, to: decide }
  - { from: fold, event: fail, to: fail }
  - { from: decide, event: ok, to: ok }
  - { from: decide, event: fail, to: fail }
```

`[VERIFIED: .kutha/dictionaries/fsm.yaml:70-78]`

### Trajectory check + lifecycles needles

```7:42:.kutha/dictionaries/checks.yaml
  - id: trajectory
    description: Active Milestone/Slice are None or exist on ROADMAP
    steps:
      - kind: file_exists
        path: .kutha/STATE.md
        stop_on_fail: true
      - kind: file_exists
        path: .kutha/ROADMAP.md
        stop_on_fail: true
      - kind: pointer_in_other_file
        path: .kutha/STATE.md
        pattern: '^\*\*Active Milestone:\*\*\s*(?P<id>M\d+|None)\s*$'
        flags: [multiline]
        other: .kutha/ROADMAP.md
        skip_values: ["None"]
        inflation_pattern: '\bM\d{3}\b'
        inflation_max: 12
      - kind: pointer_in_other_file
        path: .kutha/STATE.md
        pattern: '^\*\*Active Slice:\*\*\s*(?P<id>S\d+|None)\s*$'
        flags: [multiline]
        other: .kutha/ROADMAP.md
        skip_values: ["None"]
        label: Active Slice
        require_template: "- [ ] **{id}:"
        require_message: "Active Slice {id} is leased but ROADMAP marks it done (missing `{needle}`); close the lease or reopen the checkbox"

  - id: lifecycles
    description: STATE names three orthogonal lifecycles
    steps:
      - kind: file_contains
        path: .kutha/STATE.md
        needles: ["L_map=", "L_delivery=", "L_capability="]
        require: all
        category: collapse
        message: "STATE missing lifecycle rows: {missing}"
```

`[VERIFIED: .kutha/dictionaries/checks.yaml:7-42]`

### Typed Op (PLANE-02)

```18:53:crates/kutha-common/src/event.rs
pub enum Op {
    /// N-ary objects: subject, relation, object (interned).
    /// `claim`: when set, this Assert is another support for that claim id;
    /// when absent, the new event id becomes the claim id (ADR-011 / M011 S01).
    Assert {
        subject: TermId,
        relation: TermId,
        object: TermId,
        valid_from: ValidTime,
        valid_to: Option<ValidTime>,
        #[serde(default)]
        claim: Option<EventId>,
    },
    Retract {
        fact_seq: u64,
    },
    Correct {
        fact_seq: u64,
        object: TermId,
        valid_from: ValidTime,
        valid_to: Option<ValidTime>,
    },
    /// Behavior-emitted follow-on (still a log event; never LLM narrative).
    Behavior {
        name: String,
        caused_by: EventId,
        subject: TermId,
        relation: TermId,
        object: TermId,
        valid_from: ValidTime,
        valid_to: Option<ValidTime>,
    },
    /// Logged term definition (ADR-011 / M010 S02). Fold no-op; not a graph fact.
    Define {
        name: String,
    },
}
```

`[VERIFIED: crates/kutha-common/src/event.rs:18-53]`

### Relation schemas (PLANE-03)

```1:1:.kutha/dictionaries/relations.yaml
schema: kutha-harness-relations/v1
```

```1:1:crates/kutha-runtime/dictionaries/relations.yaml
schema: kutha-relations/v1
```

`[VERIFIED: .kutha/dictionaries/relations.yaml:1]` · `[VERIFIED: crates/kutha-runtime/dictionaries/relations.yaml:1]`

## Concrete probe catalog (planner → VERIFICATION)

Use these exact commands; record exit + one-line result in the evidence table.

| Req | Probe | Pass criterion |
|-----|-------|----------------|
| GOV-01 | `uv run kutha-gov ci` | Exit **0**; summary `0 HIGH`; SUMMARY includes green≠Accepted≠L_capability |
| GOV-01 | `uv run kutha-gov explain trajectory` | Exit 0; contains `authority: none — harness does not accept ADRs or claim product readiness` |
| GOV-02 | `rg -n 'L_map=|L_delivery=|L_capability=' .kutha/STATE.md` | All three needles present on distinct rows |
| GOV-02 | `uv run kutha-gov precommit --check lifecycles` | Exit 0; `high=0` |
| PLANE-01 | `ls crates/` | Only `kutha-common` `kutha-runtime` (workspace members) |
| PLANE-01 | `test -d scripts/kutha_gov && test -d .kutha && echo ok` | `ok` |
| PLANE-01 | `find crates/kutha-runtime -name '*.py' \| wc -l` | `0` |
| PLANE-01 | `test ! -e ports -a ! -e adapters -a ! -e domain && echo ok` | `ok` (no governor check covers root hexagon — **path probe required**) |
| PLANE-02 | `rg -n 'enum Op' -A20 crates/kutha-common/src/event.rs` | Variants Assert/Retract/Correct/Behavior/Define |
| PLANE-02 | `rg -n 'object\.created\|merge.patch\|merge_patch' crates --glob '*.rs'` | No matches |
| PLANE-03 | `head -1 .kutha/dictionaries/relations.yaml` | `schema: kutha-harness-relations/v1` |
| PLANE-03 | `head -1 crates/kutha-runtime/dictionaries/relations.yaml` | `schema: kutha-relations/v1` |
| PLANE-03 | `uv run kutha-gov precommit --check plane-mix-dicts` | Exit 0 |
| FREEZE-01 | Cite `.kutha/STATE.md` Freeze + `Active Slice: None` | Still documents freeze / no M002 lease |
| FREEZE-01 | `rg -ni 'rocksdb\|hnsw\|cypher\|neo4j\|graphiti' crates/*/Cargo.toml Cargo.toml` | No matches |
| FREEZE-01 | `uv run kutha-gov precommit --check freeze` | Exit 0 |
| MAP-01 | `uv run kutha-gov map \| head -30` | Cells show `map` column **Proposed** |
| MAP-01 | `rg -n 'Do not plan ADR-010' .planning/ROADMAP.md` | Hit present (no phase-per-cell) |
| MAP-01 | `uv run kutha-gov precommit --check honeycomb-map` | Exit 0 |
| MAP-01 | `uv run kutha-gov precommit --check adr-status` | Exit 0 (forbids Accepted on ADR-0* honeycomb) |
| D-15 | `cargo test --workspace --offline` | Exit 0 at tracer + pre-verify |

**Baseline this session:** `ci` exit 0, `0 HIGH, 0 LOW, 27 checks`; `explain trajectory` exit 0; `map` exit 0; hexagon dirs absent; no frozen deps in manifests; 32 ADR-0[1-9]* Proposed files scanned with `**Proposed**` (no `**Accepted**` hits among ADR-0*).

## D-G3 improvement shapes (without thaw)

| Symptom | Allowed fix | Forbidden |
|---------|-------------|-----------|
| HIGH from docs-coupling / stale needle | Edit cited markdown or check needle under D-G3 | New product milestone |
| HIGH from freeze check (unexpected dep) | **Remove** dep / revert; do not lease M002 in Phase 2 | Adding Rocks “for later” |
| HIGH from plane-mix-dicts | Separate schema strings / relations | Merging allowlists |
| HIGH from lifecycles | Restore L_* rows or remove collapse wording | Single “status=green” |
| LOW from honeycomb-map legend / dogfood “not an ADR” | Caption clarity in ROADMAP / harness doc | Promoting cells to Accepted |
| Missing check for a control-loop claim | invariants.yaml → checks.yaml row | Python Check subclass |
| Map stage change | honeycomb.yaml row only | Treating index as backlog |

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Phase 1: cargo-only hard gate (D-03 deferred ci) | Phase 2: `kutha-gov ci` every wave | 2026-09-29 CONTEXT | Trajectory honesty becomes automated |
| Ad-hoc process scripts | Dictionary FSM + YAML checks | H0–H4 in-tree | Append row ≠ new class |
| ASVS 4.x “V5 Input Validation” label in older templates | ASVS 5.0 moves input validation under **V2 Validation and Business Logic** | ASVS 5.0.0 | Map GSD template V5 row → allowlist controls; cite ASVS 5 V2.2 |

**Deprecated/outdated:**

- Treating `precommit` as full GOV-01 substitute
- `KUTHA_GOV_FAIL_ON_WARN` as default wave gate

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | GSD template ASVS “V2 Authentication / V5 Input Validation” labels still expected in VALIDATION.md even though ASVS 5.0 renumbered chapters | Security Domain | Planner maps wrong chapter ids — mitigate by citing ASVS 5 V2.2 allowlist explicitly |
| A2 | No additional LOW-emitting checks appear before Phase 2 execute beyond the two `severity: low` steps already in checks.yaml | Pitfall 4 / D-11 | Ledger process still correct; only baseline LOW count may change |

**If empty rows needed:** Core FSM/CLI/probe claims are `[VERIFIED]` from Read + shell this session.

## Open Questions (RESOLVED)

1. **Should VERIFICATION require `kutha-gov json` on every wave even when LOW=0?** (RESOLVED)
   - **Adopted:** Require `kutha-gov json` (or equivalent LOW id list) **only when LOW > 0** for D-11 ledger; when LOW=0, SUMMARY line with HIGH=0 LOW=0 suffices.

2. **Root hexagon absence — add a YAML check under D-G3?** (RESOLVED)
   - **Adopted:** Path probe in VERIFICATION is enough for Phase 2 (PLANE-01). Optional YAML `file_absent`/`glob_none` check is D-G3 discretionary — not a must_have unless planner adds it for automation.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `uv` | All kutha-gov commands | ✓ | 0.12.13 | — |
| Python via uv | Harness | ✓ | ≥3.13 (pin 3.13) | Do not use system python3 |
| `cargo` / `rustc` | observe_cargo + D-15 smoke | ✓ | 1.98.1 | — |
| `kutha-tenant` binary | emit_tenant (built in observe) | ✓ via ci build | — | ci builds with `--offline` |
| CBM index | Structural search | ✓ (fsm.yaml/checks.yaml `metadata_changed`) | indexed_at 2026-09-16 | Read YAML source; parent may reindex |
| Rocks/Cypher/HNSW | — | N/A frozen | — | Must not install |

**Missing dependencies with no fallback:** none for Phase 2 path.

**Missing dependencies with fallback:** none blocking.

Step 2.6: env probed — uv/cargo present; hexagon dirs absent.

## Validation Architecture

> Seed for `02-VALIDATION.md`. `workflow.nyquist_validation` is **true** in `.planning/config.json`.

### Test Framework

| Property | Value |
|----------|-------|
| Framework | (A) `kutha-gov` FSM + pytest for harness; (B) cargo libtest for product smoke |
| Config file | `pyproject.toml` `[tool.pytest.ini_options]`; `.kutha/dictionaries/fsm.yaml` |
| Quick run command | `uv run kutha-gov precommit` |
| Full suite command | `uv run kutha-gov ci` (+ `cargo test --workspace --offline` at tracer/pre-verify) |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| GOV-01 | ci fail-closed on HIGH; green ≠ Accepted | smoke + process | `uv run kutha-gov ci`; SUMMARY sentence | ✅ harness; ❌ 02-VERIFICATION Wave 0 |
| GOV-02 | three L_* named separately | dictionary check | `uv run kutha-gov precommit --check lifecycles` | ✅ |
| PLANE-01 | plane paths / no py in runtime / no root hexagon | path probes | catalog commands above | ✅ sources; ❌ VERIFICATION table Wave 0 |
| PLANE-02 | typed Op only | source probe | `rg` Op + no merge-patch | ✅ |
| PLANE-03 | distinct relation schemas | dictionary check | `precommit --check plane-mix-dicts` + head -1 | ✅ |
| FREEZE-01 | freeze lease + absence | check + path | `precommit --check freeze` + Cargo.toml rg | ✅ |
| MAP-01 | honeycomb map / Proposed | map CLI + checks | `kutha-gov map`; `precommit --check honeycomb-map adr-status` | ✅ |

### Sampling Rate

- **Per task commit:** `uv run kutha-gov precommit` (if harness YAML/docs touched)
- **Per wave merge:** `uv run kutha-gov ci` (D-G1) + D-10 SUMMARY block
- **Phase gate:** `ci` HIGH-free + D-11 WARN ledger + D-15 cargo smoke + completed probe tables before `/gsd-verify-work`

### Wave 0 Gaps

- [ ] `.planning/phases/02-honest-harness-and-freeze/02-VERIFICATION.md` — evidence tables for GOV/PLANE/FREEZE/MAP (create in tracer)
- [ ] `.planning/phases/02-honest-harness-and-freeze/02-VALIDATION.md` — Nyquist doc seeded from this section
- [ ] Wave SUMMARY template fragment for § Trajectory (D-10) — optional checklist in PLAN, not a new code file

None for harness/product *test code* — existing `ci` + checks cover automated gates. Gaps are GSD evidence artifacts only.

Existing harness automation (reuse, do not rewrite):

- `scripts/tests/test_fsm.py::FsmTests::test_ci_quantum_reaches_ok_on_this_tree`
- `scripts/tests/test_kutha_gov.py` (48 tests collected this session)

## Security Domain

> `security_enforcement: true`, `security_asvs_level: 1` in `.planning/config.json`.

### Applicable ASVS Categories

GSD template labels (left) mapped for this CLI/harness phase. ASVS **5.0** moved classical “input validation” under **V2 Validation and Business Logic** (V2.2 allow lists) `[CITED: https://github.com/OWASP/ASVS/blob/v5.0.0/5.0/en/0x11-V2-Validation-and-Business-Logic.md]`.

| ASVS Category (GSD template) | Applies | Standard Control |
|------------------------------|---------|------------------|
| V2 Authentication | no | N/A — no auth surface |
| V3 Session Management | no | N/A |
| V4 Access Control | no | Single-user CLI process gate |
| V5 Input Validation | yes (narrow) | Fail-closed relation allowlists + unknown kind → HIGH; ASVS 5 ≈ V2.2 allow-list validation |
| V6 Cryptography | no | No new crypto in Phase 2 |

### Known Threat Patterns for harness verification

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Treat governor green as product Accepted | Spoofing / repudiation of readiness | D-10 mandatory sentence; explain `authority: none` |
| Silent LOW / WARN | Tampering with evidence | D-11 ledger; unlogged LOW = failure |
| Freeze bypass via new crate dep | Elevation of privilege (process) | check `freeze` + Cargo.toml absence probes |
| Plane-mix / shared allowlist | Tampering with SoT boundary | `plane-mix-dicts` HIGH |
| Unknown FSM kind / process relation | Tampering | Fail-closed HIGH; no JSONL append |
| Enable fail_on_warn to hide policy debate | Denial of service on process | Keep D-11; do not flip env as gate |

## Project Constraints (from .cursor/rules/)

| Rule | Directive for Phase 2 plans |
|------|-----------------------------|
| `code-graph-cbm.mdc` | Prefer CBM for symbols; Grep for YAML/literals; never `delete_project`; never CBM `manage_adr`; `index_repository` only parent if missing/stale/asked; graph coverage ≠ governor green |
| `quiet-no-lifeos.mdc` | Do not invoke LifeOS/PAI/notify |
| `ce-skills-ru.mdc` | Russian chat routing only — not a Phase 2 product constraint |
| AGENTS.md (always_applied) | Active Slice None; freeze until M002; two planes; chat RU / docs EN; no Rocks/Cypher/HNSW/legal pack |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/02-honest-harness-and-freeze/02-CONTEXT.md` — D-G1…G3, D-10…D-15
- `.kutha/dictionaries/fsm.yaml` — quantum + observe_cargo args + completion comment
- `.kutha/dictionaries/checks.yaml` — trajectory, lifecycles, freeze, plane-mix-dicts, honeycomb-map, adr-status
- `scripts/kutha_gov/__main__.py` — `cmd_ci`, `cmd_explain`, `cmd_map`, exit codes
- `scripts/kutha_gov/fsm.py` — `decide` HIGH/LOW
- `scripts/kutha_gov/protocol.py` — Severity HIGH/LOW
- `scripts/kutha_gov/observe.py` — cargo observation
- `docs/process/kutha-harness.md` — CLI surface, HIGH→exit 1, LOW unless fail-on-warn
- `docs/process/governor-intake.md` — invariants/bridges/honeycomb intake
- `.kutha/STATE.md` — L_* + freeze + Active Slice None
- `crates/kutha-common/src/event.rs` — Op enum
- Relation YAML schema headers (both planes)
- Shell baseline: `uv run kutha-gov ci` exit 0; `explain trajectory`; `map`; path probes
- CBM: `list_projects`, `search_graph` (kutha_gov symbols), `check_index_coverage` (fsm/checks `metadata_changed` — prefer Read for YAML)

### Secondary (MEDIUM confidence)

- OWASP ASVS 5.0 V2.2 Input Validation (allow lists) `[CITED: https://github.com/OWASP/ASVS/blob/v5.0.0/5.0/en/0x11-V2-Validation-and-Business-Logic.md]`
- Phase 1 `01-RESEARCH.md` / `01-VERIFICATION.md` — evidence-map pattern

### Tertiary (LOW confidence)

- Whether planner should mint a new YAML check for root hexagon dirs `[ASSUMED]` A2 companion — path probe sufficient

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — pinned uv/Python/cargo + in-tree kutha-gov
- Architecture: HIGH — FSM/CLI Read + live `ci` baseline
- Pitfalls: HIGH — observed print suppression + decide semantics
- ASVS chapter mapping: MEDIUM — ASVS 5 renumber vs GSD template labels

**Research date:** 2026-09-29
**Valid until:** 2026-10-29 (harness stable; re-verify if fsm/checks dictionaries change materially)

## RESEARCH COMPLETE

**Phase:** 02 - Honest harness and freeze
**Confidence:** HIGH
