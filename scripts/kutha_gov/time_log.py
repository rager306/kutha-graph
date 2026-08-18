"""Time axis for the harness: append-only observation log + deterministic fold.

H0: JSONL beside `.kutha/` is the *process* SoT for CI quanta. Product SoT
remains `crates/kutha-runtime`. H2 will map these records onto `kutha_common::Op`
(typed interned triples), not the STCA-guide tutorial JSON merge-patch skeleton.

STATE.md / ROADMAP.md are droppable pictures of human intent + last fold.
They are not a second SoT (ADR-002 Time).
"""

from __future__ import annotations

import json
import os
import time
import uuid
from dataclasses import dataclass
from datetime import UTC, datetime
from pathlib import Path
from typing import Any

LOG_REL = ".kutha/events.jsonl"


def _now_v7() -> str:
    """RFC 9562 UUID v7. stdlib uuid.uuid7 arrives in 3.14; harness pins 3.13."""
    unix_ms = int(time.time() * 1000) & ((1 << 48) - 1)
    rand_a = int.from_bytes(os.urandom(2), "big") & 0x0FFF
    rand_b = int.from_bytes(os.urandom(8), "big") & ((1 << 62) - 1)
    value = (unix_ms << 80) | (0x7 << 76) | (rand_a << 64) | (0b10 << 62) | rand_b
    return str(uuid.UUID(int=value))


@dataclass(frozen=True)
class HarnessEvent:
    """Lean TGMS-shaped record. Same axes as kutha-common Event, no JSON patch."""

    id: str
    op: str  # assert | retract
    subject: str
    relation: str
    object: str
    ingested_at: int
    valid_from: int
    caused_by: str | None = None

    def to_json(self) -> dict[str, Any]:
        return {
            "id": self.id,
            "op": self.op,
            "subject": self.subject,
            "relation": self.relation,
            "object": self.object,
            "ingested_at": self.ingested_at,
            "valid_from": self.valid_from,
            "caused_by": self.caused_by,
        }

    @classmethod
    def from_json(cls, row: dict[str, Any]) -> HarnessEvent:
        return cls(
            id=str(row["id"]),
            op=str(row["op"]),
            subject=str(row["subject"]),
            relation=str(row["relation"]),
            object=str(row["object"]),
            ingested_at=int(row["ingested_at"]),
            valid_from=int(row["valid_from"]),
            caused_by=row.get("caused_by"),
        )


@dataclass
class HarnessFold:
    last_run: str = "none"  # ok | fail | none
    last_high: int = 0
    last_low: int = 0
    run_count: int = 0

    def apply(self, event: HarnessEvent) -> None:
        if event.op != "assert":
            return
        if event.subject == "harness.run" and event.relation == "status":
            self.last_run = event.object
            self.run_count += 1
        elif event.subject == "harness.run" and event.relation == "high":
            self.last_high = int(event.object)
        elif event.subject == "harness.run" and event.relation == "low":
            self.last_low = int(event.object)


def fold_log(path: Path) -> HarnessFold:
    picture = HarnessFold()
    if not path.is_file():
        return picture
    seq = 0
    for line in path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line:
            continue
        event = HarnessEvent.from_json(json.loads(line))
        picture.apply(event)
        seq += 1
    return picture


def append_run(
    root: Path,
    *,
    high: int,
    low: int,
    check_count: int,
    observations: list[tuple[str, str]] | None = None,
) -> HarnessEvent:
    """One runtime quantum: emit run status (+ counts). File adapter, not product WAL."""
    log_path = root / LOG_REL
    log_path.parent.mkdir(parents=True, exist_ok=True)
    ingested = int(datetime.now(UTC).timestamp())
    run_id = _now_v7()
    status = "fail" if high else "ok"
    events = [
        HarnessEvent(run_id, "assert", "harness.run", "status", status, ingested, ingested),
        HarnessEvent(
            _now_v7(), "assert", "harness.run", "high", str(high), ingested, ingested, run_id
        ),
        HarnessEvent(
            _now_v7(), "assert", "harness.run", "low", str(low), ingested, ingested, run_id
        ),
        HarnessEvent(
            _now_v7(),
            "assert",
            "harness.run",
            "checks",
            str(check_count),
            ingested,
            ingested,
            run_id,
        ),
    ]
    for relation, obj in observations or []:
        events.append(
            HarnessEvent(
                _now_v7(),
                "assert",
                "harness.observe",
                relation,
                obj,
                ingested,
                ingested,
                run_id,
            )
        )
    with log_path.open("a", encoding="utf-8") as handle:
        for event in events:
            handle.write(json.dumps(event.to_json(), separators=(",", ":")) + "\n")
    return events[0]
