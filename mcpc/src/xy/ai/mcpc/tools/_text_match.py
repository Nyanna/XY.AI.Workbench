"""Shared exact / whitespace-tolerant text search for change and replace-block."""

from __future__ import annotations

import re
from dataclasses import dataclass


@dataclass(frozen=True)
class MatchResult:
    count: int
    start: int = -1
    end: int = -1


def _fuzzy_pattern(needle: str) -> re.Pattern[str]:
    parts = re.split(r"(\s+)", needle)
    pattern = "".join(
        r"\s+" if part.isspace() else re.escape(part)
        for part in parts
        if part != ""
    )
    return re.compile(pattern)


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
