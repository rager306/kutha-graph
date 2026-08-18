"""H1: cargo test is an observation, not product SoT."""

from __future__ import annotations

import os
import re
import subprocess
from collections.abc import Mapping, Sequence
from pathlib import Path

from kutha_gov.protocol import Finding, Severity

type Step = Mapping[str, object]


def interpret_cargo_output(text: str, required: Sequence[str]) -> tuple[list[str], list[Finding]]:
    seen_ok: list[str] = []
    findings: list[Finding] = []
    for name in required:
        escaped = re.escape(name)
        ok_pat = re.compile(rf"^test {escaped} \.\.\. ok\b", re.MULTILINE)
        fail_pat = re.compile(rf"^test {escaped} \.\.\. FAILED\b", re.MULTILINE)
        if ok_pat.search(text):
            seen_ok.append(name)
        elif fail_pat.search(text):
            findings.append(
                Finding(
                    "observe_cargo",
                    Severity.HIGH,
                    "observe-fail",
                    f"observed {name} FAILED (evidence, not SoT)",
                )
            )
        else:
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
    text = (completed.stdout or "") + "\n" + (completed.stderr or "")
    findings: list[Finding] = []
    if completed.returncode != 0:
        findings.append(
            Finding(
                "observe_cargo",
                Severity.HIGH,
                "observe-fail",
                f"{label} exit {completed.returncode} (evidence, not SoT)",
            )
        )
    return completed.returncode, text, findings


def run_cargo_observation(root: object, spec: Step) -> tuple[int, list[str], list[Finding]]:
    cwd = root if isinstance(root, Path) else Path(str(root))
    timeout = _timeout_sec(spec)
    code, text, findings = _run_cargo(
        cwd,
        spec,
        default_args=["test", "--workspace", "--offline"],
        timeout=timeout,
        label="cargo test",
    )
    seen_ok, observe_findings = interpret_cargo_output(text, _str_list(spec, "required"))
    findings.extend(observe_findings)
    build = _mapping(spec, "build")
    # Timeout / missing cargo: do not spend a second wall-clock budget on build.
    # Non-zero test status still builds so emit_tenant can ingest fail.
    if build is not None and code not in {124, 127}:
        build_timeout = _timeout_sec(build) if "timeout_sec" in build else timeout
        build_code, _build_text, build_findings = _run_cargo(
            cwd,
            build,
            default_args=["build", "--offline", "-p", "kutha-runtime", "--bin", "kutha-tenant"],
            timeout=build_timeout,
            label="cargo build kutha-tenant",
        )
        findings.extend(build_findings)
        if code == 0:
            code = build_code
    return code, seen_ok, findings
