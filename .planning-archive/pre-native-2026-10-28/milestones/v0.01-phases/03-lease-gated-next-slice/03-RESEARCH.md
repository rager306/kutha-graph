# Phase 3: Lease-gated next slice - Research

**Researched:** 2026-09-29
**Domain:** Lease-gated delivery gate / negative proof (Active Slice None; GOV-03 / NEXT-01 / NEXT-02)
**Confidence:** HIGH (in-repo STATE + governor checks + Phase 2 gate reuse); MEDIUM on ASVS template label mapping (inherited)

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-L1:** Complete Phase 3 as **verification-only** under Active Slice None: prove GOV-03 / NEXT-01 / NEXT-02 with evidence probes + governor cycle; **do not** leave the phase open waiting for a future lease. A future named slice is a **new** discuss/plan (or inserted phase), not an infinite hold on Phase 3. — **Reversibility:** reversible
- **D-L2:** “New product-crate slice work” means **new leased product capability** in `crates/kutha-*` beyond the already-landed M011 S03 surface (new slice modules/APIs/features named as Active Slice delivery). **Not** a violation: GSD/docs, harness YAML under D-G3, FIT/GOV regression evidence, clarifying comments, or re-running existing tests. Evidence: `.kutha/STATE.md` still shows `Active Slice: None` (or equivalent bold cite); VERIFICATION records no new slice-delivery commits/paths for a named Active Slice; no legal-pack corpus start. — **Reversibility:** reversible
- **D-L3:** Inherit Phase 2 **D-G1…D-G3** and companions **D-10 / D-11 / D-15** for every Phase 3 execute wave (ci + explain trajectory in SUMMARY; HIGH stops; WARN ledgered; cargo at tracer + pre-verify unless `crates/` touched). Do not weaken to precommit-only. — **Reversibility:** reversible
- **D-L4:** If during execute `.kutha/STATE.md` Active Slice changes from None to a named value: **hard stop** the current verification plan; record in STATE/VERIFICATION; **do not** implement that slice under Phase 3 plans. Human chooses `/gsd-discuss-phase` / plan for the leased slice or an inserted phase. — **Reversibility:** reversible — process gate
- **D-L5:** While M011 is not closed in `.kutha/STATE.md`, NEXT-02 is proven **negatively**: captions/ROADMAP/STATE do not treat M002 or “implement honeycomb” as the implied next product milestone; no M002/Rocks/Cypher/HNSW scaffolding started; prose cites “next milestone = whatever STATE names after M011 close.” Do not require M011 to be closed to finish Phase 3. — **Reversibility:** reversible
- **D-L6:** NEXT-01 / legal-pack fence uses **path + lease cite** (same spirit as Phase 2 FREEZE/MAP): Active Slice None; no legal corpus/pack delivery tree started as substitute; honeycomb ADRs remain Proposed/map; ROADMAP still not phase-per-cell. Optional D-G3 clarity edits only. — **Reversibility:** reversible

### Claude's Discretion
User said «реши эти вопросы разумно» for gray areas 1–6 → D-L1…D-L6. Planner may tighten probe command strings; must not start a leased slice, thaw freeze, or assume M002 under this CONTEXT.

### Deferred Ideas (OUT OF SCOPE)
- Implementing a named Active Slice — new phase after STATE lease
- Closing M011 in harness STATE — harness process, not GSD Phase 3 delivery
- Assumed M002 start — freeze until explicit lease
- Legal corpus / ADR-090 pack — out of scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| GOV-03 | New product-crate slice work starts only when `.kutha/STATE.md` names that Active Slice (today: None) | Active Slice None cite + `trajectory` check note `no Active Slice` + empty `crates/` porcelain / no post-S03 crate commits + VERIFICATION “no named-slice delivery” row (D-L1/D-L2) |
| NEXT-01 | Further M011 slices beyond S03 only under lease; legal pack not a substitute | Lease cite + legal-pack path absence + `h4-lease` / honeycomb ADR-090 `delivery: frozen` + ADR-090 **Proposed** + freeze absence probes (D-L6) |
| NEXT-02 | After M011 closed, next milestone = whatever STATE names — not assumed M002 / not “implement honeycomb” | Negative proof while M011 still open (D-L5): captions + ROADMAP milestone line + no frozen-dep scaffolding; do **not** wait for M011-closed |
</phase_requirements>

## Summary

Phase 3 is **verification-first / negative proof** on the lease gate while harness lease stays **Active Slice: None**. It does **not** deliver a new M011 product slice, does **not** start a legal pack, and does **not** assume M002. Success under today’s lease is a green evidence map for GOV-03 / NEXT-01 / NEXT-02 plus the inherited Phase 2 governor cycle (D-L3 → D-G1/D-G2/D-G3 + D-10/D-11/D-15).

Harness already encodes the lease needle and many fences: check `trajectory` parses `**Active Slice:**` / `None`; `freeze` / `honeycomb-map` / `h4-lease` / ADR-090 honeycomb row block thaw and pack promotion. GSD must **observe and record** those facts (and crate quiescence), not invent a parallel enforcer. Mid-phase lease appearance is a **hard stop** (D-L4), not silent expansion into product delivery under Phase 3 plans.

`.planning/ROADMAP.md` Overview currently mixes “Phase 3 is the only *new* product work” with “Phase 3 is blocked until STATE names a slice.” Both conflict with **D-L1** (complete verification now; do not wait). Planner should schedule a **D-G3 caption clarity** edit so Overview matches verification-only posture — without rewriting harness `.kutha/STATE.md`.

**Primary recommendation:** Plan 2–3 verify-only waves mirroring Phase 2: (1) tracer — `ci` + `explain trajectory` + cargo smoke + VERIFICATION/VALIDATION skeletons + D-L4 lease snapshot; (2) paint GOV-03/NEXT-01/NEXT-02 probe tables + ROADMAP Overview clarity if needed; (3) batch REQUIREMENTS GOV-03/NEXT-01/NEXT-02 + GSD STATE closeout — zero product features, zero freeze thaw, zero legal pack.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Active Slice lease cite | Harness `.kutha/STATE.md` | check `trajectory` | Process lease SoT for delivery gate; GSD cites only |
| GOV-03 negative proof (no new crate slice) | GSD VERIFICATION probes (`git` + path) | Product crates (read-only) | Absence of post-S03 slice delivery; not a new runtime feature |
| NEXT-01 legal-pack fence | Path probes + honeycomb/h4 checks | ADR-090 Proposed doc | Same spirit as FREEZE/MAP; do not implement pack |
| NEXT-02 anti-assumption | GSD ROADMAP/STATE captions + harness Next action | check `honeycomb-map` / `freeze` | Negative prose while M011 open (D-L5) |
| Mid-phase lease change (D-L4) | Execute wave process gate | `.kutha/STATE.md` re-read | Hard stop → new discuss/plan; not Phase 3 delivery |
| Governor honesty cycle | `uv run kutha-gov ci` + `explain trajectory` | SUMMARY § Trajectory (D-10) | Inherited D-L3 / D-G1 |
| Product regression smoke | `cargo test --workspace --offline` | `observe_cargo` inside `ci` | D-15: tracer + pre-verify (+ if crates touched) |
| Honeycomb L_map discipline | `.kutha/dictionaries/honeycomb.yaml` + `map` CLI | ADR Status | Map ≠ backlog ≠ Accepted |

## Standard Stack

### Core

| Library / Tool | Version | Purpose | Why Standard |
|----------------|---------|---------|--------------|
| `uv run kutha-gov` | uv 0.12.13 `[VERIFIED: shell uv --version]` | Wave gate + explain + map + precommit | Composition root; D-L3 inherits Phase 2 |
| Python | pin `.python-version` = `3.13` `[VERIFIED: .python-version]` | Harness runtime | AGENTS forbids system python for gov |
| `.kutha/dictionaries/checks.yaml` | in-tree | Lease/freeze/map/h4 checks | Already encodes Active Slice regex |
| `cargo test --workspace --offline` | cargo/rustc 1.98.1 `[VERIFIED: shell]` | D-15 product smoke | Same args as `fsm.yaml` observe_cargo |
| Git porcelain / log | system git | GOV-03 crate quiescence evidence | Discrete absence proof for D-L2 |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|----------------|---------|---------|-------------|
| `uv run kutha-gov explain trajectory` | in-tree | D-10 trajectory reading | Every wave SUMMARY |
| `uv run kutha-gov precommit --check <id>` | in-tree | Fast single-check probes | trajectory / freeze / honeycomb-map / h4-lease / lifecycles |
| `uv run kutha-gov map` | in-tree | L_map Proposed smoke | NEXT-01 / MAP discipline |
| `uv run kutha-gov json` | in-tree | LOW finding text for D-11 | Only when `LOW > 0` |
| `uv run pytest` | pytest≥8 (dev) | Harness regression | Only if wave edits `scripts/kutha_gov` |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Verification-only Phase 3 close | Hold Phase 3 open until a slice is leased | **Rejects D-L1** — infinite hold |
| Implement M011 S04 under Phase 3 | Wait for lease then inserted phase | **Rejects D-L4** if lease appears mid-flight; rejects D-L1 today |
| `precommit` alone as wave gate | Full `ci` | Skips observe/emit/tenant — **rejects D-L3 / D-G1** |
| Close M011 in harness to “satisfy” NEXT-02 | Negative proof while open (D-L5) | Closing M011 is harness process, deferred |
| Start legal pack “as NEXT-01 evidence” | Path + lease absence (D-L6) | Implementing thaws freeze |

**Installation:** None — no new crates or PyPI packages for Phase 3.

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
Phase 3 execute wave (Active Slice expected: None)
        │
        ├─► [D-L4 open]  read .kutha/STATE.md Active Slice → record snapshot
        │
        ├─► uv run kutha-gov explain trajectory     ◄── D-10 excerpt (≤8 lines)
        │
        └─► uv run kutha-gov ci                     ◄── D-G1 / D-L3 HARD GATE
                  │
                  ▼
         fsm quantum (unchanged from Phase 2)
         … → run_checks → observe_cargo → emit → tenant → fold → decide
                  │
         high > 0 ──► STOP wave (D-G2)
         high == 0 ──► SUMMARY § Trajectory + WARN ledger if LOW>0 (D-11)
                  │
                  ▼
         Negative-proof probes → 03-VERIFICATION.md
           GOV-03: Active Slice None + crates quiescence
           NEXT-01: legal-pack paths absent + h4/honeycomb freeze
           NEXT-02: captions anti-M002 / anti-honeycomb-as-next
                  │
        ├─► [D-L4 close] re-read Active Slice
        │         │
        │         ├─ still None ──► continue / phase-verify
        │         └─ named S## ──► HARD STOP; do not implement under Phase 3
        │
        └─► Explicit cargo smoke (D-15): tracer + pre-verify
```

### Recommended Project Structure (unchanged product/harness; new GSD evidence)

```text
.kutha/
  STATE.md                      # lease cite only — do not overwrite from GSD
  dictionaries/checks.yaml      # trajectory Active Slice regex; freeze; h4-lease; honeycomb-map
crates/                         # read-only under Active Slice None (D-L2)
.planning/phases/03-lease-gated-next-slice/
  03-CONTEXT.md                 # D-L1…D-L6 locked
  03-RESEARCH.md                # this file
  03-VERIFICATION.md            # evidence SoT (create in execute)
  03-VALIDATION.md              # Nyquist seed from Validation Architecture
```

### Pattern 1: Wave-close governor loop (D-L3 → D-G1 / D-10)

**What:** Required automated gate + short trajectory reading (identical to Phase 2).
**When to use:** End of every execute wave and before phase-verify.
**Example:**

```bash
uv run kutha-gov ci
# Capture: exit code; line "harness: N HIGH, M LOW, …"
uv run kutha-gov explain trajectory
# Paste ≤8 lines into SUMMARY § Trajectory + green≠Accepted≠L_capability sentence
```

Authority line (verbatim this session):

```text
authority: none — harness does not accept ADRs or claim product readiness
```

`[VERIFIED: shell uv run kutha-gov explain trajectory]`

### Pattern 2: Active Slice None + crate quiescence (GOV-03 / D-L2)

**What:** Prove no new leased product capability landed while lease is None.
**When to use:** Every wave probe table + phase-verify.
**How:**

1. Cite `.kutha/STATE.md` line `**Active Slice:** None`
2. `uv run kutha-gov precommit --check trajectory` → expect note containing `no Active Slice`
3. `git status --porcelain -- crates/ .kutha/STATE.md` → empty for phase waves (STATE untouched)
4. `git log --oneline e77132d9275bd36ea766b8bef9cff28128dfc636..HEAD -- crates/` → empty (M011 S03 tip; 0 commits this session)
5. Record explicitly: no named Active Slice delivery paths in VERIFICATION

**Not a violation:** edits under `.planning/`, harness YAML under D-G3, docs/process clarity, re-running tests.

### Pattern 3: Mid-phase lease change hard stop (D-L4)

**What:** Detect Active Slice flipping from None to `S##` during execute.
**When to use:** Wave open + wave close (and before phase-verify).
**How:**

```bash
# Wave open / close (same command)
rg -n '^\*\*Active Slice:\*\*\s*' .kutha/STATE.md
```

- Pass for Phase 3 default plans: still `None`
- Fail / gate: any `S\d+` → stop plans; write VERIFICATION blocker; do **not** implement that slice under Phase 3; human starts discuss/plan or inserted phase

### Pattern 4: Legal-pack / honeycomb non-substitute (NEXT-01 / D-L6)

**What:** Path + lease cite; do not start corpus.
**Probes:** see catalog below (`h4-lease`, honeycomb ADR-090 frozen, ADR Status Proposed, absent pack trees).

### Anti-Patterns to Avoid

- **Waiting for a lease to “finish” Phase 3:** Violates D-L1
- **Implementing S04 because ROADMAP still lists M011 ACTIVE:** Lease is None; further slices need explicit lease
- **Treating ROADMAP Overview “blocked until STATE names a slice” as a hold:** Caption drift — fix under D-G3; do not stall
- **Starting legal pack / Rocks / Cypher / HNSW to satisfy NEXT-*:** Freeze is absence + cite
- **Silent Phase 3 expansion when Active Slice appears:** Must hard-stop (D-L4)
- **Closing M011 in `.kutha/STATE.md` from GSD plans:** Deferred; harness process only
- **Weakening gate to `precommit` only:** Rejects D-L3
- **Green governor = next milestone leased:** SUMMARY must keep green≠Accepted≠L_capability

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Lease honesty gate | Custom “is slice leased?” script | `trajectory` check + STATE cite | Regex + ROADMAP pointer already in YAML |
| Wave honesty | Ad-hoc checklist | `uv run kutha-gov ci` | FSM fail-closed on HIGH |
| Trajectory reading | Paste events.jsonl | `explain trajectory` ≤8 lines | D-10; authority none |
| Freeze / pack fence | New Python Check class | `freeze` + `h4-lease` + path probes | Intake forbids hand-rolled classes |
| NEXT-02 while M011 open | Force M011-closed token | Negative caption probes (D-L5) | Closing M011 is out of scope |
| Mid-flight lease reaction | Keep executing Phase 3 product tasks | D-L4 hard stop + new discuss | Prevents silent sprawl |

**Key insight:** Phase 3 ships **evidence that delivery did not start without a lease**, not the next slice itself. The harness already knows `Active Slice: None`; GSD must not invent delivery to look busy.

## Concrete probe catalog (planner → VERIFICATION)

Use these exact commands; record exit + one-line result in the evidence table.

| Req | Probe | Pass criterion |
|-----|-------|----------------|
| GOV-03 | `rg -n '^\*\*Active Slice:\*\*' .kutha/STATE.md` | Line is `**Active Slice:** None` |
| GOV-03 | `uv run kutha-gov precommit --check trajectory` | Exit 0; output includes `no Active Slice` (this session: `OK trajectory high=0 low=0  no Active Slice`) |
| GOV-03 | `git status --porcelain -- crates/` | Empty during Phase 3 waves |
| GOV-03 | `git log --oneline e77132d9275bd36ea766b8bef9cff28128dfc636..HEAD -- crates/` | Empty (S03 tip = thin P→Q commit) |
| GOV-03 | VERIFICATION prose row | States: no named Active Slice delivery; GSD/docs/harness-only diffs OK (D-L2) |
| NEXT-01 | Cite `.kutha/STATE.md` Next action | Contains `Do **not** start a legal pack` |
| NEXT-01 | `for p in packs/legal legal-corpus corpus crates/kutha-legal crates/kutha-pack; do test ! -e \"$p\" \|\| exit 1; done; echo absent` | Prints `absent` |
| NEXT-01 | `uv run kutha-gov precommit --check h4-lease` | Exit 0; `high=0` |
| NEXT-01 | `rg -n 'id: ADR-090' -A8 .kutha/dictionaries/honeycomb.yaml` | Shows `map: Proposed`, `delivery: frozen`, `freeze_as: "full ADR-090 ontology"` |
| NEXT-01 | `rg -n '^\*\*Proposed\*\*' docs/ADR/ADR-090-legal-reference-pack.md` | Hit under Status |
| NEXT-01 | `rg -ni 'rocksdb\|hnsw\|cypher\|neo4j\|graphiti' crates/*/Cargo.toml Cargo.toml` | No matches |
| NEXT-01 | `uv run kutha-gov precommit --check freeze` | Exit 0 |
| NEXT-01 | `rg -n 'Do not plan ADR-010' .planning/ROADMAP.md` | Hit (no phase-per-cell) |
| NEXT-02 | `rg -n '^\*\*Active Milestone:\*\*\|^L_delivery=' .kutha/STATE.md` | `Active Milestone: M011`; `L_delivery=M011-S03-done` (not `*-closed`) |
| NEXT-02 | `rg -n 'not assumed to be M002' .planning/ROADMAP.md` | Hit on Next STATE-named milestone |
| NEXT-02 | `rg -ni 'next.*(is\|=\|milestone).*M002\|implement honeycomb' .kutha/STATE.md .planning/STATE.md` | No implication that next product work **is** M002 or “implement honeycomb” (allow fencing sentences that *forbid* those) |
| NEXT-02 | `uv run kutha-gov precommit --check honeycomb-map` | Exit 0 |
| NEXT-02 | VERIFICATION / SUMMARY sentence | Explicit: next product milestone = whatever STATE names after M011 close — not assumed M002 / not implement honeycomb |
| D-L4 | Wave-open + wave-close Active Slice rg | Both `None`; if named → hard stop recorded |
| D-L3 / GOV-01 inherit | `uv run kutha-gov ci` | Exit 0; `0 HIGH`; SUMMARY green≠Accepted≠L_capability |
| D-L3 / D-10 | `uv run kutha-gov explain trajectory` | Exit 0; authority none line present |
| D-15 | `cargo test --workspace --offline` | Exit 0 at tracer + pre-verify |

**Baseline this session:**

- `.kutha/STATE.md`: `**Active Slice:** None`; `**Active Milestone:** M011`; `L_delivery=M011-S03-done` `[VERIFIED: .kutha/STATE.md:7-15]`
- `precommit --check trajectory` → `OK … no Active Slice`; `freeze` / `honeycomb-map` / `h4-lease` → `high=0`
- `git log e77132d..HEAD -- crates/` → **0** commits
- Legal pack trees (`packs/legal`, `legal-corpus`, `corpus`, `crates/kutha-legal`, `crates/kutha-pack`) → **ABSENT**
- ADR-090 Status: `**Proposed**` `[VERIFIED: docs/ADR/ADR-090-legal-reference-pack.md:3-5]`
- Honeycomb ADR-090: `delivery: frozen` / `freeze_as: "full ADR-090 ontology"` `[VERIFIED: .kutha/dictionaries/honeycomb.yaml:344-350]`

### Trajectory Active Slice regex (authoritative)

```24:32:.kutha/dictionaries/checks.yaml
      - kind: pointer_in_other_file
        path: .kutha/STATE.md
        pattern: '^\*\*Active Slice:\*\*\s*(?P<id>S\d+|None)\s*$'
        flags: [multiline]
        other: .kutha/ROADMAP.md
        skip_values: ["None"]
        label: Active Slice
        require_template: "- [ ] **{id}:"
        require_message: "Active Slice {id} is leased but ROADMAP marks it done (missing `{needle}`); close the lease or reopen the checkbox"
```

`[VERIFIED: .kutha/dictionaries/checks.yaml:24-32]`

### Legal-pack deferred invariant (not a live check)

```130:134:.kutha/dictionaries/invariants.yaml
  - id: I-legal-pack-not-next
    claim: "A legal corpus pack is not the next harness delivery after H4 overlay dogfood"
    source: "docs/process/kutha-harness.md"
    disposition: deferred
    until: "STATE names a legal pack"
```

`[VERIFIED: .kutha/dictionaries/invariants.yaml:130-134]` — Phase 3 must **not** promote this to delivery; prove with path probes + `h4-lease` instead of implementing until-clause.

## Common Pitfalls

### Pitfall 1: Holding Phase 3 open for a future lease

**What goes wrong:** Plans say “blocked until Active Slice named” and never close.
**Why it happens:** ROADMAP Overview line 9 still says Phase 3 is blocked until STATE names a slice `[VERIFIED: .planning/ROADMAP.md:9]`.
**How to avoid:** Follow D-L1 — complete negative proof now; clarity-edit Overview under D-G3.
**Warning signs:** Plans with no VERIFICATION closeout until lease appears.

### Pitfall 2: Counting any `crates/` touch as GOV-03 fail

**What goes wrong:** Clarifying comment or FIT regression fix painted as slice delivery.
**Why it happens:** Over-reading D-L2.
**How to avoid:** Violation = **new leased product capability** named as Active Slice delivery. Document intent in VERIFICATION; prefer zero crates diffs.
**Warning signs:** Blocking phase on whitespace-only crate edits.

### Pitfall 3: Missing D-L4 when STATE flips mid-execute

**What goes wrong:** Executor implements newly leased slice under Phase 3 plan IDs.
**Why it happens:** Lease check only at phase start.
**How to avoid:** Wave-open and wave-close Active Slice probes; hard stop text in PLAN.
**Warning signs:** SUMMARY claims GOV-03 pass while Active Slice is `S04`.

### Pitfall 4: “Satisfying” NEXT-02 by closing M011 or starting M002

**What goes wrong:** GSD edits `.kutha/STATE.md` to `M011-closed` or scaffolds Rocks.
**Why it happens:** Reading success criterion 3 as a delivery task.
**How to avoid:** D-L5 negative proof only; freeze absence probes.
**Warning signs:** Diff touches `.kutha/STATE.md` Active Milestone / L_delivery or adds rocksdb.

### Pitfall 5: Legal pack tree as “documentation”

**What goes wrong:** Adding `crates/kutha-legal` or corpus directories “for later.”
**Why it happens:** Confusing ADR-090 Proposed map cell with delivery start.
**How to avoid:** D-L6 path absence; keep ADR Proposed / honeycomb frozen.
**Warning signs:** New top-level `corpus/` or pack crate in `Cargo.toml` members.

### Pitfall 6: WARN/`--fail-on-warn` confusion (inherited)

**What goes wrong:** Enabling fail-on-warn collapses D-11.
**How to avoid:** Same as Phase 2 — HIGH stops; ledger LOWs; `kutha-gov json` only if LOW>0.

## Code Examples

### `cmd_ci` exit semantics (reuse)

```190:192:scripts/kutha_gov/__main__.py
    if outcome.terminal != "ok":
        return 1
    return 0
```

`[VERIFIED: scripts/kutha_gov/__main__.py:190-192]` · CBM `get_code_snippet` on `cmd_ci`

### `cmd_explain` authority line

```123:131:scripts/kutha_gov/__main__.py
def cmd_explain(root: Path, name: str) -> int:
    checks = get_checks(root)
    chk = checks.get(name)
    if not chk:
        print(f"unknown check: {name}", file=sys.stderr)
        return 2
    print(f"check: {chk.name}")
    print(f"purpose: {chk.description}")
    print("authority: none — harness does not accept ADRs or claim product readiness")
```

`[VERIFIED: scripts/kutha_gov/__main__.py:123-131]`

### Lease cite in STATE (today)

```7:9:.kutha/STATE.md
**Active Milestone:** M011
**Active Slice:** None
**Phase:** H4
```

`[VERIFIED: .kutha/STATE.md:7-9]`

### D-L4 detection snippet (shell)

```bash
SLICE=$(rg -N '^\*\*Active Slice:\*\*\s*(\S+)' -or '$1' .kutha/STATE.md)
# Phase 3 default plans require: None
# If S01|S02|… → hard stop (D-L4)
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| ROADMAP: Phase 3 “blocked until STATE names a slice” | D-L1: verification-only complete under None | 2026-09-29 CONTEXT | Phase 3 closable without product delivery |
| Assume next milestone is M002 | STATE-named only; negative NEXT-02 | STRATEGY / REQUIREMENTS | Freeze holds |
| Phase 2 deferred GOV-03 | Phase 3 proves GOV-03 negatively | This phase | Lease gate becomes falsifiable evidence |

**Deprecated/outdated:**

- Treating Phase 3 as the delivery vehicle for the next Active Slice while lease is None
- Using `precommit` alone as the Phase 3 wave gate

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | M011 S03 tip commit `e77132d9275bd36ea766b8bef9cff28128dfc636` remains the correct crates baseline for “no post-S03 product commits” through Phase 3 execute | GOV-03 probes | If new legitimate non-slice crate commits land from another track, update baseline hash in VERIFICATION — still require Active Slice None + D-L2 intent |
| A2 | Absent path set `{packs/legal, legal-corpus, corpus, crates/kutha-legal, crates/kutha-pack}` is sufficient legal-pack tree probe for D-L6 (no official pack root exists yet) | NEXT-01 | A pack could start under a novel path — mitigate with freeze Cargo.toml probe + honeycomb ADR-090 frozen + STATE “do not start a legal pack” cite |

**If this table is empty:** N/A — two assumed operational baselines documented above.

## Open Questions (RESOLVED)

1. **Does Phase 3 stay blocked until an Active Slice is named?** (RESOLVED)
   - What we know: ROADMAP Overview says blocked `[VERIFIED: .planning/ROADMAP.md:9]` and also “only *new* product work” `[VERIFIED: .planning/ROADMAP.md:5]`.
   - **Adopted (D-L1):** Complete Phase 3 as verification-only under Active Slice None. Do not wait. Schedule D-G3 clarity edits so Overview states verification-first / lease-gated *negative proof*, and that *product* slice delivery waits for a named lease (new phase).

2. **How is GOV-03 proven without implementing a slice?** (RESOLVED)
   - **Adopted (D-L2):** Active Slice None cite + `trajectory` `no Active Slice` + empty crates porcelain + empty `git log` after M011 S03 tip + VERIFICATION statement of no named-slice delivery. Docs/harness/GSD edits allowed.

3. **Can NEXT-02 pass while M011 is still open?** (RESOLVED)
   - **Adopted (D-L5):** Yes — negative caption/dep probes only; do not require `M011-closed` or start M002.

4. **What happens if Active Slice becomes named mid-phase?** (RESOLVED)
   - **Adopted (D-L4):** Hard stop current verification plans; record in VERIFICATION/GSD STATE; do not implement under Phase 3; human starts discuss/plan or inserted phase.

5. **Is `I-legal-pack-not-next` (deferred) enough for NEXT-01?** (RESOLVED)
   - **Adopted (D-L6):** Use path + lease cite + `h4-lease` + honeycomb ADR-090 frozen + ADR Proposed. Do **not** flip deferred invariant into a new HIGH check unless D-G3 explicitly adds one; absence probes are the Phase 3 evidence.

6. **Must Wave 0 invent new automated tests?** (RESOLVED)
   - **Adopted:** No new product/harness test files required. Wave 0 = GSD `03-VERIFICATION.md` + `03-VALIDATION.md` Task IDs; reuse `kutha-gov ci` / precommit / cargo.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `uv` | kutha-gov gates | ✓ | 0.12.13 | — |
| Python via uv | Harness | ✓ | 3.13 pin | Do not use system python3 |
| `cargo` / `rustc` | D-15 + observe_cargo | ✓ | 1.98.1 | — |
| `git` | GOV-03 quiescence probes | ✓ | system | — |
| CBM index | Symbol lookup | ✓ project `kutha-graph` | indexed_at 2026-09-16; `checks.yaml`/`STATE.md` freshness `metadata_changed` | Prefer Read for YAML/STATE; parent may reindex |
| Exa/Jina MCP | External research-plan fetch | ✗ this session | — | Inherit Phase 2 ASVS cite; in-repo evidence primary |
| Rocks/Cypher/HNSW | — | N/A frozen | — | Must not install |

**Missing dependencies with no fallback:** none for Phase 3 path.

**Missing dependencies with fallback:** Exa/Jina MCP unavailable — used in-repo + Phase 2 ASVS citation.

Step 2.6: env probed — uv/cargo/git present; legal pack trees absent; crates porcelain clean.

## Validation Architecture

> Seed for `03-VALIDATION.md`. `workflow.nyquist_validation` is **true** in `.planning/config.json` `[VERIFIED: .planning/config.json workflow.nyquist_validation]`.

### Test Framework

| Property | Value |
|----------|-------|
| Framework | (A) `kutha-gov` FSM + pytest harness; (B) cargo libtest for product smoke; (C) shell path/git probes for lease negative proof |
| Config file | `pyproject.toml` `[tool.pytest.ini_options]`; `.kutha/dictionaries/fsm.yaml` |
| Quick run command | `uv run kutha-gov precommit` |
| Full suite command | `uv run kutha-gov ci` (+ `cargo test --workspace --offline` at tracer / pre-verify) |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| GOV-03 | No new product-slice delivery while Active Slice None | path + git + check | catalog GOV-03 rows; `precommit --check trajectory` | ✅ sources; ❌ 03-VERIFICATION Wave 0 |
| NEXT-01 | No further M011 slice / no legal-pack substitute | path + check | catalog NEXT-01 rows; `h4-lease` + `freeze` | ✅ sources; ❌ VERIFICATION Wave 0 |
| NEXT-02 | Next milestone not assumed M002 / not implement honeycomb | prose + check | catalog NEXT-02 rows; `honeycomb-map` | ✅ captions; ❌ VERIFICATION Wave 0 |
| D-L3 inherit | ci HIGH-free + trajectory SUMMARY | smoke + process | `uv run kutha-gov ci`; `explain trajectory` | ✅ harness |
| D-L4 | Lease unchanged None across wave | process probe | Active Slice rg open/close | ✅ STATE; ❌ VERIFICATION row Wave 0 |
| D-15 | Product smoke | cargo | `cargo test --workspace --offline` | ✅ |

### Sampling Rate

- **Per task commit:** `uv run kutha-gov precommit` when harness YAML/docs touched; always re-check Active Slice if `.kutha/STATE.md` could change
- **Per wave merge:** `uv run kutha-gov ci` (D-G1) + D-10 SUMMARY + D-L4 lease snapshot pair
- **Phase gate:** `ci` HIGH-free + D-11 WARN ledger + D-15 cargo smoke + completed GOV-03/NEXT probe tables + D-L4 still None before `/gsd-verify-work`

### Wave 0 Gaps

- [ ] `.planning/phases/03-lease-gated-next-slice/03-VERIFICATION.md` — GOV-03 / NEXT-01 / NEXT-02 / D-L4 evidence tables
- [ ] `.planning/phases/03-lease-gated-next-slice/03-VALIDATION.md` — Nyquist doc seeded from this section; Task IDs filled by tracer plan
- [ ] Optional D-G3: clarify `.planning/ROADMAP.md` Overview lines 5 and 9 to match D-L1 (verification-only; not infinite hold)

None for harness/product *test code* — existing `ci` + checks cover automated gates. Gaps are GSD evidence artifacts (+ optional caption clarity).

Existing harness automation (reuse):

- `scripts/tests/test_fsm.py` / `scripts/tests/test_kutha_gov.py`
- Checks: `trajectory`, `freeze`, `honeycomb-map`, `h4-lease`, `lifecycles`, `adr-status`

## Security Domain

> `security_enforcement: true`, `security_asvs_level: 1` in `.planning/config.json`.

### Applicable ASVS Categories

Inherit Phase 2 mapping. ASVS **5.0** places allow-list style validation under **V2 Validation and Business Logic** (V2.2) `[CITED: https://github.com/OWASP/ASVS/blob/v5.0.0/5.0/en/0x11-V2-Validation-and-Business-Logic.md]` (Exa MCP unavailable this session; citation carried from Phase 2 research).

| ASVS Category (GSD template) | Applies | Standard Control |
|------------------------------|---------|------------------|
| V2 Authentication | no | N/A |
| V3 Session Management | no | N/A |
| V4 Access Control | no (narrow process) | Lease gate is process policy, not multi-user auth |
| V5 Input Validation | yes (narrow) | Fail-closed relation allowlists + unknown kind → HIGH; Active Slice pointer validation via `trajectory` |
| V6 Cryptography | no | No new crypto |

### Known Threat Patterns for lease-gated verification

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Deliver product slice without Active Slice lease | Elevation of privilege (process) | GOV-03 probes + D-L1/D-L2 |
| Substitute legal pack for leased slice | Tampering with roadmap intent | NEXT-01 path + h4/honeycomb freeze |
| Assume M002 after Phase 2/3 green | Spoofing of next milestone | NEXT-02 negative captions + freeze |
| Mid-phase lease hijack of Phase 3 plans | Tampering / repudiation | D-L4 hard stop |
| Treat governor green as lease grant | Spoofing readiness | D-10 authority none sentence |
| Silent LOW / WARN | Tampering with evidence | D-11 ledger |

## Project Constraints (from .cursor/rules/)

| Rule | Directive for Phase 3 plans |
|------|-----------------------------|
| `code-graph-cbm.mdc` | Prefer CBM for symbols; Grep for YAML/STATE literals; never `delete_project`; never CBM `manage_adr`; `index_repository` only parent if missing/stale/asked; graph coverage ≠ governor green |
| `quiet-no-lifeos.mdc` | Do not invoke LifeOS/PAI/notify |
| `ce-skills-ru.mdc` | Russian chat routing only — not a Phase 3 product constraint |
| AGENTS.md (always_applied) | Active Slice None; freeze until M002; two planes; chat RU / docs EN; no Rocks/Cypher/HNSW/legal pack; further M011 slices need explicit lease |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/03-lease-gated-next-slice/03-CONTEXT.md` — D-L1…D-L6
- `.planning/phases/02-honest-harness-and-freeze/02-CONTEXT.md` / `02-RESEARCH.md` — D-G1…G3, D-10/11/15 patterns
- `.kutha/STATE.md` — Active Slice None; M011; freeze; legal pack forbid
- `.kutha/dictionaries/checks.yaml` — trajectory Active Slice regex; freeze; honeycomb-map; h4-lease
- `.kutha/dictionaries/invariants.yaml` — I-legal-pack-not-next deferred
- `.kutha/dictionaries/honeycomb.yaml` — ADR-090 delivery frozen
- `docs/ADR/ADR-090-legal-reference-pack.md` — Status Proposed
- `scripts/kutha_gov/__main__.py` — `cmd_ci`, `cmd_explain`
- `.planning/ROADMAP.md` / `REQUIREMENTS.md` — Phase 3 goal + GOV-03/NEXT-*
- Shell baseline: precommit trajectory/freeze/h4/honeycomb-map; git crates quiescence; path absences
- CBM: `list_projects`, `search_graph` (`cmd_ci` / `cmd_explain` / `_kind_pointer_in_other_file`), `get_code_snippet` (`cmd_ci`), `check_index_coverage` (YAML/STATE `metadata_changed` → Read preferred)

### Secondary (MEDIUM confidence)

- OWASP ASVS 5.0 V2.2 allow lists `[CITED: ASVS v5.0.0 V2 doc on GitHub]` via Phase 2 research (Exa fetch unavailable this session)
- Phase 1 verification-first pattern (`01-CONTEXT` / VERIFICATION)

### Tertiary (LOW confidence)

- Exhaustiveness of legal-pack path set A2 `[ASSUMED]`

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — pinned uv/Python/cargo + in-tree kutha-gov (no new installs)
- Architecture: HIGH — D-L* locked + Read of checks/STATE + live precommit baseline
- Pitfalls: HIGH — ROADMAP caption drift verified; D-L4/D-L5 failure modes explicit
- External ASVS fetch: MEDIUM — inherited cite; MCP search unavailable

**Research date:** 2026-09-29
**Valid until:** 2026-10-29 (re-verify if Active Slice lease changes or checks.yaml trajectory pattern changes)

## RESEARCH COMPLETE

**Phase:** 03 - Lease-gated next slice
**Confidence:** HIGH
