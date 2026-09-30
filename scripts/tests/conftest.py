"""Route test temp files to the repo-local gitignored `tmp/` (not system /tmp)."""

from __future__ import annotations

import tempfile
from pathlib import Path

_SCRATCH = Path(__file__).resolve().parents[2] / "tmp"
_SCRATCH.mkdir(exist_ok=True)
tempfile.tempdir = str(_SCRATCH)
