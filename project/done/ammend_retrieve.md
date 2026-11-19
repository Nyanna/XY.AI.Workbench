`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py` ist ein Multicast für Tools.

Die Beschreibung zur Verwendung ist unzureichend und muss durch die Tool und Verwendungsbeschreibungen der wirklichen Tools ergänzt werden.

- Kombiniere hierfür einen Rahmen mit Wiederverwendung der einzelnen Toolbeschreibungen (In Public Konnstanten Umwandeln).
- Dies soll leicht Pflegbar sein, also kein einzelner langer konkatinierter String 
SystemInit:  id=5e34e7ff-1963-49ca-9d47-c64a57ba632a, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Control Request:
```yaml
id: 4b3c-5f-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
  reason: "Retrieve.py lesen, um die Struktur des Multicast-Tools zu verstehen"
```
/answer 4b3c-5f-0 allow
Control Request:
```yaml
id: 4b3c-60-5f
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
        nodes:
        - id: q7vzZq|rO6USt
          excert: "'``ast_retrieve`` tool: all reading AST tools (plus search) in a single call.\\n…"
        - id: olOlxE|dMnIlW
          excert: from __future__ import annotations
        - id: q1JBaW|syp0aa
          excert: "from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolFeature, ToolReg…"
        - id: fiQhAE|pEjXcx
          excert: "__all__ = ['RetrieveTool', 'register']"
        - id: SectionError
          excert: "@dataclass(frozen=True, kw_only=True) class _SectionError(ItemError):"
          children:
          - id: SectionError.key
            excert: "def key(self) -> dict[str, Any]:"
        - id: error_text
          excert: "def _error_text(result: ToolResult) -> str:"
        - id: merge
          excert: "def _merge(parts: list[dict[str, Any]]) -> dict[str, Any]:"
          docstring: "Merge several payloads of the same tool: list values are concatenated."
        - id: RetrieveTool
          excert: "class RetrieveTool(ToolDefinition):"
          children:
          - id: RetrieveTool.axwi72|4oz5YL
            excert: name = 'ast_retrieve'
          - id: RetrieveTool.PHbHCQ|o9m8Np
            excert: "input_schema = {"
          - id: RetrieveTool.handle
            excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
        - id: register
          excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 4b3c-60-5f allow
Control Request:
```yaml
id: 4b3c-61-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: description|DESCRIPTION
    include:
    - '*.py'
    limit: 50
  reason: Tool-Dateien im ast-Verzeichnis finden
```
/answer 4b3c-61-0 allow
Control Request:
```yaml
id: 4b3c-62-61
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
        matches:
        - path: find.py
          matches:
          - "nodes: Outline-style node descriptions (see :class:`core.OutlineNode`)"
          - "description = 'Filter the AST-node tree of a list of files by type, name, id, line range, parent type, text substring or regexp (grep). Returns matches per file with their full source. Use this when looking up content without IDs.'"
          - "'description': 'Absolute paths of the files to search (never directories).'},"
          - "'description': 'Case-insensitive substring that the node’s source must contain. Example: `import` finds nodes containing imports.'},"
          - "'description': 'Regular expression that the node’s source must match (`re.search` semantics).'}},"
        - path: script.py
          matches:
          - description = "Run a restricted Python script against a file's AST when the single-operation ast_* tools (list/find/read/replace/insert/delete) are in… 'result' to return data; edits made via 'tree' are saved automatically. See the 'code' parameter for the full tree API and node-attribute reference."
          - "'description': 'Absolute path to the file.'},"
          - "'description': \"Python source executed with 'tree' and 'ast' as only globals; assign 'result' to return a value; no import statements; builtins limite…ne header / short doc, only set for definition-like nodes when code is omitted) have no direct Located field; derive via node_code(loc) if needed.\"}},"
        - path: replace.py
          matches:
          - description = 'Replace the source or text of selected AST nodes in a batch; operations may target the same or different files. Returns the updated IDs.'
          - "'description': 'Replacement source.'},"
          - _ITEMS_DESCRIPTION = 'Node replacements to apply.'
          - "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)"
        - path: edit.py
          matches:
          - "description = 'Run create, insert, replace, edit_block, edit_marks and delete operations in one call. Each list has the item format of the tool ast_<list name>.'"
        - path: delete.py
          matches:
          - "description = 'Delete selected nodes from files, or whole files if no selector is given, for a batch of items.'"
          - _ITEMS_DESCRIPTION = 'Nodes/files to delete.'
          - "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)"
        - path: outline.py
          matches:
          - "nodes: Outline-style node descriptions (see :class:`core.OutlineNode`), in"
          - "description = 'Given a list of one or more file names, returns a hierarchical AST tree with IDs, classes, functions, sections, types, excert, docstring, children, imports, and segments but never the full content. Other AST tools require IDs as input.'"
          - "'description': 'List of absolute file paths (never directories).'}},"
        - path: common.py
          matches:
          - "'description': 'Unique node id'},"
          - "'description': 'Simple node name.'},"
          - "'description': \"Regular expression (case-insensitive) a node type name must match, e.g. 'FunctionDef' or 'pair'.\"},"
          - "'description': 'Line in the target node.'},"
          - "'description': 'End line of a range to get all nodes touching the lines.'},"
          - "'description': 'Node type name of the container.'}}"
          - "PATH_PROP = {'type': 'string', 'description': 'Absolute path to the file (never directories).'}"
          - "'description': \"Import/include/use statements this edit needs, one complete statement per entry in the file's own syntax (e.g. 'import os', 'import a.b.C;', 'use std::fmt;').\"},"
          - "'description': 'Import/include/use statements that are no longer needed. Removed if present, ignored otherwise.'}}"
        - path: edit_marks.py
          matches:
          - "description = \"Replace everything between and including the unique 'start_marker' and 'end_marker' markers, found within nodes addressed by a known id, with new 'content', for a batch of items.\""
          - "'description': \"Unique 10-30 char substring marking the beginning of the text to replace, within the node's source.\"},"
          - "'description': \"Unique 10-30 char substring marking the end of the text to replace, within the node's source.\"},"
          - "'description': 'Replacement source for the marked text.'},"
          - "'description': \"If true, 'start_marker'/'end_marker' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.\","
          - _ITEMS_DESCRIPTION = 'Marker-based edits to apply.'
          - "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)"
        - path: read.py
          matches:
          - "description = 'After `ast_outline` or `ast_find`, reads the code, text, content, and children of the given AST node IDs within the given list of absolute file paths (never directories). Use this when you already know the IDs.'"
          - "'description': 'List of AST node IDs to read.'}}"
          - _ITEMS_DESCRIPTION = 'Per-file node IDs to read.'
          - "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)"
        - path: validate.py
          matches:
          - description = 'Check that each of a list of files compiles; report success/error per file.'
          - "'description': 'Absolute paths of files to validate.'}},"
        - path: edit_block.py
          matches:
          - "description = \"Replace occurrences of short 'old_text' with 'new_text', within nodes addressed by prior known id, for a batch of items. Don't use for large edits, use ast_edit_marks instead.\""
          - "'description': 'Short text (10-100 chars) to replace within the node. Must occur exactly once, or replaceAll is set.'},"
          - "'description': 'Replacement text, may be empty to remove the text.'},"
          - "'description': \"If true, 'old_text' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.\","
          - "'description': \"If true, replace every occurrence of 'old_text' within the node instead of a single unique match.\","
          - _ITEMS_DESCRIPTION = 'Block edits to apply.'
          - "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)"
        - path: create.py
          matches:
          - description = 'Create one or more files from source.'
          - "'type': 'string', 'description': 'Absolute path of the file to create.'}, 'source': {"
        warnings:
        - "Limit of 50 matches reached (total 63); narrow pattern, directory or include/exclude filters."
        - 2 match line(s) exceeded 300 characters and were shortened in the middle.
```
/answer 4b3c-62-61 allow
Control Request:
```yaml
id: 4b3c-63-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
    ids:
    - q7vzZq|rO6USt
    - q1JBaW|syp0aa
    - RetrieveTool
  outline:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/search.py
  reason: Retrieve.py und edit.py vollständig lesen
```
/answer 4b3c-63-0 allow
Control Request:
```yaml
id: 4b3c-64-63
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit.py
        nodes:
        - id: 5ZtN89|XTSQEh
          excert: "'``ast_edit`` tool: all modifying AST tools in a single call.'"
        - id: neazUj|4NSldY
          excert: from __future__ import annotations
        - id: indDlk|k9bRxt
          excert: from xy.ai.mcpc.tools.tool_context import ToolContext
        - id: SectionError
          excert: "@dataclass(frozen=True, kw_only=True) class _SectionError(ItemError):"
          children:
          - id: SectionError.key
            excert: "def key(self) -> dict[str, Any]:"
        - id: 14h53u|AeVQqA
          excert: "__all__ = ['EditTool', 'register']"
        - id: error_text
          excert: "def _error_text(result: ToolResult) -> str:"
        - id: EditTool
          excert: "class EditTool(ToolDefinition):"
          children:
          - id: EditTool.oy2a69|JHgl3f
            excert: name = 'ast_edit'
          - id: EditTool.handle
            excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
        - id: register
          excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
      errors:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/search.py
        error: File not found.
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
        nodes:
        - id: q7vzZq|rO6USt
          type: statements
          code: "'``ast_retrieve`` tool: all reading AST tools (plus search) in a single call.\\n\\nCombines ``search``, ``ast_outline``, ``ast_find`` and ``ast_read``. Search is\\nspecial: with a location intent it yields AST node ids, which can be consumed\\nby ``read`` in the same call only if known upfront; otherwise use the ids from\\nthe response in a follow-up call.\\n'"
        - id: q1JBaW|syp0aa
          type: imports
          code: "from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolFeature, ToolRegistry, ToolResult"
        - id: RetrieveTool
          type: ClassDef
          excert: "class RetrieveTool(ToolDefinition):"
          children:
          - id: RetrieveTool.axwi72|4oz5YL
            type: statements
            code: |-
              name = 'ast_retrieve'
              features = frozenset({ToolFeature.INPUT_ASTID})
              title = 'Batch of retrieval operations (search, outline, find, read)'
              description = 'Run search, outline, find and read operations in one call. search: items of the tool search. outline: absolute file paths, as in ast_outline. find: list of queries, each in the format of ast_find. read: items of the tool ast_read.'
          - id: RetrieveTool.PHbHCQ|o9m8Np
            type: statements
            code: |-
              input_schema = {
                  'type': 'object',
                  'properties': {
                      'search': SearchTool.input_schema['properties']['items'],
                      'outline': OutlineNodesTool.input_schema['properties']['paths'],
                      'find': {
                          'type': 'array',
                          'items': FindNodesTool.input_schema,
                          'description': 'Independent ast_find queries.'},
                      'read': ReadNodeTool.input_schema['properties']['items']},
                  'additionalProperties': False}
          - id: RetrieveTool.handle
            type: FunctionDef
            code: |-
              def handle(self, ctx: ToolContext) -> ToolResult:
                  sections: dict[str, Any] = {}
                  warnings: list[str] = []
                  approve = True
                  '# (key, tool, list of argument dicts for that tool)'
                  calls: list[tuple[str, ToolDefinition, list[dict[str, Any]]]] = []
                  if ctx.arguments.get('search'):
                      calls.append(('search', SearchTool(ctx.registry), [{'items': ctx.arguments['search']}]))
                  if ctx.arguments.get('outline'):
                      calls.append(('outline', OutlineNodesTool(), [{'paths': ctx.arguments['outline']}]))
                  if ctx.arguments.get('find'):
                      calls.append(('find', FindNodesTool(), list(ctx.arguments['find'])))
                  if ctx.arguments.get('read'):
                      calls.append(('read', ReadNodeTool(), [{'items': ctx.arguments['read']}]))
                  for key, tool, arg_list in calls:
                      parts: list[dict[str, Any]] = []
                      for arguments in arg_list:
                          sub = tool.handle(ToolContext(session=ctx.session, arguments=arguments, registry=ctx.registry))
                          if sub.is_error:
                              parts = []
                              sections[key] = _SectionError(error=_error_text(sub)).to_dict()
                              approve = False
                              break
                          parts.append(sub.structured_content or {})
                          warnings.extend(sub.warnings)
                          approve = approve and sub.auto_approve
                      if parts:
                          sections[key] = _merge(parts)
                  if not sections:
                      return ToolResult.failure('At least one non-empty retrieval list is required.')
                  payload = RawPayload(
                      sections,
                      reason='composes the payloads of the individual retrieval tools',
                      warnings=tuple(warnings))
                  return ToolResult.ok(payload, auto_approve=approve)
```
/answer 4b3c-64-63 allow
ReasoningToken: 61
Control Request:
```yaml
id: 4b3c-65-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: "^\\s*description\\s*=|^[A-Z_]*DESCRIPTION\\s*=|^class \\w+\\(ToolDefinition\\)|^\\s*name\\s*="
    include:
    - search*.py
    - edit.py
    - retrieve.py
    limit: 30
  reason: Search-Tool-Beschreibung finden
```
/answer 4b3c-65-0 allow
Control Request:
```yaml
id: 4b3c-66-65
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
        matches:
        - path: mcp/exa/search.py
          matches:
          - "_DESCRIPTION = 'Search the web for any topic and get clean, ready-to-use content.\\n\\nBest for: Finding current information, facts, or answering questi…nReturns: an overview per result (id, title, author, excerpt) without text or url; call web_search_exa_results with the ids to get url and full text.'"
          - "class WebSearchExaTool(ToolDefinition):"
          - name = 'web_search_exa'
          - description = _DESCRIPTION
        - path: mcp/exa/search_results.py
          matches:
          - "_DESCRIPTION = 'Resolve ids returned by web_search_exa to their full text. With `path`, the results are saved as Markdown.'"
          - "class WebSearchExaResultsTool(ToolDefinition):"
          - name = 'web_search_exa_results'
          - description = _DESCRIPTION
        - path: ast/edit.py
          matches:
          - "class EditTool(ToolDefinition):"
          - name = 'ast_edit'
          - "description = 'Run create, insert, replace, edit_block, edit_marks and delete operations in one call. Each list has the item format of the tool ast_<list name>.'"
        - path: ast/retrieve.py
          matches:
          - "class RetrieveTool(ToolDefinition):"
          - name = 'ast_retrieve'
          - "description = 'Run search, outline, find and read operations in one call. search: items of the tool search. outline: absolute file paths, as in ast_outline. find: list of queries, each in the format of ast_find. read: items of the tool ast_read.'"
        - path: openalex/search.py
          matches:
          - "_SEARCH_DESCRIPTION = 'Keyword and boolean full-text search across OpenAlex scholarly entities (works by default). Searches titles, abstracts and full…rst page.\\n\\nReturns an identity overview per result (id, display_name/title, ...); call openalex_search_results with the ids to get the full record.'"
          - "class OpenalexSearchTool(ToolDefinition):"
          - name = 'openalex_search'
          - description = _SEARCH_DESCRIPTION
        - path: openalex/search_results.py
          matches:
          - "_DESCRIPTION = 'Resolve ids returned by openalex_search or openalex_semantic_search to their full record. With `path`, the records are saved as Markdown.'"
          - "class OpenalexSearchResultsTool(ToolDefinition):"
          - name = 'openalex_search_results'
          - description = _DESCRIPTION
        warnings:
        - 2 match line(s) exceeded 300 characters and were shortened in the middle.
```
/answer 4b3c-66-65 allow
ReasoningToken: 71
Control Request:
```yaml
id: 4b3c-67-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    pattern: ^class SearchTool|import .*SearchTool|import .*(OutlineNodesTool|FindNodesTool|ReadNodeTool)|^from .* import \($
    include:
    - '*.py'
    limit: 30
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
    node_type: ImportFrom|Import
  reason: Imports in retrieve.py und SearchTool-Definition finden
```
/answer 4b3c-67-0 allow
Control Request:
```yaml
id: 4b3c-68-67
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc/src
        matches:
        - path: xy/ai/mcpc/tools/ast/retrieve.py
          matches:
          - from xy.ai.mcpc.tools.ast.find import FindNodesTool
          - from xy.ai.mcpc.tools.ast.outline import OutlineNodesTool
          - from xy.ai.mcpc.tools.ast.read import ReadNodeTool
          - from xy.ai.mcpc.tools.search import SearchTool
        - path: xy/ai/mcpc/tools/openalex/__init__.py
          matches:
          - "from .search import OpenalexSearchTool, openalex_search"
          - "from .semantic_search import OpenalexSemanticSearchTool, openalex_semantic_search"
        - path: xy/ai/mcpc/tools/search/__init__.py
          matches:
          - "class SearchTool(ToolDefinition):"
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
        nodes:
        - id: olOlxE|dMnIlW
          type: imports
          code: |-
            from __future__ import annotations
            from dataclasses import dataclass
            from typing import Any
            from xy.ai.mcpc.tools.ast.find import FindNodesTool
            from xy.ai.mcpc.tools.ast.outline import OutlineNodesTool
            from xy.ai.mcpc.tools.ast.read import ReadNodeTool
            from xy.ai.mcpc.tools.function_registry import FunctionRegistry
            from xy.ai.mcpc.tools.results import ItemError, RawPayload
            from xy.ai.mcpc.tools.search import SearchTool
            from xy.ai.mcpc.tools.tool_context import ToolContext
        - id: q1JBaW|syp0aa
          type: imports
          code: "from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolFeature, ToolRegistry, ToolResult"
```
/answer 4b3c-68-67 allow
Control Request:
```yaml
id: 4b3c-69-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  reason: SearchTool-Beschreibung und Struktur lesen; Beschreibung der Tools in find/outline/read im Kontext
```
/answer 4b3c-69-0 allow
Control Request:
```yaml
id: 4b3c-6a-69
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
        nodes:
        - id: OyEiBH|qDejkd
          excert: "'Search tool – recursive extended-regex search for retrieval, for a batch of it…"
        - id: PBZWYD|oCP2kZ
          excert: "from dataclasses import dataclass, replace"
        - id: 8Pl1k7|sM8uy4
          excert: "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
        - id: aYnhRB|TkRNzb
          excert: "__all__ = ["
        - id: Dmisq5|1HOmR3
          excert: _INTENTS = _INTENTS_WITHOUT_LOCATION + _INTENTS_WITH_LOCATION
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
          excert: "@dataclass(frozen=True) class SearchResult(ItemResult):"
          docstring: "Result of a single search. Attributes: directory: The directory list exactly as…"
          children:
          - id: SearchResult.iJIbrz|RbNsuT
            excert: "'Result of a single search.\\n\\n    Attributes:\\n        directory: The director…"
          - id: SearchResult.key
            excert: "def key(self) -> dict[str, Any]:"
        - id: SearchItemError
          excert: "@dataclass(frozen=True) class SearchItemError(ItemError):"
          docstring: "Error running a single search, mirroring its input for result association."
          children:
          - id: SearchItemError.VNAUO2|QXFu2a
            excert: "'Error running a single search, mirroring its input for result association.'"
          - id: SearchItemError.key
            excert: "def key(self) -> dict[str, Any]:"
        - id: SearchBatchResult
          excert: "class SearchBatchResult(BatchResult[SearchResult, SearchItemError]):"
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
          excert: "def search(items: list[SearchItem], *, with_nodes: bool=False) -> SearchBatchResult:"
          docstring: "Run one or more independent searches. Limits apply per item, not per batch. Arg…"
        - id: check_intent
          excert: "def _check_intent(intent: str) -> None:"
        - id: SearchTool
          excert: "class SearchTool(ToolDefinition):"
          children:
          - id: SearchTool.IRSVLT|PCuUvD
            excert: name = 'search'
          - id: SearchTool.4W0jY1|nTluDB
            excert: "_ITEM_PROPERTIES = {"
          - id: SearchTool.Ao0JQi|xkwUus
            excert: "_ITEM_REQUIRED = ['directory', 'pattern']"
          - id: SearchTool.init
            excert: "def __init__(self, registry: ToolRegistry) -> None:"
          - id: SearchTool.handle
            excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
            docstring: "Delegate to :func: translating the MCP schema to/from the Python API."
        - id: register_search_tool
          excert: "def register_search_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
        nodes:
        - id: LjJRQf|jlFZZg
          excert: "'``ast_find`` tool: find AST nodes by type, name, id, line range or parent type…"
        - id: 6cUWvL|prGBvq
          excert: import re
        - id: indDlk|H3RaC4
          excert: "from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult"
        - id: rY0KYk|mmIbkP
          excert: "__all__ = ['FileNodesResult', 'FindNodesResult', 'ast_find', 'FindNodesTool', '…"
        - id: FileNodesResult
          excert: "@dataclass(frozen=True) class FileNodesResult:"
          docstring: "Nodes matching the given selectors within a single file. Attributes: path: The …"
        - id: FindNodesResult
          excert: "@dataclass(frozen=True) class FindNodesResult:"
          docstring: "Result of :func:`ast_find`. Attributes: files: One :class:`FileNodesResult` per…"
        - id: load_tolerant
          excert: "def _load_tolerant(path: str) -> core.Tree:"
          docstring: "Load *path*, tolerating a forgotten filename extension. If *path* does not exis…"
        - id: find_in_file
          excert: "def _find_in_file(path: str, *, exact: dict[str, Any], lineno: int | None, end_lineno: int | None, no_selector: bool, pattern: re.Pattern[str] | None, with_lines: bool, with_type: bool) -> FileNodesResult:"
        - id: ast_find
          excert: "def ast_find(paths: list[str], *, id: str | None=None, name: str | None=None, node_type: str | None=None, lineno: int | None=None, end_lineno: int | None=None, parent_type: str | None=None, text: str | None=None, regexp: str | None=None, with_lines: bool=True) -> FindNodesResult:"
          docstring: "Find nodes by id, type, name, line range, parent type, text or regexp. ``ast_fi…"
        - id: FileNodes
          excert: "@dataclass(frozen=True) class FileNodes(ItemResult):"
          docstring: Wire form of :class:`FileNodesResult`.
          children:
          - id: FileNodes.kOq5xc|DMHMKy
            excert: '''Wire form of :class:`FileNodesResult`.'''
          - id: FileNodes.key
            excert: "def key(self) -> dict[str, Any]:"
          - id: FileNodes.payload
            excert: "def payload(self) -> dict[str, Any]:"
        - id: FindItemError
          excert: "@dataclass(frozen=True) class FindItemError(ItemError):"
          children:
          - id: FindItemError.mpXwxY|zGgDrj
            excert: "path: str"
          - id: FindItemError.key
            excert: "def key(self) -> dict[str, Any]:"
        - id: FindBatchResult
          excert: "class FindBatchResult(BatchResult[FileNodes, FindItemError]):"
          docstring: One :class:`FileNodes` per input path.
        - id: FindNodesTool
          excert: "class FindNodesTool(ToolDefinition):"
          children:
          - id: FindNodesTool.4leSQE|6n0haZ
            excert: name = 'ast_find'
          - id: FindNodesTool.WDGjLy|2g3uCW
            excert: "input_schema = {"
          - id: FindNodesTool.handle
            excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
            docstring: "Delegate to :func:`ast_find`, translating the MCP schema to/from the AST API."
        - id: register
          excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
        nodes:
        - id: JY1HZH|PFSe6T
          excert: "'``ast_outline`` tool: list AST nodes of one or more files.'"
        - id: 8zfkq5|mYnKxv
          excert: from dataclasses import dataclass
        - id: indDlk|kJOxzQ
          excert: "from xy.ai.mcpc.tools.list import ListError, ListItem"
        - id: OeMnuN|SAOhjO
          excert: "__all__ = ["
        - id: OutlineNodesResult
          excert: "@dataclass(frozen=True) class OutlineNodesResult(ItemResult):"
          docstring: "Nodes of a single file. Attributes: path: The resolved path of the listed file …"
          children:
          - id: OutlineNodesResult.0zGfwO|pYFNlP
            excert: "\"Nodes of a single file.\\n\\n    Attributes:\\n        path: The resolved path of…"
          - id: OutlineNodesResult.key
            excert: "def key(self) -> dict[str, Any]:"
          - id: OutlineNodesResult.payload
            excert: "def payload(self) -> dict[str, Any]:"
        - id: OutlineNodesError
          excert: "@dataclass(frozen=True) class OutlineNodesError(ItemError):"
          docstring: "Error listing a single file. Attributes: path: The resolved path that failed, f…"
          children:
          - id: OutlineNodesError.5x2lwA|ZEDZs8
            excert: "'Error listing a single file.\\n\\n    Attributes:\\n        path: The resolved pa…"
          - id: OutlineNodesError.key
            excert: "def key(self) -> dict[str, Any]:"
        - id: OutlineNodesBatchResult
          excert: "class OutlineNodesBatchResult(BatchResult[OutlineNodesResult, OutlineNodesError]):"
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
          - id: OutlineNodesTool.fdroJ2|s5dKvY
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
        - id: ZXLaUr|0gsxpD
          excert: from dataclasses import dataclass
        - id: indDlk|H3RaC4
          excert: "from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult"
        - id: JnyjUM|0JyCWU
          excert: "__all__ = ['ReadItem', 'ReadResult', 'ReadError', 'ReadBatchResult', 'ast_read'…"
        - id: looks_like_root_intent
          excert: "def _looks_like_root_intent(ids: list[str]) -> bool:"
          docstring: Whether ``ids`` is empty or consists solely of common root-id guesses.
        - id: ReadItem
          excert: "@dataclass(frozen=True) class ReadItem:"
          docstring: "One file's node ids to read. Attributes: path: Absolute path to the file to rea…"
        - id: ReadResult
          excert: "@dataclass(frozen=True) class ReadResult(ItemResult):"
          docstring: "Nodes read from a single file. Attributes: path: The path exactly as given in t…"
          children:
          - id: ReadResult.XSpusS|BE4ytU
            excert: "\"Nodes read from a single file.\\n\\n    Attributes:\\n        path: The path exac…"
          - id: ReadResult.o8UJxD|FCAibe
            excert: "path: str"
          - id: ReadResult.key
            excert: "def key(self) -> dict[str, Any]:"
          - id: ReadResult.payload
            excert: "def payload(self) -> dict[str, Any]:"
        - id: ReadError
          excert: "@dataclass(frozen=True) class ReadError(ItemError):"
          docstring: "Error reading a whole item (e.g. bad path or a syntax error). Attributes: path:…"
          children:
          - id: ReadError.gNB1ym|2wizJp
            excert: '''Error reading a whole item (e.g. bad path or a syntax error).\n\n    Attribute…'
          - id: ReadError.key
            excert: "def key(self) -> dict[str, Any]:"
        - id: ReadBatchResult
          excert: "class ReadBatchResult(BatchResult[ReadResult, ReadError]):"
          docstring: "Result of :func:`ast_read`. Attributes: results: One :class:`ReadResult` per su…"
        - id: read_one
          excert: "def _read_one(item: ReadItem, *, with_lines: bool) -> ReadResult:"
        - id: outline_instead
          excert: "def _outline_instead(item: ReadItem, *, with_lines: bool, prior_warnings: tuple[str, ...]=()) -> tuple[list[ReadResult], list[ReadError]]:"
          docstring: The file's full outline; the result always carries a warning that this is a fal…
        - id: ast_read
          excert: "def ast_read(items: list[ReadItem], *, with_lines: bool=True) -> ReadBatchResult:"
          docstring: "Recursively read the subtree of each addressed node, across one or more files. …"
        - id: ReadNodeTool
          excert: "class ReadNodeTool(ToolDefinition):"
          children:
          - id: ReadNodeTool.Qka00o|1eoKTG
            excert: name = 'ast_read'
          - id: ReadNodeTool.4gWrRr|O9B7Vp
            excert: "_ITEM_PROPERTIES = {"
          - id: ReadNodeTool.handle
            excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
            docstring: "Delegate to :func:`ast_read`, translating the MCP schema to/from the AST API."
        - id: register
          excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 4b3c-6a-69 allow
ReasoningToken: 130
Control Request:
```yaml
id: 4b3c-6b-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    ids:
    - IRSVLT|PCuUvD
    - aYnhRB|TkRNzb
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
    ids:
    - 4leSQE|6n0haZ
    - rY0KYk|mmIbkP
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    ids:
    - fdroJ2|s5dKvY
    - OeMnuN|SAOhjO
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
    ids:
    - Qka00o|1eoKTG
    - JnyjUM|0JyCWU
  reason: Name/Description-Blöcke der vier Einzeltools lesen
```
/answer 4b3c-6b-0 allow
Control Request:
```yaml
id: 4b3c-6c-6b
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
        nodes:
        - id: aYnhRB|TkRNzb
          type: statements
          code: |-
            __all__ = [
                'SearchError',
                'SearchLineMatch',
                'SearchNodeMatch',
                'SearchMatch',
                'SearchItem',
                'SearchResult',
                'SearchItemError',
                'SearchBatchResult',
                'search',
                'SearchTool',
                'register_search_tool']
            _DEFAULT_LIMIT = 15
            _INTENTS_WITHOUT_LOCATION = ('probe', 'survey', 'sample', 'check_existence', 'find_examples_or_conventions')
            _INTENTS_WITH_LOCATION = ('locate_for_edit', 'retrieve_context', 'locate_error', 'edit')
        warnings:
        - No node matched 'IRSVLT|PCuUvD'.
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
        nodes:
        - id: rY0KYk|mmIbkP
          type: statements
          code: "__all__ = ['FileNodesResult', 'FindNodesResult', 'ast_find', 'FindNodesTool', 'register']"
        warnings:
        - No node matched '4leSQE|6n0haZ'.
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
        nodes:
        - id: OeMnuN|SAOhjO
          type: statements
          code: |-
            __all__ = [
                'OutlineNodesResult',
                'OutlineNodesError',
                'OutlineNodesBatchResult',
                'ast_outline',
                'OutlineNodesTool',
                'register']
            _MAX_DIR_EXPANSION = 5
        warnings:
        - No node matched 'fdroJ2|s5dKvY'.
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
        nodes:
        - id: JnyjUM|0JyCWU
          type: statements
          code: |-
            __all__ = ['ReadItem', 'ReadResult', 'ReadError', 'ReadBatchResult', 'ast_read', 'ReadNodeTool', 'register']
            _ROOT_INTENT_IDS = {'root', '_module_', '__module__', 'module', '', '*'}
        warnings:
        - No node matched 'Qka00o|1eoKTG'.
```
/answer 4b3c-6c-6b allow
Text:
 Ich habe die Struktur der vier Einzeltools erfasst. Als Nächstes lese ich ihre Klassenköpfe mit den Beschreibungen.
ReasoningToken: 25
Control Request:
```yaml
id: 4b3c-6d-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    regexp: ^\s*(name|description|title) =
    parent_type: ClassDef
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
    regexp: ^\s*name = 'ast_
    parent_type: ClassDef
  reason: Klassenattribute (name/title/description) der vier Tools im Wortlaut lesen
```
/answer 4b3c-6d-0 allow
Control Request:
```yaml
id: 4b3c-6e-6d
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
        nodes: []
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
        nodes: []
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
        nodes: []
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
        nodes: []
```
/answer 4b3c-6e-6d allow
Control Request:
```yaml
id: 4b3c-6f-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    ids:
    - SearchTool.IRSVLT|PCuUvD
    - SearchTool
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
    ids:
    - FindNodesTool.4leSQE|6n0haZ
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    ids:
    - OutlineNodesTool.fdroJ2|s5dKvY
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
    ids:
    - ReadNodeTool.Qka00o|1eoKTG
  reason: Klassen der Tools mit Klassenattributen lesen
```
/answer 4b3c-6f-0 allow
Control Request:
```yaml
id: 4b3c-70-6f
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
        nodes:
        - id: SearchTool.IRSVLT|PCuUvD
          type: statements
          code: |-
            name = 'search'
            title = 'Search code, text, id, and content within directories'
            description = "Run one or more independent searches with an extended regular expression, for a batch of items. Always use the 'include' and 'exclude' filters."
        - id: SearchTool
          type: ClassDef
          excert: "class SearchTool(ToolDefinition):"
          children:
          - id: SearchTool.IRSVLT|PCuUvD
            type: statements
            code: |-
              name = 'search'
              title = 'Search code, text, id, and content within directories'
              description = "Run one or more independent searches with an extended regular expression, for a batch of items. Always use the 'include' and 'exclude' filters."
          - id: SearchTool.4W0jY1|nTluDB
            type: statements
            code: |-
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
                      'default': _DEFAULT_INTENT}}
          - id: SearchTool.Ao0JQi|xkwUus
            type: statements
            code: |-
              _ITEM_REQUIRED = ['directory', 'pattern']
              _ITEMS_DESCRIPTION = 'Independent searches to run.'
              input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)
          - id: SearchTool.init
            type: FunctionDef
            code: |-
              def __init__(self, registry: ToolRegistry) -> None:
                  self._registry = registry
          - id: SearchTool.handle
            type: FunctionDef
            code: |-
              def handle(self, ctx: ToolContext) -> ToolResult:
                  """Delegate to :func: translating the MCP schema to/from the Python API."""
                  with_lines = self._registry.session_has_feature(ctx.session, ToolFeature.INPUT_LINENO)
                  with_nodes = self._registry.session_has_feature(ctx.session, ToolFeature.INPUT_ASTID)
                  intents: dict[int, str] = {}

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
                      located = item_intent in _INTENTS_WITH_LOCATION
                      entry: dict[str, Any] = {'path': f'{m.directory}/{m.filename}' if m.directory else m.filename}
                      if m.nodes:
                          entry['nodes'] = [{'id': n.node_id, 'matches': line_dicts(
                              n.matches, located and with_lines)} for n in m.nodes]
                      if m.matches:
                          lines = line_dicts(m.matches, located and (with_lines or with_nodes))
                          entry['matches'] = lines if located else list(dict.fromkeys(lines))
                      return entry

                  def result_payload(r: SearchResult) -> dict[str, Any]:
                      if not r.matches:
                          return {}
                      return {'matches': [file_dict(m, intents[id(r)]) for m in r.matches]}

                  def run(items: list[SearchItem]) -> SearchBatchResult:
                      results: list[SearchResult] = []
                      errors: list[SearchItemError] = []
                      for item in items:
                          batch = search([item], with_nodes=with_nodes)
                          effective = item.intent if item.intent is not None else _DEFAULT_INTENT
                          for r in batch.results:
                              intents[id(r)] = effective
                          results += batch.results
                          errors += batch.errors
                      return SearchBatchResult(results=results, errors=errors)
                  return handle_batch_tool(ctx, item_factory, run, SearchError, result_payload)
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
        nodes:
        - id: FindNodesTool.4leSQE|6n0haZ
          type: statements
          code: |-
            name = 'ast_find'
            title = 'Find code, text, id, and content within the specified files.'
            description = 'Filter the AST-node tree of a list of files by type, name, id, line range, parent type, text substring or regexp (grep). Returns matches per file with their full source. Use this when looking up content without IDs.'
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
        nodes:
        - id: OutlineNodesTool.fdroJ2|s5dKvY
          type: statements
          code: |-
            name = 'ast_outline'
            features = frozenset({ToolFeature.PROVIDES_OUTLINE})
            title = 'Hierarchical AST outline with IDs from a set of defined files'
            description = 'Given a list of one or more file names, returns a hierarchical AST tree with IDs, classes, functions, sections, types, excert, docstring, children, imports, and segments but never the full content. Other AST tools require IDs as input.'
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
        nodes:
        - id: ReadNodeTool.Qka00o|1eoKTG
          type: statements
          code: |-
            name = 'ast_read'
            features = frozenset({ToolFeature.INPUT_ASTID})
            title = 'Reads the code, text, and content of the nodes identified by the given IDs within the specified files.'
            description = 'After `ast_outline` or `ast_find`, reads the code, text, content, and children of the given AST node IDs within the given list of absolute file paths (never directories). Use this when you already know the IDs.'
```
/answer 4b3c-70-6f allow
Text:
 Ich lege in den vier Einzeltools je eine öffentliche Konstante für die Beschreibung an. `retrieve.py` setzt daraus eine Rahmenbeschreibung zusammen.
ReasoningToken: 1971
Control Request:
```yaml
id: 4b3c-71-0
toolName: ast_edit
arguments:
  insert:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: SearchTool
    position: before
    source: |
      SEARCH_DESCRIPTION = "Run one or more independent searches with an extended regular expression, for a batch of items. Always use the 'include' and 'exclude' filters."
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
    id: FindNodesTool
    position: before
    source: |
      AST_FIND_DESCRIPTION = 'Filter the AST-node tree of a list of files by type, name, id, line range, parent type, text substring or regexp (grep). Returns matches per file with their full source. Use this when looking up content without IDs.'
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    id: OutlineNodesTool
    position: before
    source: |
      AST_OUTLINE_DESCRIPTION = 'Given a list of one or more file names, returns a hierarchical AST tree with IDs, classes, functions, sections, types, excert, docstring, children, imports, and segments but never the full content. Other AST tools require IDs as input.'
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
    id: ReadNodeTool
    position: before
    source: |
      AST_READ_DESCRIPTION = 'After `ast_outline` or `ast_find`, reads the code, text, content, and children of the given AST node IDs within the given list of absolute file paths (never directories). Use this when you already know the IDs.'
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
    id: RetrieveTool
    position: before
    source: |+
      RETRIEVE_INTRO = (
          'Run search, outline, find and read operations in one call. '
          'All lists are optional, but at least one must be non-empty. '
          'The response contains one section per given list, keyed by the list name.')
      RETRIEVE_SEARCH_USAGE = "`search`: list of items, each in the format of the tool `search`."
      RETRIEVE_OUTLINE_USAGE = "`outline`: list of absolute file paths, as in the tool `ast_outline`."
      RETRIEVE_FIND_USAGE = "`find`: list of independent queries, each in the format of the tool `ast_find`."
      RETRIEVE_READ_USAGE = "`read`: list of items, each in the format of the tool `ast_read`."
      RETRIEVE_NOTES = (
          'The lists are processed independently: AST node ids found by `search`, `outline` or `find` '
          'in this call cannot be used by `read` in the same call. '
          'Use the ids from the response in a follow-up call, or pass `read` only ids that are already known.')


      def _tool_section(usage: str, tool_name: str, tool_description: str) -> str:
          """Usage line of a list parameter followed by the description of the tool it delegates to."""
          return f'{usage}\nDescription of `{tool_name}`: {tool_description}'


      RETRIEVE_DESCRIPTION = '\n\n'.join((
          RETRIEVE_INTRO,
          _tool_section(RETRIEVE_SEARCH_USAGE, 'search', SEARCH_DESCRIPTION),
          _tool_section(RETRIEVE_OUTLINE_USAGE, 'ast_outline', AST_OUTLINE_DESCRIPTION),
          _tool_section(RETRIEVE_FIND_USAGE, 'ast_find', AST_FIND_DESCRIPTION),
          _tool_section(RETRIEVE_READ_USAGE, 'ast_read', AST_READ_DESCRIPTION),
          RETRIEVE_NOTES))


  edit_marks:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: SearchTool.IRSVLT|PCuUvD
    start_marker: description = "Run one or more
    end_marker: filters."
    content: description = SEARCH_DESCRIPTION
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
    id: FindNodesTool.4leSQE|6n0haZ
    start_marker: description = 'Filter the AST
    end_marker: without IDs.'
    content: description = AST_FIND_DESCRIPTION
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    id: OutlineNodesTool.fdroJ2|s5dKvY
    start_marker: description = 'Given a list
    end_marker: as input.'
    content: description = AST_OUTLINE_DESCRIPTION
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
    id: ReadNodeTool.Qka00o|1eoKTG
    start_marker: description = 'After `ast_outline`
    end_marker: know the IDs.'
    content: description = AST_READ_DESCRIPTION
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
    id: RetrieveTool.axwi72|4oz5YL
    start_marker: description = 'Run search
    end_marker: of ast_read.'
    content: description = RETRIEVE_DESCRIPTION
    addImport:
    - from xy.ai.mcpc.tools.ast.find import AST_FIND_DESCRIPTION
    - from xy.ai.mcpc.tools.ast.outline import AST_OUTLINE_DESCRIPTION
    - from xy.ai.mcpc.tools.ast.read import AST_READ_DESCRIPTION
    - from xy.ai.mcpc.tools.search import SEARCH_DESCRIPTION
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: aYnhRB|TkRNzb
    old_text: |2-
          'search',
          'SearchTool',
    new_text: |2-
          'SEARCH_DESCRIPTION',
          'search',
          'SearchTool',
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
    id: rY0KYk|mmIbkP
    old_text: "__all__ = ['FileNodesResult',"
    new_text: "__all__ = ['AST_FIND_DESCRIPTION', 'FileNodesResult',"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    id: OeMnuN|SAOhjO
    old_text: |-
      __all__ = [
          'OutlineNodesResult',
    new_text: |-
      __all__ = [
          'AST_OUTLINE_DESCRIPTION',
          'OutlineNodesResult',
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
    id: JnyjUM|0JyCWU
    old_text: "__all__ = ['ReadItem',"
    new_text: "__all__ = ['AST_READ_DESCRIPTION', 'ReadItem',"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
    id: fiQhAE|pEjXcx
    old_text: "__all__ = ['RetrieveTool', 'register']"
    new_text: |-
      __all__ = [
          'RETRIEVE_INTRO',
          'RETRIEVE_SEARCH_USAGE',
          'RETRIEVE_OUTLINE_USAGE',
          'RETRIEVE_FIND_USAGE',
          'RETRIEVE_READ_USAGE',
          'RETRIEVE_NOTES',
          'RETRIEVE_DESCRIPTION',
          'RetrieveTool',
          'register']
  reason: Beschreibungen der Einzeltools in öffentliche Konstanten umwandeln und in retrieve.py aus mehreren Bausteinen zusammensetzen
```
/answer 4b3c-71-0 allow
Control Request:
```yaml
id: 4b3c-72-71
result:
  structuredContent:
    insert:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
        id: SearchTool
        inserted: 1
        ids:
        - yY0ITk|0Z5NUv
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
        id: FindNodesTool
        inserted: 1
        ids:
        - U8UN1n|JzikhG
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
        id: OutlineNodesTool
        inserted: 1
        ids:
        - SMaLm2|sKs8R4
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
        id: ReadNodeTool
        inserted: 1
        ids:
        - h6gfAe|I9OD4t
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
        id: RetrieveTool
        inserted: 8
        ids:
        - tgw7QR|uYDbkE
        - iiw3rw|2nSw9t
        - tool_section
        - tool_section.cKv1TH|91QFUL
        - MwquOI|93avxJ
    edit_block:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
        id: aYnhRB|TkRNzb
        new_id: aYnhRB|TkRNzb
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
        id: rY0KYk|mmIbkP
        new_id: rY0KYk|mmIbkP
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
        id: OeMnuN|SAOhjO
        new_id: OeMnuN|SAOhjO
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
        id: JnyjUM|0JyCWU
        new_id: JnyjUM|0JyCWU
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
        id: fiQhAE|pEjXcx
        new_id: fiQhAE|pEjXcx
    edit_marks:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
        id: SearchTool.IRSVLT|PCuUvD
        new_id: SearchTool.IRSVLT|PCuUvD
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
        id: FindNodesTool.4leSQE|6n0haZ
        new_id: FindNodesTool.4leSQE|6n0haZ
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
        id: OutlineNodesTool.fdroJ2|s5dKvY
        new_id: OutlineNodesTool.fdroJ2|s5dKvY
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
        id: ReadNodeTool.Qka00o|1eoKTG
        new_id: ReadNodeTool.Qka00o|1eoKTG
      errors:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
        id: RetrieveTool.axwi72|4oz5YL
        error: End marker not found in node.
        reason: not_found
        position: end
        next_step: reread_node
```
/answer 4b3c-72-71 allow
Text:
 Vier der fünf Dateien sind angepasst. In `retrieve.py` hat der End-Marker nicht gepasst, den korrigiere ich.
ReasoningToken: 50
Control Request:
```yaml
id: 4b3c-73-0
toolName: ast_edit
arguments:
  edit_marks:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
    id: RetrieveTool.axwi72|4oz5YL
    start_marker: description = 'Run search
    end_marker: tool ast_read.'
    content: description = RETRIEVE_DESCRIPTION
  reason: Beschreibung in RetrieveTool auf die neue Konstante umstellen (End-Marker korrigiert)
```
/answer 4b3c-73-0 allow
Text:
 Jetzt prüfe ich das Ergebnis: Imports, Konstanten und Klassenattribute in `retrieve.py`.
Control Request:
```yaml
id: 4b3c-74-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
    regexp: DESCRIPTION|^from xy.ai.mcpc.tools
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
    regexp: _DESCRIPTION
    node_type: statements
  reason: Ergebnis in retrieve.py und Einzeltools verifizieren
```
/answer 4b3c-74-0 allow
Control Request:
```yaml
id: 4b3c-75-74
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
        nodes:
        - id: Pexehx|5Ivw5r
          code: |-
            __all__ = [
                'RETRIEVE_INTRO',
                'RETRIEVE_SEARCH_USAGE',
                'RETRIEVE_OUTLINE_USAGE',
                'RETRIEVE_FIND_USAGE',
                'RETRIEVE_READ_USAGE',
                'RETRIEVE_NOTES',
                'RETRIEVE_DESCRIPTION',
                'RetrieveTool',
                'register']
        - id: MwquOI|93avxJ
          code: |-
            RETRIEVE_DESCRIPTION = '\n\n'.join(
                (RETRIEVE_INTRO, _tool_section(
                    RETRIEVE_SEARCH_USAGE, 'search', SEARCH_DESCRIPTION), _tool_section(
                        RETRIEVE_OUTLINE_USAGE, 'ast_outline', AST_OUTLINE_DESCRIPTION), _tool_section(
                            RETRIEVE_FIND_USAGE, 'ast_find', AST_FIND_DESCRIPTION), _tool_section(
                                RETRIEVE_READ_USAGE, 'ast_read', AST_READ_DESCRIPTION), RETRIEVE_NOTES))
        - id: RetrieveTool.yRKnVG|Ru3hYu
          code: |-
            name = 'ast_retrieve'
            features = frozenset({ToolFeature.INPUT_ASTID})
            title = 'Batch of retrieval operations (search, outline, find, read)'
            description = RETRIEVE_DESCRIPTION
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
        nodes:
        - id: aYnhRB|h2LCHj
          type: statements
          code: |-
            __all__ = [
                'SearchError',
                'SearchLineMatch',
                'SearchNodeMatch',
                'SearchMatch',
                'SearchItem',
                'SearchResult',
                'SearchItemError',
                'SearchBatchResult',
                'SEARCH_DESCRIPTION',
                'search',
                'SearchTool',
                'register_search_tool']
            _DEFAULT_LIMIT = 15
            _INTENTS_WITHOUT_LOCATION = ('probe', 'survey', 'sample', 'check_existence', 'find_examples_or_conventions')
            _INTENTS_WITH_LOCATION = ('locate_for_edit', 'retrieve_context', 'locate_error', 'edit')
        - id: yY0ITk|0Z5NUv
          type: statements
          code: "SEARCH_DESCRIPTION = \"Run one or more independent searches with an extended regular expression, for a batch of items. Always use the 'include' and 'exclude' filters.\""
        - id: SearchTool.oLXcUH|KvvPBp
          type: statements
          code: |-
            name = 'search'
            title = 'Search code, text, id, and content within directories'
            description = SEARCH_DESCRIPTION
        - id: SearchTool.Ao0JQi|xkwUus
          type: statements
          code: |-
            _ITEM_REQUIRED = ['directory', 'pattern']
            _ITEMS_DESCRIPTION = 'Independent searches to run.'
            input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
        nodes:
        - id: G0er2K|9TR2Gc
          type: statements
          code: "__all__ = ['AST_FIND_DESCRIPTION', 'FileNodesResult', 'FindNodesResult', 'ast_find', 'FindNodesTool', 'register']"
        - id: U8UN1n|JzikhG
          type: statements
          code: "AST_FIND_DESCRIPTION = 'Filter the AST-node tree of a list of files by type, name, id, line range, parent type, text substring or regexp (grep). Returns matches per file with their full source. Use this when looking up content without IDs.'"
        - id: FindNodesTool.SGW12S|6pbFCf
          type: statements
          code: |-
            name = 'ast_find'
            title = 'Find code, text, id, and content within the specified files.'
            description = AST_FIND_DESCRIPTION
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
        nodes:
        - id: a8OyYL|zwndtP
          type: statements
          code: |-
            __all__ = [
                'AST_OUTLINE_DESCRIPTION',
                'OutlineNodesResult',
                'OutlineNodesError',
                'OutlineNodesBatchResult',
                'ast_outline',
                'OutlineNodesTool',
                'register']
            _MAX_DIR_EXPANSION = 5
        - id: SMaLm2|sKs8R4
          type: statements
          code: "AST_OUTLINE_DESCRIPTION = 'Given a list of one or more file names, returns a hierarchical AST tree with IDs, classes, functions, sections, types, excert, docstring, children, imports, and segments but never the full content. Other AST tools require IDs as input.'"
        - id: OutlineNodesTool.uhyEc1|AVDtSi
          type: statements
          code: |-
            name = 'ast_outline'
            features = frozenset({ToolFeature.PROVIDES_OUTLINE})
            title = 'Hierarchical AST outline with IDs from a set of defined files'
            description = AST_OUTLINE_DESCRIPTION
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
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
        nodes:
        - id: 4ZhXJG|RpJyoG
          type: statements
          code: |-
            __all__ = [
                'AST_READ_DESCRIPTION',
                'ReadItem',
                'ReadResult',
                'ReadError',
                'ReadBatchResult',
                'ast_read',
                'ReadNodeTool',
                'register']
            _ROOT_INTENT_IDS = {'root', '_module_', '__module__', 'module', '', '*'}
        - id: h6gfAe|I9OD4t
          type: statements
          code: "AST_READ_DESCRIPTION = 'After `ast_outline` or `ast_find`, reads the code, text, content, and children of the given AST node IDs within the given list of absolute file paths (never directories). Use this when you already know the IDs.'"
        - id: ReadNodeTool.zyKIMw|l7VeL3
          type: statements
          code: |-
            name = 'ast_read'
            features = frozenset({ToolFeature.INPUT_ASTID})
            title = 'Reads the code, text, and content of the nodes identified by the given IDs within the specified files.'
            description = AST_READ_DESCRIPTION
            _ITEM_PROPERTIES = {
                'path': PATH_PROP,
                'ids': {
                    'type': 'array',
                    'items': {
                        'type': 'string'},
                    'description': 'List of AST node IDs to read.'}}
            _ITEM_REQUIRED = ['path']
            _ITEMS_DESCRIPTION = 'Per-file node IDs to read.'
        - id: ReadNodeTool.ei5nIo|Lu9IT0
          type: statements
          code: "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)"
```
/answer 4b3c-75-74 allow
ReasoningToken: 71
Control Request:
```yaml
id: 4b3c-76-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
    node_type: imports
  reason: "Prüfen, dass die Imports in retrieve.py ergänzt wurden"
```
/answer 4b3c-76-0 allow
Control Request:
```yaml
id: 4b3c-77-76
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
        nodes:
        - id: olOlxE|dMnIlW
          type: imports
          code: |-
            from __future__ import annotations
            from dataclasses import dataclass
            from typing import Any
            from xy.ai.mcpc.tools.ast.find import FindNodesTool
            from xy.ai.mcpc.tools.ast.outline import OutlineNodesTool
            from xy.ai.mcpc.tools.ast.read import ReadNodeTool
            from xy.ai.mcpc.tools.function_registry import FunctionRegistry
            from xy.ai.mcpc.tools.results import ItemError, RawPayload
            from xy.ai.mcpc.tools.search import SearchTool
            from xy.ai.mcpc.tools.tool_context import ToolContext
        - id: q1JBaW|syp0aa
          type: imports
          code: "from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolFeature, ToolRegistry, ToolResult"
```
/answer 4b3c-77-76 allow
Text:
 Die Imports fehlen noch, denn die `addImport`-Angaben gingen mit dem fehlgeschlagenen Edit verloren. Ich ergänze sie jetzt direkt in den bestehenden Importzeilen.
ReasoningToken: 33
Control Request:
```yaml
id: 4b3c-78-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
    id: olOlxE|dMnIlW
    old_text: from xy.ai.mcpc.tools.ast.find import FindNodesTool
    new_text: "from xy.ai.mcpc.tools.ast.find import AST_FIND_DESCRIPTION, FindNodesTool"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
    id: olOlxE|dMnIlW
    old_text: from xy.ai.mcpc.tools.ast.outline import OutlineNodesTool
    new_text: "from xy.ai.mcpc.tools.ast.outline import AST_OUTLINE_DESCRIPTION, OutlineNodesTool"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
    id: olOlxE|dMnIlW
    old_text: from xy.ai.mcpc.tools.ast.read import ReadNodeTool
    new_text: "from xy.ai.mcpc.tools.ast.read import AST_READ_DESCRIPTION, ReadNodeTool"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
    id: olOlxE|dMnIlW
    old_text: from xy.ai.mcpc.tools.search import SearchTool
    new_text: "from xy.ai.mcpc.tools.search import SEARCH_DESCRIPTION, SearchTool"
  reason: Fehlende Konstanten-Imports in retrieve.py ergänzen
```
/answer 4b3c-78-0 allow
ReasoningToken: 36
Control Request:
```yaml
id: 4b3c-79-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
    node_type: imports
  reason: Endkontrolle der Imports
```
/answer 4b3c-79-0 allow
Control Request:
```yaml
id: 4b3c-7a-79
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/retrieve.py
        nodes:
        - id: OFjR6W|ZBH0A0
          type: imports
          code: |-
            from __future__ import annotations
            from dataclasses import dataclass
            from typing import Any
            from xy.ai.mcpc.tools.ast.find import AST_FIND_DESCRIPTION, FindNodesTool
            from xy.ai.mcpc.tools.ast.outline import AST_OUTLINE_DESCRIPTION, OutlineNodesTool
            from xy.ai.mcpc.tools.ast.read import AST_READ_DESCRIPTION, ReadNodeTool
            from xy.ai.mcpc.tools.function_registry import FunctionRegistry
            from xy.ai.mcpc.tools.results import ItemError, RawPayload
        - id: q1JBaW|3SjxOO
          type: imports
          code: |-
            from xy.ai.mcpc.tools.search import SEARCH_DESCRIPTION, SearchTool
            from xy.ai.mcpc.tools.tool_context import ToolContext
            from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolFeature, ToolRegistry, ToolResult
```
/answer 4b3c-7a-79 allow
Result Stats: id=5e34e7ff-1963-49ca-9d47-c64a57ba632a, total: 51060, in: 28, out: 11738, reason: 0, read: 307453, write: 39294, start: 1795106792421, end: 1795106799611, time: 7129, total: 94845, io: 0, control: 61, sent: 0, recv: 190
Die Beschreibung von `ast_retrieve` wird jetzt aus einem Rahmen und den Beschreibungen der vier Einzeltools zusammengesetzt. Ich habe den Code weder ausgeführt noch einen Test oder eine Syntaxprüfung laufen lassen. Ich habe nur gelesen, was in den Dateien steht.

**Neue öffentliche Konstanten in den Einzeltools**

Jedes Tool hat seine bisherige `description` unverändert in eine Konstante vor der Klasse verschoben. Die Klasse setzt `description = <KONSTANTE>`, und die Konstanten stehen in `__all__`.

| Datei | Konstante |
|---|---|
| `tools/search/__init__.py` | `SEARCH_DESCRIPTION` |
| `tools/ast/find.py` | `AST_FIND_DESCRIPTION` |
| `tools/ast/outline.py` | `AST_OUTLINE_DESCRIPTION` |
| `tools/ast/read.py` | `AST_READ_DESCRIPTION` |

**`tools/ast/retrieve.py`**

Die Gesamtbeschreibung besteht aus einzelnen Bausteinen, nicht aus einem langen String. Alle Bausteine sind öffentlich und in `__all__` eingetragen:

- `RETRIEVE_INTRO` ist der Rahmen. Er sagt, dass alle Listen optional sind, mindestens eine nicht leer sein muss und die Antwort pro Liste einen Abschnitt enthält.
- `RETRIEVE_SEARCH_USAGE`, `RETRIEVE_OUTLINE_USAGE`, `RETRIEVE_FIND_USAGE` und `RETRIEVE_READ_USAGE` beschreiben je einen Parameter.
- `RETRIEVE_NOTES` weist darauf hin, dass die Listen unabhängig laufen. IDs aus `search`, `outline` oder `find` kann `read` im selben Aufruf nicht nutzen. Das entspricht dem Modul-Docstring.
- `_tool_section(usage, tool_name, tool_description)` ist ein privater Helfer. Er hängt an jede Parameterzeile die Beschreibung des zugehörigen Tools an.
- `RETRIEVE_DESCRIPTION` fügt Intro, die vier Abschnitte und die Hinweise mit `'\n\n'.join(...)` zusammen.

`RetrieveTool.description` ist jetzt `RETRIEVE_DESCRIPTION`. Die Importe der vier Konstanten habe ich ergänzt.

Ändert sich künftig die Beschreibung eines Einzeltools, wirkt das automatisch in `ast_retrieve`.

Die Beschreibung von `ast_edit` in `edit.py` ist ähnlich knapp. Ich habe sie nicht angefasst, weil du nur `retrieve.py` genannt hast. Dort ließe sich dasselbe Muster anwenden.
