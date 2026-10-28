"""H1: cargo test is an observation, not product SoT."""

from __future__ import annotations

import fcntl
import os
import re
import subprocess
import time
from collections.abc import Iterator, Mapping, Sequence
from contextlib import contextmanager
from pathlib import Path

from kutha_gov.protocol import Finding, Severity

type Step = Mapping[str, object]

_LOCK_NAME = ".kutha-gov-cargo.lock"
# Single category literal so M145 honesty selftest (count=1) stays non-vacuous.
_OBSERVE_INCOMPLETE = "observe-incomplete"
# Single category literal so M147 retried selftest (count=1) stays non-vacuous.
_OBSERVE_RETRIED = "observe-retried"


def _cargo_target_dir(cwd: Path) -> Path:
    raw = os.environ.get("CARGO_TARGET_DIR", "").strip()
    if raw:
        return Path(raw)
    return cwd / "target"


def _lock_wait_sec() -> float:
    raw = os.environ.get("KUTHA_GOV_CARGO_LOCK_WAIT_SEC", "").strip()
    if raw:
        try:
            return max(0.0, float(raw))
        except ValueError:
            pass
    return 600.0


def _observe_retries() -> int:
    """Extra cargo attempts after an incomplete observe (M146). Default 1."""
    raw = os.environ.get("KUTHA_GOV_CARGO_OBSERVE_RETRIES", "").strip()
    if raw:
        try:
            return max(0, int(raw))
        except ValueError:
            pass
    return 1


def _observe_fail_tail_chars() -> int:
    """Max chars of cargo output kept on observe-fail (M150). Default 240."""
    raw = os.environ.get("KUTHA_GOV_OBSERVE_FAIL_TAIL", "").strip()
    if raw:
        try:
            return max(0, int(raw))
        except ValueError:
            pass
    return 240


def _output_tail(text: str, *, limit: int) -> str:
    collapsed = " ".join(text.split())
    if limit <= 0 or not collapsed:
        return ""
    if len(collapsed) <= limit:
        return collapsed
    return collapsed[-limit:]


def _format_observe_fail(
    label: str,
    returncode: int,
    *,
    elapsed_ms: int,
    stdout: str,
    stderr: str,
) -> str:
    """Build observe-fail message with wall time and truncated output (M150)."""
    prefer = stderr if stderr.strip() else stdout
    tail = _output_tail(prefer, limit=_observe_fail_tail_chars())
    base = f"{label} exit {returncode} elapsed_ms={elapsed_ms}"
    if tail:
        return f"{base} tail={tail} (evidence, not SoT)"
    return f"{base} (evidence, not SoT)"


def _has_observe_incomplete(findings: Sequence[Finding]) -> bool:
    return any(f.category == _OBSERVE_INCOMPLETE for f in findings)


@contextmanager
def _exclusive_cargo_lock(cwd: Path) -> Iterator[None]:
    """M114: serialize observe Cargo against concurrent cargo/bench on the same target."""
    target = _cargo_target_dir(cwd)
    target.mkdir(parents=True, exist_ok=True)
    lock_path = target / _LOCK_NAME
    fd = os.open(lock_path, os.O_CREAT | os.O_RDWR, 0o644)
    deadline = time.monotonic() + _lock_wait_sec()
    try:
        while True:
            try:
                fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
                break
            except BlockingIOError:
                if time.monotonic() >= deadline:
                    raise TimeoutError(
                        f"cargo observe lock busy ({lock_path}); "
                        "waited KUTHA_GOV_CARGO_LOCK_WAIT_SEC"
                    ) from None
                time.sleep(0.05)
        yield
    finally:
        try:
            fcntl.flock(fd, fcntl.LOCK_UN)
        finally:
            os.close(fd)


def interpret_cargo_output(
    text: str,
    required: Sequence[str],
    *,
    cargo_exit: int | None = None,
) -> tuple[list[str], list[Finding]]:
    """Map cargo stdout to seen-ok names and HIGH findings.

    M145: when ``cargo_exit`` is non-zero and no required name has an ok/FAILED
    line, emit one incomplete finding instead of N x ``observe-missing`` so
    infra fails (exit 101 / cold target) stay readable.

    M149: same incomplete collapse when exit is non-zero, some names are
    missing, and no required name FAILED — a partial ``ok`` stream under
    infra failure must retry (M146), not mint N x missing.
    """
    seen_ok: list[str] = []
    findings: list[Finding] = []
    any_failed = False
    missing: list[str] = []
    for name in required:
        escaped = re.escape(name)
        ok_pat = re.compile(rf"^test (?:[\w:]+::)?{escaped} \.\.\. ok\b", re.MULTILINE)
        fail_pat = re.compile(rf"^test (?:[\w:]+::)?{escaped} \.\.\. FAILED\b", re.MULTILINE)
        if ok_pat.search(text):
            seen_ok.append(name)
        elif fail_pat.search(text):
            any_failed = True
            findings.append(
                Finding(
                    "observe_cargo",
                    Severity.HIGH,
                    "observe-fail",
                    f"observed {name} FAILED (evidence, not SoT)",
                )
            )
        else:
            missing.append(name)
    if cargo_exit is not None and cargo_exit != 0 and required and missing and not any_failed:
        return (
            [],
            [
                Finding(
                    "observe_cargo",
                    Severity.HIGH,
                    _OBSERVE_INCOMPLETE,
                    f"cargo exit {cargo_exit}; required tests not run "
                    "(re-run after warm build; evidence, not SoT)",
                )
            ],
        )
    for name in missing:
        findings.append(
            Finding(
                "observe_cargo",
                Severity.HIGH,
                "observe-missing",
                f"did not observe required test {name} in cargo output",
            )
        )
    return seen_ok, findings


def _str_list(step: Step, key: str) -> list[str]:
    value = step.get(key, [])
    if not isinstance(value, list):
        return []
    return [item for item in value if isinstance(item, str)]


def _mapping(step: Step, key: str) -> Mapping[str, object] | None:
    value = step.get(key)
    return value if isinstance(value, Mapping) else None


def _timeout_sec(step: Step) -> int:
    raw_env = os.environ.get("KUTHA_GOV_CARGO_TIMEOUT_SEC", "").strip()
    if raw_env:
        try:
            return max(1, int(raw_env))
        except ValueError:
            pass
    raw = step.get("timeout_sec", 180)
    return raw if isinstance(raw, int) and raw > 0 else 180


def _run_cargo(
    cwd: Path,
    spec: Step,
    *,
    default_args: list[str],
    timeout: int,
    label: str,
) -> tuple[int, str, list[Finding]]:
    bin_name = spec.get("bin", "cargo")
    bin_s = bin_name if isinstance(bin_name, str) else "cargo"
    args = _str_list(spec, "args") or default_args
    cmd = [bin_s, *args]
    env = os.environ.copy()
    env["CARGO_TERM_COLOR"] = "never"
    started = time.monotonic()
    try:
        completed = subprocess.run(
            cmd,
            cwd=str(cwd),
            check=False,
            capture_output=True,
            text=True,
            timeout=timeout,
            env=env,
        )
    except FileNotFoundError:
        return (
            127,
            "",
            [
                Finding(
                    "observe_cargo",
                    Severity.HIGH,
                    "observe-error",
                    f"{bin_s} not found (cannot {label})",
                )
            ],
        )
    except subprocess.TimeoutExpired:
        return (
            124,
            "",
            [
                Finding(
                    "observe_cargo",
                    Severity.HIGH,
                    "observe-error",
                    f"{label} timed out (evidence, not SoT)",
                )
            ],
        )
    elapsed_ms = int((time.monotonic() - started) * 1000)
    text = (completed.stdout or "") + "\n" + (completed.stderr or "")
    findings: list[Finding] = []
    if completed.returncode != 0:
        findings.append(
            Finding(
                "observe_cargo",
                Severity.HIGH,
                "observe-fail",
                _format_observe_fail(
                    label,
                    completed.returncode,
                    elapsed_ms=elapsed_ms,
                    stdout=completed.stdout or "",
                    stderr=completed.stderr or "",
                ),
            )
        )
    return completed.returncode, text, findings


def run_cargo_observation(root: object, spec: Step) -> tuple[int, list[str], list[Finding]]:
    cwd = root if isinstance(root, Path) else Path(str(root))
    timeout = _timeout_sec(spec)
    required = _str_list(spec, "required")
    try:
        with _exclusive_cargo_lock(cwd):
            attempts = 1 + _observe_retries()
            code = 0
            seen_ok: list[str] = []
            findings: list[Finding] = []
            recovered_after_incomplete = False
            for attempt in range(attempts):
                code, text, run_findings = _run_cargo(
                    cwd,
                    spec,
                    default_args=["test", "--workspace", "--offline"],
                    timeout=timeout,
                    label="cargo test",
                )
                seen_ok, observe_findings = interpret_cargo_output(
                    text,
                    required,
                    cargo_exit=code,
                )
                incomplete = _has_observe_incomplete(observe_findings)
                # Replace prior attempt findings so a successful retry is clean.
                findings = list(run_findings) + list(observe_findings)
                # A timeout or missing binary already spent the wall-clock budget;
                # retrying doubles it and contradicts the build skip below.
                if code in {124, 127}:
                    break
                if not incomplete:
                    if attempt > 0:
                        recovered_after_incomplete = True
                    break
                if attempt + 1 >= attempts:
                    break
            if recovered_after_incomplete and code == 0:
                findings.append(
                    Finding(
                        "observe_cargo",
                        Severity.LOW,
                        _OBSERVE_RETRIED,
                        "cargo recovered after incomplete retry (evidence, not SoT)",
                    )
                )
            build = _mapping(spec, "build")
            # Timeout / missing cargo: do not spend a second wall-clock budget on build.
            # Non-zero test status still builds so emit_tenant can ingest fail.
            if build is not None and code not in {124, 127}:
                build_timeout = _timeout_sec(build) if "timeout_sec" in build else timeout
                build_code, _build_text, build_findings = _run_cargo(
                    cwd,
                    build,
                    default_args=[
                        "build",
                        "--offline",
                        "-p",
                        "kutha-runtime",
                        "--bin",
                        "kutha-tenant",
                        "--bin",
                        "kutha-migrate",
                    ],
                    timeout=build_timeout,
                    label="cargo build kutha-tenant+kutha-migrate",
                )
                findings.extend(build_findings)
                if code == 0:
                    code = build_code
            return code, seen_ok, findings
    except TimeoutError as exc:
        return (
            75,
            [],
            [
                Finding(
                    "observe_cargo",
                    Severity.HIGH,
                    "observe-error",
                    f"{exc} (evidence, not SoT)",
                )
            ],
        )
