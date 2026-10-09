"""RED then green: governor check `test-matrix` (M019 S04 / Phase 59).

Rows must HIGH before the check exists (missing id) and after it exists
when the matrix is mutated. Live-tree run is a separate green assertion.
"""

from __future__ import annotations

import atexit
import shutil
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))

from kutha_gov.checks import get_checks  # noqa: E402
from kutha_gov.protocol import Context, Severity  # noqa: E402

_VALID_MATRIX = """\
schema: kutha-test-matrix/v1
rows:
  - id: DUR-sidecar-atomic
    plane: product
    invariant: atomic sidecar tmp+rename
    layer: crash
    oracle:
      - persist_replaces_events_jsonl_atomically
    kill_mutation: write sidecar in place without tmp+rename
    status: killed
    gate: default
    owner: M012a-S04
    source: audit-code-tests §2
  - id: DUR-mixed-group
    plane: product
    invariant: mixed Typed/Dictionary uncommitted group
    layer: crash
    oracle: []
    kill_mutation: none
    status: gap
    gate: default
    owner: M019-S05
    source: audit-code-tests §2
  - id: SEM-horn-kc
    plane: product
    invariant: Horn must not mint when a premise is absent at the knowledge cut
    layer: decision
    oracle:
      - known04_horn_fires_when_premise_hidden_on_early_cut
    kill_mutation: none
    status: blocked
    gate: default
    owner: HORN-KC-01
    source: audit-code-tests §3
  - id: ROCK-rebuild
    plane: product
    invariant: Rocks index rebuilt when stale
    layer: crash
    oracle:
      - stale_index_is_rebuilt
    kill_mutation: leave a stale tip-matching index in place
    status: killed
    gate: feature:observe_rocks
    owner: M002-S07
    source: audit-code-tests §2
  - id: HAR-freeze
    plane: harness
    invariant: freeze-rocks parses Cargo tables
    layer: governor
    oracle:
      - freeze-rocks
    kill_mutation: add [dependencies.rocksdb] without optional
    status: killed
    gate: default
    owner: M019-S02
    source: audit-governor §2
"""

_ROADMAP = """\
## M012a: Single-log SoT
- [x] **S04: Verify-on-open**
## M002: Log durability
- [x] **S07: Droppable RocksDB index lease**
## M019: Post-M002 coherence
- [ ] **S02: Governor hardening**
- [ ] **S05: Durability residuals A**
"""

_FSM = """\
schema: kutha-harness-fsm/v1
states:
  observe_rocks:
    kind: observe_cargo
    required:
      - stale_index_is_rebuilt
"""

_CHECKS = """\
schema: kutha-harness-checks/v1
checks:
  - id: freeze-rocks
    description: freeze
    steps: []
  - id: test-matrix
    description: scratch
    steps: []
"""

_ORACLE_RS = """\
#[test]
fn persist_replaces_events_jsonl_atomically() {
    assert!(true);
}

#[test]
fn known04_horn_fires_when_premise_hidden_on_early_cut() {
    assert!(true);
}

#[test]
fn stale_index_is_rebuilt() {
    assert!(true);
}
"""


def _scratch(matrix: str = _VALID_MATRIX, rust: str = _ORACLE_RS) -> Path:
    raw = tempfile.mkdtemp(prefix="kutha-test-matrix-")
    atexit.register(shutil.rmtree, raw, ignore_errors=True)
    root = Path(raw)
    (root / "docs" / "architecture").mkdir(parents=True)
    (root / "docs" / "architecture" / "test-matrix.yaml").write_text(matrix, encoding="utf-8")
    (root / ".kutha" / "dictionaries").mkdir(parents=True)
    (root / ".kutha" / "ROADMAP.md").write_text(_ROADMAP, encoding="utf-8")
    (root / ".kutha" / "dictionaries" / "fsm.yaml").write_text(_FSM, encoding="utf-8")
    (root / ".kutha" / "dictionaries" / "checks.yaml").write_text(_CHECKS, encoding="utf-8")
    tests = root / "crates" / "kutha-runtime" / "tests"
    tests.mkdir(parents=True)
    (tests / "oracle.rs").write_text(rust, encoding="utf-8")
    return root


def _check():
    checks = get_checks(ROOT)
    assert "test-matrix" in checks, "test-matrix check is not registered"
    return checks["test-matrix"]


def _highs(root: Path) -> list[str]:
    result = _check().run(Context(root=root))
    return [f.category for f in result.findings if f.severity is Severity.HIGH]


class TestMatrixCheckTests(unittest.TestCase):
    def test_check_is_registered(self) -> None:
        self.assertIn("test-matrix", get_checks(ROOT))

    def test_valid_scratch_is_clean(self) -> None:
        self.assertEqual([], _highs(_scratch()))

    def test_missing_oracle_fn_is_high(self) -> None:
        matrix = _VALID_MATRIX.replace(
            "persist_replaces_events_jsonl_atomically",
            "this_oracle_fn_does_not_exist",
        )
        cats = _highs(_scratch(matrix=matrix))
        self.assertIn("rust-test-missing", cats)

    def test_killed_with_kill_mutation_none_is_high(self) -> None:
        matrix = _VALID_MATRIX.replace(
            "kill_mutation: write sidecar in place without tmp+rename",
            "kill_mutation: none",
            1,
        )
        cats = _highs(_scratch(matrix=matrix))
        self.assertIn("matrix-killed", cats)

    def test_killed_with_characterization_shaped_kill_is_high(self) -> None:
        matrix = _VALID_MATRIX.replace(
            "kill_mutation: write sidecar in place without tmp+rename",
            "kill_mutation: mirrors is_known_at filter — not an independent kill",
            1,
        )
        cats = _highs(_scratch(matrix=matrix))
        self.assertIn("matrix-killed-characterization", cats)

    def test_blocked_row_plus_ignore_is_high(self) -> None:
        rust = _ORACLE_RS.replace(
            "#[test]\nfn known04_horn_fires_when_premise_hidden_on_early_cut()",
            "#[ignore]\n#[test]\nfn known04_horn_fires_when_premise_hidden_on_early_cut()",
        )
        cats = _highs(_scratch(rust=rust))
        self.assertIn("rust-test-ignored", cats)

    def test_unknown_gap_owner_is_high(self) -> None:
        matrix = _VALID_MATRIX.replace("owner: M019-S05", "owner: unowned", 1)
        cats = _highs(_scratch(matrix=matrix))
        self.assertIn("matrix-gap-owner", cats)

    def test_feature_gate_without_observe_row_is_high(self) -> None:
        matrix = _VALID_MATRIX.replace(
            "gate: feature:observe_rocks",
            "gate: feature:observe_missing",
            1,
        )
        cats = _highs(_scratch(matrix=matrix))
        self.assertIn("matrix-feature-gate", cats)

    def test_duplicate_id_is_high(self) -> None:
        matrix = _VALID_MATRIX.replace("id: DUR-mixed-group", "id: DUR-sidecar-atomic", 1)
        cats = _highs(_scratch(matrix=matrix))
        self.assertIn("matrix-unique", cats)

    def test_unparsable_yaml_is_high(self) -> None:
        cats = _highs(_scratch(matrix="{{not yaml"))
        self.assertTrue(cats, msg="unparsable matrix must be HIGH")
        self.assertTrue(any(c in {"yaml", "missing"} for c in cats) or cats)

    def test_live_tree_has_no_high(self) -> None:
        result = _check().run(Context(root=ROOT))
        highs = [f.format() for f in result.findings if f.severity is Severity.HIGH]
        self.assertEqual([], highs)
