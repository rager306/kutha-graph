"""CE whole-repo 2026-10-15: every mapped finding keeps a living oracle."""

from __future__ import annotations

import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MAP = ROOT / "docs/process/ce-review-regression-map.md"
STORE = ROOT / "crates/kutha-runtime/src/store.rs"

# Finding label -> oracle symbols that must exist as a Rust fn or a checks.yaml id.
COVERAGE: dict[str, tuple[str, ...]] = {
    "#1": ("migrate_pin_cite_reopens_after_nonempty_justifications_sidecar",),
    "#13": (
        "migrate_op_wire_reports_dropped_jsonl_tail",
        "migrate_pin_wire_reports_dropped_jsonl_tail",
        "migrate_pin_cite_reports_dropped_jsonl_tail",
    ),
    "#9": (
        "kutha_migrate_help_exits_zero_on_stdout",
        "kutha_migrate_rejects_extra_positionals",
    ),
    "#6-8": ("dict-load-fail-closed",),
    "#3": (
        "retract_second_phase_examines_constant_in_unrelated_live_d",
        "retract_non_trigger_still_drops_after_premise_rel_index",
    ),
    "#2": ("bench_rss_before_is_taken_immediately_before_sample",),
    "#4": (
        "late_starting_premise_fires_when_trigger_starts_earlier",
        "horn_overlap_discovery_is_order_independent",
    ),
    "#5": ("kinds-family-split",),
    "Correct/CorrectInterval": (
        "correct_of_behavior_drops_retract_index",
        "correct_interval_of_behavior_drops_retract_index",
    ),
    "rule-pin": ("late_starting_rule_pin_fires_on_overlap",),
    "seg-files": ("migrate_verify_fault_leaves_no_segment_files",),
    "keep-first": ("from_strings_keeps_first_id_for_duplicate_name",),
    "unindex_behavior": ("unindex_cause_examines_constant_in_live_d",),
}


def _haystack() -> str:
    parts = [(ROOT / ".kutha/dictionaries/checks.yaml").read_text(encoding="utf-8")]
    for path in (ROOT / "crates").rglob("*.rs"):
        if "target" in path.parts:
            continue
        parts.append(path.read_text(encoding="utf-8"))
    for path in (ROOT / "scripts/tests").glob("test_*.py"):
        parts.append(path.read_text(encoding="utf-8"))
    return "\n".join(parts)


class Ce20261015RegressionMapTests(unittest.TestCase):
    def test_mapped_findings_name_living_oracles(self) -> None:
        text = MAP.read_text(encoding="utf-8")
        section = text.split("## 2026-10-15 HEAD review", 1)[1].split("## Rule", 1)[0]
        hay = _haystack()
        for label, oracles in COVERAGE.items():
            label_in_map = "#6\u2013#8" if label == "#6-8" else label
            self.assertIn(label_in_map, section)
            for name in oracles:
                self.assertIn(name, section, msg=f"{label} oracle {name} missing from map")
                self.assertIn(name, hay, msg=f"{label} oracle {name} missing from code")

    def test_migrate_compat_calls_op_wire_before_pins(self) -> None:
        """Testing gap: swapped op-wire/pins order must not stay green on markers alone."""
        src = STORE.read_text(encoding="utf-8")
        start = src.find("pub fn migrate_compat")
        self.assertGreaterEqual(start, 0)
        body = src[start : start + 600]
        op_at = body.find("migrate_op_wire(dir)")
        pins_at = body.find("migrate_pins(dir)")
        self.assertGreaterEqual(op_at, 0)
        self.assertGreater(pins_at, op_at)
