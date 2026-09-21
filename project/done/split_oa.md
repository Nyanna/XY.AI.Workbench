Die Datei `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/__init__.py` ist zu groß. Splitte sie auf jedes Tool bekommt seine eigene Datei.
Result Stats: id=b2fc706a-1e14-4ecb-bfb6-dfbb76ef6b0b, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
SystemInit:  id=b2fc706a-1e14-4ecb-bfb6-dfbb76ef6b0b, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: e694-1
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/__init__.py
  reason: "Struktur der Datei ermitteln, um Tools zu identifizieren"
```
/answer e694-1 allow
Result Stats: id=b2fc706a-1e14-4ecb-bfb6-dfbb76ef6b0b, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
Control Request:
```yaml
id: e694-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/__init__.py
      nodes:
      - id: 322ZBs|Sp8Hzo
        signature: '"OpenAlex tools.\n\nThree tools sit on top of the :mod:`xy.ai.mcpc.openalex` in…'
      - id: 9HEuwz|iPNgvp
        signature: import logging
      - id: pacGMv|aJFnWk
        signature: "from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolRe…"
      - id: FYap6O|JdAZIY
        signature: "__all__ = ["
      - id: GNlOJg|VGbydf
        signature: logger = logging.getLogger('xy.ai.mcpc.tools.openalex')
      - id: build_client
        signature: "def _build_client(config: ServerConfig) -> OpenAlexClient:"
        docstring: "Build the OpenAlex client once, at registration time."
      - id: clamp
        signature: "def _clamp(value: Any, default: int, maximum: int) -> int:"
      - id: error_result
        signature: "def _error_result(exc: Exception) -> ToolResult:"
      - id: ok_result
        signature: "def _ok_result(structured: dict[str, Any]) -> ToolResult:"
        docstring: "# OpenAlex occasionally returns fields (titles, abstracts, ...) that"
      - id: summarise_list
        signature: "def _summarise_list(data: dict[str, Any]) -> dict[str, Any]:"
      - id: SearchResult
        signature: "@dataclass(frozen=True, slots=True) class SearchResult:"
        docstring: Parsed ``openalex_search`` / ``openalex_semantic_search`` response.
      - id: WorkResult
        signature: "@dataclass(frozen=True, slots=True) class WorkResult:"
        docstring: Parsed ``openalex_work`` response.
      - id: to_search_result
        signature: "def _to_search_result(structured: dict[str, Any], entity: str) -> SearchResult:"
      - id: openalex_search_raw
        signature: "def _openalex_search_raw(query: str, entity: str='works', exact: bool=False, fields: str | None=None, filter: str | None=None, sort: str | None=None, limit: int | None=None) -> dict[str, Any]:"
      - id: openalex_search
        signature: "def openalex_search(query: str, entity: str='works', exact: bool=False, fields: str | None=None, filter: str | None=None, sort: str | None=None, limit: int | None=None) -> SearchResult:"
        docstring: "Keyword/boolean full-text search across OpenAlex scholarly entities. Args: quer…"
      - id: openalex_semantic_search_raw
        signature: "def _openalex_semantic_search_raw(query: str, fields: str | None=None, filter: str | None=None, limit: int | None=None) -> dict[str, Any]:"
      - id: openalex_semantic_search
        signature: "def openalex_semantic_search(query: str, fields: str | None=None, filter: str | None=None, limit: int | None=None) -> SearchResult:"
        docstring: "AI-powered semantic search over OpenAlex works. Args: query: Natural-language d…"
      - id: openalex_work_raw
        signature: "def _openalex_work_raw(id: str, fields: str | None=None) -> dict[str, Any]:"
      - id: openalex_work
        signature: "def openalex_work(id: str, fields: str | None=None) -> WorkResult:"
        docstring: "Fetch a single OpenAlex work by identifier. Args: id: OpenAlex id/URL, DOI (bar…"
      - id: 4ImrFK|YEIfYo
        signature: _SEARCH_DESCRIPTION = 'Keyword and boolean full-text search across OpenAlex sch…
      - id: 4EBREd|pYm2xV
        signature: "_SEARCH_INPUT_SCHEMA: dict[str,"
      - id: X8kvyk|4obZY7
        signature: _SEMANTIC_SEARCH_DESCRIPTION = 'AI-powered semantic search over OpenAlex works:…
      - id: zig7No|wiVdgv
        signature: "_SEMANTIC_INPUT_SCHEMA: dict[str,"
      - id: tgzTNf|YLu9c4
        signature: _WORK_DESCRIPTION = 'Fetch a single OpenAlex work by identifier. Accepts an Ope…
      - id: Rjg0YT|YCl7pk
        signature: "_WORK_INPUT_SCHEMA: dict[str,"
      - id: OpenalexSearchTool
        signature: "class OpenalexSearchTool(ToolDefinition):"
        children:
        - id: OpenalexSearchTool.WsRjVz|pCdxoy
          signature: name = 'openalex_search'
        - id: OpenalexSearchTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
      - id: OpenalexSemanticSearchTool
        signature: "class OpenalexSemanticSearchTool(ToolDefinition):"
        children:
        - id: OpenalexSemanticSearchTool.rW4mOE|WhSE3M
          signature: name = 'openalex_semantic_search'
        - id: OpenalexSemanticSearchTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
      - id: OpenalexWorkTool
        signature: "class OpenalexWorkTool(ToolDefinition):"
        children:
        - id: OpenalexWorkTool.B3vAMU|9O2a7O
          signature: name = 'openalex_work'
        - id: OpenalexWorkTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
      - id: register_openalex_tools
        signature: "def register_openalex_tools(registry: ToolRegistry, environment: AppEnvironment) -> None:"
        docstring: Register the three OpenAlex tools onto *registry*.
```
/answer e694-2 allow
Result Stats: id=b2fc706a-1e14-4ecb-bfb6-dfbb76ef6b0b, total: 0, in: 0, out: 0, reason: 13, read: 0, write: 0
ReasoningToken: 13
Control Request:
```yaml
id: e694-3
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/__init__.py
    ids:
    - 322ZBs|Sp8Hzo
    - 9HEuwz|iPNgvp
    - pacGMv|aJFnWk
    - FYap6O|JdAZIY
    - GNlOJg|VGbydf
    - build_client
    - clamp
    - error_result
    - ok_result
    - summarise_list
    - SearchResult
    - WorkResult
    - to_search_result
    - openalex_search_raw
    - openalex_search
    - openalex_semantic_search_raw
    - openalex_semantic_search
    - openalex_work_raw
    - openalex_work
    - 4ImrFK|YEIfYo
    - 4EBREd|pYm2xV
    - X8kvyk|4obZY7
    - zig7No|wiVdgv
    - tgzTNf|YLu9c4
    - Rjg0YT|YCl7pk
    - OpenalexSearchTool
    - OpenalexSemanticSearchTool
    - OpenalexWorkTool
    - register_openalex_tools
  reason: "Vollständigen Quellcode aller Knoten lesen, um Datei sinnvoll aufzuteilen"
```
/answer e694-3 allow
Result Stats: id=b2fc706a-1e14-4ecb-bfb6-dfbb76ef6b0b, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
Control Request:
```yaml
id: e694-4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/__init__.py
      nodes:
      - id: 322ZBs|Sp8Hzo
        type: statements
        code: "\"OpenAlex tools.\\n\\nThree tools sit on top of the :mod:`xy.ai.mcpc.openalex` interface package and\\napply standard assumptions so an AI agent can use OpenAlex without knowing the\\nraw API:\\n\\n* ``openalex_search``          – keyword / boolean full-text search.\\n* ``openalex_semantic_search`` – AI (embedding) search by meaning.\\n* ``openalex_work``            – fetch a single work by id / DOI.\\n\\nShared conventions\\n------------------\\n* **First page only.** Results are always page 1; ``limit`` controls how many\\n  hits come back (paging deeper is intentionally not exposed).\\n* **Field presets.** Instead of raw ``select`` fields, callers pick a semantic\\n  preset (see :mod:`xy.ai.mcpc.openalex.presets`).\\n* **Readable abstracts.** OpenAlex's ``abstract_inverted_index`` is rebuilt into\\n  a plain-text ``abstract`` field.\\n* **Authentication.** The API key (``MCPC_OPENALEX_KEY``) and optional\\n  ``mailto`` come from the server config and are handled by the client.\\n\""
      - id: 9HEuwz|iPNgvp
        type: imports
        code: |-
          import logging
          from dataclasses import dataclass
          from typing import Any
          from xy.ai.mcpc.config import ServerConfig
          from xy.ai.mcpc.openalex import DEFAULT_SEARCH_PRESET, DEFAULT_WORK_PRESET, GroupByItem, OpenAlexAPIError, OpenAlexClient, OpenAlexError, OpenAlexRecord, Work, parse_entity, parse_group_by, project_results, resolve_select
          from xy.ai.mcpc.openalex.client import ENTITIES
          from xy.ai.mcpc.openalex.presets import WORK_PRESET_NAMES
      - id: pacGMv|aJFnWk
        type: imports
        code: |-
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
          from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
          from xy.ai.mcpc.utils.text_sanitize import sanitize_value
      - id: FYap6O|JdAZIY
        type: statements
        code: |-
          __all__ = [
              'SearchResult',
              'WorkResult',
              'openalex_search',
              'openalex_semantic_search',
              'openalex_work',
              'OpenalexSearchTool',
              'OpenalexSemanticSearchTool',
              'OpenalexWorkTool',
              'register_openalex_tools']
          '#: Hard caps that mirror the OpenAlex API limits.'
          _MAX_PER_PAGE = 50
          _MAX_SEMANTIC_RESULTS = 50
          _DEFAULT_SEARCH_LIMIT = 25
          _DEFAULT_SEMANTIC_LIMIT = 10
          _ENTITY_NAMES = sorted(ENTITIES)
          _WORK_PRESETS = list(WORK_PRESET_NAMES)
      - id: GNlOJg|VGbydf
        type: statements
        code: |-
          logger = logging.getLogger('xy.ai.mcpc.tools.openalex')
          _client: OpenAlexClient | None = None
      - id: build_client
        type: FunctionDef
        code: |-
          def _build_client(config: ServerConfig) -> OpenAlexClient:
              """Build the OpenAlex client once, at registration time.
              """
              return OpenAlexClient(
                  api_key=config.openalex_api_key,
                  base_url=config.openalex_base_url,
                  mailto=config.openalex_mailto)
      - id: clamp
        type: FunctionDef
        code: |-
          def _clamp(value: Any, default: int, maximum: int) -> int:
              try:
                  number = int(value)
              except (TypeError, ValueError):
                  return default
              return max(1, min(number, maximum))
      - id: error_result
        type: FunctionDef
        code: |-
          def _error_result(exc: Exception) -> ToolResult:
              message = str(exc)
              if isinstance(exc, OpenAlexAPIError) and exc.status is not None:
                  message = f'{message} (status {exc.status})'
              logger.error('OpenAlex request failed: %s', message, exc_info=exc)
              return ToolResult(content=[text_content(message)], is_error=True)
      - id: ok_result
        type: FunctionDef
        code: |-
          def _ok_result(structured: dict[str, Any]) -> ToolResult:
              """# OpenAlex occasionally returns fields (titles, abstracts, ...) that"""
              '# contain raw non-printable control characters; strip them so downstream'
              '# consumers (notably YAML block-scalar rendering) never choke on them.'
              return ToolResult(structured_content=sanitize_value(structured))
      - id: summarise_list
        type: FunctionDef
        code: |-
          def _summarise_list(data: dict[str, Any]) -> dict[str, Any]:
              meta = data.get('meta') or {}
              results = project_results(data.get('results') or [])
              structured: dict[str, Any] = {'count': meta.get('count'), 'returned': len(
                  results), 'page': meta.get('page'), 'per_page': meta.get('per_page'), 'results': results}
              if data.get('group_by'):
                  structured['group_by'] = data['group_by']
              return structured
      - id: SearchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True, slots=True)
          class SearchResult:
              """Parsed ``openalex_search`` / ``openalex_semantic_search`` response."""
              count: int | None
              returned: int
              page: int | None
              per_page: int | None
              results: list[OpenAlexRecord]
              group_by: list[GroupByItem] | None = None
      - id: WorkResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True, slots=True)
          class WorkResult:
              """Parsed ``openalex_work`` response."""
              work: Work
      - id: to_search_result
        type: FunctionDef
        code: |-
          def _to_search_result(structured: dict[str, Any], entity: str) -> SearchResult:
              group_by = structured.get('group_by')
              return SearchResult(
                  count=structured.get('count'),
                  returned=structured.get(
                      'returned',
                      0),
                  page=structured.get('page'),
                  per_page=structured.get('per_page'),
                  results=[
                      parse_entity(
                          entity,
                          item) for item in structured.get(
                          'results',
                          [])],
                  group_by=parse_group_by(group_by) if group_by else None)
      - id: openalex_search_raw
        type: FunctionDef
        code: |-
          def _openalex_search_raw(query: str, entity: str='works', exact: bool=False, fields: str | None=None, filter: str | None=None, sort: str | None=None, limit: int | None=None) -> dict[str, Any]:
              client = _client
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
                  page=1) if entity == 'works' else client.list_entities(
                      entity,
                      search_exact=query if exact else None,
                      search=None if exact else query,
                      filters=filter,
                      sort=sort,
                      select=select,
                      per_page=resolved_limit,
                  page=1)
              return _summarise_list(data)
      - id: openalex_search
        type: FunctionDef
        code: |-
          def openalex_search(query: str, entity: str='works', exact: bool=False, fields: str | None=None, filter: str | None=None, sort: str | None=None, limit: int | None=None) -> SearchResult:
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
                  Parsed search results, one dataclass per *entity* record.

              Raises:
                  OpenAlexError: If the OpenAlex API request fails.
              """
              structured = _openalex_search_raw(
                  query,
                  entity=entity,
                  exact=exact,
                  fields=fields,
                  filter=filter,
                  sort=sort,
                  limit=limit)
              return _to_search_result(structured, entity)
      - id: openalex_semantic_search_raw
        type: FunctionDef
        code: |-
          def _openalex_semantic_search_raw(query: str, fields: str | None=None, filter: str | None=None, limit: int | None=None) -> dict[str, Any]:
              client = _client
              preset = fields or DEFAULT_SEARCH_PRESET
              resolved_limit = _clamp(limit, _DEFAULT_SEMANTIC_LIMIT, _MAX_SEMANTIC_RESULTS)
              select = resolve_select(preset, 'works')
              data = client.semantic_search_works(query, filters=filter, select=select, per_page=resolved_limit, page=1)
              return _summarise_list(data)
      - id: openalex_semantic_search
        type: FunctionDef
        code: |-
          def openalex_semantic_search(query: str, fields: str | None=None, filter: str | None=None, limit: int | None=None) -> SearchResult:
              """AI-powered semantic search over OpenAlex works.

              Args:
                  query: Natural-language description of what to look for.
                  fields: Field preset for each work (default: core).
                  filter: Optional OpenAlex filter string (no cited_by_count/country_code).
                  limit: Max results (1-50, default 10).

              Returns:
                  Parsed search results (:class:`~xy.ai.mcpc.openalex.Work` records).

              Raises:
                  OpenAlexError: If the OpenAlex API request fails.
              """
              structured = _openalex_semantic_search_raw(query, fields=fields, filter=filter, limit=limit)
              return _to_search_result(structured, 'works')
      - id: openalex_work_raw
        type: FunctionDef
        code: |-
          def _openalex_work_raw(id: str, fields: str | None=None) -> dict[str, Any]:
              client = _client
              preset = fields or DEFAULT_WORK_PRESET
              select = resolve_select(preset, 'works')
              data = client.get_work(id, select=select)
              work = project_results([data])[0]
              return {'work': work}
      - id: openalex_work
        type: FunctionDef
        code: |-
          def openalex_work(id: str, fields: str | None=None) -> WorkResult:
              """Fetch a single OpenAlex work by identifier.

              Args:
                  id: OpenAlex id/URL, DOI (bare or URL), or namespaced id (pmid:, mag:, ...).
                  fields: Field preset (default: full).

              Returns:
                  The parsed work record.

              Raises:
                  OpenAlexError: If the OpenAlex API request fails.
              """
              structured = _openalex_work_raw(id, fields=fields)
              return WorkResult(work=parse_entity('works', structured['work']))
      - id: 4ImrFK|YEIfYo
        type: statements
        code: "_SEARCH_DESCRIPTION = 'Keyword and boolean full-text search across OpenAlex scholarly entities (works by default). Searches titles, abstracts and full text for works; names for authors, sources and institutions.\\n\\nQuery syntax: use uppercase AND / OR / NOT and double-quoted phrases, e.g. (\"machine learning\" OR \"deep learning\") NOT survey. Set exact=true for unstemmed matching and wildcards (machin*). Results are sorted by relevance and limited to the first page.'"
      - id: 4EBREd|pYm2xV
        type: statements
        code: |-
          _SEARCH_INPUT_SCHEMA: dict[str,
                                     Any] = {'type': 'object',
                                             'properties': {'query': {'type': 'string',
                                                                      'description': 'Full-text query. Supports boolean AND/OR/NOT (uppercase), quoted phrases and proximity ("a b"~5).'},
                                                            'entity': {'type': 'string',
                                                                       'enum': _ENTITY_NAMES,
                                                                       'description': 'Entity type to search (default: works).'},
                                                            'exact': {'type': 'boolean',
                                                                      'description': 'Use exact (unstemmed) search; required for wildcards like machin*. Default: false.'},
                                                            'fields': {'type': 'string',
                                                                       'enum': _WORK_PRESETS,
                                                                       'description': 'Field preset controlling how much of each record is returned (works only). Default: core. Presets: minimal, core, bibliographic, authorship, access, metrics, topics, abstract, references, full.'},
                                                            'filter': {'type': 'string',
                                                                       'description': "Optional OpenAlex filter string applied alongside the search, e.g. 'publication_year:>2020,is_oa:true'. Comma-separated key:value pairs."},
                                                            'sort': {'type': 'string',
                                                                     'description': "Optional sort override, e.g. 'cited_by_count:desc' or 'publication_date:desc'. Defaults to relevance."},
                                                            'limit': {'type': 'integer',
                                                                      'description': f'Max results from the first page (1-{_MAX_PER_PAGE}, default {_DEFAULT_SEARCH_LIMIT}).',
                                                                      'minimum': 1,
                                                                      'maximum': _MAX_PER_PAGE}},
                                             'required': ['query']}
      - id: X8kvyk|4obZY7
        type: statements
        code: "_SEMANTIC_SEARCH_DESCRIPTION = 'AI-powered semantic search over OpenAlex works: finds works by meaning using embeddings, even when the wording differs. Best for paragraph-length input such as an abstract, a research question or a grant aim (up to ~2000 characters).\\n\\nSupports most filters, but NOT cited_by_count or country_code filters. Returns at most 50 works from the first page, ranked by semantic similarity.'"
      - id: zig7No|wiVdgv
        type: statements
        code: |-
          _SEMANTIC_INPUT_SCHEMA: dict[str,
                                       Any] = {'type': 'object',
                                               'properties': {'query': {'type': 'string',
                                                                        'description': 'Natural-language description of what you are looking for. Longer, richer input yields better matches (truncated at 2000 characters).'},
                                                              'fields': {'type': 'string',
                                                                         'enum': _WORK_PRESETS,
                                                                         'description': 'Field preset for each work (default: core).'},
                                                              'filter': {'type': 'string',
                                                                         'description': "Optional OpenAlex filter string, e.g. 'publication_year:>2020,is_oa:true'. Note: cited_by_count and country_code filters are not supported by semantic search."},
                                                              'limit': {'type': 'integer',
                                                                        'description': f'Max results (1-{_MAX_SEMANTIC_RESULTS}, default {_DEFAULT_SEMANTIC_LIMIT}).',
                                                                        'minimum': 1,
                                                                        'maximum': _MAX_SEMANTIC_RESULTS}},
                                               'required': ['query']}
      - id: tgzTNf|YLu9c4
        type: statements
        code: "_WORK_DESCRIPTION = 'Fetch a single OpenAlex work by identifier. Accepts an OpenAlex id (W2741809807), an OpenAlex URL, a DOI (10.7717/peerj.4375 or a doi.org URL) or a namespaced id such as pmid:14907713. Returns the full record by default, with the abstract reconstructed to plain text.'"
      - id: Rjg0YT|YCl7pk
        type: statements
        code: |-
          _WORK_INPUT_SCHEMA: dict[str,
                                   Any] = {'type': 'object',
                                           'properties': {'id': {'type': 'string',
                                                                 'description': 'Work identifier: OpenAlex id/URL, DOI (bare or URL), or namespaced id (pmid:, mag:, ...).'},
                                                          'fields': {'type': 'string',
                                                                     'enum': _WORK_PRESETS,
                                                                     'description': 'Field preset (default: full). Use a narrower preset such as bibliographic or abstract to reduce size.'}},
                                           'required': ['id']}
      - id: OpenalexSearchTool
        type: ClassDef
        signature: "class OpenalexSearchTool(ToolDefinition):"
        children:
        - id: OpenalexSearchTool.WsRjVz|pCdxoy
          type: statements
          code: |-
            name = 'openalex_search'
            title = 'OpenAlex search'
            description = _SEARCH_DESCRIPTION
            input_schema = _SEARCH_INPUT_SCHEMA
        - id: OpenalexSearchTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                args = ctx.arguments
                try:
                    structured = _openalex_search_raw(
                        query=args['query'],
                        entity=args.get(
                            'entity',
                            'works'),
                        exact=bool(
                            args.get(
                                'exact',
                                False)),
                        fields=args.get('fields'),
                        filter=args.get('filter'),
                        sort=args.get('sort'),
                        limit=args.get('limit'))
                except OpenAlexError as exc:
                    return _error_result(exc)
                return _ok_result(structured)
      - id: OpenalexSemanticSearchTool
        type: ClassDef
        signature: "class OpenalexSemanticSearchTool(ToolDefinition):"
        children:
        - id: OpenalexSemanticSearchTool.rW4mOE|WhSE3M
          type: statements
          code: |-
            name = 'openalex_semantic_search'
            title = 'OpenAlex semantic search'
            description = _SEMANTIC_SEARCH_DESCRIPTION
            input_schema = _SEMANTIC_INPUT_SCHEMA
        - id: OpenalexSemanticSearchTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                args = ctx.arguments
                try:
                    structured = _openalex_semantic_search_raw(
                        query=args['query'],
                        fields=args.get('fields'),
                        filter=args.get('filter'),
                        limit=args.get('limit'))
                except OpenAlexError as exc:
                    return _error_result(exc)
                return _ok_result(structured)
      - id: OpenalexWorkTool
        type: ClassDef
        signature: "class OpenalexWorkTool(ToolDefinition):"
        children:
        - id: OpenalexWorkTool.B3vAMU|9O2a7O
          type: statements
          code: |-
            name = 'openalex_work'
            title = 'OpenAlex work'
            description = _WORK_DESCRIPTION
            input_schema = _WORK_INPUT_SCHEMA
        - id: OpenalexWorkTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                args = ctx.arguments
                try:
                    structured = _openalex_work_raw(id=args['id'], fields=args.get('fields'))
                except OpenAlexError as exc:
                    return _error_result(exc)
                return _ok_result(structured)
      - id: register_openalex_tools
        type: FunctionDef
        code: |-
          def register_openalex_tools(registry: ToolRegistry, environment: AppEnvironment) -> None:
              """Register the three OpenAlex tools onto *registry*."""
              global _client
              _client = _build_client(environment.config)
              registry.register(OpenalexSearchTool())
              registry.register(OpenalexSemanticSearchTool())
              registry.register(OpenalexWorkTool())
              functions = environment.functions
              if functions is not None:
                  functions.register(openalex_search)
                  functions.register(openalex_semantic_search)
                  functions.register(openalex_work)
```
/answer e694-4 allow
Result Stats: id=b2fc706a-1e14-4ecb-bfb6-dfbb76ef6b0b, total: 0, in: 0, out: 0, reason: 23, read: 0, write: 0
ReasoningToken: 23
Control Request:
```yaml
id: e694-5
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: tools\.openalex
    include:
    - '*.py'
    exclude:
    - node_modules/**
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: _openalex_(search|semantic_search|work)_raw|_build_client|openalex\._client
    include:
    - '*.py'
    exclude:
    - node_modules/**
  reason: "Prüfen, ob andere Module private Namen aus tools/openalex/__init__.py importieren, um Split ohne Breaking Changes durchzuführen"
```
/answer e694-5 allow
Result Stats: id=b2fc706a-1e14-4ecb-bfb6-dfbb76ef6b0b, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
Control Request:
```yaml
id: e694-6
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc
      matches:
      - path: src/xy/ai/mcpc/tools/__init__.py
        matches:
        - lineno: 12
          match: from xy.ai.mcpc.tools.openalex import register_openalex_tools
      - path: src/xy/ai/mcpc/tools/openalex/__init__.py
        matches:
        - lineno: 49
          match: logger = logging.getLogger('xy.ai.mcpc.tools.openalex')
      - path: src/xy/ai/mcpc/openalex/client.py
        matches:
        - lineno: 6
          match: higher-level *tools* layer (:mod:`xy.ai.mcpc.tools.openalex`) builds on top of
      - path: src/xy/ai/mcpc/openalex/__init__.py
        matches:
        - lineno: 12
          match: :mod:`xy.ai.mcpc.tools.openalex`.
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc
      matches:
      - path: src/xy/ai/mcpc/tools/openalex/__init__.py
        matches:
        - lineno: 52
          match: "def _build_client(config: ServerConfig) -> OpenAlexClient:"
        - lineno: 121
          match: "def _openalex_search_raw(query: str, entity: str='works', exact: bool=False, fields: str | None=None, filter: str | None=None, sort: str | None=None, limit: int | None=None) -> dict[str, Any]:"
        - lineno: 162
          match: structured = _openalex_search_raw(
        - lineno: 172
          match: "def _openalex_semantic_search_raw(query: str, fields: str | None=None, filter: str | None=None, limit: int | None=None) -> dict[str, Any]:"
        - lineno: 195
          match: "structured = _openalex_semantic_search_raw(query, fields=fields, filter=filter, limit=limit)"
        - lineno: 198
          match: "def _openalex_work_raw(id: str, fields: str | None=None) -> dict[str, Any]:"
        - lineno: 219
          match: "structured = _openalex_work_raw(id, fields=fields)"
        - lineno: 277
          match: structured = _openalex_search_raw(
        - lineno: 303
          match: structured = _openalex_semantic_search_raw(
        - lineno: 321
          match: "structured = _openalex_work_raw(id=args['id'], fields=args.get('fields'))"
        - lineno: 329
          match: _client = _build_client(environment.config)
```
/answer e694-6 allow
Result Stats: id=b2fc706a-1e14-4ecb-bfb6-dfbb76ef6b0b, total: 0, in: 0, out: 0, reason: 2844, read: 0, write: 0
ReasoningToken: 2844
Text:
 Nur `register_openalex_tools` wird extern importiert. Ich erstelle die Dateien: `_common.py`, `search.py`, `semantic_search.py`, `work.py` und schreibe `__init__.py` neu.
Control Request:
```yaml
id: e694-7
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/_common.py
    source: |
      """Shared client state, dataclasses and helpers for the OpenAlex tools."""

      import logging
      from dataclasses import dataclass
      from typing import Any

      from xy.ai.mcpc.config import ServerConfig
      from xy.ai.mcpc.openalex import (
          GroupByItem,
          OpenAlexAPIError,
          OpenAlexClient,
          OpenAlexRecord,
          Work,
          parse_entity,
          parse_group_by,
          project_results,
      )
      from xy.ai.mcpc.tools.tool_registry import ToolResult, text_content
      from xy.ai.mcpc.utils.text_sanitize import sanitize_value

      logger = logging.getLogger('xy.ai.mcpc.tools.openalex')

      #: Hard caps that mirror the OpenAlex API limits.
      MAX_PER_PAGE = 50
      MAX_SEMANTIC_RESULTS = 50
      DEFAULT_SEARCH_LIMIT = 25
      DEFAULT_SEMANTIC_LIMIT = 10

      _client: OpenAlexClient | None = None


      def build_client(config: ServerConfig) -> OpenAlexClient:
          """Build the OpenAlex client once, at registration time."""
          return OpenAlexClient(
              api_key=config.openalex_api_key,
              base_url=config.openalex_base_url,
              mailto=config.openalex_mailto)


      def set_client(client: OpenAlexClient) -> None:
          global _client
          _client = client


      def get_client() -> OpenAlexClient:
          return _client


      def clamp(value: Any, default: int, maximum: int) -> int:
          try:
              number = int(value)
          except (TypeError, ValueError):
              return default
          return max(1, min(number, maximum))


      def error_result(exc: Exception) -> ToolResult:
          message = str(exc)
          if isinstance(exc, OpenAlexAPIError) and exc.status is not None:
              message = f'{message} (status {exc.status})'
          logger.error('OpenAlex request failed: %s', message, exc_info=exc)
          return ToolResult(content=[text_content(message)], is_error=True)


      def ok_result(structured: dict[str, Any]) -> ToolResult:
          # OpenAlex occasionally returns fields (titles, abstracts, ...) that
          # contain raw non-printable control characters; strip them so downstream
          # consumers (notably YAML block-scalar rendering) never choke on them.
          return ToolResult(structured_content=sanitize_value(structured))


      def summarise_list(data: dict[str, Any]) -> dict[str, Any]:
          meta = data.get('meta') or {}
          results = project_results(data.get('results') or [])
          structured: dict[str, Any] = {
              'count': meta.get('count'),
              'returned': len(results),
              'page': meta.get('page'),
              'per_page': meta.get('per_page'),
              'results': results}
          if data.get('group_by'):
              structured['group_by'] = data['group_by']
          return structured


      @dataclass(frozen=True, slots=True)
      class SearchResult:
          """Parsed ``openalex_search`` / ``openalex_semantic_search`` response."""
          count: int | None
          returned: int
          page: int | None
          per_page: int | None
          results: list[OpenAlexRecord]
          group_by: list[GroupByItem] | None = None


      @dataclass(frozen=True, slots=True)
      class WorkResult:
          """Parsed ``openalex_work`` response."""
          work: Work


      def to_search_result(structured: dict[str, Any], entity: str) -> SearchResult:
          group_by = structured.get('group_by')
          return SearchResult(
              count=structured.get('count'),
              returned=structured.get('returned', 0),
              page=structured.get('page'),
              per_page=structured.get('per_page'),
              results=[parse_entity(entity, item) for item in structured.get('results', [])],
              group_by=parse_group_by(group_by) if group_by else None)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/search.py
    source: |
      """``openalex_search`` tool: keyword/boolean full-text search."""

      from typing import Any

      from xy.ai.mcpc.openalex import DEFAULT_SEARCH_PRESET, OpenAlexError, resolve_select
      from xy.ai.mcpc.openalex.client import ENTITIES
      from xy.ai.mcpc.openalex.presets import WORK_PRESET_NAMES
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolResult

      from ._common import (
          DEFAULT_SEARCH_LIMIT,
          MAX_PER_PAGE,
          SearchResult,
          clamp,
          error_result,
          get_client,
          ok_result,
          summarise_list,
          to_search_result,
      )

      _ENTITY_NAMES = sorted(ENTITIES)
      _WORK_PRESETS = list(WORK_PRESET_NAMES)

      _SEARCH_DESCRIPTION = 'Keyword and boolean full-text search across OpenAlex scholarly entities (works by default). Searches titles, abstracts and full text for works; names for authors, sources and institutions.\n\nQuery syntax: use uppercase AND / OR / NOT and double-quoted phrases, e.g. ("machine learning" OR "deep learning") NOT survey. Set exact=true for unstemmed matching and wildcards (machin*). Results are sorted by relevance and limited to the first page.'

      _SEARCH_INPUT_SCHEMA: dict[str, Any] = {
          'type': 'object',
          'properties': {
              'query': {
                  'type': 'string',
                  'description': 'Full-text query. Supports boolean AND/OR/NOT (uppercase), quoted phrases and proximity ("a b"~5).'},
              'entity': {
                  'type': 'string',
                  'enum': _ENTITY_NAMES,
                  'description': 'Entity type to search (default: works).'},
              'exact': {
                  'type': 'boolean',
                  'description': 'Use exact (unstemmed) search; required for wildcards like machin*. Default: false.'},
              'fields': {
                  'type': 'string',
                  'enum': _WORK_PRESETS,
                  'description': 'Field preset controlling how much of each record is returned (works only). Default: core. Presets: minimal, core, bibliographic, authorship, access, metrics, topics, abstract, references, full.'},
              'filter': {
                  'type': 'string',
                  'description': "Optional OpenAlex filter string applied alongside the search, e.g. 'publication_year:>2020,is_oa:true'. Comma-separated key:value pairs."},
              'sort': {
                  'type': 'string',
                  'description': "Optional sort override, e.g. 'cited_by_count:desc' or 'publication_date:desc'. Defaults to relevance."},
              'limit': {
                  'type': 'integer',
                  'description': f'Max results from the first page (1-{MAX_PER_PAGE}, default {DEFAULT_SEARCH_LIMIT}).',
                  'minimum': 1,
                  'maximum': MAX_PER_PAGE}},
          'required': ['query']}


      def _openalex_search_raw(query: str, entity: str='works', exact: bool=False, fields: str | None=None, filter: str | None=None, sort: str | None=None, limit: int | None=None) -> dict[str, Any]:
          client = get_client()
          preset = fields or DEFAULT_SEARCH_PRESET
          resolved_limit = clamp(limit, DEFAULT_SEARCH_LIMIT, MAX_PER_PAGE)
          select = resolve_select(preset, entity)
          data = client.search_works(
              query,
              exact=exact,
              filters=filter,
              sort=sort,
              select=select,
              per_page=resolved_limit,
              page=1) if entity == 'works' else client.list_entities(
                  entity,
                  search_exact=query if exact else None,
                  search=None if exact else query,
                  filters=filter,
                  sort=sort,
                  select=select,
                  per_page=resolved_limit,
                  page=1)
          return summarise_list(data)


      def openalex_search(query: str, entity: str='works', exact: bool=False, fields: str | None=None, filter: str | None=None, sort: str | None=None, limit: int | None=None) -> SearchResult:
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
              Parsed search results, one dataclass per *entity* record.

          Raises:
              OpenAlexError: If the OpenAlex API request fails.
          """
          structured = _openalex_search_raw(
              query,
              entity=entity,
              exact=exact,
              fields=fields,
              filter=filter,
              sort=sort,
              limit=limit)
          return to_search_result(structured, entity)


      class OpenalexSearchTool(ToolDefinition):
          name = 'openalex_search'
          title = 'OpenAlex search'
          description = _SEARCH_DESCRIPTION
          input_schema = _SEARCH_INPUT_SCHEMA

          def handle(self, ctx: ToolContext) -> ToolResult:
              args = ctx.arguments
              try:
                  structured = _openalex_search_raw(
                      query=args['query'],
                      entity=args.get('entity', 'works'),
                      exact=bool(args.get('exact', False)),
                      fields=args.get('fields'),
                      filter=args.get('filter'),
                      sort=args.get('sort'),
                      limit=args.get('limit'))
              except OpenAlexError as exc:
                  return error_result(exc)
              return ok_result(structured)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/semantic_search.py
    source: |
      """``openalex_semantic_search`` tool: AI (embedding) search by meaning."""

      from typing import Any

      from xy.ai.mcpc.openalex import DEFAULT_SEARCH_PRESET, OpenAlexError, resolve_select
      from xy.ai.mcpc.openalex.presets import WORK_PRESET_NAMES
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolResult

      from ._common import (
          DEFAULT_SEMANTIC_LIMIT,
          MAX_SEMANTIC_RESULTS,
          SearchResult,
          clamp,
          error_result,
          get_client,
          ok_result,
          summarise_list,
          to_search_result,
      )

      _WORK_PRESETS = list(WORK_PRESET_NAMES)

      _SEMANTIC_SEARCH_DESCRIPTION = 'AI-powered semantic search over OpenAlex works: finds works by meaning using embeddings, even when the wording differs. Best for paragraph-length input such as an abstract, a research question or a grant aim (up to ~2000 characters).\n\nSupports most filters, but NOT cited_by_count or country_code filters. Returns at most 50 works from the first page, ranked by semantic similarity.'

      _SEMANTIC_INPUT_SCHEMA: dict[str, Any] = {
          'type': 'object',
          'properties': {
              'query': {
                  'type': 'string',
                  'description': 'Natural-language description of what you are looking for. Longer, richer input yields better matches (truncated at 2000 characters).'},
              'fields': {
                  'type': 'string',
                  'enum': _WORK_PRESETS,
                  'description': 'Field preset for each work (default: core).'},
              'filter': {
                  'type': 'string',
                  'description': "Optional OpenAlex filter string, e.g. 'publication_year:>2020,is_oa:true'. Note: cited_by_count and country_code filters are not supported by semantic search."},
              'limit': {
                  'type': 'integer',
                  'description': f'Max results (1-{MAX_SEMANTIC_RESULTS}, default {DEFAULT_SEMANTIC_LIMIT}).',
                  'minimum': 1,
                  'maximum': MAX_SEMANTIC_RESULTS}},
          'required': ['query']}


      def _openalex_semantic_search_raw(query: str, fields: str | None=None, filter: str | None=None, limit: int | None=None) -> dict[str, Any]:
          client = get_client()
          preset = fields or DEFAULT_SEARCH_PRESET
          resolved_limit = clamp(limit, DEFAULT_SEMANTIC_LIMIT, MAX_SEMANTIC_RESULTS)
          select = resolve_select(preset, 'works')
          data = client.semantic_search_works(query, filters=filter, select=select, per_page=resolved_limit, page=1)
          return summarise_list(data)


      def openalex_semantic_search(query: str, fields: str | None=None, filter: str | None=None, limit: int | None=None) -> SearchResult:
          """AI-powered semantic search over OpenAlex works.

          Args:
              query: Natural-language description of what to look for.
              fields: Field preset for each work (default: core).
              filter: Optional OpenAlex filter string (no cited_by_count/country_code).
              limit: Max results (1-50, default 10).

          Returns:
              Parsed search results (:class:`~xy.ai.mcpc.openalex.Work` records).

          Raises:
              OpenAlexError: If the OpenAlex API request fails.
          """
          structured = _openalex_semantic_search_raw(query, fields=fields, filter=filter, limit=limit)
          return to_search_result(structured, 'works')


      class OpenalexSemanticSearchTool(ToolDefinition):
          name = 'openalex_semantic_search'
          title = 'OpenAlex semantic search'
          description = _SEMANTIC_SEARCH_DESCRIPTION
          input_schema = _SEMANTIC_INPUT_SCHEMA

          def handle(self, ctx: ToolContext) -> ToolResult:
              args = ctx.arguments
              try:
                  structured = _openalex_semantic_search_raw(
                      query=args['query'],
                      fields=args.get('fields'),
                      filter=args.get('filter'),
                      limit=args.get('limit'))
              except OpenAlexError as exc:
                  return error_result(exc)
              return ok_result(structured)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/work.py
    source: |
      """``openalex_work`` tool: fetch a single work by id / DOI."""

      from typing import Any

      from xy.ai.mcpc.openalex import DEFAULT_WORK_PRESET, OpenAlexError, parse_entity, project_results, resolve_select
      from xy.ai.mcpc.openalex.presets import WORK_PRESET_NAMES
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolResult

      from ._common import WorkResult, error_result, get_client, ok_result

      _WORK_PRESETS = list(WORK_PRESET_NAMES)

      _WORK_DESCRIPTION = 'Fetch a single OpenAlex work by identifier. Accepts an OpenAlex id (W2741809807), an OpenAlex URL, a DOI (10.7717/peerj.4375 or a doi.org URL) or a namespaced id such as pmid:14907713. Returns the full record by default, with the abstract reconstructed to plain text.'

      _WORK_INPUT_SCHEMA: dict[str, Any] = {
          'type': 'object',
          'properties': {
              'id': {
                  'type': 'string',
                  'description': 'Work identifier: OpenAlex id/URL, DOI (bare or URL), or namespaced id (pmid:, mag:, ...).'},
              'fields': {
                  'type': 'string',
                  'enum': _WORK_PRESETS,
                  'description': 'Field preset (default: full). Use a narrower preset such as bibliographic or abstract to reduce size.'}},
          'required': ['id']}


      def _openalex_work_raw(id: str, fields: str | None=None) -> dict[str, Any]:
          client = get_client()
          preset = fields or DEFAULT_WORK_PRESET
          select = resolve_select(preset, 'works')
          data = client.get_work(id, select=select)
          work = project_results([data])[0]
          return {'work': work}


      def openalex_work(id: str, fields: str | None=None) -> WorkResult:
          """Fetch a single OpenAlex work by identifier.

          Args:
              id: OpenAlex id/URL, DOI (bare or URL), or namespaced id (pmid:, mag:, ...).
              fields: Field preset (default: full).

          Returns:
              The parsed work record.

          Raises:
              OpenAlexError: If the OpenAlex API request fails.
          """
          structured = _openalex_work_raw(id, fields=fields)
          return WorkResult(work=parse_entity('works', structured['work']))


      class OpenalexWorkTool(ToolDefinition):
          name = 'openalex_work'
          title = 'OpenAlex work'
          description = _WORK_DESCRIPTION
          input_schema = _WORK_INPUT_SCHEMA

          def handle(self, ctx: ToolContext) -> ToolResult:
              args = ctx.arguments
              try:
                  structured = _openalex_work_raw(id=args['id'], fields=args.get('fields'))
              except OpenAlexError as exc:
                  return error_result(exc)
              return ok_result(structured)
  reason: Neue Dateien pro Tool erstellen (gemeinsame Basis + search/semantic_search/work)
```
/answer e694-7 allow
Result Stats: id=b2fc706a-1e14-4ecb-bfb6-dfbb76ef6b0b, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
Control Request:
```yaml
id: e694-8
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/__init__.py
  reason: "Alten Inhalt der __init__.py entfernen, um sie durch das schlanke Register-Modul zu ersetzen"
```
/answer e694-8 allow
Result Stats: id=b2fc706a-1e14-4ecb-bfb6-dfbb76ef6b0b, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
Control Request:
```yaml
id: e694-9
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/__init__.py
    source: |
      """OpenAlex tools.

      Three tools sit on top of the :mod:`xy.ai.mcpc.openalex` interface package and
      apply standard assumptions so an AI agent can use OpenAlex without knowing the
      raw API:

      * ``openalex_search``          – keyword / boolean full-text search.
      * ``openalex_semantic_search`` – AI (embedding) search by meaning.
      * ``openalex_work``            – fetch a single work by id / DOI.

      Each tool lives in its own module (:mod:`.search`, :mod:`.semantic_search`,
      :mod:`.work`); shared client state and helpers live in :mod:`._common`.

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

      from xy.ai.mcpc.tools.tool_context import AppEnvironment
      from xy.ai.mcpc.tools.tool_registry import ToolRegistry

      from ._common import SearchResult, WorkResult, build_client, set_client
      from .search import OpenalexSearchTool, openalex_search
      from .semantic_search import OpenalexSemanticSearchTool, openalex_semantic_search
      from .work import OpenalexWorkTool, openalex_work

      __all__ = [
          'SearchResult',
          'WorkResult',
          'openalex_search',
          'openalex_semantic_search',
          'openalex_work',
          'OpenalexSearchTool',
          'OpenalexSemanticSearchTool',
          'OpenalexWorkTool',
          'register_openalex_tools']


      def register_openalex_tools(registry: ToolRegistry, environment: AppEnvironment) -> None:
          """Register the three OpenAlex tools onto *registry*."""
          set_client(build_client(environment.config))
          registry.register(OpenalexSearchTool())
          registry.register(OpenalexSemanticSearchTool())
          registry.register(OpenalexWorkTool())
          functions = environment.functions
          if functions is not None:
              functions.register(openalex_search)
              functions.register(openalex_semantic_search)
              functions.register(openalex_work)
  reason: "Schlanke __init__.py erstellen, die Tools aus den neuen Modulen importiert und registriert"
```
/answer e694-9 allow
Result Stats: id=b2fc706a-1e14-4ecb-bfb6-dfbb76ef6b0b, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
Control Request:
```yaml
id: e694-a
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/_common.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/search.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/semantic_search.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/work.py
  reason: Syntaktische Korrektheit aller neuen/geänderten Dateien prüfen
```
/answer e694-a allow
Result Stats: id=b2fc706a-1e14-4ecb-bfb6-dfbb76ef6b0b, total: 43205, in: 147, out: 13036, reason: 0, read: 134767, write: 30022
Text:
 Jetzt `__init__.py` neu schreiben:
Alle Dateien validieren erfolgreich. Die Datei wurde in fünf Module aufgeteilt:

- `_common.py` – Client-State, Dataclasses (`SearchResult`, `WorkResult`) und Hilfsfunktionen
- `search.py` – `openalex_search` Tool
- `semantic_search.py` – `openalex_semantic_search` Tool
- `work.py` – `openalex_work` Tool
- `__init__.py` – nur noch Re-Exports und `register_openalex_tools`
