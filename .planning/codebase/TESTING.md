---
last_mapped_commit: e77132d9275bd36ea766b8bef9cff28128dfc636
last_mapped_at: 2026-09-29
---
# Testing Patterns

**Analysis Date:** 2026-09-29

## Test Framework

**Runner (Rust product):**
- Built-in `cargo test` (Rust edition 2021 workspace)
- Config: per-crate `Cargo.toml` under `crates/kutha-common`, `crates/kutha-runtime` — no custom harness crate
- Assertions: `assert!`, `assert_eq!`, `matches!` macros

**Runner (Python harness):**
- **pytest** ≥8 (`pyproject.toml` `[dependency-groups].dev` and `[tool.pytest.ini_options]`)
- Suites themselves use **`unittest.TestCase`** classes discovered by pytest
- Config: `testpaths = ["scripts/tests"]`, `pythonpath = ["scripts"]`, `python_files = ["test_*.py"]`
- Assertions: `self.assertEqual`, `self.assertTrue`, `self.assertIn`, …

**Assertion Library:**
- Rust: std test macros
- Python: `unittest` methods (not `pytest.raises` as the dominant style)

**Run Commands:**

```bash
cargo test --workspace                    # All product tests (unit + integration)
cargo test -p kutha-runtime               # Runtime crate only
cargo test --workspace --offline          # Same args the FSM observe step uses

uv run pytest                             # All harness tests under scripts/tests/
uv run pytest scripts/tests/test_fsm.py   # One file
uv run kutha-gov py                       # ruff + ty + pyrefly (not unit tests)
uv run kutha-gov precommit                # Dictionary checks only (commit profile)
uv run kutha-gov ci                       # Full FSM quantum: checks → cargo observe → emit → fold
```

## Test File Organization

**Location:**
- Rust **unit**: co-located `#[cfg(test)] mod tests { … }` at bottom of source modules
- Rust **integration / fitness**: `crates/kutha-runtime/tests/*.rs` (separate crate binary tests)
- Rust **fixtures**: `crates/kutha-runtime/tests/fixtures/` (`legal_pit.md`, `harness_process.jsonl`)
- Python: separate tree `scripts/tests/` (not under `kutha_gov/`)

**Naming:**
- Rust unit/integration fn: `snake_case` describing behavior — `unknown_claim_does_not_append`, `ff5_as_of_t1_differs_from_as_of_t2_on_statute_log`
- Prefix integration files by thread: `ff5_`, `ff6_`, `h2_`, `h4_`, `m010_`, `m011_`
- Python: class `*Tests(unittest.TestCase)`, methods `test_*`

**Structure:**

```text
crates/kutha-common/src/*.rs          # occasional #[cfg(test)] (e.g. intern.rs)
crates/kutha-runtime/src/*.rs         # unit mods: allow, leapfrog, quantum, wal, materializer, …
crates/kutha-runtime/tests/           # fitness / cross-module scenarios
crates/kutha-runtime/tests/fixtures/  # shared fixture files
scripts/tests/test_kutha_gov.py       # dictionary, checks, fold, CLI contracts
scripts/tests/test_fsm.py             # FSM machine, observe, tenant observation
.kutha/dictionaries/fsm.yaml          # observe_cargo.required = named Rust tests
.kutha/dictionaries/bridges.yaml      # bridges cite product tests / freezes
.kutha/dictionaries/checks.yaml       # static governor checks (not cargo)
```

## Test Structure

**Suite Organization (Rust integration):**

```rust
//! Milestone/feature note — what invariant is under test.

use kutha_common::Op;
use kutha_runtime::Runtime;

#[test]
fn unknown_claim_does_not_append() {
    let mut rt = Runtime::default();
    // arrange: intern terms, capture log length
    let err = rt.emit(Op::Assert { /* … claim: Some(ghost) */ }).unwrap_err();
    assert!(matches!(err, kutha_runtime::RuntimeError::UnknownClaim { .. }));
    assert_eq!(n, rt.log().len(), "fail-closed: unknown claim must not append");
}
```

**Suite Organization (Python):**

```python
from __future__ import annotations

import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))

from kutha_gov.protocol import Context, Severity  # noqa: E402

class HarnessTests(unittest.TestCase):
    def setUp(self) -> None:
        self.ctx = Context(root=ROOT)

    def test_unknown_kind_fails_closed(self) -> None:
        …
```

**Patterns:**
- Prefer **behavior names** over implementation names in test fn titles
- Fail-closed cases: assert **both** the error variant **and** that SoT did not grow (`log().len()`, empty fold)
- Temporal fitness: use opaque integer valid-times (`2010`, `2015`, `2021`), not wall-clock “now”
- End many scenarios with `rt.replay_check().unwrap()` when mutation must stay replay-safe
- Python live-tree smoke: `test_ci_green_on_this_tree` / `test_no_high_on_live_tree` call real dictionaries on repo root

## Mocking

**Framework:** `unittest.mock` (`MagicMock`, `patch`) — only in harness tests that shell out

**Patterns:**

```python
from unittest.mock import MagicMock, patch

calls: list[list[str]] = []

def fake_run(cmd: list[str], **_kwargs: object) -> MagicMock:
    calls.append(list(cmd))
    completed = MagicMock()
    completed.returncode = 0
    completed.stdout = "…"
    completed.stderr = ""
    return completed

with patch("kutha_gov.observe.subprocess.run", side_effect=fake_run):
    code, seen, findings = run_cargo_observation(ROOT, spec)
```

See `ObserveBuildTests` / `TenantTests` in `scripts/tests/test_fsm.py`.

**What to Mock:**
- `subprocess.run` for cargo observe and tenant binary selection
- Env vars with save/restore in `try`/`finally` (`KUTHA_TENANT_BIN`, `KUTHA_GOV_BUDGET`, `CARGO_TARGET_DIR`)

**What NOT to Mock:**
- Dictionary YAML loading for core contract tests — read real `.kutha/dictionaries/`
- Product `Runtime` / fold / allowlist — use real in-memory runtime
- Do not mock away fail-closed kind allowlists

## Fixtures and Factories

**Test Data (Rust):**

```rust
let mut rt = Runtime::default();
let p = rt.intern("P");
let rel = rt.intern("relatedTo");
let true_ = rt.intern("true");
rt.emit(Op::Assert { subject: p, relation: rel, object: true_, … }).unwrap();
```

- File fixtures: `crates/kutha-runtime/tests/fixtures/legal_pit.md`, `harness_process.jsonl`
- Synthetic logs: `Runtime::from_dict_and_events(dict, events, cascade)` for replay/lineage negatives

**Test Data (Python):**

```python
with tempfile.TemporaryDirectory() as raw:
    root = Path(raw)
    dict_dir = root / ".kutha" / "dictionaries"
    dict_dir.mkdir(parents=True, exist_ok=True)
    (dict_dir / "relations.yaml").write_text("schema: …\n", encoding="utf-8")
```

**Location:**
- Prefer temp trees for append/log side effects (`append_run`, membership editions)
- Prefer live `ROOT` for discovery / ci-green / map / green-tree invariants

## Coverage

**Requirements:** None enforced via `cargo-tarpaulin` / `pytest-cov` in-repo

**Evidence plane (harness):**
- FSM `observe_cargo.required` lists **named** Rust test functions that must appear as `test … … ok` in cargo output (`scripts/kutha_gov/observe.py` → `interpret_cargo_output`)
- Missing or FAILED required name → HIGH finding (`observe-missing` / `observe-fail`) — evidence, not product SoT
- Current required names live in `.kutha/dictionaries/fsm.yaml` under `states.observe_cargo.required` (FF5/FF6/H2/H4/M010/M011 fitness tests)
- Bridge ledger `.kutha/dictionaries/bridges.yaml` + checks such as `observe-required-fn` / `m011-claim-supports` keep names aligned with source

**View Coverage:**

```bash

# No dedicated coverage report command in-repo.

# Fitness gate visibility:

uv run kutha-gov ci          # runs cargo test --workspace --offline inside observe
rg 'required:' -A30 .kutha/dictionaries/fsm.yaml
```

## Test Types

**Unit Tests:**
- Rust: pure helpers and module contracts — `leapfrog_intersect`, `parse_relations`, WAL/materializer slices, many `quantum.rs` cases
- Python: kind runners, fold determinism, YAML map-list / when_match_then_match behaviors in `test_kutha_gov.py`

**Integration Tests:**
- Rust `crates/kutha-runtime/tests/*`: multi-op timelines, allowlist fail-closed, tenant JSONL ingest, process allows, semantic open, claim supports
- Python FSM quantum path: `test_ci_quantum_reaches_ok_on_this_tree` expects full trace including `observe_cargo` and named evidence tuples

**E2E Tests:**
- `uv run kutha-gov ci` / `HarnessTests.test_ci_green_on_this_tree` — full dictionary + observe + emit/fold on the live tree
- Not a browser/UI E2E stack — Not applicable for frontend

**Governor checks (static tests-as-policy):**
- YAML steps in `.kutha/dictionaries/checks.yaml` — file presence, freeze needles, lifecycle collapse, ADR status tags, ledger consistency
- Run without cargo: `uv run kutha-gov precommit` or `uv run kutha-gov precommit --check <id>`
- Adding a product fitness test often requires: Rust `#[test]`, optional bridge row, entry in `observe_cargo.required`

## Common Patterns

**Async Testing:**
- Not applicable — no async runtime in product or harness tests

**Error Testing (Rust):**

```rust
let err = rt.emit(…).unwrap_err();
assert!(
    matches!(err, kutha_runtime::RuntimeError::UnknownRelation { ref name } if name == "notALegalRelation"),
    "{err:?}"
);
assert_eq!(n, rt.log().len(), "fail-closed: log must not grow");
```

**Error Testing (Python):**

```python
result = CheckResult(check="probe")
run_step("probe", {"kind": "not_a_kind"}, self.ctx, result)
highs = [f for f in result.findings if f.severity is Severity.HIGH]
self.assertEqual(1, len(highs))
self.assertEqual("unknown-kind", highs[0].category)
```

**Determinism:**
- Harness fold: apply the same event list to two `HarnessFold` instances and compare state (`test_harness_fold_is_deterministic`)
- Product: `replay_check` after mutations

**Env isolation:**
- Always restore mutated environment variables in `finally` blocks (`test_fsm.py` budget / tenant bin tests)

## Tooling Gate Matrix

| Command | Cargo | Dict checks | JSONL emit | Ruff/ty/pyrefly |
|---------|-------|-------------|------------|-----------------|
| `cargo test --workspace` | yes | no | no | no |
| `uv run pytest` | no* | via tests | temp only | no |
| `uv run kutha-gov precommit` | no | yes | no | no |
| `uv run kutha-gov ci` | observe | yes | yes | no |
| `uv run kutha-gov py` | no | no | no | yes |
| pre-commit hooks | no | yes | no | ruff on staged `scripts/` |

\*Some pytest cases invoke cargo indirectly only if they call `run_quantum` / `main(["ci"])` on the live tree.

## Where to Add New Tests

**New product behavior:**
1. Prefer unit `#[cfg(test)]` next to the module for pure logic
2. Add `crates/kutha-runtime/tests/<thread>_*.rs` for temporal / allowlist / claim fitness
3. If the test is a delivery gate, add the exact `fn` name to `.kutha/dictionaries/fsm.yaml` `observe_cargo.required` and update bridge/check dictionaries per `docs/process/governor-intake.md`

**New harness behavior:**
1. Add `test_*` methods to `scripts/tests/test_kutha_gov.py` or `test_fsm.py` (or a new `test_*.py` under `scripts/tests/`)
2. Prefer tempfile roots for mutating logs; live `ROOT` for discovery/green contracts
3. Do **not** add Python check classes under `scripts/kutha_gov/checks/` — encode policy in YAML

---

*Testing analysis: 2026-09-29*
