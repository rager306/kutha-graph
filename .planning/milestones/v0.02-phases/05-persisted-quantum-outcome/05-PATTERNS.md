# Phase 5: Persisted quantum outcome - Pattern Map

**Mapped:** 2026-09-29
**Files analyzed:** 10
**Analogs found:** 10 / 10
**CBM:** project `kutha-graph`; `list_projects` + `trace_path(persist|emit)` + `get_code_snippet(store.persist|store.open|QuantumReceipt.from_events)`; cited paths mostly `metadata_changed` / `not_tracked` (`indexed_at` 2026-09-16) — excerpts below are from **source Read**, not stale graph SoT.

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `crates/kutha-runtime/src/store.rs` | service | file-I/O | `crates/kutha-runtime/src/store.rs` (`persist`/`open`/`write_terms`/`read_terms`) | exact |
| `crates/kutha-runtime/src/quantum.rs` | service | event-driven + transform | `crates/kutha-runtime/src/quantum.rs` (`Runtime`, `emit`, `QuantumOutcome`, budget unit tests) | exact |
| `crates/kutha-runtime/src/receipt.rs` | model | transform | `crates/kutha-runtime/src/receipt.rs` (`QuantumReceipt`) + `crates/kutha-runtime/src/snapshot.rs` (serde row shape) | role-match |
| `crates/kutha-runtime/src/lib.rs` | config | request-response | `crates/kutha-runtime/src/lib.rs` (`pub use` barrel) | exact |
| `crates/kutha-runtime/tests/m011_quantum_outcome.rs` | test | file-I/O + request-response | `crates/kutha-runtime/tests/m010_semantic_open.rs` (persist→drop lease→open) + unit oracles in `quantum.rs` (`budget_aborts_storm` / `cascade_idles_without_inverse_loop`) | exact |
| `.kutha/dictionaries/fsm.yaml` | config | batch | `.kutha/dictionaries/fsm.yaml` (`observe_cargo.required` Phase 4 names) | exact |
| `.kutha/dictionaries/checks.yaml` | config | batch | `.kutha/dictionaries/checks.yaml` (`m011-partial-correction`) | exact |
| `.kutha/dictionaries/bridges.yaml` | config | batch | `.kutha/dictionaries/bridges.yaml` (`B-m011-partial-correction`) | exact |
| `.kutha/dictionaries/honeycomb.yaml` | config | batch | `.kutha/dictionaries/honeycomb.yaml` (`ADR-013` evidence list; target cell `ADR-014`) | role-match |
| `CHANGELOG.md` | config | batch | `CHANGELOG.md` (2026-09-29 Product + Process + GATE needles) | exact |

## Pattern Assignments

### `crates/kutha-runtime/src/store.rs` (service, file-I/O)

**Analog:** `crates/kutha-runtime/src/store.rs` (self-extend)

**Imports / consts pattern** (lines 1–9):
```rust
use crate::quantum::{cascade_limit, Runtime, RuntimeError};
use crate::snapshot::Snapshot;
use crate::wal;
use kutha_common::{Event, Op};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const TERMS_REL: &str = "terms.jsonl";
```
Copy: add `const OUTCOMES_REL: &str = "quantum_outcomes.jsonl";` beside `TERMS_REL`.

**Core JSONL write pattern** (lines 13–26) — mirror for outcomes **after** events/WAL/terms; prefer outcomes **last** (before or after snapshot is planner choice; RESEARCH: after events so mid-crash ⇒ incomplete, not orphan Full):
```rust
pub fn persist(runtime: &Runtime, dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    let events = encoded_log(runtime);
    wal::append_events(&dir.join("events.wal"), &events)?;
    let mut events_file = File::create(dir.join("events.jsonl"))?;
    for e in &events {
        serde_json::to_writer(&mut events_file, e).map_err(json_err)?;
        events_file.write_all(b"\n")?;
    }
    write_terms(dir, runtime.dictionary().strings())?;
    let snap = runtime.snapshot();
    let mut sf = File::create(dir.join("snapshot.json"))?;
    serde_json::to_writer(&mut sf, &snap).map_err(json_err)?;
    Ok(())
}
```
Copy for outcomes (same hand-roll as events — do **not** add `serde-jsonlines`):
```rust
let mut of = File::create(dir.join(OUTCOMES_REL))?;
for row in runtime.outcome_records() {
    serde_json::to_writer(&mut of, row).map_err(json_err)?;
    of.write_all(b"\n")?;
}
```

**JSONL read pattern** (lines 36–45 events; 109–119 `read_terms`) — reuse for `load_outcomes`:
```rust
for line in BufReader::new(f).lines() {
    let line = line?;
    if line.trim().is_empty() {
        continue;
    }
    v.push(serde_json::from_str(&line).map_err(json_err)?);
}
```

**Open multi-path contract** (lines 50–72): every successful return path must attach loaded outcomes (snapshot early-return at 50–56 today skips other sidecars — **do not** leave outcomes only on the Define/terms branches). Missing outcomes file ⇒ empty list, never invent `Full`.

**Error handling** (lines 122–128): `json_err` → `InvalidData`; `runtime_err` for Runtime rebuild failures. Apply same to corrupt outcomes lines.

**Tracked:** `git ls-files` confirms `crates/kutha-runtime/src/store.rs`.

---

### `crates/kutha-runtime/src/quantum.rs` (service, event-driven + transform)

**Analog:** `crates/kutha-runtime/src/quantum.rs`

**Imports pattern** (lines 1–9):
```rust
use crate::allow::load_allowed_names;
use crate::csr::CsrLease;
use crate::fold::GraphFold;
use crate::log::EventLog;
use crate::receipt::QuantumReceipt;
use crate::snapshot::Snapshot;
use kutha_common::{Event, EventId, Op, TermDictionary, TermId};
use std::collections::HashSet;
use std::fmt;
```
Copy: import new disposition / persisted-row types (from same module or `receipt.rs`).

**In-memory outcome shape** (lines 74–78) — disposition derives from these fields (D-O2); do not treat `Ok` alone as completion:
```rust
pub struct QuantumOutcome {
    pub receipt: QuantumReceipt,
    pub events_in_quantum: usize,
}
```

**Runtime struct buffer** (lines 81–90 / `new` 99–112): add an outcomes `Vec` field initialized empty in `new`, `from_snapshot` (166–189), and `from_dict_and_events`. Snapshot must **not** own this buffer (D-O1 / ADR-010).

**Core emit budget loop** (lines 336–355) — **do not change cascade math** (D-O7); after building `QuantumOutcome`, push a persisted row into the buffer:
```rust
while let Some(event) = pending.pop() {
    if used >= self.max_cascade {
        aborted = true;
        break;
    }
    used += 1;
    ids.push(event.id);
    digests.push(event.digest_bytes());
    let follow = self.follow_ons(&event);
    self.fold.apply(&event);
    self.log.append(event);
    for e in follow {
        pending.push(e);
    }
}

Ok(QuantumOutcome {
    receipt: QuantumReceipt::from_events(ids, &digests, aborted),
    events_in_quantum: used,
})
```

**Disposition mapping** (RESEARCH Pattern 1; fields from lines 74–78 / 352–354):
```rust
fn disposition(aborted: bool, events_in_quantum: usize) -> OutcomeDisposition {
    match (aborted, events_in_quantum) {
        (true, 0) => OutcomeDisposition::Zero,
        (true, _) => OutcomeDisposition::Partial,
        (false, _) => OutcomeDisposition::Full,
    }
}
```

**Budget fixture oracles** (lines 501–540) — integration tests reuse the same Assert→`knows` storm; Zero uses `Runtime::new(0)` (env `KUTHA_MAX_CASCADE=0` is filtered — lines 65–71):
```rust
fn cascade_idles_without_inverse_loop() { /* Runtime::default(); events_in_quantum==2; !aborted */ }
fn budget_aborts_storm() { /* Runtime::new(1); aborted; events_in_quantum==1 */ }
```

**Fail-closed API analog for `record_resume`** (lines 276–286 `admit_claim`): unknown / duplicate identity returns `RuntimeError`, does not invent success:
```rust
fn admit_claim(&self, op: &Op) -> Result<(), RuntimeError> {
    // ...
    Err(RuntimeError::UnknownClaim { claim: *claim })
}
```
Optional thin anti-duplicate resume (D-O4): same style — `Err` if `resume_of` already present.

**persist_open round-trip unit style** (lines 620–642): temp dir + `store::persist` / `open` + asserts — prefer integration test file for OUT-01/02, but same helper shape (`uuid_like` temp dirs).

---

### `crates/kutha-runtime/src/receipt.rs` (model, transform)

**Analog:** `crates/kutha-runtime/src/receipt.rs` + serde pattern from `crates/kutha-runtime/src/snapshot.rs`

**Core receipt** (lines 4–24) — keep in-memory `[u8; 32]`; do **not** force `Serialize` on `QuantumReceipt` without a hex/bytes strategy (RESEARCH Pitfall 4):
```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuantumReceipt {
    pub event_ids: Vec<EventId>,
    pub digest: [u8; 32],
    pub aborted_on_budget: bool,
}

impl QuantumReceipt {
    pub fn from_events(ids: Vec<EventId>, event_digests: &[[u8; 32]], aborted: bool) -> Self {
        // Sha256 over digests + aborted byte
    }
}
```

**Serde row analog** (`snapshot.rs` lines 4–14):
```rust
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Snapshot { /* ... */ }
```
Copy for `PersistedQuantumOutcome` / `OutcomeDisposition` (RESEARCH recommended shape: `quantum_id`, `disposition`, `aborted_on_budget`, `events_in_quantum`, `event_ids`, `receipt_digest_hex`, `resume_of`). Persist digest as lowercase hex `String` (64 chars), not raw `[u8; 32]` in JSONL.

**Planner note:** helpers may live in `receipt.rs` or `quantum.rs`; either is fine if `lib.rs` re-exports public types.

---

### `crates/kutha-runtime/src/lib.rs` (config / barrel)

**Analog:** `crates/kutha-runtime/src/lib.rs` (lines 16–24)
```rust
pub use quantum::{QuantumOutcome, Runtime, RuntimeError};
pub use receipt::QuantumReceipt;
pub use snapshot::Snapshot;
```
Copy: `pub use` new disposition / persisted-row / `record_resume` surface if public for tests.

---

### `crates/kutha-runtime/tests/m011_quantum_outcome.rs` (test, file-I/O + request-response)

**Primary analog:** `crates/kutha-runtime/tests/m010_semantic_open.rs`  
**Secondary analog:** `crates/kutha-runtime/src/quantum.rs` budget unit tests (501–540)  
**GATE naming analog:** `crates/kutha-runtime/tests/m011_partial_correction.rs` (module doc + exact `fn` names)

**Imports / module doc** (`m010_semantic_open.rs` lines 1–4):
```rust
//! M010: open recovers intern meanings; live intern logs Op::Define (ADR-011).

use kutha_common::Op;
use kutha_runtime::{store, Runtime};
```
Copy for S05: `//! M011 S05: persisted quantum outcomes …`; import disposition / resume API as exported.

**Persist → discard lease → open** (`m010_semantic_open.rs` lines 25–49) — OUT-01 must also discard `snapshot.json` and still see dispositions (outcomes are **not** a lease):
```rust
store::persist(&rt, &dir).unwrap();
std::fs::remove_file(dir.join("snapshot.json")).unwrap();
let opened = store::open(&dir).unwrap();
```

**Temp dir helper** (lines 81–89):
```rust
fn uuid_like() -> String {
    format!(
        "{:x}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    )
}
```

**Named oracle fn names** (must match GATE needles byte-for-byte — RESEARCH Validation):
- `budgets_0_1_2_distinguish_zero_partial_full_after_persist_open`
- `crash_after_prefix_has_no_terminal_success_until_explicit_resume`

**OUT-01 body sketch:** three runtimes `Runtime::new(0)`, `Runtime::new(1)`, `Runtime::default()` (or `new(2+)`) on Assert `knows`; persist; open (optionally without snapshot); assert Zero / Partial / Full from persisted records — assert on disposition / `aborted_on_budget`×count, **not** only `is_ok()`.

**OUT-02 body sketch:** `Runtime::new(1)` Partial prefix; persist events without terminal Full (omit/strip outcomes file or skip Full row); open → no Full; `record_resume(quantum_id)`; persist; reopen → Resume with `resume_of`; open never auto-resumes.

**Tracked:** new file will be untracked until first commit; analogs `m010_semantic_open.rs` and `m011_partial_correction.rs` are git-tracked.

---

### `.kutha/dictionaries/fsm.yaml` (config, batch)

**Analog:** same file, `observe_cargo.required` (lines 31–45)

**Core pattern** — append two names (do not remove Phase 4 entries):
```yaml
required:
  # ... existing ...
  - interval_patch_leaves_vt_2012_and_2021_residuals
  - whole_version_correct_does_not_invent_residuals
  # Phase 5 append:
  - budgets_0_1_2_distinguish_zero_partial_full_after_persist_open
  - crash_after_prefix_has_no_terminal_success_until_explicit_resume
```

---

### `.kutha/dictionaries/checks.yaml` (config, batch)

**Analog:** `m011-partial-correction` (lines 291–313)

**Core GATE-01 trio step** — new id `m011-quantum-outcome`, category `m011-s05` (do **not** pile onto `m011-partial-correction`):
```yaml
  - id: m011-quantum-outcome
    description: M011 S05 — persisted Zero/Partial/Full; crash prefix needs explicit resume
    steps:
      - kind: file_contains
        path: crates/kutha-runtime/tests/m011_quantum_outcome.rs
        needles:
          - "fn budgets_0_1_2_distinguish_zero_partial_full_after_persist_open"
          - "fn crash_after_prefix_has_no_terminal_success_until_explicit_resume"
        require: all
        category: m011-s05
        message: "M011 quantum-outcome tests missing {missing}"
      # optional product needles, e.g. OUTCOMES_REL / OutcomeDisposition in store.rs or quantum.rs
```

---

### `.kutha/dictionaries/bridges.yaml` (config, batch)

**Analog:** `B-m011-partial-correction` (lines 44–47)

```yaml
  - id: B-m011-quantum-outcome
    claim: "Budgets 0/1/2 distinguish Zero/Partial/Full after persist→open; crash after prefix has no terminal success until explicit resume"
    cites: "crates/kutha-runtime/tests/m011_quantum_outcome.rs"
    check: m011-quantum-outcome
```

Schema header (lines 1–7) stays untouched; append under `bridges:`.

---

### `.kutha/dictionaries/honeycomb.yaml` (config, batch)

**Analog:** `ADR-013` evidence list (lines 108–120); **target cell** `ADR-014` (lines 122–131, currently `evidence: []`)

**Core pattern** — append named test fns; keep `map: Proposed`:
```yaml
  - id: ADR-014
    # ...
    map: Proposed
    evidence:
      - budgets_0_1_2_distinguish_zero_partial_full_after_persist_open
      - crash_after_prefix_has_no_terminal_success_until_explicit_resume
```
Do not promote L_map to Accepted. Timing: GATE-01 wave (RESEARCH Open Question 3).

---

### `CHANGELOG.md` (config, batch)

**Analog:** top entry `## 2026-09-29 — Product: interval-patch CorrectInterval residuals` (lines 7–19)

**Core pattern:**
- **Product** bullets for sidecar + disposition + named OUT-01/02 tests
- **Process** bullets for FSM + `m011-quantum-outcome` / `B-m011-quantum-outcome` GATE-01
- Keep Product vs Process distinct; optional Trajectory subsection per D-O5 / D-10
- Do **not** edit `.kutha/STATE.md` or check ROADMAP S05 during delivery (GATE-02)

## Shared Patterns

### JSONL sidecar (authoritative, not a lease)
**Source:** `crates/kutha-runtime/src/store.rs` lines 9–26, 100–119  
**Apply to:** `persist` / `open` outcomes path  
`File::create` rewrite + `serde_json::to_writer` + `\n`; line-loop deserialize; missing file ⇒ empty. Distinct from droppable `snapshot.json`.

### Emit → outcome buffer → persist
**Source:** `quantum.rs` emit (336–355) + `store::persist`  
**Apply to:** every successful emit that must survive disk  
Cascade math unchanged; recording only. Prefer writing outcomes after event bytes.

### Disposition from abort × count
**Source:** `QuantumOutcome` + `QuantumReceipt.aborted_on_budget`  
**Apply to:** OUT-01 encoding and persisted rows  
`(true,0)=Zero`, `(true,>0)=Partial`, `(false,_)=Full`. `Ok(_)` ≠ completion (ADR-014).

### Budget storm fixtures
**Source:** `quantum.rs` `budget_aborts_storm` / `cascade_idles_without_inverse_loop`  
**Apply to:** `m011_quantum_outcome.rs`  
Inverse-`knows` Assert; Zero must use `Runtime::new(0)`, not env.

### Fail-closed identity / no invented success
**Source:** `admit_claim` / `UnknownClaim` (276–286); open missing-snapshot fail path (69–72)  
**Apply to:** open (never invent Full); optional duplicate `record_resume`  
Incomplete = absence of Full, not auto-Resume (D-O3).

### GATE-01 dictionary trio
**Source:** Phase 4 `fsm.yaml` + `checks.yaml` `m011-partial-correction` + `bridges.yaml` `B-m011-partial-correction`  
**Apply to:** S05 registration  
Same test fn string in observe list, `file_contains` needles, and bridge `cites`/`check`. New check id — do not extend S04 check.

### docs-coupling changelog
**Source:** `CHANGELOG.md` 2026-09-29 entry  
**Apply to:** crate + dictionary waves  
Product with crate diffs; Process with dictionary/GATE; bridge row paired with new check.

## No Analog Found

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| — | — | — | All planned files have at least a role-match analog. Closest gap: **persisted resume row API** has no prior `record_resume` — use `admit_claim` fail-closed + new JSONL row shape from RESEARCH. **`[u8; 32]` hex encoding** has no in-tree helper — hand-roll lowercase hex on the persisted row only. |

## Metadata

**Analog search scope:** `crates/kutha-runtime/src/{store,quantum,receipt,snapshot,lib}.rs`, `crates/kutha-runtime/tests/{m010_semantic_open,m011_partial_correction}.rs`, `.kutha/dictionaries/{fsm,checks,bridges,honeycomb}.yaml`, `CHANGELOG.md`; CBM `trace_path` on `persist`/`emit`, `get_code_snippet` on store/receipt  
**Files scanned:** ~15 tracked sources  
**Tracked-source gate:** all named analogs verified via `git ls-files` (non-empty)  
**Pattern extraction date:** 2026-09-29  
**CBM coverage caveat:** store/quantum/tests/dicts `metadata_changed` or `not_tracked`; planner should treat this PATTERNS.md + source paths as authoritative until integrator reindexes
