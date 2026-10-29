#!/usr/bin/env python3
"""Fetch a tiny eCFR fixture for the Kutha legal PIT probe.

eCFR is a US government work; see the fixture README for public-domain status.
This tool is not run in CI. Raw XML stays in gitignored tmp/; only extracted
text is committed.
"""
from __future__ import annotations

import argparse
import gzip
import sys
import time
import urllib.error
import urllib.request
import xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "crates/kutha-runtime/tests/fixtures/ecfr"
RAW = ROOT / "tmp/ecfr-raw"

SECTIONS = ("304.1", "304.2", "304.10")
DATES = ("2015-12-18", "2017-01-23", "2017-03-14")
BASE = "https://www.ecfr.gov/api/versioner/v1/full/{date}/title-1.xml?part=304&section={section}"


def fetch(url: str) -> str:
    request = urllib.request.Request(
        url,
        headers={
            "Accept-Encoding": "gzip",
            "User-Agent": "kutha-research/0.0.01 (PIT probe; contact: repo owner)",
        },
    )
    with urllib.request.urlopen(request, timeout=60) as response:
        body = response.read()
        if response.headers.get("Content-Encoding") == "gzip":
            body = gzip.decompress(body)
    return body.decode("utf-8", errors="replace")


def extract(xml_text: str, section: str) -> str:
    if "<DIV8" not in xml_text or "<HEAD>" not in xml_text:
        raise ValueError(f"{section}: response is not eCFR section XML")
    root = ET.fromstring(xml_text)
    div = root if root.tag == "DIV8" else root.find(".//DIV8")
    if div is None:
        raise ValueError(f"{section}: DIV8 missing")
    lines: list[str] = []
    head = div.find("HEAD")
    if head is not None and "".join(head.itertext()).strip():
        lines.append("".join(head.itertext()).strip())
    for p in div.iter("P"):
        text = "".join(p.itertext()).strip()
        if text:
            lines.append(text)
    if not lines:
        raise ValueError(f"{section}: no HEAD/P text")
    return "\n\n".join(lines) + "\n"


def fetch_one(section: str, date: str) -> str:
    url = BASE.format(date=date, section=section)
    RAW.mkdir(parents=True, exist_ok=True)
    raw_path = RAW / f"{section}_{date}.xml"
    last = ""
    for attempt in range(5):
        try:
            xml_text = fetch(url)
            text = extract(xml_text, section)
            raw_path.write_text(xml_text, encoding="utf-8")
            return text
        except (urllib.error.URLError, TimeoutError, ValueError) as exc:
            last = str(exc)
            time.sleep(2 * (attempt + 1))
    raise SystemExit(f"{section} {date}: failed after retries: {last}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true", help="validate committed fixtures only")
    args = parser.parse_args()
    OUT.mkdir(parents=True, exist_ok=True)
    for section in SECTIONS:
        for date in DATES:
            path = OUT / f"{section}_{date}.txt"
            if args.check:
                if not path.is_file() or not path.read_text(encoding="utf-8").strip():
                    print(f"MISSING {path.relative_to(ROOT)}", file=sys.stderr)
                    return 1
                print(f"OK {path.relative_to(ROOT)}")
                continue
            text = fetch_one(section, date)
            path.write_text(text, encoding="utf-8")
            print(f"wrote {path.relative_to(ROOT)} ({len(text)} bytes)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
