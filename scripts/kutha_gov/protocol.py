"""Harness check protocol. HIGH fails CI; LOW is advisory."""

from __future__ import annotations

from dataclasses import dataclass, field
from enum import StrEnum
from pathlib import Path


class Severity(StrEnum):
    HIGH = "high"
    LOW = "low"


@dataclass
class Finding:
    check: str
    severity: Severity
    category: str
    message: str
    file: str | None = None
    line: int | None = None

    def format(self) -> str:
        tag = "HIGH" if self.severity is Severity.HIGH else "low"
        loc = "-"
        if self.file and self.line:
            loc = f"{self.file}:{self.line}"
        elif self.file:
            loc = self.file
        return f"[{tag}] {loc} ({self.check}/{self.category}) {self.message}"


@dataclass
class CheckResult:
    check: str
    findings: list[Finding] = field(default_factory=list)
    scanned: int = 0
    note: str = ""

    @property
    def passed(self) -> bool:
        return not any(f.severity is Severity.HIGH for f in self.findings)

    @property
    def high_count(self) -> int:
        return sum(1 for f in self.findings if f.severity is Severity.HIGH)

    @property
    def low_count(self) -> int:
        return sum(1 for f in self.findings if f.severity is Severity.LOW)


@dataclass
class Context:
    root: Path
    fail_on_warn: bool = False

    def read(self, rel_path: str) -> str | None:
        path = self.root / rel_path
        if not path.is_file():
            return None
        return path.read_text(encoding="utf-8")


class Check:
    name: str = ""
    description: str = ""

    def run(self, ctx: Context) -> CheckResult:
        raise NotImplementedError
