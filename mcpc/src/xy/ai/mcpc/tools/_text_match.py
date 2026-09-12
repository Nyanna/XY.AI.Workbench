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
import difflib
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
    """Raised when the search text/marker could not be applied.

    ``reason`` classifies the cause: ``whitespace_mismatch`` (matches once all
    whitespace is ignored – a verified fix is in ``corrected_text``),
    ``content_changed`` (an unverified single-candidate guess is in ``guess``),
    ``guard_rejected`` (text found but the edit would merge/split lines or break
    syntax), or ``not_found`` (no lead at all – see ``next_step``). ``position``
    marks which marker failed (``start``/``end``), for marker-based edits.
    """

    def __init__(self, message: str, *, reason: str, position: str | None=None, corrected_text: str | None=None, guess: str | None=None, next_step: str | None=None) -> None:
        super().__init__(message)
        self.reason = reason
        self.position = position
        self.corrected_text = corrected_text
        self.guess = guess
        self.next_step = next_step

class TextOrderError(TextMatchError):

    def __init__(self, message: str) -> None:
        super().__init__(message)
        self.reason = 'marker_order'

class TextAmbiguous(TextMatchError):

    def __init__(self, message: str, count: int) -> None:
        super().__init__(message)
        self.count = count
        self.reason = 'ambiguous'
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
_FUZZY_CAP = 200
_FUZZY_MIN_RATIO = 0.6
_FUZZY_MIN_MARGIN = 0.08

def _ws_free_span(haystack: str, needle: str) -> str | None:
    """Return the single real span in ``haystack`` matching ``needle`` once all
    whitespace (any amount/kind, anywhere) is ignored on both sides, or ``None``
    if there is no such span or more than one – a verified fix, not a guess.
    """
    if not needle:
        return None
    pattern = re.compile('\\s*'.join((re.escape(c) for c in needle)))
    found = list(pattern.finditer(haystack))
    return found[0].group(0) if len(found) == 1 else None

def _fuzzy_candidate(haystack: str, needle: str) -> str | None:
    """Heuristic, unverified guess at ``needle``'s replacement in ``haystack``.

    Compares ``needle`` against every same-line-count window of ``haystack`` and
    returns the sole window that clearly stands out (score above threshold, and
    ahead of the runner-up by a margin) – ``None`` otherwise, so callers never
    surface a guess that is itself ambiguous. Bounded by ``_FUZZY_CAP`` to keep
    cost proportional to the (short) search text, not the file.
    """
    if not 0 < len(needle) <= _FUZZY_CAP or not haystack:
        return None
    h_lines = haystack.split('\n')
    width = needle.count('\n') + 1
    if len(h_lines) < width:
        return None
    scored = sorted(((difflib.SequenceMatcher(None,
                                              needle,
                                              '\n'.join(h_lines[i:i + width])).ratio(),
                      i) for i in range(len(h_lines) - width + 1)),
                    key=lambda t: t[0],
                    reverse=True)
    if not scored or scored[0][0] < _FUZZY_MIN_RATIO:
        return None
    if len(scored) > 1 and scored[1][0] >= scored[0][0] - _FUZZY_MIN_MARGIN:
        return None
    best = '\n'.join(h_lines[scored[0][1]:scored[0][1] + width])
    return best if best != needle else None

def _diagnose_not_found(haystack: str, needle: str, *, guard_rejected: bool, where: str, label: str='Text', position: str | None=None) -> TextNotFound:
    """Build the ``TextNotFound`` to raise once ``needle`` could not be applied,
    distinguishing a rejected-but-present match, a whitespace-only mismatch (fact),
    a likely content change (unverified guess), or a genuine miss (re-read hint).
    """
    if guard_rejected:
        return TextNotFound(
            f'{label} found in {where} but rejected: the edit would merge/split lines or break syntax there.',
            reason='guard_rejected',
            position=position)
    corrected = _ws_free_span(haystack, needle)
    if corrected is not None:
        return TextNotFound(
            f'{label} not found in {where} as given, but matches once whitespace/tabs/newlines are ignored.',
            reason='whitespace_mismatch',
            position=position,
            corrected_text=corrected)
    guess = _fuzzy_candidate(haystack, needle)
    if guess is not None:
        return TextNotFound(f'{label} not found in {where}; its content appears to have changed.',
                            reason='content_changed', position=position, guess=guess)
    return TextNotFound(
        f'{label} not found in {where}.',
        reason='not_found',
        position=position,
        next_step='reread_node')

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
    guard_rejected = False
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
            guard_rejected = True
            continue
        return result
    raise _diagnose_not_found(block, old_text, guard_rejected=guard_rejected, where=where)

def replace_between(block: str, begin_marker: str, end_marker: str, content: str, *, exact: bool, accept: MarksGuard | None=None, max_level: int=3, where: str='block') -> str:
    """Replace the span between (and including) both markers with ``content``.

    Both markers are matched at the same escalating tolerance level; the first
    level whose unique markers yield an ``accept``-approved result wins. Raises
    :class:`TextNotFound`/:class:`TextAmbiguous`/:class:`TextMatchError`.
    """
    start_found = False
    guard_rejected = False
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
            raise TextOrderError('End marker must start after start marker ends.')
        result = block[:sm.start] + _mirror_escaping(block[sm.start:em.end], content) + block[em.end:]
        if accept and (not accept(block[sm.start:sm.end], block[em.start:em.end], result)):
            guard_rejected = True
            continue
        return result
    if not start_found:
        raise _diagnose_not_found(
            block,
            begin_marker,
            guard_rejected=False,
            where=where,
            label='Start marker',
            position='start')
    raise _diagnose_not_found(
        block,
        end_marker,
        guard_rejected=guard_rejected,
        where=where,
        label='End marker',
        position='end')