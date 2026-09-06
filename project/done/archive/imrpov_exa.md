Verbessere die Exa Tools in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa.py`.

Exa API Doku: `/home/user/xyan/xy.ai.workbench/project/exa.md`

- Die ursprünglichen beiden Exa Tools in separate Dateien auslagern
- Das MCP Schema und Function von `web_fetch_exa` auf eine URL Liste reduzieren statt coerce, um die Verwendung zu vereinfachen.
- Folgende Felder aus beiden ResultItems entfernen: published_date, score, image, favicon, highlight_scores
- wird "id" im Exa result (beide) nicht mit übergeben soll eine random Hash ihren Platz einnehmen (6 Zeichen 0-9a-zA-Z)
- Leere Felder, Leere Arrays oder Null Werte, werden im MCP Result nicht ausgegeben
- Das Feld "highlights" wird umbenannt in "excerpt" (Mapping). Ist das Feld leer, wird es um einen Eintrag mit den ersten 100 Zeichen und um einen Eintrag mit den letzten 100 Zeichen ergänzt

Die Zweistufigkeit im Retrieval hat sich bewährt. Beide Tools sollen zweistufig werden.
Das heißt ein `web_search_exa` Aufruf, führt eine Suche durch und speichert die vollständigen Ergebnisse in einem Cache. Zurückgegeben wird nur eine Ergebnisliste in der, der Volltext ('text') und die URL nicht vorkommt.
Ein weiteres Tool wird erstellt('web_search_exa_results') das eine Liste von IDs empfängt und daraufhin eine Liste von Ergebnissen mit ID, URL und Volltext zurückgibt.

Das Tool "web_fetch_exa" führt einen fetch durch, speichert die Ergebnisse unter den IDs im cache und gibt nur eine Übersichtsliste mit Metriken zurück unter der Verwendung und im Format von `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py` (wiederverwenden, innere Funktion extrahieren und publizieren auf Basis von string)
Erstelle ein weiteres Tool "web_fetch_exa_results" das eine Liste von IDs akzeptiert und erst daraufhin URL und Volltexte liefert.
Dieses Tool soll auch die Zeilenweise Filterung per extended Regexp analog "grep -e" erlauben, sowie einen analogen Kontext Parameter der im Default auch die vorhergehende und folgende Zeile mit ausgibt (Kontext: 1).
Alle 4 Exa Tools werden unter dem alias "exa" registriert (Beispiel: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py`).
SystemInit:  id=c6aa0e76-64c9-413d-a9b7-8dc32f91b16f, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: c564-1
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa.py
  ids:
  - root
  reason: "Aktuellen Stand der Exa Tools lesen, um sie zu refaktorieren"
```
/answer c564-1 allow
Control Request:
```yaml
id: c564-2
result:
  content:
  - type: text
    text: No node matched 'root'.
  isError: true
```
/answer c564-2 allow
Control Request:
```yaml
id: c564-3
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
  reason: Struktur der file_stats Implementierung ansehen für Wiederverwendung im fetch tool
```
/answer c564-3 allow
Control Request:
```yaml
id: c564-4
result:
  structuredContent:
    nodes:
    - id: 2qVAW6
      type: statements
    - id: 7pKq8V
      type: imports
    - id: 5xkcFZ
      type: statements
    - id: FileStatsError
      type: ClassDef
      signature: "class FileStatsError(Exception):"
      docstring: Raised when file metrics cannot be computed.
    - id: FileStatsResult
      type: ClassDef
      signature: "@dataclass(frozen=True) class FileStatsResult:"
    - id: calculate_complexity
      type: FunctionDef
      signature: "def _calculate_complexity(text: str) -> float:"
      docstring: Calculate data structure complexity (0.0 to 1.0). Based on character set divers…
    - id: compute_file_stats
      type: FunctionDef
      signature: "def compute_file_stats(path: Path) -> FileStatsResult:"
      docstring: Compute the file-metrics block for *path* (also reused by the outline tool). As…
    - id: file_stats
      type: FunctionDef
      signature: "def file_stats(path: str) -> FileStatsResult:"
      docstring: "Compute file metrics for the absolute path ``path``. Args: path: Absolute path …"
    - id: FileStatsTool
      type: ClassDef
      signature: "class FileStatsTool(ToolDefinition):"
      children:
      - id: FileStatsTool.sRkGMF
        type: statements
      - id: FileStatsTool.WU5XVz
        type: statements
      - id: FileStatsTool.e5owbr
        type: statements
      - id: FileStatsTool.handle
        type: FunctionDef
        signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
        docstring: "Delegate to :func:`file_stats`, translating the MCP schema to/from the Python A…"
    - id: register_file_stats_tool
      type: FunctionDef
      signature: "def register_file_stats_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer c564-4 allow
Control Request:
```yaml
id: c564-5
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
  reason: Struktur des ast Tools ansehen als Beispiel für alias-Registrierung mehrerer Tools
```
/answer c564-5 allow
Control Request:
```yaml
id: c564-6
result:
  structuredContent:
    nodes:
    - id: b2Cas8
      type: statements
    - id: lDD4c5
      type: imports
    - id: 7P2hSl
      type: statements
    - id: register_ast_tools
      type: FunctionDef
      signature: "def register_ast_tools(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
      docstring: Register every ``ast_*`` tool and the ``ast`` alias.
```
/answer c564-6 allow
ReasoningToken: 14
Control Request:
```yaml
id: c564-7
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa.py
  reason: Struktur der aktuellen Exa Tools ermitteln
```
/answer c564-7 allow
Control Request:
```yaml
id: c564-8
result:
  structuredContent:
    nodes:
    - id: koroB7
      type: statements
    - id: PluoNM
      type: imports
    - id: AX0njS
      type: statements
    - id: 0t22JW
      type: statements
    - id: EiBYOQ
      type: statements
    - id: lNFmBV
      type: statements
    - id: eEFMDR
      type: statements
    - id: WebSearchResultItem
      type: ClassDef
      signature: "@dataclass(frozen=True, slots=True) class WebSearchResultItem:"
      docstring: One entry of a ``web_search_exa`` response; fields mirror Exa's payload.
    - id: WebSearchResult
      type: ClassDef
      signature: "@dataclass(frozen=True, slots=True) class WebSearchResult:"
      docstring: Parsed ``web_search_exa`` response.
    - id: WebFetchResultItem
      type: ClassDef
      signature: "@dataclass(frozen=True, slots=True) class WebFetchResultItem:"
      docstring: One entry of a ``web_fetch_exa`` response; fields mirror Exa's payload.
    - id: WebFetchResult
      type: ClassDef
      signature: "@dataclass(frozen=True, slots=True) class WebFetchResult:"
      docstring: Parsed ``web_fetch_exa`` response.
    - id: parse_search_result
      type: FunctionDef
      signature: "def _parse_search_result(raw: dict[str, Any]) -> WebSearchResult:"
    - id: parse_fetch_result
      type: FunctionDef
      signature: "def _parse_fetch_result(raw: dict[str, Any]) -> WebFetchResult:"
    - id: coerce_urls
      type: FunctionDef
      signature: "def _coerce_urls(urls: list[str] | str) -> list[str]:"
      docstring: Accept a single URL or a JSON-encoded list for ``urls`` leniently.
    - id: ExaBridge
      type: ClassDef
      signature: "class ExaBridge(McpBridge):"
      docstring: Bridge to the Exa remote MCP server.
      children:
      - id: ExaBridge.0FVPg9
        type: statements
      - id: ExaBridge.build_client
        type: FunctionDef
        signature: "def build_client(self, config: ServerConfig) -> McpClient:"
    - id: i2zrJD
      type: statements
    - id: get_bridge
      type: FunctionDef
      signature: "def _get_bridge() -> ExaBridge:"
      docstring: Return the module-level Exa bridge configured by :func:`register_exa_tools`.
    - id: web_search_exa_raw
      type: FunctionDef
      signature: "def _web_search_exa_raw(query: str, numResults: int | None=None) -> dict[str, Any]:"
    - id: web_search_exa
      type: FunctionDef
      signature: "def web_search_exa(query: str, numResults: int | None=None) -> WebSearchResult:"
      docstring: "Search the web for any topic and get clean, ready-to-use content. Best for: Fin…"
    - id: web_fetch_exa_raw
      type: FunctionDef
      signature: "def _web_fetch_exa_raw(urls: list[str] | str, maxCharacters: int | None=None) -> dict[str, Any]:"
    - id: web_fetch_exa
      type: FunctionDef
      signature: "def web_fetch_exa(urls: list[str] | str, maxCharacters: int | None=None) -> WebFetchResult:"
      docstring: "Read a webpage's full content as clean markdown. Best for: Extracting full cont…"
    - id: WebSearchExaTool
      type: ClassDef
      signature: "class WebSearchExaTool(ToolDefinition):"
      children:
      - id: WebSearchExaTool.q9ndig
        type: statements
      - id: WebSearchExaTool.handle
        type: FunctionDef
        signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
    - id: WebFetchExaTool
      type: ClassDef
      signature: "class WebFetchExaTool(ToolDefinition):"
      children:
      - id: WebFetchExaTool.eeITsF
        type: statements
      - id: WebFetchExaTool.handle
        type: FunctionDef
        signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
    - id: register_exa_tools
      type: FunctionDef
      signature: "def register_exa_tools(registry: ToolRegistry, environment: AppEnvironment) -> None:"
      docstring: Register the Exa-backed ``web_search_exa`` and ``web_fetch_exa`` tools.
```
/answer c564-8 allow
Control Request:
```yaml
id: c564-9
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
  ids:
  - 2qVAW6
  - 7pKq8V
  - 5xkcFZ
  - FileStatsError
  - FileStatsResult
  - calculate_complexity
  - compute_file_stats
  - file_stats
  - FileStatsTool
  - register_file_stats_tool
  reason: Vollständigen Code der file_stats Implementierung lesen für Wiederverwendung im fetch tool
```
/answer c564-9 allow
Control Request:
```yaml
id: c564-a
result:
  structuredContent:
    nodes:
    - id: 2qVAW6
      type: statements
      code: "'File stats tool – returns file metrics for access and processing planning.\\n\\nProvides compact metrics including complexity, timestamps, size, line/word\\ncounts, line length statistics, and average words per line.\\n'"
    - id: 7pKq8V
      type: imports
      code: |-
        import hashlib
        import re
        from dataclasses import asdict, dataclass
        from datetime import datetime, timezone
        from pathlib import Path
        from typing import Any
        from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
        from xy.ai.mcpc.tools.tool_context import ToolContext
        from xy.ai.mcpc.tools.function_registry import FunctionRegistry
    - id: 5xkcFZ
      type: statements
      code: |-
        __all__ = [
            'FileStatsError',
            'FileStatsResult',
            'compute_file_stats',
            'file_stats',
            'FileStatsTool',
            'register_file_stats_tool']
    - id: FileStatsError
      type: ClassDef
      code: |-
        class FileStatsError(Exception):
            """Raised when file metrics cannot be computed."""
    - id: FileStatsResult
      type: ClassDef
      code: |-
        @dataclass(frozen=True)
        class FileStatsResult:
            path: str
            size_bytes: int
            lines: int
            words: int
            complexity: float
            created: str
            modified: str
            accessed: str
            line_length_max: int
            line_length_min: int
            line_length_avg: float
            words_per_line_avg: float
            checksum: str
    - id: calculate_complexity
      type: FunctionDef
      code: |-
        def _calculate_complexity(text: str) -> float:
            """Calculate data structure complexity (0.0 to 1.0).

            Based on character set diversity and pattern variation.
            """
            if not text:
                return 0.0
            has_alpha = bool(re.search('[a-zA-Z]', text))
            has_digit = bool(re.search('\\d', text))
            has_punct = bool(re.search('[^\\w\\s]', text))
            has_space = bool(re.search('\\s', text))
            has_upper = bool(re.search('[A-Z]', text))
            has_lower = bool(re.search('[a-z]', text))
            char_type_score = sum([has_alpha, has_digit, has_punct, has_space, has_upper, has_lower]) / 6.0
            unique_chars = len(set(text))
            entropy_score = min(1.0, unique_chars / 256.0)
            complexity = char_type_score * 0.4 + entropy_score * 0.6
            return round(complexity, 3)
    - id: compute_file_stats
      type: FunctionDef
      code: |-
        def compute_file_stats(path: Path) -> FileStatsResult:
            """Compute the file-metrics block for *path* (also reused by the outline tool).

            Assumes *path* is an existing regular file.
            """
            raw_bytes = path.read_bytes()
            text = raw_bytes.decode('utf-8', errors='replace')
            lines = text.splitlines()
            size_bytes = len(raw_bytes)
            num_lines = len(lines)
            num_words = len(text.split())
            complexity = _calculate_complexity(text)
            checksum = hashlib.sha256(text.encode('utf-8')).hexdigest()
            line_lengths = [len(line) for line in lines] if lines else [0]
            line_length_max = max(line_lengths) if line_lengths else 0
            line_length_min = min(line_lengths) if line_lengths else 0
            line_length_avg = round(sum(line_lengths) / len(line_lengths), 2) if line_lengths else 0.0
            words_per_line = [len(line.split()) for line in lines]
            words_per_line_avg = round(sum(words_per_line) / len(words_per_line), 2) if words_per_line else 0.0
            stat = path.stat()
            created = datetime.fromtimestamp(stat.st_birthtime if hasattr(stat, 'st_birthtime')
                                             else stat.st_mtime, tz=timezone.utc).isoformat()
            modified = datetime.fromtimestamp(stat.st_mtime, tz=timezone.utc).isoformat()
            accessed = datetime.fromtimestamp(stat.st_atime, tz=timezone.utc).isoformat()
            return FileStatsResult(
                path=str(
                    path.resolve()),
                size_bytes=size_bytes,
                lines=num_lines,
                words=num_words,
                complexity=complexity,
                created=created,
                modified=modified,
                accessed=accessed,
                line_length_max=line_length_max,
                line_length_min=line_length_min,
                line_length_avg=line_length_avg,
                words_per_line_avg=words_per_line_avg,
                checksum=checksum)
    - id: file_stats
      type: FunctionDef
      code: |-
        def file_stats(path: str) -> FileStatsResult:
            """Compute file metrics for the absolute path ``path``.

            Args:
                path: Absolute path to file (must exist and be a regular file).

            Returns:
                FileStatsResult with:
                    size_bytes: File size in bytes.
                    lines: Total number of lines.
                    words: Total number of words (whitespace-split).
                    complexity: Data structure complexity score (0.0 to 1.0, based on character diversity).
                    created: File creation timestamp (ISO format, UTC).
                    modified: Last modification timestamp (ISO format, UTC).
                    accessed: Last access timestamp (ISO format, UTC).
                    line_length_max: Longest line length in characters.
                    line_length_min: Shortest line length in characters.
                    line_length_avg: Average line length (rounded to 2 decimals).
                    words_per_line_avg: Average words per line (rounded to 2 decimals).
                    checksum: SHA256 checksum of file content.

            Raises:
                FileStatsError: If path is not absolute.
                FileStatsError: If file not found or not a regular file.

            Note:
                Binary files are decoded as UTF-8 with error replacement.
                Timestamps use fallback to mtime if birthtime not available (Linux).
            """
            file_path = Path(path)
            if not file_path.is_absolute():
                raise FileStatsError('Path must be absolute.')
            if not file_path.exists():
                raise FileStatsError('File not found.')
            if not file_path.is_file():
                raise FileStatsError('Not a regular file.')
            return compute_file_stats(file_path)
    - id: FileStatsTool
      type: ClassDef
      signature: "class FileStatsTool(ToolDefinition):"
      children:
      - id: FileStatsTool.sRkGMF
        type: statements
        code: |-
          name = 'file_stats'
          title = 'File stats'
          description = 'Get file metrics for access and processing planning: complexity, timestamps, size, line/word counts, and line length statistics.'
          input_schema = {
              'type': 'object',
              'properties': {
                  'path': {
                      'type': 'string',
                      'description': 'Absolute file path.'}},
              'required': ['path']}
      - id: FileStatsTool.WU5XVz
        type: statements
        code: |-
          output_schema = {
              'type': 'object',
              'properties': {
                  'path': {
                      'type': 'string',
                      'description': 'Absolute file path.'},
                  'size_bytes': {
                      'type': 'integer',
                      'description': 'File size in bytes.'},
                  'lines': {
                      'type': 'integer',
                              'description': 'Total number of lines.'},
                  'words': {
                      'type': 'integer',
                      'description': 'Total number of words.'},
                  'complexity': {
                      'type': 'number',
                      'description': 'Data structure complexity (0.0 to 1.0).'},
                  'created': {
                      'type': 'string',
                      'description': 'Creation timestamp (ISO 8601).'},
                  'modified': {
                      'type': 'string',
                      'description': 'Last modification timestamp (ISO 8601).'},
                  'accessed': {
                      'type': 'string',
                      'description': 'Last access timestamp (ISO 8601).'},
                  'line_length_max': {
                      'type': 'integer',
                      'description': 'Maximum line length in characters.'},
                  'line_length_min': {
                      'type': 'integer',
                      'description': 'Minimum line length in characters.'},
                  'line_length_avg': {
                      'type': 'number',
                      'description': 'Average line length in characters.'},
                  'words_per_line_avg': {
                      'type': 'number',
                      'description': 'Average number of words per line.'},
                  'checksum': {
                      'type': 'string',
                      'description': 'sha256 checksum of the file content.'}},
              'required': [
                  'path',
                  'size_bytes',
                  'lines',
                  'words',
                  'complexity',
                  'created',
                  'modified',
                  'accessed',
                  'line_length_max',
                  'line_length_min',
                  'line_length_avg',
                  'words_per_line_avg',
                  'checksum']}
      - id: FileStatsTool.e5owbr
        type: statements
        code: "annotations = {'readOnlyHint': True, 'openWorldHint': False}"
      - id: FileStatsTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`file_stats`, translating the MCP schema to/from the Python API."""
              args: dict[str, Any] = ctx.arguments
              try:
                  result = file_stats(args['path'])
              except FileStatsError as exc:
                  return ToolResult(content=[text_content(str(exc))], is_error=True)
              return ToolResult(content=[], structured_content=asdict(result), auto_approve=True)
    - id: register_file_stats_tool
      type: FunctionDef
      code: |-
        def register_file_stats_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
            registry.register(FileStatsTool())
            functions.register(file_stats)
```
/answer c564-a allow
Control Request:
```yaml
id: c564-b
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
  ids:
  - b2Cas8
  - lDD4c5
  - 7P2hSl
  - register_ast_tools
  reason: Vollständigen Code des ast Alias-Registrierungsmusters lesen
```
/answer c564-b allow
Control Request:
```yaml
id: c564-c
result:
  structuredContent:
    nodes:
    - id: b2Cas8
      type: statements
      code: "'``ast_*`` tool family built on the standard-library ``ast`` module.\\n\\nA content-hash validated cache (:mod:`.core`) holds parsed modules; comments are\\nconverted to standalone string-literal annotations on import so they survive the\\n``parse``/``unparse`` round-trip. Retrieval is layered on a\\nsingle ``list`` tree (``ast_list`` structure, ``ast_find`` property/text/regexp\\nfiltering with source, ``ast_read`` reads subtrees by id); mutation is\\nnode-level CRUD, each tool in its own ``*`` module (``ast_create``/``ast_delete``\\ncover the whole-file case too), with two in-node editors ``ast_edit_marks``\\n(marker-delimited) and ``ast_edit_block`` (exact block), a restricted ``script``\\nand a ``validate`` compile check.\\n'"
    - id: lDD4c5
      type: imports
      code: |-
        from xy.ai.mcpc.tools.tool_registry import ToolRegistry
        from xy.ai.mcpc.tools.function_registry import FunctionRegistry
        from xy.ai.mcpc.tools.ast import create, delete, edit_block, edit_marks, find, insert, list, read, replace, script, validate
    - id: 7P2hSl
      type: statements
      code: |-
        __all__ = ['register_ast_tools', 'ALIAS']
        '#: Alias name that activates the whole family in one go.'
        ALIAS = 'ast'
        _ALIAS_MEMBERS = (
            'ast_list',
            'ast_find',
            'ast_read',
            'ast_insert',
            'ast_edit_marks',
            'ast_edit_block',
            'ast_replace',
            'ast_delete',
            'ast_create',
            'ast_script',
            'ast_validate')
    - id: register_ast_tools
      type: FunctionDef
      code: |-
        def register_ast_tools(registry: ToolRegistry, functions: FunctionRegistry) -> None:
            """Register every ``ast_*`` tool and the ``ast`` alias."""
            list.register(registry, functions)
            find.register(registry, functions)
            read.register(registry, functions)
            insert.register(registry, functions)
            edit_marks.register(registry, functions)
            edit_block.register(registry, functions)
            replace.register(registry, functions)
            delete.register(registry, functions)
            create.register(registry, functions)
            script.register(registry, functions)
            validate.register(registry, functions)
            registry.register_alias(ALIAS, _ALIAS_MEMBERS)
```
/answer c564-c allow
ReasoningToken: 38
Control Request:
```yaml
id: c564-d
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa.py
  ids:
  - koroB7
  - PluoNM
  - AX0njS
  - 0t22JW
  - EiBYOQ
  - lNFmBV
  - eEFMDR
  - WebSearchResultItem
  - WebSearchResult
  - WebFetchResultItem
  - WebFetchResult
  - parse_search_result
  - parse_fetch_result
  - coerce_urls
  - ExaBridge
  - i2zrJD
  - get_bridge
  - web_search_exa_raw
  - web_search_exa
  - web_fetch_exa_raw
  - web_fetch_exa
  - WebSearchExaTool
  - WebFetchExaTool
  - register_exa_tools
  reason: Vollständigen aktuellen Code aller Exa-Bestandteile lesen
```
/answer c564-d allow
Control Request:
```yaml
id: c564-e
result:
  structuredContent:
    nodes:
    - id: koroB7
      type: statements
      code: '"Exa bridge – forwards ``web_search_exa`` and ``web_fetch_exa`` to Exa''s MCP.\n\nExa is reached through its remote MCP server; the API key is taken from the\nserver configuration.  MCPC advertises its own descriptions and input schemas.\n"'
    - id: PluoNM
      type: imports
      code: |-
        from dataclasses import dataclass
        from typing import Any
        from xy.ai.mcpc.server.json_codec import JsonCodec
        from xy.ai.mcpc.config import ServerConfig
        from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
        from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
        from xy.ai.mcpc.tools.mcp.bridge import McpBridge, McpBridgeError, compact
        from xy.ai.mcpc.tools.mcp.client import McpClient, McpClientError
    - id: AX0njS
      type: statements
      code: |-
        __all__ = [
            'ExaBridge',
            'WebSearchResultItem',
            'WebSearchResult',
            'WebFetchResultItem',
            'WebFetchResult',
            'web_search_exa',
            'web_fetch_exa',
            'WebSearchExaTool',
            'WebFetchExaTool',
            'register_exa_tools']
        _WEB_SEARCH_DESCRIPTION = 'Search the web for any topic and get clean, ready-to-use content.\n\nBest for: Finding current information, facts, or answering questions about any topic.\nReturns: Clean text content from top search results.'
    - id: 0t22JW
      type: statements
      code: |-
        _WEB_SEARCH_SCHEMA: dict[str,
                                 Any] = {'type': 'object',
                                         'properties': {'query': {'type': 'string',
                                                                  'description': 'Natural language search query. Should be a semantically rich description of the ideal page.'},
                                                        'numResults': {'type': 'integer',
                                                                       'description': 'Number of search results to return (default: 10).',
                                                                       'minimum': 1}},
                                         'required': ['query']}
    - id: EiBYOQ
      type: statements
      code: |-
        _SEARCH_OUTPUT_SCHEMA: dict[str, Any] = {'type': 'object', 'properties': {'content': {
            'type': 'string', 'description': 'Clean text content from the top search results.'}}, 'required': ['content']}
        _WEB_FETCH_DESCRIPTION = "Read a webpage's full content as clean markdown. Use to read any URL.\n\nBest for: Extracting full content from known URLs. Batch multiple URLs in one call.\nReturns: Clean text content and metadata from the page(s)."
    - id: lNFmBV
      type: statements
      code: |-
        _WEB_FETCH_SCHEMA: dict[str,
                                Any] = {'type': 'object',
                                        'properties': {'urls': {'type': 'array',
                                                                'items': {'type': 'string'},
                                                                'description': 'URLs to read. Batch multiple URLs in one call.'},
                                                       'maxCharacters': {'type': 'integer',
                                                                         'description': 'Maximum characters to extract per page (default: 3000)',
                                                                         'minimum': 1}},
                                        'required': ['urls']}
    - id: eEFMDR
      type: statements
      code: |-
        _FETCH_OUTPUT_SCHEMA: dict[str, Any] = {'type': 'object', 'properties': {'content': {
            'type': 'string', 'description': 'Clean text content extracted from the requested page(s).'}}, 'required': ['content']}
        _RO: dict[str, Any] = {'readOnlyHint': True, 'openWorldHint': True}
    - id: WebSearchResultItem
      type: ClassDef
      code: |-
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
    - id: WebSearchResult
      type: ClassDef
      code: |-
        @dataclass(frozen=True, slots=True)
        class WebSearchResult:
            """Parsed ``web_search_exa`` response."""
            results: list[WebSearchResultItem]
            autoprompt_string: str | None = None
    - id: WebFetchResultItem
      type: ClassDef
      code: |-
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
    - id: WebFetchResult
      type: ClassDef
      code: |-
        @dataclass(frozen=True, slots=True)
        class WebFetchResult:
            """Parsed ``web_fetch_exa`` response."""
            results: list[WebFetchResultItem]
    - id: parse_search_result
      type: FunctionDef
      code: |-
        def _parse_search_result(raw: dict[str, Any]) -> WebSearchResult:
            items = [WebSearchResultItem(**item) for item in raw.get('results', [])]
            return WebSearchResult(results=items, autoprompt_string=raw.get('autoprompt_string'))
    - id: parse_fetch_result
      type: FunctionDef
      code: |-
        def _parse_fetch_result(raw: dict[str, Any]) -> WebFetchResult:
            items = [WebFetchResultItem(**item) for item in raw.get('results', [])]
            return WebFetchResult(results=items)
    - id: coerce_urls
      type: FunctionDef
      code: |-
        def _coerce_urls(urls: list[str] | str) -> list[str]:
            """Accept a single URL or a JSON-encoded list for ``urls`` leniently."""
            if isinstance(urls, str):
                '# Accept a JSON-encoded list carried as a string; a plain URL (or any'
                '# non-list) is wrapped as a single-element list.'
                parsed = JsonCodec.try_decode(urls)
                return parsed if isinstance(parsed, list) else [urls]
            return urls
    - id: ExaBridge
      type: ClassDef
      signature: "class ExaBridge(McpBridge):"
      docstring: Bridge to the Exa remote MCP server.
      children:
      - id: ExaBridge.0FVPg9
        type: statements
        code: '''Bridge to the Exa remote MCP server.'''
      - id: ExaBridge.build_client
        type: FunctionDef
        code: |-
          def build_client(self, config: ServerConfig) -> McpClient:
              api_key = config.exa_api_key
              if not api_key:
                  raise McpClientError('Exa API key is not configured (set MCPC_EXA_API_KEY / EXA_API_KEY).')
              return McpClient(config.exa_mcp_url, headers={'x-api-key': api_key})
    - id: i2zrJD
      type: statements
      code: |-
        '#: Module-level bridge, built by :func:`register_exa_tools`.'
        _bridge: ExaBridge | None = None
    - id: get_bridge
      type: FunctionDef
      code: |-
        def _get_bridge() -> ExaBridge:
            """Return the module-level Exa bridge configured by :func:`register_exa_tools`."""
            if _bridge is None:
                raise McpBridgeError('Exa tools used before register_exa_tools() was called.')
            return _bridge
    - id: web_search_exa_raw
      type: FunctionDef
      code: |-
        def _web_search_exa_raw(query: str, numResults: int | None=None) -> dict[str, Any]:
            return _get_bridge().call('web_search_exa', compact(query=query, numResults=numResults), type='deep')
    - id: web_search_exa
      type: FunctionDef
      code: |-
        def web_search_exa(query: str, numResults: int | None=None) -> WebSearchResult:
            """Search the web for any topic and get clean, ready-to-use content.

            Best for: Finding current information, facts, or answering questions
            about any topic.

            Args:
                query: Natural language search query; should be a semantically
                    rich description of the ideal page.
                numResults: Number of search results to return (default: 10).

            Returns:
                Parsed search results.

            Raises:
                McpBridgeError: if the Exa call fails.
            """
            return _parse_search_result(_web_search_exa_raw(query, numResults))
    - id: web_fetch_exa_raw
      type: FunctionDef
      code: |-
        def _web_fetch_exa_raw(urls: list[str] | str, maxCharacters: int | None=None) -> dict[str, Any]:
            arguments = compact(urls=_coerce_urls(urls), maxCharacters=maxCharacters)
            return _get_bridge().call('web_fetch_exa', arguments)
    - id: web_fetch_exa
      type: FunctionDef
      code: |-
        def web_fetch_exa(urls: list[str] | str, maxCharacters: int | None=None) -> WebFetchResult:
            """Read a webpage's full content as clean markdown.

            Best for: Extracting full content from known URLs. Batch multiple
            URLs in one call.

            Args:
                urls: URL(s) to read; a single URL or a list of URLs.
                maxCharacters: Maximum characters to extract per page (default: 3000).

            Returns:
                Parsed page content and metadata.

            Raises:
                McpBridgeError: if the Exa call fails.
            """
            return _parse_fetch_result(_web_fetch_exa_raw(urls, maxCharacters))
    - id: WebSearchExaTool
      type: ClassDef
      signature: "class WebSearchExaTool(ToolDefinition):"
      children:
      - id: WebSearchExaTool.q9ndig
        type: statements
        code: |-
          name = 'web_search_exa'
          title = 'Exa web search'
          description = _WEB_SEARCH_DESCRIPTION
          input_schema = _WEB_SEARCH_SCHEMA
          output_schema = _SEARCH_OUTPUT_SCHEMA
          annotations = _RO
      - id: WebSearchExaTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              args = ctx.arguments
              try:
                  result = _web_search_exa_raw(query=args['query'], numResults=args.get('numResults'))
              except McpBridgeError as exc:
                  return ToolResult(content=[text_content(str(exc))], is_error=True)
              return ToolResult(structured_content=result)
    - id: WebFetchExaTool
      type: ClassDef
      signature: "class WebFetchExaTool(ToolDefinition):"
      children:
      - id: WebFetchExaTool.eeITsF
        type: statements
        code: |-
          name = 'web_fetch_exa'
          title = 'Exa web fetch'
          description = _WEB_FETCH_DESCRIPTION
          input_schema = _WEB_FETCH_SCHEMA
          output_schema = _FETCH_OUTPUT_SCHEMA
          annotations = _RO
      - id: WebFetchExaTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              args = ctx.arguments
              try:
                  result = _web_fetch_exa_raw(urls=args['urls'], maxCharacters=args.get('maxCharacters'))
              except McpBridgeError as exc:
                  return ToolResult(content=[text_content(str(exc))], is_error=True)
              return ToolResult(structured_content=result)
    - id: register_exa_tools
      type: FunctionDef
      code: |-
        def register_exa_tools(registry: ToolRegistry, environment: AppEnvironment) -> None:
            """Register the Exa-backed ``web_search_exa`` and ``web_fetch_exa`` tools."""
            global _bridge
            _bridge = ExaBridge(environment.config)
            registry.register(WebSearchExaTool())
            registry.register(WebFetchExaTool())
            environment.functions.register(web_search_exa)
            environment.functions.register(web_fetch_exa)
```
/answer c564-e allow
Control Request:
```yaml
id: c564-f
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
  reason: Struktur eines einzelnen ast_* Tool-Moduls als Vorlage ansehen
```
/answer c564-f deny nicht relevant
Control Request:
```yaml
id: c564-10
toolName: grep
arguments:
  directory:
  - /home/user/xyan/xy.ai.workbench/mcpc/src
  pattern: register_exa_tools|register_alias|AppEnvironment
  include:
  - '*.py'
  exclude:
  - '*.min.js'
  reason: Aufrufstellen und Signatur von register_exa_tools sowie register_alias/AppEnvironment finden
```
/answer c564-10 deny nicht relevant
ReasoningToken: 225
Control Request:
```yaml
id: c564-11
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
  reason: Referenzstruktur eines einzelnen ast_*-Tool-Moduls (create.py) als Vorlage für die Aufteilung der Exa-Tools in eigene Dateien ansehen
```
/answer c564-11 deny nicht relevant
Control Request:
```yaml
id: c564-12
toolName: grep
arguments:
  directory:
  - /home/user/xyan/xy.ai.workbench/mcpc/src
  pattern: register_exa_tools\(
  include:
  - '*.py'
  exclude: []
  reason: "Herausfinden, wo register_exa_tools aktuell aufgerufen wird, um die neue register_exa_tools-Signatur konsistent zu halten"
```
/answer c564-12 allow
Control Request:
```yaml
id: c564-13
result:
  structuredContent:
    matches:
    - path: xy/ai/mcpc/tools/mcp/exa.py
      lineno: 119
      match: '        raise McpBridgeError(''Exa tools used before register_exa_tools() was called.'')'
    - path: xy/ai/mcpc/tools/mcp/exa.py
      lineno: 198
      match: "def register_exa_tools(registry: ToolRegistry, environment: AppEnvironment) -> None:"
    - path: xy/ai/mcpc/tools/__init__.py
      lineno: 53
      match: "    register_exa_tools(registry, environment)"
```
/answer c564-13 allow
Control Request:
```yaml
id: c564-14
toolName: grep
arguments:
  directory:
  - /home/user/xyan/xy.ai.workbench/mcpc/src
  pattern: def register_alias|class ToolRegistry|class FunctionRegistry|class ToolContext|class AppEnvironment
  include:
  - '*.py'
  exclude: []
  reason: "Signaturen von ToolRegistry.register_alias, FunctionRegistry, ToolContext und AppEnvironment finden, um neue Tools korrekt zu implementieren"
```
/answer c564-14 allow
Control Request:
```yaml
id: c564-15
result:
  structuredContent:
    matches:
    - path: xy/ai/mcpc/tools/tool_context.py
      lineno: 15
      match: "class ToolContext:"
    - path: xy/ai/mcpc/tools/tool_context.py
      lineno: 22
      match: "class AppEnvironment:"
    - path: xy/ai/mcpc/tools/tool_registry.py
      lineno: 187
      match: "class ToolRegistry:"
    - path: xy/ai/mcpc/tools/tool_registry.py
      lineno: 196
      match: "    def register_alias(self, alias: str, members: 'Iterable[str]') -> None:"
    - path: xy/ai/mcpc/tools/function_registry.py
      lineno: 32
      match: "class FunctionRegistry:"
```
/answer c564-15 allow
ReasoningToken: 178
Control Request:
```yaml
id: c564-16
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_context.py
  ids:
  - ToolContext
  - AppEnvironment
  reason: "Definition von ToolContext und AppEnvironment lesen, um die Exa-Tool-Handler korrekt zu implementieren"
```
/answer c564-16 deny nicht relevant
Control Request:
```yaml
id: c564-17
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
  ids:
  - ToolRegistry
  reason: register_alias und register-Methoden von ToolRegistry lesen für die Registrierung der neuen Exa-Tools
```
/answer c564-17 deny nicht relevant
Control Request:
```yaml
id: c564-18
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/function_registry.py
  ids:
  - FunctionRegistry
  reason: "FunctionRegistry API lesen, um Python-Funktionen der neuen Exa-Tools zu registrieren"
```
/answer c564-18 deny nicht relevant
ReasoningToken: 239
Control Request:
```yaml
id: c564-19
toolName: grep
arguments:
  directory:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
  pattern: class ToolContext|class AppEnvironment|class ToolRegistry|def register|class FunctionRegistry|def register_alias|class ToolResult|def text_content|class ToolDefinition
  include:
  - tool_context.py
  - tool_registry.py
  - function_registry.py
  exclude: []
  limit: 50
  reason: "Zeilennummern und Signaturen der Kern-APIs (ToolContext, AppEnvironment, ToolRegistry, FunctionRegistry, ToolDefinition) finden, um neue Exa-Tools korrekt zu implementieren"
```
/answer c564-19 deny nicht relevant
ReasoningToken: 187
Control Request:
```yaml
id: c564-1a
toolName: ast_find
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
  node_type: ClassDef
  name: ToolRegistry
  reason: "register_alias und register-Methoden der ToolRegistry-Klasse lesen, um die vier neuen Exa-Tools korrekt zu registrieren (benötigt für Umbau von exa.py wie vom Nutzer beauftragt)"
```
/answer c564-1a deny nicht relevant
ReasoningToken: 391
Control Request:
```yaml
id: c564-1b
toolName: ask_user
arguments:
  question: "Ich kann aktuell keine weiteren Dateien außerhalb von exa.py, file_stats/__init__.py und ast/__init__.py lesen (z.B. tool_registry.py, tool_context.py, function_registry.py werden mit \"DENIED: nicht relevant\" abgelehnt), obwohl ich die genauen Signaturen von ToolContext, AppEnvironment, ToolRegistry.register_alias, FunctionRegistry etc. brauche, um die neuen Tools korrekt zu implementieren. Kannst du den Zugriff freigeben, oder soll ich mich ausschließlich auf die Muster aus den bereits gelesenen Dateien (exa.py, file_stats/__init__.py, ast/__init__.py) stützen?"
  reason: "Wiederholte Denials beim Lesen von Kern-Infrastrukturdateien, die für eine korrekte Implementierung der neuen Exa-Tools nötig sind."
```
/answer c564-1b deny Das ist nicht notwendig, du hast bereits die Konventionen und aufrufe der Exa-Tools. Du musst nicht verstehen wie die Registry funktioniert um einfach die register Function aufzuzurufen.
ReasoningToken: 24
Control Request:
```yaml
id: c564-1c
toolName: ast_find
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/bridge.py
  name: compact
  reason: "compact()-Hilfsfunktion aus bridge.py lesen (wird bereits in exa.py verwendet), um sie für das Entfernen leerer Felder im neuen Exa-Code zu benutzen"
```
/answer c564-1c allow
Control Request:
```yaml
id: c564-1d
result:
  structuredContent:
    nodes:
    - id: compact
      type: FunctionDef
      code: |-
        def compact(**kwargs: Any) -> dict[str, Any]:
            """Build a remote-call argument dict, dropping keys whose value is ``None``.

            Shared helper for the core functions in ``context7``, ``exa`` and
            ``github``, which forward only the arguments the caller actually
            supplied.
            """
            return {k: v for k, v in kwargs.items() if v is not None}
```
/answer c564-1d allow
Control Request:
```yaml
id: c564-1e
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/bridge.py
  reason: "Gesamtstruktur von bridge.py (McpBridge, McpBridgeError, compact) sehen, da diese bereits von exa.py importiert und verwendet werden"
```
/answer c564-1e allow
Control Request:
```yaml
id: c564-1f
result:
  structuredContent:
    nodes:
    - id: wG9OyY
      type: statements
    - id: TfLVtt
      type: imports
    - id: compact
      type: FunctionDef
      signature: "def compact(**kwargs: Any) -> dict[str, Any]:"
      docstring: "Build a remote-call argument dict, dropping keys whose value is ``None``. Share…"
    - id: McpBridgeError
      type: ClassDef
      signature: "class McpBridgeError(RuntimeError):"
      docstring: "Raised when a forwarded call fails, at transport level or because the remote to…"
    - id: McpBridge
      type: ClassDef
      signature: "class McpBridge:"
      docstring: Lazily connects to one external MCP server and forwards ``tools/call``.
      children:
      - id: McpBridge.whl1ag
        type: statements
      - id: McpBridge.init
        type: FunctionDef
        signature: "def __init__(self, config: ServerConfig | None=None) -> None:"
      - id: McpBridge.build_client
        type: FunctionDef
        signature: "def build_client(self, config: ServerConfig) -> McpClient:"
        docstring: "Create the client for the target server (called once, lazily)."
      - id: McpBridge.get_client
        type: FunctionDef
        signature: "def get_client(self) -> McpClient:"
      - id: McpBridge.call
        type: FunctionDef
        signature: "def call(self, remote_tool: str, arguments: dict[str, Any]) -> dict[str, Any]:"
        docstring: "Forward a call and return the remote's structured result. Raises: McpBridgeErro…"
    - id: extract_result
      type: FunctionDef
      signature: "def _extract_result(remote_tool: str, result: dict[str, Any]) -> dict[str, Any]:"
      docstring: "Resolve a remote ``CallToolResult`` into structured data, or raise."
```
/answer c564-1f allow
ReasoningToken: 5589
Control Request:
```yaml
id: c564-20
toolName: grep
arguments:
  directory:
  - /home/user/xyan/xy.ai.workbench/project
  pattern: web_search_exa|web_fetch_exa|highlights|autoprompt|numResults|maxCharacters|published_date|highlight_scores
  include:
  - exa.md
  exclude: []
  limit: 50
  reason: "Exa API-Dokumentation nach den relevanten Tool-Namen und Feldnamen durchsuchen, um die Response-Struktur korrekt abzubilden"
```
/answer c564-20 allow
Control Request:
```yaml
id: c564-21
result:
  structuredContent:
    matches:
    - path: exa.md
      lineno: 66
      match: "                    highlights:"
    - path: exa.md
      lineno: 70
      match: '                      highlights key developments...'
    - path: exa.md
      lineno: 185
      match: "        numResults:"
    - path: exa.md
      lineno: 200
      match: "                Deprecated: Use highlights or text instead. Returns page"
    - path: exa.md
      lineno: 206
      match: "                    Deprecated: Use highlights or text instead. Returns page"
    - path: exa.md
      lineno: 212
      match: "                    maxCharacters:"
    - path: exa.md
      lineno: 221
      match: "                    Deprecated: Use highlights or text instead. Returns page"
    - path: exa.md
      lineno: 238
      match: "                Content options for text, highlights, summary, extras, and"
    - path: exa.md
      lineno: 419
      match: '                highlights or text instead.'
    - path: exa.md
      lineno: 461
      match: '                highlights or text instead.'
    - path: exa.md
      lineno: 951
      match: "                    maxCharacters:"
    - path: exa.md
      lineno: 1043
      match: "        highlights:"
    - path: exa.md
      lineno: 1050
      match: "                  title: Simple highlights retrieval"
    - path: exa.md
      lineno: 1052
      match: "                    If true, returns highlights with default settings. If false,"
    - path: exa.md
      lineno: 1053
      match: '                    disables highlights.'
    - path: exa.md
      lineno: 1061
      match: '                            Custom query that guides which highlights the LLM'
    - path: exa.md
      lineno: 1075
      match: "                            returned highlights. With highlights.dynamic, the"
    - path: exa.md
      lineno: 1079
      match: "                            compatible with maxCharacters or numSentences. Beta:"
    - path: exa.md
      lineno: 1081
      match: '                            dynamic-highlights-2026-08-28` request header;'
    - path: exa.md
      lineno: 1086
      match: "                          x-exa-beta-flag: dynamic-highlights-2026-08-28"
    - path: exa.md
      lineno: 1099
      match: "                            with maxCharacters. Beta: requires the `Exa-Beta:"
    - path: exa.md
      lineno: 1100
      match: '                            dynamic-highlights-2026-08-28` request header;'
    - path: exa.md
      lineno: 1104
      match: "                          x-exa-beta-flag: dynamic-highlights-2026-08-28"
    - path: exa.md
      lineno: 1109
      match: "                    maxCharacters:"
    - path: exa.md
      lineno: 1116
      match: '                            highlights. Controls the total length of highlight'
    - path: exa.md
      lineno: 1118
      match: '                            10000. Not compatible with highlights.dynamic.'
    - path: exa.md
      lineno: 1128
      match: "                            characters per sentence. Pass highlights: true for"
    - path: exa.md
      lineno: 1129
      match: "                            default highlights, or { query } to guide selection"
    - path: exa.md
      lineno: 1134
      match: "                    highlightsPerUrl:"
    - path: exa.md
      lineno: 1140
      match: "                            Currently ignored. Pass highlights: true for default"
    - path: exa.md
      lineno: 1141
      match: "                            highlights, or { query } to guide selection with"
    - path: exa.md
      lineno: 1146
      match: "                  title: Advanced highlights options"
    - path: exa.md
      lineno: 1149
      match: "                    highlights: true for the highest-quality default; supply"
    - path: exa.md
      lineno: 1245
      match: "                Deprecated: Use highlights or text instead. Returns page"
    - path: exa.md
      lineno: 1251
      match: "                    Deprecated: Use highlights or text instead. Returns page"
    - path: exa.md
      lineno: 1257
      match: "                    maxCharacters:"
    - path: exa.md
      lineno: 1266
      match: "                    Deprecated: Use highlights or text instead. Returns page"
    - path: exa.md
      lineno: 1400
      match: "        highlights:"
    - path: exa.md
      lineno: 1401
      match: "          description: Array of highlights extracted from the search result content."
    - path: exa.md
      lineno: 1418
      match: '            This overview paper on Large Language Models (LLMs) highlights key'
    - path: exa.md
      lineno: 2322
      match: "            highlights, and summaries billed outside the bundled search price)."
    - path: exa.md
      lineno: 2330
      match: "            highlights:"
    - path: exa.md
      lineno: 2568
      match: "                    highlights:"
    - path: exa.md
      lineno: 2572
      match: '                      highlights key developments...'
    - path: exa.md
      lineno: 2668
      match: '            highlights or text instead.'
    - path: exa.md
      lineno: 2887
      match: "                    maxCharacters:"
    - path: exa.md
      lineno: 2979
      match: "        highlights:"
    - path: exa.md
      lineno: 2986
      match: "                  title: Simple highlights retrieval"
    - path: exa.md
      lineno: 2988
      match: "                    If true, returns highlights with default settings. If false,"
    - path: exa.md
      lineno: 2989
      match: '                    disables highlights.'
```
/answer c564-21 allow
ReasoningToken: 8485
Control Request:
```yaml
id: c564-22
toolName: grep
arguments:
  directory:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
  pattern: import.*exa|from.*exa
  include:
  - __init__.py
  exclude: []
  limit: 20
  reason: "Import-Zeile von register_exa_tools in tools/__init__.py finden, um sicherzustellen, dass der neue Paketpfad kompatibel bleibt"
```
/answer c564-22 allow
Control Request:
```yaml
id: c564-23
result:
  structuredContent:
    matches:
    - path: mcp/__init__.py
      lineno: 5
      match: "from xy.ai.mcpc.tools.mcp.exa import ExaBridge, register_exa_tools"
    - path: __init__.py
      lineno: 11
      match: "from xy.ai.mcpc.tools.mcp import register_context7_tools, register_exa_tools, register_github_tools"
```
/answer c564-23 allow
ReasoningToken: 757
Control Request:
```yaml
id: c564-24
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/__init__.py
  ids:
  - all
  reason: "Vollständigen Inhalt von mcp/__init__.py lesen, um zu prüfen, welche Namen aus exa.py re-exportiert werden"
```
/answer c564-24 allow
Control Request:
```yaml
id: c564-25
result:
  content:
  - type: text
    text: No node matched 'all'.
  isError: true
```
/answer c564-25 allow
ReasoningToken: 150
Control Request:
```yaml
id: c564-26
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/__init__.py
  reason: "Struktur von mcp/__init__.py sehen, um die re-exportierten Namen aus exa.py zu identifizieren"
```
/answer c564-26 allow
Control Request:
```yaml
id: c564-27
result:
  structuredContent:
    nodes:
    - id: FCqAFY
      type: statements
    - id: BYpfwH
      type: imports
    - id: daS8aM
      type: statements
```
/answer c564-27 allow
Control Request:
```yaml
id: c564-28
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/__init__.py
  ids:
  - FCqAFY
  - BYpfwH
  - daS8aM
  reason: "Vollständigen Inhalt von mcp/__init__.py lesen, inkl. Imports und __all__, um Kompatibilität nach der Umstrukturierung von exa.py sicherzustellen"
```
/answer c564-28 allow
Control Request:
```yaml
id: c564-29
result:
  structuredContent:
    nodes:
    - id: FCqAFY
      type: statements
      code: '''Bridges that expose external MCP servers as MCPC tools.'''
    - id: BYpfwH
      type: imports
      code: |-
        from xy.ai.mcpc.tools.mcp.bridge import McpBridge, McpBridgeError, compact
        from xy.ai.mcpc.tools.mcp.client import DEFAULT_PROTOCOL_VERSION, McpClient, McpClientError
        from xy.ai.mcpc.tools.mcp.context7 import Context7Bridge, register_context7_tools
        from xy.ai.mcpc.tools.mcp.exa import ExaBridge, register_exa_tools
        from xy.ai.mcpc.tools.mcp.github import GitHubBridge, register_github_tools
    - id: daS8aM
      type: statements
      code: |-
        __all__ = [
            'Context7Bridge',
            'DEFAULT_PROTOCOL_VERSION',
            'ExaBridge',
            'GitHubBridge',
            'McpBridge',
            'McpBridgeError',
            'McpClient',
            'McpClientError',
            'compact',
            'register_context7_tools',
            'register_exa_tools',
            'register_github_tools']
```
/answer c564-29 allow
Control Request:
```yaml
id: c564-2a
toolName: ast_insert
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
  id: compute_file_stats
  position: before
  source: |+
    @dataclass(frozen=True)
    class TextStatsResult:
        size_bytes: int
        lines: int
        words: int
        complexity: float
        line_length_max: int
        line_length_min: int
        line_length_avg: float
        words_per_line_avg: float
        checksum: str


    def compute_text_stats(text: str) -> TextStatsResult:
        """Compute size/line/word/complexity metrics for *text*.

        Extracted from :func:`compute_file_stats` so string content (e.g. fetched
        web pages) can be scored the same way without touching the filesystem.
        """
        lines = text.splitlines()
        size_bytes = len(text.encode('utf-8'))
        num_lines = len(lines)
        num_words = len(text.split())
        complexity = _calculate_complexity(text)
        checksum = hashlib.sha256(text.encode('utf-8')).hexdigest()
        line_lengths = [len(line) for line in lines] if lines else [0]
        line_length_max = max(line_lengths)
        line_length_min = min(line_lengths)
        line_length_avg = round(sum(line_lengths) / len(line_lengths), 2)
        words_per_line = [len(line.split()) for line in lines]
        words_per_line_avg = round(sum(words_per_line) / len(words_per_line), 2) if words_per_line else 0.0
        return TextStatsResult(
            size_bytes=size_bytes,
            lines=num_lines,
            words=num_words,
            complexity=complexity,
            line_length_max=line_length_max,
            line_length_min=line_length_min,
            line_length_avg=line_length_avg,
            words_per_line_avg=words_per_line_avg,
            checksum=checksum)

  reason: "Innere Textmetrik-Berechnung aus compute_file_stats extrahieren und als eigenständige, string-basierte Funktion publizieren, damit sie vom Exa fetch-Tool wiederverwendet werden kann"
```
/answer c564-2a allow
Control Request:
```yaml
id: c564-2b
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
  id: 5xkcFZ
  old_text: |-
    __all__ = [
        'FileStatsError',
        'FileStatsResult',
        'compute_file_stats',
  new_text: |-
    __all__ = [
        'FileStatsError',
        'FileStatsResult',
        'TextStatsResult',
        'compute_text_stats',
        'compute_file_stats',
  reason: TextStatsResult und compute_text_stats als öffentliche API des file_stats Moduls publizieren
```
/answer c564-2b allow
Control Request:
```yaml
id: c564-2c
toolName: ast_replace
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
  id: compute_file_stats
  source: |-
    def compute_file_stats(path: Path) -> FileStatsResult:
        """Compute the file-metrics block for *path* (also reused by the outline tool).

        Assumes *path* is an existing regular file.
        """
        raw_bytes = path.read_bytes()
        text = raw_bytes.decode('utf-8', errors='replace')
        text_stats = compute_text_stats(text)
        stat = path.stat()
        created = datetime.fromtimestamp(stat.st_birthtime if hasattr(stat, 'st_birthtime')
                                         else stat.st_mtime, tz=timezone.utc).isoformat()
        modified = datetime.fromtimestamp(stat.st_mtime, tz=timezone.utc).isoformat()
        accessed = datetime.fromtimestamp(stat.st_atime, tz=timezone.utc).isoformat()
        return FileStatsResult(
            path=str(path.resolve()),
            created=created,
            modified=modified,
            accessed=accessed,
            # size_bytes uses the raw byte count, not the (possibly re-encoded) text.
            **{**asdict(text_stats), 'size_bytes': size_bytes if (size_bytes := len(raw_bytes)) else 0})
  reason: "compute_file_stats auf die extrahierte compute_text_stats-Funktion umstellen, restliche Logik (Pfad/Zeitstempel) bleibt filesystemspezifisch"
```
/answer c564-2c allow
ReasoningToken: 1097
Text:
 Jetzt `compute_file_stats` auf die extrahierte Funktion umstellen:
Control Request:
```yaml
id: c564-2d
toolName: ast_replace
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
  id: compute_file_stats
  source: |-
    def compute_file_stats(path: Path) -> FileStatsResult:
        """Compute the file-metrics block for *path* (also reused by the outline tool).

        Assumes *path* is an existing regular file.
        """
        raw_bytes = path.read_bytes()
        text = raw_bytes.decode('utf-8', errors='replace')
        text_stats = asdict(compute_text_stats(text))
        text_stats['size_bytes'] = len(raw_bytes)
        stat = path.stat()
        created = datetime.fromtimestamp(stat.st_birthtime if hasattr(stat, 'st_birthtime')
                                         else stat.st_mtime, tz=timezone.utc).isoformat()
        modified = datetime.fromtimestamp(stat.st_mtime, tz=timezone.utc).isoformat()
        accessed = datetime.fromtimestamp(stat.st_atime, tz=timezone.utc).isoformat()
        return FileStatsResult(
            path=str(path.resolve()),
            created=created,
            modified=modified,
            accessed=accessed,
            **text_stats)
  reason: "compute_file_stats vereinfachen: nutzt compute_text_stats und überschreibt nur size_bytes mit der rohen Byteanzahl"
```
/answer c564-2d allow
Control Request:
```yaml
id: c564-2e
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
  reason: Syntaktische Korrektheit der überarbeiteten file_stats-Implementierung prüfen
```
/answer c564-2e allow
Control Request:
```yaml
id: c564-2f
toolName: ast_delete
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa.py
  reason: "Alte einteilige exa.py entfernen, wird durch das neue exa/-Paket mit vier getrennten Tool-Dateien ersetzt"
```
/answer c564-2f allow
Control Request:
```yaml
id: c564-30
toolName: ast_create
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/bridge.py
  reason: "Gemeinsame Bridge zum Exa Remote-MCP-Server als eigenes Modul anlegen, wiederverwendet von allen vier Exa-Tools"
  source: |
    """Shared bridge to the Exa remote MCP server, used by all ``exa`` tools."""

    from xy.ai.mcpc.config import ServerConfig
    from xy.ai.mcpc.tools.mcp.bridge import McpBridge, McpBridgeError
    from xy.ai.mcpc.tools.mcp.client import McpClient, McpClientError

    __all__ = ['ExaBridge', 'init_bridge', 'get_bridge']


    class ExaBridge(McpBridge):
        """Bridge to the Exa remote MCP server."""

        def build_client(self, config: ServerConfig) -> McpClient:
            api_key = config.exa_api_key
            if not api_key:
                raise McpClientError('Exa API key is not configured (set MCPC_EXA_API_KEY / EXA_API_KEY).')
            return McpClient(config.exa_mcp_url, headers={'x-api-key': api_key})


    #: Module-level bridge, built by :func:`~xy.ai.mcpc.tools.mcp.exa.register_exa_tools`.
    _bridge: ExaBridge | None = None


    def init_bridge(config: ServerConfig) -> None:
        global _bridge
        _bridge = ExaBridge(config)


    def get_bridge() -> ExaBridge:
        """Return the module-level Exa bridge configured by ``register_exa_tools``."""
        if _bridge is None:
            raise McpBridgeError('Exa tools used before register_exa_tools() was called.')
        return _bridge
```
/answer c564-30 allow
ReasoningToken: 25
Text:
 Validieren und Gesamtdatei prüfen:
Control Request:
```yaml
id: c564-31
toolName: ast_create
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/core.py
  reason: Gemeinsame Normalisierungs- und Cache-Bausteine für die zweistufigen Exa-Tools (Suche und Fetch) anlegen
  source: |
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
    #: Fields the Exa payload carries but that add no value for our tools.
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


    #: Cache instances shared between each stage-1 tool and its stage-2 counterpart.
    search_cache = ResultCache()
    fetch_cache = ResultCache()
```
/answer c564-31 allow
ReasoningToken: 241
ReasoningToken: 274
Control Request:
```yaml
id: c564-32
toolName: ast_create
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/search.py
  reason: "web_search_exa als eigenständiges, zweistufiges Tool anlegen (Stufe 1: Suche + Cache, Übersicht ohne Text/URL)"
  source: |
    """``web_search_exa`` - stage 1 of the two-stage Exa search retrieval.

    Runs a search and caches each full result (incl. text and url) by id;
    returns only an overview list without text/url. Call ``web_search_exa_results``
    with the returned ids to resolve url and full text.
    """

    from dataclasses import dataclass
    from typing import Any

    from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import ToolContext
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry
    from xy.ai.mcpc.tools.mcp.bridge import McpBridgeError, compact
    from xy.ai.mcpc.tools.mcp.exa.bridge import get_bridge
    from xy.ai.mcpc.tools.mcp.exa.core import normalize_item, search_cache, strip_empty

    __all__ = ['WebSearchResult', 'web_search_exa', 'WebSearchExaTool', 'register']

    _DESCRIPTION = ('Search the web for any topic and get clean, ready-to-use content.\n\n'
                     'Best for: Finding current information, facts, or answering questions about any topic.\n'
                     'Returns: an overview per result (id, title, author, excerpt) without text or url; '
                     'call web_search_exa_results with the ids to get url and full text.')

    _INPUT_SCHEMA: dict[str, Any] = {
        'type': 'object',
        'properties': {
            'query': {
                'type': 'string',
                'description': 'Natural language search query. Should be a semantically rich description of the ideal page.'},
            'numResults': {
                'type': 'integer',
                'description': 'Number of search results to return (default: 10).',
                'minimum': 1}},
        'required': ['query']}

    _ITEM_SCHEMA: dict[str, Any] = {
        'type': 'object',
        'properties': {
            'id': {'type': 'string', 'description': 'Result id; pass to web_search_exa_results for url and text.'},
            'title': {'type': 'string'},
            'author': {'type': 'string'},
            'excerpt': {'type': 'array', 'items': {'type': 'string'}, 'description': 'Short excerpt(s) of the page text.'}},
        'required': ['id']}

    _OUTPUT_SCHEMA: dict[str, Any] = {
        'type': 'object',
        'properties': {
            'results': {'type': 'array', 'items': _ITEM_SCHEMA},
            'autoprompt_string': {'type': 'string'}},
        'required': ['results']}

    _ANNOTATIONS = {'readOnlyHint': True, 'openWorldHint': True}


    @dataclass(frozen=True, slots=True)
    class WebSearchResult:
        """Overview of a ``web_search_exa`` call; url/text via ``web_search_exa_results``."""
        results: list[dict[str, Any]]
        autoprompt_string: str | None = None


    def _web_search_exa_raw(query: str, numResults: int | None = None) -> dict[str, Any]:
        return get_bridge().call('web_search_exa', compact(query=query, numResults=numResults))


    def web_search_exa(query: str, numResults: int | None = None) -> WebSearchResult:
        """Search the web for any topic and get clean, ready-to-use content.

        Best for: Finding current information, facts, or answering questions
        about any topic.

        Args:
            query: Natural language search query; should be a semantically
                rich description of the ideal page.
            numResults: Number of search results to return (default: 10).

        Returns:
            Overview per result (no text/url); resolve ids via
            ``web_search_exa_results`` for the full text.

        Raises:
            McpBridgeError: if the Exa call fails.
        """
        raw = _web_search_exa_raw(query, numResults)
        items = [normalize_item(item) for item in raw.get('results', [])]
        for item in items:
            search_cache.put(item)
        overview = [strip_empty({k: v for k, v in item.items() if k not in ('text', 'url')}) for item in items]
        return WebSearchResult(results=overview, autoprompt_string=raw.get('autoprompt_string'))


    class WebSearchExaTool(ToolDefinition):
        name = 'web_search_exa'
        title = 'Exa web search'
        description = _DESCRIPTION
        input_schema = _INPUT_SCHEMA
        output_schema = _OUTPUT_SCHEMA
        annotations = _ANNOTATIONS

        def handle(self, ctx: ToolContext) -> ToolResult:
            args = ctx.arguments
            try:
                result = web_search_exa(query=args['query'], numResults=args.get('numResults'))
            except McpBridgeError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            structured = strip_empty({'results': result.results, 'autoprompt_string': result.autoprompt_string})
            return ToolResult(structured_content=structured)


    def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
        registry.register(WebSearchExaTool())
        functions.register(web_search_exa)
```
/answer c564-32 allow
Control Request:
```yaml
id: c564-33
toolName: ast_create
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/search_results.py
  reason: "web_search_exa_results als Stufe-2-Tool anlegen, das IDs aus web_search_exa zu URL und Volltext auflöst"
  source: |
    """``web_search_exa_results`` - stage 2: resolve ``web_search_exa`` ids to url and text."""

    from typing import Any

    from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
    from xy.ai.mcpc.tools.tool_context import ToolContext
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry
    from xy.ai.mcpc.tools.mcp.exa.core import search_cache, strip_empty

    __all__ = ['web_search_exa_results', 'WebSearchExaResultsTool', 'register']

    _DESCRIPTION = ('Resolve ids returned by web_search_exa to their url and full text.\n\n'
                     'Best for: reading the full content of specific web_search_exa results.')

    _INPUT_SCHEMA: dict[str, Any] = {
        'type': 'object',
        'properties': {
            'ids': {'type': 'array', 'items': {'type': 'string'}, 'description': 'Result ids returned by web_search_exa.'}},
        'required': ['ids']}

    _OUTPUT_SCHEMA: dict[str, Any] = {
        'type': 'object',
        'properties': {
            'results': {
                'type': 'array',
                'items': {
                    'type': 'object',
                    'properties': {
                        'id': {'type': 'string'},
                        'url': {'type': 'string'},
                        'text': {'type': 'string'}},
                    'required': ['id']}}},
        'required': ['results']}

    _ANNOTATIONS = {'readOnlyHint': True, 'openWorldHint': False}


    def web_search_exa_results(ids: list[str]) -> list[dict[str, Any]]:
        """Resolve ids from a prior ``web_search_exa`` call to url and full text.

        Args:
            ids: Result ids returned by ``web_search_exa``.

        Returns:
            One entry per known id, with ``id``, ``url`` and ``text``.
        """
        items = search_cache.get_many(ids)
        return [strip_empty({'id': item['id'], 'url': item.get('url'), 'text': item.get('text')}) for item in items]


    class WebSearchExaResultsTool(ToolDefinition):
        name = 'web_search_exa_results'
        title = 'Exa web search results'
        description = _DESCRIPTION
        input_schema = _INPUT_SCHEMA
        output_schema = _OUTPUT_SCHEMA
        annotations = _ANNOTATIONS

        def handle(self, ctx: ToolContext) -> ToolResult:
            results = web_search_exa_results(ids=ctx.arguments['ids'])
            return ToolResult(structured_content={'results': results}, auto_approve=True)


    def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
        registry.register(WebSearchExaResultsTool())
        functions.register(web_search_exa_results)
```
/answer c564-33 allow
Control Request:
```yaml
id: c564-34
toolName: ast_create
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch.py
  reason: "web_fetch_exa als eigenständiges, zweistufiges Tool anlegen (Stufe 1: Fetch + Cache, Übersicht mit file_stats-Metriken statt Volltext)"
  source: |
    """``web_fetch_exa`` - stage 1 of the two-stage Exa fetch retrieval.

    Fetches page content and caches each full result (incl. text and url) by id;
    returns only an overview with file_stats-style text metrics, no text/url.
    Call ``web_fetch_exa_results`` with the returned ids to resolve url and text.
    """

    from dataclasses import asdict, dataclass
    from typing import Any

    from xy.ai.mcpc.tools.file_stats import compute_text_stats
    from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import ToolContext
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry
    from xy.ai.mcpc.tools.mcp.bridge import McpBridgeError, compact
    from xy.ai.mcpc.tools.mcp.exa.bridge import get_bridge
    from xy.ai.mcpc.tools.mcp.exa.core import fetch_cache, normalize_item, strip_empty

    __all__ = ['WebFetchResult', 'web_fetch_exa', 'WebFetchExaTool', 'register']

    _DESCRIPTION = ("Read a webpage's full content as clean markdown. Use to read any URL.\n\n"
                     'Best for: Extracting full content from known URLs. Batch multiple URLs in one call.\n'
                     'Returns: an overview per url (id, title, text metrics) without text/url; '
                     'call web_fetch_exa_results with the ids to get url and full text.')

    _INPUT_SCHEMA: dict[str, Any] = {
        'type': 'object',
        'properties': {
            'urls': {'type': 'array', 'items': {'type': 'string'}, 'description': 'URLs to fetch. Batch multiple URLs in one call.'},
            'maxCharacters': {
                'type': 'integer',
                'description': 'Maximum characters to extract per page (default: 3000).',
                'minimum': 1}},
        'required': ['urls']}

    _METRICS_SCHEMA: dict[str, Any] = {
        'size_bytes': {'type': 'integer'},
        'lines': {'type': 'integer'},
        'words': {'type': 'integer'},
        'complexity': {'type': 'number'},
        'line_length_max': {'type': 'integer'},
        'line_length_min': {'type': 'integer'},
        'line_length_avg': {'type': 'number'},
        'words_per_line_avg': {'type': 'number'},
        'checksum': {'type': 'string'}}

    _ITEM_SCHEMA: dict[str, Any] = {
        'type': 'object',
        'properties': {
            'id': {'type': 'string', 'description': 'Result id; pass to web_fetch_exa_results for url and text.'},
            'title': {'type': 'string'},
            'author': {'type': 'string'},
            'summary': {'type': 'string'},
            'excerpt': {'type': 'array', 'items': {'type': 'string'}, 'description': 'Short excerpt(s) of the page text.'},
            **_METRICS_SCHEMA},
        'required': ['id']}

    _OUTPUT_SCHEMA: dict[str, Any] = {
        'type': 'object',
        'properties': {'results': {'type': 'array', 'items': _ITEM_SCHEMA}},
        'required': ['results']}

    _ANNOTATIONS = {'readOnlyHint': True, 'openWorldHint': True}


    @dataclass(frozen=True, slots=True)
    class WebFetchResult:
        """Overview of a ``web_fetch_exa`` call; url/text via ``web_fetch_exa_results``."""
        results: list[dict[str, Any]]


    def _web_fetch_exa_raw(urls: list[str], maxCharacters: int | None = None) -> dict[str, Any]:
        return get_bridge().call('web_fetch_exa', compact(urls=urls, maxCharacters=maxCharacters))


    def web_fetch_exa(urls: list[str], maxCharacters: int | None = None) -> WebFetchResult:
        """Read one or more webpages' full content as clean markdown.

        Best for: Extracting full content from known URLs. Batch multiple
        URLs in one call.

        Args:
            urls: URLs to fetch.
            maxCharacters: Maximum characters to extract per page (default: 3000).

        Returns:
            Overview per url (file_stats-style text metrics, no text/url);
            resolve ids via ``web_fetch_exa_results`` for the full text.

        Raises:
            McpBridgeError: if the Exa call fails.
        """
        raw = _web_fetch_exa_raw(urls, maxCharacters)
        items = [normalize_item(item) for item in raw.get('results', [])]
        for item in items:
            fetch_cache.put(item)
        overview = []
        for item in items:
            metrics = asdict(compute_text_stats(item.get('text') or ''))
            entry = {k: v for k, v in item.items() if k not in ('text', 'url')}
            overview.append(strip_empty({**entry, **metrics}))
        return WebFetchResult(results=overview)


    class WebFetchExaTool(ToolDefinition):
        name = 'web_fetch_exa'
        title = 'Exa web fetch'
        description = _DESCRIPTION
        input_schema = _INPUT_SCHEMA
        output_schema = _OUTPUT_SCHEMA
        annotations = _ANNOTATIONS

        def handle(self, ctx: ToolContext) -> ToolResult:
            args = ctx.arguments
            try:
                result = web_fetch_exa(urls=args['urls'], maxCharacters=args.get('maxCharacters'))
            except McpBridgeError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content={'results': result.results})


    def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
        registry.register(WebFetchExaTool())
        functions.register(web_fetch_exa)
```
/answer c564-34 allow
Control Request:
```yaml
id: c564-35
toolName: ast_create
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch_results.py
  reason: "web_fetch_exa_results als Stufe-2-Tool anlegen, das IDs aus web_fetch_exa zu URL und Volltext auflöst, inkl. grep-artiger Zeilenfilterung mit Kontext"
  source: |
    """``web_fetch_exa_results`` - stage 2: resolve ``web_fetch_exa`` ids to url and text.

    Optionally filters the text line-wise with an extended regular expression
    (``grep -E`` semantics), including a configurable amount of context lines.
    """

    import re
    from typing import Any

    from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import ToolContext
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry
    from xy.ai.mcpc.tools.mcp.exa.core import fetch_cache, strip_empty

    __all__ = ['web_fetch_exa_results', 'WebFetchExaResultsTool', 'register']

    _DESCRIPTION = ('Resolve ids returned by web_fetch_exa to their url and full text.\n\n'
                     'Best for: reading the full content of specific web_fetch_exa results; '
                     'optionally pre-filter long pages line-wise with a regular expression.')

    _INPUT_SCHEMA: dict[str, Any] = {
        'type': 'object',
        'properties': {
            'ids': {'type': 'array', 'items': {'type': 'string'}, 'description': 'Result ids returned by web_fetch_exa.'},
            'pattern': {'type': 'string', 'description': 'Extended regular expression (grep -E semantics) to filter text lines.'},
            'context': {
                'type': 'integer',
                'description': 'Context lines kept before/after each match (default: 1); only used with pattern.',
                'minimum': 0}},
        'required': ['ids']}

    _OUTPUT_SCHEMA: dict[str, Any] = {
        'type': 'object',
        'properties': {
            'results': {
                'type': 'array',
                'items': {
                    'type': 'object',
                    'properties': {
                        'id': {'type': 'string'},
                        'url': {'type': 'string'},
                        'text': {'type': 'string'}},
                    'required': ['id']}}},
        'required': ['results']}

    _ANNOTATIONS = {'readOnlyHint': True, 'openWorldHint': False}


    def _grep_lines(text: str, pattern: str, context: int) -> str:
        """Keep lines matching *pattern* plus *context* lines around each match ('grep -E' style)."""
        lines = text.splitlines()
        regex = re.compile(pattern)
        matched = [i for i, line in enumerate(lines) if regex.search(line)]
        if not matched:
            return ''
        keep = sorted({j for i in matched for j in range(max(0, i - context), min(len(lines), i + context + 1))})
        grouped: list[list[int]] = []
        for i in keep:
            if grouped and i == grouped[-1][-1] + 1:
                grouped[-1].append(i)
            else:
                grouped.append([i])
        return '\n--\n'.join('\n'.join(lines[i] for i in group) for group in grouped)


    def web_fetch_exa_results(ids: list[str], pattern: str | None = None, context: int = 1) -> list[dict[str, Any]]:
        """Resolve ids from a prior ``web_fetch_exa`` call to url and full text.

        Args:
            ids: Result ids returned by ``web_fetch_exa``.
            pattern: Optional extended regular expression to filter text lines
                (``grep -E`` semantics); non-adjacent matching blocks are
                separated by a ``--`` line, like ``grep``.
            context: Context lines kept before/after each match (default: 1).

        Returns:
            One entry per known id, with ``id``, ``url`` and ``text``.
        """
        items = fetch_cache.get_many(ids)
        results = []
        for item in items:
            text = item.get('text') or ''
            if pattern:
                text = _grep_lines(text, pattern, context)
            results.append(strip_empty({'id': item['id'], 'url': item.get('url'), 'text': text}))
        return results


    class WebFetchExaResultsTool(ToolDefinition):
        name = 'web_fetch_exa_results'
        title = 'Exa web fetch results'
        description = _DESCRIPTION
        input_schema = _INPUT_SCHEMA
        output_schema = _OUTPUT_SCHEMA
        annotations = _ANNOTATIONS

        def handle(self, ctx: ToolContext) -> ToolResult:
            args = ctx.arguments
            try:
                results = web_fetch_exa_results(ids=args['ids'], pattern=args.get('pattern'), context=args.get('context', 1))
            except re.error as exc:
                return ToolResult(content=[text_content(f'Invalid pattern: {exc}')], is_error=True)
            return ToolResult(structured_content={'results': results}, auto_approve=True)


    def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
        registry.register(WebFetchExaResultsTool())
        functions.register(web_fetch_exa_results)
```
/answer c564-35 allow
Control Request:
```yaml
id: c564-36
toolName: ast_create
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/__init__.py
  reason: "Paket-Einstiegspunkt für die vier Exa-Tools anlegen: registriert alle Tools und den 'exa' Alias, analog zum ast-Paket"
  source: |
    """``exa`` tool family - Exa web search/fetch bridged as two-stage MCPC tools.

    Each stage-1 tool (``web_search_exa`` / ``web_fetch_exa``) normalizes and
    caches Exa's raw results by id, returning only a compact overview; the
    matching stage-2 tool (``web_search_exa_results`` / ``web_fetch_exa_results``)
    resolves those ids back to url and full text.
    """

    from xy.ai.mcpc.tools.tool_context import AppEnvironment
    from xy.ai.mcpc.tools.tool_registry import ToolRegistry
    from xy.ai.mcpc.tools.mcp.exa import fetch, fetch_results, search, search_results
    from xy.ai.mcpc.tools.mcp.exa.bridge import ExaBridge, init_bridge

    __all__ = ['ExaBridge', 'register_exa_tools', 'ALIAS']

    #: Alias name that activates the whole family in one go.
    ALIAS = 'exa'
    _ALIAS_MEMBERS = ('web_search_exa', 'web_search_exa_results', 'web_fetch_exa', 'web_fetch_exa_results')


    def register_exa_tools(registry: ToolRegistry, environment: AppEnvironment) -> None:
        """Register every Exa tool and the ``exa`` alias."""
        init_bridge(environment.config)
        search.register(registry, environment.functions)
        search_results.register(registry, environment.functions)
        fetch.register(registry, environment.functions)
        fetch_results.register(registry, environment.functions)
        registry.register_alias(ALIAS, _ALIAS_MEMBERS)
```
/answer c564-36 allow
Control Request:
```yaml
id: c564-37
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/bridge.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/core.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/search.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/search_results.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/exa/fetch_results.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
  reason: Alle neuen und geänderten Dateien auf syntaktische Korrektheit prüfen
```
/answer c564-37 allow
ReasoningToken: 8473
Text:
 Jetzt das `__init__.py` des `exa`-Pakets erstellen:
Text:
 Nun validieren:
Result Stats:  id=c6aa0e76-64c9-413d-a9b7-8dc32f91b16f, total: 0, in: 179, out: 46524, reason: 0, read: 845573, write: 69794
Alle Dateien kompilieren fehlerfrei. Die Umstrukturierung ist abgeschlossen:

- `tools/mcp/exa/` (statt der einzelnen `exa.py`) mit `bridge.py`, `core.py` (Normalisierung, Cache, `strip_empty`) sowie je einer Datei pro Tool: `search.py`, `search_results.py`, `fetch.py`, `fetch_results.py`, registriert über `__init__.py` unter dem Alias `exa`.
- `web_fetch_exa` nimmt jetzt eine reine URL-Liste ohne Coerce-Logik entgegen.
- Beide Tools sind zweistufig: Stufe 1 cached Volltext/URL und liefert nur eine Übersicht (Suche: ohne Text/URL; Fetch: mit `file_stats`-Metriken statt Text/URL), Stufe 2 (`*_results`) löst IDs zu URL/Text auf.
- `web_fetch_exa_results` unterstützt zusätzlich `grep -E`-artige Zeilenfilterung mit Kontextparameter (Default 1).
- Feldbereinigung (`published_date`, `score`, `image`, `favicon`, `highlight_scores` entfernt; `id`-Fallback als 6-stelliger Zufallshash; `highlights`→`excerpt` mit Auto-Fallback; leere/Null-Werte werden rekursiv aus dem MCP-Ergebnis entfernt) ist zentral in `core.py` implementiert.
- `file_stats/__init__.py` wurde um `compute_text_stats`/`TextStatsResult` (string-basiert) erweitert, `compute_file_stats` nutzt sie nun intern; das neue Fetch-Tool importiert diese Funktion direkt.
