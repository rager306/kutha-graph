"""H2: ingest process JSONL through kutha-tenant (product runtime), not SoT."""

from __future__ import annotations

import os
import re
import subprocess
from collections.abc import Mapping
from pathlib import Path

from kutha_gov.protocol import Finding, Severity

type Step = Mapping[str, object]


def interpret_tenant_output(text: str) -> list[Finding]:
    findings: list[Finding] = []
    match = re.search(
        r"^tenant: ingested=(\d+) last_status=(\S+) last_vf=(\d+) as_of_match=(\d+)", text, re.M
    )
    if match is None:
        findings.append(
            Finding(
                "emit_tenant",
                Severity.HIGH,
                "tenant-missing",
                "did not observe tenant: ingested=… as_of_match=… (evidence, not SoT)",
            )
        )
        return findings
    ingested, status, _vf, matched = match.groups()
    if ingested == "0" or matched != "1":
        findings.append(
            Finding(
                "emit_tenant",
                Severity.HIGH,
                "tenant-fail",
                f"tenant ingest={ingested} status={status} as_of_match={matched} (evidence, not SoT)",
            )
        )
    return findings


def _str_list(step: Step, key: str) -> list[str]:
    value = step.get(key, [])
    if not isinstance(value, list):
        return []
    return [item for item in value if isinstance(item, str)]


def _timeout_sec(step: Step) -> int:
    raw_env = os.environ.get("KUTHA_TENANT_TIMEOUT_SEC", "").strip()
    if raw_env:
        try:
            return max(1, int(raw_env))
        except ValueError:
            pass
    raw = step.get("timeout_sec", 180)
    return raw if isinstance(raw, int) and raw > 0 else 180


def resolve_tenant_bin(root: Path, spec: Step) -> str:
    """Binary path: env, then an existing spec path, then CARGO_TARGET_DIR/debug/<name>."""
    env_bin = os.environ.get("KUTHA_TENANT_BIN", "").strip()
    if env_bin:
        return env_bin
    named = spec.get("bin")
    named_s = named.strip() if isinstance(named, str) and named.strip() else "kutha-tenant"
    candidate = Path(named_s)
    if candidate.is_file():
        return str(candidate)
    rooted = root / named_s
    if rooted.is_file():
        return str(rooted)
    target = os.environ.get("CARGO_TARGET_DIR", "").strip()
    target_path = Path(target) if target else root / "target"
    return str(target_path / "debug" / Path(named_s).name)


def run_tenant_observation(root: object, spec: Step) -> tuple[int, list[Finding], str]:
    cwd = root if isinstance(root, Path) else Path(str(root))
    bin_s = resolve_tenant_bin(cwd, spec)
    args = _str_list(spec, "args")
    env = os.environ.copy()
    env["CARGO_TERM_COLOR"] = "never"
    harness = env.get("KUTHA_HARNESS_LOG", "").strip() or ".kutha/events.jsonl"
    tenant_dir = env.get("KUTHA_TENANT_DIR", "").strip() or ".kutha/tenant"
    env["KUTHA_HARNESS_LOG"] = harness
    env["KUTHA_TENANT_DIR"] = tenant_dir
    try:
        completed = subprocess.run(
            [bin_s, *args],
            cwd=str(cwd),
            check=False,
            capture_output=True,
            text=True,
            timeout=_timeout_sec(spec),
            env=env,
        )
    except FileNotFoundError:
        return (
            127,
            [
                Finding(
                    "emit_tenant",
                    Severity.HIGH,
                    "tenant-error",
                    f"{bin_s} not found (cannot ingest process log onto Kutha)",
                )
            ],
            "",
        )
    except subprocess.TimeoutExpired:
        return (
            124,
            [
                Finding(
                    "emit_tenant",
                    Severity.HIGH,
                    "tenant-error",
                    "kutha-tenant timed out (evidence, not SoT)",
                )
            ],
            "",
        )
    text = (completed.stdout or "") + "\n" + (completed.stderr or "")
    findings = interpret_tenant_output(text)
    if completed.returncode != 0 and not findings:
        findings.append(
            Finding(
                "emit_tenant",
                Severity.HIGH,
                "tenant-fail",
                f"kutha-tenant exit {completed.returncode} (evidence, not SoT)",
            )
        )
    return completed.returncode, findings, text
