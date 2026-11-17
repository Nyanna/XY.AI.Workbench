Ich möchte das im Project `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc` eine einheitliche Methodik und Verhalten für Warnungen (MCP Tools) Verwendung findet. Analysiere und mache einen Vorschlag.

- Die Feldbenamung soll vereinheitlicht werden (Vielleicht gemeinsame ABC).
- Es soll getrennt werden zwischen Warnungen in Batch Items und auf Resultebene
- Das ganze soll nach Möglichkeit durch Code forciert sein, keine reine Konvention
- Result oder Error Objekte sollen nach Möglichkeit durch eine ABC Hierarchie abgebildet werden, die die Konvention erzwingt.
- Beachte das eine reine Textsuche dir alles vollspammen wird Beispielweise mit Logging ("logger.warning") das hier ausdrücklich nicht gemeint ist
SystemInit:  id=43112839-11be-4621-8d0d-cda5af1a91db, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Text:
 Ich verschaffe mir zuerst einen Überblick über die Projektstruktur und die Stellen, an denen Warnungen in Results vorkommen.
Control Request:
```yaml
id: 6bd4-2-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
    pattern: warnings?\b
    include:
    - '*.py'
    exclude:
    - '**/__pycache__/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
    pattern: ^class \w+\(.*(BaseModel|ABC|TypedDict|Protocol|Generic).*\)
    include:
    - '*.py'
    exclude:
    - '**/__pycache__/**'
    limit: 50
  reason: Find fields/identifiers named warning(s) (not logger calls) and the existing model/ABC class landscape.
```
/answer 6bd4-2-0 allow
Control Request:
```yaml
id: 6bd4-3-2
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
      matches:
      - path: control/handler.py
        matches:
        - "logger.warning('Control: manager not enabled, returning 404')"
        - "logger.warning('Control: invalid JSON body: %s', exc)"
        - "logger.warning(\"Control: 'approvals' is not a list: %r\", approvals)"
      - path: control/manager.py
        matches:
        - "logger.warning(\"Approval entry missing 'id', skipped: %s\", approval)"
        - "logger.warning('Unknown approval id %s, skipped', item_id)"
        - "logger.warning('Control item %s [%s/%s] timed out, auto-approving', item.tool_name, item.phase, item.id)"
      - path: server/server.py
        matches:
        - Returns ``None`` (after logging a warning) when the transport is disabled
        - "logger.warning('WebSocket transport unavailable: %s', exc)"
      - path: server/ws_transport.py
        matches:
        - "logger.warning('WS: unknown endpoint %s', parsed.path)"
        - "logger.warning('WS: origin forbidden: %s', origin)"
        - "logger.warning('WS: session id missing')"
        - "logger.warning('WS: session id is not a valid UUID')"
      - path: tools/mcp/exa/fetch_results.py
        matches:
        - "logger.warning('web_fetch_exa_results: unknown or expired id(s): %s', missing)"
        - "logger.warning('web_fetch_exa_results: invalid pattern %r: %s', args.get('pattern'), exc)"
      - path: tools/mcp/exa/fetch.py
        matches:
        - "logger.warning('web_fetch_exa: no \"# title / URL:\" markers found in fallback text, cannot split into per-url items')"
        - "logger.warning('web_fetch_exa failed: %s', exc)"
      - path: tools/mcp/exa/search.py
        matches:
        - logger.warning(
        - "logger.warning('web_search_exa failed: %s', exc)"
      - path: tools/mcp/exa/search_results.py
        matches:
        - "logger.warning('web_search_exa_results: unknown or expired id(s): %s', missing)"
      - path: tools/mcp/exa/core.py
        matches:
        - logger.warning(
        - "logger.warning('Exa result item is not a dict (got %s), skipping its fields: %r', type(raw).__name__, raw)"
      - path: tools/ast/outline.py
        matches:
        - "if result.warning:"
        - parts.append(result.warning)
      - path: tools/ast/read.py
        matches:
        - "warning = f'ast_read fell back to ast_outline: ids {"
        - "entry['warning'] = [warning]"
      - path: tools/ast/core.py
        matches:
        - "logger.warning('Import handling failed for %s', tree.path, exc_info=True)"
      - path: tools/openalex/search_results.py
        matches:
        - "logger.warning('openalex_search_results: unknown or expired id(s): %s', missing)"
      - path: tools/list/__init__.py
        matches:
        - "warning: str | None = None"
        - '''# file count below each). Both reduced results are returned with a warning.'''
        - "warning = f\"Result reduced because the total number of entries ({match_count}) exceeds the limit of {_MAX_ENTRIES}. Narrow down the result using the 'pattern' regular expression parameter.\""
        - "return ListResult(path=item.path, entries=entries, warning=warning)"
        - "warning = f\"Result reduced to top-level directories the limit of {_MAX_ENTRIES} is exceeded ({"
        - "return {'path': r.path, 'entries': r.entries, **({'warning': r.warning} if r.warning else {})}"
      - path: tools/search/__init__.py
        matches:
        - "warning: Set if ``limit`` was reached and further matches may exist, or if match lines"
        - "warning: str | None = None"
        - warning = None
        - "warning = f'Limit of {item.limit} matches reached (total {total_matches}); {hint}.'"
        - "shortened_warning = f'{shortened} match line(s) exceeded {_MAX_LINE_LENGTH} characters and were shortened in the middle.'"
        - "warning = f'{warning} {shortened_warning}' if warning else shortened_warning"
        - "return SearchResult(directory=item.directory, matches=matches, warning=warning)"
        - "if r.warning is not None:"
        - "entry['warning'] = r.warning"
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
      matches:
      - path: ast/openapi/AppendInfoNodesValidateServer.py
        matches:
        - "class AppendInfoNodesValidateServer(abc.ABC):"
      - path: ast/openapi/AppendInfoNodesValidateClient.py
        matches:
        - "class AppendInfoNodesValidateClient(abc.ABC):"
      - path: embedded/tools.py
        matches:
        - "class ToolProvider(ABC):"
      - path: cli/session.py
        matches:
        - "class Process(Protocol):"
      - path: tools/ast/base.py
        matches:
        - "class Engine(ABC):"
      - path: tools/ast/rpc/_process.py
        matches:
        - "class RpcProcess(ABC):"
      - path: tools/skills/__init__.py
        matches:
        - "class Skill(ABC):"
      - path: tools/tool_registry.py
        matches:
        - "class ToolDefinition(ABC):"
```
/answer 6bd4-3-2 allow
ReasoningToken: 15
Control Request:
```yaml
id: 6bd4-4-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
    pattern: \.py$
    include:
    - '*.py'
    exclude:
    - '**/__pycache__/**'
    limit: 50
  reason: List python files in the package to understand module layout.
```
/answer 6bd4-4-0 allow
Control Request:
```yaml
id: 6bd4-5-4
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
```
/answer 6bd4-5-4 allow
Text:
 Erste Treffer: `warning` (str) in `list`, `search`, `ast/outline`, `ast/read` (als Liste!) – schon uneinheitlich. Ich schaue mir die Tool-Struktur und die Result-/Error-Typen genauer an.
Control Request:
```yaml
id: 6bd4-6-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  reason: "Understand ToolDefinition ABC and how results/warnings are built in list, search, ast outline/read."
```
/answer 6bd4-6-0 allow
ReasoningToken: 47
Control Request:
```yaml
id: 6bd4-7-6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
      nodes:
      - id: M6pfOV|iTlyps
        excert: '''Central tool registry and tool result helpers.\n\nTools are registered once in…'
      - id: FPT4hs|aHGJs9
        excert: "from dataclasses import dataclass, field"
      - id: 07wTbv|BnS8bM
        excert: "'#: Default value for the Anthropic-specific ``anthropic/maxResultSizeChars``'"
      - id: ToolResult
        excert: "@dataclass(slots=True) class ToolResult:"
        docstring: The result of a tool call (maps onto MCP ``CallToolResult``).
        children:
        - id: ToolResult.6F9vfX|khLiSB
          excert: '''The result of a tool call (maps onto MCP ``CallToolResult``).'''
        - id: ToolResult.post_init
          excert: "def __post_init__(self) -> None:"
        - id: ToolResult.to_dict
          excert: "def to_dict(self) -> dict[str, Any]:"
      - id: text_content
        excert: "def text_content(text: str) -> dict[str, Any]:"
        docstring: Build a ``TextContent`` block.
      - id: attach_message
        excert: "def attach_message(result: dict[str, Any], message: str) -> None:"
      - id: BFWVtm|SRXvgt
        excert: "'#: A handler receives the invocation context and returns one of:'"
      - id: Tool
        excert: "@dataclass(slots=True) class Tool:"
        docstring: A registered tool and its MCP metadata.
        children:
        - id: Tool.cjwFjQ|SNEi1f
          excert: '''A registered tool and its MCP metadata.'''
        - id: Tool.to_spec
          excert: "def to_spec(self) -> dict[str, Any]:"
          docstring: Return the MCP ``Tool`` object advertised via ``tools/list``.
      - id: ToolDefinition
        excert: "class ToolDefinition(ABC):"
        docstring: Base class for registering a tool as an object. Subclasses declare the MCP meta…
        children:
        - id: ToolDefinition.lDzjus|OawaHI
          excert: '''Base class for registering a tool as an object.\n\n    Subclasses declare the …'
        - id: ToolDefinition.rremWZ|d5jedw
          excert: "name: str"
        - id: ToolDefinition.handle
          excert: "@abstractmethod def handle(self, ctx: ToolContext) -> 'ToolResult | str | dict[str, Any]':"
        - id: ToolDefinition.call
          excert: "def __call__(self, ctx: ToolContext) -> 'ToolResult | str | dict[str, Any]':"
        - id: ToolDefinition.to_tool
          excert: "def to_tool(self) -> Tool:"
      - id: normalize_result
        excert: "def normalize_result(value: 'ToolResult | str | dict[str, Any] | None') -> ToolResult:"
        docstring: Coerce whatever a handler returned into a :class:`ToolResult`.
      - id: hIMaii|mbWWig
        excert: "\"#: Name of the mandatory reason property injected into every tool's input\""
      - id: inject_property
        excert: "def _inject_property(schema: dict[str, Any], name: str, description: str, *, required: bool, first: bool=False) -> dict[str, Any]:"
        docstring: Return *schema* with an additional property generically injected. Used both for…
      - id: with_mandatory_reason
        excert: "def _with_mandatory_reason(schema: dict[str, Any]) -> dict[str, Any]:"
        docstring: "Return *schema* with a mandatory, short ``reason`` property injected. Every too…"
      - id: ToolRegistry
        excert: "class ToolRegistry:"
        docstring: Process-wide registry of available tools.
        children:
        - id: ToolRegistry.9VdZpn|xgr9BO
          excert: '''Process-wide registry of available tools.'''
        - id: ToolRegistry.init
          excert: "def __init__(self) -> None:"
        - id: ToolRegistry.register_alias
          excert: "def register_alias(self, alias: str, members: 'Iterable[str]') -> None:"
          docstring: "Define (or extend) a tool-set alias expanding to *members*. Generic mechanism: …"
        - id: ToolRegistry.expand_aliases
          excert: "def expand_aliases(self, names: 'Iterable[str]') -> set[str]:"
          docstring: Expand any alias names in *names* to their member tool names. The alias ``"all"…
        - id: ToolRegistry.is_enabled
          excert: "def is_enabled(self, session: Session, name: str) -> bool:"
          docstring: "Whether *name* is enabled for *session*, honouring tool-set aliases."
        - id: ToolRegistry.register
          excert: "def register(self, tool: ToolDefinition) -> Tool:"
        - id: ToolRegistry.tool
          excert: "def tool(self, name: str, *, description: str, input_schema: dict[str, Any], title: str | None=None, output_schema: dict[str, Any] | None=None, annotations: dict[str, Any] | None=None) -> Callable[[ToolHandler], ToolHandler]:"
          docstring: Decorator registering the decorated function as a tool handler.
        - id: ToolRegistry.get
          excert: "def get(self, name: str) -> Tool | None:"
        - id: ToolRegistry.contains
          excert: "def __contains__(self, name: object) -> bool:"
        - id: ToolRegistry.len
          excert: "def __len__(self) -> int:"
        - id: ToolRegistry.names
          excert: "def names(self) -> list[str]:"
        - id: ToolRegistry.list_for_session
          excert: "def list_for_session(self, session: Session) -> list[Tool]:"
          docstring: Reconcile the registry with the session's enabled-tool configuration. Returns t…
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
      nodes:
      - id: JrBuUj|ISKZu0
        excert: "\"List tool – returns files below a directory, grouped by relative subdirectory …"
      - id: dvO55c|qVCPEH
        excert: import os
      - id: ZrSLiQ|rTmMVq
        excert: "__all__ = ["
      - id: ListError
        excert: "class ListError(Exception):"
        docstring: Raised when a directory listing cannot be performed.
      - id: ListItem
        excert: "@dataclass(frozen=True) class ListItem:"
        docstring: "One directory to list. Attributes: path: Absolute directory path to list (must …"
      - id: ListResult
        excert: "@dataclass(frozen=True) class ListResult:"
        docstring: "Result of listing a single directory, mirroring its input path for result assoc…"
      - id: ListItemError
        excert: "@dataclass(frozen=True) class ListItemError:"
        docstring: "Error listing a single directory, mirroring its input path for result associati…"
      - id: ListBatchResult
        excert: "@dataclass(frozen=True) class ListBatchResult:"
        docstring: "Result of :func:`list`. Attributes: results: One :class:`ListResult` per succes…"
      - id: list_one
        excert: "def _list_one(item: ListItem) -> ListResult:"
        docstring: "List all files below the absolute directory ``item.path``, grouped like ``ls -R…"
      - id: dedupe_hierarchy
        excert: "def _dedupe_hierarchy(items: list[ListItem]) -> list[ListItem]:"
        docstring: Remove items whose path lies below another item's path in the same directory hi…
      - id: list
        excert: "def list(items: list[ListItem]) -> ListBatchResult:"
        docstring: "List each directory in ``items``. Args: items: Directories to list. Must be non…"
      - id: ListTool
        excert: "class ListTool(ToolDefinition):"
        children:
        - id: ListTool.7wRGko|3xW2xR
          excert: name = 'list'
        - id: ListTool.4gWrRr|Coidj4
          excert: "_ITEM_PROPERTIES = {"
        - id: ListTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`list`, translating the MCP schema to/from the Python API."
      - id: register_list_tool
        excert: "def register_list_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
      nodes:
      - id: OyEiBH|qDejkd
        excert: "'Search tool – recursive extended-regex search for retrieval, for a batch of it…"
      - id: PBZWYD|FmTmX2
        excert: "from dataclasses import dataclass, replace"
      - id: 8Pl1k7|dBAVKD
        excert: "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
      - id: wTnFQX|MKeuzI
        excert: "__all__ = ["
      - id: gph2jG|n4wV62
        excert: "_NODE_TOOLS = frozenset({'ast_read', 'ast_edit_block', 'ast_edit_marks', 'ast_i…"
      - id: SearchError
        excert: "class SearchError(Exception):"
        docstring: Raised when a search cannot be executed or its output cannot be parsed.
      - id: SearchLineMatch
        excert: "@dataclass(frozen=True) class SearchLineMatch:"
        docstring: A single matching line within a file.
      - id: SearchNodeMatch
        excert: "@dataclass(frozen=True) class SearchNodeMatch:"
        docstring: Matching lines attributed to the most specific enclosing AST node.
      - id: SearchMatch
        excert: "@dataclass(frozen=True) class SearchMatch:"
        docstring: All matches found within a single file. ``nodes`` is set instead of relying on …
      - id: SearchItem
        excert: "@dataclass(frozen=True) class SearchItem:"
        docstring: "One independent search to run. Attributes: directory: Absolute paths of the dir…"
      - id: SearchResult
        excert: "@dataclass(frozen=True) class SearchResult:"
        docstring: "Result of a single search. Attributes: directory: The directory list exactly as…"
      - id: SearchItemError
        excert: "@dataclass(frozen=True) class SearchItemError:"
        docstring: "Error running a single search, mirroring its input for result association."
      - id: SearchBatchResult
        excert: "@dataclass(frozen=True) class SearchBatchResult:"
        docstring: "Result of :func:`search`. Attributes: results: One :class:`SearchResult` per su…"
      - id: make_line_match
        excert: "def _make_line_match(lineno: int, text: str) -> SearchLineMatch:"
      - id: parse_search_stdout
        excert: "def _parse_search_stdout(stdout: str) -> list[SearchMatch]:"
        docstring: "Parse 'path:line:content' stdout into :class:`SearchMatch` objects, grouped by …"
      - id: as_list
        excert: "def _as_list(value: list[str] | str | None) -> list[str]:"
        docstring: Normalize an optional list into a list (empty if ``None``). A bare string is to…
      - id: run_search
        excert: "def _run_search(directory: list[str], pattern: str, *, exclude: list[str] | None=None, include: list[str] | None=None, limit: int=_DEFAULT_LIMIT) -> tuple[ProcessResult, int]:"
        docstring: Recursively search one or more directories for ``pattern`` (extended regexp). A…
      - id: search_one
        excert: "def _search_one(item: SearchItem) -> SearchResult:"
      - id: resolve_nodes
        excert: "def _resolve_nodes(match: SearchMatch, roots: list[str]) -> SearchMatch:"
        docstring: Regroup a file's line matches by the most specific enclosing AST node. ``match`…
      - id: search
        excert: "def search(items: list[SearchItem], *, intent: str=_DEFAULT_INTENT, with_nodes: bool=False) -> SearchBatchResult:"
        docstring: "Run one or more independent searches. Limits apply per item, not per batch. Arg…"
      - id: check_intent
        excert: "def _check_intent(intent: str) -> None:"
      - id: SearchTool
        excert: "class SearchTool(ToolDefinition):"
        children:
        - id: SearchTool.IRSVLT|PCuUvD
          excert: name = 'search'
        - id: SearchTool.g4tkLE|v74Wwu
          excert: "_ITEM_PROPERTIES = {"
        - id: SearchTool.SQiS9d|8EWkWE
          excert: "_ITEM_REQUIRED = ['directory', 'pattern']"
        - id: SearchTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func: translating the MCP schema to/from the Python API."
      - id: register_search_tool
        excert: "def register_search_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
      nodes:
      - id: JY1HZH|PFSe6T
        excert: "'``ast_outline`` tool: list AST nodes of one or more files.'"
      - id: e6pSwx|7lUxqQ
        excert: from dataclasses import dataclass
      - id: OeMnuN|SAOhjO
        excert: "__all__ = ["
      - id: OutlineNodesResult
        excert: "@dataclass(frozen=True) class OutlineNodesResult:"
        docstring: "Nodes of a single file. Attributes: path: The resolved path of the listed file …"
      - id: OutlineNodesError
        excert: "@dataclass(frozen=True) class OutlineNodesError:"
        docstring: "Error listing a single file. Attributes: path: The resolved path that failed, f…"
      - id: OutlineNodesBatchResult
        excert: "@dataclass(frozen=True) class OutlineNodesBatchResult:"
        docstring: "Result of :func:`ast_outline`. Attributes: results: One :class:`OutlineNodesRes…"
      - id: expand_path
        excert: "def _expand_path(path_str: str) -> list[str]:"
        docstring: "Expand *path_str* to its own single-element list, unless it names a directory t…"
      - id: outline_one
        excert: "def _outline_one(path: str, *, with_lines: bool) -> OutlineNodesResult:"
      - id: directory_listing_hint
        excert: "def _directory_listing_hint(path: str) -> str:"
        docstring: "Build an error message for a path that turned out to be a directory, anticipati…"
      - id: ast_outline
        excert: "def ast_outline(paths: list[str], *, with_lines: bool=True) -> OutlineNodesBatchResult:"
        docstring: List the hierarchical AST-node tree of one or more files. The tree is the found…
      - id: OutlineNodesTool
        excert: "class OutlineNodesTool(ToolDefinition):"
        children:
        - id: OutlineNodesTool.kZa70t|NgnQPq
          excert: name = 'ast_outline'
        - id: OutlineNodesTool.WDGjLy|cDLLDp
          excert: "input_schema = {"
        - id: OutlineNodesTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_outline`, translating the MCP schema to/from the AST API."
      - id: register
        excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
      nodes:
      - id: QftHvL|jTnLqY
        excert: "'``ast_read`` tool: read one or more node subtrees (with source) by id, across …"
      - id: vLfIF6|SP3nbZ
        excert: from dataclasses import dataclass
      - id: JnyjUM|0JyCWU
        excert: "__all__ = ['ReadItem', 'ReadResult', 'ReadError', 'ReadBatchResult', 'ast_read'…"
      - id: looks_like_root_intent
        excert: "def _looks_like_root_intent(ids: list[str]) -> bool:"
        docstring: Whether ``ids`` is empty or consists solely of common root-id guesses.
      - id: ReadItem
        excert: "@dataclass(frozen=True) class ReadItem:"
        docstring: "One file's node ids to read. Attributes: path: Absolute path to the file to rea…"
      - id: ReadResult
        excert: "@dataclass(frozen=True) class ReadResult:"
        docstring: "Nodes read from a single file. Attributes: path: The path exactly as given in t…"
      - id: ReadError
        excert: "@dataclass(frozen=True) class ReadError:"
        docstring: "Error reading a whole item (e.g. bad path or empty ``ids``). Attributes: path: …"
      - id: ReadBatchResult
        excert: "@dataclass(frozen=True) class ReadBatchResult:"
        docstring: "Result of :func:`ast_read`. Attributes: results: One :class:`ReadResult` per su…"
      - id: read_one
        excert: "def _read_one(item: ReadItem, *, with_lines: bool) -> ReadResult:"
      - id: ast_read
        excert: "def ast_read(items: list[ReadItem], *, with_lines: bool=True) -> ReadBatchResult:"
        docstring: "Recursively read the subtree of each addressed node, across one or more files. …"
      - id: ReadNodeTool
        excert: "class ReadNodeTool(ToolDefinition):"
        children:
        - id: ReadNodeTool.VeePTW|cPwHYq
          excert: name = 'ast_read'
        - id: ReadNodeTool.4gWrRr|O9B7Vp
          excert: "_ITEM_PROPERTIES = {"
        - id: ReadNodeTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_read`, translating the MCP schema to/from the AST API. P…"
      - id: register
        excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 6bd4-7-6 allow
Control Request:
```yaml
id: 6bd4-8-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: \b(class \w*(Result|Error|Item|Response)\w*|@dataclass|'error'|"error"|isError|is_error|'errors'|'skipped'|'missing'|'note'|'hint'|'notice')
    include:
    - '*.py'
    exclude:
    - '**/__pycache__/**'
    limit: 50
  reason: "Find result/error classes, dataclasses and ad-hoc dict keys (error, missing, note, hint) used as warning-like signals."
```
/answer 6bd4-8-0 allow
Control Request:
```yaml
id: 6bd4-9-8
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
      matches:
      - path: mcp/github.py
        matches:
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
      - path: mcp/client.py
        matches:
        - "class McpClientError(RuntimeError):"
      - path: mcp/bridge.py
        matches:
        - "class McpBridgeError(RuntimeError):"
        - remote tool itself reported ``isError``."""
        - tool call itself reports ``isError``.
        - "if result.get('isError', False):"
      - path: mcp/context7.py
        matches:
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
      - path: mcp/exa/fetch_results.py
        matches:
        - "class WebFetchExaResultsTool(ToolDefinition):"
        - "return ToolResult(content=[text_content(f'Invalid pattern: {exc}')], is_error=True)"
        - "return ToolResult(content=[text_content(f'Error resolving web_fetch_exa results: {exc}')], is_error=True)"
      - path: mcp/exa/fetch.py
        matches:
        - "class WebFetchResult:"
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
        - "return ToolResult(content=[text_content(f'Unexpected error in web_fetch_exa: {exc}')], is_error=True)"
      - path: mcp/exa/search.py
        matches:
        - "class WebSearchResult:"
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
        - "return ToolResult(content=[text_content(f'Unexpected error in web_search_exa: {exc}')], is_error=True)"
      - path: mcp/exa/search_results.py
        matches:
        - "class WebSearchExaResultsTool(ToolDefinition):"
        - "return ToolResult(content=[text_content(f'Error resolving web_search_exa results: {exc}')], is_error=True)"
      - path: mcp/exa/core.py
        matches:
        - "class ResultCache:"
      - path: tool_usage/__init__.py
        matches:
        - "class ToolUsageError(Exception):"
      - path: ast/base.py
        matches:
        - "class AstError(Exception):"
        - "class AstTextError(AstError):"
      - path: ast/find.py
        matches:
        - "class FileNodesResult:"
        - "class FindNodesResult:"
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
      - path: ast/script.py
        matches:
        - "class ScriptError(Exception):"
        - "class AstScriptResult:"
        - "return ToolResult(content=[text_content(str(exc))], is_error=True)"
      - path: ast/replace.py
        matches:
        - "class ReplaceItem:"
        - "class ReplaceResult:"
        - "class ReplaceError:"
        - "class ReplaceBatchResult:"
      - path: ast/delete.py
        matches:
        - "class DeleteItem:"
        - "class DeleteResult:"
        - "class DeleteError:"
        - "class DeleteBatchResult:"
      - path: ast/outline.py
        matches:
        - "class OutlineNodesResult:"
        - "class OutlineNodesError:"
        - "class OutlineNodesBatchResult:"
      - path: ast/edit_marks.py
        matches:
        - "class EditMarksItem:"
        - "class EditMarksResult:"
        - "class EditMarksError:"
        - "class EditMarksBatchResult:"
      - path: ast/read.py
        matches:
        - "class ReadItem:"
        - "class ReadResult:"
        - "class ReadError:"
        - "class ReadBatchResult:"
      - path: ast/validate.py
        matches:
        - "class ValidateError(Exception):"
        - "class ValidateResult:"
      warning: "Limit of 50 matches reached (total 167); narrow pattern, directory or include/exclude filters."
```
/answer 6bd4-9-8 allow
Text:
 Es gibt ein zentrales `_tool_helpers.handle_batch_tool`. Ich lese es und die konkreten Result-Klassen, um zu sehen, wie Serialisierung und Warnungen heute zusammenspielen.
Control Request:
```yaml
id: 6bd4-a-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
    pattern: def (handle_batch_tool|batch_schema|attach_message)|_tool_helpers
    include:
    - '*.py'
    exclude:
    - '**/__pycache__/**'
    limit: 50
  reason: Locate the shared batch helper module and its users.
```
/answer 6bd4-a-0 allow
Control Request:
```yaml
id: 6bd4-b-a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
      matches:
      - path: tools/mcp/exa/fetch_results.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import require_items
      - path: tools/mcp/exa/fetch.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import require_items
      - path: tools/mcp/exa/search_results.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import require_items
      - path: tools/tool_usage/__init__.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import require_items
      - path: tools/ast/find.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import require_items
      - path: tools/ast/replace.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import batch_schema
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
      - path: tools/ast/delete.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import batch_schema
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
      - path: tools/ast/outline.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import require_items, serialize_batch_result"
      - path: tools/ast/edit_marks.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import batch_schema
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
      - path: tools/ast/read.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import require_items, batch_schema"
      - path: tools/ast/validate.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import require_items
      - path: tools/ast/edit_block.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import batch_schema
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
      - path: tools/ast/create.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import batch_schema
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
      - path: tools/ast/insert.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import batch_schema
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
      - path: tools/skills/__init__.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
      - path: tools/bash/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import batch_schema, require_items"
      - path: tools/bash/run_command.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import batch_schema, require_items"
      - path: tools/_tool_helpers.py
        matches:
        - "def batch_schema(item_properties: dict[str, Any], required: list[str], description: str) -> dict[str, Any]:"
        - "def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], Any], error_class: type, result_ser…: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None, auto_approve: bool=False) -> ToolResult:"
      - path: tools/edit_lines/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
      - path: tools/tool_registry.py
        matches:
        - "def attach_message(result: dict[str, Any], message: str) -> None:"
      - path: tools/edit_block/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
      - path: tools/openalex/search_results.py
        matches:
        - from xy.ai.mcpc.tools._tool_helpers import require_items
      - path: tools/file_stats/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
      - path: tools/edit_marks/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
      - path: tools/list/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
      - path: tools/edit_line/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
      - path: tools/write/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
      - path: tools/read/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import require_items, serialize_batch_result, batch_schema"
      - path: tools/search/__init__.py
        matches:
        - "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
      warning: 1 match line(s) exceeded 300 characters and were shortened in the middle.
```
/answer 6bd4-b-a allow
ReasoningToken: 102
Control Request:
```yaml
id: 6bd4-c-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
  node_type: ClassDef
  regexp: (Result|Error)\b
  reason: "Read result/error dataclasses of list and search in full (fields, docstrings)."
```
/answer 6bd4-c-0 allow
Control Request:
```yaml
id: 6bd4-d-c
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
      nodes:
      - id: ListError
        type: ClassDef
        code: |-
          class ListError(Exception):
              """Raised when a directory listing cannot be performed."""
      - id: ListResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ListResult:
              """Result of listing a single directory, mirroring its input path for result association."""
              path: str
              entries: list[str]
              warning: str | None = None
      - id: ListItemError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ListItemError:
              """Error listing a single directory, mirroring its input path for result association."""
              path: str
              error: str
      - id: ListBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ListBatchResult:
              """Result of :func:`list`.

              Attributes:
                  results: One :class:`ListResult` per successfully listed directory.
                  errors: One :class:`ListItemError` per directory that failed.
              """
              results: list[ListResult]
              errors: list[ListItemError]
      - id: ListTool
        type: ClassDef
        code: |-
          class ListTool(ToolDefinition):
              name = 'list'
              title = 'List directory contents'
              description = 'List all files below one or more absolute directory paths, recursively, as a flat list, for a batch of items. Filter each result with a regular expression. Limits apply per item, not per batch.'
              _ITEM_PROPERTIES = {
                  'path': {
                      'type': 'string', 'description': 'Absolute directory path.'}, 'pattern': {
                          'type': 'string', 'description': 'Regular expression to filter the result.'}, 'recursive': {
                              'type': 'boolean', 'description': 'List subdirectories recursively. Default true.'}}
              _ITEM_REQUIRED = ['path']
              _ITEMS_DESCRIPTION = 'Directories to list.'
              input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)

              def handle(self, ctx: ToolContext) -> ToolResult:
                  """Delegate to :func:`list`, translating the MCP schema to/from the Python API."""

                  def item_factory(it: dict[str, Any]) -> ListItem:
                      return ListItem(path=it['path'], pattern=it.get('pattern'), recursive=it.get('recursive', True))

                  def result_serializer(r):
                      return {'path': r.path, 'entries': r.entries, **({'warning': r.warning} if r.warning else {})}
                  return handle_batch_tool(ctx, item_factory, list, ListError, result_serializer)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
      nodes:
      - id: SearchError
        type: ClassDef
        code: |-
          class SearchError(Exception):
              """Raised when a search cannot be executed or its output cannot be parsed."""
      - id: SearchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class SearchResult:
              """Result of a single search.

              Attributes:
                  directory: The directory list exactly as given in the input.
                  matches: The matches found, grouped by file (empty if none).
                  warning: Set if ``limit`` was reached and further matches may exist, or if match lines
                      exceeded the maximum length and were shortened with an ellipsis.

              """
              directory: list[str]
              matches: list[SearchMatch]
              warning: str | None = None
      - id: SearchItemError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class SearchItemError:
              """Error running a single search, mirroring its input for result association."""
              directory: list[str]
              pattern: str
              error: str
      - id: SearchBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class SearchBatchResult:
              """Result of :func:`search`.

              Attributes:
                  results: One :class:`SearchResult` per successfully executed search.
                  errors: One :class:`SearchItemError` per search that failed.
              """
              results: list[SearchResult]
              errors: list[SearchItemError]
      - id: SearchTool
        type: ClassDef
        code: |-
          class SearchTool(ToolDefinition):
              name = 'search'
              title = 'Search code, text, id, and content within directories'
              description = "Run one or more independent searches with an extended regular expression, for a batch of items. Always use the 'include' and 'exclude' filters."
              _ITEM_PROPERTIES = {
                  'directory': {
                      'type': 'array',
                      'items': {
                          'type': 'string'},
                      'minItems': 1,
                      'description': 'Absolute paths of the directories to search recursively. Always use the narrowest subtree(s) that are likely to contain the target files.'},
                  'pattern': {
                      'type': 'string',
                              'description': 'Extended regular expression to search for. Make the pattern as specific as possible to reduce noise.'},
                  'exclude': {
                      'type': 'array',
                      'items': {
                          'type': 'string'},
                      'description': "Globs of file names to exclude from the search, e.g. '*.min.js'. Always set this to exclude build artefacts, dependencies (e.g. 'node_modules/**'), and minified files."},
                  'include': {
                      'type': 'array',
                      'items': {
                          'type': 'string'},
                      'description': "Globs of file names to include in the search, e.g. '*.py'. Always set this to restrict the search to the relevant file types; omit only when the file type is unknown."},
                  'limit': {
                      'type': 'integer',
                      'description': 'Maximum number of matching lines to return for this item.',
                      'default': _DEFAULT_LIMIT,
                      'minimum': 1,
                      'maximum': _MAX_LIMIT},
                  'intent': {
                      'type': 'string',
                      'enum': list(_INTENTS),
                      'description': 'Intent for this item; overrides the batch-level intent.'}}
              _ITEM_REQUIRED = ['directory', 'pattern']
              _ITEMS_DESCRIPTION = 'Independent searches to run.'
              input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)
              input_schema['properties']['intent'] = {
                  'type': 'string',
                  'enum': list(_INTENTS),
                  'default': _DEFAULT_INTENT,
                  'description': 'Default intent for all items.' + _INTENT_DESCRIPTION}

              def handle(self, ctx: ToolContext) -> ToolResult:
                  """Delegate to :func: translating the MCP schema to/from the Python API."""
                  intent = ctx.arguments.get('intent', _DEFAULT_INTENT)
                  enabled = ctx.session.enabled_tools
                  with_lines = bool(_LINE_TOOLS & enabled)
                  with_nodes = bool(_NODE_TOOLS & enabled)

                  def item_factory(it: dict[str, Any]) -> SearchItem:
                      return SearchItem(
                          directory=it['directory'],
                          pattern=it['pattern'],
                          exclude=it.get('exclude'),
                          include=it.get('include'),
                          limit=int(
                              it.get(
                                  'limit',
                                  _DEFAULT_LIMIT)),
                          intent=it.get('intent'))

                  def line_dicts(lms: list[SearchLineMatch], with_lineno: bool) -> list[Any]:
                      if with_lineno:
                          return [{'lineno': lm.lineno, 'match': lm.match} for lm in lms]
                      return [lm.match for lm in lms]

                  def file_dict(m: SearchMatch, item_intent: str) -> dict[str, Any]:
                      editing = item_intent == 'editing'
                      entry: dict[str, Any] = {'path': f'{m.directory}/{m.filename}' if m.directory else m.filename}
                      if m.nodes:
                          entry['nodes'] = [{'id': n.node_id, 'matches': line_dicts(
                              n.matches, editing and with_lines)} for n in m.nodes]
                      if m.matches:
                          lines = line_dicts(m.matches, editing and (with_lines or with_nodes))
                          entry['matches'] = lines if editing else list(dict.fromkeys(lines))
                      return entry

                  def result_serializer(rendered: tuple[SearchResult, str]) -> dict[str, Any]:
                      r, item_intent = rendered
                      entry: dict[str, Any] = {'directory': r.directory}
                      if r.matches:
                          entry['matches'] = [file_dict(m, item_intent) for m in r.matches]
                      if r.warning is not None:
                          entry['warning'] = r.warning
                      return entry

                  def error_serializer(e: SearchItemError) -> dict[str, Any]:
                      return {'directory': e.directory, 'pattern': e.pattern, 'error': e.error}

                  def run(items: list[SearchItem]) -> SimpleNamespace:
                      results: list[tuple[SearchResult, str]] = []
                      errors: list[SearchItemError] = []
                      for item in items:
                          batch = search([item], intent=intent, with_nodes=with_nodes)
                          effective = item.intent if item.intent is not None else intent
                          results += [(r, effective) for r in batch.results]
                          errors += batch.errors
                      return SimpleNamespace(results=results, errors=errors)
                  return handle_batch_tool(ctx, item_factory, run, SearchError, result_serializer, error_serializer)
```
/answer 6bd4-d-c allow
Text:
 Jetzt `_tool_helpers.py` vollständig und `ToolResult`/`attach_message` in der Registry – dort liegt der zentrale Durchgriffspunkt.
Control Request:
```yaml
id: 6bd4-e-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
  reason: Get node ids of helpers in _tool_helpers.py.
```
/answer 6bd4-e-0 allow
Control Request:
```yaml
id: 6bd4-f-e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
      nodes:
      - id: j2F2xB|e5OyD1
        excert: '''Common helpers for tool definition and request handling.'''
      - id: YOvHfY|j1xNTQ
        excert: "from dataclasses import dataclass, asdict"
      - id: compact
        excert: "def _compact(value: Any) -> Any:"
        docstring: Recursively drop ``None`` values and empty lists from a dataclass-derived struc…
      - id: batch_schema
        excert: "def batch_schema(item_properties: dict[str, Any], required: list[str], description: str) -> dict[str, Any]:"
        docstring: "Build the standard ``{items: [...]}`` input schema shared by all batch tools. N…"
      - id: 1tCi6I|MiXew1
        excert: "__all__ = ['BatchError', 'require_items', 'handle_batch_tool', 'serialize_batch…"
      - id: BatchError
        excert: "@dataclass class BatchError:"
        docstring: Error from a single batch item.
      - id: aR0fQj|kPKhQR
        excert: T = TypeVar('T')
      - id: require_items
        excert: "def require_items(ctx: ToolContext, key: str='items') -> 'tuple[list[Any], ToolResult | None]':"
        docstring: "Return the non-empty list at *key* in ``ctx.arguments``, or an error result. Re…"
      - id: handle_batch_tool
        excert: "def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], Any], error_class: type, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None, auto_approve: bool=False) -> ToolResult:"
        docstring: "Common handler for batch-processing tools. Args: ctx: Tool context with argumen…"
      - id: serialize_batch_result
        excert: "def serialize_batch_result(batch_result: Any, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None) -> dict[str, Any]:"
        docstring: "Serialize a batch result, only including non-empty result/error lists. Args: ba…"
```
/answer 6bd4-f-e allow
Control Request:
```yaml
id: 6bd4-10-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
    ids:
    - ToolResult
    - attach_message
    - normalize_result
  reason: "Read ToolResult, attach_message and normalize_result – the existing top-level message mechanism."
```
/answer 6bd4-10-0 allow
Control Request:
```yaml
id: 6bd4-11-10
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
      nodes:
      - id: ToolResult
        type: ClassDef
        excert: "@dataclass(slots=True) class ToolResult:"
        docstring: The result of a tool call (maps onto MCP ``CallToolResult``).
        children:
        - id: ToolResult.6F9vfX|khLiSB
          type: statements
          code: |-
            'The result of a tool call (maps onto MCP ``CallToolResult``).'
            content: list[dict[str, Any]] = field(default_factory=list)
            structured_content: dict[str, Any] | None = None
            is_error: bool = False
            auto_approve: bool = False
            message: str | None = None
        - id: ToolResult.post_init
          type: FunctionDef
          code: |-
            def __post_init__(self) -> None:
                if self.is_error and self.structured_content:
                    raise ValueError('ToolResult: structured_content must not be set when is_error is True.')
        - id: ToolResult.to_dict
          type: FunctionDef
          code: |-
            def to_dict(self) -> dict[str, Any]:
                result: dict[str, Any] = {}
                if self.content:
                    result['content'] = list(self.content)
                if self.structured_content:
                    result['structuredContent'] = dict(self.structured_content)
                if self.message:
                    attach_message(result, self.message)
                if self.is_error:
                    result['isError'] = True
                return result
      - id: attach_message
        type: FunctionDef
        code: |-
          def attach_message(result: dict[str, Any], message: str) -> None:
              structured = dict(result.get('structuredContent') or {})
              structured[MESSAGE_PROPERTY] = message
              result['structuredContent'] = structured
              reference = f"The user attached a message: read '{MESSAGE_PROPERTY}' in structuredContent."
              result['content'] = [*result.get('content', []), text_content(reference)]
      - id: normalize_result
        type: FunctionDef
        code: |-
          def normalize_result(value: 'ToolResult | str | dict[str, Any] | None') -> ToolResult:
              """Coerce whatever a handler returned into a :class:`ToolResult`."""
              if isinstance(value, ToolResult):
                  return value
              if value is None:
                  return ToolResult(content=[])
              if isinstance(value, str):
                  return ToolResult(content=[text_content(value)])
              if isinstance(value, dict):
                  '# A dict already shaped like a CallToolResult is passed through.'
                  if 'content' in value and isinstance(value['content'], list):
                      return ToolResult(content=value['content'], structured_content=value.get(
                          'structuredContent'), is_error=bool(value.get('isError', False)))
                  '# Otherwise treat the dict as structured content.'
                  rendered = JsonCodec.encode(value)
                  return ToolResult(content=[text_content(rendered)], structured_content=value)
              raise TypeError(f'Unsupported tool return type: {type(value)!r}')
```
/answer 6bd4-11-10 allow
Text:
 `ToolResult.message` ist ein *User-Hinweis* (kein Warning) – wichtig zur Abgrenzung. Jetzt die Helfer und die abweichenden Warning-Stellen (`ast/read`, `ast/outline`).
ReasoningToken: 77
Control Request:
```yaml
id: 6bd4-12-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
    ids:
    - compact
    - BatchError
    - require_items
    - handle_batch_tool
    - serialize_batch_result
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
    ids:
    - ReadResult
    - ReadError
    - ReadBatchResult
    - read_one
    - ast_read
    - ReadNodeTool
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    ids:
    - OutlineNodesResult
    - OutlineNodesError
    - OutlineNodesBatchResult
    - ast_outline
    - OutlineNodesTool
  reason: "Read the batch serialization pipeline and the two AST tools with a diverging warning representation (list vs str, entry-level)."
```
/answer 6bd4-12-0 allow
Control Request:
```yaml
id: 6bd4-13-12
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
      nodes:
      - id: compact
        type: FunctionDef
        code: |-
          def _compact(value: Any) -> Any:
              """Recursively drop ``None`` values and empty lists from a dataclass-derived structure."""
              if isinstance(value, dict):
                  return {k: _compact(v) for k, v in value.items() if v is not None and v != []}
              if isinstance(value, list):
                  return [_compact(v) for v in value]
              return value
      - id: BatchError
        type: ClassDef
        code: |-
          @dataclass
          class BatchError:
              """Error from a single batch item."""
              error: str
      - id: require_items
        type: FunctionDef
        code: |-
          def require_items(ctx: ToolContext, key: str='items') -> 'tuple[list[Any], ToolResult | None]':
              """Return the non-empty list at *key* in ``ctx.arguments``, or an error result.

              Returns:
                  ``(values, None)`` on success, or ``([], error_result)`` if *key* is missing or
                  empty; callers must return ``error_result`` immediately in that case.
              """
              values = ctx.arguments.get(key) or []
              if not values:
                  return ([], ToolResult(content=[text_content(f"'{key}' must be a non-empty list.")], is_error=True))
              return (values, None)
      - id: handle_batch_tool
        type: FunctionDef
        code: |-
          def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], Any], error_class: type, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None, auto_approve: bool=False) -> ToolResult:
              """Common handler for batch-processing tools.

              Args:
                  ctx: Tool context with arguments.
                  item_factory: Function to convert raw dict to typed item.
                  batch_fn: Function to process list of items, returning batch result.
                  error_class: Exception class raised by `batch_fn` for whole-batch failures.
                  result_serializer: Optional custom serializer for result items (default: asdict).
                  error_serializer: Optional custom serializer for error items (default: asdict).

              Returns:
                  ToolResult with serialized batch result.
              """
              raw_items, error = require_items(ctx)
              if error is not None:
                  return error
              items: list[T] = []
              for it in raw_items:
                  try:
                      items.append(item_factory(it))
                  except KeyError as exc:
                      key = exc.args[0] if exc.args else exc
                      return ToolResult(content=[text_content(f'Invalid item: missing required property {key!r}')], is_error=True)
                  except (ValueError, TypeError) as exc:
                      return ToolResult(content=[text_content(f'Invalid item: {exc}')], is_error=True)
              try:
                  batch_result = batch_fn(items)
              except error_class as exc:
                  return ToolResult(content=[text_content(str(exc))], is_error=True)
              content = serialize_batch_result(
                  batch_result,
                  result_serializer=result_serializer,
                  error_serializer=error_serializer)
              has_error = hasattr(batch_result, 'errors') and batch_result.errors
              return ToolResult(structured_content=content, auto_approve=not has_error and auto_approve)
      - id: serialize_batch_result
        type: FunctionDef
        code: |-
          def serialize_batch_result(batch_result: Any, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None) -> dict[str, Any]:
              """Serialize a batch result, only including non-empty result/error lists.

              Args:
                  batch_result: Object with optional `results` and `errors` attributes.
                  result_serializer: Optional custom serializer for result items (default: asdict).
                  error_serializer: Optional custom serializer for error items (default: asdict).

              Returns:
                  Dict with only populated fields.
              """
              if result_serializer is None:
                  result_serializer = lambda r: _compact(asdict(r)) if hasattr(r, '__dataclass_fields__') else r
              if error_serializer is None:
                  error_serializer = lambda e: _compact(asdict(e)) if hasattr(e, '__dataclass_fields__') else e
              content: dict[str, Any] = {}
              if hasattr(batch_result, 'results') and batch_result.results:
                  content['results'] = [result_serializer(r) for r in batch_result.results]
              if hasattr(batch_result, 'errors') and batch_result.errors:
                  content['errors'] = [error_serializer(e) for e in batch_result.errors]
              notice = getattr(batch_result, 'notice', None)
              if notice:
                  content['notice'] = notice
              return content
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
      nodes:
      - id: ReadResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReadResult:
              """Nodes read from a single file.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  nodes: One expanded subtree per resolved id, in the given order; same
                      shape as :func:`ast_find`'s results (see :class:`core.OutlineNode`).
                  errors: One message per requested id that could not be resolved (id
                      unknown/ambiguous, and no unambiguous name/fuzzy match found).
              """
              path: str
              nodes: list[core.OutlineNode]
              errors: list[str]
      - id: ReadError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReadError:
              """Error reading a whole item (e.g. bad path or empty ``ids``).

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  error: The error message.
              """
              path: str
              error: str
      - id: ReadBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReadBatchResult:
              """Result of :func:`ast_read`.

              Attributes:
                  results: One :class:`ReadResult` per successfully read file.
                  errors: One :class:`ReadError` per file that failed entirely.
              """
              results: list[ReadResult]
              errors: list[ReadError]
      - id: read_one
        type: FunctionDef
        code: |-
          def _read_one(item: ReadItem, *, with_lines: bool) -> ReadResult:
              if not item.ids:
                  raise core.AstError("'ids' must be a non-empty list of node ids.")
              tree = core.load(item.path)[1]
              nodes, errs = core.read_subtrees(core.locate_all(tree), item.ids, with_lines=with_lines)
              return ReadResult(path=item.path, nodes=nodes, errors=errs)
      - id: ast_read
        type: FunctionDef
        code: |-
          def ast_read(items: list[ReadItem], *, with_lines: bool=True) -> ReadBatchResult:
              """Recursively read the subtree of each addressed node, across one or more files.

              Each id resolves to a subtree: a node whose body consists solely of nested
              classes/functions is expanded into ``children`` instead of source, so the agent
              can descend to the innermost editable block; any other node is returned whole,
              as ``code`` ready to hand back to ``ast_replace`` via its ``id``.

              An id that doesn't match any node is retried as a node *name* (exact, then a
              conservative fuzzy match) instead of failing the whole item; ids that still
              can't be resolved are reported in that item's ``errors``, not raised.

              Args:
                  items: Per-file node ids to read. Must be non-empty.
                  with_lines: Whether to populate each node's line range.

              Returns:
                  ReadBatchResult: One result per file (with its own id-level errors), and
                  one file-level error per file that failed entirely (bad path, empty
                  ``ids``, or a syntax error).

              Raises:
                  core.AstError: If ``items`` is empty.
              """
              if not items:
                  raise core.AstError("'items' must be a non-empty list.")
              results: list[ReadResult] = []
              errors: list[ReadError] = []
              for item in items:
                  try:
                      results.append(_read_one(item, with_lines=with_lines))
                  except core.AstError as exc:
                      errors.append(ReadError(path=item.path, error=str(exc)))
              return ReadBatchResult(results=results, errors=errors)
      - id: ReadNodeTool
        type: ClassDef
        excert: "class ReadNodeTool(ToolDefinition):"
        children:
        - id: ReadNodeTool.VeePTW|cPwHYq
          type: statements
          code: |-
            name = 'ast_read'
            title = 'Reads the code, text, and content of the nodes identified by the given IDs within the specified files.'
            description = 'After `ast_outline` or `ast_find`, reads the code, text, content, and children of the given AST node IDs within the given list of absolute file paths (never directories). Use this when you already know the IDs.'
        - id: ReadNodeTool.4gWrRr|O9B7Vp
          type: statements
          code: |-
            _ITEM_PROPERTIES = {
                'path': PATH_PROP,
                'ids': {
                    'type': 'array',
                    'items': {
                        'type': 'string'},
                    'description': 'List of AST node IDs to read.'}}
            _ITEM_REQUIRED = ['path']
            _ITEMS_DESCRIPTION = 'Per-file node IDs to read.'
            input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)
        - id: ReadNodeTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_read`, translating the MCP schema to/from the AST API.

                    Per item, falls back to :func:`ast_outline` when ``ids`` looks like agents
                    habitually mis-guessing a root id (``root``/``_module_``/``module``/empty)
                    and none of them resolve to a node: returns that file's full outline
                    instead of an error, and notes the redirect in the item's ``errors``.
                    """
                with_lines = bool({'tools', 'edit_lines'} & ctx.session.enabled_tools)
                raw_items, error = require_items(ctx)
                if error is not None:
                    return error
                results: list[dict[str, Any]] = []
                errors: list[dict[str, str]] = []
                for raw in raw_items:
                    path = raw.get('path')
                    ids: list[str] = raw.get('ids') or []
                    item_result: ReadResult | None = None
                    if ids:
                        batch = ast_read([ReadItem(path=path, ids=ids)], with_lines=with_lines)
                        if batch.errors:
                            errors.append({'path': batch.errors[0].path, 'error': batch.errors[0].error})
                            continue
                        item_result = batch.results[0]
                    if _looks_like_root_intent(ids) and (item_result is None or not item_result.nodes):
                        list_batch = ast_outline([path], with_lines=with_lines)
                        for lr in list_batch.results:
                            entry: dict[str, Any] = {'path': lr.path, 'nodes': [core.to_dict(n) for n in lr.nodes]}
                            if ids:
                                warning = f'ast_read fell back to ast_outline: ids {
                                    ids!r} resolved to no node; returning the full outline of {
                                        lr.path} instead.'
                                entry['warning'] = [warning]
                                entry['error'] = [
                                    f'ids {
                                        ids!r} resolved to no node; redirected to ast_outline, returning the full outline instead.']
                            results.append(entry)
                        for le in list_batch.errors:
                            errors.append({'path': le.path, 'error': le.error})
                        continue
                    pathResult = {'path': item_result.path, 'nodes': [core.to_dict(n) for n in item_result.nodes]}
                    if item_result.errors:
                        pathResult['errors'] = item_result.errors
                    results.append(pathResult)
                structured_content: dict[str, Any] = {}
                if results:
                    structured_content['results'] = results
                if errors:
                    structured_content['errors'] = errors
                return ToolResult(structured_content=structured_content)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
      nodes:
      - id: OutlineNodesResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class OutlineNodesResult:
              """Nodes of a single file.

              Attributes:
                  path: The resolved path of the listed file (see ``ast_outline``'s
                      directory-expansion note), for result association.
                  nodes: Outline-style node descriptions (see :class:`core.OutlineNode`), in
                      document order, suited for retrieval and navigation.
              """
              path: str
              nodes: list[core.OutlineNode]
      - id: OutlineNodesError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class OutlineNodesError:
              """Error listing a single file.

              Attributes:
                  path: The resolved path that failed, for result association.
                  error: The error message.
              """
              path: str
              error: str
      - id: OutlineNodesBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class OutlineNodesBatchResult:
              """Result of :func:`ast_outline`.

              Attributes:
                  results: One :class:`OutlineNodesResult` per successfully listed file.
                  errors: One :class:`OutlineNodesError` per file that failed.
              """
              results: list[OutlineNodesResult]
              errors: list[OutlineNodesError]
      - id: ast_outline
        type: FunctionDef
        code: |-
          def ast_outline(paths: list[str], *, with_lines: bool=True) -> OutlineNodesBatchResult:
              """List the hierarchical AST-node tree of one or more files.

              The tree is the foundation every other tool builds on: each node carries its
              unique, primarily name-based ``id`` and line range, but never its source –
              use ``ast_find`` (property/text filtering) or ``ast_read`` (by id) to
              retrieve source.

              A path naming a directory without subdirectories and holding at most
              5 files is transparently expanded to those files; any other directory
              is reported as an error whose message contains the directory's listing
              (via the ``list`` tool), anticipating the likely intent.

              Args:
                  paths: Absolute paths of the files to list. Must be non-empty.
                  with_lines: Whether to populate each node's line range.

              Returns:
                  OutlineNodesBatchResult: One result per listed file, one error per failed file.

              Raises:
                  core.AstError: If ``paths`` is empty.
              """
              if not paths:
                  raise core.AstError("'paths' must be a non-empty list.")
              results: list[OutlineNodesResult] = []
              errors: list[OutlineNodesError] = []
              for path in paths:
                  for real_path in _expand_path(path):
                      if Path(real_path).is_dir():
                          errors.append(OutlineNodesError(path=real_path, error=_directory_listing_hint(real_path)))
                          continue
                      try:
                          results.append(_outline_one(real_path, with_lines=with_lines))
                      except core.AstError as exc:
                          errors.append(OutlineNodesError(path=real_path, error=str(exc)))
              return OutlineNodesBatchResult(results=results, errors=errors)
      - id: OutlineNodesTool
        type: ClassDef
        excert: "class OutlineNodesTool(ToolDefinition):"
        children:
        - id: OutlineNodesTool.kZa70t|NgnQPq
          type: statements
          code: |-
            name = 'ast_outline'
            title = 'Hierarchical AST outline with IDs from a set of defined files'
            description = 'Given a list of one or more file names, returns a hierarchical AST tree with IDs, classes, functions, sections, type, docstring, children, imports, and segments. Other AST tools require IDs as input.'
        - id: OutlineNodesTool.WDGjLy|cDLLDp
          type: statements
          code: |-
            input_schema = {
                'type': 'object',
                'properties': {
                    'paths': {
                        'type': 'array',
                        'minItems': 1,
                        'items': {
                            'type': 'string'},
                        'description': 'List of absolute file paths (never directories).'}},
                'required': ['paths']}
        - id: OutlineNodesTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_outline`, translating the MCP schema to/from the AST API."""
                paths, error = require_items(ctx, key='paths')
                if error is not None:
                    return error
                with_lines = bool({'tools', 'edit_lines'} & ctx.session.enabled_tools)
                batch = ast_outline(paths=paths, with_lines=with_lines)
                result_serializer = lambda r: {'path': r.path, 'nodes': [core.to_dict(n) for n in r.nodes]}
                error_serializer = lambda e: {'path': e.path, 'error': e.error}
                content = serialize_batch_result(batch, result_serializer, error_serializer)
                return ToolResult(structured_content=content)
```
/answer 6bd4-13-12 allow
Thinking:
Ich prüfe jetzt alle Stellen, an denen `notice` verwendet wird, und suche nach weiteren Ad-hoc-Feldern, da `serialize_batch_result` das Batch-Level-`notice` bereits per `getattr` abfragt.


ReasoningToken: 256
Control Request:
```yaml
id: 6bd4-14-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
    pattern: "\\bnotice\\b|\\bnotes?\\b *[:=]|\\bhints?\\b *[:=]|\\bskipped\\b *[:=]|\\bmissing\\b *[:=]|\\btruncated\\b *[:=]"
    include:
    - '*.py'
    exclude:
    - '**/__pycache__/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
    pattern: ^\s+(errors|warnings?|notice|messages?|issues|problems)\s*:\s*(list|str|dict)
    include:
    - '*.py'
    exclude:
    - '**/__pycache__/**'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: result\.warning|\.warning\b
    include:
    - '*.py'
    exclude:
    - '**/__pycache__/**'
    limit: 20
  reason: Find batch-level 'notice' producers and every dataclass field that carries errors/warnings/notices in some form.
```
/answer 6bd4-14-0 allow
Control Request:
```yaml
id: 6bd4-15-14
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
      matches:
      - path: control/manager.py
        matches:
        - "logger.warning(\"Approval entry missing 'id', skipped: %s\", approval)"
      - path: server/http_transport.py
        matches:
        - "self.comm_log.log(session_id, IN, JsonCodec.for_log(raw), http='POST', note='unparseable')"
      - path: server/ws_transport.py
        matches:
        - "self.comm_log.log(session_id, IN, JsonCodec.for_log(text), transport='ws', note='unparseable')"
      - path: server/mcp_protocol.py
        matches:
        - "missing = [key for key in required if key not in value]"
        - "if missing:"
      - path: tools/mcp/exa/fetch_results.py
        matches:
        - "missing = [i for i in ids if i not in {item['id'] for item in items}]"
        - "if missing:"
      - path: tools/mcp/exa/search.py
        matches:
        - notice.
      - path: tools/mcp/exa/search_results.py
        matches:
        - "missing = [i for i in ids if i not in {item['id'] for item in items}]"
        - "if missing:"
      - path: tools/ast/replace.py
        matches:
        - "notice: str | None = None"
        - "return ReplaceBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: tools/ast/delete.py
        matches:
        - Both are ignored (and flagged in the batch notice) when a whole file is deleted.
        - "notice: str | None = None"
        - "return DeleteBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: tools/ast/outline.py
        matches:
        - "hint = 'Path is a directory, not a file; listing its contents instead.'"
      - path: tools/ast/edit_marks.py
        matches:
        - "notice: str | None = None"
        - "return EditMarksBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: tools/ast/edit_block.py
        matches:
        - "notice: str | None = None"
        - "return EditBlockBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: tools/ast/python/_imports.py
        matches:
        - "missing = [a for a in node.names if _key(node, a) not in existing]"
        - "if not missing:"
      - path: tools/ast/core.py
        matches:
        - "``unchecked``, which puts the notice on the batch. Engines without dedicated support"
      - path: tools/ast/insert.py
        matches:
        - "notice: str | None = None"
        - "return InsertBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: tools/ast/imports.py
        matches:
        - "missing = [s for s in adds if normalize_import(s) not in present]"
        - "if missing:"
        - "if doomed or missing:"
        - "missing = [s for s in adds if normalize_import(s) not in present.keys() - removed_keys]"
      - path: tools/skills/__init__.py
        matches:
        - "hint: str"
        - "hint = 'Apply proactively whenever creating, editing, or reviewing Markdown documents — even when formatting is not explicitly mentioned.'"
      - path: tools/bash/run_command.py
        matches:
        - missing = required - values.keys()
        - "if missing:"
      - path: tools/_tool_helpers.py
        matches:
        - "notice = getattr(batch_result, 'notice', None)"
        - "if notice:"
        - "content['notice'] = notice"
      - path: tools/openalex/search_results.py
        matches:
        - "missing = [i for i in ids if i not in found]"
        - "if missing:"
      - path: tools/search/__init__.py
        matches:
        - "truncated: bool = False"
        - "return SearchLineMatch(lineno=lineno, match=f'{text[:half]}…{text[-half:]}', truncated=True)"
        - "hint = 'narrow pattern, directory or include/exclude filters'"
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
      matches:
      - path: tools/tool_usage/__init__.py
        matches:
        - "errors: list[dict[str, str]] = []"
      - path: tools/ast/base.py
        matches:
        - "errors: list[str] = []"
      - path: tools/ast/replace.py
        matches:
        - "errors: list[ReplaceError]"
        - "notice: str | None = None"
        - "errors: list[ReplaceError] = []"
      - path: tools/ast/delete.py
        matches:
        - "errors: list[DeleteError]"
        - "notice: str | None = None"
        - "errors: list[DeleteError] = []"
      - path: tools/ast/outline.py
        matches:
        - "errors: list[OutlineNodesError]"
        - "errors: list[OutlineNodesError] = []"
      - path: tools/ast/edit_marks.py
        matches:
        - "errors: list[EditMarksError]"
        - "notice: str | None = None"
        - "errors: list[EditMarksError] = []"
      - path: tools/ast/read.py
        matches:
        - "errors: list[str]"
        - "errors: list[ReadError]"
        - "errors: list[ReadError] = []"
        - "errors: list[dict[str, str]] = []"
      - path: tools/ast/edit_block.py
        matches:
        - "errors: list[EditBlockError]"
        - "notice: str | None = None"
        - "errors: list[EditBlockError] = []"
      - path: tools/ast/create.py
        matches:
        - "errors: list[CreateFileError]"
        - "errors: list[CreateFileError] = []"
      - path: tools/ast/insert.py
        matches:
        - "errors: list[InsertError]"
        - "notice: str | None = None"
        - "errors: list[InsertError] = []"
      - path: tools/skills/__init__.py
        matches:
        - "errors: list[SkillsItemError]"
        - "errors: list[SkillsItemError] = []"
      - path: tools/edit_lines/__init__.py
        matches:
        - "errors: list[EditLinesItemError]"
        - "errors: list[EditLinesItemError] = []"
      - path: tools/tool_registry.py
        matches:
        - "message: str | None = None"
      - path: tools/edit_block/__init__.py
        matches:
        - "errors: list[EditBlockItemError]"
        - "errors: list[EditBlockItemError] = []"
      - path: tools/file_stats/__init__.py
        matches:
        - "errors: list[FileStatsItemError]"
        - "errors: list[FileStatsItemError] = []"
      - path: tools/edit_marks/__init__.py
        matches:
        - "errors: list[EditMarksItemError]"
        - "errors: list[EditMarksItemError] = []"
      - path: tools/list/__init__.py
        matches:
        - "warning: str | None = None"
        - "errors: list[ListItemError]"
        - "errors: list[ListItemError] = []"
      - path: tools/edit_line/__init__.py
        matches:
        - "errors: list[EditLineItemError]"
        - "errors: list[EditLineItemError] = []"
      - path: tools/write/__init__.py
        matches:
        - "errors: list[WriteItemError]"
        - "errors: list[WriteItemError] = []"
      - path: tools/read/__init__.py
        matches:
        - "errors: list[ReadItemError]"
        - "errors: list[ReadItemError] = []"
      - path: tools/search/__init__.py
        matches:
        - "warning: str | None = None"
        - "errors: list[SearchItemError]"
        - "errors: list[SearchItemError] = []"
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches:
      - path: outline.py
        matches:
        - "if result.warning:"
        - parts.append(result.warning)
      - path: core.py
        matches:
        - "logger.warning('Import handling failed for %s', tree.path, exc_info=True)"
```
/answer 6bd4-15-14 allow
Text:
 Jetzt die konkreten Quellen von `notice` (`import_notice`), den Outline-Fall mit `result.warning`, und die Ad-hoc-Stellen in `exa`, `read`, `edit_*`.
Control Request:
```yaml
id: 6bd4-16-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: def import_notice|import_notice\(|unchecked|import_(warning|note|status)
    include:
    - '*.py'
    exclude:
    - '**/__pycache__/**'
    limit: 30
    intent: survey
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: if result\.warning
    include:
    - outline.py
    exclude:
    - '**/__pycache__/**'
    intent: editing
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash
    pattern: "^(class \\w+|@dataclass.*|    \\w+: .*)$"
    include:
    - '*.py'
    exclude:
    - '**/__pycache__/**'
    limit: 50
  reason: "Locate import_notice implementation, the outline warning consumer, and result dataclass fields of non-AST tools (exa/read/write/bash)."
```
/answer 6bd4-16-0 allow
Control Request:
```yaml
id: 6bd4-17-16
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
      matches:
      - path: ast/replace.py
        matches:
        - "return ReplaceBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: ast/delete.py
        matches:
        - unchecked = ImportOutcome(unchecked=True) if item.add_import or item.remove_import else None
        - "return DeleteResult(path=item.path, id=None, result='success', imports=unchecked)"
        - "return DeleteBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: ast/edit_marks.py
        matches:
        - "return EditMarksBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: ast/edit_block.py
        matches:
        - "return EditBlockBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: ast/core.py
        matches:
        - "``unchecked``, which puts the notice on the batch. Engines without dedicated support"
        - return ImportOutcome(unchecked=True)
      - path: ast/insert.py
        matches:
        - "return InsertBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: ast/imports.py
        matches:
        - "unchecked: Whether at least one statement could not be handled and the"
        - "unchecked: bool = False"
        - "def import_notice(results: list[Any]) -> str | None:"
        - "\"\"\"The single batch-level hint, if any result carries unchecked imports.\"\"\""
        - "if any((r.imports is not None and r.imports.unchecked for r in results)):"
        - "``add`` is only checked: a statement missing as a line is reported as unchecked"
        - outcome.unchecked = any((normalize_import(_checked(s)) not in present for s in add))
        - outcome.unchecked = True
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches:
      - path: outline.py
        nodes:
        - id: directory_listing_hint.2HhHCU|bmwJWd
          matches:
          - "if result.warning:"
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash
      matches:
      - path: fetch_results.py
        matches:
        - "grouped: list[list[int]] = []"
      - path: fetch.py
        matches:
        - "@dataclass(frozen=True, slots=True)"
        - "results: list[dict[str, Any]]"
        - "items: list[dict[str, Any]] = []"
      - path: search.py
        matches:
        - "@dataclass(frozen=True, slots=True)"
        - "results: list[dict[str, Any]]"
        - "autoprompt_string: str | None = None"
        - "items: list[dict[str, Any]] = []"
      - path: __init__.py
        matches:
        - '@dataclass(frozen=True)'
        - "path: str"
        - "min_line: int | None = None"
        - "max_line: int | None = None"
        - "min_char: int | None = None"
        - "max_char: int | None = None"
        - "start: str | None = None"
        - "end: str | None = None"
        - "content: str"
        - "checksum: str"
        - "is_full_file: bool"
        - "unchanged: str | None = None"
        - "error: str"
        - "results: list[ReadResult]"
        - "errors: list[ReadItemError]"
        - "results: list[ReadResult] = []"
        - "errors: list[ReadItemError] = []"
        - "entry: dict[str, Any] = {'path': result.path, 'checksum': result.checksum}"
        - "mode: str"
        - "result: str"
        - "results: list[WriteResult]"
        - "errors: list[WriteItemError]"
        - "results: list[WriteResult] = []"
        - "errors: list[WriteItemError] = []"
        - "cwd: str"
      warning: "Limit of 50 matches reached (total 62); narrow pattern, directory or include/exclude filters."
```
/answer 6bd4-17-16 allow
Thinking:
Ich prüfe jetzt noch die Stellen, die Warnungen darstellen, aber anders weitergegeben werden: Exa, `ImportOutcome.unchecked`, `read.unchanged` und `directory_listing_hint`.


ReasoningToken: 134
Control Request:
```yaml
id: 6bd4-18-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa
    pattern: "if missing:"
    include:
    - fetch_results.py
    - search_results.py
    intent: editing
    exclude:
    - '**/__pycache__/**'
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex
    pattern: "if missing:"
    include:
    - search_results.py
    intent: editing
    exclude:
    - '**/__pycache__/**'
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: class ImportOutcome|def directory_listing_hint|def _directory_listing_hint
    include:
    - imports.py
    - outline.py
    intent: editing
    exclude:
    - '**/__pycache__/**'
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read
    pattern: unchanged
    include:
    - __init__.py
    intent: survey
    exclude:
    - '**/__pycache__/**'
    limit: 8
  reason: Get AST node ids for the semantically warning-like code paths to read them in context.
```
/answer 6bd4-18-0 allow
Control Request:
```yaml
id: 6bd4-19-18
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa
      matches:
      - path: fetch_results.py
        nodes:
        - id: web_fetch_exa_results.2aBVtp|vKRrCb
          matches:
          - "if missing:"
      - path: search_results.py
        nodes:
        - id: web_search_exa_results.1KxjtX|TL3YHl
          matches:
          - "if missing:"
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex
      matches:
      - path: search_results.py
        nodes:
        - id: openalex_search_results.FO0AXn|V77Cax
          matches:
          - "if missing:"
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches:
      - path: outline.py
        nodes:
        - id: directory_listing_hint
          matches:
          - "def _directory_listing_hint(path: str) -> str:"
      - path: imports.py
        nodes:
        - id: ImportOutcome
          matches:
          - "class ImportOutcome:"
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read
      matches:
      - path: __init__.py
        matches:
        - "unchanged: str | None = None"
        - session-agnostic and never sets :attr:`ReadResult.unchanged`.
        - unchanged = cache.get(key) == result.checksum
        - "if unchanged:"
        - unchanged='Content unchanged since the last identical read. Use the former read result.')
        - "if result.unchanged:"
        - "entry['unchanged'] = result.unchanged"
```
/answer 6bd4-19-18 allow
Control Request:
```yaml
id: 6bd4-1a-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch_results.py
    ids:
    - web_fetch_exa_results.2aBVtp|vKRrCb
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/search_results.py
    ids:
    - openalex_search_results.FO0AXn|V77Cax
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    ids:
    - directory_listing_hint
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    ids:
    - ImportOutcome
    - import_notice
  reason: "Read how unknown ids, directory hints and unchecked imports are communicated today."
```
/answer 6bd4-1a-0 allow
Control Request:
```yaml
id: 6bd4-1b-1a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch_results.py
      nodes:
      - id: web_fetch_exa_results.2aBVtp|vKRrCb
        type: statements
        code: |-
          items = fetch_cache.get_many(ids)
          missing = [i for i in ids if i not in {item['id'] for item in items}]
          if missing:
              logger.warning('web_fetch_exa_results: unknown or expired id(s): %s', missing)
          results = []
          for item in items:
              text = item.get('text') or ''
              if pattern:
                  text = _grep_lines(text, pattern, context)
              results.append(strip_empty({'id': item['id'], 'text': text}))
          return results
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/search_results.py
      nodes:
      - id: openalex_search_results.FO0AXn|V77Cax
        type: statements
        code: |-
          'Resolve ids from a prior openalex_search/openalex_semantic_search call to full records.\n\n    Args:\n        ids: OpenAlex ids returned by ``openalex_search`` or\n            ``openalex_semantic_search``.\n\n    Returns:\n        One full record per known id.\n    '
          items = search_cache.get_many(ids)
          found = {item['id'] for item in items}
          missing = [i for i in ids if i not in found]
          if missing:
              logger.warning('openalex_search_results: unknown or expired id(s): %s', missing)
          return items
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
      nodes:
      - id: directory_listing_hint
        type: FunctionDef
        code: |-
          def _directory_listing_hint(path: str) -> str:
              """Build an error message for a path that turned out to be a directory,
              anticipating the likely intent: list its contents (via the ``list`` tool)
              instead of outlining it as a file.
              """
              hint = 'Path is a directory, not a file; listing its contents instead.'
              try:
                  batch = _list_dirs([ListItem(path=path)])
              except ListError as exc:
                  return f'{hint} Listing failed: {exc}'
              if batch.errors:
                  return f'{hint} Listing failed: {batch.errors[0].error}'
              result = batch.results[0]
              parts = [hint]
              if result.warning:
                  parts.append(result.warning)
              parts.extend(result.entries)
              return '\n'.join(parts)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
      nodes:
      - id: ImportOutcome
        type: ClassDef
        code: |-
          @dataclass
          class ImportOutcome:
              """What an engine did for one item's ``addImport``/``removeImport``.

              Attributes:
                  added: Statements that were missing and have been inserted.
                  removed: Statements that were present and have been removed.
                  unchecked: Whether at least one statement could not be handled and the
                      agent has to check it manually.
              """
              added: list[str] = field(default_factory=list)
              removed: list[str] = field(default_factory=list)
              unchecked: bool = False
      - id: import_notice
        type: FunctionDef
        code: |-
          def import_notice(results: list[Any]) -> str | None:
              """The single batch-level hint, if any result carries unchecked imports."""
              if any((r.imports is not None and r.imports.unchecked for r in results)):
                  return IMPORT_NOTICE
              return None
```
/answer 6bd4-1b-1a allow
Control Request:
```yaml
id: 6bd4-1c-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/validate.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
  node_type: ClassDef
  regexp: Result|Error
  reason: "See result/error shapes of replace, find, validate, script, create to catalogue the full variety."
```
/answer 6bd4-1c-0 allow
Control Request:
```yaml
id: 6bd4-1d-1c
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
      nodes:
      - id: ReplaceResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReplaceResult:
              """Result of a single node replacement.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  result: Always ``"success"``.
                  new_id: The node's new id, only set if the replacement changed it.
              """
              path: str
              id: str | None
              result: str
              new_id: str | None = None
              imports: ImportOutcome | None = None
      - id: ReplaceError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReplaceError:
              """Error applying a single node replacement.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  error: The error message.
              """
              path: str
              id: str | None
              error: str
      - id: ReplaceBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReplaceBatchResult:
              """Result of :func:`ast_replace`.

              Attributes:
                  results: One :class:`ReplaceResult` per successful replacement.
                  errors: One :class:`ReplaceError` per replacement that failed.
              """
              results: list[ReplaceResult]
              errors: list[ReplaceError]
              notice: str | None = None
      - id: ReplaceNodeTool
        type: ClassDef
        code: |-
          class ReplaceNodeTool(ToolDefinition):
              name = 'ast_replace'
              title = 'Replace the complete source of the AST nodes identified by the given IDs with new content.'
              description = 'Replace the source or text of selected AST nodes in a batch; operations may target the same or different files. Returns the updated IDs.'
              _ITEM_PROPERTIES = {
                  'path': PATH_PROP,
                  'source': {
                      'type': 'string',
                      'description': 'Replacement source.'},
                  **PATH_SELECTOR_PROPS,
                  **IMPORT_PROPS}
              _ITEM_REQUIRED = ['path', 'source', 'id']
              _ITEMS_DESCRIPTION = 'Node replacements to apply.'
              input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)

              def handle(self, ctx: ToolContext) -> ToolResult:
                  """Delegate to :func:`ast_replace`, translating the MCP schema to/from the AST API."""

                  def item_factory(it: dict[str, Any]) -> ReplaceItem:
                      add_import, remove_import = parse_import_args(it)
                      return ReplaceItem(
                          path=it['path'],
                          source=it['source'],
                          id=it.get('id'),
                          add_import=add_import,
                          remove_import=remove_import)

                  def result_serializer(r: ReplaceResult) -> dict[str, Any]:
                      entry = {'path': r.path, 'result': r.result}
                      if r.id is not None:
                          entry['id'] = r.id
                      if r.new_id is not None:
                          entry['new_id'] = r.new_id
                      if (imports := import_entry(r.imports)) is not None:
                          entry['imports'] = imports
                      return entry

                  def error_serializer(e: ReplaceError) -> dict[str, Any]:
                      entry = {'path': e.path, 'error': e.error}
                      if e.id is not None:
                          entry['id'] = e.id
                      return entry
                  return handle_batch_tool(
                      ctx,
                      item_factory,
                      ast_replace,
                      core.AstError,
                      result_serializer,
                      error_serializer,
                      auto_approve=True)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
      nodes:
      - id: FileNodesResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class FileNodesResult:
              """Nodes matching the given selectors within a single file.

              Attributes:
                  path: The path exactly as given in the input.
                  nodes: Outline-style node descriptions (see :class:`core.OutlineNode`)
                      matching the given selectors, suited for retrieval and navigation.
              """
              path: str
              nodes: list[core.OutlineNode]
      - id: FindNodesResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class FindNodesResult:
              """Result of :func:`ast_find`.

              Attributes:
                  files: One :class:`FileNodesResult` per input path, in order.
              """
              files: list[FileNodesResult]
      - id: FindNodesTool
        type: ClassDef
        code: |-
          class FindNodesTool(ToolDefinition):
              name = 'ast_find'
              title = 'Find code, text, id, and content within the specified files.'
              description = 'Filter the AST-node tree of a list of files by type, name, id, line range, parent type, text substring or regexp (grep). Returns matches per file with their full source. Use this when looking up content without IDs.'
              input_schema = {
                  'type': 'object',
                  'properties': {
                      'paths': {
                          'type': 'array',
                          'items': {
                              'type': 'string'},
                          'description': 'Absolute paths of the files to search (never directories).'},
                      **SELECTOR_PROPS,
                      'text': {
                          'type': 'string',
                                  'description': 'Case-insensitive substring that the node’s source must contain. Example: `import` finds nodes containing imports.'},
                      'regexp': {
                          'type': 'string',
                          'description': 'Regular expression that the node’s source must match (`re.search` semantics).'}},
                  'required': ['paths']}

              def handle(self, ctx: ToolContext) -> ToolResult:
                  """Delegate to :func:`ast_find`, translating the MCP schema to/from the AST API."""
                  args: dict[str, Any] = ctx.arguments
                  paths, error = require_items(ctx, key='paths')
                  if error is not None:
                      return error
                  with_lines = bool({'tools', 'edit_lines'} & ctx.session.enabled_tools)
                  try:
                      result = ast_find(
                          paths=paths,
                          id=args.get('id'),
                          name=args.get('name'),
                          node_type=args.get('node_type'),
                          lineno=args.get('lineno'),
                          end_lineno=args.get('end_lineno'),
                          parent_type=args.get('parent_type'),
                          text=args.get('text'),
                          regexp=args.get('regexp'),
                          with_lines=with_lines)
                  except core.AstError as exc:
                      return ToolResult(content=[text_content(str(exc))], is_error=True)
                  return ToolResult(structured_content={'files': [{'path': f.path, 'nodes': [
                                    core.to_dict(n) for n in f.nodes]} for f in result.files]})
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/validate.py
      nodes:
      - id: ValidateError
        type: ClassDef
        code: |-
          class ValidateError(Exception):
              """Raised when the validate operation cannot be performed at all."""
      - id: FileCheck
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class FileCheck:
              """Compile-check result for a single file, as returned by :func:`ast_validate`.

              Attributes:
                  path: The path exactly as given in the input.
                  ok: Whether the file compiled successfully.
                  error: Error message (with line number) if ``ok`` is ``False``, else ``None``.
              """
              path: str
              ok: bool
              error: str | None
      - id: ValidateResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ValidateResult:
              """Result of :func:`ast_validate`.

              Attributes:
                  all_ok: Whether every file in ``files`` compiled successfully.
                  files: One :class:`FileCheck` per input path, in the given order.
              """
              all_ok: bool
              files: list[FileCheck] = field(default_factory=list)
      - id: ValidateTool
        type: ClassDef
        code: |-
          class ValidateTool(ToolDefinition):
              name = 'ast_validate'
              title = 'Validate files'
              description = 'Check that each of a list of files compiles; report success/error per file.'
              input_schema = {
                  'type': 'object',
                  'properties': {
                      'paths': {
                          'type': 'array',
                          'items': {
                              'type': 'string'},
                          'description': 'Absolute paths of files to validate.'}},
                  'required': ['paths']}

              def handle(self, ctx: ToolContext) -> ToolResult:
                  """Delegate to :func:`ast_validate`, translating the MCP schema to/from the AST API."""
                  paths, error = require_items(ctx, key='paths')
                  if error is not None:
                      return error
                  try:
                      result = ast_validate(paths)
                  except ValidateError as exc:
                      return ToolResult(content=[text_content(str(exc))], is_error=True)
                  return ToolResult(
                      structured_content={
                          'all_ok': result.all_ok,
                          'files': [
                              f.__dict__ for f in result.files]},
                      auto_approve=result.all_ok)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
      nodes:
      - id: ScriptError
        type: ClassDef
        code: |-
          class ScriptError(Exception):
              """Raised when an AST script cannot be run to completion."""
      - id: AstScriptResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class AstScriptResult:
              """Result of :func:`ast_script`.

              Attributes:
                  result: Always ``"success"``.
                  value: ``repr()`` of the script's ``result`` variable, if the script set one;
                      otherwise ``None``.
              """
              result: str
              value: str | None = None
      - id: ScriptTool
        type: ClassDef
        code: |-
          class ScriptTool(ToolDefinition):
              name = 'ast_script'
              title = 'Run AST script'
              description = "Run a restricted Python script against a file's AST when the single-operation ast_* tools (list/find/read/replace/insert/delete) are insufficient for a complex, multi-step, or conditional transform. Sandboxed globals: 'tree' and 'ast'; no imports; assign 'result' to return data; edits made via 'tree' are saved automatically. See the 'code' parameter for the full tree API and node-attribute reference."
              input_schema = {
                  'type': 'object',
                  'properties': {
                      'path': {
                          'type': 'string',
                          'description': 'Absolute path to the file.'},
                      'code': {
                          'type': 'string',
                          'description': "Python source executed with 'tree' and 'ast' as only globals; assign 'result' to return a value; no import statements; builtins limited to isinstance/issubclass/getattr/setattr/hasattr/delattr/len/list/dict/set/tuple/str/int/float/bool/enumerate/range/sorted/reversed/zip/map/filter/any/all/min/max/sum/type/repr. tree API: find(id=None, node_type=None, name=None, parent_type=None)->list[Located]; locate_all()->list[Located] (every addressable node, document order); node_code(loc)->str; replace(loc, code)->str|None; insert(loc, code, position='after'|'before')->int; delete(loc)->None; append(code)->int; source->str; path. Located attrs (engine-independent, mirror the id/type/lines/code fields used by ast_outline/ast_find/ast_read/ast_edit_*/ast_replace/ast_insert/ast_delete): node_id (unique dotted name/hash path from root, e.g. 'MyClass.method' — the id used by every ast_* tool); node_type (reported type, e.g. 'FunctionDef'/'pair'); name (simple name or None); lineno/end_lineno (1-based inclusive span, i.e. the 'lines' field elsewhere); parent_type (enclosing node's type, None at top level); expandable (True = pure container of nested defs, i.e. what 'read' descends into instead of inlining code); node/parent (opaque node handles for use only as tree.* arguments, never introspect their internals); index (position among parent's addressable children); tree (owning Tree; use its .source/.path, nothing else). Note: 'signature'/'docstring' shown by list/find/read (one-line header / short doc, only set for definition-like nodes when code is omitted) have no direct Located field; derive via node_code(loc) if needed."}},
                  'required': [
                      'path',
                      'code']}

              def handle(self, ctx: ToolContext) -> ToolResult:
                  """Delegate to :func:`ast_script`, translating the MCP schema to/from the Python API."""
                  args: dict[str, Any] = ctx.arguments
                  try:
                      result = ast_script(args['path'], args['code'])
                  except (core.AstError, ScriptError) as exc:
                      return ToolResult(content=[text_content(str(exc))], is_error=True)
                  structured: dict[str, Any] = {'result': result.result}
                  if result.value is not None:
                      structured['value'] = result.value
                  return ToolResult(structured_content=structured)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
      nodes:
      - id: CreateFileResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class CreateFileResult:
              """Result of creating a single file.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  result: Always ``"success"``.
              """
              path: str
              result: str
      - id: CreateFileError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class CreateFileError:
              """Error creating a single file.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  error: The error message.
              """
              path: str
              error: str
      - id: CreateFilesResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class CreateFilesResult:
              """Result of :func:`ast_create`.

              Attributes:
                  results: One :class:`CreateFileResult` per file created successfully.
                  errors: One :class:`CreateFileError` per file that failed.
              """
              results: list[CreateFileResult]
              errors: list[CreateFileError]
      - id: CreateFileTool
        type: ClassDef
        code: |-
          class CreateFileTool(ToolDefinition):
              name = 'ast_create'
              title = 'Create files'
              description = 'Create one or more files from source.'
              _ITEM_PROPERTIES = {
                  'path': {
                      'type': 'string', 'description': 'Absolute path of the file to create.'}, 'source': {
                          'type': 'string', 'description': 'Source for the new file.'}, 'overwrite': {
                              'type': 'boolean', 'description': 'Allow replacing an existing file.', 'default': False}}
              _ITEM_REQUIRED = ['path', 'source']
              _ITEMS_DESCRIPTION = 'Files to create.'
              input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)

              def handle(self, ctx: ToolContext) -> ToolResult:
                  """Delegate to :func:`ast_create`, translating the MCP schema to/from the Python API."""

                  def item_factory(it: dict[str, Any]) -> CreateFileItem:
                      return CreateFileItem(path=it['path'], source=it['source'], overwrite=it.get('overwrite', False))
                  return handle_batch_tool(ctx, item_factory, ast_create, core.AstError, auto_approve=True)
```
/answer 6bd4-1d-1c allow
Thinking:
Als Nächstes prüfe ich, wie `structured_content` in `content` für den Client gespiegelt wird, weil davon abhängt, ob eine zentrale `warnings`-Injektion in `ToolResult.to_dict` reicht.


ReasoningToken: 4088
Control Request:
```yaml
id: 6bd4-1e-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
    pattern: structured_content|structuredContent|MESSAGE_PROPERTY
    include:
    - '*.py'
    exclude:
    - '**/__pycache__/**'
    - tools/ast/**
    - tools/edit_*/**
    limit: 30
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: "serialize_batch_result|handle_batch_tool|\\.notice\\b|\\['notice'\\]|'warning'"
    include:
    - test_*.py
    - '*_test.py'
    exclude:
    - '**/__pycache__/**'
    - src/**
    limit: 20
  reason: "Find where structuredContent is turned into text content / sent to the client, and which tests pin the current warning/notice formats."
```
/answer 6bd4-1e-0 allow
Control Request:
```yaml
id: 6bd4-1f-1e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
      matches:
      - path: server/mcp_protocol.py
        matches:
        - "structured_content={"
        - "return ToolResult(structured_content={'answer': decision.approval_hint}).to_dict()"
        - "return ToolResult(structured_content={'answer': combined_hint}).to_dict()"
      - path: tools/mcp/github.py
        matches:
        - return ToolResult(structured_content=result)
      - path: tools/mcp/bridge.py
        matches:
        - '''# Use structuredContent from the remote server when present. Otherwise'''
        - structured = result.get('structuredContent')
      - path: tools/mcp/context7.py
        matches:
        - "return ToolResult(structured_content={'libraries': [asdict(library) for library in libraries]})"
        - "return ToolResult(structured_content={'sections': [asdict(section) for section in sections]})"
      - path: tools/mcp/exa/fetch_results.py
        matches:
        - "return ToolResult(structured_content={'results': results}, auto_approve=False)"
      - path: tools/mcp/exa/fetch.py
        matches:
        - "'#: ``structuredContent`` (observed in practice on ``mcp.exa.ai``: it never'"
        - "'#: sends ``structuredContent`` for this tool). Each fetched url renders as a'"
        - "return ToolResult(structured_content={'results': result.results})"
      - path: tools/mcp/exa/search.py
        matches:
        - "'#: does not return ``structuredContent`` (observed in practice on'"
        - "'#: ``mcp.exa.ai``: it never sends ``structuredContent`` for this tool).'"
        - return ToolResult(structured_content=structured)
      - path: tools/mcp/exa/search_results.py
        matches:
        - "return ToolResult(structured_content={'results': results}, auto_approve=False)"
      - path: tools/mcp/exa/core.py
        matches:
        - '``McpBridge`` normally hands back either the remote''s ``structuredContent``'
        - "``structuredContent``, not JSON) - in that case ``McpBridge`` falls back to"
      - path: tools/tool_usage/__init__.py
        matches:
        - "structured_content: dict[str, Any] = {'usages': usages}"
        - "structured_content['errors'] = errors"
        - return ToolResult(structured_content=structured_content)
      - path: tools/ast/find.py
        matches:
        - "return ToolResult(structured_content={'files': [{'path': f.path, 'nodes': ["
      - path: tools/ast/script.py
        matches:
        - return ToolResult(structured_content=structured)
      - path: tools/ast/outline.py
        matches:
        - return ToolResult(structured_content=content)
      - path: tools/ast/read.py
        matches:
        - "structured_content: dict[str, Any] = {}"
        - "structured_content['results'] = results"
        - "structured_content['errors'] = errors"
        - return ToolResult(structured_content=structured_content)
      - path: tools/ast/validate.py
        matches:
        - "structured_content={"
      warning: "Limit of 30 matches reached (total 62); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc
      matches:
      - path: tests/test_ast_imports.py
        matches:
        - assert not result.errors and result.notice is None
        - assert result.notice == IMPORT_NOTICE
```
/answer 6bd4-1f-1e allow
Result Stats: id=43112839-11be-4621-8d0d-cda5af1a91db, total: 69597, in: 22, out: 20272, reason: 4165, read: 262358, write: 49303, start: 1794947608422, end: 1794947673796, time: 65228, total: 148150, io: 0, control: 146, sent: 0, recv: 988
ReasoningToken: 4165
Ich habe nichts geändert. Das ist Analyse und Vorschlag. Die Textsuche habe ich so gefiltert, dass `logger.warning` nicht mitzählt, und stattdessen Felder und Result-Klassen per AST gelesen.

## 1. Ist-Zustand

Es gibt keine gemeinsame Basis. Warnungen laufen heute über fünf verschiedene Mechanismen:

| Stelle | Heute | Problem |
|---|---|---|
| `list` (`ListResult`), `search` (`SearchResult`) | `warning: str \| None` am Item, von Hand serialisiert (`if r.warning` bzw. `is not None`) | Zwei Warnungen werden per `f'{a} {b}'` zu einem String verklebt (`search`). |
| `ast_read` (Handler) | `entry['warning'] = [str]` als Liste mit einem Element. Zusätzlich `entry['error'] = [str]` an einem *erfolgreichen* Result-Eintrag | Der Handler umgeht `handle_batch_tool` und baut Dicts selbst. Dieselbe Meldung steht doppelt als warning und error. |
| `ReadResult.errors: list[str]` | Teilfehler (nicht auflösbare ids) in einem erfolgreichen Item | Semantisch eine Warnung, heißt aber `errors`. |
| `ast_replace`, `ast_delete`, `ast_insert`, `ast_edit_block`, `ast_edit_marks` | `notice: str \| None` am `*BatchResult`, befüllt durch `import_notice(results)` | `serialize_batch_result` liest es per `getattr(batch_result, 'notice', None)`. Das ist Duck-Typing, und es gibt kein Typ-Gate. |
| `ast_outline._directory_listing_hint` | `ListResult.warning` wird in den Fehlertext hineinkopiert | Die Struktur geht verloren. |
| `tools/read` | `ReadResult.unchanged: str \| None` | Ein Status-Hinweis als Spezialfeld. |
| `web_fetch_exa_results`, `web_search_exa_results`, `openalex_search_results` | Unbekannte oder abgelaufene ids erscheinen nur in `logger.warning` | Der Agent sieht den Datenverlust nie. Das ist die eigentliche Lücke. |
| `ToolResult.message` | Nachricht des Users (`attach_message`) | Ein anderer Kanal, keine Warnung. Er muss als reservierter Name abgegrenzt bleiben. |

Bei den Errors ist es ähnlich:
- Jedes Tool definiert eigene `XxxError`-Klassen, mit wechselnden Schlüsselfeldern (`path`, `path+id`, `directory+pattern`).
- Der Feldname `error: str` ist bis auf den `ast_read`-Fall (dort eine Liste) einheitlich, aber nur zufällig.
- `BatchError` in `_tool_helpers` scheint ungenutzt zu sein. Das müsste ich noch prüfen.
- `ToolResult(content=[text_content(...)], is_error=True)` steht etwa 20-mal copy-pasted im Code.

Die Stelle, an der man ansetzen kann, ist `serialize_batch_result` zusammen mit `handle_batch_tool`. Nicht darüber laufen `ast_read`, `ast_outline`, `read`, `tool_usage`, `ast_find`, `ast_validate`, `ast_script` und Exa/OpenAlex. Für sie braucht es `ToolResult` als zweiten Durchgriffspunkt.

## 2. Vorschlag

### Semantische Regel

- **Error** heißt: Das Item hat kein Ergebnis geliefert. Der Feldname ist immer `error: str`.
- **Item-Warning** heißt: Das Item hat ein Ergebnis geliefert, aber mit Einschränkung (gekürzt, Fallback, Teilauflösung). Das Feld ist `warnings` am Result-Eintrag.
- **Result-Warning** betrifft den ganzen Call oder ist über mehrere Items identisch (zum Beispiel "Imports ungeprüft"). Das Feld ist `warnings` im Root von `structuredContent`.
- Ein Error-Item hat keine Warnings, und ein Error-`ToolResult` hat keine Warnings.

### Neues Modul `tools/results.py`

```python
from __future__ import annotations

from abc import ABC, abstractmethod
from dataclasses import dataclass, field, replace
from typing import Any, Generic, TypeVar, final

WARNINGS_KEY = 'warnings'
RESERVED_NAMES = frozenset({'warning', 'warnings', 'notice', 'notices', 'hint', 'error', 'errors', 'message'})


def _guard_fields(cls: type, allowed: frozenset[str]) -> None:
    clash = (set(cls.__dict__.get('__annotations__', {})) & RESERVED_NAMES) - allowed
    if clash:
        raise TypeError(f'{cls.__name__}: reserved field name(s) {sorted(clash)}; use the base class API')
    if 'to_dict' in cls.__dict__:
        raise TypeError(f'{cls.__name__}: to_dict is final; override payload()')


@dataclass(frozen=True, kw_only=True)
class ItemOutcome(ABC):
    @abstractmethod
    def key(self) -> dict[str, Any]:
        """Input fields mirrored for result association."""

    @abstractmethod
    def to_dict(self) -> dict[str, Any]: ...


@dataclass(frozen=True, kw_only=True)
class ItemResult(ItemOutcome):
    warnings: tuple[str, ...] = ()

    def __init_subclass__(cls, **kw: Any) -> None:
        super().__init_subclass__(**kw)
        _guard_fields(cls, allowed=frozenset())

    def payload(self) -> dict[str, Any]:
        return compact_asdict(self, exclude={WARNINGS_KEY})

    @final
    def to_dict(self) -> dict[str, Any]:
        out = {**self.key(), **self.payload()}
        if self.warnings:
            out[WARNINGS_KEY] = list(self.warnings)
        return out

    def warn(self, *messages: str) -> 'ItemResult':
        return replace(self, warnings=(*self.warnings, *messages))


@dataclass(frozen=True, kw_only=True)
class ItemError(ItemOutcome):
    error: str

    def __init_subclass__(cls, **kw: Any) -> None:
        super().__init_subclass__(**kw)
        _guard_fields(cls, allowed=frozenset())

    @final
    def to_dict(self) -> dict[str, Any]:
        return {**self.key(), 'error': self.error}


R = TypeVar('R', bound=ItemResult)
E = TypeVar('E', bound=ItemError)


@dataclass(frozen=True, kw_only=True)
class BatchResult(ABC, Generic[R, E]):
    results: list[R] = field(default_factory=list)
    errors: list[E] = field(default_factory=list)
    warnings: tuple[str, ...] = ()

    def __post_init__(self) -> None:
        if not all(isinstance(r, ItemResult) for r in self.results):
            raise TypeError('results must be ItemResult instances')
        if not all(isinstance(e, ItemError) for e in self.errors):
            raise TypeError('errors must be ItemError instances')

    def derived_warnings(self) -> tuple[str, ...]:
        return ()

    @final
    def all_warnings(self) -> list[str]:
        return list(dict.fromkeys((*self.warnings, *self.derived_warnings())))
```

Konkrete Klassen schrumpfen auf das Wesentliche:

```python
@dataclass(frozen=True, kw_only=True)
class ListResult(ItemResult):
    path: str
    entries: list[str]

    def key(self): return {'path': self.path}


class ReplaceBatchResult(BatchResult[ReplaceResult, ReplaceError]):
    def derived_warnings(self):
        return (IMPORT_NOTICE,) if any(r.imports and r.imports.unchecked for r in self.results) else ()
```

`import_notice()` und das Feld `notice` entfallen. Die Warnung wird aus den Items abgeleitet und kann nicht vergessen werden.

### `ToolResult` als Result-Ebene für alle Tools, auch ohne Batch

```python
@dataclass(slots=True)
class ToolResult:
    ...
    warnings: list[str] = field(default_factory=list)

    def __post_init__(self) -> None:
        if self.is_error and (self.structured_content or self.warnings):
            raise ValueError('error results carry neither structured content nor warnings')
        if self.structured_content and RESERVED_ROOT & self.structured_content.keys():
            raise ValueError('reserved key in structured_content; use ToolResult.warnings / .message')

    @classmethod
    def failure(cls, message: str) -> 'ToolResult':
        return cls(content=[text_content(message)], is_error=True)
```

- `to_dict` ist der einzige Schreiber des Root-Keys `warnings`, analog zu `attach_message`.
- `handle_batch_tool` setzt `ToolResult(structured_content=..., warnings=batch.all_warnings())`.
- Die Exa- und OpenAlex-`*_results`-Tools übergeben ihre `missing`-ids so: `ToolResult(structured_content=..., warnings=[f'unknown or expired id(s): {missing}'])`.

### Wodurch es per Code erzwungen wird

1. **Beim Import:** `__init_subclass__` verbietet Felder wie `warning`, `notice`, `hint` oder `errors` in `ItemResult` und `ItemError`. Ebenso verbietet es das Überschreiben von `to_dict`.
2. **Ein einziger Serialisierungspunkt:** `serialize_batch_result` prüft `isinstance(batch_result, BatchResult)` statt `getattr`. Es ruft `to_dict()` auf und schreibt Root-`warnings` nur aus `all_warnings()`. Kontextabhängige Serializer (`search` mit `intent`, `core.to_dict(n)` mit `with_lines`) liefern nur `payload`. Setzt ein Payload einen reservierten Key, gibt es einen Fehler.
3. **Typen:** `handle_batch_tool(batch_fn: Callable[[list[T]], BatchResult[Any, Any]])` wird von mypy und pyright geprüft. `BatchResult.__post_init__` prüft zusätzlich zur Laufzeit.
4. **Escape-Hatch schließen:** `ToolDefinition.handle` darf nur noch `ToolResult` zurückgeben. Die Rückgabe von `str` oder `dict` samt Durchreichen von `content`-Dicts in `normalize_result` fällt weg.
5. **Contract-Test:** Der Test iteriert rekursiv über `ItemResult.__subclasses__()` und `BatchResult.__subclasses__()`. Zusätzlich parst er `tools/**/*.py` per `ast` und schlägt an, wenn irgendwo ein annotiertes Feld oder ein Dict-Key `warning`, `notice` oder `hint` außerhalb von `results.py` vorkommt. Das ist eine strukturelle Prüfung und keine Textsuche.
6. **Optional:** Ein gemeinsames `WARNINGS_SCHEMA`-Fragment kann in jedes `output_schema` eingehängt werden, damit alle Tools es identisch ankündigen.

### Migrationszuordnung

- `ListResult.warning` und `SearchResult.warning` werden zu `warnings: tuple[str, ...]`. Bei `search` werden die zwei Warnungen getrennt statt verklebt.
- `ReadResult.errors: list[str]` wird zu Item-`warnings` (Teilauflösung).
- Der `ast_read`-Handler läuft künftig über `handle_batch_tool`. Der Outline-Fallback wandert in die Fachfunktion und wird dort ein Item-Warning. Das doppelte `error` entfällt.
- Die fünf `notice`-Felder werden zu `derived_warnings()` (siehe oben).
- `_directory_listing_hint` verwendet `result.warnings`, die Struktur bleibt dabei erhalten.
- Alle `ToolResult(..., is_error=True)` werden zu `ToolResult.failure(...)`.
- `BatchError` wird entfernt, falls tatsächlich ungenutzt.

## 3. Reihenfolge

1. `results.py` und `ToolResult.warnings` / `.failure` ergänzen. Das ist rein additiv.
2. `list` und `search` migrieren, weil sie die einfachsten Item-Warnings haben.
3. Die `ast_*`-Batch-Tools mit `notice` migrieren und danach `ast_read` und `ast_outline`.
4. Exa- und OpenAlex-`*_results` auf `ToolResult.warnings` umstellen.
5. Verschärfen: `handle` auf `ToolResult` einschränken, `getattr`-Duck-Typing und `BatchError` entfernen, Contract-Test aktivieren.

Das ändert das Wire-Format:
- `warning` und `notice` werden zu `warnings` (Liste).
- `tests/test_ast_imports.py` prüft derzeit `result.notice` und muss angepasst werden.
- Bei der Migration `kw_only=True` beachten: Aufrufer, die `*BatchResult` positional konstruieren, müssten umgestellt werden. Die gesehenen Aufrufe sind alle keyword-basiert.

## 4. Entscheidungen

1. **Key-Name:** Ich empfehle `warnings` auf beiden Ebenen, getrennt durch Position (Item-Eintrag vs. Root) und durch Typ im Code.
2. **Format:** Ich empfehle `list[str]` für LLM-Konsumenten. Keine strukturierte Form `{code, message}`.
3. **`read.unchanged`:** Es ändert, ob Inhalt vorhanden ist, und ist damit eher Daten als Warnung. Ich würde es als typisiertes Feld lassen.