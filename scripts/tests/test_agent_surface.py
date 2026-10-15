"""M074 S01 — agent-facing docs name live CBM project and kutha-gov flags."""

from __future__ import annotations

import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]


class AgentSurfaceDiscoverabilityTests(unittest.TestCase):
    def test_gsd_auto_contract_cbm_column_prefers_kutha_graph(self) -> None:
        text = (ROOT / "docs/process/gsd-auto-contract.md").read_text(encoding="utf-8")
        self.assertIn("CBM (`kutha-graph`)", text)
        # Historical alias may appear as a footnote, not as the table column primary.
        self.assertNotIn("CBM (`root-kutha-graph`)", text)

    def test_agents_md_lists_live_harness_flags(self) -> None:
        text = (ROOT / "AGENTS.md").read_text(encoding="utf-8")
        for needle in (
            "--jobs",
            "--skip-cargo",
            "kutha-gov json",
            "--check",
            "ID[,ID",
            "repeatable",
            "run_high=",
        ):
            with self.subTest(needle=needle):
                self.assertIn(needle, text)

    def test_agents_md_prefers_built_migrate_bin(self) -> None:
        text = (ROOT / "AGENTS.md").read_text(encoding="utf-8")
        self.assertIn("debug/kutha-migrate", text)
        self.assertNotIn(
            "cargo run -p kutha-runtime --bin kutha-migrate",
            text,
        )

    def test_process_notes_prefer_built_migrate_bin(self) -> None:
        for rel in (
            "docs/process/op-compat.md",
            "docs/process/pin-compat.md",
        ):
            text = (ROOT / rel).read_text(encoding="utf-8")
            with self.subTest(rel=rel):
                self.assertIn("debug/kutha-migrate", text)

    def test_harness_cutport_names_built_migrate(self) -> None:
        text = (ROOT / "docs/process/kutha-harness.md").read_text(encoding="utf-8")
        self.assertIn("| migrate-bin |", text)
        self.assertIn("debug/kutha-migrate", text)

    def test_harness_documents_check_surfaces(self) -> None:
        """M087: living harness CLI note must match json|precommit|selftest --check."""
        text = (ROOT / "docs/process/kutha-harness.md").read_text(encoding="utf-8")
        self.assertIn("`json`, `precommit`, or `selftest`", text)
        self.assertIn("uv run kutha-gov selftest --check", text)
        self.assertNotIn(
            "`--check ID` is valid only with `json` or `precommit`",
            text,
        )

    def test_agents_md_check_not_for_ci(self) -> None:
        """M088: AGENTS Flags must not imply --check works with ci."""
        text = (ROOT / "AGENTS.md").read_text(encoding="utf-8")
        self.assertIn("json|precommit|selftest only", text)
        self.assertIn("not `ci`", text)

    def test_env_example_documents_built_migrate(self) -> None:
        text = (ROOT / ".env.example").read_text(encoding="utf-8")
        self.assertIn("debug/kutha-migrate", text)
        self.assertIn("KUTHA_MIGRATE_DIR", text)


if __name__ == "__main__":
    unittest.main()
