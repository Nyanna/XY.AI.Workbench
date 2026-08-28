Die Tools müssen vom Decorator auf die ABC Registrierung umgestellt werden, Beispiel: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_block/__init__.py`.
Die registrierten Functions dürfen keine Locals sein, sondern müssen im Module Root direkt ansprechbar sein.

Tools:
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/__init__.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/github.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/context7.py`
SystemInit:  id=8d88764e-0067-4fa7-8e42-fe59a9580496, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: ded6-1
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_block/__init__.py
  reason: Understand the target ABC registration pattern to migrate other tools to it.
```
/answer ded6-1 allow
Control Request:
```yaml
id: ded6-2
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/__init__.py
  reason: Read current decorator-based implementation to convert it to the ABC registration pattern.
```
/answer ded6-2 allow
Control Request:
```yaml
id: ded6-3
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/github.py
  reason: Read current decorator-based implementation to convert it to the ABC registration pattern.
```
/answer ded6-3 allow
Control Request:
```yaml
id: ded6-4
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa.py
  reason: Read current decorator-based implementation to convert it to the ABC registration pattern.
```
/answer ded6-4 allow
Control Request:
```yaml
id: ded6-5
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/context7.py
  reason: Read current decorator-based implementation to convert it to the ABC registration pattern.
```
/answer ded6-5 allow
ReasoningToken: 25
Control Request:
```yaml
id: ded6-6
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/registry.py
  reason: Understand ToolDefinition/ToolRegistry API including the .tool decorator to correctly migrate decorator-based registrations to ABC class registration.
```
/answer ded6-6 deny nicht notwendig
Control Request:
```yaml
id: ded6-7
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/function_registry.py
  reason: Understand FunctionRegistry.register API for module-level function registration.
```
/answer ded6-7 deny nicht notwendig
Control Request:
```yaml
id: ded6-8
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_context.py
  reason: Understand AppEnvironment/ToolContext to correctly wire ABC-based tools with config/functions.
```
/answer ded6-8 deny nicht notwendig
ReasoningToken: 120
Control Request:
```yaml
id: ded6-9
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/__init__.py
  mode: replace
  reason: Migrate OpenAlex tools from @registry.tool decorator to ABC-based ToolDefinition registration; move formerly-local functions to module root using a module-level client accessor.
  content: |
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
    from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
    from xy.ai.mcpc.utils.text_sanitize import sanitize_value

    __all__ = [
        "openalex_search",
        "openalex_semantic_search",
        "openalex_work",
        "OpenalexSearchTool",
        "OpenalexSemanticSearchTool",
        "OpenalexWorkTool",
        "register_openalex_tools",
    ]
    #: Hard caps that mirror the OpenAlex API limits.
    _MAX_PER_PAGE = 50 # was 200
    _MAX_SEMANTIC_RESULTS = 50
    _DEFAULT_SEARCH_LIMIT = 25
    _DEFAULT_SEMANTIC_LIMIT = 10

    _ENTITY_NAMES = sorted(ENTITIES)
    _WORK_PRESETS = list(WORK_PRESET_NAMES)

    logger = logging.getLogger("xy.ai.mcpc.tools.openalex")

    #: Module-level client, (re)built by :func:`register_openalex_tools`.
    _client: OpenAlexClient | None = None


    def _build_client(config: ServerConfig | None) -> OpenAlexClient:
        """Build the OpenAlex client once, at registration time.
        """
        config = config or ServerConfig()
        return OpenAlexClient(
            api_key=config.openalex_api_key,
            base_url=config.openalex_base_url,
            mailto=config.openalex_mailto,
        )


    def _get_client() -> OpenAlexClient:
        """Return the module-level OpenAlex client, building a default one on first use."""
        global _client
        if _client is None:
            _client = _build_client(None)
        return _client


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
        client = _get_client()
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
        client = _get_client()
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
        client = _get_client()
        preset = fields or DEFAULT_WORK_PRESET
        select = resolve_select(preset, "works")
        data = client.get_work(id, select=select)
        work = project_results([data])[0]
        return {"work": work}


    _RO: dict[str, Any] = {"readOnlyHint": True, "openWorldHint": True}

    _LIST_OUTPUT_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "count": {"type": "integer"},
            "returned": {"type": "integer"},
            "results": {"type": "array", "items": {"type": "object"}},
        },
    }

    _SEARCH_DESCRIPTION = (
        "Keyword and boolean full-text search across OpenAlex scholarly "
        "entities (works by default). Searches titles, abstracts and "
        "full text for works; names for authors, sources and institutions.\n\n"
        "Query syntax: use uppercase AND / OR / NOT and double-quoted "
        'phrases, e.g. (\"machine learning\" OR \"deep learning\") NOT survey. '
        "Set exact=true for unstemmed matching and wildcards (machin*). "
        "Results are sorted by relevance and limited to the first page."
    )
    _SEARCH_INPUT_SCHEMA: dict[str, Any] = {
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
    }

    _SEMANTIC_SEARCH_DESCRIPTION = (
        "AI-powered semantic search over OpenAlex works: finds works by "
        "meaning using embeddings, even when the wording differs. Best for "
        "paragraph-length input such as an abstract, a research question or "
        "a grant aim (up to ~2000 characters).\n\n"
        "Supports most filters, but NOT cited_by_count or country_code "
        "filters. Returns at most 50 works from the first page, ranked by "
        "semantic similarity."
    )
    _SEMANTIC_INPUT_SCHEMA: dict[str, Any] = {
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
                "description": "Field preset for each work (default: core).",
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
    }

    _WORK_DESCRIPTION = (
        "Fetch a single OpenAlex work by identifier. Accepts an OpenAlex id "
        "(W2741809807), an OpenAlex URL, a DOI (10.7717/peerj.4375 or a "
        "doi.org URL) or a namespaced id such as pmid:14907713. Returns the "
        "full record by default, with the abstract reconstructed to plain "
        "text."
    )
    _WORK_INPUT_SCHEMA: dict[str, Any] = {
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
    }
    _WORK_OUTPUT_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {"work": {"type": "object"}},
    }


    class OpenalexSearchTool(ToolDefinition):
        name = "openalex_search"
        title = "OpenAlex search"
        description = _SEARCH_DESCRIPTION
        input_schema = _SEARCH_INPUT_SCHEMA
        output_schema = _LIST_OUTPUT_SCHEMA
        annotations = _RO

        def handle(self, ctx: ToolContext) -> ToolResult:
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


    class OpenalexSemanticSearchTool(ToolDefinition):
        name = "openalex_semantic_search"
        title = "OpenAlex semantic search"
        description = _SEMANTIC_SEARCH_DESCRIPTION
        input_schema = _SEMANTIC_INPUT_SCHEMA
        output_schema = _LIST_OUTPUT_SCHEMA
        annotations = _RO

        def handle(self, ctx: ToolContext) -> ToolResult:
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


    class OpenalexWorkTool(ToolDefinition):
        name = "openalex_work"
        title = "OpenAlex work"
        description = _WORK_DESCRIPTION
        input_schema = _WORK_INPUT_SCHEMA
        output_schema = _WORK_OUTPUT_SCHEMA
        annotations = _RO

        def handle(self, ctx: ToolContext) -> ToolResult:
            args = ctx.arguments
            try:
                structured = openalex_work(id=args["id"], fields=args.get("fields"))
            except OpenAlexError as exc:
                return _error_result(exc)
            return _ok_result(structured)


    def register_openalex_tools(registry: ToolRegistry, environment: "AppEnvironment | None" = None) -> None:
        """Register the three OpenAlex tools onto *registry*."""
        global _client
        _client = _build_client(environment.config if environment is not None else None)
        registry.register(OpenalexSearchTool())
        registry.register(OpenalexSemanticSearchTool())
        registry.register(OpenalexWorkTool())
        functions = environment.functions if environment is not None else None
        if functions is not None:
            functions.register(openalex_search)
            functions.register(openalex_semantic_search)
            functions.register(openalex_work)
```
/answer ded6-9 allow
Control Request:
```yaml
id: ded6-a
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa.py
  mode: replace
  reason: "Move core Exa functions and ToolDefinition subclasses to module root instead of closures/locals inside register_exa_tools, using a module-level bridge accessor."
  content: |
    """Exa bridge – forwards ``web_search_exa`` and ``web_fetch_exa`` to Exa's MCP.

    Exa is reached through its remote MCP server; the API key is taken from the
    server configuration.  MCPC advertises its own descriptions and input schemas.
    """

    from __future__ import annotations

    from typing import Any

    from xy.ai.mcpc.server.json_codec import JsonCodec
    from xy.ai.mcpc.config import ServerConfig
    from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
    from xy.ai.mcpc.tools.mcp.bridge import McpBridge, McpBridgeError, compact
    from xy.ai.mcpc.tools.mcp.client import McpClient, McpClientError

    __all__ = [
        "ExaBridge",
        "web_search_exa",
        "web_fetch_exa",
        "WebSearchExaTool",
        "WebFetchExaTool",
        "register_exa_tools",
    ]

    _WEB_SEARCH_DESCRIPTION = (
        "Search the web for any topic and get clean, ready-to-use content.\n\n"
        "Best for: Finding current information, facts, or "
        "answering questions about any topic.\n"
        "Returns: Clean text content from top search results."
    )
    _WEB_SEARCH_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "query": {
                "type": "string",
                "description": (
                    "Natural language search query. Should be a semantically rich "
                    "description of the ideal page."
                ),
            },
            "numResults": {
                "type": "integer",
                "description": "Number of search results to return (default: 10).",
                "minimum": 1,
            },
        },
        "required": ["query"],
    }
    _SEARCH_OUTPUT_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "content": {
                "type": "string",
                "description": "Clean text content from the top search results.",
            },
        },
        "required": ["content"],
    }

    _WEB_FETCH_DESCRIPTION = (
        "Read a webpage's full content as clean markdown. Use to read any URL.\n\n"
        "Best for: Extracting full content from known URLs. Batch multiple URLs in "
        "one call.\n"
        "Returns: Clean text content and metadata from the page(s)."
    )
    _WEB_FETCH_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "urls": {
                "type": "array",
                "items": {"type": "string"},
                "description": "URLs to read. Batch multiple URLs in one call.",
            },
            "maxCharacters": {
                "type": "integer",
                "description": "Maximum characters to extract per page (default: 3000)",
                "minimum": 1,
            },
        },
        "required": ["urls"],
    }
    _FETCH_OUTPUT_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "content": {
                "type": "string",
                "description": "Clean text content extracted from the requested page(s).",
            },
        },
        "required": ["content"],
    }

    _RO: dict[str, Any] = {"readOnlyHint": True, "openWorldHint": True}


    def _coerce_urls(urls: list[str] | str) -> list[str]:
        """Accept a single URL or a JSON-encoded list for ``urls`` leniently."""
        if isinstance(urls, str):
            # Accept a JSON-encoded list carried as a string; a plain URL (or any
            # non-list) is wrapped as a single-element list.
            parsed = JsonCodec.try_decode(urls)
            return parsed if isinstance(parsed, list) else [urls]
        return urls


    class ExaBridge(McpBridge):
        """Bridge to the Exa remote MCP server."""

        def build_client(self, config: ServerConfig) -> McpClient:
            api_key = config.exa_api_key
            if not api_key:
                raise McpClientError(
                    "Exa API key is not configured (set MCPC_EXA_API_KEY / EXA_API_KEY)."
                )
            return McpClient(config.exa_mcp_url, headers={"x-api-key": api_key})


    #: Module-level bridge, built by :func:`register_exa_tools`.
    _bridge: ExaBridge | None = None


    def _get_bridge() -> ExaBridge:
        """Return the module-level Exa bridge configured by :func:`register_exa_tools`."""
        if _bridge is None:
            raise McpBridgeError("Exa tools used before register_exa_tools() was called.")
        return _bridge


    def web_search_exa(query: str, numResults: int | None = None) -> dict:
        """Search the web for any topic and get clean, ready-to-use content.

        Best for: Finding current information, facts, or answering questions
        about any topic.

        Args:
            query: Natural language search query; should be a semantically
                rich description of the ideal page.
            numResults: Number of search results to return (default: 10).

        Returns:
            Clean text content from the top results.

        Raises:
            McpBridgeError: if the Exa call fails.
        """
        return _get_bridge().call("web_search_exa", compact(query=query, numResults=numResults))


    def web_fetch_exa(urls: list[str] | str, maxCharacters: int | None = None) -> dict:
        """Read a webpage's full content as clean markdown.

        Best for: Extracting full content from known URLs. Batch multiple
        URLs in one call.

        Args:
            urls: URL(s) to read; a single URL or a list of URLs.
            maxCharacters: Maximum characters to extract per page (default: 3000).

        Returns:
            Clean text content and metadata from the page(s).

        Raises:
            McpBridgeError: if the Exa call fails.
        """
        arguments = compact(urls=_coerce_urls(urls), maxCharacters=maxCharacters)
        return _get_bridge().call("web_fetch_exa", arguments)


    class WebSearchExaTool(ToolDefinition):
        name = "web_search_exa"
        title = "Exa web search"
        description = _WEB_SEARCH_DESCRIPTION
        input_schema = _WEB_SEARCH_SCHEMA
        output_schema = _SEARCH_OUTPUT_SCHEMA
        annotations = _RO

        def handle(self, ctx: ToolContext) -> ToolResult:
            args = ctx.arguments
            try:
                result = web_search_exa(query=args["query"], numResults=args.get("numResults"))
            except McpBridgeError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content=result)


    class WebFetchExaTool(ToolDefinition):
        name = "web_fetch_exa"
        title = "Exa web fetch"
        description = _WEB_FETCH_DESCRIPTION
        input_schema = _WEB_FETCH_SCHEMA
        output_schema = _FETCH_OUTPUT_SCHEMA
        annotations = _RO

        def handle(self, ctx: ToolContext) -> ToolResult:
            args = ctx.arguments
            try:
                result = web_fetch_exa(urls=args["urls"], maxCharacters=args.get("maxCharacters"))
            except McpBridgeError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content=result)


    def register_exa_tools(
        registry: ToolRegistry,
        environment: AppEnvironment,
    ) -> None:
        """Register the Exa-backed ``web_search_exa`` and ``web_fetch_exa`` tools."""
        global _bridge
        _bridge = ExaBridge(environment.config)
        registry.register(WebSearchExaTool())
        registry.register(WebFetchExaTool())
        environment.functions.register(web_search_exa)
        environment.functions.register(web_fetch_exa)
```
/answer ded6-a allow
Control Request:
```yaml
id: ded6-b
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/context7.py
  mode: replace
  reason: "Move core Context7 functions and ToolDefinition subclasses to module root instead of closures/locals inside register_context7_tools, using a module-level bridge accessor."
  content: |
    """Context7 bridge – library documentation tools backed by the Context7 MCP server.

    Exposes two tools:
      context7_libraries      →  resolveLibraryId
      context7_documentation  →  queryDocs
    """

    from __future__ import annotations

    from typing import Any

    from xy.ai.mcpc.config import ServerConfig
    from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
    from xy.ai.mcpc.tools.mcp.bridge import McpBridge, McpBridgeError, compact
    from xy.ai.mcpc.tools.mcp.client import McpClient

    __all__ = [
        "Context7Bridge",
        "context7_libraries",
        "context7_documentation",
        "Context7LibrariesTool",
        "Context7DocumentationTool",
        "register_context7_tools",
    ]

    _RESOLVE_DESCRIPTION = (
        "Search Context7 for a library and return its canonical library ID.\n\n"
        "Best for: Resolving a library name to the ID needed by context7_documentation.\n"
        "Returns: Ranked list of matching libraries with ID, title, description, "
        "snippet count, reputation, benchmark score, and available versions."
    )
    _RESOLVE_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "libraryName": {
                "type": "string",
                "description": "Library name to search for (e.g. 'react', 'next.js', 'vue').",
            },
            "query": {
                "type": "string",
                "description": (
                    "User's original question or task – used for relevance ranking "
                    "(e.g. 'How to manage state with hooks')."
                ),
            },
        },
        "required": ["libraryName", "query"],
    }
    _RESOLVE_OUTPUT: dict[str, Any] = {
        "type": "object",
        "properties": {
            "content": {
                "type": "string",
                "description": (
                    "Ranked list of matching libraries. Each entry contains "
                    "Context7-compatible library ID, title, description, code snippet count, "
                    "source reputation, benchmark score, and available versions."
                ),
            },
        },
        "required": ["content"],
    }

    _QUERY_DOCS_DESCRIPTION = (
        "Fetch documentation and code examples for a library from Context7.\n\n"
        "Best for: Retrieving accurate API docs, usage examples, and configuration guides "
        "for any library or framework.\n"
        "Use context7_libraries first to obtain the correct libraryId.\n"
        "Returns: Documentation snippets and code examples relevant to the query.\n\n"
        "Keep each query scoped to a single concept. For multi-concept questions, "
        "make separate calls per concept unless the question is about how the concepts interact.\n"
    )
    _QUERY_DOCS_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "libraryId": {
                "type": "string",
                "description": (
                    "Context7-compatible library ID as returned by context7_libraries "
                    "(e.g. '/reactjs/react.dev', '/vercel/next.js'). "
                    "Optionally suffix with a version: '/vercel/next.js/v14.3.0'."
                ),
            },
            "query": {
                "type": "string",
                "description": (
                    "The question or task to find documentation for, scoped to a single concept. "
                    "Be specific and include relevant details "
                    "(e.g. 'React useEffect cleanup function examples')."
                ),
            },
        },
        "required": ["libraryId", "query"],
    }
    _QUERY_DOCS_OUTPUT: dict[str, Any] = {
        "type": "object",
        "properties": {
            "content": {
                "type": "string",
                "description": "Documentation snippets and code examples relevant to the query.",
            },
        },
        "required": ["content"],
    }

    _RO: dict[str, Any] = {"readOnlyHint": True, "openWorldHint": True}


    class Context7Bridge(McpBridge):
        """Bridge to the Context7 remote MCP server."""

        def build_client(self, config: ServerConfig) -> McpClient:
            headers: dict[str, str] = {}
            if config.context7_api_key:
                headers["CONTEXT7_API_KEY"] = config.context7_api_key
            return McpClient(config.context7_mcp_url, headers=headers)


    #: Module-level bridge, built by :func:`register_context7_tools`.
    _bridge: Context7Bridge | None = None


    def _get_bridge() -> Context7Bridge:
        """Return the module-level Context7 bridge configured by :func:`register_context7_tools`."""
        if _bridge is None:
            raise McpBridgeError("Context7 tools used before register_context7_tools() was called.")
        return _bridge


    def context7_libraries(libraryName: str, query: str) -> dict:
        """Search Context7 for a library and return its canonical library ID.

        Best for: Resolving a library name to the ID needed by
        ``context7_documentation``.

        Args:
            libraryName: Library name to search for (e.g. 'react', 'next.js', 'vue').
            query: User's original question or task, used for relevance ranking.

        Returns:
            Ranked list of matching libraries (ID, title, description, snippet
            count, reputation, benchmark score, versions).

        Raises:
            McpBridgeError: if the Context7 call fails.
        """
        return _get_bridge().call("resolve-library-id", compact(libraryName=libraryName, query=query))


    def context7_documentation(libraryId: str, query: str) -> dict:
        """Fetch documentation and code examples for a library from Context7.

        Best for: Retrieving accurate API docs, usage examples, and
        configuration guides for any library or framework. Use
        ``context7_libraries`` first to obtain the correct libraryId. Keep
        each query scoped to a single concept.

        Args:
            libraryId: Context7-compatible library ID as returned by
                ``context7_libraries`` (e.g. '/reactjs/react.dev'), optionally
                suffixed with a version.
            query: The question or task to find documentation for, scoped to
                a single concept.

        Returns:
            Documentation snippets and code examples relevant to the query.

        Raises:
            McpBridgeError: if the Context7 call fails.
        """
        return _get_bridge().call("query-docs", compact(libraryId=libraryId, query=query))


    class Context7LibrariesTool(ToolDefinition):
        name = "context7_libraries"
        title = "Context7 resolve library ID"
        description = _RESOLVE_DESCRIPTION
        input_schema = _RESOLVE_SCHEMA
        output_schema = _RESOLVE_OUTPUT
        annotations = _RO

        def handle(self, ctx: ToolContext) -> ToolResult:
            args = ctx.arguments
            try:
                result = context7_libraries(libraryName=args["libraryName"], query=args["query"])
            except McpBridgeError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content=result)


    class Context7DocumentationTool(ToolDefinition):
        name = "context7_documentation"
        title = "Context7 query docs"
        description = _QUERY_DOCS_DESCRIPTION
        input_schema = _QUERY_DOCS_SCHEMA
        output_schema = _QUERY_DOCS_OUTPUT
        annotations = _RO

        def handle(self, ctx: ToolContext) -> ToolResult:
            args = ctx.arguments
            try:
                result = context7_documentation(libraryId=args["libraryId"], query=args["query"])
            except McpBridgeError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content=result)


    def register_context7_tools(
        registry: ToolRegistry,
        environment: AppEnvironment,
    ) -> None:
        """Register the Context7-backed ``context7_libraries`` and
        ``context7_documentation`` tools."""
        global _bridge
        _bridge = Context7Bridge(environment.config)
        registry.register(Context7LibrariesTool())
        registry.register(Context7DocumentationTool())
        environment.functions.register(context7_libraries)
        environment.functions.register(context7_documentation)
```
/answer ded6-b allow
ReasoningToken: 16000
Control Request:
```yaml
id: ded6-c
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/github.py
  mode: replace
  reason: "Move all core GitHub functions to module root instead of closures inside register_github_tools, and replace the dynamic type()-based tool factory with a proper top-level ToolDefinition subclass, per the ABC registration pattern."
  content: |
    """GitHub bridge – read-only research tools backed by GitHub's remote MCP server.

    Only read-only tools are registered: file/code access, issues, discussions,
    pull requests, commits, and project information.
    """

    from __future__ import annotations

    from typing import Any, Callable

    from xy.ai.mcpc.config import ServerConfig
    from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
    from xy.ai.mcpc.tools.mcp.bridge import McpBridge, McpBridgeError, compact
    from xy.ai.mcpc.tools.mcp.client import McpClient, McpClientError

    __all__ = ["GitHubBridge", "GitHubTool", "register_github_tools"]

    _CONTENT_OUTPUT: dict[str, Any] = {
        "type": "object",
        "properties": {
            "content": {
                "type": "string",
                "description": "Response content from the GitHub MCP server.",
            },
        },
        "required": ["content"],
    }

    _RO: dict[str, Any] = {"readOnlyHint": True, "openWorldHint": True}

    _GET_FILE_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner (user or org)."},
            "repo": {"type": "string", "description": "Repository name."},
            "path": {
                "type": "string",
                "description": "Path to the file or directory (omit for root).",
            },
            "ref": {
                "type": "string",
                "description": (
                    "Branch, tag, or ref (e.g. refs/heads/main, refs/pull/42/head). "
                    "Ignored when sha is provided."
                ),
            },
            "sha": {
                "type": "string",
                "description": "Exact commit SHA; takes precedence over ref.",
            },
        },
        "required": ["owner", "repo"],
    }

    _GET_TREE_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner."},
            "repo": {"type": "string", "description": "Repository name."},
            "tree_sha": {
                "type": "string",
                "description": "SHA, branch, or tag to read the tree from (defaults to default branch).",
            },
            "recursive": {
                "type": "boolean",
                "description": "Recurse into sub-trees (default false).",
            },
            "path_filter": {
                "type": "string",
                "description": "Optional path prefix to filter results (e.g. 'src/').",
            },
        },
        "required": ["owner", "repo"],
    }

    _SEARCH_CODE_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "query": {
                "type": "string",
                "description": (
                    "GitHub code search query (max 256 chars). "
                    "Qualifiers: repo:owner/repo, org:, language:, path:, "
                    "filename:, extension:, in:file|path."
                ),
            },
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 15).",
                "minimum": 1,
                "maximum": 15,
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        },
        "required": ["query"],
    }

    _SEARCH_COMMITS_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "query": {
                "type": "string",
                "description": (
                    "GitHub commit search query. Scope with repo:owner/repo or org:. "
                    "Qualifiers: author:, committer:, author-date:, committer-date:, "
                    "merge:true|false, hash:."
                ),
            },
            "sort": {
                "type": "string",
                "description": "Sort by author-date or committer-date (defaults to best match).",
            },
            "order": {"type": "string", "description": "Sort order: asc | desc."},
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 15).",
                "minimum": 1,
                "maximum": 15,
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        },
        "required": ["query"],
    }

    _SEARCH_REPOS_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "query": {
                "type": "string",
                "description": (
                    "Repository search query. Supports qualifiers: "
                    "topic:, language:, stars:>N, user:, org:, is:archived."
                ),
            },
            "sort": {
                "type": "string",
                "description": "Sort by: stars | forks | help-wanted-issues | updated.",
            },
            "order": {"type": "string", "description": "Sort order: asc | desc."},
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 10).",
                "minimum": 1,
                "maximum": 10,
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
            "minimal_output": {
                "type": "boolean",
                "description": "Return minimal repository info (default true).",
            },
        },
        "required": ["query"],
    }

    _ISSUE_READ_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner."},
            "repo": {"type": "string", "description": "Repository name."},
            "issue_number": {"type": "integer", "description": "Issue number."},
            "method": {
                "type": "string",
                "description": (
                    "Read operation to perform:\n"
                    "  get – issue body and metadata\n"
                    "  get_comments – issue comments\n"
                    "  get_sub_issues – child issues\n"
                    "  get_parent – parent issue (if this is a sub-issue)\n"
                    "  get_labels – labels assigned to the issue"
                ),
                "enum": ["get", "get_comments", "get_sub_issues", "get_parent", "get_labels"],
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 20).",
                "minimum": 1,
                "maximum": 20,
            },
        },
        "required": ["owner", "repo", "issue_number", "method"],
    }

    _LIST_ISSUES_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner."},
            "repo": {"type": "string", "description": "Repository name."},
            "state": {
                "type": "string",
                "description": "Filter by state: open | closed (default: both).",
            },
            "labels": {
                "type": "array",
                "items": {"type": "string"},
                "description": "Filter by label names.",
            },
            "since": {
                "type": "string",
                "description": "Only issues updated after this ISO 8601 timestamp.",
            },
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 15).",
                "minimum": 1,
                "maximum": 15,
            },
            "after": {
                "type": "string",
                "description": "Cursor for pagination (from previous response).",
            },
        },
        "required": ["owner", "repo"],
    }

    _SEARCH_ISSUES_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "query": {
                "type": "string",
                "description": "Search query using GitHub issues search syntax.",
            },
            "owner": {
                "type": "string",
                "description": "Restrict to this owner (requires repo).",
            },
            "repo": {
                "type": "string",
                "description": "Restrict to this repo (requires owner).",
            },
            "sort": {"type": "string", "description": "Sort field."},
            "order": {"type": "string", "description": "Sort order: asc | desc."},
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 15).",
                "minimum": 1,
                "maximum": 15,
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        },
        "required": ["query"],
    }

    _GET_DISCUSSION_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner."},
            "repo": {"type": "string", "description": "Repository name."},
            "discussionNumber": {"type": "integer", "description": "Discussion number."},
        },
        "required": ["owner", "repo", "discussionNumber"],
    }

    _GET_DISCUSSION_COMMENTS_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner."},
            "repo": {"type": "string", "description": "Repository name."},
            "discussionNumber": {"type": "integer", "description": "Discussion number."},
            "includeReplies": {
                "type": "boolean",
                "description": "Include nested replies per comment (up to 100, default false).",
            },
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 20).",
                "minimum": 1,
                "maximum": 20,
            },
            "after": {"type": "string", "description": "Cursor for pagination."},
        },
        "required": ["owner", "repo", "discussionNumber"],
    }

    _LIST_DISCUSSIONS_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner or org."},
            "repo": {
                "type": "string",
                "description": "Repository name (omit for org-level discussions).",
            },
            "category": {
                "type": "string",
                "description": "Filter by discussion category ID.",
            },
            "orderBy": {
                "type": "string",
                "description": "Order by field (requires direction).",
            },
            "direction": {"type": "string", "description": "Order direction: ASC | DESC."},
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 20).",
                "minimum": 1,
                "maximum": 20,
            },
            "after": {"type": "string", "description": "Cursor for pagination."},
        },
        "required": ["owner"],
    }

    _PR_READ_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner."},
            "repo": {"type": "string", "description": "Repository name."},
            "pullNumber": {"type": "integer", "description": "Pull request number."},
            "method": {
                "type": "string",
                "description": (
                    "Data to retrieve:\n"
                    "  get – PR body and metadata\n"
                    "  get_diff – unified diff\n"
                    "  get_status – combined commit status\n"
                    "  get_files – changed files\n"
                    "  get_commits – commits on the PR\n"
                    "  get_review_comments – review threads\n"
                    "  get_reviews – review summaries\n"
                    "  get_comments – general comments\n"
                    "  get_check_runs – CI check runs"
                ),
                "enum": [
                    "get",
                    "get_diff",
                    "get_status",
                    "get_files",
                    "get_commits",
                    "get_review_comments",
                    "get_reviews",
                    "get_comments",
                    "get_check_runs",
                ],
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 10).",
                "minimum": 1,
                "maximum": 10,
            },
            "after": {
                "type": "string",
                "description": "Cursor for pagination (get_review_comments only).",
            },
        },
        "required": ["owner", "repo", "pullNumber", "method"],
    }

    _LIST_PRS_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner."},
            "repo": {"type": "string", "description": "Repository name."},
            "state": {"type": "string", "description": "Filter: open | closed | all."},
            "base": {"type": "string", "description": "Filter by base branch name."},
            "sort": {
                "type": "string",
                "description": "Sort by: created | updated | popularity | long-running.",
            },
            "direction": {"type": "string", "description": "Sort direction: asc | desc."},
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 10).",
                "minimum": 1,
                "maximum": 10,
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        },
        "required": ["owner", "repo"],
    }

    _SEARCH_PRS_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "query": {
                "type": "string",
                "description": "Search query using GitHub pull request search syntax.",
            },
            "owner": {
                "type": "string",
                "description": "Restrict to this owner (requires repo).",
            },
            "repo": {
                "type": "string",
                "description": "Restrict to this repo (requires owner).",
            },
            "sort": {"type": "string", "description": "Sort field."},
            "order": {"type": "string", "description": "Sort order: asc | desc."},
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 10).",
                "minimum": 1,
                "maximum": 10,
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        },
        "required": ["query"],
    }

    _GET_COMMIT_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner."},
            "repo": {"type": "string", "description": "Repository name."},
            "sha": {"type": "string", "description": "Commit SHA, branch name, or tag name."},
            "detail": {
                "type": "string",
                "description": (
                    "File detail level:\n"
                    "  none – omit files entirely\n"
                    "  stats – per-file counts (default)\n"
                    "  full_patch – includes diff content (can be large)"
                ),
                "enum": ["none", "stats", "full_patch"],
            },
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 10).",
                "minimum": 1,
                "maximum": 10,
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        },
        "required": ["owner", "repo", "sha"],
    }

    _LIST_COMMITS_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "owner": {"type": "string", "description": "Repository owner."},
            "repo": {"type": "string", "description": "Repository name."},
            "sha": {
                "type": "string",
                "description": "Branch, tag, or SHA to list commits from (defaults to default branch).",
            },
            "path": {
                "type": "string",
                "description": "Only commits touching this file path.",
            },
            "author": {
                "type": "string",
                "description": "Filter by author username or email.",
            },
            "since": {
                "type": "string",
                "description": "Only commits after this date (ISO 8601: YYYY-MM-DDTHH:MM:SSZ).",
            },
            "until": {
                "type": "string",
                "description": "Only commits before this date (ISO 8601).",
            },
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 10).",
                "minimum": 1,
                "maximum": 10,
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        },
        "required": ["owner", "repo"],
    }

    _PROJECTS_GET_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "method": {
                "type": "string",
                "description": (
                    "Operation:\n"
                    "  get_project – project metadata\n"
                    "  get_project_field – a single project field\n"
                    "  get_project_item – a single project item\n"
                    "  get_project_status_update – a status update"
                ),
                "enum": [
                    "get_project",
                    "get_project_field",
                    "get_project_item",
                    "get_project_status_update",
                ],
            },
            "owner": {
                "type": "string",
                "description": "Owner (user or org login).",
            },
            "owner_type": {
                "type": "string",
                "description": "Owner type: user | org (auto-detected if omitted).",
            },
            "project_number": {"type": "integer", "description": "Project number."},
            "field_id": {
                "type": "integer",
                "description": "Field ID (required for get_project_field).",
            },
            "item_id": {
                "type": "integer",
                "description": "Item ID (required for get_project_item).",
            },
            "fields": {
                "type": "array",
                "items": {"type": "string"},
                "description": "Field IDs to include in get_project_item response.",
            },
            "status_update_id": {
                "type": "string",
                "description": "Status update node ID (required for get_project_status_update).",
            },
        },
        "required": ["method"],
    }

    _PROJECTS_LIST_SCHEMA: dict[str, Any] = {
        "type": "object",
        "properties": {
            "method": {
                "type": "string",
                "description": (
                    "Operation:\n"
                    "  list_projects – projects for an owner\n"
                    "  list_project_fields – fields of a project\n"
                    "  list_project_items – items in a project\n"
                    "  list_project_status_updates – status updates"
                ),
                "enum": [
                    "list_projects",
                    "list_project_fields",
                    "list_project_items",
                    "list_project_status_updates",
                ],
            },
            "owner": {"type": "string", "description": "Owner (user or org login)."},
            "owner_type": {
                "type": "string",
                "description": "Owner type: user | org.",
            },
            "project_number": {
                "type": "integer",
                "description": "Project number (required for fields, items, and status updates).",
            },
            "query": {
                "type": "string",
                "description": (
                    "Filter string: for list_projects use title/state filters; "
                    "for list_project_items use GitHub project filter syntax."
                ),
            },
            "fields": {
                "type": "array",
                "items": {"type": "string"},
                "description": "Field IDs to include for list_project_items.",
            },
            "per_page": {
                "type": "integer",
                "description": "Results per page (max 20).",
                "minimum": 1,
                "maximum": 20,
            },
            "after": {"type": "string", "description": "Forward pagination cursor."},
            "before": {"type": "string", "description": "Backward pagination cursor."},
        },
        "required": ["method", "owner"],
    }


    class GitHubBridge(McpBridge):
        """Bridge to the GitHub remote MCP server (read-only)."""

        def build_client(self, config: ServerConfig) -> McpClient:
            pat = config.github_api_pat
            if not pat:
                raise McpClientError(
                    "GitHub PAT is not configured (set MCPC_GITHUB_PAT)."
                )
            return McpClient(
                config.github_mcp_url,
                headers={"Authorization": f"Bearer {pat}"},
            )


    #: Module-level bridge, built by :func:`register_github_tools`.
    _bridge: GitHubBridge | None = None


    def _get_bridge() -> GitHubBridge:
        """Return the module-level GitHub bridge configured by :func:`register_github_tools`."""
        if _bridge is None:
            raise McpBridgeError("GitHub tools used before register_github_tools() was called.")
        return _bridge


    def github_get_file(
        owner: str, repo: str, path: str | None = None, ref: str | None = None, sha: str | None = None
    ) -> dict:
        """Read a file or directory listing from a GitHub repository.

        Best for: Fetching source code, configs, and READMEs at any ref or commit.

        Args:
            owner: Repository owner (user or org).
            repo: Repository name.
            path: Path to the file or directory (omit for root).
            ref: Branch, tag, or ref (e.g. refs/heads/main); ignored when sha is given.
            sha: Exact commit SHA; takes precedence over ref.
        """
        return _get_bridge().call("get_file_contents", compact(owner=owner, repo=repo, path=path, ref=ref, sha=sha))


    def github_get_tree(
        owner: str,
        repo: str,
        tree_sha: str | None = None,
        recursive: bool | None = None,
        path_filter: str | None = None,
    ) -> dict:
        """List the file tree of a GitHub repository at a given ref.

        Best for: Understanding project layout before reading individual files.

        Args:
            owner: Repository owner.
            repo: Repository name.
            tree_sha: SHA, branch, or tag to read the tree from (defaults to default branch).
            recursive: Recurse into sub-trees (default false).
            path_filter: Optional path prefix to filter results (e.g. 'src/').
        """
        return _get_bridge().call(
            "get_repository_tree",
            compact(owner=owner, repo=repo, tree_sha=tree_sha, recursive=recursive, path_filter=path_filter),
        )


    def github_search_code(query: str, perPage: int | None = None, page: int | None = None) -> dict:
        """Search GitHub code across repositories.

        Best for: Finding specific functions, patterns, or usages across the
        GitHub ecosystem.

        Args:
            query: GitHub code search query (max 256 chars); qualifiers:
                repo:owner/repo, org:, language:, path:, filename:, extension:, in:file|path.
            perPage: Results per page (max 15).
            page: Page number (min 1).
        """
        return _get_bridge().call("search_code", compact(query=query, perPage=perPage, page=page))


    def github_search_commits(
        query: str,
        sort: str | None = None,
        order: str | None = None,
        perPage: int | None = None,
        page: int | None = None,
    ) -> dict:
        """Search commit messages on GitHub.

        Best for: Finding commits by message keyword, author, or date across repositories.

        Args:
            query: GitHub commit search query; scope with repo:owner/repo or org:;
                qualifiers: author:, committer:, author-date:, committer-date:,
                merge:true|false, hash:.
            sort: Sort by author-date or committer-date (defaults to best match).
            order: Sort order: asc | desc.
            perPage: Results per page (max 15).
            page: Page number (min 1).
        """
        return _get_bridge().call(
            "search_commits", compact(query=query, sort=sort, order=order, perPage=perPage, page=page)
        )


    def github_search_repos(
        query: str,
        sort: str | None = None,
        order: str | None = None,
        perPage: int | None = None,
        page: int | None = None,
        minimal_output: bool | None = None,
    ) -> dict:
        """Search GitHub for repositories matching a query.

        Best for: Discovering projects by name, topic, language, or stars.

        Args:
            query: Repository search query; qualifiers: topic:, language:,
                stars:>N, user:, org:, is:archived.
            sort: Sort by: stars | forks | help-wanted-issues | updated.
            order: Sort order: asc | desc.
            perPage: Results per page (max 10).
            page: Page number (min 1).
            minimal_output: Return minimal repository info (default true).
        """
        return _get_bridge().call(
            "search_repositories",
            compact(query=query, sort=sort, order=order, perPage=perPage, page=page, minimal_output=minimal_output),
        )


    def github_issue_read(
        owner: str,
        repo: str,
        issue_number: int,
        method: str,
        page: int | None = None,
        perPage: int | None = None,
    ) -> dict:
        """Read a GitHub issue: body, comments, sub-issues, labels, or parent.

        Args:
            owner: Repository owner.
            repo: Repository name.
            issue_number: Issue number.
            method: One of get | get_comments | get_sub_issues | get_parent | get_labels.
            page: Page number (min 1).
            perPage: Results per page (max 20).
        """
        return _get_bridge().call(
            "issue_read",
            compact(owner=owner, repo=repo, issue_number=issue_number, method=method, page=page, perPage=perPage),
        )


    def github_list_issues(
        owner: str,
        repo: str,
        state: str | None = None,
        labels: list[str] | None = None,
        since: str | None = None,
        perPage: int | None = None,
        after: str | None = None,
    ) -> dict:
        """List issues in a GitHub repository with optional filters.

        Best for: Enumerating open or closed issues, filtering by label or state.

        Args:
            owner: Repository owner.
            repo: Repository name.
            state: Filter by state: open | closed (default: both).
            labels: Filter by label names.
            since: Only issues updated after this ISO 8601 timestamp.
            perPage: Results per page (max 15).
            after: Cursor for pagination (from previous response).
        """
        return _get_bridge().call(
            "list_issues",
            compact(owner=owner, repo=repo, state=state, labels=labels, since=since, perPage=perPage, after=after),
        )


    def github_search_issues(
        query: str,
        owner: str | None = None,
        repo: str | None = None,
        sort: str | None = None,
        order: str | None = None,
        perPage: int | None = None,
        page: int | None = None,
    ) -> dict:
        """Search GitHub issues using GitHub's issue search syntax.

        Best for: Finding issues by keyword, author, label, or state across repositories.

        Args:
            query: Search query using GitHub issues search syntax.
            owner: Restrict to this owner (requires repo).
            repo: Restrict to this repo (requires owner).
            sort: Sort field.
            order: Sort order: asc | desc.
            perPage: Results per page (max 15).
            page: Page number (min 1).
        """
        return _get_bridge().call(
            "search_issues",
            compact(query=query, owner=owner, repo=repo, sort=sort, order=order, perPage=perPage, page=page),
        )


    def github_get_discussion(owner: str, repo: str, discussionNumber: int) -> dict:
        """Get the body and metadata of a single GitHub Discussion.

        Best for: Reading a specific community discussion or Q&A thread.

        Args:
            owner: Repository owner.
            repo: Repository name.
            discussionNumber: Discussion number.
        """
        return _get_bridge().call(
            "get_discussion", compact(owner=owner, repo=repo, discussionNumber=discussionNumber)
        )


    def github_get_discussion_comments(
        owner: str,
        repo: str,
        discussionNumber: int,
        includeReplies: bool | None = None,
        perPage: int | None = None,
        after: str | None = None,
    ) -> dict:
        """Get comments for a GitHub Discussion, optionally including nested replies.

        Best for: Reading community feedback, answers, and Q&A responses.

        Args:
            owner: Repository owner.
            repo: Repository name.
            discussionNumber: Discussion number.
            includeReplies: Include nested replies per comment (up to 100, default false).
            perPage: Results per page (max 20).
            after: Cursor for pagination.
        """
        return _get_bridge().call(
            "get_discussion_comments",
            compact(
                owner=owner,
                repo=repo,
                discussionNumber=discussionNumber,
                includeReplies=includeReplies,
                perPage=perPage,
                after=after,
            ),
        )


    def github_list_discussions(
        owner: str,
        repo: str | None = None,
        category: str | None = None,
        orderBy: str | None = None,
        direction: str | None = None,
        perPage: int | None = None,
        after: str | None = None,
    ) -> dict:
        """List GitHub Discussions for a repository or organisation.

        Best for: Browsing community discussions, optionally filtered by category.

        Args:
            owner: Repository owner or org.
            repo: Repository name (omit for org-level discussions).
            category: Filter by discussion category ID.
            orderBy: Order by field (requires direction).
            direction: Order direction: ASC | DESC.
            perPage: Results per page (max 20).
            after: Cursor for pagination.
        """
        return _get_bridge().call(
            "list_discussions",
            compact(
                owner=owner, repo=repo, category=category, orderBy=orderBy, direction=direction,
                perPage=perPage, after=after,
            ),
        )


    def github_pr_read(
        owner: str,
        repo: str,
        pullNumber: int,
        method: str,
        page: int | None = None,
        perPage: int | None = None,
        after: str | None = None,
    ) -> dict:
        """Read details of a GitHub Pull Request: body, diff, files, commits, reviews, or comments.

        Args:
            owner: Repository owner.
            repo: Repository name.
            pullNumber: Pull request number.
            method: One of get | get_diff | get_status | get_files | get_commits |
                get_review_comments | get_reviews | get_comments | get_check_runs.
            page: Page number (min 1).
            perPage: Results per page (max 10).
            after: Cursor for pagination (get_review_comments only).
        """
        return _get_bridge().call(
            "pull_request_read",
            compact(owner=owner, repo=repo, pullNumber=pullNumber, method=method, page=page, perPage=perPage, after=after),
        )


    def github_list_prs(
        owner: str,
        repo: str,
        state: str | None = None,
        base: str | None = None,
        sort: str | None = None,
        direction: str | None = None,
        perPage: int | None = None,
        page: int | None = None,
    ) -> dict:
        """List pull requests in a GitHub repository.

        Best for: Enumerating open or merged PRs with optional state and base-branch filters.

        Args:
            owner: Repository owner.
            repo: Repository name.
            state: Filter: open | closed | all.
            base: Filter by base branch name.
            sort: Sort by: created | updated | popularity | long-running.
            direction: Sort direction: asc | desc.
            perPage: Results per page (max 10).
            page: Page number (min 1).
        """
        return _get_bridge().call(
            "list_pull_requests",
            compact(owner=owner, repo=repo, state=state, base=base, sort=sort, direction=direction,
                    perPage=perPage, page=page),
        )


    def github_search_prs(
        query: str,
        owner: str | None = None,
        repo: str | None = None,
        sort: str | None = None,
        order: str | None = None,
        perPage: int | None = None,
        page: int | None = None,
    ) -> dict:
        """Search GitHub pull requests using GitHub's PR search syntax.

        Best for: Finding PRs by keyword, author, state, or label across repositories.

        Args:
            query: Search query using GitHub pull request search syntax.
            owner: Restrict to this owner (requires repo).
            repo: Restrict to this repo (requires owner).
            sort: Sort field.
            order: Sort order: asc | desc.
            perPage: Results per page (max 10).
            page: Page number (min 1).
        """
        return _get_bridge().call(
            "search_pull_requests",
            compact(query=query, owner=owner, repo=repo, sort=sort, order=order, perPage=perPage, page=page),
        )


    def github_get_commit(
        owner: str,
        repo: str,
        sha: str,
        detail: str | None = None,
        perPage: int | None = None,
        page: int | None = None,
    ) -> dict:
        """Get details of a single GitHub commit including changed files.

        Best for: Inspecting what changed in a specific commit.

        Args:
            owner: Repository owner.
            repo: Repository name.
            sha: Commit SHA, branch name, or tag name.
            detail: File detail level: none | stats (default) | full_patch.
            perPage: Results per page (max 10).
            page: Page number (min 1).
        """
        return _get_bridge().call(
            "get_commit", compact(owner=owner, repo=repo, sha=sha, detail=detail, perPage=perPage, page=page)
        )


    def github_list_commits(
        owner: str,
        repo: str,
        sha: str | None = None,
        path: str | None = None,
        author: str | None = None,
        since: str | None = None,
        until: str | None = None,
        perPage: int | None = None,
        page: int | None = None,
    ) -> dict:
        """List commits in a GitHub repository, optionally filtered by author, path, or date.

        Best for: Reviewing recent history or changes to a specific file.

        Args:
            owner: Repository owner.
            repo: Repository name.
            sha: Branch, tag, or SHA to list commits from (defaults to default branch).
            path: Only commits touching this file path.
            author: Filter by author username or email.
            since: Only commits after this date (ISO 8601).
            until: Only commits before this date (ISO 8601).
            perPage: Results per page (max 10).
            page: Page number (min 1).
        """
        return _get_bridge().call(
            "list_commits",
            compact(owner=owner, repo=repo, sha=sha, path=path, author=author, since=since, until=until,
                    perPage=perPage, page=page),
        )


    def github_projects_get(
        method: str,
        owner: str | None = None,
        owner_type: str | None = None,
        project_number: int | None = None,
        field_id: int | None = None,
        item_id: int | None = None,
        fields: list[str] | None = None,
        status_update_id: str | None = None,
    ) -> dict:
        """Get details of a GitHub Project or one of its fields, items, or status updates.

        Args:
            method: One of get_project | get_project_field | get_project_item |
                get_project_status_update.
            owner: Owner (user or org login).
            owner_type: Owner type: user | org (auto-detected if omitted).
            project_number: Project number.
            field_id: Field ID (required for get_project_field).
            item_id: Item ID (required for get_project_item).
            fields: Field IDs to include in get_project_item response.
            status_update_id: Status update node ID (required for get_project_status_update).
        """
        return _get_bridge().call(
            "projects_get",
            compact(
                method=method, owner=owner, owner_type=owner_type, project_number=project_number,
                field_id=field_id, item_id=item_id, fields=fields, status_update_id=status_update_id,
            ),
        )


    def github_projects_list(
        method: str,
        owner: str,
        owner_type: str | None = None,
        project_number: int | None = None,
        query: str | None = None,
        fields: list[str] | None = None,
        per_page: int | None = None,
        after: str | None = None,
        before: str | None = None,
    ) -> dict:
        """List GitHub Projects resources: projects, fields, items, or status updates.

        Args:
            method: One of list_projects | list_project_fields | list_project_items |
                list_project_status_updates.
            owner: Owner (user or org login).
            owner_type: Owner type: user | org.
            project_number: Project number (required for fields, items, and status updates).
            query: Filter string (title/state filters for list_projects; GitHub
                project filter syntax for list_project_items).
            fields: Field IDs to include for list_project_items.
            per_page: Results per page (max 20).
            after: Forward pagination cursor.
            before: Backward pagination cursor.
        """
        return _get_bridge().call(
            "projects_list",
            compact(
                method=method, owner=owner, owner_type=owner_type, project_number=project_number,
                query=query, fields=fields, per_page=per_page, after=after, before=before,
            ),
        )


    class GitHubTool(ToolDefinition):
        """Generic read-only GitHub tool: forwards ``ctx.arguments`` to a bridge-backed
        core function, translating an :class:`McpBridgeError` into an error
        :class:`ToolResult`. The core function remains the only thing that talks
        to the bridge.
        """

        def __init__(
            self, name: str, title: str, description: str, input_schema: dict[str, Any], core: Callable[..., dict]
        ) -> None:
            self.name = name
            self.title = title
            self.description = description
            self.input_schema = input_schema
            self.output_schema = _CONTENT_OUTPUT
            self.annotations = _RO
            self._core = core

        def handle(self, ctx: ToolContext) -> ToolResult:
            try:
                result = self._core(**ctx.arguments)
            except McpBridgeError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content=result)


    _TOOLS: list[tuple[str, str, str, dict[str, Any], Callable[..., dict]]] = [
        (
            "github_get_file",
            "GitHub get file contents",
            "Read a file or directory listing from a GitHub repository.\n\n"
            "Best for: Fetching source code, configs, and READMEs at any ref or commit.",
            _GET_FILE_SCHEMA,
            github_get_file,
        ),
        (
            "github_get_tree",
            "GitHub get repository tree",
            "List the file tree of a GitHub repository at a given ref.\n\n"
            "Best for: Understanding project layout before reading individual files.",
            _GET_TREE_SCHEMA,
            github_get_tree,
        ),
        (
            "github_search_code",
            "GitHub search code",
            "Search GitHub code across repositories.\n\n"
            "Best for: Finding specific functions, patterns, or usages across the GitHub ecosystem.",
            _SEARCH_CODE_SCHEMA,
            github_search_code,
        ),
        (
            "github_search_commits",
            "GitHub search commits",
            "Search commit messages on GitHub.\n\n"
            "Best for: Finding commits by message keyword, author, or date across repositories.",
            _SEARCH_COMMITS_SCHEMA,
            github_search_commits,
        ),
        (
            "github_search_repos",
            "GitHub search repositories",
            "Search GitHub for repositories matching a query.\n\n"
            "Best for: Discovering projects by name, topic, language, or stars.",
            _SEARCH_REPOS_SCHEMA,
            github_search_repos,
        ),
        (
            "github_issue_read",
            "GitHub read issue",
            "Read a GitHub issue: body, comments, sub-issues, labels, or parent.\n\n"
            "method: get | get_comments | get_sub_issues | get_parent | get_labels",
            _ISSUE_READ_SCHEMA,
            github_issue_read,
        ),
        (
            "github_list_issues",
            "GitHub list issues",
            "List issues in a GitHub repository with optional filters.\n\n"
            "Best for: Enumerating open or closed issues, filtering by label or state.",
            _LIST_ISSUES_SCHEMA,
            github_list_issues,
        ),
        (
            "github_search_issues",
            "GitHub search issues",
            "Search GitHub issues using GitHub's issue search syntax.\n\n"
            "Best for: Finding issues by keyword, author, label, or state across repositories.",
            _SEARCH_ISSUES_SCHEMA,
            github_search_issues,
        ),
        (
            "github_get_discussion",
            "GitHub get discussion",
            "Get the body and metadata of a single GitHub Discussion.\n\n"
            "Best for: Reading a specific community discussion or Q&A thread.",
            _GET_DISCUSSION_SCHEMA,
            github_get_discussion,
        ),
        (
            "github_get_discussion_comments",
            "GitHub get discussion comments",
            "Get comments for a GitHub Discussion, optionally including nested replies.\n\n"
            "Best for: Reading community feedback, answers, and Q&A responses.",
            _GET_DISCUSSION_COMMENTS_SCHEMA,
            github_get_discussion_comments,
        ),
        (
            "github_list_discussions",
            "GitHub list discussions",
            "List GitHub Discussions for a repository or organisation.\n\n"
            "Best for: Browsing community discussions, optionally filtered by category.",
            _LIST_DISCUSSIONS_SCHEMA,
            github_list_discussions,
        ),
        (
            "github_pr_read",
            "GitHub read pull request",
            "Read details of a GitHub Pull Request: body, diff, files, commits, "
            "reviews, or comments.\n\n"
            "method: get | get_diff | get_status | get_files | get_commits | "
            "get_review_comments | get_reviews | get_comments | get_check_runs",
            _PR_READ_SCHEMA,
            github_pr_read,
        ),
        (
            "github_list_prs",
            "GitHub list pull requests",
            "List pull requests in a GitHub repository.\n\n"
            "Best for: Enumerating open or merged PRs with optional state and base-branch filters.",
            _LIST_PRS_SCHEMA,
            github_list_prs,
        ),
        (
            "github_search_prs",
            "GitHub search pull requests",
            "Search GitHub pull requests using GitHub's PR search syntax.\n\n"
            "Best for: Finding PRs by keyword, author, state, or label across repositories.",
            _SEARCH_PRS_SCHEMA,
            github_search_prs,
        ),
        (
            "github_get_commit",
            "GitHub get commit",
            "Get details of a single GitHub commit including changed files.\n\n"
            "Best for: Inspecting what changed in a specific commit.",
            _GET_COMMIT_SCHEMA,
            github_get_commit,
        ),
        (
            "github_list_commits",
            "GitHub list commits",
            "List commits in a GitHub repository, optionally filtered by author, path, or date.\n\n"
            "Best for: Reviewing recent history or changes to a specific file.",
            _LIST_COMMITS_SCHEMA,
            github_list_commits,
        ),
        (
            "github_projects_get",
            "GitHub get project",
            "Get details of a GitHub Project or one of its fields, items, or status updates.\n\n"
            "method: get_project | get_project_field | get_project_item | get_project_status_update",
            _PROJECTS_GET_SCHEMA,
            github_projects_get,
        ),
        (
            "github_projects_list",
            "GitHub list projects",
            "List GitHub Projects resources: projects, fields, items, or status updates.\n\n"
            "method: list_projects | list_project_fields | list_project_items | "
            "list_project_status_updates",
            _PROJECTS_LIST_SCHEMA,
            github_projects_list,
        ),
    ]


    def register_github_tools(
        registry: ToolRegistry,
        environment: AppEnvironment,
    ) -> None:
        """Register read-only GitHub research tools."""
        global _bridge
        _bridge = GitHubBridge(environment.config)
        functions = environment.functions
        for name, title, description, input_schema, core in _TOOLS:
            registry.register(GitHubTool(name, title, description, input_schema, core))
            functions.register(core)
```
/answer ded6-c allow
Cannot read STDOUT: the CLI process is not started (uuid=8d88764e-0067-4fa7-8e42-fe59a9580496, processAlive=false). A prompt must be sent (writeLine) before reading.