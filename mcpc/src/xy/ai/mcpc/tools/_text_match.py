"""Shared exact / whitespace-tolerant text search for change and replace-block."""
import re
from dataclasses import dataclass

@dataclass(frozen=True)
class MatchResult:
    count: int
    start: int = -1
    end: int = -1

def _fuzzy_pattern(needle: str) -> re.Pattern[str]:
    parts = [p for p in re.split('(\\s+)', needle) if p != '']
    last = len(parts) - 1
    segments: list[str] = []
    for i, part in enumerate(parts):
        interior = part.isspace() and 0 < i < last
        segments.append('\\s+' if interior else re.escape(part))
    return re.compile(''.join(segments))

def find_all(haystack: str, needle: str, *, exact: bool) -> list[MatchResult]:
    """Return all non-overlapping occurrences of ``needle`` in ``haystack``."""
    if exact:
        results: list[MatchResult] = []
        start = 0
        while True:
            idx = haystack.find(needle, start)
            if idx == -1:
                break
            results.append(MatchResult(count=1, start=idx, end=idx + len(needle)))
            start = idx + len(needle)
        return results
    pattern = _fuzzy_pattern(needle)
    return [MatchResult(count=1, start=m.start(), end=m.end()) for m in pattern.finditer(haystack)]

def find(haystack: str, needle: str, *, exact: bool) -> MatchResult:
    if exact:
        count = haystack.count(needle)
        if count != 1:
            return MatchResult(count=count)
        start = haystack.index(needle)
        return MatchResult(count=1, start=start, end=start + len(needle))
    pattern = _fuzzy_pattern(needle)
    matches = list(pattern.finditer(haystack))
    if len(matches) != 1:
        return MatchResult(count=len(matches))
    match = matches[0]
    return MatchResult(count=1, start=match.start(), end=match.end())