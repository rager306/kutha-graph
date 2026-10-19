"""M114 S01 — observe_cargo takes an exclusive flock on the Cargo target dir."""

from __future__ import annotations

import fcntl
import os
import subprocess
import sys
import tempfile
import textwrap
import time
import unittest
from pathlib import Path

from kutha_gov.observe import run_cargo_observation


def _stub_cargo(root: Path) -> Path:
    stub = root / "stub-cargo"
    stub.write_text(
        "#!/bin/sh\necho 'test m114_lock_oracle ... ok'\nexit 0\n",
        encoding="utf-8",
    )
    stub.chmod(0o755)
    return stub


class CargoObserveLockTests(unittest.TestCase):
    def test_observe_times_out_when_cargo_lock_held(self) -> None:
        root = Path(tempfile.mkdtemp(prefix="kutha-m114-"))
        stub = _stub_cargo(root)
        target = root / "target"
        target.mkdir()
        lock_path = target / ".kutha-gov-cargo.lock"
        holder = subprocess.Popen(
            [
                sys.executable,
                "-c",
                textwrap.dedent(
                    f"""
                    import fcntl, os, time
                    fd = os.open({str(lock_path)!r}, os.O_CREAT | os.O_RDWR, 0o644)
                    fcntl.flock(fd, fcntl.LOCK_EX)
                    time.sleep(30)
                    """
                ),
            ],
        )
        try:
            # Wait until the child holds the lock.
            for _ in range(50):
                if lock_path.is_file():
                    probe = os.open(lock_path, os.O_RDWR)
                    try:
                        fcntl.flock(probe, fcntl.LOCK_EX | fcntl.LOCK_NB)
                    except BlockingIOError:
                        os.close(probe)
                        break
                    else:
                        fcntl.flock(probe, fcntl.LOCK_UN)
                        os.close(probe)
                time.sleep(0.02)
            else:
                self.fail("holder never acquired the cargo lock")

            prev_wait = os.environ.get("KUTHA_GOV_CARGO_LOCK_WAIT_SEC")
            prev_target = os.environ.get("CARGO_TARGET_DIR")
            os.environ["KUTHA_GOV_CARGO_LOCK_WAIT_SEC"] = "0"
            os.environ["CARGO_TARGET_DIR"] = str(target)
            try:
                code, _ok, findings = run_cargo_observation(
                    root,
                    {
                        "bin": str(stub),
                        "args": ["test"],
                        "required": ["m114_lock_oracle"],
                        "timeout_sec": 5,
                    },
                )
            finally:
                if prev_wait is None:
                    os.environ.pop("KUTHA_GOV_CARGO_LOCK_WAIT_SEC", None)
                else:
                    os.environ["KUTHA_GOV_CARGO_LOCK_WAIT_SEC"] = prev_wait
                if prev_target is None:
                    os.environ.pop("CARGO_TARGET_DIR", None)
                else:
                    os.environ["CARGO_TARGET_DIR"] = prev_target
        finally:
            holder.terminate()
            holder.wait(timeout=5)
        self.assertNotEqual(0, code)
        msgs = " ".join(f.message for f in findings)
        self.assertIn("lock", msgs.lower())

    def test_concurrent_observe_does_not_overlap_stub(self) -> None:
        root = Path(tempfile.mkdtemp(prefix="kutha-m114-ov-"))
        log = root / "runs.log"
        stub = root / "stub-cargo"
        stub.write_text(
            "#!/bin/sh\n"
            f"echo start >> '{log}'\n"
            "sleep 0.25\n"
            f"echo end >> '{log}'\n"
            "echo 'test m114_overlap ... ok'\n"
            "exit 0\n",
            encoding="utf-8",
        )
        stub.chmod(0o755)
        target = root / "target"
        target.mkdir()
        worker = textwrap.dedent(
            f"""
            import os, sys
            sys.path.insert(0, {str(Path(__file__).resolve().parents[1])!r})
            os.environ["CARGO_TARGET_DIR"] = {str(target)!r}
            from kutha_gov.observe import run_cargo_observation
            from pathlib import Path
            code, _ok, findings = run_cargo_observation(
                Path({str(root)!r}),
                {{
                    "bin": {str(stub)!r},
                    "args": ["test"],
                    "required": ["m114_overlap"],
                    "timeout_sec": 15,
                }},
            )
            raise SystemExit(0 if code == 0 and not findings else 1)
            """
        )
        env = os.environ.copy()
        env["CARGO_TARGET_DIR"] = str(target)
        # Prefer repo scripts/ on PYTHONPATH for kutha_gov.
        scripts = str(Path(__file__).resolve().parents[1])
        env["PYTHONPATH"] = scripts + os.pathsep + env.get("PYTHONPATH", "")
        procs = [
            subprocess.Popen([sys.executable, "-c", worker], env=env)
            for _ in range(2)
        ]
        codes = [p.wait(timeout=30) for p in procs]
        self.assertEqual([0, 0], codes)
        lines = log.read_text(encoding="utf-8").splitlines()
        self.assertEqual(["start", "end", "start", "end"], lines)


if __name__ == "__main__":
    unittest.main()
