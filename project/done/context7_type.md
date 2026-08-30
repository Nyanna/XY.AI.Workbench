Das Tool `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/context7.py` soll typisierte Resultobjekte erhalten.

## context7_libraries
`context7_libraries` Soll das Context7 Antwort parsen und nur Title, "Context7-compatible library ID" und Description verwenden.
Das Tool ist dir auch Verfügbar um die Rückgabe zu testen.

```
- Title: React Documentation
- Context7-compatible library ID: /reactjs/react.dev
- Description: The library for web and native user interfaces
- Code Snippets: 1250
- Source Reputation: High
- Benchmark Score: 98
- Versions: 19.0.0, 18.3.1, 18.2.0
----------
- Title: React Native
- Context7-compatible library ID: /facebook/react-native
- Description: A framework for building native applications using React
- Code Snippets: 890
- Source Reputation: High
- Benchmark Score: 95
- Versions: 0.76.0, 0.75.4
```

## context7_documentation

`context7_documentation` soll zumindest eine Liste aus den Ergebnissen parsen anhand von `---` oder einen Fehler entsprechend Zurückleiten anhand eines Triggertextes.

### Output Format

On success, the tool returns the documentation as plain text, formatted for easy consumption by the AI model:

```
# Server Components

Server Components let you write UI that can be rendered and optionally cached on the server.

## Example

\`\`\`tsx
async function ServerComponent() {
  const data = await fetchData();
  return <div>{data}</div>;
}
\`\`\`

---

# Using Server Components with Client Components

You can import Server Components into Client Components...
```

#### On Failure

```
No documentation found for library "/invalid/library". This might have happened because you used an invalid Context7-compatible library ID. Use 'resolveLibraryId' to get a valid ID.
```
SystemInit:  id=7626723e-cb12-4e28-b2f6-22df312c74e9, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: e95a-1
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/context7.py
  reason: "Aktuellen Code des Tools lesen, um typisierte Resultobjekte zu ergänzen"
```
/answer e95a-1 allow
Control Request:
```yaml
id: e95a-2
result:
  content:
  - type: text
    text: "Tool 'read_file' failed: name 'checksum' is not defined"
  isError: true
```
/answer e95a-2 allow
Cannot read STDOUT: the CLI process is not started (uuid=7626723e-cb12-4e28-b2f6-22df312c74e9, processAlive=false). A prompt must be sent (writeLine) before reading.
Ok
SystemInit:  id=7626723e-cb12-4e28-b2f6-22df312c74e9, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Bitte sag mir Bescheid, sobald der mcpc-Server wieder verbunden ist, dann mache ich direkt mit der Aufgabe weiter (Datei lesen und typisierte Resultobjekte für `context7_libraries` und `context7_documentation` ergänzen).
Ist da
SystemInit:  id=7626723e-cb12-4e28-b2f6-22df312c74e9, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 325f-1
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/context7.py
  reason: "Aktuellen Code des Tools lesen, um typisierte Resultobjekte zu ergänzen"
```
/answer 325f-1 allow
Control Request:
```yaml
id: 325f-2
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/bridge.py
  reason: "Verstehen wie McpBridge.call funktioniert und welches Format das Ergebnis hat, um Parsing korrekt zu implementieren"
```
/answer 325f-2 allow
Control Request:
```yaml
id: 325f-3
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/registry.py
  reason: ToolResult/ToolDefinition Struktur verstehen für typisierte Rückgabeobjekte
```
/answer 325f-3 deny nicht relevant
ReasoningToken: 72
Control Request:
```yaml
id: 325f-4
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa.py
  reason: "Prüfen ob es bereits ein Muster für typisierte Resultobjekte in Schwesterdateien (exa.py) gibt, das für context7.py übernommen werden soll"
```
/answer 325f-4 deny lies nur Zeilen 100 bis 140
ReasoningToken: 195
Control Request:
```yaml
id: 325f-5
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa.py
  min_line: 100
  max_line: 140
  reason: "Prüfen ob es bereits ein Muster für typisierte Resultobjekte in Schwesterdateien (exa.py) gibt, das für context7.py übernommen werden soll"
```
/answer 325f-5 allow
Control Request:
```yaml
id: 325f-6
result:
  structuredContent:
    checksum: 1717b00d3f84d6b89f9fda0673e0c4dd47c1442a1e85f974e71b7e21613afacd
    content: |2+


      @dataclass(frozen=True, slots=True)
      class WebSearchResultItem:
          """One entry of a ``web_search_exa`` response; fields mirror Exa's payload."""

          title: str | None = None
          url: str | None = None
          published_date: str | None = None
          author: str | None = None
          score: float | None = None
          id: str | None = None
          text: str | None = None
          highlights: list[str] | None = None


      @dataclass(frozen=True, slots=True)
      class WebSearchResult:
          """Parsed ``web_search_exa`` response."""

          results: list[WebSearchResultItem]
          autoprompt_string: str | None = None


      @dataclass(frozen=True, slots=True)
      class WebFetchResultItem:
          """One entry of a ``web_fetch_exa`` response; fields mirror Exa's payload."""

          id: str | None = None
          url: str | None = None
          title: str | None = None
          text: str | None = None
          highlights: list[str] | None = None
          highlight_scores: list[float] | None = None
          summary: str | None = None
          author: str | None = None
          published_date: str | None = None
          image: str | None = None
          favicon: str | None = None
```
/answer 325f-6 allow
ReasoningToken: 40
Control Request:
```yaml
id: 325f-7
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa.py
  reason: "Vollständigen Code von exa.py lesen, um das Muster für typisierte Resultobjekte, Parsing-Funktionen und deren Verwendung in Tool-Handles zu verstehen"
```
/answer 325f-7 deny nicht relevant
ReasoningToken: 105
Control Request:
```yaml
id: 325f-8
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa.py
  min_line: 1
  max_line: 100
  reason: "Imports und Kontext vor den Dataclasses lesen, um das Muster vollständig zu verstehen"
```
/answer 325f-8 deny nicht relevant
Control Request:
```yaml
id: 325f-9
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa.py
  min_line: 140
  max_line: 260
  reason: Parsing-Funktionen und Tool-Handle-Verwendung der Dataclasses nach der Definition lesen
```
/answer 325f-9 deny nicht relevant
ReasoningToken: 23
Control Request:
```yaml
id: 325f-a
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/registry.py
  reason: "ToolResult-Klasse verstehen: wie structured_content aus Dataclass-Objekten gebaut wird (z.B. via dataclasses.asdict), um das gleiche Muster wie in exa.py für context7.py zu übernehmen"
```
/answer 325f-a deny asdict ist okay, im falle von `context7_documentation` Kann das MCP Output Schema auf eine Liste geändert werden.
ReasoningToken: 251
Control Request:
```yaml
id: 325f-b
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/context7.py
  mode: replace
  reason: "context7.py mit typisierten Resultobjekten (Library, DocumentationSection) und Parsing-Logik überschreiben"
  content: |
    """Context7 bridge – library documentation tools backed by the Context7 MCP server.

    Exposes two tools:
      context7_libraries      →  resolveLibraryId
      context7_documentation  →  queryDocs
    """


    import re
    from dataclasses import asdict, dataclass
    from typing import Any

    from xy.ai.mcpc.config import ServerConfig
    from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
    from xy.ai.mcpc.tools.mcp.bridge import McpBridge, McpBridgeError, compact
    from xy.ai.mcpc.tools.mcp.client import McpClient

    __all__ = [
        "Context7Bridge",
        "Library",
        "DocumentationSection",
        "context7_libraries",
        "context7_documentation",
        "Context7LibrariesTool",
        "Context7DocumentationTool",
        "register_context7_tools",
    ]

    _RESOLVE_DESCRIPTION = (
        "Search Context7 for a library and return its canonical library ID.\n\n"
        "Best for: Resolving a library name to the ID needed by context7_documentation.\n"
        "Returns: Ranked list of matching libraries with library ID, title, and description."
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
        "type": "array",
        "description": "Ranked list of matching libraries.",
        "items": {
            "type": "object",
            "properties": {
                "title": {"type": ["string", "null"]},
                "library_id": {
                    "type": ["string", "null"],
                    "description": "Context7-compatible library ID.",
                },
                "description": {"type": ["string", "null"]},
            },
        },
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
        "type": "array",
        "description": "Documentation, split into sections on '---' separators.",
        "items": {
            "type": "object",
            "properties": {
                "content": {"type": "string"},
            },
            "required": ["content"],
        },
    }

    _RO: dict[str, Any] = {"readOnlyHint": True, "openWorldHint": True}

    _NOT_FOUND_TRIGGER = "No documentation found for library"
    _BLOCK_SEPARATOR = re.compile(r"(?m)^-{3,}\s*$")
    _LIBRARY_FIELD = re.compile(r"(?m)^-\s*(.+?):\s*(.*)$")


    @dataclass(frozen=True, slots=True)
    class Library:
        """One Context7 library search result."""

        title: str | None = None
        library_id: str | None = None
        description: str | None = None


    @dataclass(frozen=True, slots=True)
    class DocumentationSection:
        """One documentation section of a Context7 ``queryDocs`` response."""

        content: str


    def _parse_libraries(text: str) -> list[Library]:
        libraries: list[Library] = []
        for block in _BLOCK_SEPARATOR.split(text):
            fields = {label.strip(): value.strip() for label, value in _LIBRARY_FIELD.findall(block)}
            if not fields:
                continue
            libraries.append(
                Library(
                    title=fields.get("Title"),
                    library_id=fields.get("Context7-compatible library ID"),
                    description=fields.get("Description"),
                )
            )
        return libraries


    def _parse_documentation(text: str) -> list[DocumentationSection]:
        if _NOT_FOUND_TRIGGER in text:
            raise McpBridgeError(text.strip())
        return [
            DocumentationSection(content=section.strip())
            for section in _BLOCK_SEPARATOR.split(text)
            if section.strip()
        ]


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


    def context7_libraries(libraryName: str, query: str) -> list[Library]:
        """Search Context7 for a library and return its canonical library ID.

        Best for: Resolving a library name to the ID needed by
        ``context7_documentation``.

        Args:
            libraryName: Library name to search for (e.g. 'react', 'next.js', 'vue').
            query: User's original question or task, used for relevance ranking.

        Returns:
            Ranked list of matching libraries (title, library ID, description).

        Raises:
            McpBridgeError: if the Context7 call fails.
        """
        result = _get_bridge().call("resolve-library-id", compact(libraryName=libraryName, query=query))
        return _parse_libraries(result.get("content", ""))


    def context7_documentation(libraryId: str, query: str) -> list[DocumentationSection]:
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
            Documentation, split into sections on '---' separators.

        Raises:
            McpBridgeError: if the Context7 call fails, or no documentation was found.
        """
        result = _get_bridge().call("query-docs", compact(libraryId=libraryId, query=query))
        return _parse_documentation(result.get("content", ""))


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
                libraries = context7_libraries(libraryName=args["libraryName"], query=args["query"])
            except McpBridgeError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content=[asdict(library) for library in libraries])


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
                sections = context7_documentation(libraryId=args["libraryId"], query=args["query"])
            except McpBridgeError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content=[asdict(section) for section in sections])


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
/answer 325f-b allow
Control Request:
```yaml
id: 325f-c
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/context7.py
  reason: "Geschriebene Datei zur Kontrolle nochmal lesen, bevor das Tool live getestet wird"
```
/answer 325f-c allow
Ich habe beide Tools für dich aktiviert
SystemInit:  id=3b94391a-5c8a-41eb-959b-2c0b140923e6, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Verstanden – ich habe aktuell aber noch keine konkrete Aufgabe von dir erhalten. Welche zwei Tools meinst du, und was soll ich damit für dich erledigen?
