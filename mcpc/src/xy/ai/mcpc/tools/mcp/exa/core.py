"""Shared building blocks for the two-stage ``exa`` tool family.

Each stage-1 tool (``web_search_exa`` / ``web_fetch_exa``) normalizes Exa's raw
result items, caches the full item (incl. url/text) by id, and returns only a
trimmed overview; the matching stage-2 tool (``*_results``) resolves ids from
that cache back to url/text.
"""
import random
import string
from typing import Any
__all__ = ['normalize_item', 'strip_empty', 'ResultCache', 'search_cache', 'fetch_cache']
_ID_ALPHABET = string.digits + string.ascii_letters
'#: Fields the Exa payload carries but that add no value for our tools.'
_DROPPED_FIELDS = ('published_date', 'score', 'image', 'favicon', 'highlight_scores')

def _random_id() -> str:
    return ''.join(random.choices(_ID_ALPHABET, k=6))

def strip_empty(value: Any) -> Any:
    """Recursively drop ``None``, ``''``, ``[]`` and ``{}`` from *value*."""
    if isinstance(value, dict):
        cleaned = {k: strip_empty(v) for k, v in value.items()}
        return {k: v for k, v in cleaned.items() if v not in (None, '', [], {})}
    if isinstance(value, list):
        return [strip_empty(v) for v in value]
    return value

def normalize_item(raw: dict[str, Any]) -> dict[str, Any]:
    """Normalize one raw Exa result item for caching and display.

    Drops ``_DROPPED_FIELDS``, backfills a missing ``id`` with a random 6-char
    id, and renames ``highlights`` to ``excerpt`` - synthesizing the first and
    last 100 characters of ``text`` when no highlights were returned.
    """
    item = {k: v for k, v in raw.items() if k not in _DROPPED_FIELDS}
    item['id'] = item.get('id') or _random_id()
    excerpt = item.pop('highlights', None) or []
    text = item.get('text') or ''
    if not excerpt and text:
        excerpt = [text[:100], text[-100:]]
    item['excerpt'] = excerpt
    return item

class ResultCache:
    """In-memory store of normalized items, keyed by id."""

    def __init__(self) -> None:
        self._items: dict[str, dict[str, Any]] = {}

    def put(self, item: dict[str, Any]) -> str:
        self._items[item['id']] = item
        return item['id']

    def get_many(self, ids: list[str]) -> list[dict[str, Any]]:
        return [self._items[item_id] for item_id in ids if item_id in self._items]
'#: Cache instances shared between each stage-1 tool and its stage-2 counterpart.'
search_cache = ResultCache()
fetch_cache = ResultCache()