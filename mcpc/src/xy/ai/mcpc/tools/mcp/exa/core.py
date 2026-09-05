"""Shared building blocks for the two-stage ``exa`` tool family.
"""
import logging
import random
import string
from typing import Any, Callable
from xy.ai.mcpc.tools.mcp.bridge import McpBridgeError
__all__ = ['normalize_item', 'strip_empty', 'extract_results', 'ResultCache', 'search_cache', 'fetch_cache', 'logger']
'#: Shared logger for the whole ``exa`` tool family.'
logger = logging.getLogger('xy.ai.mcpc.tools.mcp.exa')
_ID_ALPHABET = string.digits + string.ascii_letters
'#: Fields the Exa payload carries but that add no value for our tools.'
_DROPPED_FIELDS = ('published_date', 'score', 'image', 'favicon', 'highlight_scores')
'#: Excerpt caps applied by ``normalize_item``.'
_MAX_EXCERPTS = 10
_MAX_EXCERPT_LENGTH = 100

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

def extract_results(raw: dict[str, Any], remote_tool: str, *, text_parser: 'Callable[[str], list[dict[str, Any]]] | None'=None) -> list[dict[str, Any]]:
    """Pull the ``results`` array out of a raw Exa ``CallToolResult`` payload.

    ``McpBridge`` normally hands back either the remote's ``structuredContent``
    or a dict parsed from a JSON text body. The Exa remote MCP server can
    instead reply with a single human-readable *text* block (no
    ``structuredContent``, not JSON) - in that case ``McpBridge`` falls back to
    ``{"content": <raw text>}``. A bare ``raw.get('results', [])`` would read
    that shape as "zero results" and silently return an empty, seemingly
    successful response, discarding the actual page content and hiding the
    real problem from both logs and the caller.

    When *text_parser* is given and this fallback shape is hit, it is used to
    parse Exa's markdown-ish plain-text format into result dicts instead of
    immediately failing. If parsing yields nothing (or no *text_parser* was
    given), this logs the full context (including a preview of what the
    remote actually sent) and raises - so a genuine shape mismatch is never
    silently mistaken for "no results found".

    Raises:
        McpBridgeError: if ``raw`` has no usable ``results`` array and the
            text fallback (if any) did not recover any items.
    """
    results = raw.get('results')
    if results is None:
        content = raw.get('content')
        if text_parser is not None and isinstance(content, str) and content.strip():
            parsed = text_parser(content)
            if parsed:
                logger.info(
                    "Exa '%s': remote sent unstructured text instead of structured data; recovered %d item(s) via markdown fallback parsing.",
                    remote_tool,
                    len(parsed))
                return parsed
            logger.warning(
                "Exa '%s': remote sent unstructured text but the markdown fallback parser found no items in it (keys=%s).",
                remote_tool,
                sorted(
                    raw.keys()))
        preview = str(content if content is not None else raw)[:300]
        logger.error(
            "Exa '%s': response has no 'results' field (keys=%s); this usually means the remote server returned unstructured text instead of structured data. Preview: %r",
            remote_tool,
            sorted(
                raw.keys()),
            preview)
        raise McpBridgeError(
            f"Exa '{remote_tool}' returned an unexpected response shape (no 'results' field; keys={
                sorted(
                    raw.keys())}). The remote server likely sent unstructured text instead of structured data. Preview: {preview}")
    if not isinstance(results, list):
        logger.error("Exa '%s': 'results' field is not a list (got %s)", remote_tool, type(results).__name__)
        raise McpBridgeError(
            f"Exa '{remote_tool}' returned a malformed 'results' field (expected list, got {
                type(results).__name__}).")
    return results

def normalize_item(raw: dict[str, Any]) -> dict[str, Any]:
    """Normalize one raw Exa result item for caching and display.

    Drops ``_DROPPED_FIELDS``, backfills a missing ``id`` with a random 6-char
    id, and renames ``highlights`` to ``excerpt`` - synthesizing the first and
    last 100 characters of ``text`` when no highlights were returned. The
    excerpt is capped to ``_MAX_EXCERPTS`` entries of at most
    ``_MAX_EXCERPT_LENGTH`` characters each.
    """
    if not isinstance(raw, dict):
        logger.warning('Exa result item is not a dict (got %s), skipping its fields: %r', type(raw).__name__, raw)
        raw = {}
    item = {k: v for k, v in raw.items() if k not in _DROPPED_FIELDS}
    item['id'] = item.get('id') or _random_id()
    excerpt = item.pop('highlights', None) or []
    text = item.get('text') or ''
    if not excerpt and text:
        excerpt = [text[:_MAX_EXCERPT_LENGTH], text[-_MAX_EXCERPT_LENGTH:]]
    item['excerpt'] = [e[:_MAX_EXCERPT_LENGTH] for e in excerpt[:_MAX_EXCERPTS]]
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