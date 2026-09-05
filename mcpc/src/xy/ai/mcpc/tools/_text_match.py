"""Shared text matching over a target block (file content or node source).

Matching escalates through successive tolerance levels so a failed edit can be
retried more permissively without hallucinated whitespace/escaping/quoting
breaking it:

* 1 – whitespace runs match any whitespace (default behaviour).
* 2 – a whitespace run also matches literal escape sequences (``\\n``, ``\\t`` …),
  so real newlines match the escaped ones of an AST-unparsed string literal.
* 3 – quote/backslash delimiter runs match any such run regardless of kind or
  length, absorbing wrong string-literal quoting/escaping.

Callers pass an ``accept`` guard to veto a level: AST tools validate the result
through their engine, plain-text tools require the match to preserve line
structure (so no two lines are merged into a syntax error).
"""
import re
from collections.abc import Callable
from dataclasses import dataclass
_LEVELS = (1, 2, 3)
_SEP_ESCAPE = '(?:\\s|\\\\[ntrfv0abx])'
_DELIM_CHARS = '\'"\\'
_DELIM_RUN = '[\\\'"\\\\]*'

@dataclass(frozen=True)
class MatchResult:
    count: int
    start: int = -1
    end: int = -1

class TextMatchError(Exception):
    """Base error for the shared text-block matcher."""

class TextNotFound(TextMatchError):
    pass

class TextAmbiguous(TextMatchError):

    def __init__(self, message: str, count: int) -> None:
        super().__init__(message)
        self.count = count
'# accept(matched_span, result_text) -> keep this candidate'
ReplaceGuard = Callable[[str, str], bool]
'# accept(begin_span, end_span, result_text) -> keep this candidate'
MarksGuard = Callable[[str, str, str], bool]

def _delim_runs(part: str) -> list[tuple[str, bool]]:
    runs: list[tuple[str, bool]] = []
    for ch in part:
        is_delim = ch in _DELIM_CHARS
        if runs and runs[-1][1] == is_delim:
            runs[-1] = (runs[-1][0] + ch, is_delim)
        else:
            runs.append((ch, is_delim))
    return runs

def _token(part: str, level: int) -> str:
    if level < 3:
        return re.escape(part)
    return ''.join((_DELIM_RUN if is_delim else re.escape(run) for run, is_delim in _delim_runs(part)))

def _pattern(needle: str, level: int) -> re.Pattern[str]:
    parts = [p for p in re.split('(\\s+)', needle) if p != '']
    last = len(parts) - 1
    segments: list[str] = []
    for i, part in enumerate(parts):
        if part.isspace():
            interior = 0 < i < last
            if interior:
                segments.append(_SEP_ESCAPE + '+' if level >= 2 else '\\s+')
            else:
                segments.append(re.escape(part))
        else:
            segments.append(_token(part, level))
    return re.compile(''.join(segments))

def _matches(haystack: str, needle: str, level: int) -> list[MatchResult]:
    if level == 0:
        out: list[MatchResult] = []
        start = 0
        while (idx := haystack.find(needle, start)) != -1:
            out.append(MatchResult(1, idx, idx + len(needle)))
            start = idx + len(needle)
        return out
    return [MatchResult(1, m.start(), m.end()) for m in _pattern(needle, level).finditer(haystack)]

def _levels(exact: bool, max_level: int) -> tuple[int, ...]:
    return (0,) if exact else tuple((level for level in _LEVELS if level <= max_level))

def find_all(haystack: str, needle: str, *, exact: bool) -> list[MatchResult]:
    """Return all non-overlapping occurrences of ``needle`` (exact or level-1)."""
    return _matches(haystack, needle, 0 if exact else 1)

def find(haystack: str, needle: str, *, exact: bool) -> MatchResult:
    found = find_all(haystack, needle, exact=exact)
    if len(found) != 1:
        return MatchResult(count=len(found))
    return found[0]

def line_preserving(reference: str) -> ReplaceGuard:
    """Guard: the matched span keeps ``reference``'s newline count (no merged lines)."""
    expected = reference.count('\n')
    return lambda span, _result: span.count('\n') == expected

def marks_line_preserving(begin_marker: str, end_marker: str) -> MarksGuard:
    begin_n, end_n = (begin_marker.count('\n'), end_marker.count('\n'))
    return lambda begin_span, end_span, _result: begin_span.count('\n') == begin_n and end_span.count('\n') == end_n

def _mirror_escaping(span: str, replacement: str) -> str:
    """Encode ``replacement``'s raw newlines/tabs like the replaced ``span``.

    When the region being replaced sits inside a single-line string literal (its
    newlines are escaped ``\\n`` rather than raw), a replacement carrying raw
    newlines would break the literal. Mirror the span's escaping so it stays valid.
    """
    if '\n' in replacement and '\\n' in span and ('\n' not in span):
        return replacement.replace('\\', '\\\\').replace('\n', '\\n').replace('\t', '\\t')
    return replacement

def replace_in_block(block: str, old_text: str, new_text: str, *, exact: bool, replace_all: bool=False, accept: ReplaceGuard | None=None, max_level: int=3, where: str='block') -> str:
    """Replace ``old_text`` with ``new_text`` in ``block``, escalating tolerance.

    The first tolerance level yielding a unique (or, with ``replace_all``, any)
    match whose result is approved by ``accept`` wins. Raises :class:`TextNotFound`
    or :class:`TextAmbiguous` (with ``where`` naming the target) otherwise.
    """
    for level in _levels(exact, max_level):
        found = _matches(block, old_text, level)
        if not found:
            continue
        if not replace_all and len(found) > 1:
            raise TextAmbiguous(f'Text is ambiguous – found {len(found)} occurrences in {where}.', len(found))
        result = block
        spans: list[str] = []
        for match in sorted(found, key=lambda m: m.start, reverse=True):
            span = block[match.start:match.end]
            spans.append(span)
            result = result[:match.start] + _mirror_escaping(span, new_text) + result[match.end:]
        if accept and (not all((accept(span, result) for span in spans))):
            continue
        return result
    raise TextNotFound(f'Text not found in {where}.')

def replace_between(block: str, begin_marker: str, end_marker: str, content: str, *, exact: bool, accept: MarksGuard | None=None, max_level: int=3, where: str='block') -> str:
    """Replace the span between (and including) both markers with ``content``.

    Both markers are matched at the same escalating tolerance level; the first
    level whose unique markers yield an ``accept``-approved result wins. Raises
    :class:`TextNotFound`/:class:`TextAmbiguous`/:class:`TextMatchError`.
    """
    start_found = False
    for level in _levels(exact, max_level):
        starts = _matches(block, begin_marker, level)
        if not starts:
            continue
        if len(starts) > 1:
            raise TextAmbiguous(f'Start marker is ambiguous – found {len(starts)} occurrences in {where}.', len(starts))
        start_found = True
        ends = _matches(block, end_marker, level)
        if not ends:
            continue
        if len(ends) > 1:
            raise TextAmbiguous(f'End marker is ambiguous – found {len(ends)} occurrences in {where}.', len(ends))
        sm, em = (starts[0], ends[0])
        if em.start < sm.end:
            raise TextMatchError('End marker must start after start marker ends.')
        result = block[:sm.start] + _mirror_escaping(block[sm.start:em.end], content) + block[em.end:]
        if accept and (not accept(block[sm.start:sm.end], block[em.start:em.end], result)):
            continue
        return result
    if not start_found:
        raise TextNotFound(f'Start marker not found in {where}.')
    raise TextNotFound(f'End marker not found in {where}.')