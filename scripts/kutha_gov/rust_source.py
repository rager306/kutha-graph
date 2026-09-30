"""Scan Rust sources for named #[test] items and asserting bodies (D-T1).

Comments, string literals (including raw strings), and char literals are blanked
before brace matching so `'{'`, `'"'`, and `r#"{ assert!(..); }"#` cannot fake a
body or an oracle. Nested `fn` items are not tests. This is not rustc.
"""

from __future__ import annotations

import re
from dataclasses import dataclass

ASSERT_MACROS: frozenset[str] = frozenset(
    {
        "assert",
        "assert_eq",
        "assert_ne",
        "debug_assert",
        "debug_assert_eq",
        "debug_assert_ne",
    }
)
DEFAULT_ASSERT_PREFIXES: tuple[str, ...] = ("assert_",)

_FN_RE = re.compile(
    r"""
    \b
    (?:pub(?:\s*\([^)]*\))?\s+)?
    (async\s+)?
    (?:unsafe\s+)?
    (?:extern\s+)?
    fn\s+
    ([A-Za-z_][A-Za-z0-9_]*)
    """,
    re.VERBOSE,
)
_MACRO_RE = re.compile(
    r"\b(assert|assert_eq|assert_ne|debug_assert|debug_assert_eq|debug_assert_ne)\s*!\s*\("
)
_CALL_RE = re.compile(r"\b([A-Za-z_][A-Za-z0-9_]*)\s*(!?)\s*\(")


def _ident_cont(ch: str) -> bool:
    return ch.isalnum() or ch == "_"


def _token_start(src: str, i: int) -> bool:
    return i == 0 or not _ident_cont(src[i - 1])


def _blank(chars: list[str], start: int, end: int) -> None:
    last = min(end, len(chars))
    for i in range(start, last):
        if chars[i] != "\n":
            chars[i] = " "


def _raw_open(src: str, i: int) -> tuple[int, int] | None:
    """Return (content_start, hash_count) for r#*\" / br#*\" / cr#*\"."""
    if not _token_start(src, i):
        return None
    n = len(src)
    j = i
    if j < n and src[j] in "bc":
        j += 1
        if j >= n or src[j] != "r":
            return None
    elif not (j < n and src[j] == "r"):
        return None
    j += 1
    hashes = 0
    while j < n and src[j] == "#":
        hashes += 1
        j += 1
    if j < n and src[j] == '"':
        return j + 1, hashes
    return None


def _scan_raw(src: str, content_start: int, hashes: int) -> int:
    i = content_start
    n = len(src)
    closer = '"' + ("#" * hashes)
    while i < n:
        if src.startswith(closer, i):
            return i + len(closer)
        i += 1
    return n


def _scan_string(src: str, i: int) -> int:
    """`i` points at the opening double-quote. Honours `\\` and `\\\"`."""
    i += 1
    n = len(src)
    while i < n:
        ch = src[i]
        if ch == "\\":
            i += 2
            continue
        if ch == '"':
            return i + 1
        i += 1
    return n


def _scan_tick(src: str, i: int) -> tuple[str, int]:
    """Classify `'…'` / `b'…'` / `c'…'` vs a lifetime starting at `i`."""
    n = len(src)
    if src[i] in "bc" and i + 1 < n and src[i + 1] == "'":
        if not _token_start(src, i):
            return "other", i + 1
        i += 1
    if i >= n or src[i] != "'":
        return "other", i + 1
    i += 1
    if i >= n:
        return "other", i
    if src[i] == "\\":
        i += 1
        if i < n and src[i] == "u" and i + 1 < n and src[i + 1] == "{":
            i += 2
            while i < n and src[i] != "}":
                i += 1
            if i < n:
                i += 1
        elif i < n:
            i += 1
        if i < n and src[i] == "'":
            i += 1
        return "char", i
    if src[i].isalpha() or src[i] == "_":
        j = i + 1
        while j < n and _ident_cont(src[j]):
            j += 1
        if j == i + 1 and j < n and src[j] == "'":
            return "char", j + 1
        return "lifetime", j
    if i + 1 < n and src[i + 1] == "'":
        return "char", i + 2
    return "char", i + 1


def _scan_block_comment(src: str, i: int) -> int:
    n = len(src)
    depth = 1
    i += 2
    while i < n and depth:
        if src.startswith("/*", i):
            depth += 1
            i += 2
            continue
        if src.startswith("*/", i):
            depth -= 1
            i += 2
            continue
        i += 1
    return i


def strip_noise(src: str) -> str:
    """Same-length copy with comments, strings, and char literals blanked."""
    chars = list(src)
    i = 0
    n = len(src)
    while i < n:
        raw = _raw_open(src, i)
        if raw is not None:
            content_start, hashes = raw
            end = _scan_raw(src, content_start, hashes)
            _blank(chars, i, end)
            i = end
            continue
        if _token_start(src, i) and src[i] in "bc" and i + 1 < n and src[i + 1] == '"':
            end = _scan_string(src, i + 1)
            _blank(chars, i, end)
            i = end
            continue
        if src[i] == '"':
            end = _scan_string(src, i)
            _blank(chars, i, end)
            i = end
            continue
        if src[i] == "'" or (
            _token_start(src, i) and src[i] in "bc" and i + 1 < n and src[i + 1] == "'"
        ):
            kind, end = _scan_tick(src, i)
            if kind == "char":
                _blank(chars, i, end)
            i = end
            continue
        if src.startswith("//", i):
            end = src.find("\n", i)
            if end < 0:
                end = n
            _blank(chars, i, end)
            i = end
            continue
        if src.startswith("/*", i):
            end = _scan_block_comment(src, i)
            _blank(chars, i, end)
            i = end
            continue
        i += 1
    return "".join(chars)


def match_brace(text: str, open_i: int) -> int | None:
    depth = 0
    for i in range(open_i, len(text)):
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                return i + 1
    return None


def find_fn_body(text: str, name_end: int) -> tuple[int, int] | None:
    """Body `{` … `}` after a signature; `{` inside `<>`/`()`/`[]` is not the body."""
    i = name_end
    n = len(text)
    angle = 0
    paren = 0
    bracket = 0
    while i < n:
        ch = text[i]
        if ch == "-" and i + 1 < n and text[i + 1] == ">":
            i += 2
            continue
        if angle == 0 and paren == 0 and bracket == 0:
            if ch == "{":
                end = match_brace(text, i)
                if end is None:
                    return None
                return i, end
            if ch == ";":
                return None
        if ch == "<":
            angle += 1
        elif ch == ">" and angle > 0:
            angle -= 1
        elif ch == "(":
            paren += 1
        elif ch == ")" and paren > 0:
            paren -= 1
        elif ch == "[":
            bracket += 1
        elif ch == "]" and bracket > 0:
            bracket -= 1
        i += 1
    return None


def _skip_ws_left(text: str, i: int) -> int:
    j = i
    while j > 0 and text[j - 1].isspace():
        j -= 1
    return j


def _match_bracket_left(text: str, close_i: int) -> int | None:
    depth = 0
    i = close_i
    while i >= 0:
        ch = text[i]
        if ch == "]":
            depth += 1
        elif ch == "[":
            depth -= 1
            if depth == 0:
                return i
        i -= 1
    return None


def attributes_before(text: str, fn_start: int) -> tuple[str, ...]:
    found: list[str] = []
    i = fn_start
    while True:
        i = _skip_ws_left(text, i)
        if i <= 0 or text[i - 1] != "]":
            break
        open_i = _match_bracket_left(text, i - 1)
        if open_i is None or open_i == 0 or text[open_i - 1] != "#":
            break
        found.append(text[open_i - 1 : i])
        i = open_i - 1
    found.reverse()
    return tuple(found)


def _attr_compact(attr: str) -> str:
    inner = attr[2:-1] if attr.startswith("#[") and attr.endswith("]") else attr
    return "".join(inner.split())


def is_test_attr(attr: str) -> bool:
    compact = _attr_compact(attr)
    return compact == "test" or compact.startswith("test(") or compact.startswith("test=")


def is_ignore_attr(attr: str) -> bool:
    compact = _attr_compact(attr)
    return compact == "ignore" or compact.startswith("ignore(") or compact.startswith("ignore=")


@dataclass(frozen=True)
class FnItem:
    name: str
    start: int
    async_fn: bool
    nested: bool
    attrs: tuple[str, ...]
    body_start: int
    body_end: int

    @property
    def has_test(self) -> bool:
        return any(is_test_attr(attr) for attr in self.attrs)

    @property
    def has_ignore(self) -> bool:
        return any(is_ignore_attr(attr) for attr in self.attrs)

    def line(self, src: str) -> int:
        return src[: self.start].count("\n") + 1


def parse_fns(src: str) -> list[FnItem]:
    stripped = strip_noise(src)
    items: list[FnItem] = []
    bodies: list[tuple[int, int]] = []
    for match in _FN_RE.finditer(stripped):
        nested = any(start < match.start() < end for start, end in bodies)
        body = find_fn_body(stripped, match.end())
        if body is None:
            continue
        bodies.append(body)
        items.append(
            FnItem(
                name=match.group(2),
                start=match.start(),
                async_fn=bool(match.group(1)),
                nested=nested,
                attrs=attributes_before(stripped, match.start()),
                body_start=body[0],
                body_end=body[1],
            )
        )
    return items


def body_text(stripped: str, item: FnItem) -> str:
    return stripped[item.body_start : item.body_end]


def first_macro_span_in(text: str) -> tuple[int, int] | None:
    match = _MACRO_RE.search(text)
    if match is None:
        return None
    return match.start(), match.end()


def assert_hits(body: str, prefixes: tuple[str, ...] = DEFAULT_ASSERT_PREFIXES) -> bool:
    if first_macro_span_in(body) is not None:
        return True
    for match in _CALL_RE.finditer(body):
        name, bang = match.group(1), match.group(2)
        if bang == "!":
            continue
        if any(name.startswith(prefix) for prefix in prefixes if prefix):
            return True
    return False


def missing_symbols(body: str, symbols: list[str]) -> list[str]:
    return [symbol for symbol in symbols if symbol not in body]


def assert_macro_spans_in_test(src: str, name: str) -> list[tuple[int, int]]:
    """Original-file spans of every assert…!( in a qualifying named test body."""
    stripped = strip_noise(src)
    out: list[tuple[int, int]] = []
    for item in parse_fns(src):
        if item.name != name or item.nested or item.async_fn or item.has_ignore:
            continue
        if not item.has_test:
            continue
        body = body_text(stripped, item)
        for match in _MACRO_RE.finditer(body):
            out.append((item.body_start + match.start(), item.body_start + match.end()))
        if out:
            return out
    return out


def first_assert_macro_span(src: str, name: str) -> tuple[int, int] | None:
    """Original-file span of the first assert…!( in a qualifying named test body."""
    spans = assert_macro_spans_in_test(src, name)
    if not spans:
        return None
    return spans[0]


def line_at(src: str, index: int) -> int:
    return src[:index].count("\n") + 1
