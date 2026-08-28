"""Agent-facing OpenAlex tools.

Three tools sit on top of the :mod:`xy.ai.mcpc.openalex` interface package and
apply standard assumptions so an AI agent can use OpenAlex without knowing the
raw API:

* ``openalex_search``          – keyword / boolean full-text search.
* ``openalex_semantic_search`` – AI (embedding) search by meaning.
* ``openalex_work``            – fetch a single work by id / DOI.

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

import logging
from typing import Any

from xy.ai.mcpc.config import ServerConfig
from xy.ai.mcpc.openalex import (
    DEFAULT_SEARCH_PRESET,
    DEFAULT_WORK_PRESET,
    OpenAlexAPIError,
    OpenAlexClient,
    OpenAlexError,
    project_results,
    resolve_select,
)
from xy.ai.mcpc.openalex.client import ENTITIES
from xy.ai.mcpc.openalex.presets import WORK_PRESET_NAMES
from xy.ai.mcpc.tools.registry import ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.utils.text_sanitize import sanitize_value

__all__ = ["register_openalex_tools"]
#: Hard caps that mirror the OpenAlex API limits.
_MAX_PER_PAGE = 50 # was 200
_MAX_SEMANTIC_RESULTS = 50
_DEFAULT_SEARCH_LIMIT = 25
_DEFAULT_SEMANTIC_LIMIT = 10

_ENTITY_NAMES = sorted(ENTITIES)
_WORK_PRESETS = list(WORK_PRESET_NAMES)

logger = logging.getLogger("xy.ai.mcpc.tools.openalex")


def _build_client(config: ServerConfig | None) -> OpenAlexClient:
    """Build the OpenAlex client once, at registration time.
    """
    config = config or ServerConfig()
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
    if isinstance(exc, OpenAlexAPIError) and exc.status is not None:
        message = f"{message} (status {exc.status})"
    logger.error("OpenAlex request failed: %s", message, exc_info=exc)
    return ToolResult(content=[text_content(message)], is_error=True)


def _ok_result(structured: dict[str, Any]) -> ToolResult:
    # OpenAlex occasionally returns fields (titles, abstracts, ...) that
    # contain raw non-printable control characters; strip them so downstream
    # consumers (notably YAML block-scalar rendering) never choke on them.
    return ToolResult(structured_content=sanitize_value(structured))


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


def _register_search(registry: ToolRegistry, client: OpenAlexClient, functions: "FunctionRegistry | None" = None) -> None:
    def openalex_search(
        query: str,
        entity: str = "works",
        exact: bool = False,
        fields: str | None = None,
        filter: str | None = None,
        sort: str | None = None,
        limit: int | None = None,
    ) -> dict[str, Any]:
        """Keyword/boolean full-text search across OpenAlex scholarly entities.

        Args:
            query: Full-text query (boolean AND/OR/NOT, quoted phrases, proximity).
            entity: Entity type to search (default: works).
            exact: Use exact (unstemmed) search; required for wildcards.
            fields: Field preset controlling how much of each record is returned.
            filter: Optional OpenAlex filter string.
            sort: Optional sort override; defaults to relevance.
            limit: Max results from the first page.

        Returns:
            dict with ``count``, ``returned`` and ``results``.

        Raises:
            OpenAlexError: If the OpenAlex API request fails.
        """
        preset = fields or DEFAULT_SEARCH_PRESET
        resolved_limit = _clamp(limit, _DEFAULT_SEARCH_LIMIT, _MAX_PER_PAGE)
        select = resolve_select(preset, entity)
        data = client.search_works(
            query,
            exact=exact,
            filters=filter,
            sort=sort,
            select=select,
            per_page=resolved_limit,
            page=1,
        ) if entity == "works" else client.list_entities(
            entity,
            search_exact=query if exact else None,
            search=None if exact else query,
            filters=filter,
            sort=sort,
            select=select,
            per_page=resolved_limit,
            page=1,
        )
        return _summarise_list(data)

    @registry.tool(
        "openalex_search",
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
    def handler(ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            structured = openalex_search(
                query=args["query"],
                entity=args.get("entity", "works"),
                exact=bool(args.get("exact", False)),
                fields=args.get("fields"),
                filter=args.get("filter"),
                sort=args.get("sort"),
                limit=args.get("limit"),
            )
        except OpenAlexError as exc:
            return _error_result(exc)
        return _ok_result(structured)

    functions.register(openalex_search)


def _register_semantic_search(registry: ToolRegistry, client: OpenAlexClient, functions: "FunctionRegistry | None" = None) -> None:
    def openalex_semantic_search(
        query: str,
        fields: str | None = None,
        filter: str | None = None,
        limit: int | None = None,
    ) -> dict[str, Any]:
        """AI-powered semantic search over OpenAlex works.

        Args:
            query: Natural-language description of what to look for.
            fields: Field preset for each work (default: core).
            filter: Optional OpenAlex filter string (no cited_by_count/country_code).
            limit: Max results (1-50, default 10).

        Returns:
            dict with ``count``, ``returned`` and ``results``.

        Raises:
            OpenAlexError: If the OpenAlex API request fails.
        """
        preset = fields or DEFAULT_SEARCH_PRESET
        resolved_limit = _clamp(limit, _DEFAULT_SEMANTIC_LIMIT, _MAX_SEMANTIC_RESULTS)
        select = resolve_select(preset, "works")
        data = client.semantic_search_works(
            query,
            filters=filter,
            select=select,
            per_page=resolved_limit,
            page=1,
        )
        return _summarise_list(data)

    @registry.tool(
        "openalex_semantic_search",
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
    def handler(ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            structured = openalex_semantic_search(
                query=args["query"],
                fields=args.get("fields"),
                filter=args.get("filter"),
                limit=args.get("limit"),
            )
        except OpenAlexError as exc:
            return _error_result(exc)
        return _ok_result(structured)

    functions.register(openalex_semantic_search)


def _register_work(registry: ToolRegistry, client: OpenAlexClient, functions: "FunctionRegistry | None" = None) -> None:
    def openalex_work(id: str, fields: str | None = None) -> dict[str, Any]:
        """Fetch a single OpenAlex work by identifier.

        Args:
            id: OpenAlex id/URL, DOI (bare or URL), or namespaced id (pmid:, mag:, ...).
            fields: Field preset (default: full).

        Returns:
            dict with the ``work`` record.

        Raises:
            OpenAlexError: If the OpenAlex API request fails.
        """
        preset = fields or DEFAULT_WORK_PRESET
        select = resolve_select(preset, "works")
        data = client.get_work(id, select=select)
        work = project_results([data])[0]
        return {"work": work}

    @registry.tool(
        "openalex_work",
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
    def handler(ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            structured = openalex_work(id=args["id"], fields=args.get("fields"))
        except OpenAlexError as exc:
            return _error_result(exc)
        return _ok_result(structured)

    functions.register(openalex_work)


# --------------------------------------------------------------------- register
def register_openalex_tools(registry: ToolRegistry, environment: "AppEnvironment | None" = None) -> None:
    """Register the three OpenAlex tools onto *registry*."""
    client = _build_client(environment.config if environment is not None else None)
    functions = environment.functions if environment is not None else None
    _register_search(registry, client, functions)
    _register_semantic_search(registry, client, functions)
    _register_work(registry, client, functions)
