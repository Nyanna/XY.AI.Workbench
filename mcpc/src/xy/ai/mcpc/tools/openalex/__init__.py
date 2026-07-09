"""Agent-facing OpenAlex tools.

Three tools sit on top of the :mod:`xy.ai.mcpc.openalex` interface package and
apply standard assumptions so an AI agent can use OpenAlex without knowing the
raw API:

* ``openalex-search``          – keyword / boolean full-text search.
* ``openalex-semantic-search`` – AI (embedding) search by meaning.
* ``openalex-work``            – fetch a single work by id / DOI.

Shared conventions
------------------
* **First page only.** Results are always page 1; ``limit`` controls how many
  hits come back (paging deeper is intentionally not exposed).
* **Field presets.** Instead of raw ``select`` fields, callers pick a semantic
  preset (see :mod:`xy.ai.mcpc.openalex.presets`).
* **Readable abstracts.** OpenAlex's ``abstract_inverted_index`` is rebuilt into
  a plain-text ``abstract`` field.
* **Authentication.** The API key (``MCPC_OPENALEX_KEY``) and optional
  ``mailto`` come from the server config and are handled by the client.
"""

from __future__ import annotations

import json
from typing import Any

from ...config import ServerConfig
from ...openalex import (
    DEFAULT_SEARCH_PRESET,
    DEFAULT_WORK_PRESET,
    OpenAlexAPIError,
    OpenAlexClient,
    OpenAlexError,
    project_results,
    resolve_select,
)
from ...openalex.client import ENTITIES
from ...openalex.presets import WORK_PRESET_NAMES
from ...registry import ToolContext, ToolRegistry, ToolResult, text_content

#: Hard caps that mirror the OpenAlex API limits.
_MAX_PER_PAGE = 200
_MAX_SEMANTIC_RESULTS = 50
_DEFAULT_SEARCH_LIMIT = 25
_DEFAULT_SEMANTIC_LIMIT = 10

_ENTITY_NAMES = sorted(ENTITIES)
_WORK_PRESETS = list(WORK_PRESET_NAMES)


# --------------------------------------------------------------------- helpers
def _client(ctx: ToolContext) -> OpenAlexClient:
    config = ctx.services.config if ctx.services is not None else ServerConfig()
    return OpenAlexClient(
        api_key=config.openalex_api_key,
        base_url=config.openalex_base_url,
        mailto=config.openalex_mailto,
    )


def _clamp(value: Any, default: int, maximum: int) -> int:
    try:
        number = int(value)
    except (TypeError, ValueError):
        return default
    return max(1, min(number, maximum))


def _error_result(exc: Exception) -> ToolResult:
    message = str(exc)
    structured: dict[str, Any] = {"error": message}
    if isinstance(exc, OpenAlexAPIError) and exc.status is not None:
        structured["status"] = exc.status
    return ToolResult(
        content=[text_content(message)],
        structured_content=structured,
        is_error=True,
    )


def _ok_result(structured: dict[str, Any]) -> ToolResult:
    text = json.dumps(structured, ensure_ascii=False, indent=2, default=str)
    return ToolResult(content=[text_content(text)], structured_content=structured)


def _summarise_list(data: dict[str, Any]) -> dict[str, Any]:
    meta = data.get("meta") or {}
    results = project_results(data.get("results") or [])
    structured: dict[str, Any] = {
        "count": meta.get("count"),
        "returned": len(results),
        "page": meta.get("page"),
        "per_page": meta.get("per_page"),
        "results": results,
    }
    if data.get("group_by"):
        structured["group_by"] = data["group_by"]
    return structured


# ----------------------------------------------------------------- tool: search
def _register_search(registry: ToolRegistry) -> None:
    @registry.tool(
        "openalex-search",
        title="OpenAlex search",
        description=(
            "Keyword and boolean full-text search across OpenAlex scholarly "
            "entities (works by default). Searches titles, abstracts and "
            "full text for works; names for authors, sources and institutions.\n\n"
            "Query syntax: use uppercase AND / OR / NOT and double-quoted "
            'phrases, e.g. (\"machine learning\" OR \"deep learning\") NOT survey. '
            "Set exact=true for unstemmed matching and wildcards (machin*). "
            "Results are sorted by relevance and limited to the first page."
        ),
        input_schema={
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": (
                        "Full-text query. Supports boolean AND/OR/NOT "
                        "(uppercase), quoted phrases and proximity (\"a b\"~5)."
                    ),
                },
                "entity": {
                    "type": "string",
                    "enum": _ENTITY_NAMES,
                    "description": "Entity type to search (default: works).",
                },
                "exact": {
                    "type": "boolean",
                    "description": (
                        "Use exact (unstemmed) search; required for wildcards "
                        "like machin*. Default: false."
                    ),
                },
                "fields": {
                    "type": "string",
                    "enum": _WORK_PRESETS,
                    "description": (
                        "Field preset controlling how much of each record is "
                        "returned (works only). Default: core. Presets: "
                        "minimal, core, bibliographic, authorship, access, "
                        "metrics, topics, abstract, references, full."
                    ),
                },
                "filter": {
                    "type": "string",
                    "description": (
                        "Optional OpenAlex filter string applied alongside the "
                        "search, e.g. 'publication_year:>2020,is_oa:true'. "
                        "Comma-separated key:value pairs."
                    ),
                },
                "sort": {
                    "type": "string",
                    "description": (
                        "Optional sort override, e.g. 'cited_by_count:desc' or "
                        "'publication_date:desc'. Defaults to relevance."
                    ),
                },
                "limit": {
                    "type": "integer",
                    "description": (
                        f"Max results from the first page (1-{_MAX_PER_PAGE}, "
                        f"default {_DEFAULT_SEARCH_LIMIT})."
                    ),
                    "minimum": 1,
                    "maximum": _MAX_PER_PAGE,
                },
            },
            "required": ["query"],
        },
        output_schema={
            "type": "object",
            "properties": {
                "count": {"type": "integer"},
                "returned": {"type": "integer"},
                "results": {"type": "array", "items": {"type": "object"}},
            },
        },
        annotations={"readOnlyHint": True, "openWorldHint": True},
    )
    def openalex_search(ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        query = args["query"]
        entity = args.get("entity", "works")
        exact = bool(args.get("exact", False))
        preset = args.get("fields", DEFAULT_SEARCH_PRESET)
        filters = args.get("filter")
        sort = args.get("sort")
        limit = _clamp(args.get("limit"), _DEFAULT_SEARCH_LIMIT, _MAX_PER_PAGE)

        select = resolve_select(preset, entity)
        try:
            data = _client(ctx).search_works(
                query,
                exact=exact,
                filters=filters,
                sort=sort,
                select=select,
                per_page=limit,
                page=1,
            ) if entity == "works" else _client(ctx).list_entities(
                entity,
                search_exact=query if exact else None,
                search=None if exact else query,
                filters=filters,
                sort=sort,
                select=select,
                per_page=limit,
                page=1,
            )
        except OpenAlexError as exc:
            return _error_result(exc)
        return _ok_result(_summarise_list(data))


# -------------------------------------------------------- tool: semantic search
def _register_semantic_search(registry: ToolRegistry) -> None:
    @registry.tool(
        "openalex-semantic-search",
        title="OpenAlex semantic search",
        description=(
            "AI-powered semantic search over OpenAlex works: finds works by "
            "meaning using embeddings, even when the wording differs. Best for "
            "paragraph-length input such as an abstract, a research question or "
            "a grant aim (up to ~2000 characters).\n\n"
            "Supports most filters, but NOT cited_by_count or country_code "
            "filters. Returns at most 50 works from the first page, ranked by "
            "semantic similarity."
        ),
        input_schema={
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": (
                        "Natural-language description of what you are looking "
                        "for. Longer, richer input yields better matches "
                        "(truncated at 2000 characters)."
                    ),
                },
                "fields": {
                    "type": "string",
                    "enum": _WORK_PRESETS,
                    "description": (
                        "Field preset for each work (default: core)."
                    ),
                },
                "filter": {
                    "type": "string",
                    "description": (
                        "Optional OpenAlex filter string, e.g. "
                        "'publication_year:>2020,is_oa:true'. Note: "
                        "cited_by_count and country_code filters are not "
                        "supported by semantic search."
                    ),
                },
                "limit": {
                    "type": "integer",
                    "description": (
                        f"Max results (1-{_MAX_SEMANTIC_RESULTS}, default "
                        f"{_DEFAULT_SEMANTIC_LIMIT})."
                    ),
                    "minimum": 1,
                    "maximum": _MAX_SEMANTIC_RESULTS,
                },
            },
            "required": ["query"],
        },
        output_schema={
            "type": "object",
            "properties": {
                "count": {"type": "integer"},
                "returned": {"type": "integer"},
                "results": {"type": "array", "items": {"type": "object"}},
            },
        },
        annotations={"readOnlyHint": True, "openWorldHint": True},
    )
    def openalex_semantic_search(ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        query = args["query"]
        preset = args.get("fields", DEFAULT_SEARCH_PRESET)
        filters = args.get("filter")
        limit = _clamp(
            args.get("limit"), _DEFAULT_SEMANTIC_LIMIT, _MAX_SEMANTIC_RESULTS
        )

        select = resolve_select(preset, "works")
        try:
            data = _client(ctx).semantic_search_works(
                query,
                filters=filters,
                select=select,
                per_page=limit,
                page=1,
            )
        except OpenAlexError as exc:
            return _error_result(exc)
        return _ok_result(_summarise_list(data))


# ------------------------------------------------------------- tool: single work
def _register_work(registry: ToolRegistry) -> None:
    @registry.tool(
        "openalex-work",
        title="OpenAlex work",
        description=(
            "Fetch a single OpenAlex work by identifier. Accepts an OpenAlex id "
            "(W2741809807), an OpenAlex URL, a DOI (10.7717/peerj.4375 or a "
            "doi.org URL) or a namespaced id such as pmid:14907713. Returns the "
            "full record by default, with the abstract reconstructed to plain "
            "text."
        ),
        input_schema={
            "type": "object",
            "properties": {
                "id": {
                    "type": "string",
                    "description": (
                        "Work identifier: OpenAlex id/URL, DOI (bare or URL), "
                        "or namespaced id (pmid:, mag:, ...)."
                    ),
                },
                "fields": {
                    "type": "string",
                    "enum": _WORK_PRESETS,
                    "description": (
                        "Field preset (default: full). Use a narrower preset "
                        "such as bibliographic or abstract to reduce size."
                    ),
                },
            },
            "required": ["id"],
        },
        output_schema={
            "type": "object",
            "properties": {"work": {"type": "object"}},
        },
        annotations={"readOnlyHint": True, "openWorldHint": True},
    )
    def openalex_work(ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        work_id = args["id"]
        preset = args.get("fields", DEFAULT_WORK_PRESET)

        select = resolve_select(preset, "works")
        try:
            data = _client(ctx).get_work(work_id, select=select)
        except OpenAlexError as exc:
            return _error_result(exc)
        work = project_results([data])[0]
        return _ok_result({"work": work})


# --------------------------------------------------------------------- register
def register_openalex_tools(registry: ToolRegistry) -> None:
    """Register the three OpenAlex tools onto *registry*."""
    _register_search(registry)
    _register_semantic_search(registry)
    _register_work(registry)


__all__ = ["register_openalex_tools"]
