Alle genannten Tools sollen um einen Batch Support erweitert werden. Das bedeutet das alle Input-Parameter gemeinsam von einer Liste umschlossen werden wie auch die Outputs von einer Liste umschlossen werden. Dadurch muss der Ouput in einigen Tools ergänzt werden um den gespiegelten Input von Path und IDs. Die ist notwendig für die Ergebnisassoziation. Das Ziel ist die Minimierung von Tool Zyklen und dem Agenten größere Flexibilität in seiner Arbeitsweise zu erlauben.
Bei Fehlern enthält der Output dann eine Liste von Ergebnissen und eine getrennte Liste von Fehlern.

Tools:
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py`
	- Das ausführen, mehrerer unabhängiger Grep aufrufe. Limits gelten Pro Aufruf, nicht pro Batch.
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py`
	- Das Auflisten multipler Zielverzeichnisse, Limits gelten Pro Aufruf, nicht pro Batch.
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py`
	- Das Auflisten multipler Dateien
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py`
	- Gleichzeitige Editieroperationen erlauben.
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py`
	- Gleichzeitige Editieroperationen erlauben.
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_line/__init__.py`
	- Gleichzeitige Editieroperationen erlauben.
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_lines/__init__.py`
	- Gleichzeitige Editieroperationen erlauben. Eine doppelte Zieldatei wird nur einmal editiert. Jede weitere Operation im selben Batch erzeugt einen Fehler da sich sukzessiv die Offsets verschieben können. 
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py`
	- Das Auslesen multipler Dateien
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py`
	- Das Auslesen multipler Skills
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py`
	- Das Schreiben oder Ersetzen multipler Zieldateien

Beispielweise wurden die AST-Tools schon auf Batch Support umgestellt: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py`
SystemInit:  id=749919e9-f0f6-4244-aedc-06153908e154, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 95d8-36
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    ids:
    - root
  reason: AST-Beispiel für Batch-Pattern verstehen
```
/answer 95d8-36 allow
ReasoningToken: 30
Control Request:
```yaml
id: 95d8-37
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
      nodes:
      - id: aoOqEy
        type: statements
      - id: IwT4eM
        type: imports
      - id: 298I9E
        type: statements
      - id: EditBlockItem
        type: ClassDef
        signature: "@dataclass(frozen=True) class EditBlockItem:"
        docstring: "One block edit to apply. Attributes: path: Absolute path to the file to modify.…"
      - id: EditBlockResult
        type: ClassDef
        signature: "@dataclass(frozen=True) class EditBlockResult:"
        docstring: "Result of applying a single block edit. Attributes: path: The path exactly as g…"
      - id: EditBlockError
        type: ClassDef
        signature: "@dataclass(frozen=True) class EditBlockError:"
        docstring: "Error applying a single block edit. Attributes: path: The path exactly as given…"
      - id: EditBlockBatchResult
        type: ClassDef
        signature: "@dataclass(frozen=True) class EditBlockBatchResult:"
        docstring: "Result of :func:`ast_edit_block`. Attributes: results: One :class:`EditBlockRes…"
      - id: node_guard
        type: FunctionDef
        signature: "def _node_guard(engine, reference):"
        docstring: Guard for tolerant node edits. Engines that flag malformed edits on replace nee…
      - id: edit_block_one
        type: FunctionDef
        signature: "def _edit_block_one(item: EditBlockItem) -> EditBlockResult:"
      - id: ast_edit_block
        type: FunctionDef
        signature: "def ast_edit_block(items: list[EditBlockItem]) -> EditBlockBatchResult:"
        docstring: Replace occurrence(s) of ``old_text`` with ``new_text`` inside one or more node…
      - id: EditBlockNodeTool
        type: ClassDef
        signature: "class EditBlockNodeTool(ToolDefinition):"
        children:
        - id: EditBlockNodeTool.O46Q9w
          type: statements
        - id: EditBlockNodeTool.DVbGWC
          type: statements
        - id: EditBlockNodeTool.8YWeHc
          type: statements
        - id: EditBlockNodeTool.sfOEmu
          type: statements
        - id: EditBlockNodeTool.handle
          type: FunctionDef
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_edit_block`, translating the MCP schema to/from the AST …"
      - id: register
        type: FunctionDef
        signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
      errors:
      - "ids ['root'] resolved to no node; redirected to ast_list, returning the full outline instead."
    errors: []
```
/answer 95d8-37 allow
Control Request:
```yaml
id: 95d8-38
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py
  reason: Struktur der zu ändernden Tools erfassen
```
/answer 95d8-38 allow
Control Request:
```yaml
id: 95d8-39
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
      nodes:
      - id: f67yFW
        type: statements
      - id: v4K2gi
        type: imports
      - id: UtlRYm
        type: statements
      - id: GrepError
        type: ClassDef
        signature: "class GrepError(Exception):"
        docstring: Raised when a grep search cannot be executed or its output cannot be parsed.
      - id: GrepMatch
        type: ClassDef
        signature: "@dataclass(frozen=True) class GrepMatch:"
        docstring: "A single grep match, parsed from a 'path:line:content' output line."
      - id: parse_grep_stdout
        type: FunctionDef
        signature: "def _parse_grep_stdout(stdout: str) -> list[GrepMatch]:"
        docstring: Parse grep's 'path:line:content' stdout into :class:`GrepMatch` objects.
      - id: as_list
        type: FunctionDef
        signature: "def _as_list(value: list[str] | None) -> list[str]:"
        docstring: Normalize an optional list into a list (empty if ``None``).
      - id: run_grep
        type: FunctionDef
        signature: "def _run_grep(directory: list[str], pattern: str, *, exclude: list[str] | None=None, include: list[str] | None=None, limit: int=_DEFAULT_LIMIT) -> ProcessResult:"
        docstring: Recursively search one or more directories for ``pattern`` (extended regexp). A…
      - id: grep
        type: FunctionDef
        signature: "def grep(directory: list[str], pattern: str, *, exclude: list[str] | None=None, include: list[str] | None=None, limit: int=_DEFAULT_LIMIT) -> list[GrepMatch]:"
        docstring: Recursively search one or more directories for ``pattern`` (extended regexp). A…
      - id: GrepTool
        type: ClassDef
        signature: "class GrepTool(ToolDefinition):"
        children:
        - id: GrepTool.jaNMzd
          type: statements
        - id: GrepTool.SO35Sv
          type: statements
        - id: GrepTool.5Z8RLe
          type: statements
        - id: GrepTool.O2wQfB
          type: statements
        - id: GrepTool.handle
          type: FunctionDef
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`grep`, translating the MCP schema to/from the Python API."
      - id: register_grep_tool
        type: FunctionDef
        signature: "def register_grep_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
      nodes:
      - id: Am5co8
        type: statements
      - id: WyX1mZ
        type: imports
      - id: PsfC9k
        type: statements
      - id: ListError
        type: ClassDef
        signature: "class ListError(Exception):"
        docstring: Raised when a directory listing cannot be performed.
      - id: ListResult
        type: ClassDef
        signature: "@dataclass(frozen=True) class ListResult:"
      - id: list
        type: FunctionDef
        signature: "def list(path: str, pattern: str | None=None) -> ListResult:"
        docstring: "List all files below the absolute directory ``path``, grouped like ``ls -R``. A…"
      - id: ListTool
        type: ClassDef
        signature: "class ListTool(ToolDefinition):"
        children:
        - id: ListTool.yuoNjV
          type: statements
        - id: ListTool.tN2LHL
          type: statements
        - id: ListTool.handle
          type: FunctionDef
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`list`, translating the MCP schema to/from the Python API."
      - id: register_list_tool
        type: FunctionDef
        signature: "def register_list_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
      nodes:
      - id: 2qVAW6
        type: statements
      - id: 7pKq8V
        type: imports
      - id: 428AXS
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
      - id: TextStatsResult
        type: ClassDef
        signature: "@dataclass(frozen=True) class TextStatsResult:"
      - id: compute_text_stats
        type: FunctionDef
        signature: "def compute_text_stats(text: str) -> TextStatsResult:"
        docstring: Compute size/line/word/complexity metrics for *text*. Extracted from :func:`com…
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
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
      nodes:
      - id: sDn4vm
        type: statements
      - id: aBYvRQ
        type: imports
      - id: 5lNV6U
        type: statements
      - id: EditBlockError
        type: ClassDef
        signature: "class EditBlockError(Exception):"
        docstring: Raised when a edit-block operation cannot be performed.
      - id: EditBlockResult
        type: ClassDef
        signature: "@dataclass(frozen=True) class EditBlockResult:"
      - id: edit_block
        type: FunctionDef
        signature: "def edit_block(path: str, old_text: str, new_text: str, exact: bool=False, replace_all: bool=False) -> EditBlockResult:"
        docstring: Replace occurrence(s) of ``old_text`` in the file at ``path`` with ``new_text``…
      - id: EditBlockTool
        type: ClassDef
        signature: "class EditBlockTool(ToolDefinition):"
        children:
        - id: EditBlockTool.cUwq7H
          type: statements
        - id: EditBlockTool.c52fkG
          type: statements
        - id: EditBlockTool.ZqxRbM
          type: statements
        - id: EditBlockTool.handle
          type: FunctionDef
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`edit_block`, translating the MCP schema to/from the Python A…"
      - id: register_edit_block_tool
        type: FunctionDef
        signature: "def register_edit_block_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py
      nodes:
      - id: 2oh2po
        type: statements
      - id: 0kGWYF
        type: imports
      - id: kp2FTL
        type: statements
      - id: EditMarksError
        type: ClassDef
        signature: "class EditMarksError(Exception):"
        docstring: Raised when a replace operation cannot be performed.
      - id: EditMarksResult
        type: ClassDef
        signature: "@dataclass(frozen=True) class EditMarksResult:"
      - id: edit_marks_text
        type: FunctionDef
        signature: "def edit_marks_text(text: str, begin_marker: str, content: str, end_marker: str, exact: bool=False) -> str:"
        docstring: Replace everything between and including 'begin_marker' and 'end_marker' with c…
      - id: edit_marks
        type: FunctionDef
        signature: "def edit_marks(path: str, begin_marker: str, end_marker: str, content: str, exact: bool=False) -> EditMarksResult:"
        docstring: Replace everything between and including 'start' and 'end' with content. Both m…
      - id: EditMarksTool
        type: ClassDef
        signature: "class EditMarksTool(ToolDefinition):"
        children:
        - id: EditMarksTool.3AO8Cb
          type: statements
        - id: EditMarksTool.gjnjFR
          type: statements
        - id: EditMarksTool.buscHT
          type: statements
        - id: EditMarksTool.handle
          type: FunctionDef
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`edit_marks`, translating the MCP schema to/from the Python A…"
      - id: register_edit_marks_tool
        type: FunctionDef
        signature: "def register_edit_marks_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    errors: []
```
/answer 95d8-39 allow
ReasoningToken: 41
Control Request:
```yaml
id: 95d8-3a
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    ids:
    - aoOqEy
    - IwT4eM
    - 298I9E
    - EditBlockItem
    - EditBlockResult
    - EditBlockError
    - EditBlockBatchResult
    - node_guard
    - edit_block_one
    - ast_edit_block
    - EditBlockNodeTool
    - register
  reason: Vollständigen Code des AST-Batch-Beispiels lesen
```
/answer 95d8-3a allow
Control Request:
```yaml
id: 95d8-3b
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
      nodes:
      - id: aoOqEy
        type: statements
        code: "'``ast_edit_block`` tool: exact-block (old_text -> new_text) edits within selected nodes.'"
      - id: IwT4eM
        type: imports
        code: |-
          from dataclasses import dataclass
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.ast import core
          from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_text
          from xy.ai.mcpc.tools._text_match import replace_in_block, line_preserving, TextMatchError
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: 298I9E
        type: statements
        code: |-
          __all__ = [
              'EditBlockItem',
              'EditBlockResult',
              'EditBlockError',
              'EditBlockBatchResult',
              'ast_edit_block',
              'EditBlockNodeTool',
              'register']
      - id: EditBlockItem
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditBlockItem:
              """One block edit to apply.

              Attributes:
                  path: Absolute path to the file to modify.
                  old_text: Unique 10-100 char block to find within the node's source (unless ``replace_all``).
                  new_text: Replacement text (may be empty to delete the block).
                  exact: If False (default), whitespace in ``old_text`` is matched tolerantly.
                  replace_all: If True, replace every occurrence instead of requiring a single match.
                  id: Unique id of the target node. If omitted, the node is searched for by ``old_text``.
              """
              path: str
              old_text: str
              new_text: str
              exact: bool = False
              replace_all: bool = False
              id: str | None = None
      - id: EditBlockResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditBlockResult:
              """Result of applying a single block edit.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  result: Always ``"success"``.
                  new_id: The node's new id, only set if the edit changed it.
              """
              path: str
              id: str | None
              result: str
              new_id: str | None = None
      - id: EditBlockError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditBlockError:
              """Error applying a single block edit.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  error: The error message.
                  candidates: On ambiguity (id omitted, several nodes matched), the candidate node ids.
              """
              path: str
              id: str | None
              error: str
              candidates: list[str] | None = None
      - id: EditBlockBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditBlockBatchResult:
              """Result of :func:`ast_edit_block`.

              Attributes:
                  results: One :class:`EditBlockResult` per successfully applied edit.
                  errors: One :class:`EditBlockError` per edit that failed.
              """
              results: list[EditBlockResult]
              errors: list[EditBlockError]
      - id: node_guard
        type: FunctionDef
        code: |-
          def _node_guard(engine, reference):
              """Guard for tolerant node edits.

              Engines that flag malformed edits on replace need no extra check. Others
              (e.g. markup grammars) must not merge lines and must still re-parse cleanly.
              """
              if engine.validates_syntax:
                  return None
              keep_lines = line_preserving(reference)
              return lambda span, result: keep_lines(span, result) and engine.validate(result) is None
      - id: edit_block_one
        type: FunctionDef
        code: |-
          def _edit_block_one(item: EditBlockItem) -> EditBlockResult:
              file_path = core.require_path(item.path)
              tree = core.CACHE.get_tree(file_path)
              target = select_by_text(tree, [item.old_text], id=item.id)
              node_source = core.edit_node_source(target)
              try:
                  new_source = replace_in_block(
                      node_source,
                      item.old_text,
                      item.new_text,
                      exact=item.exact,
                      replace_all=item.replace_all,
                      accept=_node_guard(
                          tree.engine,
                          item.old_text),
                      max_level=3 if tree.engine.validates_syntax else 2,
                      where='node')
              except TextMatchError as exc:
                  raise core.AstError(str(exc)) from exc
              new_id = core.replace_node(target, new_source)
              core.CACHE.save(file_path, tree)
              return EditBlockResult(path=item.path, id=item.id, result='success', new_id=new_id)
      - id: ast_edit_block
        type: FunctionDef
        code: |-
          def ast_edit_block(items: list[EditBlockItem]) -> EditBlockBatchResult:
              """Replace occurrence(s) of ``old_text`` with ``new_text`` inside one or more nodes.

              Each addressed node's source is unparsed, its ``old_text`` block replaced
              (as with ``edit_block``), re-parsed, and used to replace the node.

              Args:
                  items: Block edits to apply. Must be non-empty.

              Returns:
                  EditBlockBatchResult: One result per successful edit, one error per failed edit.

              Raises:
                  core.AstError: If ``items`` is empty.
              """
              if not items:
                  raise core.AstError("'items' must be a non-empty list.")
              results: list[EditBlockResult] = []
              errors: list[EditBlockError] = []
              for item in items:
                  try:
                      results.append(_edit_block_one(item))
                  except core.AstAmbiguous as exc:
                      errors.append(EditBlockError(path=item.path, id=item.id, error=str(exc), candidates=exc.candidates))
                  except core.AstError as exc:
                      errors.append(EditBlockError(path=item.path, id=item.id, error=str(exc)))
              return EditBlockBatchResult(results=results, errors=errors)
      - id: EditBlockNodeTool
        type: ClassDef
        signature: "class EditBlockNodeTool(ToolDefinition):"
        children:
        - id: EditBlockNodeTool.O46Q9w
          type: statements
          code: |-
            name = 'ast_edit_block'
            title = 'Replace short text within AST nodes'
            description = "Replace occurrence(s) of short 'old_text' with 'new_text', within nodes addressed by id, for a batch of items. Don't use for large edits, use ast_edit_marks instead."
        - id: EditBlockNodeTool.DVbGWC
          type: statements
          code: |-
            input_schema = {
                'type': 'object',
                'properties': {
                    'items': {
                        'type': 'array',
                        'minItems': 1,
                        'items': {
                            'type': 'object',
                            'additionalProperties': False,
                            'properties': {
                                'path': {
                                    'type': 'string',
                                    'description': 'Absolute path to the file.'},
                                'old_text': {
                                    'type': 'string',
                                    'minLength': 10,
                                    'maxLength': 100,
                                    'description': 'Short text (10-100 chars) to replace within the node. Must occur exactly once, or replaceAll is set.'},
                                'new_text': {
                                    'type': 'string',
                                            'description': 'Replacement text, may be empty to remove the text.'},
                                'exact': {
                                    'type': 'boolean',
                                    'description': "If true, 'old_text' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                                    'default': False},
                                'replaceAll': {
                                    'type': 'boolean',
                                    'description': "If true, replace every occurrence of 'old_text' within the node instead of a single unique match.",
                                    'default': False},
                                **PATH_SELECTOR_PROPS},
                            'required': [
                                'path',
                                'old_text',
                                'new_text']},
                        'description': 'Block edits to apply.'}},
                'required': ['items']}
        - id: EditBlockNodeTool.8YWeHc
          type: statements
          code: |-
            output_schema = {
                'type': 'object', 'properties': {
                    'results': {
                        'type': 'array', 'items': {
                            'type': 'object', 'properties': {
                                'path': {
                                    'type': 'string'}, 'id': {
                                        'type': 'string'}, 'result': {
                                            'type': 'string', 'description': 'Result status'}, 'new_id': {
                                                'type': 'string', 'description': "The node's new id. Prefer 'ast_find' over 'ast_list'."}}, 'required': [
                                                    'path', 'result']}}, 'errors': {
                                                        'type': 'array', 'items': {
                                                            'type': 'object', 'properties': {
                                                                'path': {
                                                                    'type': 'string'}, 'id': {
                                                                        'type': 'string'}, 'error': {
                                                                            'type': 'string'}, 'candidates': {
                                                                                'type': 'array', 'items': {
                                                                                    'type': 'string'}, 'description': 'On ambiguity (id omitted, several nodes matched), the candidate node ids.'}}, 'required': [
                                                                                        'path', 'error']}}}, 'required': [
                                                                                            'results', 'errors']}
        - id: EditBlockNodeTool.sfOEmu
          type: statements
          code: "annotations = {'readOnlyHint': False, 'openWorldHint': False}"
        - id: EditBlockNodeTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_edit_block`, translating the MCP schema to/from the AST API."""
                args: dict[str, Any] = ctx.arguments
                raw_items = args.get('items') or []
                if not raw_items:
                    return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
                items = [
                    EditBlockItem(
                        path=it['path'],
                        old_text=it['old_text'],
                        new_text=it['new_text'],
                        exact=it.get(
                            'exact',
                            False),
                        replace_all=it.get(
                            'replaceAll',
                            False),
                        id=it.get('id')) for it in raw_items]
                batch = ast_edit_block(items)
                results = []
                for r in batch.results:
                    entry = {'path': r.path, 'id': r.id, 'result': r.result}
                    if r.new_id is not None:
                        entry['new_id'] = r.new_id
                    results.append(entry)
                errors = []
                for e in batch.errors:
                    entry = {'path': e.path, 'id': e.id, 'error': e.error}
                    if e.candidates is not None:
                        entry['candidates'] = e.candidates
                    errors.append(entry)
                is_error = bool(batch.errors) and (not batch.results)
                return ToolResult(
                    structured_content={
                        'results': results,
                        'errors': errors},
                    is_error=is_error,
                    auto_approve=not is_error)
      - id: register
        type: FunctionDef
        code: |-
          def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
              registry.register(EditBlockNodeTool())
              functions.register(ast_edit_block)
      errors: []
    errors: []
```
/answer 95d8-3b allow
Control Request:
```yaml
id: 95d8-3c
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
    ids:
    - f67yFW
    - v4K2gi
    - UtlRYm
    - GrepError
    - GrepMatch
    - parse_grep_stdout
    - as_list
    - run_grep
    - grep
    - GrepTool
    - register_grep_tool
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    ids:
    - Am5co8
    - WyX1mZ
    - PsfC9k
    - ListError
    - ListResult
    - list
    - ListTool
    - register_list_tool
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
    ids:
    - 2qVAW6
    - 7pKq8V
    - 428AXS
    - FileStatsError
    - FileStatsResult
    - calculate_complexity
    - TextStatsResult
    - compute_text_stats
    - compute_file_stats
    - file_stats
    - FileStatsTool
    - register_file_stats_tool
  reason: Vollständigen Code der zu ändernden Tools lesen
```
/answer 95d8-3c allow
Control Request:
```yaml
id: 95d8-3d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
      nodes:
      - id: f67yFW
        type: statements
        code: '''Grep tool – recursive extended-regex search for retrieval.'''
      - id: v4K2gi
        type: imports
        code: |-
          from dataclasses import dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools._directories import normalize_directories
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, run_process
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
          import re
      - id: UtlRYm
        type: statements
        code: |-
          __all__ = ['GrepError', 'GrepMatch', 'grep', 'GrepTool', 'register_grep_tool']
          _DEFAULT_LIMIT = 15
          _MAX_LIMIT = 50
      - id: GrepError
        type: ClassDef
        code: |-
          class GrepError(Exception):
              """Raised when a grep search cannot be executed or its output cannot be parsed."""
      - id: GrepMatch
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class GrepMatch:
              """A single grep match, parsed from a 'path:line:content' output line."""
              directory: str
              filename: str
              lineno: int
              match: str
      - id: parse_grep_stdout
        type: FunctionDef
        code: |-
          def _parse_grep_stdout(stdout: str) -> list[GrepMatch]:
              """Parse grep's 'path:line:content' stdout into :class:`GrepMatch` objects."""
              matches: list[GrepMatch] = []
              for line in stdout.splitlines():
                  if not line:
                      continue
                  path, sep, rest = line.partition(':')
                  if not sep:
                      raise GrepError(f'Cannot parse grep output line: {line!r}')
                  lineno_str, sep, match = rest.partition(':')
                  if not sep or not lineno_str.isdigit():
                      raise GrepError(f'Cannot parse grep output line: {line!r}')
                  directory, _, filename = path.rpartition('/')
                  matches.append(GrepMatch(directory=directory, filename=filename, lineno=int(lineno_str), match=match))
              return matches
      - id: as_list
        type: FunctionDef
        code: |-
          def _as_list(value: list[str] | None) -> list[str]:
              """Normalize an optional list into a list (empty if ``None``)."""
              return list(value) if value is not None else []
      - id: run_grep
        type: FunctionDef
        code: |-
          def _run_grep(directory: list[str], pattern: str, *, exclude: list[str] | None=None, include: list[str] | None=None, limit: int=_DEFAULT_LIMIT) -> ProcessResult:
              """Recursively search one or more directories for ``pattern`` (extended regexp).

              Args:
                  directory: Absolute paths of the directories to search (each must exist and
                      be a directory).
                  pattern: Extended regular expression (grep -E syntax).
                  exclude: Globs of file names to exclude from the search, if given.
                  include: Globs of file names to include in the search, if given.
                  limit: Maximum number of matching lines to return (1..``_MAX_LIMIT``).

              Returns:
                  ProcessResult with:
                      exit_code: 0 if matches were found, 1 if none were found, >=2 on grep error.
                      stdout: Matching lines as 'path:line:content', with ``path`` relative to
                          whichever searched directory it was found under, truncated to at most
                          ``limit`` lines.
                      stderr: Standard error output (up to 3000 chars; see stderr_file if longer).
                      stdout_file: Absolute path to temp file with full stdout if to large.
                      stderr_file: Absolute path to temp file with full stderr if to large.

              Raises:
                  GrepError: If a directory is not absolute.
                  GrepError: If a directory does not exist or is not a directory.
                  GrepError: If no directory is given.
                  GrepError: If pattern is empty.
                  GrepError: If limit is not between 1 and ``_MAX_LIMIT``.
                  GrepError: If grep binary cannot be launched.
              """
              directory_paths = normalize_directories([Path(d) for d in _as_list(directory)])
              if not directory_paths:
                  raise GrepError('At least one directory is required.')
              for directory_path in directory_paths:
                  if not directory_path.is_absolute():
                      raise GrepError('directory must be an absolute path.')
                  if not directory_path.is_dir():
                      raise GrepError('Directory not found or not a directory.')
              if not pattern:
                  raise GrepError('pattern must not be empty.')
              if not 1 <= limit <= _MAX_LIMIT:
                  raise GrepError(f'limit must be between 1 and {_MAX_LIMIT}.')
              cmd = ['grep', '--recursive', '--line-number', '--extended-regexp', '--binary-files=without-match', '--color=never']
              for pattern_glob in _as_list(include):
                  cmd.append(f'--include={pattern_glob}')
              for pattern_glob in _as_list(exclude):
                  cmd.append(f'--exclude={pattern_glob}')
              cmd += ['--', pattern, *(str(p) for p in directory_paths)]
              try:
                  result = run_process(cmd)
              except LaunchError as exc:
                  raise GrepError(f'Failed to launch grep: {exc}') from exc
              prefixes = sorted((str(p).rstrip('/') + '/' for p in directory_paths), key=len, reverse=True)
              prefix_pattern = '|'.join((re.escape(p) for p in prefixes))
              stdout = re.sub(f'^(?:{prefix_pattern})', '', result.stdout, flags=re.MULTILINE)
              lines = stdout.splitlines()
              stdout = '\n'.join(lines[:limit])
              return ProcessResult(exit_code=result.exit_code, stdout=stdout, stderr=result.stderr)
      - id: grep
        type: FunctionDef
        code: |-
          def grep(directory: list[str], pattern: str, *, exclude: list[str] | None=None, include: list[str] | None=None, limit: int=_DEFAULT_LIMIT) -> list[GrepMatch]:
              """Recursively search one or more directories for ``pattern`` (extended regexp).

              Args:
                  directory: Absolute paths of the directories to search (each must exist and
                      be a directory).
                  pattern: Extended regular expression (grep -E syntax).
                  exclude: Globs of file names to exclude from the search, if given.
                  include: Globs of file names to include in the search, if given.
                  limit: Maximum number of matches to return (1..``_MAX_LIMIT``).

              Returns:
                  List of GrepMatch objects, each with the directory (relative to whichever
                  searched directory it was found under), the filename and the match
                  ('line:content'). Empty if no matches were found.

              Raises:
                  GrepError: If a directory is not absolute.
                  GrepError: If a directory does not exist or is not a directory.
                  GrepError: If no directory is given.
                  GrepError: If pattern is empty.
                  GrepError: If limit is not between 1 and ``_MAX_LIMIT``.
                  GrepError: If grep binary cannot be launched.
                  GrepError: If grep exits with an error (exit code >= 2).
                  GrepError: If the grep output cannot be parsed into directory, filename and match.
              """
              result = _run_grep(directory, pattern, exclude=exclude, include=include, limit=limit)
              if result.exit_code >= 2:
                  raise GrepError(f'grep failed (exit code {result.exit_code}): {result.stderr}')
              return _parse_grep_stdout(result.stdout)
      - id: GrepTool
        type: ClassDef
        signature: "class GrepTool(ToolDefinition):"
        children:
        - id: GrepTool.jaNMzd
          type: statements
          code: |-
            name = 'grep'
            title = 'Search files with grep'
            description = f"Recursively search a directory for lines matching an extended regular expression. Always use the 'include' and 'exclude' filters."
        - id: GrepTool.SO35Sv
          type: statements
          code: |-
            input_schema = {
                'type': 'object',
                'properties': {
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
                        'description': f'Maximum number of matching lines to return.',
                        'default': _DEFAULT_LIMIT,
                        'minimum': 1,
                        'maximum': _MAX_LIMIT}},
                'required': [
                    'directory',
                    'pattern']}
        - id: GrepTool.5Z8RLe
          type: statements
          code: |-
            output_schema = {
                'type': 'object', 'properties': {
                    'matches': {
                        'type': 'array', 'items': {
                            'type': 'object', 'properties': {
                                'path': {
                                    'type': 'string'}, 'lineno': {
                                        'type': 'integer'}, 'match': {
                                            'type': 'string'}}, 'required': [
                                                'path', 'lineno', 'match']}}, 'warning': {
                                                    'type': 'string'}}, 'required': ['matches']}
        - id: GrepTool.O2wQfB
          type: statements
          code: "annotations = {'readOnlyHint': True, 'idempotentHint': True, 'openWorldHint': False}"
        - id: GrepTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`grep`, translating the MCP schema to/from the Python API."""
                args: dict[str, Any] = ctx.arguments
                limit = int(args.get('limit', _DEFAULT_LIMIT))
                try:
                    matches = grep(
                        args['directory'],
                        args['pattern'],
                        exclude=args.get('exclude'),
                        include=args.get('include'),
                        limit=limit)
                except GrepError as exc:
                    return ToolResult(content=[text_content(str(exc))], is_error=True)
                structured_content: dict[str,
                                         Any] = {'matches': [{'path': f'{match.directory}/{match.filename}' if match.directory else match.filename,
                                                              'lineno': match.lineno,
                                                              'match': match.match} for match in matches]}
                content = []
                if len(matches) >= limit:
                    warning = f'Limit of {limit} matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit.'
                    structured_content['warning'] = warning
                    content.append(text_content(warning))
                return ToolResult(content=content, structured_content=structured_content)
      - id: register_grep_tool
        type: FunctionDef
        code: |-
          def register_grep_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
              registry.register(GrepTool())
              functions.register(grep)
      errors: []
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
      nodes:
      - id: Am5co8
        type: statements
        code: "\"List tool – returns files below a directory, grouped by relative subdirectory like ``ls -R``.\\n\\nWalks the given absolute directory recursively and returns all file paths\\n(files only, no directories), grouped by the relative directory they live in\\n(e.g. ``./src/pkg:`` followed by tab-indented file names), mirroring the\\noutput format of ``ls -R``. An optional regular expression can be supplied to\\nfilter the resulting files (matched against each file's path relative to the\\nrequested directory). Common VCS/build/cache directories (e.g. ``.git``) are\\nalways excluded. To keep results manageable, the number of matched files is\\ncapped; use ``pattern`` to narrow down large directories instead of raising\\nthe limit.\\n\""
      - id: WyX1mZ
        type: imports
        code: |-
          import os
          import re
          from dataclasses import dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools._directories import normalize_directory
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: PsfC9k
        type: statements
        code: |-
          __all__ = ['ListError', 'ListResult', 'list', 'ListTool', 'register_list_tool']
          _MAX_ENTRIES = 50
          _EXCLUDED_DIRS = {
              '.git',
              '.hg',
              '.svn',
              '__pycache__',
              '.mypy_cache',
              '.pytest_cache',
              '.ruff_cache',
              '.tox',
              '.venv',
              'venv',
              'node_modules',
              '.idea',
              '.vscode',
              'dist',
              'build',
              '.cache'}
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
              entries: list[str]
      - id: list
        type: FunctionDef
        code: |-
          def list(path: str, pattern: str | None=None) -> ListResult:
              """List all files below the absolute directory ``path``, grouped like ``ls -R``.

              Args:
                  path: Absolute directory path to list (must exist and be a directory).
                  pattern: Optional regular expression to filter results. Only matching file paths are included.

              Returns:
                  ListResult with:
                      entries: Lines of output, one directory header (e.g. ``./sub:``)
                          followed by its file names. Directories without matching
                          files are omitted.

              Raises:
                  ListError: If path is not absolute.
                  ListError: If path does not exist or is not a directory.
                  ListError: If pattern is not a valid regular expression.
              """
              dir_path = Path(path)
              if not dir_path.is_absolute():
                  raise ListError('Path must be absolute.')
              dir_path = normalize_directory(dir_path)
              if not dir_path.is_dir():
                  raise ListError('Directory not found or not a directory.')
              try:
                  regex = re.compile(pattern) if pattern else None
              except re.error as exc:
                  raise ListError(f'Invalid regex pattern: {exc}') from exc
              groups: dict[str, list[str]] = {}
              match_count = 0
              for root, dirs, files in os.walk(str(dir_path)):
                  rel_dir = os.path.relpath(root, str(dir_path))
                  matched_files = []
                  for file in sorted(files):
                      rel_path = os.path.normpath(os.path.join(rel_dir, file))
                      if regex is None or regex.search(rel_path):
                          matched_files.append(file)
                  if matched_files:
                      groups[rel_dir] = matched_files
                      match_count += len(matched_files)
              if match_count > _MAX_ENTRIES:
                  raise ListError(
                      f"Too many entries ({match_count}) exceed the limit of {_MAX_ENTRIES}. Narrow down the result using the 'pattern' regular expression parameter.")
              entries = []
              for rel_dir in sorted(groups):
                  header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
                  entries.append(f'{header}:')
                  entries.extend(groups[rel_dir])
              return ListResult(entries=entries)
      - id: ListTool
        type: ClassDef
        signature: "class ListTool(ToolDefinition):"
        children:
        - id: ListTool.yuoNjV
          type: statements
          code: |-
            name = 'list'
            title = 'List directory contents'
            description = 'List all files below an absolute directory path, recursively, as a flat list. Filter the result with a regular expression.'
            input_schema = {
                'type': 'object',
                'properties': {
                    'path': {
                        'type': 'string',
                        'description': 'Absolute directory path.'},
                    'pattern': {
                        'type': 'string',
                        'description': 'Regular expression to filter the result.'}},
                'required': ['path']}
        - id: ListTool.tN2LHL
          type: statements
          code: |-
            output_schema = {
                'type': 'object',
                'properties': {
                    'entries': {
                        'type': 'array',
                        'items': {
                            'type': 'string'}}},
                'required': ['entries']}
            annotations = {'readOnlyHint': True, 'openWorldHint': False}
        - id: ListTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`list`, translating the MCP schema to/from the Python API."""
                args: dict[str, Any] = ctx.arguments
                try:
                    result = list(path=args['path'], pattern=args.get('pattern'))
                except ListError as exc:
                    return ToolResult(content=[text_content(str(exc))], is_error=True)
                return ToolResult(structured_content={'entries': result.entries})
      - id: register_list_tool
        type: FunctionDef
        code: |-
          def register_list_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
              registry.register(ListTool())
              functions.register(list)
      errors: []
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
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
      - id: 428AXS
        type: statements
        code: |-
          __all__ = [
              'FileStatsError',
              'FileStatsResult',
              'TextStatsResult',
              'compute_text_stats',
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
      - id: TextStatsResult
        type: ClassDef
        code: |-
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
      - id: compute_text_stats
        type: FunctionDef
        code: |-
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
      - id: compute_file_stats
        type: FunctionDef
        code: |-
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
                  path=str(
                      path.resolve()),
                  created=created,
                  modified=modified,
                  accessed=accessed,
                  **text_stats)
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
      errors: []
    errors: []
```
/answer 95d8-3d allow
ReasoningToken: 76
Control Request:
```yaml
id: 95d8-3e
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
    ids:
    - sDn4vm
    - aBYvRQ
    - 5lNV6U
    - EditBlockError
    - EditBlockResult
    - edit_block
    - EditBlockTool
    - register_edit_block_tool
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py
    ids:
    - 2oh2po
    - 0kGWYF
    - kp2FTL
    - EditMarksError
    - EditMarksResult
    - edit_marks_text
    - edit_marks
    - EditMarksTool
    - register_edit_marks_tool
  reason: Vollständigen Code der edit_block und edit_marks Tools lesen
```
/answer 95d8-3e allow
Control Request:
```yaml
id: 95d8-3f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
      nodes:
      - id: sDn4vm
        type: statements
        code: '''Edit-block tool – Edits an exact block of text (old -> new) in a file.'''
      - id: aBYvRQ
        type: imports
        code: |-
          from dataclasses import dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools._text_match import replace_in_block, line_preserving, TextMatchError
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: 5lNV6U
        type: statements
        code: "__all__ = ['EditBlockError', 'EditBlockResult', 'edit_block', 'EditBlockTool', 'register_edit_block_tool']"
      - id: EditBlockError
        type: ClassDef
        code: |-
          class EditBlockError(Exception):
              """Raised when a edit-block operation cannot be performed."""
      - id: EditBlockResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditBlockResult:
              result: str
      - id: edit_block
        type: FunctionDef
        code: |-
          def edit_block(path: str, old_text: str, new_text: str, exact: bool=False, replace_all: bool=False) -> EditBlockResult:
              """Replace occurrence(s) of ``old_text`` in the file at ``path`` with ``new_text``.

              Matching escalates through whitespace/escape/quote tolerance; a candidate is
              only accepted when it preserves ``old_text``'s line structure, so no two lines
              are merged into a syntax error.

              Args:
                  path: Absolute path to file (must be a regular file).
                  old_text: Unique text to find and replace (must occur exactly once, unless replace_all).
                  new_text: replacement text.
                  exact: If False (default), whitespace in old_text is matched tolerantly.
                         If True, whitespace must match exactly.
                  replace_all: If True, replace every occurrence of old_text instead of requiring
                               a single unique match.

              Returns:
                  EditBlockResult with success status.

              Raises:
                  EditBlockError: If path is not absolute, not found, or not a regular file.
                  EditBlockError: If old_text not found, or appears more than once in file (when
                                  replace_all is False).
                  EditBlockError: If write operation fails.
              """
              file_path = Path(path)
              if not file_path.is_absolute():
                  raise EditBlockError('Path must be absolute.')
              if not file_path.exists():
                  raise EditBlockError('File not found.')
              if not file_path.is_file():
                  raise EditBlockError('Not a regular file.')
              text = file_path.read_text(encoding='utf-8')
              try:
                  result_text = replace_in_block(
                      text,
                      old_text,
                      new_text,
                      exact=exact,
                      replace_all=replace_all,
                      accept=line_preserving(old_text),
                      max_level=2,
                      where='file')
              except TextMatchError as exc:
                  raise EditBlockError(str(exc)) from exc
              try:
                  file_path.write_text(result_text, encoding='utf-8')
              except OSError as exc:
                  raise EditBlockError(f'Write failed: {exc}') from exc
              return EditBlockResult(result='success')
      - id: EditBlockTool
        type: ClassDef
        signature: "class EditBlockTool(ToolDefinition):"
        children:
        - id: EditBlockTool.cUwq7H
          type: statements
          code: |-
            name = 'edit_block'
            title = 'Replace text in file'
            description = "Replace a short text inside an file. 'old_text' must occur exactly once, unless 'replaceAll' is set. By default whitespace (spaces, tabs, newlines) is matched tolerantly; set 'exact' to require exact whitespace matching."
        - id: EditBlockTool.c52fkG
          type: statements
          code: |-
            input_schema = {
                'type': 'object',
                'properties': {
                    'path': {
                        'type': 'string',
                        'description': 'Absolute path to the target file.'},
                    'old_text': {
                        'type': 'string',
                        'minLength': 10,
                        'maxLength': 100,
                        'description': 'Text (10-100 chars) to find and replace. Must occur exactly once, unless replaceAll is set.'},
                    'new_text': {
                        'type': 'string',
                                'description': "Text that replace 'old_text (empty to perform a deletion)'."},
                    'exact': {
                        'type': 'boolean',
                        'description': "If true, 'old_text' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                        'default': False},
                    'replaceAll': {
                        'type': 'boolean',
                        'description': "If true, replace every occurrence of 'old_text' instead of requiring a single unique match. Defaults to false.",
                        'default': False}},
                'required': [
                    'path',
                    'old_text',
                    'new_text']}
        - id: EditBlockTool.ZqxRbM
          type: statements
          code: |-
            output_schema = {'type': 'object', 'properties': {'result': {'type': 'string'}}, 'required': []}
            annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}
        - id: EditBlockTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`edit_block`, translating the MCP schema to/from the Python API."""
                args: dict[str, Any] = ctx.arguments
                try:
                    result = edit_block(
                        path=args['path'],
                        old_text=args['old_text'],
                        new_text=args['new_text'],
                        exact=args.get(
                            'exact',
                            False),
                        replace_all=args.get(
                            'replaceAll',
                            False))
                except EditBlockError as exc:
                    return ToolResult(content=[text_content(str(exc))], is_error=True)
                return ToolResult(structured_content={'result': result.result}, auto_approve=True)
      - id: register_edit_block_tool
        type: FunctionDef
        code: |-
          def register_edit_block_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
              registry.register(EditBlockTool())
              functions.register(edit_block)
      errors: []
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py
      nodes:
      - id: 2oh2po
        type: statements
        code: '''Edit Marks tool – replaces the text strictly including two markers.'''
      - id: 0kGWYF
        type: imports
        code: |-
          from dataclasses import dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools._text_match import replace_between, marks_line_preserving, TextMatchError
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: kp2FTL
        type: statements
        code: |-
          __all__ = [
              'EditMarksError',
              'EditMarksResult',
              'edit_marks',
              'edit_marks_text',
              'EditMarksTool',
              'register_edit_mark_tool']
      - id: EditMarksError
        type: ClassDef
        code: |-
          class EditMarksError(Exception):
              """Raised when a replace operation cannot be performed."""
      - id: EditMarksResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditMarksResult:
              result: str
      - id: edit_marks_text
        type: FunctionDef
        code: |-
          def edit_marks_text(text: str, begin_marker: str, content: str, end_marker: str, exact: bool=False) -> str:
              """Replace everything between and including 'begin_marker' and 'end_marker' with content, in *text*.

              Both markers are included in the replacement. Matching escalates through
              whitespace/escape/quote tolerance; each marker match must keep its own line
              count, so tolerant matching never merges lines into a syntax error.

              Args:
                  text: Source text to edit.
                  begin_marker: Unique substring marking the beginning of the block.
                  content: Replacement text.
                  end_marker: Unique substring marking the end of the block.
                  exact: If False (default), whitespace in start/end is matched tolerantly. If True, whitespace must match exactly.

              Returns:
                  The edited text.

              Raises:
                  EditMarksError: If start or end markers are not found or appear more than once.
                  EditMarksError: If end marker does not start after start marker ends.
              """
              begin, end = (begin_marker, end_marker) if exact else (begin_marker.strip(), end_marker.strip())
              try:
                  return replace_between(
                      text,
                      begin,
                      end,
                      content,
                      exact=exact,
                      accept=marks_line_preserving(
                          begin,
                          end),
                      max_level=2,
                      where='file')
              except TextMatchError as exc:
                  raise EditMarksError(str(exc)) from exc
      - id: edit_marks
        type: FunctionDef
        code: |-
          def edit_marks(path: str, begin_marker: str, end_marker: str, content: str, exact: bool=False) -> EditMarksResult:
              """Replace everything between and including 'start' and 'end' with content.

              Both markers are included in the replacement.

              Args:
                  path: Absolute path to target file.
                  start: Unique substring marking the beginning of the block.
                  end: Unique substring marking the end of the block.
                  content: Replacement text.
                  exact: If False (default), whitespace in start/end is matched tolerantly. If True, whitespace must match exactly.

              Returns:
                  EditMarksResult with success status.

              Raises:
                  EditMarksError: If path is not absolute, not found, or not a regular file.
                  EditMarksError: If start or end markers are not found or appear more than once.
                  EditMarksError: If end marker does not start after start marker ends.
              """
              file_path = Path(path)
              if not file_path.is_absolute():
                  raise EditMarksError('Path must be absolute.')
              if not file_path.exists():
                  raise EditMarksError('File not found.')
              if not file_path.is_file():
                  raise EditMarksError('Not a regular file.')
              text = file_path.read_text(encoding='utf-8')
              result_text = edit_marks_text(text, begin_marker, content, end_marker, exact=exact)
              try:
                  file_path.write_text(result_text, encoding='utf-8')
              except OSError as exc:
                  raise EditMarksError(f'Write failed: {exc}') from exc
              return EditMarksResult(result='success')
      - id: EditMarksTool
        type: ClassDef
        signature: "class EditMarksTool(ToolDefinition):"
        children:
        - id: EditMarksTool.3AO8Cb
          type: statements
          code: |-
            name = 'edit_marks'
            title = 'Replace text between two marks'
            description = "Replace everything between and including the unique 'start_marker' and 'end_marker' markers with new 'content'."
        - id: EditMarksTool.gjnjFR
          type: statements
          code: |-
            input_schema = {
                'type': 'object',
                'strict': True,
                'additionalProperties': False,
                'properties': {
                    'path': {
                        'type': 'string',
                        'description': 'Absolute path to the target file.'},
                    'begin_marker': {
                        'type': 'string',
                        'minLength': 10,
                        'maxLength': 30,
                        'description': 'Unique 10-30 char substring marking the beginning of the text to replace.'},
                    'content': {
                        'type': 'string',
                                'description': 'Replacement source for the marked text.'},
                    'end_marker': {
                        'type': 'string',
                        'minLength': 10,
                        'maxLength': 30,
                        'description': 'Unique 10-30 char substring marking the end of the text to replace'},
                    'exact': {
                        'type': 'boolean',
                        'description': "If true, 'begin_marker'/'end_marker' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                        'default': False}},
                'required': [
                    'path',
                    'begin_marker',
                    'end_marker',
                    'content']}
        - id: EditMarksTool.buscHT
          type: statements
          code: |-
            output_schema = {
                'type': 'object',
                'properties': {
                    'result': {
                        'type': 'string',
                        'description': '``success`` on success.'}},
                'required': []}
            annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}
        - id: EditMarksTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`edit_marks`, translating the MCP schema to/from the Python API."""
                args: dict[str, Any] = ctx.arguments
                try:
                    result = edit_marks(
                        path=args['path'],
                        begin_marker=args['begin_marker'],
                        end_marker=args['end_marker'],
                        content=args['content'],
                        exact=args.get(
                            'exact',
                            False))
                except EditMarksError as exc:
                    return ToolResult(content=[text_content(str(exc))], is_error=True)
                return ToolResult(structured_content={'result': result.result}, auto_approve=True)
      - id: register_edit_marks_tool
        type: FunctionDef
        code: |-
          def register_edit_marks_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
              registry.register(EditMarksTool())
              functions.register(edit_marks)
      errors: []
    errors: []
```
/answer 95d8-3f allow
Control Request:
```yaml
id: 95d8-40
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_line/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_lines/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
  reason: Struktur der weiteren Tools erfassen
```
/answer 95d8-40 allow
Control Request:
```yaml
id: 95d8-41
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_line/__init__.py
      nodes:
      - id: iWnq9o
        type: statements
      - id: 7EDQM6
        type: imports
      - id: yVGFJf
        type: statements
      - id: EditLineError
        type: ClassDef
        signature: "class EditLineError(Exception):"
        docstring: Raised when an edit-line operation cannot be performed.
      - id: EditLineResult
        type: ClassDef
        signature: "@dataclass(frozen=True) class EditLineResult:"
      - id: edit_line
        type: FunctionDef
        signature: "def edit_line(path: str, old_line: str, new_lines: str, exact: bool=False, replace_all: bool=False) -> EditLineResult:"
        docstring: Replace a single line ``old_line`` in the file at ``path`` with ``new_lines``. …
      - id: EditLineTool
        type: ClassDef
        signature: "class EditLineTool(ToolDefinition):"
        children:
        - id: EditLineTool.w15KyL
          type: statements
        - id: EditLineTool.uJqnih
          type: statements
        - id: EditLineTool.ZqxRbM
          type: statements
        - id: EditLineTool.handle
          type: FunctionDef
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`edit_line`, translating the MCP schema to/from the Python AP…"
      - id: register_edit_line_tool
        type: FunctionDef
        signature: "def register_edit_line_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_lines/__init__.py
      nodes:
      - id: MkL6Sx
        type: statements
      - id: WIeTDO
        type: imports
      - id: r4ImzN
        type: statements
      - id: EditLinesError
        type: ClassDef
        signature: "class EditLinesError(Exception):"
        docstring: Raised when a edit-lines operation cannot be performed.
      - id: EditLinesResult
        type: ClassDef
        signature: "@dataclass(frozen=True) class EditLinesResult:"
      - id: edit_lines
        type: FunctionDef
        signature: "def edit_lines(path: str, offset: int, amount: int, content: str) -> EditLinesResult:"
        docstring: Replace ``amount`` lines starting at line ``offset`` in the file at ``path`` wi…
      - id: EditLinesTool
        type: ClassDef
        signature: "class EditLinesTool(ToolDefinition):"
        children:
        - id: EditLinesTool.tEvV2o
          type: statements
        - id: EditLinesTool.jo1Gt5
          type: statements
        - id: EditLinesTool.8Dn9GB
          type: statements
        - id: EditLinesTool.handle
          type: FunctionDef
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`edit_lines`, translating the MCP schema to/from the Python A…"
      - id: register_edit_lines_tool
        type: FunctionDef
        signature: "def register_edit_lines_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
      nodes:
      - id: h86WCz
        type: statements
      - id: E1DzXA
        type: imports
      - id: 2gnNKJ
        type: statements
      - id: ReadError
        type: ClassDef
        signature: "class ReadError(Exception):"
        docstring: Raised when a file cannot be read or the requested range is invalid.
      - id: ReadResult
        type: ClassDef
        signature: "@dataclass(frozen=True) class ReadResult:"
      - id: cache_key
        type: FunctionDef
        signature: "def _cache_key(session_id: str, arguments: dict[str, Any]) -> str:"
      - id: read_file
        type: FunctionDef
        signature: "def read_file(path: str, min_line: int | None=None, max_line: int | None=None, min_char: int | None=None, max_char: int | None=None, start: str | None=None, end: str | None=None) -> ReadResult:"
        docstring: "Read file contents, optionally sliced to a range. Args: path: Absolute path to …"
      - id: ReadTool
        type: ClassDef
        signature: "class ReadTool(ToolDefinition):"
        children:
        - id: ReadTool.DXCaGL
          type: statements
        - id: ReadTool.QchUS6
          type: statements
        - id: ReadTool.PpiarV
          type: statements
        - id: ReadTool.handle
          type: FunctionDef
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`read_file`, then apply session-level change detection and MC…"
      - id: register_read_tool
        type: FunctionDef
        signature: "def register_read_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
      nodes:
      - id: TNPCm6
        type: statements
      - id: 7mldao
        type: imports
      - id: yELdoc
        type: statements
      - id: Skill
        type: ClassDef
        signature: "class Skill(ToolDefinition, ABC):"
        docstring: "Base class for skills: input-less tools returning maintainable hints."
        children:
        - id: Skill.Ofdi2O
          type: statements
        - id: Skill.init
          type: FunctionDef
          signature: "def __init__(self) -> None:"
        - id: Skill.handle
          type: FunctionDef
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: Call ``core`` and wrap its result into the MCP output schema.
        - id: Skill.core
          type: FunctionDef
          signature: "@staticmethod @abstractmethod def core() -> str:"
          docstring: Real function backing this skill; also registered in the ``FunctionRegistry``.
      - id: markdown_format
        type: FunctionDef
        signature: "def markdown_format() -> str:"
        docstring: Preferred formatting rules for Pandoc-compatible Markdown documents. Load when …
      - id: MarkdownFormatSkill
        type: ClassDef
        signature: "class MarkdownFormatSkill(Skill):"
      - id: tSwo7s
        type: statements
      - id: register_skills
        type: FunctionDef
        signature: "def register_skills(registry: ToolRegistry, environment: AppEnvironment) -> None:"
        docstring: Register every skill in *skills* (defaults to :data:`SKILLS`) in both registrie…
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
      nodes:
      - id: 8kT6Ro
        type: statements
      - id: WIeTDO
        type: imports
      - id: lYkglf
        type: statements
      - id: WriteError
        type: ClassDef
        signature: "class WriteError(Exception):"
        docstring: Raised when a write operation cannot be performed.
      - id: WriteResult
        type: ClassDef
        signature: "@dataclass(frozen=True) class WriteResult:"
      - id: write
        type: FunctionDef
        signature: "def write(path: str, mode: str, content: str) -> WriteResult:"
        docstring: "Write ``content`` to ``path``; ``mode`` is ``replace`` or ``append``. Args: pat…"
      - id: WriteTool
        type: ClassDef
        signature: "class WriteTool(ToolDefinition):"
        children:
        - id: WriteTool.SXDPme
          type: statements
        - id: WriteTool.8dPhMv
          type: statements
        - id: WriteTool.8Dn9GB
          type: statements
        - id: WriteTool.handle
          type: FunctionDef
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`write`, translating the MCP schema to/from the Python API."
      - id: register_write_tool
        type: FunctionDef
        signature: "def register_write_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    errors: []
```
/answer 95d8-41 allow
Control Request:
```yaml
id: 95d8-42
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_line/__init__.py
    ids:
    - iWnq9o
    - 7EDQM6
    - yVGFJf
    - EditLineError
    - EditLineResult
    - edit_line
    - EditLineTool
    - register_edit_line_tool
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_lines/__init__.py
    ids:
    - MkL6Sx
    - WIeTDO
    - r4ImzN
    - EditLinesError
    - EditLinesResult
    - edit_lines
    - EditLinesTool
    - register_edit_lines_tool
  reason: Vollständigen Code der edit_line und edit_lines Tools lesen
```
/answer 95d8-42 allow
ReasoningToken: 308
Control Request:
```yaml
id: 95d8-43
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_line/__init__.py
      nodes:
      - id: iWnq9o
        type: statements
        code: '''Edit-line tool – Replaces a single line with one or more lines in a file.\n'''
      - id: 7EDQM6
        type: imports
        code: |-
          from dataclasses import dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.edit_block import EditBlockError, edit_block
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: yVGFJf
        type: statements
        code: "__all__ = ['EditLineError', 'EditLineResult', 'edit_line', 'EditLineTool', 'register_edit_line_tool']"
      - id: EditLineError
        type: ClassDef
        code: |-
          class EditLineError(Exception):
              """Raised when an edit-line operation cannot be performed."""
      - id: EditLineResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditLineResult:
              result: str
      - id: edit_line
        type: FunctionDef
        code: |-
          def edit_line(path: str, old_line: str, new_lines: str, exact: bool=False, replace_all: bool=False) -> EditLineResult:
              """Replace a single line ``old_line`` in the file at ``path`` with ``new_lines``.

              Delegates to :func:`edit_block`; ``old_line`` must be a single line (no newline
              characters). ``new_lines`` may be a single line or multiple lines joined by ``\\n``.

              Args:
                  path: Absolute path to file (must be a regular file).
                  old_line: The single line to find and replace (must occur exactly once,
                            unless replace_all). Must not contain a newline character.
                  new_lines: Replacement content; either a single line or multiple lines
                             (joined with '\\n') to replace old_line with.
                  exact: If False (default), whitespace in old_line is matched tolerantly.
                         If True, whitespace must match exactly.
                  replace_all: If True, replace every occurrence of old_line instead of
                               requiring a single unique match.

              Returns:
                  EditLineResult with success status.

              Raises:
                  EditLineError: If old_line contains a newline character.
                  EditLineError: Wraps any EditBlockError raised while delegating.
              """
              if '\n' in old_line or '\r' in old_line:
                  raise EditLineError('old_line must be a single line without newline characters.')
              try:
                  result = edit_block(path=path, old_text=old_line, new_text=new_lines, exact=exact, replace_all=replace_all)
              except EditBlockError as exc:
                  raise EditLineError(str(exc)) from exc
              return EditLineResult(result=result.result)
      - id: EditLineTool
        type: ClassDef
        signature: "class EditLineTool(ToolDefinition):"
        children:
        - id: EditLineTool.w15KyL
          type: statements
          code: |-
            name = 'edit_line'
            title = 'Replace a single line in file'
            description = "Replace exactly one line inside an file with one or more lines. 'old_line' must be a single line without a newline character. Choose 'old_line' to be unique within the file. 'old_line' must occur exactly once, unless 'replaceAll' is set. By default whitespace is matched tolerantly; set 'exact' to require exact whitespace matching."
        - id: EditLineTool.uJqnih
          type: statements
          code: |-
            input_schema = {
                'type': 'object',
                'properties': {
                    'path': {
                        'type': 'string',
                        'description': 'Absolute path to the target file.'},
                    'old_line': {
                        'type': 'string',
                        'description': 'A single line to find and replace without a newline character. Keep it short and distinct enough to match uniquely. Must occur exactly once unless replaceAll is set. Never pass multiple lines here.'},
                    'new_lines': {
                        'type': 'string',
                                'description': "Replacement content for 'old_line', may be empty to delete the line."},
                    'exact': {
                        'type': 'boolean',
                        'description': "If true, 'old_line' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                        'default': False},
                    'replaceAll': {
                        'type': 'boolean',
                        'description': "If true, replace every occurrence of 'old_line' instead of requiring a single unique match. Defaults to false.",
                        'default': False}},
                'required': [
                    'path',
                    'old_line',
                    'new_lines']}
        - id: EditLineTool.ZqxRbM
          type: statements
          code: |-
            output_schema = {'type': 'object', 'properties': {'result': {'type': 'string'}}, 'required': []}
            annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}
        - id: EditLineTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`edit_line`, translating the MCP schema to/from the Python API."""
                args: dict[str, Any] = ctx.arguments
                try:
                    result = edit_line(
                        path=args['path'],
                        old_line=args['old_line'],
                        new_lines=args['new_lines'],
                        exact=args.get(
                            'exact',
                            False),
                        replace_all=args.get(
                            'replaceAll',
                            False))
                except EditLineError as exc:
                    return ToolResult(content=[text_content(str(exc))], is_error=True)
                return ToolResult(structured_content={'result': result.result}, auto_approve=True)
      - id: register_edit_line_tool
        type: FunctionDef
        code: |-
          def register_edit_line_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
              registry.register(EditLineTool())
              functions.register(edit_line)
      errors: []
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_lines/__init__.py
      nodes:
      - id: MkL6Sx
        type: statements
        code: '''Edit-lines tool – replaces a range of lines inside an existing file.\n'''
      - id: WIeTDO
        type: imports
        code: |-
          from dataclasses import dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: r4ImzN
        type: statements
        code: "__all__ = ['EditLinesError', 'EditLinesResult', 'edit_lines', 'EditLinesTool', 'register_edit_lines_tool']"
      - id: EditLinesError
        type: ClassDef
        code: |-
          class EditLinesError(Exception):
              """Raised when a edit-lines operation cannot be performed."""
      - id: EditLinesResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditLinesResult:
              result: str
      - id: edit_lines
        type: FunctionDef
        code: |-
          def edit_lines(path: str, offset: int, amount: int, content: str) -> EditLinesResult:
              """Replace ``amount`` lines starting at line ``offset`` in the file at ``path`` with ``content``.

              Args:
                  path: Absolute path to file (must be a regular file).
                  offset: Zero-based line offset where to start replacement (must be >= 0).
                  length: Number of lines to replace (must be >= 0).
                  content: Replacement text (should include its own trailing newline if a line break is wanted).

              Returns:
                  EditLinesResult with success status.

              Raises:
                  EditLinesError: If path is not absolute, not found, or not a regular file.
                  EditLinesError: If offset or length are out of bounds.
                  EditLinesError: If write operation fails.

              Note:
                  Lines are 0-based. content may be empty to perform pure deletion.
              """
              file_path = Path(path)
              if not file_path.is_absolute():
                  raise EditLinesError('Path must be absolute.')
              if not file_path.exists():
                  raise EditLinesError('File not found.')
              if not file_path.is_file():
                  raise EditLinesError('Not a regular file.')
              try:
                  text = file_path.read_text(encoding='utf-8')
                  lines = text.splitlines(keepends=True)
                  if offset < 0 or offset > len(lines):
                      raise EditLinesError('Offset is out of bounds.')
                  if amount < 0 or offset + amount > len(lines):
                      raise EditLinesError('Length is out of bounds.')
                  new_lines = lines[:offset] + [content] + lines[offset + amount:]
                  new_text = ''.join(new_lines)
                  file_path.write_text(new_text, encoding='utf-8')
              except OSError as exc:
                  raise EditLinesError(f'Replace failed: {exc}') from exc
              return EditLinesResult(result='success')
      - id: EditLinesTool
        type: ClassDef
        signature: "class EditLinesTool(ToolDefinition):"
        children:
        - id: EditLinesTool.tEvV2o
          type: statements
          code: |-
            name = 'edit_lines'
            title = 'Replace lines in file by line offsets'
            description = 'Replace a range of lines inside an existing file with new content. The range is defined by a zero-based line ``offset`` and a ``length`` (number of lines to remove starting at the offset). The supplied ``content`` is written in place of the removed lines; it should include its own trailing newline if a line break is wanted.'
        - id: EditLinesTool.jo1Gt5
          type: statements
          code: |-
            input_schema = {
                'type': 'object',
                'properties': {
                    'path': {
                        'type': 'string',
                        'description': 'Absolute path to the file to modify.'},
                    'offset': {
                        'type': 'integer',
                        'description': 'Zero-based line offset of the first line to replace.',
                        'minimum': 0},
                    'amount': {
                        'type': 'integer',
                                'description': 'Number of lines to remove starting at ``offset``.',
                                'minimum': 0},
                    'content': {
                        'type': 'string',
                        'description': 'Replacement text (may be empty to perform a pure deletion).'}},
                'required': [
                    'path',
                    'offset',
                    'length',
                    'content']}
        - id: EditLinesTool.8Dn9GB
          type: statements
          code: |-
            output_schema = {
                'type': 'object',
                'properties': {
                    'result': {
                        'type': 'string',
                        'description': '``success`` on success.'}},
                'required': ['result']}
            annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}
        - id: EditLinesTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`edit_lines`, translating the MCP schema to/from the Python API."""
                args: dict[str, Any] = ctx.arguments
                try:
                    result = edit_lines(path=args['path'], offset=args['offset'], amount=args['amount'], content=args['content'])
                except EditLinesError as exc:
                    return ToolResult(content=[text_content(str(exc))], is_error=True)
                return ToolResult(structured_content={'result': result.result}, auto_approve=True)
      - id: register_edit_lines_tool
        type: FunctionDef
        code: |-
          def register_edit_lines_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
              registry.register(EditLinesTool())
              functions.register(edit_lines)
      errors: []
    errors: []
```
/answer 95d8-43 allow
Control Request:
```yaml
id: 95d8-44
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
    ids:
    - h86WCz
    - E1DzXA
    - 2gnNKJ
    - ReadError
    - ReadResult
    - cache_key
    - read_file
    - ReadTool
    - register_read_tool
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
    ids:
    - 8kT6Ro
    - WIeTDO
    - lYkglf
    - WriteError
    - WriteResult
    - write
    - WriteTool
    - register_write_tool
  reason: Vollständigen Code der read und write Tools lesen
```
/answer 95d8-44 allow
Control Request:
```yaml
id: 95d8-45
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
      nodes:
      - id: h86WCz
        type: statements
        code: "'Read tool – returns file contents, optionally sliced by line, character offset, or marker.\\n'"
      - id: E1DzXA
        type: imports
        code: |-
          import hashlib
          import json
          from dataclasses import dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: 2gnNKJ
        type: statements
        code: |-
          __all__ = ['ReadError', 'ReadResult', 'read_file', 'ReadTool', 'register_read_tool']
          _CACHE_STATE_KEY = '_read_cache'
      - id: ReadError
        type: ClassDef
        code: |-
          class ReadError(Exception):
              """Raised when a file cannot be read or the requested range is invalid."""
      - id: ReadResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReadResult:
              content: str
              checksum: str
              is_full_file: bool
      - id: cache_key
        type: FunctionDef
        code: |-
          def _cache_key(session_id: str, arguments: dict[str, Any]) -> str:
              payload = json.dumps({'session': session_id, 'arguments': arguments}, sort_keys=True)
              return hashlib.sha256(payload.encode('utf-8')).hexdigest()
      - id: read_file
        type: FunctionDef
        code: |-
          def read_file(path: str, min_line: int | None=None, max_line: int | None=None, min_char: int | None=None, max_char: int | None=None, start: str | None=None, end: str | None=None) -> ReadResult:
              """Read file contents, optionally sliced to a range.

              Args:
                  path: Absolute path to file (must exist and be readable).
                  min_line: Range start as line number, inclusive, 1-based. Mutually exclusive with
                            min_char and start.
                  max_line: Range end as line number, inclusive, 1-based. Mutually exclusive with
                            max_char and end.
                  min_char: Range start as character offset, inclusive, 0-based. Mutually exclusive with
                            min_line and start.
                  max_char: Range end as character offset, exclusive, 0-based. Mutually exclusive with
                            max_line and end.
                  start: Range start as unique substring marker (inclusive). Mutually exclusive with
                         min_line and min_char. Marker must occur exactly once in file.
                  end: Range end as unique substring marker (inclusive). Mutually exclusive with
                       max_line and max_char. Marker must occur exactly once in file.

              Returns:
                  ReadResult with:
                      content: The file content or requested slice (UTF-8 decoded, errors replaced).
                      checksum: SHA256 checksum of the content.
                      is_full_file: True if entire file was read (no range specified).

              Raises:
                  ReadError: If path is not absolute, not found, or not a regular file.
                  ReadError: If conflicting range parameters provided (e.g., min_line AND min_char).
                  ReadError: If start/end markers not found or appear more than once.
                  ReadError: If end position resolves before start position.

              Note:
                  Line numbering is 1-based (first line is 1). Character offsets are 0-based.
                  Session-level change detection via cache: identical reads return checksum only.
              """
              if min_line is not None and min_char is not None:
                  raise ReadError('``min_line`` and ``min_char`` are mutually exclusive.')
              if max_line is not None and max_char is not None:
                  raise ReadError('``max_line`` and ``max_char`` are mutually exclusive.')
              if min_line is not None and start is not None:
                  raise ReadError('``min_line`` and ``start`` are mutually exclusive.')
              if min_char is not None and start is not None:
                  raise ReadError('``min_char`` and ``start`` are mutually exclusive.')
              if max_line is not None and end is not None:
                  raise ReadError('``max_line`` and ``end`` are mutually exclusive.')
              if max_char is not None and end is not None:
                  raise ReadError('``max_char`` and ``end`` are mutually exclusive.')
              file_path = Path(path)
              if not file_path.is_absolute():
                  raise ReadError('Path must be absolute.')
              if not file_path.exists():
                  raise ReadError('File not found.')
              if not file_path.is_file():
                  raise ReadError("Not a regular file. Don't read directories with this tool!")
              raw_bytes = file_path.read_bytes()
              text = raw_bytes.decode('utf-8', errors='replace')
              lines = text.splitlines(keepends=True)
              total_lines = len(lines)

              def line_start_offset(line_num: int) -> int:
                  n = max(0, min(line_num - 1, total_lines))
                  return sum((len(l) for l in lines[:n]))

              def line_end_offset(line_num: int) -> int:
                  n = max(0, min(line_num, total_lines))
                  return sum((len(l) for l in lines[:n]))
              if start is not None:
                  start_count = text.count(start)
                  if start_count == 0:
                      raise ReadError('Start marker not found in file.')
                  if start_count > 1:
                      raise ReadError(f'Start marker is ambiguous – found {start_count} occurrences in file.')
                  region_start = text.index(start)
              elif min_line is not None:
                  region_start = line_start_offset(min_line)
              elif min_char is not None:
                  region_start = min_char
              else:
                  region_start = 0
              if end is not None:
                  end_count = text.count(end)
                  if end_count == 0:
                      raise ReadError('End marker not found in file.')
                  if end_count > 1:
                      raise ReadError(f'End marker is ambiguous – found {end_count} occurrences in file.')
                  region_end = text.index(end) + len(end)
              elif max_line is not None:
                  region_end = line_end_offset(max_line)
              elif max_char is not None:
                  region_end = max_char
              else:
                  region_end = len(text)
              if region_end < region_start:
                  raise ReadError('Resolved end position must not lie before the resolved start position.')
              is_full_file = min_line is None and max_line is None and (
                  min_char is None) and (
                      max_char is None) and (
                          start is None) and (
                              end is None)
              if not is_full_file and len(text) and (region_end - region_start > 0.7 * len(text)):
                  raise ReadError('The requested range selects more than 70% of the file. Read the whole file instead (omit the range parameters) and rely on the checksum-based conditional read to detect unchanged content.')
              sliced = text[region_start:region_end]
              checksum = hashlib.sha256(sliced.encode('utf-8')).hexdigest()
              return ReadResult(content=sliced, checksum=checksum, is_full_file=is_full_file)
      - id: ReadTool
        type: ClassDef
        signature: "class ReadTool(ToolDefinition):"
        children:
        - id: ReadTool.DXCaGL
          type: statements
          code: |-
            name = 'read_file'
            title = 'Read file content'
            description = 'Read a file as text, optionally sliced to a range.'
        - id: ReadTool.QchUS6
          type: statements
          code: |-
            input_schema = {
                'type': 'object',
                'properties': {
                    'path': {
                        'type': 'string',
                        'description': 'Absolute file path.'},
                    'min_line': {
                        'type': 'integer',
                        'description': 'Range start: line number, inclusive, 1-based. Excludes start and min_char.',
                        'minimum': 1},
                    'max_line': {
                        'type': 'integer',
                                'description': 'Range end: line number, inclusive, 1-based. Excludes end and max_char.',
                                'minimum': 1},
                    'min_char': {
                        'type': 'integer',
                        'description': 'Range start: character offset, inclusive, 0-based. Excludes min_line.',
                        'minimum': 0},
                    'max_char': {
                        'type': 'integer',
                        'description': 'Range end: character offset, exclusive, 0-based. Excludes max_line.',
                        'minimum': 0},
                    'start': {
                        'type': 'string',
                        'description': 'Range start: unique marker substring, inclusive. Excludes min_line and min_char.'},
                    'end': {
                        'type': 'string',
                        'description': 'Range end: unique marker substring, inclusive. Excludes max_line and max_char.'}},
                'required': ['path']}
        - id: ReadTool.PpiarV
          type: statements
          code: |-
            output_schema = {
                'type': 'object',
                'properties': {
                    'content': {
                        'type': 'string'},
                    'checksum': {
                        'type': 'string',
                        'description': 'sha256 checksum of the read content.'},
                    'unchanged': {
                        'type': 'boolean',
                                'description': 'True if the content is identical to a previous read with the same parameters'}},
                'required': ['checksum']}
            annotations = {'readOnlyHint': True, 'openWorldHint': False}
        - id: ReadTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`read_file`, then apply session-level change detection and MCP packing."""
                args: dict[str, Any] = ctx.arguments
                try:
                    result = read_file(
                        path=args['path'],
                        min_line=args.get('min_line'),
                        max_line=args.get('max_line'),
                        min_char=args.get('min_char'),
                        max_char=args.get('max_char'),
                        start=args.get('start'),
                        end=args.get('end'))
                except ReadError as exc:
                    return ToolResult(content=[text_content(str(exc))], is_error=True)
                session = ctx.session
                key = _cache_key(session.id, args)
                with session.lock:
                    cache: dict[str, str] = session.state.setdefault(_CACHE_STATE_KEY, {})
                    previous_checksum = cache.get(key)
                    cache[key] = result.checksum
                unchanged = previous_checksum == result.checksum
                structured: dict[str, Any] = {'checksum': result.checksum}
                if unchanged:
                    structured['unchanged'] = True
                else:
                    structured['content'] = result.content
                content: list[dict[str, Any]] = []
                if unchanged:
                    content.append(text_content('Content unchanged since the last identical read. Use the former read result.'))
                return ToolResult(content=content, structured_content=structured, auto_approve=result.is_full_file)
      - id: register_read_tool
        type: FunctionDef
        code: |-
          def register_read_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
              registry.register(ReadTool())
              functions.register(read_file)
      errors: []
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
      nodes:
      - id: 8kT6Ro
        type: statements
        code: '''Write tool – writes a file completely or appends lines to it.'''
      - id: WIeTDO
        type: imports
        code: |-
          from dataclasses import dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: lYkglf
        type: statements
        code: "__all__ = ['WriteError', 'WriteResult', 'write', 'WriteTool', 'register_write_tool']"
      - id: WriteError
        type: ClassDef
        code: |-
          class WriteError(Exception):
              """Raised when a write operation cannot be performed."""
      - id: WriteResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class WriteResult:
              result: str
      - id: write
        type: FunctionDef
        code: |-
          def write(path: str, mode: str, content: str) -> WriteResult:
              """Write ``content`` to ``path``; ``mode`` is ``replace`` or ``append``.

              Args:
                  path: Absolute path to file to write (created if not exists).
                  mode: Write mode: "replace" overwrites entire file, "append" adds content at end.
                  content: Text content to write.

              Returns:
                  WriteResult with success status.

              Raises:
                  WriteError: If path is not absolute.
                  WriteError: If write operation fails (permission, disk full, etc.).
              """
              file_path = Path(path)
              if not file_path.is_absolute():
                  raise WriteError('Path must be absolute.')
              try:
                  file_path.parent.mkdir(parents=True, exist_ok=True)
                  file_mode = 'a' if mode == 'append' else 'w'
                  with file_path.open(file_mode, encoding='utf-8') as fh:
                      fh.write(content)
              except OSError as exc:
                  raise WriteError(f'Write failed: {exc}') from exc
              return WriteResult(result='success')
      - id: WriteTool
        type: ClassDef
        signature: "class WriteTool(ToolDefinition):"
        children:
        - id: WriteTool.SXDPme
          type: statements
          code: |-
            name = 'write'
            title = 'Write file'
            description = 'Write content to a file. In ``replace`` mode the file is overwritten with the supplied content. In ``append`` mode the content is added at the end of the existing file (the file is created if it does not yet exist).'
        - id: WriteTool.8dPhMv
          type: statements
          code: |-
            input_schema = {
                'type': 'object',
                'properties': {
                    'path': {
                        'type': 'string',
                        'description': 'Absolute path to the file to write.'},
                    'mode': {
                        'type': 'string',
                        'enum': [
                                'replace',
                                'append'],
                        'description': '``replace`` – overwrite the file with the new content. ``append`` – add the new content after the existing content.'},
                    'content': {
                        'type': 'string',
                        'description': 'Text to write to the file.'}},
                'required': [
                    'path',
                    'mode',
                    'content']}
        - id: WriteTool.8Dn9GB
          type: statements
          code: |-
            output_schema = {
                'type': 'object',
                'properties': {
                    'result': {
                        'type': 'string',
                        'description': '``success`` on success.'}},
                'required': ['result']}
            annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}
        - id: WriteTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`write`, translating the MCP schema to/from the Python API."""
                args: dict[str, Any] = ctx.arguments
                try:
                    result = write(path=args['path'], mode=args['mode'], content=args['content'])
                except WriteError as exc:
                    return ToolResult(content=[text_content(str(exc))], is_error=True)
                return ToolResult(structured_content={'result': result.result}, auto_approve=True)
      - id: register_write_tool
        type: FunctionDef
        code: |-
          def register_write_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
              registry.register(WriteTool())
              functions.register(write)
      errors: []
    errors: []
```
/answer 95d8-45 allow
Control Request:
```yaml
id: 95d8-46
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
    ids:
    - TNPCm6
    - 7mldao
    - yELdoc
    - Skill
    - markdown_format
    - MarkdownFormatSkill
    - tSwo7s
    - register_skills
  reason: Vollständigen Code des skills Tools lesen
```
/answer 95d8-46 allow
Control Request:
```yaml
id: 95d8-47
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
      nodes:
      - id: TNPCm6
        type: statements
        code: "'Skills – on-demand hints an agent can request.\\n\\nA *skill* is a small, easy-to-maintain unit combining an MCP tool schema\\n(name, title, description, hint) with a real, named module-level function\\nthat backs it as ``core`` (``FunctionRegistry`` requires actually existing\\nfunctions with a real signature/docstring.\\n'"
      - id: 7mldao
        type: imports
        code: |-
          from abc import ABC, abstractmethod
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: yELdoc
        type: statements
        code: "__all__ = ['Skill', 'SKILLS', 'MarkdownFormatSkill', 'register_skills', 'markdown_format']"
      - id: Skill
        type: ClassDef
        signature: "class Skill(ToolDefinition, ABC):"
        docstring: "Base class for skills: input-less tools returning maintainable hints."
        children:
        - id: Skill.Ofdi2O
          type: statements
          code: |-
            'Base class for skills: input-less tools returning maintainable hints.'
            hint: str
            input_schema = {'type': 'object', 'properties': {}}
            output_schema = {'type': 'object', 'properties': {'instructions': {'type': 'string'}}, 'required': ['instructions']}
            annotations = {'readOnlyHint': True, 'openWorldHint': False}
        - id: Skill.init
          type: FunctionDef
          code: |-
            def __init__(self) -> None:
                self.description = f'{self.description}\n\n{self.hint}'
        - id: Skill.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Call ``core`` and wrap its result into the MCP output schema."""
                return ToolResult(structured_content={'instructions': self.core()})
        - id: Skill.core
          type: FunctionDef
          code: |-
            @staticmethod
            @abstractmethod
            def core() -> str:
                """Real function backing this skill; also registered in the ``FunctionRegistry``."""
      - id: markdown_format
        type: FunctionDef
        code: |-
          def markdown_format() -> str:
              """Preferred formatting rules for Pandoc-compatible Markdown documents.

              Load when formatting rules are requested or required; apply proactively
              whenever creating, editing, or reviewing Markdown documents — even when
              formatting is not explicitly mentioned.

              Returns:
                  The formatting instructions text.
              """
              return '* Use a line containing only `***` to insert a page break in PDF output.\n* Insert page breaks before top-level chapters (H1) at the start of each chapter.\n* Use `\\n---\\n` as a section separator before second-order chapters (H2) at the start of each chapter.\n* All files must end with an additional newline to prevent Markdown formatting errors on merge.\n* Use third-order headings and below only when necessary for navigation; use simple bold paragraph headings instead.\n* Chapter headings are numbered for H1–H3 only; lower-order headings do not contain numbering.\n* Use LaTeX (`$$`) for block mathematical expressions and inline LaTeX (`$`) for inline mathematical symbols, expressions, and formulas.'
      - id: MarkdownFormatSkill
        type: ClassDef
        code: |-
          class MarkdownFormatSkill(Skill):
              name = 'markdown_format'
              title = 'markdown-format'
              description = 'Preferred formatting rules for Pandoc-compatible Markdown documents. Load when formatting rules are requested or required.'
              hint = 'Apply proactively whenever creating, editing, or reviewing Markdown documents — even when formatting is not explicitly mentioned.'
              core = staticmethod(markdown_format)
      - id: tSwo7s
        type: statements
        code: |-
          '#: All declared skills.  Append here to add a new one.'
          SKILLS: list[Skill] = [MarkdownFormatSkill()]
      - id: register_skills
        type: FunctionDef
        code: |-
          def register_skills(registry: ToolRegistry, environment: AppEnvironment) -> None:
              """Register every skill in *skills* (defaults to :data:`SKILLS`) in both registries."""
              for skill in SKILLS:
                  registry.register(skill)
                  environment.functions.register(skill.core)
      errors: []
    errors: []
```
/answer 95d8-47 allow
Control Request:
```yaml
id: 95d8-48
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/function_registry.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
  reason: "Verstehen wie FunctionRegistry und ToolResult/ToolDefinition funktionieren, für konsistente Batch-Implementierung"
```
/answer 95d8-48 allow
Control Request:
```yaml
id: 95d8-49
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/function_registry.py
      nodes:
      - id: aPjaxG
        type: statements
      - id: tAKrfO
        type: imports
      - id: Si8ssC
        type: statements
      - id: FunctionEntry
        type: ClassDef
        signature: "@dataclass(slots=True) class FunctionEntry:"
        docstring: A single registered callable and the id it was published under.
        children:
        - id: FunctionEntry.VX6qOj
          type: statements
        - id: FunctionEntry.name
          type: FunctionDef
          signature: "@property def name(self) -> str:"
        - id: FunctionEntry.doc
          type: FunctionDef
          signature: "@property def doc(self) -> str:"
      - id: FunctionRegistry
        type: ClassDef
        signature: "class FunctionRegistry:"
        docstring: Process-wide registry of functions/bound methods usable as tools.
        children:
        - id: FunctionRegistry.yIM3yA
          type: statements
        - id: FunctionRegistry.init
          type: FunctionDef
          signature: "def __init__(self) -> None:"
        - id: FunctionRegistry.register
          type: FunctionDef
          signature: "def register(self, func: Callable[..., Any]) -> str:"
          docstring: Register *func* (a function or a bound method) under *id*.
        - id: FunctionRegistry.get
          type: FunctionDef
          signature: "def get(self, name: str) -> FunctionEntry | None:"
        - id: FunctionRegistry.all
          type: FunctionDef
          signature: "def all(self) -> list[FunctionEntry]:"
        - id: FunctionRegistry.ids
          type: FunctionDef
          signature: "def ids(self) -> list[str]:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
      nodes:
      - id: iTlyps
        type: statements
      - id: aHGJs9
        type: imports
      - id: BnS8bM
        type: statements
      - id: ToolResult
        type: ClassDef
        signature: "@dataclass(slots=True) class ToolResult:"
        docstring: The result of a tool call (maps onto MCP ``CallToolResult``).
        children:
        - id: ToolResult.U2I24j
          type: statements
        - id: ToolResult.tewJRa
          type: statements
        - id: ToolResult.to_dict
          type: FunctionDef
          signature: "def to_dict(self) -> dict[str, Any]:"
      - id: text_content
        type: FunctionDef
        signature: "def text_content(text: str) -> dict[str, Any]:"
        docstring: Build a ``TextContent`` block.
      - id: SRXvgt
        type: statements
      - id: Tool
        type: ClassDef
        signature: "@dataclass(slots=True) class Tool:"
        docstring: A registered tool and its MCP metadata.
        children:
        - id: Tool.SNEi1f
          type: statements
        - id: Tool.to_spec
          type: FunctionDef
          signature: "def to_spec(self) -> dict[str, Any]:"
          docstring: Return the MCP ``Tool`` object advertised via ``tools/list``.
      - id: ToolDefinition
        type: ClassDef
        signature: "class ToolDefinition(ABC):"
        docstring: Base class for registering a tool as an object. Subclasses declare the MCP meta…
        children:
        - id: ToolDefinition.OawaHI
          type: statements
        - id: ToolDefinition.cD1DZg
          type: statements
        - id: ToolDefinition.handle
          type: FunctionDef
          signature: "@abstractmethod def handle(self, ctx: ToolContext) -> 'ToolResult | str | dict[str, Any]':"
        - id: ToolDefinition.call
          type: FunctionDef
          signature: "def __call__(self, ctx: ToolContext) -> 'ToolResult | str | dict[str, Any]':"
        - id: ToolDefinition.to_tool
          type: FunctionDef
          signature: "def to_tool(self) -> Tool:"
      - id: normalize_result
        type: FunctionDef
        signature: "def normalize_result(value: 'ToolResult | str | dict[str, Any] | None') -> ToolResult:"
        docstring: Coerce whatever a handler returned into a :class:`ToolResult`.
      - id: xW8tkr
        type: statements
      - id: inject_property
        type: FunctionDef
        signature: "def _inject_property(schema: dict[str, Any], name: str, description: str, *, required: bool) -> dict[str, Any]:"
        docstring: Return *schema* with an additional property generically injected. Used both for…
      - id: with_mandatory_reason
        type: FunctionDef
        signature: "def _with_mandatory_reason(schema: dict[str, Any]) -> dict[str, Any]:"
        docstring: "Return *schema* with a mandatory, short ``reason`` property injected. Every too…"
      - id: with_optional_control_hint
        type: FunctionDef
        signature: "def _with_optional_control_hint(schema: dict[str, Any]) -> dict[str, Any]:"
        docstring: Return *schema* with the optional ``controlHint`` output property injected. Doc…
      - id: ToolRegistry
        type: ClassDef
        signature: "class ToolRegistry:"
        docstring: Process-wide registry of available tools.
        children:
        - id: ToolRegistry.xgr9BO
          type: statements
        - id: ToolRegistry.init
          type: FunctionDef
          signature: "def __init__(self) -> None:"
        - id: ToolRegistry.register_alias
          type: FunctionDef
          signature: "def register_alias(self, alias: str, members: 'Iterable[str]') -> None:"
          docstring: "Define (or extend) a tool-set alias expanding to *members*. Generic mechanism: …"
        - id: ToolRegistry.expand_aliases
          type: FunctionDef
          signature: "def expand_aliases(self, names: 'Iterable[str]') -> set[str]:"
          docstring: Expand any alias names in *names* to their member tool names. The alias ``"all"…
        - id: ToolRegistry.is_enabled
          type: FunctionDef
          signature: "def is_enabled(self, session: Session, name: str) -> bool:"
          docstring: "Whether *name* is enabled for *session*, honouring tool-set aliases."
        - id: ToolRegistry.register
          type: FunctionDef
          signature: "def register(self, tool: ToolDefinition) -> Tool:"
        - id: ToolRegistry.tool
          type: FunctionDef
          signature: "def tool(self, name: str, *, description: str, input_schema: dict[str, Any], title: str | None=None, output_schema: dict[str, Any] | None=None, annotations: dict[str, Any] | None=None) -> Callable[[ToolHandler], ToolHandler]:"
          docstring: Decorator registering the decorated function as a tool handler.
        - id: ToolRegistry.get
          type: FunctionDef
          signature: "def get(self, name: str) -> Tool | None:"
        - id: ToolRegistry.contains
          type: FunctionDef
          signature: "def __contains__(self, name: object) -> bool:"
        - id: ToolRegistry.len
          type: FunctionDef
          signature: "def __len__(self) -> int:"
        - id: ToolRegistry.names
          type: FunctionDef
          signature: "def names(self) -> list[str]:"
        - id: ToolRegistry.list_for_session
          type: FunctionDef
          signature: "def list_for_session(self, session: Session) -> list[Tool]:"
          docstring: Reconcile the registry with the session's enabled-tool configuration. Returns t…
    errors: []
```
/answer 95d8-49 allow
ReasoningToken: 28
ReasoningToken: 254
Control Request:
```yaml
id: 95d8-4a
toolName: grep
arguments:
  directory:
  - /home/user/xyan/xy.ai.workbench/mcpc/src
  pattern: markdown_format|register_skills|SKILLS\b
  include:
  - '*.py'
  exclude:
  - '*.min.py'
  reason: "Prüfen ob 'markdown_format' Toolname oder SKILLS anderweitig referenziert wird, bevor Skills-Architektur geändert wird"
```
/answer 95d8-4a allow
Control Request:
```yaml
id: 95d8-4b
result:
  structuredContent:
    matches:
    - path: xy/ai/mcpc/cli/parameters.py
      lineno: 122
      match: "        env['CLAUDE_CODE_DISABLE_BUNDLED_SKILLS'] = '1'"
    - path: xy/ai/mcpc/cli/parameters.py
      lineno: 128
      match: "        env['CLAUDE_CODE_DISABLE_POLICY_SKILLS'] = '1'"
    - path: xy/ai/mcpc/tools/skills/__init__.py
      lineno: 12
      match: "__all__ = ['Skill', 'SKILLS', 'MarkdownFormatSkill', 'register_skills', 'markdown_format']"
    - path: xy/ai/mcpc/tools/skills/__init__.py
      lineno: 33
      match: "def markdown_format() -> str:"
    - path: xy/ai/mcpc/tools/skills/__init__.py
      lineno: 46
      match: '    name = ''markdown_format'''
    - path: xy/ai/mcpc/tools/skills/__init__.py
      lineno: 50
      match: '    core = staticmethod(markdown_format)'
    - path: xy/ai/mcpc/tools/skills/__init__.py
      lineno: 52
      match: "SKILLS: list[Skill] = [MarkdownFormatSkill()]"
    - path: xy/ai/mcpc/tools/skills/__init__.py
      lineno: 54
      match: "def register_skills(registry: ToolRegistry, environment: AppEnvironment) -> None:"
    - path: xy/ai/mcpc/tools/skills/__init__.py
      lineno: 55
      match: '    """Register every skill in *skills* (defaults to :data:`SKILLS`) in both registries."""'
    - path: xy/ai/mcpc/tools/skills/__init__.py
      lineno: 56
      match: "    for skill in SKILLS:"
    - path: xy/ai/mcpc/tools/__init__.py
      lineno: 20
      match: from xy.ai.mcpc.tools.skills import register_skills
    - path: xy/ai/mcpc/tools/__init__.py
      lineno: 52
      match: "    register_skills(registry, environment)"
```
/answer 95d8-4b allow
ReasoningToken: 3640
Control Request:
```yaml
id: 95d8-4c
toolName: grep
arguments:
  directory:
  - /home/user/xyan/xy.ai.workbench/mcpc/src
  pattern: from xy\.ai\.mcpc\.tools\.(edit_block|edit_marks|edit_line|edit_lines|read|write|file_stats|list|grep) import
  include:
  - '*.py'
  exclude:
  - '*.min.py'
  reason: "Prüfen wo diese Tool-Funktionen anderweitig importiert und direkt verwendet werden, um Breaking Changes zu vermeiden"
```
/answer 95d8-4c allow
Control Request:
```yaml
id: 95d8-4d
result:
  structuredContent:
    matches:
    - path: xy/ai/mcpc/tools/mcp/exa/fetch.py
      lineno: 10
      match: from xy.ai.mcpc.tools.file_stats import compute_text_stats
    - path: xy/ai/mcpc/tools/__init__.py
      lineno: 9
      match: from xy.ai.mcpc.tools.list import register_list_tool
    - path: xy/ai/mcpc/tools/__init__.py
      lineno: 14
      match: from xy.ai.mcpc.tools.read import register_read_tool
    - path: xy/ai/mcpc/tools/__init__.py
      lineno: 15
      match: from xy.ai.mcpc.tools.edit_marks import register_edit_marks_tool
    - path: xy/ai/mcpc/tools/__init__.py
      lineno: 17
      match: from xy.ai.mcpc.tools.edit_lines import register_edit_lines_tool
    - path: xy/ai/mcpc/tools/__init__.py
      lineno: 18
      match: from xy.ai.mcpc.tools.edit_block import register_edit_block_tool
    - path: xy/ai/mcpc/tools/__init__.py
      lineno: 19
      match: from xy.ai.mcpc.tools.edit_line import register_edit_line_tool
    - path: xy/ai/mcpc/tools/__init__.py
      lineno: 21
      match: from xy.ai.mcpc.tools.write import register_write_tool
    - path: xy/ai/mcpc/tools/__init__.py
      lineno: 23
      match: from xy.ai.mcpc.tools.grep import register_grep_tool
    - path: xy/ai/mcpc/tools/__init__.py
      lineno: 28
      match: from xy.ai.mcpc.tools.file_stats import register_file_stats_tool
    - path: xy/ai/mcpc/tools/edit_line/__init__.py
      lineno: 8
      match: "from xy.ai.mcpc.tools.edit_block import EditBlockError, edit_block"
```
/answer 95d8-4d allow
ReasoningToken: 884
Control Request:
```yaml
id: 95d8-4e
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/function_registry.py
    ids:
    - aPjaxG
    - tAKrfO
    - Si8ssC
    - FunctionEntry
    - FunctionRegistry
  reason: Verstehen wie Funktionsname und Docstring für FunctionRegistry ermittelt werden
```
/answer 95d8-4e allow
Control Request:
```yaml
id: 95d8-4f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/function_registry.py
      nodes:
      - id: aPjaxG
        type: statements
        code: "'Registry for plain Python functions/bound methods exposed to the\\n``tool_search`` / ``tool_usage`` / ``tool_call`` family.\\n\\nComplements the classic MCP :class:`~xy.ai.mcpc.tools.tool_registry.ToolRegistry`:\\nentries here are never advertised via ``tools/list``. They are ordinary\\nPython callables (module-level functions or bound methods on a live\\ninstance) that ``tool_search`` can find by keyword, ``tool_usage`` can\\nintrospect (signature, docstring, referenced project-local types), and\\n``tool_call`` can inject by id into its sandboxed script namespace.\\n'"
      - id: tAKrfO
        type: imports
        code: |-
          import inspect
          import logging
          from dataclasses import dataclass
          from typing import Any, Callable
      - id: Si8ssC
        type: statements
        code: |-
          __all__ = ['FunctionEntry', 'FunctionRegistry']
          logger = logging.getLogger('xy.ai.mcpc.control')
      - id: FunctionEntry
        type: ClassDef
        signature: "@dataclass(slots=True) class FunctionEntry:"
        docstring: A single registered callable and the id it was published under.
        children:
        - id: FunctionEntry.VX6qOj
          type: statements
          code: |-
            'A single registered callable and the id it was published under.'
            id: str
            func: Callable[..., Any]
        - id: FunctionEntry.name
          type: FunctionDef
          code: |-
            @property
            def name(self) -> str:
                return getattr(self.func, '__name__', self.id)
        - id: FunctionEntry.doc
          type: FunctionDef
          code: |-
            @property
            def doc(self) -> str:
                return inspect.getdoc(self.func) or ''
      - id: FunctionRegistry
        type: ClassDef
        signature: "class FunctionRegistry:"
        docstring: Process-wide registry of functions/bound methods usable as tools.
        children:
        - id: FunctionRegistry.yIM3yA
          type: statements
          code: '''Process-wide registry of functions/bound methods usable as tools.'''
        - id: FunctionRegistry.init
          type: FunctionDef
          code: |-
            def __init__(self) -> None:
                self._entries: dict[str, FunctionEntry] = {}
        - id: FunctionRegistry.register
          type: FunctionDef
          code: |-
            def register(self, func: Callable[..., Any]) -> str:
                """Register *func* (a function or a bound method) under *id*.
                    """
                entry_id = getattr(func, '__qualname__', None) or getattr(func, '__name__')
                existing = self._entries.get(entry_id)
                if existing is not None and existing.func is not func:
                    raise ValueError(f'Function already registered under id: {entry_id}')
                self._entries[entry_id] = FunctionEntry(id=entry_id, func=func)
                logger.debug('Registered function: %s', entry_id)
                return entry_id
        - id: FunctionRegistry.get
          type: FunctionDef
          code: |-
            def get(self, name: str) -> FunctionEntry | None:
                return self._entries.get(name)
        - id: FunctionRegistry.all
          type: FunctionDef
          code: |-
            def all(self) -> list[FunctionEntry]:
                return list(self._entries.values())
        - id: FunctionRegistry.ids
          type: FunctionDef
          code: |-
            def ids(self) -> list[str]:
                return list(self._entries)
      errors: []
    errors: []
```
/answer 95d8-4f allow
ReasoningToken: 521
Control Request:
```yaml
id: 95d8-50
toolName: grep
arguments:
  directory:
  - /home/user/xyan/xy.ai.workbench/mcpc/src
  pattern: compute_file_stats|from xy\.ai\.mcpc\.tools\.file_stats import
  include:
  - '*.py'
  exclude:
  - '*.min.py'
  reason: "Prüfen wo compute_file_stats verwendet wird, um Kompatibilität sicherzustellen"
```
/answer 95d8-50 allow
Control Request:
```yaml
id: 95d8-51
result:
  structuredContent:
    matches:
    - path: xy/ai/mcpc/tools/mcp/exa/fetch.py
      lineno: 10
      match: from xy.ai.mcpc.tools.file_stats import compute_text_stats
    - path: xy/ai/mcpc/tools/__init__.py
      lineno: 28
      match: from xy.ai.mcpc.tools.file_stats import register_file_stats_tool
    - path: xy/ai/mcpc/tools/file_stats/__init__.py
      lineno: 20
      match: "    'compute_file_stats',"
    - path: xy/ai/mcpc/tools/file_stats/__init__.py
      lineno: 78
      match: '    Extracted from :func:`compute_file_stats` so string content (e.g. fetched'
    - path: xy/ai/mcpc/tools/file_stats/__init__.py
      lineno: 104
      match: "def compute_file_stats(path: Path) -> FileStatsResult:"
    - path: xy/ai/mcpc/tools/file_stats/__init__.py
      lineno: 162
      match: '    return compute_file_stats(file_path)'
```
/answer 95d8-51 allow
ReasoningToken: 3555
Control Request:
```yaml
id: 95d8-52
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
    ids:
    - ToolDefinition
    - Tool
    - ToolResult
  reason: Prüfen der ToolDefinition Basisklasse für korrekte Skills-Tool Implementierung
```
/answer 95d8-52 allow
Control Request:
```yaml
id: 95d8-53
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
      nodes:
      - id: ToolDefinition
        type: ClassDef
        signature: "class ToolDefinition(ABC):"
        docstring: Base class for registering a tool as an object. Subclasses declare the MCP meta…
        children:
        - id: ToolDefinition.OawaHI
          type: statements
          code: "'Base class for registering a tool as an object.\\n\\n    Subclasses declare the MCP metadata as class attributes and implement\\n    :meth:`handle`; an instance is itself a callable :data:`ToolHandler` and\\n    can be passed directly to :meth:`ToolRegistry.register`::\\n\\n        class MyTool(ToolDefinition):\\n            name = \"my_tool\"\\n            description = \"...\"\\n            input_schema = {...}\\n\\n            def handle(self, ctx: ToolContext) -> ToolResult:\\n                ...\\n\\n        registry.register(MyTool())\\n    '"
        - id: ToolDefinition.cD1DZg
          type: statements
          code: |-
            name: str
            description: str
            input_schema: dict[str, Any]
            title: str | None = None
            output_schema: dict[str, Any] | None = None
            annotations: dict[str, Any] | None = None
        - id: ToolDefinition.handle
          type: FunctionDef
          code: |-
            @abstractmethod
            def handle(self, ctx: ToolContext) -> 'ToolResult | str | dict[str, Any]':
                ...
        - id: ToolDefinition.call
          type: FunctionDef
          code: |-
            def __call__(self, ctx: ToolContext) -> 'ToolResult | str | dict[str, Any]':
                return self.handle(ctx)
        - id: ToolDefinition.to_tool
          type: FunctionDef
          code: |-
            def to_tool(self) -> Tool:
                return Tool(
                    name=self.name,
                    description=self.description,
                    input_schema=self.input_schema,
                    handler=self,
                    title=self.title,
                    output_schema=self.output_schema,
                    annotations=self.annotations)
      - id: Tool
        type: ClassDef
        signature: "@dataclass(slots=True) class Tool:"
        docstring: A registered tool and its MCP metadata.
        children:
        - id: Tool.SNEi1f
          type: statements
          code: |-
            'A registered tool and its MCP metadata.'
            name: str
            description: str
            input_schema: dict[str, Any]
            handler: ToolHandler
            title: str | None = None
            output_schema: dict[str, Any] | None = None
            annotations: dict[str, Any] | None = None
            meta: dict[str, Any] | None = None
        - id: Tool.to_spec
          type: FunctionDef
          code: |-
            def to_spec(self) -> dict[str, Any]:
                """Return the MCP ``Tool`` object advertised via ``tools/list``."""
                spec: dict[str, Any] = {'name': self.name, 'description': self.description, 'inputSchema': self.input_schema}
                if self.title is not None:
                    spec['title'] = self.title
                if self.output_schema is not None:
                    spec['outputSchema'] = self.output_schema
                if self.meta is not None:
                    spec['_meta'] = self.meta
                if self.annotations is not None:
                    spec['annotations'] = self.annotations
                return spec
      - id: ToolResult
        type: ClassDef
        signature: "@dataclass(slots=True) class ToolResult:"
        docstring: The result of a tool call (maps onto MCP ``CallToolResult``).
        children:
        - id: ToolResult.U2I24j
          type: statements
          code: |-
            'The result of a tool call (maps onto MCP ``CallToolResult``).'
            content: list[dict[str, Any]] = field(default_factory=list)
            structured_content: dict[str, Any] | None = None
            is_error: bool = False
            auto_approve: bool = False
            control_hint: str | None = None
        - id: ToolResult.tewJRa
          type: statements
          code: "'Optional hint attached by the controller on approval (``/allow <id> <hint>``).\\n\\n    Embedded as :data:`CONTROL_HINT_PROPERTY` *inside* ``structuredContent``\\n    (see :meth:`to_dict`) rather than as a top-level ``CallToolResult`` field:\\n    MCP clients only surface ``content``/``structuredContent``/``isError`` to\\n    the model, so a sibling top-level key would silently be dropped before\\n    ever reaching the agent.\\n    '"
        - id: ToolResult.to_dict
          type: FunctionDef
          code: |-
            def to_dict(self) -> dict[str, Any]:
                result: dict[str, Any] = {}
                if self.content:
                    result['content'] = list(self.content)
                structured = dict(self.structured_content) if self.structured_content else {}
                if self.control_hint:
                    structured[CONTROL_HINT_PROPERTY] = self.control_hint
                if structured:
                    result['structuredContent'] = structured
                if self.is_error:
                    result['isError'] = True
                return result
      errors: []
    errors: []
```
/answer 95d8-53 allow
ReasoningToken: 33156
Control Request:
```yaml
id: 95d8-54
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
    overwrite: true
    source: |
      """Grep tool – recursive extended-regex search for retrieval, for a batch of items."""

      from dataclasses import dataclass
      from pathlib import Path
      from typing import Any

      from xy.ai.mcpc.tools._directories import normalize_directories
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, run_process
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      import re

      __all__ = [
          'GrepError',
          'GrepMatch',
          'GrepItem',
          'GrepResult',
          'GrepItemError',
          'GrepBatchResult',
          'grep',
          'GrepTool',
          'register_grep_tool']
      _DEFAULT_LIMIT = 15
      _MAX_LIMIT = 50


      class GrepError(Exception):
          """Raised when a grep search cannot be executed or its output cannot be parsed."""


      @dataclass(frozen=True)
      class GrepMatch:
          """A single grep match, parsed from a 'path:line:content' output line."""
          directory: str
          filename: str
          lineno: int
          match: str


      @dataclass(frozen=True)
      class GrepItem:
          """One independent grep search to run.

          Attributes:
              directory: Absolute paths of the directories to search recursively.
              pattern: Extended regular expression (grep -E syntax) to search for.
              exclude: Globs of file names to exclude from the search, if given.
              include: Globs of file names to include in the search, if given.
              limit: Maximum number of matches to return (1..``_MAX_LIMIT``). Applies per item, not per batch.
          """
          directory: list[str]
          pattern: str
          exclude: list[str] | None = None
          include: list[str] | None = None
          limit: int = _DEFAULT_LIMIT


      @dataclass(frozen=True)
      class GrepResult:
          """Result of a single grep search, mirroring its input for result association.

          Attributes:
              directory: The directory list exactly as given in the input.
              pattern: The pattern exactly as given in the input.
              matches: The matches found (empty if none).
              warning: Set if ``limit`` was reached and further matches may exist.
          """
          directory: list[str]
          pattern: str
          matches: list[GrepMatch]
          warning: str | None = None


      @dataclass(frozen=True)
      class GrepItemError:
          """Error running a single grep search, mirroring its input for result association."""
          directory: list[str]
          pattern: str
          error: str


      @dataclass(frozen=True)
      class GrepBatchResult:
          """Result of :func:`grep`.

          Attributes:
              results: One :class:`GrepResult` per successfully executed search.
              errors: One :class:`GrepItemError` per search that failed.
          """
          results: list[GrepResult]
          errors: list[GrepItemError]


      def _parse_grep_stdout(stdout: str) -> list[GrepMatch]:
          """Parse grep's 'path:line:content' stdout into :class:`GrepMatch` objects."""
          matches: list[GrepMatch] = []
          for line in stdout.splitlines():
              if not line:
                  continue
              path, sep, rest = line.partition(':')
              if not sep:
                  raise GrepError(f'Cannot parse grep output line: {line!r}')
              lineno_str, sep, match = rest.partition(':')
              if not sep or not lineno_str.isdigit():
                  raise GrepError(f'Cannot parse grep output line: {line!r}')
              directory, _, filename = path.rpartition('/')
              matches.append(GrepMatch(directory=directory, filename=filename, lineno=int(lineno_str), match=match))
          return matches


      def _as_list(value: list[str] | None) -> list[str]:
          """Normalize an optional list into a list (empty if ``None``)."""
          return list(value) if value is not None else []


      def _run_grep(directory: list[str], pattern: str, *, exclude: list[str] | None=None, include: list[str] | None=None, limit: int=_DEFAULT_LIMIT) -> ProcessResult:
          """Recursively search one or more directories for ``pattern`` (extended regexp).

          Args:
              directory: Absolute paths of the directories to search (each must exist and
                  be a directory).
              pattern: Extended regular expression (grep -E syntax).
              exclude: Globs of file names to exclude from the search, if given.
              include: Globs of file names to include in the search, if given.
              limit: Maximum number of matching lines to return (1..``_MAX_LIMIT``).

          Returns:
              ProcessResult with:
                  exit_code: 0 if matches were found, 1 if none were found, >=2 on grep error.
                  stdout: Matching lines as 'path:line:content', with ``path`` relative to
                      whichever searched directory it was found under, truncated to at most
                      ``limit`` lines.
                  stderr: Standard error output (up to 3000 chars; see stderr_file if longer).
                  stdout_file: Absolute path to temp file with full stdout if to large.
                  stderr_file: Absolute path to temp file with full stderr if to large.

          Raises:
              GrepError: If a directory is not absolute.
              GrepError: If a directory does not exist or is not a directory.
              GrepError: If no directory is given.
              GrepError: If pattern is empty.
              GrepError: If limit is not between 1 and ``_MAX_LIMIT``.
              GrepError: If grep binary cannot be launched.
          """
          directory_paths = normalize_directories([Path(d) for d in _as_list(directory)])
          if not directory_paths:
              raise GrepError('At least one directory is required.')
          for directory_path in directory_paths:
              if not directory_path.is_absolute():
                  raise GrepError('directory must be an absolute path.')
              if not directory_path.is_dir():
                  raise GrepError('Directory not found or not a directory.')
          if not pattern:
              raise GrepError('pattern must not be empty.')
          if not 1 <= limit <= _MAX_LIMIT:
              raise GrepError(f'limit must be between 1 and {_MAX_LIMIT}.')
          cmd = ['grep', '--recursive', '--line-number', '--extended-regexp', '--binary-files=without-match', '--color=never']
          for pattern_glob in _as_list(include):
              cmd.append(f'--include={pattern_glob}')
          for pattern_glob in _as_list(exclude):
              cmd.append(f'--exclude={pattern_glob}')
          cmd += ['--', pattern, *(str(p) for p in directory_paths)]
          try:
              result = run_process(cmd)
          except LaunchError as exc:
              raise GrepError(f'Failed to launch grep: {exc}') from exc
          prefixes = sorted((str(p).rstrip('/') + '/' for p in directory_paths), key=len, reverse=True)
          prefix_pattern = '|'.join((re.escape(p) for p in prefixes))
          stdout = re.sub(f'^(?:{prefix_pattern})', '', result.stdout, flags=re.MULTILINE)
          lines = stdout.splitlines()
          stdout = '\n'.join(lines[:limit])
          return ProcessResult(exit_code=result.exit_code, stdout=stdout, stderr=result.stderr)


      def _grep_one(item: GrepItem) -> GrepResult:
          result = _run_grep(item.directory, item.pattern, exclude=item.exclude, include=item.include, limit=item.limit)
          if result.exit_code >= 2:
              raise GrepError(f'grep failed (exit code {result.exit_code}): {result.stderr}')
          matches = _parse_grep_stdout(result.stdout)
          warning = None
          if len(matches) >= item.limit:
              warning = f'Limit of {item.limit} matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit.'
          return GrepResult(directory=item.directory, pattern=item.pattern, matches=matches, warning=warning)


      def grep(items: list[GrepItem]) -> GrepBatchResult:
          """Run one or more independent grep searches. Limits apply per item, not per batch.

          Args:
              items: Grep searches to run. Must be non-empty.

          Returns:
              GrepBatchResult: one result per successful search, one error per failed search.

          Raises:
              GrepError: If items is empty.
          """
          if not items:
              raise GrepError("'items' must be a non-empty list.")
          results: list[GrepResult] = []
          errors: list[GrepItemError] = []
          for item in items:
              try:
                  results.append(_grep_one(item))
              except GrepError as exc:
                  errors.append(GrepItemError(directory=item.directory, pattern=item.pattern, error=str(exc)))
          return GrepBatchResult(results=results, errors=errors)


      class GrepTool(ToolDefinition):
          name = 'grep'
          title = 'Search files with grep'
          description = "Run one or more independent grep searches for lines matching an extended regular expression, for a batch of items. Always use the 'include' and 'exclude' filters. Limits apply per item, not per batch."
          input_schema = {
              'type': 'object',
              'properties': {
                  'items': {
                      'type': 'array',
                      'minItems': 1,
                      'items': {
                          'type': 'object',
                          'additionalProperties': False,
                          'properties': {
                              'directory': {
                                  'type': 'array',
                                  'items': {'type': 'string'},
                                  'minItems': 1,
                                  'description': 'Absolute paths of the directories to search recursively. Always use the narrowest subtree(s) that are likely to contain the target files.'},
                              'pattern': {
                                  'type': 'string',
                                  'description': 'Extended regular expression to search for. Make the pattern as specific as possible to reduce noise.'},
                              'exclude': {
                                  'type': 'array',
                                  'items': {'type': 'string'},
                                  'description': "Globs of file names to exclude from the search, e.g. '*.min.js'. Always set this to exclude build artefacts, dependencies (e.g. 'node_modules/**'), and minified files."},
                              'include': {
                                  'type': 'array',
                                  'items': {'type': 'string'},
                                  'description': "Globs of file names to include in the search, e.g. '*.py'. Always set this to restrict the search to the relevant file types; omit only when the file type is unknown."},
                              'limit': {
                                  'type': 'integer',
                                  'description': 'Maximum number of matching lines to return for this item.',
                                  'default': _DEFAULT_LIMIT,
                                  'minimum': 1,
                                  'maximum': _MAX_LIMIT}},
                          'required': ['directory', 'pattern']},
                      'description': 'Independent grep searches to run.'}},
              'required': ['items']}
          output_schema = {
              'type': 'object',
              'properties': {
                  'results': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {
                              'directory': {'type': 'array', 'items': {'type': 'string'}},
                              'pattern': {'type': 'string'},
                              'matches': {
                                  'type': 'array',
                                  'items': {
                                      'type': 'object',
                                      'properties': {
                                          'path': {'type': 'string'},
                                          'lineno': {'type': 'integer'},
                                          'match': {'type': 'string'}},
                                      'required': ['path', 'lineno', 'match']}},
                              'warning': {'type': 'string'}},
                          'required': ['directory', 'pattern', 'matches']}},
                  'errors': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {
                              'directory': {'type': 'array', 'items': {'type': 'string'}},
                              'pattern': {'type': 'string'},
                              'error': {'type': 'string'}},
                          'required': ['directory', 'pattern', 'error']}}},
              'required': ['results', 'errors']}
          annotations = {'readOnlyHint': True, 'idempotentHint': True, 'openWorldHint': False}

          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`grep`, translating the MCP schema to/from the Python API."""
              args: dict[str, Any] = ctx.arguments
              raw_items = args.get('items') or []
              if not raw_items:
                  return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
              items = [
                  GrepItem(
                      directory=it['directory'],
                      pattern=it['pattern'],
                      exclude=it.get('exclude'),
                      include=it.get('include'),
                      limit=int(it.get('limit', _DEFAULT_LIMIT))) for it in raw_items]
              batch = grep(items)
              results = []
              for r in batch.results:
                  entry: dict[str, Any] = {
                      'directory': r.directory,
                      'pattern': r.pattern,
                      'matches': [
                          {'path': f'{m.directory}/{m.filename}' if m.directory else m.filename, 'lineno': m.lineno, 'match': m.match}
                          for m in r.matches]}
                  if r.warning is not None:
                      entry['warning'] = r.warning
                  results.append(entry)
              errors = [{'directory': e.directory, 'pattern': e.pattern, 'error': e.error} for e in batch.errors]
              is_error = bool(batch.errors) and not batch.results
              return ToolResult(structured_content={'results': results, 'errors': errors}, is_error=is_error, auto_approve=not is_error)


      def register_grep_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
          registry.register(GrepTool())
          functions.register(grep)
  reason: grep-Tool auf Batch-Support umstellen
```
/answer 95d8-54 allow
Control Request:
```yaml
id: 95d8-55
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    overwrite: true
    source: |
      """List tool – returns files below a directory, grouped by relative subdirectory like ``ls -R``, for a batch of items.

      Walks each given absolute directory recursively and returns all file paths
      (files only, no directories), grouped by the relative directory they live in
      (e.g. ``./src/pkg:`` followed by tab-indented file names), mirroring the
      output format of ``ls -R``. An optional regular expression can be supplied to
      filter the resulting files (matched against each file's path relative to the
      requested directory). Common VCS/build/cache directories (e.g. ``.git``) are
      always excluded. To keep results manageable, the number of matched files per
      item is capped; use ``pattern`` to narrow down large directories instead of
      raising the limit.
      """

      import os
      import re
      from dataclasses import dataclass
      from pathlib import Path
      from typing import Any

      from xy.ai.mcpc.tools._directories import normalize_directory
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry

      __all__ = [
          'ListError',
          'ListItem',
          'ListResult',
          'ListItemError',
          'ListBatchResult',
          'list',
          'ListTool',
          'register_list_tool']
      _MAX_ENTRIES = 50
      _EXCLUDED_DIRS = {
          '.git',
          '.hg',
          '.svn',
          '__pycache__',
          '.mypy_cache',
          '.pytest_cache',
          '.ruff_cache',
          '.tox',
          '.venv',
          'venv',
          'node_modules',
          '.idea',
          '.vscode',
          'dist',
          'build',
          '.cache'}


      class ListError(Exception):
          """Raised when a directory listing cannot be performed."""


      @dataclass(frozen=True)
      class ListItem:
          """One directory to list.

          Attributes:
              path: Absolute directory path to list (must exist and be a directory).
              pattern: Optional regular expression to filter results. Only matching file paths are included.
          """
          path: str
          pattern: str | None = None


      @dataclass(frozen=True)
      class ListResult:
          """Result of listing a single directory, mirroring its input path for result association."""
          path: str
          entries: list[str]


      @dataclass(frozen=True)
      class ListItemError:
          """Error listing a single directory, mirroring its input path for result association."""
          path: str
          error: str


      @dataclass(frozen=True)
      class ListBatchResult:
          """Result of :func:`list`.

          Attributes:
              results: One :class:`ListResult` per successfully listed directory.
              errors: One :class:`ListItemError` per directory that failed.
          """
          results: list[ListResult]
          errors: list[ListItemError]


      def _list_one(item: ListItem) -> ListResult:
          """List all files below the absolute directory ``item.path``, grouped like ``ls -R``.

          Raises:
              ListError: If path is not absolute.
              ListError: If path does not exist or is not a directory.
              ListError: If pattern is not a valid regular expression.
              ListError: If more than ``_MAX_ENTRIES`` files match.
          """
          dir_path = Path(item.path)
          if not dir_path.is_absolute():
              raise ListError('Path must be absolute.')
          dir_path = normalize_directory(dir_path)
          if not dir_path.is_dir():
              raise ListError('Directory not found or not a directory.')
          try:
              regex = re.compile(item.pattern) if item.pattern else None
          except re.error as exc:
              raise ListError(f'Invalid regex pattern: {exc}') from exc
          groups: dict[str, list[str]] = {}
          match_count = 0
          for root, dirs, files in os.walk(str(dir_path)):
              rel_dir = os.path.relpath(root, str(dir_path))
              matched_files = []
              for file in sorted(files):
                  rel_path = os.path.normpath(os.path.join(rel_dir, file))
                  if regex is None or regex.search(rel_path):
                      matched_files.append(file)
              if matched_files:
                  groups[rel_dir] = matched_files
                  match_count += len(matched_files)
          if match_count > _MAX_ENTRIES:
              raise ListError(
                  f"Too many entries ({match_count}) exceed the limit of {_MAX_ENTRIES}. Narrow down the result using the 'pattern' regular expression parameter.")
          entries = []
          for rel_dir in sorted(groups):
              header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
              entries.append(f'{header}:')
              entries.extend(groups[rel_dir])
          return ListResult(path=item.path, entries=entries)


      def list(items: list[ListItem]) -> ListBatchResult:
          """List each directory in ``items``.

          Args:
              items: Directories to list. Must be non-empty.

          Returns:
              ListBatchResult: one result per successfully listed directory, one error per failed directory.

          Raises:
              ListError: If items is empty.
          """
          if not items:
              raise ListError("'items' must be a non-empty list.")
          results: list[ListResult] = []
          errors: list[ListItemError] = []
          for item in items:
              try:
                  results.append(_list_one(item))
              except ListError as exc:
                  errors.append(ListItemError(path=item.path, error=str(exc)))
          return ListBatchResult(results=results, errors=errors)


      class ListTool(ToolDefinition):
          name = 'list'
          title = 'List directory contents'
          description = 'List all files below one or more absolute directory paths, recursively, as a flat list, for a batch of items. Filter each result with a regular expression. Limits apply per item, not per batch.'
          input_schema = {
              'type': 'object',
              'properties': {
                  'items': {
                      'type': 'array',
                      'minItems': 1,
                      'items': {
                          'type': 'object',
                          'additionalProperties': False,
                          'properties': {
                              'path': {'type': 'string', 'description': 'Absolute directory path.'},
                              'pattern': {'type': 'string', 'description': 'Regular expression to filter the result.'}},
                          'required': ['path']},
                      'description': 'Directories to list.'}},
              'required': ['items']}
          output_schema = {
              'type': 'object',
              'properties': {
                  'results': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {
                              'path': {'type': 'string'},
                              'entries': {'type': 'array', 'items': {'type': 'string'}}},
                          'required': ['path', 'entries']}},
                  'errors': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {'path': {'type': 'string'}, 'error': {'type': 'string'}},
                          'required': ['path', 'error']}}},
              'required': ['results', 'errors']}
          annotations = {'readOnlyHint': True, 'openWorldHint': False}

          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`list`, translating the MCP schema to/from the Python API."""
              args: dict[str, Any] = ctx.arguments
              raw_items = args.get('items') or []
              if not raw_items:
                  return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
              items = [ListItem(path=it['path'], pattern=it.get('pattern')) for it in raw_items]
              batch = list(items)
              results = [{'path': r.path, 'entries': r.entries} for r in batch.results]
              errors = [{'path': e.path, 'error': e.error} for e in batch.errors]
              is_error = bool(batch.errors) and not batch.results
              return ToolResult(structured_content={'results': results, 'errors': errors}, is_error=is_error, auto_approve=not is_error)


      def register_list_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
          registry.register(ListTool())
          functions.register(list)
  reason: list-Tool auf Batch-Support umstellen
```
/answer 95d8-55 allow
Control Request:
```yaml
id: 95d8-56
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
    overwrite: true
    source: |
      """File stats tool – returns file metrics for access and processing planning, for a batch of items.

      Provides compact metrics including complexity, timestamps, size, line/word
      counts, line length statistics, and average words per line.
      """
      import hashlib
      import re
      from dataclasses import asdict, dataclass
      from datetime import datetime, timezone
      from pathlib import Path
      from typing import Any

      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry

      __all__ = [
          'FileStatsError',
          'FileStatsItem',
          'FileStatsResult',
          'FileStatsItemError',
          'FileStatsBatchResult',
          'TextStatsResult',
          'compute_text_stats',
          'compute_file_stats',
          'file_stats',
          'FileStatsTool',
          'register_file_stats_tool']


      class FileStatsError(Exception):
          """Raised when file metrics cannot be computed."""


      @dataclass(frozen=True)
      class FileStatsItem:
          """One file to compute metrics for.

          Attributes:
              path: Absolute path to file (must exist and be a regular file).
          """
          path: str


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


      @dataclass(frozen=True)
      class FileStatsItemError:
          """Error computing metrics for a single file, mirroring its input path for result association."""
          path: str
          error: str


      @dataclass(frozen=True)
      class FileStatsBatchResult:
          """Result of :func:`file_stats`.

          Attributes:
              results: One :class:`FileStatsResult` per file whose metrics could be computed.
              errors: One :class:`FileStatsItemError` per file that failed.
          """
          results: list[FileStatsResult]
          errors: list[FileStatsItemError]


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
              path=str(
                  path.resolve()),
              created=created,
              modified=modified,
              accessed=accessed,
              **text_stats)


      def _file_stats_one(item: FileStatsItem) -> FileStatsResult:
          file_path = Path(item.path)
          if not file_path.is_absolute():
              raise FileStatsError('Path must be absolute.')
          if not file_path.exists():
              raise FileStatsError('File not found.')
          if not file_path.is_file():
              raise FileStatsError('Not a regular file.')
          return compute_file_stats(file_path)


      def file_stats(items: list[FileStatsItem]) -> FileStatsBatchResult:
          """Compute file metrics for each absolute path in ``items``.

          Args:
              items: Files to compute metrics for. Must be non-empty.

          Returns:
              FileStatsBatchResult: one result per file whose metrics could be computed, one error
              per failed file. Each result includes:
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
              FileStatsError: If items is empty.

          Note:
              Binary files are decoded as UTF-8 with error replacement.
              Timestamps use fallback to mtime if birthtime not available (Linux).
          """
          if not items:
              raise FileStatsError("'items' must be a non-empty list.")
          results: list[FileStatsResult] = []
          errors: list[FileStatsItemError] = []
          for item in items:
              try:
                  results.append(_file_stats_one(item))
              except FileStatsError as exc:
                  errors.append(FileStatsItemError(path=item.path, error=str(exc)))
          return FileStatsBatchResult(results=results, errors=errors)


      class FileStatsTool(ToolDefinition):
          name = 'file_stats'
          title = 'File stats'
          description = 'Get file metrics for access and processing planning, for a batch of items: complexity, timestamps, size, line/word counts, and line length statistics.'
          input_schema = {
              'type': 'object',
              'properties': {
                  'items': {
                      'type': 'array',
                      'minItems': 1,
                      'items': {
                          'type': 'object',
                          'additionalProperties': False,
                          'properties': {'path': {'type': 'string', 'description': 'Absolute file path.'}},
                          'required': ['path']},
                      'description': 'Files to compute metrics for.'}},
              'required': ['items']}
          output_schema = {
              'type': 'object',
              'properties': {
                  'results': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {
                              'path': {'type': 'string', 'description': 'Absolute file path.'},
                              'size_bytes': {'type': 'integer', 'description': 'File size in bytes.'},
                              'lines': {'type': 'integer', 'description': 'Total number of lines.'},
                              'words': {'type': 'integer', 'description': 'Total number of words.'},
                              'complexity': {'type': 'number', 'description': 'Data structure complexity (0.0 to 1.0).'},
                              'created': {'type': 'string', 'description': 'Creation timestamp (ISO 8601).'},
                              'modified': {'type': 'string', 'description': 'Last modification timestamp (ISO 8601).'},
                              'accessed': {'type': 'string', 'description': 'Last access timestamp (ISO 8601).'},
                              'line_length_max': {'type': 'integer', 'description': 'Maximum line length in characters.'},
                              'line_length_min': {'type': 'integer', 'description': 'Minimum line length in characters.'},
                              'line_length_avg': {'type': 'number', 'description': 'Average line length in characters.'},
                              'words_per_line_avg': {'type': 'number', 'description': 'Average number of words per line.'},
                              'checksum': {'type': 'string', 'description': 'sha256 checksum of the file content.'}},
                          'required': [
                              'path', 'size_bytes', 'lines', 'words', 'complexity', 'created', 'modified', 'accessed',
                              'line_length_max', 'line_length_min', 'line_length_avg', 'words_per_line_avg', 'checksum']}},
                  'errors': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {'path': {'type': 'string'}, 'error': {'type': 'string'}},
                          'required': ['path', 'error']}}},
              'required': ['results', 'errors']}
          annotations = {'readOnlyHint': True, 'openWorldHint': False}

          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`file_stats`, translating the MCP schema to/from the Python API."""
              args: dict[str, Any] = ctx.arguments
              raw_items = args.get('items') or []
              if not raw_items:
                  return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
              items = [FileStatsItem(path=it['path']) for it in raw_items]
              batch = file_stats(items)
              results = [asdict(r) for r in batch.results]
              errors = [{'path': e.path, 'error': e.error} for e in batch.errors]
              is_error = bool(batch.errors) and not batch.results
              return ToolResult(content=[], structured_content={'results': results, 'errors': errors}, is_error=is_error, auto_approve=not is_error)


      def register_file_stats_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
          registry.register(FileStatsTool())
          functions.register(file_stats)
  reason: file_stats-Tool auf Batch-Support umstellen
```
/answer 95d8-56 allow
Control Request:
```yaml
id: 95d8-57
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
    overwrite: true
    source: |
      """Edit-block tool – edits an exact block of text (old -> new) in a file, for a batch of items."""

      from dataclasses import dataclass
      from pathlib import Path
      from typing import Any

      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools._text_match import replace_in_block, line_preserving, TextMatchError
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry

      __all__ = [
          'EditBlockError',
          'EditBlockItem',
          'EditBlockResult',
          'EditBlockItemError',
          'EditBlockBatchResult',
          'edit_block',
          'EditBlockTool',
          'register_edit_block_tool']


      class EditBlockError(Exception):
          """Raised when a edit-block operation cannot be performed."""


      @dataclass(frozen=True)
      class EditBlockItem:
          """One block edit to apply.

          Attributes:
              path: Absolute path to file (must be a regular file).
              old_text: Unique text to find and replace (must occur exactly once, unless replace_all).
              new_text: Replacement text (may be empty to perform a deletion).
              exact: If False (default), whitespace in old_text is matched tolerantly.
                     If True, whitespace must match exactly.
              replace_all: If True, replace every occurrence of old_text instead of requiring
                           a single unique match.
          """
          path: str
          old_text: str
          new_text: str
          exact: bool = False
          replace_all: bool = False


      @dataclass(frozen=True)
      class EditBlockResult:
          """Result of applying a single block edit, mirroring its input path for result association."""
          path: str
          result: str


      @dataclass(frozen=True)
      class EditBlockItemError:
          """Error applying a single block edit, mirroring its input path for result association."""
          path: str
          error: str


      @dataclass(frozen=True)
      class EditBlockBatchResult:
          """Result of :func:`edit_block`.

          Attributes:
              results: One :class:`EditBlockResult` per successfully applied edit.
              errors: One :class:`EditBlockItemError` per edit that failed.
          """
          results: list[EditBlockResult]
          errors: list[EditBlockItemError]


      def _edit_block_one(item: EditBlockItem) -> EditBlockResult:
          file_path = Path(item.path)
          if not file_path.is_absolute():
              raise EditBlockError('Path must be absolute.')
          if not file_path.exists():
              raise EditBlockError('File not found.')
          if not file_path.is_file():
              raise EditBlockError('Not a regular file.')
          text = file_path.read_text(encoding='utf-8')
          try:
              result_text = replace_in_block(
                  text,
                  item.old_text,
                  item.new_text,
                  exact=item.exact,
                  replace_all=item.replace_all,
                  accept=line_preserving(item.old_text),
                  max_level=2,
                  where='file')
          except TextMatchError as exc:
              raise EditBlockError(str(exc)) from exc
          try:
              file_path.write_text(result_text, encoding='utf-8')
          except OSError as exc:
              raise EditBlockError(f'Write failed: {exc}') from exc
          return EditBlockResult(path=item.path, result='success')


      def edit_block(items: list[EditBlockItem]) -> EditBlockBatchResult:
          """Replace occurrence(s) of ``old_text`` with ``new_text`` in one or more files.

          Matching escalates through whitespace/escape/quote tolerance; a candidate is
          only accepted when it preserves ``old_text``'s line structure, so no two lines
          are merged into a syntax error.

          Args:
              items: Block edits to apply. Must be non-empty.

          Returns:
              EditBlockBatchResult: one result per successfully applied edit, one error per failed edit.

          Raises:
              EditBlockError: If items is empty.
          """
          if not items:
              raise EditBlockError("'items' must be a non-empty list.")
          results: list[EditBlockResult] = []
          errors: list[EditBlockItemError] = []
          for item in items:
              try:
                  results.append(_edit_block_one(item))
              except EditBlockError as exc:
                  errors.append(EditBlockItemError(path=item.path, error=str(exc)))
          return EditBlockBatchResult(results=results, errors=errors)


      class EditBlockTool(ToolDefinition):
          name = 'edit_block'
          title = 'Replace text in file'
          description = "Replace a short text inside one or more files, for a batch of items. 'old_text' must occur exactly once, unless 'replaceAll' is set. By default whitespace (spaces, tabs, newlines) is matched tolerantly; set 'exact' to require exact whitespace matching."
          input_schema = {
              'type': 'object',
              'properties': {
                  'items': {
                      'type': 'array',
                      'minItems': 1,
                      'items': {
                          'type': 'object',
                          'additionalProperties': False,
                          'properties': {
                              'path': {'type': 'string', 'description': 'Absolute path to the target file.'},
                              'old_text': {
                                  'type': 'string',
                                  'minLength': 10,
                                  'maxLength': 100,
                                  'description': 'Text (10-100 chars) to find and replace. Must occur exactly once, unless replaceAll is set.'},
                              'new_text': {
                                  'type': 'string',
                                  'description': "Text that replaces 'old_text' (empty to perform a deletion)."},
                              'exact': {
                                  'type': 'boolean',
                                  'description': "If true, 'old_text' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                                  'default': False},
                              'replaceAll': {
                                  'type': 'boolean',
                                  'description': "If true, replace every occurrence of 'old_text' instead of requiring a single unique match. Defaults to false.",
                                  'default': False}},
                          'required': ['path', 'old_text', 'new_text']},
                      'description': 'Block edits to apply.'}},
              'required': ['items']}
          output_schema = {
              'type': 'object',
              'properties': {
                  'results': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {'path': {'type': 'string'}, 'result': {'type': 'string'}},
                          'required': ['path', 'result']}},
                  'errors': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {'path': {'type': 'string'}, 'error': {'type': 'string'}},
                          'required': ['path', 'error']}}},
              'required': ['results', 'errors']}
          annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`edit_block`, translating the MCP schema to/from the Python API."""
              args: dict[str, Any] = ctx.arguments
              raw_items = args.get('items') or []
              if not raw_items:
                  return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
              items = [
                  EditBlockItem(
                      path=it['path'],
                      old_text=it['old_text'],
                      new_text=it['new_text'],
                      exact=it.get('exact', False),
                      replace_all=it.get('replaceAll', False)) for it in raw_items]
              batch = edit_block(items)
              results = [{'path': r.path, 'result': r.result} for r in batch.results]
              errors = [{'path': e.path, 'error': e.error} for e in batch.errors]
              is_error = bool(batch.errors) and not batch.results
              return ToolResult(structured_content={'results': results, 'errors': errors}, is_error=is_error, auto_approve=not is_error)


      def register_edit_block_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
          registry.register(EditBlockTool())
          functions.register(edit_block)
  reason: edit_block-Tool auf Batch-Support umstellen
```
/answer 95d8-57 allow
Control Request:
```yaml
id: 95d8-58
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py
    overwrite: true
    source: |
      """Edit Marks tool – replaces the text strictly including two markers, for a batch of items."""

      from dataclasses import dataclass
      from pathlib import Path
      from typing import Any

      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools._text_match import replace_between, marks_line_preserving, TextMatchError
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry

      __all__ = [
          'EditMarksError',
          'EditMarksItem',
          'EditMarksResult',
          'EditMarksItemError',
          'EditMarksBatchResult',
          'edit_marks',
          'edit_marks_text',
          'EditMarksTool',
          'register_edit_marks_tool']


      class EditMarksError(Exception):
          """Raised when a replace operation cannot be performed."""


      @dataclass(frozen=True)
      class EditMarksItem:
          """One marker-based replacement to apply.

          Attributes:
              path: Absolute path to target file.
              begin_marker: Unique substring marking the beginning of the block.
              end_marker: Unique substring marking the end of the block.
              content: Replacement text.
              exact: If False (default), whitespace in start/end is matched tolerantly. If True, whitespace must match exactly.
          """
          path: str
          begin_marker: str
          end_marker: str
          content: str
          exact: bool = False


      @dataclass(frozen=True)
      class EditMarksResult:
          """Result of applying a single marker-based replacement, mirroring its input path for result association."""
          path: str
          result: str


      @dataclass(frozen=True)
      class EditMarksItemError:
          """Error applying a single marker-based replacement, mirroring its input path for result association."""
          path: str
          error: str


      @dataclass(frozen=True)
      class EditMarksBatchResult:
          """Result of :func:`edit_marks`.

          Attributes:
              results: One :class:`EditMarksResult` per successfully applied edit.
              errors: One :class:`EditMarksItemError` per edit that failed.
          """
          results: list[EditMarksResult]
          errors: list[EditMarksItemError]


      def edit_marks_text(text: str, begin_marker: str, content: str, end_marker: str, exact: bool=False) -> str:
          """Replace everything between and including 'begin_marker' and 'end_marker' with content, in *text*.

          Both markers are included in the replacement. Matching escalates through
          whitespace/escape/quote tolerance; each marker match must keep its own line
          count, so tolerant matching never merges lines into a syntax error.

          Args:
              text: Source text to edit.
              begin_marker: Unique substring marking the beginning of the block.
              content: Replacement text.
              end_marker: Unique substring marking the end of the block.
              exact: If False (default), whitespace in start/end is matched tolerantly. If True, whitespace must match exactly.

          Returns:
              The edited text.

          Raises:
              EditMarksError: If start or end markers are not found or appear more than once.
              EditMarksError: If end marker does not start after start marker ends.
          """
          begin, end = (begin_marker, end_marker) if exact else (begin_marker.strip(), end_marker.strip())
          try:
              return replace_between(
                  text,
                  begin,
                  end,
                  content,
                  exact=exact,
                  accept=marks_line_preserving(
                      begin,
                      end),
                  max_level=2,
                  where='file')
          except TextMatchError as exc:
              raise EditMarksError(str(exc)) from exc


      def _edit_marks_one(item: EditMarksItem) -> EditMarksResult:
          file_path = Path(item.path)
          if not file_path.is_absolute():
              raise EditMarksError('Path must be absolute.')
          if not file_path.exists():
              raise EditMarksError('File not found.')
          if not file_path.is_file():
              raise EditMarksError('Not a regular file.')
          text = file_path.read_text(encoding='utf-8')
          result_text = edit_marks_text(text, item.begin_marker, item.content, item.end_marker, exact=item.exact)
          try:
              file_path.write_text(result_text, encoding='utf-8')
          except OSError as exc:
              raise EditMarksError(f'Write failed: {exc}') from exc
          return EditMarksResult(path=item.path, result='success')


      def edit_marks(items: list[EditMarksItem]) -> EditMarksBatchResult:
          """Replace everything between and including 'begin_marker' and 'end_marker' with content, in one or more files.

          Both markers are included in the replacement.

          Args:
              items: Marker-based edits to apply. Must be non-empty.

          Returns:
              EditMarksBatchResult: one result per successfully applied edit, one error per failed edit.

          Raises:
              EditMarksError: If items is empty.
          """
          if not items:
              raise EditMarksError("'items' must be a non-empty list.")
          results: list[EditMarksResult] = []
          errors: list[EditMarksItemError] = []
          for item in items:
              try:
                  results.append(_edit_marks_one(item))
              except EditMarksError as exc:
                  errors.append(EditMarksItemError(path=item.path, error=str(exc)))
          return EditMarksBatchResult(results=results, errors=errors)


      class EditMarksTool(ToolDefinition):
          name = 'edit_marks'
          title = 'Replace text between two marks'
          description = "Replace everything between and including the unique 'begin_marker' and 'end_marker' markers, found in one or more files, with new 'content', for a batch of items."
          input_schema = {
              'type': 'object',
              'properties': {
                  'items': {
                      'type': 'array',
                      'minItems': 1,
                      'items': {
                          'type': 'object',
                          'additionalProperties': False,
                          'properties': {
                              'path': {'type': 'string', 'description': 'Absolute path to the target file.'},
                              'begin_marker': {
                                  'type': 'string',
                                  'minLength': 10,
                                  'maxLength': 30,
                                  'description': 'Unique 10-30 char substring marking the beginning of the text to replace.'},
                              'content': {'type': 'string', 'description': 'Replacement source for the marked text.'},
                              'end_marker': {
                                  'type': 'string',
                                  'minLength': 10,
                                  'maxLength': 30,
                                  'description': 'Unique 10-30 char substring marking the end of the text to replace.'},
                              'exact': {
                                  'type': 'boolean',
                                  'description': "If true, 'begin_marker'/'end_marker' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                                  'default': False}},
                          'required': ['path', 'begin_marker', 'end_marker', 'content']},
                      'description': 'Marker-based edits to apply.'}},
              'required': ['items']}
          output_schema = {
              'type': 'object',
              'properties': {
                  'results': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {'path': {'type': 'string'}, 'result': {'type': 'string', 'description': '``success`` on success.'}},
                          'required': ['path', 'result']}},
                  'errors': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {'path': {'type': 'string'}, 'error': {'type': 'string'}},
                          'required': ['path', 'error']}}},
              'required': ['results', 'errors']}
          annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`edit_marks`, translating the MCP schema to/from the Python API."""
              args: dict[str, Any] = ctx.arguments
              raw_items = args.get('items') or []
              if not raw_items:
                  return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
              items = [
                  EditMarksItem(
                      path=it['path'],
                      begin_marker=it['begin_marker'],
                      end_marker=it['end_marker'],
                      content=it['content'],
                      exact=it.get('exact', False)) for it in raw_items]
              batch = edit_marks(items)
              results = [{'path': r.path, 'result': r.result} for r in batch.results]
              errors = [{'path': e.path, 'error': e.error} for e in batch.errors]
              is_error = bool(batch.errors) and not batch.results
              return ToolResult(structured_content={'results': results, 'errors': errors}, is_error=is_error, auto_approve=not is_error)


      def register_edit_marks_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
          registry.register(EditMarksTool())
          functions.register(edit_marks)
  reason: edit_marks-Tool auf Batch-Support umstellen
```
/answer 95d8-58 allow
Control Request:
```yaml
id: 95d8-59
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_line/__init__.py
    overwrite: true
    source: |
      """Edit-line tool – replaces a single line with one or more lines in a file, for a batch of items."""

      from dataclasses import dataclass
      from typing import Any

      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.edit_block import EditBlockItem, edit_block
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry

      __all__ = [
          'EditLineError',
          'EditLineItem',
          'EditLineResult',
          'EditLineItemError',
          'EditLineBatchResult',
          'edit_line',
          'EditLineTool',
          'register_edit_line_tool']


      class EditLineError(Exception):
          """Raised when an edit-line operation cannot be performed."""


      @dataclass(frozen=True)
      class EditLineItem:
          """One single-line edit to apply.

          Attributes:
              path: Absolute path to file (must be a regular file).
              old_line: The single line to find and replace (must occur exactly once,
                        unless replace_all). Must not contain a newline character.
              new_lines: Replacement content; either a single line or multiple lines
                         (joined with '\\n') to replace old_line with.
              exact: If False (default), whitespace in old_line is matched tolerantly.
                     If True, whitespace must match exactly.
              replace_all: If True, replace every occurrence of old_line instead of
                           requiring a single unique match.
          """
          path: str
          old_line: str
          new_lines: str
          exact: bool = False
          replace_all: bool = False


      @dataclass(frozen=True)
      class EditLineResult:
          """Result of applying a single line edit, mirroring its input path for result association."""
          path: str
          result: str


      @dataclass(frozen=True)
      class EditLineItemError:
          """Error applying a single line edit, mirroring its input path for result association."""
          path: str
          error: str


      @dataclass(frozen=True)
      class EditLineBatchResult:
          """Result of :func:`edit_line`.

          Attributes:
              results: One :class:`EditLineResult` per successfully applied edit.
              errors: One :class:`EditLineItemError` per edit that failed.
          """
          results: list[EditLineResult]
          errors: list[EditLineItemError]


      def _edit_line_one(item: EditLineItem) -> EditLineResult:
          if '\n' in item.old_line or '\r' in item.old_line:
              raise EditLineError('old_line must be a single line without newline characters.')
          block_item = EditBlockItem(
              path=item.path,
              old_text=item.old_line,
              new_text=item.new_lines,
              exact=item.exact,
              replace_all=item.replace_all)
          batch = edit_block([block_item])
          if batch.errors:
              raise EditLineError(batch.errors[0].error)
          return EditLineResult(path=item.path, result=batch.results[0].result)


      def edit_line(items: list[EditLineItem]) -> EditLineBatchResult:
          """Replace a single line with one or more lines, in one or more files.

          Delegates to :func:`edit_block`; ``old_line`` must be a single line (no newline
          characters). ``new_lines`` may be a single line or multiple lines joined by ``\\n``.

          Args:
              items: Single-line edits to apply. Must be non-empty.

          Returns:
              EditLineBatchResult: one result per successfully applied edit, one error per failed edit.

          Raises:
              EditLineError: If items is empty.
          """
          if not items:
              raise EditLineError("'items' must be a non-empty list.")
          results: list[EditLineResult] = []
          errors: list[EditLineItemError] = []
          for item in items:
              try:
                  results.append(_edit_line_one(item))
              except EditLineError as exc:
                  errors.append(EditLineItemError(path=item.path, error=str(exc)))
          return EditLineBatchResult(results=results, errors=errors)


      class EditLineTool(ToolDefinition):
          name = 'edit_line'
          title = 'Replace a single line in file'
          description = "Replace exactly one line inside one or more files with one or more lines, for a batch of items. 'old_line' must be a single line without a newline character. Choose 'old_line' to be unique within the file. 'old_line' must occur exactly once, unless 'replaceAll' is set. By default whitespace is matched tolerantly; set 'exact' to require exact whitespace matching."
          input_schema = {
              'type': 'object',
              'properties': {
                  'items': {
                      'type': 'array',
                      'minItems': 1,
                      'items': {
                          'type': 'object',
                          'additionalProperties': False,
                          'properties': {
                              'path': {'type': 'string', 'description': 'Absolute path to the target file.'},
                              'old_line': {
                                  'type': 'string',
                                  'description': 'A single line to find and replace without a newline character. Keep it short and distinct enough to match uniquely. Must occur exactly once unless replaceAll is set. Never pass multiple lines here.'},
                              'new_lines': {
                                  'type': 'string',
                                  'description': "Replacement content for 'old_line', may be empty to delete the line."},
                              'exact': {
                                  'type': 'boolean',
                                  'description': "If true, 'old_line' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                                  'default': False},
                              'replaceAll': {
                                  'type': 'boolean',
                                  'description': "If true, replace every occurrence of 'old_line' instead of requiring a single unique match. Defaults to false.",
                                  'default': False}},
                          'required': ['path', 'old_line', 'new_lines']},
                      'description': 'Single-line edits to apply.'}},
              'required': ['items']}
          output_schema = {
              'type': 'object',
              'properties': {
                  'results': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {'path': {'type': 'string'}, 'result': {'type': 'string'}},
                          'required': ['path', 'result']}},
                  'errors': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {'path': {'type': 'string'}, 'error': {'type': 'string'}},
                          'required': ['path', 'error']}}},
              'required': ['results', 'errors']}
          annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`edit_line`, translating the MCP schema to/from the Python API."""
              args: dict[str, Any] = ctx.arguments
              raw_items = args.get('items') or []
              if not raw_items:
                  return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
              items = [
                  EditLineItem(
                      path=it['path'],
                      old_line=it['old_line'],
                      new_lines=it['new_lines'],
                      exact=it.get('exact', False),
                      replace_all=it.get('replaceAll', False)) for it in raw_items]
              batch = edit_line(items)
              results = [{'path': r.path, 'result': r.result} for r in batch.results]
              errors = [{'path': e.path, 'error': e.error} for e in batch.errors]
              is_error = bool(batch.errors) and not batch.results
              return ToolResult(structured_content={'results': results, 'errors': errors}, is_error=is_error, auto_approve=not is_error)


      def register_edit_line_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
          registry.register(EditLineTool())
          functions.register(edit_line)
  reason: edit_line-Tool auf Batch-Support umstellen
```
/answer 95d8-59 allow
Control Request:
```yaml
id: 95d8-5a
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_lines/__init__.py
    overwrite: true
    source: |
      """Edit-lines tool – replaces a range of lines inside existing files, for a batch of items."""

      from dataclasses import dataclass
      from pathlib import Path
      from typing import Any

      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry

      __all__ = [
          'EditLinesError',
          'EditLinesItem',
          'EditLinesResult',
          'EditLinesItemError',
          'EditLinesBatchResult',
          'edit_lines',
          'EditLinesTool',
          'register_edit_lines_tool']


      class EditLinesError(Exception):
          """Raised when a edit-lines operation cannot be performed."""


      @dataclass(frozen=True)
      class EditLinesItem:
          """One line-range replacement to apply.

          Attributes:
              path: Absolute path to file (must be a regular file).
              offset: Zero-based line offset where to start replacement (must be >= 0).
              amount: Number of lines to replace (must be >= 0).
              content: Replacement text (should include its own trailing newline if a line break is wanted).
          """
          path: str
          offset: int
          amount: int
          content: str


      @dataclass(frozen=True)
      class EditLinesResult:
          """Result of applying a single line-range replacement, mirroring its input path for result association."""
          path: str
          result: str


      @dataclass(frozen=True)
      class EditLinesItemError:
          """Error applying a single line-range replacement, mirroring its input path for result association."""
          path: str
          error: str


      @dataclass(frozen=True)
      class EditLinesBatchResult:
          """Result of :func:`edit_lines`.

          Attributes:
              results: One :class:`EditLinesResult` per successfully applied edit.
              errors: One :class:`EditLinesItemError` per edit that failed.
          """
          results: list[EditLinesResult]
          errors: list[EditLinesItemError]


      def _edit_lines_one(item: EditLinesItem) -> EditLinesResult:
          file_path = Path(item.path)
          if not file_path.is_absolute():
              raise EditLinesError('Path must be absolute.')
          if not file_path.exists():
              raise EditLinesError('File not found.')
          if not file_path.is_file():
              raise EditLinesError('Not a regular file.')
          try:
              text = file_path.read_text(encoding='utf-8')
              lines = text.splitlines(keepends=True)
              if item.offset < 0 or item.offset > len(lines):
                  raise EditLinesError('Offset is out of bounds.')
              if item.amount < 0 or item.offset + item.amount > len(lines):
                  raise EditLinesError('Amount is out of bounds.')
              new_lines = lines[:item.offset] + [item.content] + lines[item.offset + item.amount:]
              new_text = ''.join(new_lines)
              file_path.write_text(new_text, encoding='utf-8')
          except OSError as exc:
              raise EditLinesError(f'Replace failed: {exc}') from exc
          return EditLinesResult(path=item.path, result='success')


      def edit_lines(items: list[EditLinesItem]) -> EditLinesBatchResult:
          """Replace a range of lines in one or more files, for a batch of items.

          A given path may only be edited once per batch: since offsets can shift after
          each edit, every further item targeting an already-processed path fails.

          Args:
              items: Line-range replacements to apply. Must be non-empty.

          Returns:
              EditLinesBatchResult: one result per successfully applied edit, one error per failed edit.

          Raises:
              EditLinesError: If items is empty.

          Note:
              Lines are 0-based. content may be empty to perform pure deletion.
          """
          if not items:
              raise EditLinesError("'items' must be a non-empty list.")
          results: list[EditLinesResult] = []
          errors: list[EditLinesItemError] = []
          seen: set[str] = set()
          for item in items:
              if item.path in seen:
                  errors.append(EditLinesItemError(
                      path=item.path,
                      error='Duplicate path in this batch; edit_lines allows only one edit per path per batch because offsets can shift after a prior edit.'))
                  continue
              seen.add(item.path)
              try:
                  results.append(_edit_lines_one(item))
              except EditLinesError as exc:
                  errors.append(EditLinesItemError(path=item.path, error=str(exc)))
          return EditLinesBatchResult(results=results, errors=errors)


      class EditLinesTool(ToolDefinition):
          name = 'edit_lines'
          title = 'Replace lines in file by line offsets'
          description = 'Replace a range of lines inside one or more existing files with new content, for a batch of items. The range is defined by a zero-based line ``offset`` and an ``amount`` (number of lines to remove starting at the offset). The supplied ``content`` is written in place of the removed lines; it should include its own trailing newline if a line break is wanted. A given path may only be edited once per batch, since offsets can shift after a prior edit.'
          input_schema = {
              'type': 'object',
              'properties': {
                  'items': {
                      'type': 'array',
                      'minItems': 1,
                      'items': {
                          'type': 'object',
                          'additionalProperties': False,
                          'properties': {
                              'path': {'type': 'string', 'description': 'Absolute path to the file to modify.'},
                              'offset': {'type': 'integer', 'description': 'Zero-based line offset of the first line to replace.', 'minimum': 0},
                              'amount': {'type': 'integer', 'description': 'Number of lines to remove starting at ``offset``.', 'minimum': 0},
                              'content': {'type': 'string', 'description': 'Replacement text (may be empty to perform a pure deletion).'}},
                          'required': ['path', 'offset', 'amount', 'content']},
                      'description': 'Line-range replacements to apply. A given path may only appear once.'}},
              'required': ['items']}
          output_schema = {
              'type': 'object',
              'properties': {
                  'results': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {'path': {'type': 'string'}, 'result': {'type': 'string', 'description': '``success`` on success.'}},
                          'required': ['path', 'result']}},
                  'errors': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {'path': {'type': 'string'}, 'error': {'type': 'string'}},
                          'required': ['path', 'error']}}},
              'required': ['results', 'errors']}
          annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`edit_lines`, translating the MCP schema to/from the Python API."""
              args: dict[str, Any] = ctx.arguments
              raw_items = args.get('items') or []
              if not raw_items:
                  return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
              items = [
                  EditLinesItem(path=it['path'], offset=it['offset'], amount=it['amount'], content=it['content'])
                  for it in raw_items]
              batch = edit_lines(items)
              results = [{'path': r.path, 'result': r.result} for r in batch.results]
              errors = [{'path': e.path, 'error': e.error} for e in batch.errors]
              is_error = bool(batch.errors) and not batch.results
              return ToolResult(structured_content={'results': results, 'errors': errors}, is_error=is_error, auto_approve=not is_error)


      def register_edit_lines_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
          registry.register(EditLinesTool())
          functions.register(edit_lines)
  reason: edit_lines-Tool auf Batch-Support umstellen inkl. Duplikat-Pfad-Schutz
```
/answer 95d8-5a allow
ReasoningToken: 1211
Control Request:
```yaml
id: 95d8-5b
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
    overwrite: true
    source: |
      """Read tool – returns file contents, optionally sliced by line, character offset, or marker, for a batch of items."""

      import hashlib
      import json
      from dataclasses import dataclass
      from pathlib import Path
      from typing import Any

      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry

      __all__ = [
          'ReadError',
          'ReadItem',
          'ReadResult',
          'ReadItemError',
          'ReadBatchResult',
          'read_file',
          'ReadTool',
          'register_read_tool']
      _CACHE_STATE_KEY = '_read_cache'


      class ReadError(Exception):
          """Raised when a file cannot be read or the requested range is invalid."""


      @dataclass(frozen=True)
      class ReadItem:
          """One file (optionally sliced to a range) to read.

          Attributes:
              path: Absolute path to file (must exist and be readable).
              min_line: Range start as line number, inclusive, 1-based. Mutually exclusive with
                        min_char and start.
              max_line: Range end as line number, inclusive, 1-based. Mutually exclusive with
                        max_char and end.
              min_char: Range start as character offset, inclusive, 0-based. Mutually exclusive with
                        min_line and start.
              max_char: Range end as character offset, exclusive, 0-based. Mutually exclusive with
                        max_line and end.
              start: Range start as unique substring marker (inclusive). Mutually exclusive with
                     min_line and min_char. Marker must occur exactly once in file.
              end: Range end as unique substring marker (inclusive). Mutually exclusive with
                   max_line and max_char. Marker must occur exactly once in file.
          """
          path: str
          min_line: int | None = None
          max_line: int | None = None
          min_char: int | None = None
          max_char: int | None = None
          start: str | None = None
          end: str | None = None


      @dataclass(frozen=True)
      class ReadResult:
          """Result of reading a single file, mirroring its input path for result association."""
          path: str
          content: str
          checksum: str
          is_full_file: bool


      @dataclass(frozen=True)
      class ReadItemError:
          """Error reading a single file, mirroring its input path for result association."""
          path: str
          error: str


      @dataclass(frozen=True)
      class ReadBatchResult:
          """Result of :func:`read_file`.

          Attributes:
              results: One :class:`ReadResult` per successfully read file.
              errors: One :class:`ReadItemError` per file that failed.
          """
          results: list[ReadResult]
          errors: list[ReadItemError]


      def _cache_key(session_id: str, item: ReadItem) -> str:
          payload = json.dumps({'session': session_id, 'item': item.__dict__}, sort_keys=True)
          return hashlib.sha256(payload.encode('utf-8')).hexdigest()


      def _read_one(item: ReadItem) -> ReadResult:
          if item.min_line is not None and item.min_char is not None:
              raise ReadError('``min_line`` and ``min_char`` are mutually exclusive.')
          if item.max_line is not None and item.max_char is not None:
              raise ReadError('``max_line`` and ``max_char`` are mutually exclusive.')
          if item.min_line is not None and item.start is not None:
              raise ReadError('``min_line`` and ``start`` are mutually exclusive.')
          if item.min_char is not None and item.start is not None:
              raise ReadError('``min_char`` and ``start`` are mutually exclusive.')
          if item.max_line is not None and item.end is not None:
              raise ReadError('``max_line`` and ``end`` are mutually exclusive.')
          if item.max_char is not None and item.end is not None:
              raise ReadError('``max_char`` and ``end`` are mutually exclusive.')
          file_path = Path(item.path)
          if not file_path.is_absolute():
              raise ReadError('Path must be absolute.')
          if not file_path.exists():
              raise ReadError('File not found.')
          if not file_path.is_file():
              raise ReadError("Not a regular file. Don't read directories with this tool!")
          raw_bytes = file_path.read_bytes()
          text = raw_bytes.decode('utf-8', errors='replace')
          lines = text.splitlines(keepends=True)
          total_lines = len(lines)

          def line_start_offset(line_num: int) -> int:
              n = max(0, min(line_num - 1, total_lines))
              return sum((len(l) for l in lines[:n]))

          def line_end_offset(line_num: int) -> int:
              n = max(0, min(line_num, total_lines))
              return sum((len(l) for l in lines[:n]))
          if item.start is not None:
              start_count = text.count(item.start)
              if start_count == 0:
                  raise ReadError('Start marker not found in file.')
              if start_count > 1:
                  raise ReadError(f'Start marker is ambiguous – found {start_count} occurrences in file.')
              region_start = text.index(item.start)
          elif item.min_line is not None:
              region_start = line_start_offset(item.min_line)
          elif item.min_char is not None:
              region_start = item.min_char
          else:
              region_start = 0
          if item.end is not None:
              end_count = text.count(item.end)
              if end_count == 0:
                  raise ReadError('End marker not found in file.')
              if end_count > 1:
                  raise ReadError(f'End marker is ambiguous – found {end_count} occurrences in file.')
              region_end = text.index(item.end) + len(item.end)
          elif item.max_line is not None:
              region_end = line_end_offset(item.max_line)
          elif item.max_char is not None:
              region_end = item.max_char
          else:
              region_end = len(text)
          if region_end < region_start:
              raise ReadError('Resolved end position must not lie before the resolved start position.')
          is_full_file = item.min_line is None and item.max_line is None and (
              item.min_char is None) and (
                  item.max_char is None) and (
                      item.start is None) and (
                          item.end is None)
          if not is_full_file and len(text) and (region_end - region_start > 0.7 * len(text)):
              raise ReadError('The requested range selects more than 70% of the file. Read the whole file instead (omit the range parameters) and rely on the checksum-based conditional read to detect unchanged content.')
          sliced = text[region_start:region_end]
          checksum = hashlib.sha256(sliced.encode('utf-8')).hexdigest()
          return ReadResult(path=item.path, content=sliced, checksum=checksum, is_full_file=is_full_file)


      def read_file(items: list[ReadItem]) -> ReadBatchResult:
          """Read one or more files, each optionally sliced to a range.

          Args:
              items: Files to read. Must be non-empty.

          Returns:
              ReadBatchResult: one result per successfully read file, one error per failed file.

          Raises:
              ReadError: If items is empty.

          Note:
              Line numbering is 1-based (first line is 1). Character offsets are 0-based.
          """
          if not items:
              raise ReadError("'items' must be a non-empty list.")
          results: list[ReadResult] = []
          errors: list[ReadItemError] = []
          for item in items:
              try:
                  results.append(_read_one(item))
              except ReadError as exc:
                  errors.append(ReadItemError(path=item.path, error=str(exc)))
          return ReadBatchResult(results=results, errors=errors)


      class ReadTool(ToolDefinition):
          name = 'read_file'
          title = 'Read file content'
          description = 'Read one or more files as text, optionally sliced to a range, for a batch of items.'
          input_schema = {
              'type': 'object',
              'properties': {
                  'items': {
                      'type': 'array',
                      'minItems': 1,
                      'items': {
                          'type': 'object',
                          'additionalProperties': False,
                          'properties': {
                              'path': {'type': 'string', 'description': 'Absolute file path.'},
                              'min_line': {'type': 'integer', 'description': 'Range start: line number, inclusive, 1-based. Excludes start and min_char.', 'minimum': 1},
                              'max_line': {'type': 'integer', 'description': 'Range end: line number, inclusive, 1-based. Excludes end and max_char.', 'minimum': 1},
                              'min_char': {'type': 'integer', 'description': 'Range start: character offset, inclusive, 0-based. Excludes min_line.', 'minimum': 0},
                              'max_char': {'type': 'integer', 'description': 'Range end: character offset, exclusive, 0-based. Excludes max_line.', 'minimum': 0},
                              'start': {'type': 'string', 'description': 'Range start: unique marker substring, inclusive. Excludes min_line and min_char.'},
                              'end': {'type': 'string', 'description': 'Range end: unique marker substring, inclusive. Excludes max_line and max_char.'}},
                          'required': ['path']},
                      'description': 'Files to read.'}},
              'required': ['items']}
          output_schema = {
              'type': 'object',
              'properties': {
                  'results': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {
                              'path': {'type': 'string'},
                              'content': {'type': 'string'},
                              'checksum': {'type': 'string', 'description': 'sha256 checksum of the read content.'},
                              'unchanged': {'type': 'boolean', 'description': 'True if the content is identical to a previous read with the same parameters.'}},
                          'required': ['path', 'checksum']}},
                  'errors': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {'path': {'type': 'string'}, 'error': {'type': 'string'}},
                          'required': ['path', 'error']}}},
              'required': ['results', 'errors']}
          annotations = {'readOnlyHint': True, 'openWorldHint': False}

          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`read_file`, then apply session-level change detection and MCP packing."""
              args: dict[str, Any] = ctx.arguments
              raw_items = args.get('items') or []
              if not raw_items:
                  return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
              items = [
                  ReadItem(
                      path=it['path'],
                      min_line=it.get('min_line'),
                      max_line=it.get('max_line'),
                      min_char=it.get('min_char'),
                      max_char=it.get('max_char'),
                      start=it.get('start'),
                      end=it.get('end')) for it in raw_items]
              batch = read_file(items)
              session = ctx.session
              results: list[dict[str, Any]] = []
              content: list[dict[str, Any]] = []
              all_full_file = True
              with session.lock:
                  cache: dict[str, str] = session.state.setdefault(_CACHE_STATE_KEY, {})
                  for item, result in zip(items, batch.results):
                      key = _cache_key(session.id, item)
                      previous_checksum = cache.get(key)
                      cache[key] = result.checksum
                      unchanged = previous_checksum == result.checksum
                      entry: dict[str, Any] = {'path': result.path, 'checksum': result.checksum}
                      if unchanged:
                          entry['unchanged'] = True
                          content.append(text_content(f'{result.path}: content unchanged since the last identical read. Use the former read result.'))
                      else:
                          entry['content'] = result.content
                      if not result.is_full_file:
                          all_full_file = False
                      results.append(entry)
              errors = [{'path': e.path, 'error': e.error} for e in batch.errors]
              is_error = bool(batch.errors) and not batch.results
              return ToolResult(
                  content=content,
                  structured_content={'results': results, 'errors': errors},
                  is_error=is_error,
                  auto_approve=(not is_error) and all_full_file)


      def register_read_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
          registry.register(ReadTool())
          functions.register(read_file)
  reason: read-Tool auf Batch-Support umstellen
```
/answer 95d8-5b allow
Control Request:
```yaml
id: 95d8-5c
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
    overwrite: true
    source: |
      """Write tool – writes files completely or appends content to them, for a batch of items."""

      from dataclasses import dataclass
      from pathlib import Path
      from typing import Any

      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry

      __all__ = [
          'WriteError',
          'WriteItem',
          'WriteResult',
          'WriteItemError',
          'WriteBatchResult',
          'write',
          'WriteTool',
          'register_write_tool']


      class WriteError(Exception):
          """Raised when a write operation cannot be performed."""


      @dataclass(frozen=True)
      class WriteItem:
          """One file to write.

          Attributes:
              path: Absolute path to file to write (created if not exists).
              mode: Write mode: "replace" overwrites entire file, "append" adds content at end.
              content: Text content to write.
          """
          path: str
          mode: str
          content: str


      @dataclass(frozen=True)
      class WriteResult:
          """Result of writing a single file, mirroring its input path for result association."""
          path: str
          result: str


      @dataclass(frozen=True)
      class WriteItemError:
          """Error writing a single file, mirroring its input path for result association."""
          path: str
          error: str


      @dataclass(frozen=True)
      class WriteBatchResult:
          """Result of :func:`write`.

          Attributes:
              results: One :class:`WriteResult` per successfully written file.
              errors: One :class:`WriteItemError` per file that failed.
          """
          results: list[WriteResult]
          errors: list[WriteItemError]


      def _write_one(item: WriteItem) -> WriteResult:
          file_path = Path(item.path)
          if not file_path.is_absolute():
              raise WriteError('Path must be absolute.')
          try:
              file_path.parent.mkdir(parents=True, exist_ok=True)
              file_mode = 'a' if item.mode == 'append' else 'w'
              with file_path.open(file_mode, encoding='utf-8') as fh:
                  fh.write(item.content)
          except OSError as exc:
              raise WriteError(f'Write failed: {exc}') from exc
          return WriteResult(path=item.path, result='success')


      def write(items: list[WriteItem]) -> WriteBatchResult:
          """Write or replace one or more target files, for a batch of items.

          Args:
              items: Files to write. Must be non-empty.

          Returns:
              WriteBatchResult: one result per successfully written file, one error per failed file.

          Raises:
              WriteError: If items is empty.
          """
          if not items:
              raise WriteError("'items' must be a non-empty list.")
          results: list[WriteResult] = []
          errors: list[WriteItemError] = []
          for item in items:
              try:
                  results.append(_write_one(item))
              except WriteError as exc:
                  errors.append(WriteItemError(path=item.path, error=str(exc)))
          return WriteBatchResult(results=results, errors=errors)


      class WriteTool(ToolDefinition):
          name = 'write'
          title = 'Write file'
          description = 'Write content to one or more files, for a batch of items. In ``replace`` mode a file is overwritten with the supplied content. In ``append`` mode the content is added at the end of the existing file (the file is created if it does not yet exist).'
          input_schema = {
              'type': 'object',
              'properties': {
                  'items': {
                      'type': 'array',
                      'minItems': 1,
                      'items': {
                          'type': 'object',
                          'additionalProperties': False,
                          'properties': {
                              'path': {'type': 'string', 'description': 'Absolute path to the file to write.'},
                              'mode': {
                                  'type': 'string',
                                  'enum': ['replace', 'append'],
                                  'description': '``replace`` – overwrite the file with the new content. ``append`` – add the new content after the existing content.'},
                              'content': {'type': 'string', 'description': 'Text to write to the file.'}},
                          'required': ['path', 'mode', 'content']},
                      'description': 'Files to write or replace.'}},
              'required': ['items']}
          output_schema = {
              'type': 'object',
              'properties': {
                  'results': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {'path': {'type': 'string'}, 'result': {'type': 'string', 'description': '``success`` on success.'}},
                          'required': ['path', 'result']}},
                  'errors': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {'path': {'type': 'string'}, 'error': {'type': 'string'}},
                          'required': ['path', 'error']}}},
              'required': ['results', 'errors']}
          annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`write`, translating the MCP schema to/from the Python API."""
              args: dict[str, Any] = ctx.arguments
              raw_items = args.get('items') or []
              if not raw_items:
                  return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
              items = [WriteItem(path=it['path'], mode=it['mode'], content=it['content']) for it in raw_items]
              batch = write(items)
              results = [{'path': r.path, 'result': r.result} for r in batch.results]
              errors = [{'path': e.path, 'error': e.error} for e in batch.errors]
              is_error = bool(batch.errors) and not batch.results
              return ToolResult(structured_content={'results': results, 'errors': errors}, is_error=is_error, auto_approve=not is_error)


      def register_write_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
          registry.register(WriteTool())
          functions.register(write)
  reason: write-Tool auf Batch-Support umstellen
```
/answer 95d8-5c allow
Control Request:
```yaml
id: 95d8-5d
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
    overwrite: true
    source: |
      """Skills – on-demand hints an agent can request, for a batch of items.

      A *skill* is a small, easy-to-maintain unit combining metadata (name, title,
      description, hint) with a real, named module-level function backing it as
      ``core`` (``FunctionRegistry`` requires actually existing functions with a
      real signature/docstring). All skills are exposed through a single batching
      tool so several skills can be fetched in one call.
      """

      from abc import ABC, abstractmethod
      from dataclasses import dataclass
      from typing import Any

      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
      from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry

      __all__ = [
          'Skill',
          'SKILLS',
          'MarkdownFormatSkill',
          'SkillsError',
          'SkillsItem',
          'SkillsResult',
          'SkillsItemError',
          'SkillsBatchResult',
          'get_skills',
          'SkillsTool',
          'register_skills',
          'markdown_format']


      class Skill(ABC):
          """Metadata and backing function for one on-demand skill hint."""
          name: str
          title: str
          description: str
          hint: str

          @staticmethod
          @abstractmethod
          def core() -> str:
              """Real function backing this skill; also registered in the ``FunctionRegistry``."""


      def markdown_format() -> str:
          """Preferred formatting rules for Pandoc-compatible Markdown documents.

          Load when formatting rules are requested or required; apply proactively
          whenever creating, editing, or reviewing Markdown documents — even when
          formatting is not explicitly mentioned.

          Returns:
              The formatting instructions text.
          """
          return '* Use a line containing only `***` to insert a page break in PDF output.\n* Insert page breaks before top-level chapters (H1) at the start of each chapter.\n* Use `\\n---\\n` as a section separator before second-order chapters (H2) at the start of each chapter.\n* All files must end with an additional newline to prevent Markdown formatting errors on merge.\n* Use third-order headings and below only when necessary for navigation; use simple bold paragraph headings instead.\n* Chapter headings are numbered for H1–H3 only; lower-order headings do not contain numbering.\n* Use LaTeX (`$$`) for block mathematical expressions and inline LaTeX (`$`) for inline mathematical symbols, expressions, and formulas.'


      class MarkdownFormatSkill(Skill):
          name = 'markdown_format'
          title = 'markdown-format'
          description = 'Preferred formatting rules for Pandoc-compatible Markdown documents. Load when formatting rules are requested or required.'
          hint = 'Apply proactively whenever creating, editing, or reviewing Markdown documents — even when formatting is not explicitly mentioned.'
          core = staticmethod(markdown_format)


      #: All declared skills. Append here to add a new one.
      SKILLS: list[Skill] = [MarkdownFormatSkill()]


      class SkillsError(Exception):
          """Raised when a skill cannot be resolved."""


      @dataclass(frozen=True)
      class SkillsItem:
          """One skill to fetch.

          Attributes:
              name: Name of the skill to load.
          """
          name: str


      @dataclass(frozen=True)
      class SkillsResult:
          """Result of fetching a single skill, mirroring its input name for result association."""
          name: str
          instructions: str


      @dataclass(frozen=True)
      class SkillsItemError:
          """Error fetching a single skill, mirroring its input name for result association."""
          name: str
          error: str


      @dataclass(frozen=True)
      class SkillsBatchResult:
          """Result of :func:`get_skills`.

          Attributes:
              results: One :class:`SkillsResult` per successfully fetched skill.
              errors: One :class:`SkillsItemError` per skill that could not be resolved.
          """
          results: list[SkillsResult]
          errors: list[SkillsItemError]


      def _skill_by_name(name: str) -> Skill:
          for skill in SKILLS:
              if skill.name == name:
                  return skill
          raise SkillsError(f'Unknown skill: {name!r}')


      def get_skills(items: list[SkillsItem]) -> SkillsBatchResult:
          """Fetch the instructions of one or more skills, for a batch of items.

          Args:
              items: Skills to fetch. Must be non-empty.

          Returns:
              SkillsBatchResult: one result per successfully fetched skill, one error per unknown skill.

          Raises:
              SkillsError: If items is empty.
          """
          if not items:
              raise SkillsError("'items' must be a non-empty list.")
          results: list[SkillsResult] = []
          errors: list[SkillsItemError] = []
          for item in items:
              try:
                  skill = _skill_by_name(item.name)
                  results.append(SkillsResult(name=item.name, instructions=skill.core()))
              except SkillsError as exc:
                  errors.append(SkillsItemError(name=item.name, error=str(exc)))
          return SkillsBatchResult(results=results, errors=errors)


      class SkillsTool(ToolDefinition):
          name = 'skills'
          title = 'Load skills'
          input_schema = {
              'type': 'object',
              'properties': {
                  'items': {
                      'type': 'array',
                      'minItems': 1,
                      'items': {
                          'type': 'object',
                          'additionalProperties': False,
                          'properties': {
                              'name': {
                                  'type': 'string',
                                  'enum': [skill.name for skill in SKILLS],
                                  'description': 'Name of the skill to load.'}},
                          'required': ['name']},
                      'description': 'Skills to load.'}},
              'required': ['items']}
          output_schema = {
              'type': 'object',
              'properties': {
                  'results': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {'name': {'type': 'string'}, 'instructions': {'type': 'string'}},
                          'required': ['name', 'instructions']}},
                  'errors': {
                      'type': 'array',
                      'items': {
                          'type': 'object',
                          'properties': {'name': {'type': 'string'}, 'error': {'type': 'string'}},
                          'required': ['name', 'error']}}},
              'required': ['results', 'errors']}
          annotations = {'readOnlyHint': True, 'openWorldHint': False}

          def __init__(self) -> None:
              catalog = '\n'.join(f'- {skill.name}: {skill.description}' for skill in SKILLS)
              self.description = f'Load one or more on-demand skill hints, for a batch of items. Available skills:\n{catalog}'

          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`get_skills`, translating the MCP schema to/from the Python API."""
              args: dict[str, Any] = ctx.arguments
              raw_items = args.get('items') or []
              if not raw_items:
                  return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
              items = [SkillsItem(name=it['name']) for it in raw_items]
              batch = get_skills(items)
              results = [{'name': r.name, 'instructions': r.instructions} for r in batch.results]
              errors = [{'name': e.name, 'error': e.error} for e in batch.errors]
              is_error = bool(batch.errors) and not batch.results
              return ToolResult(structured_content={'results': results, 'errors': errors}, is_error=is_error, auto_approve=not is_error)


      def register_skills(registry: ToolRegistry, environment: AppEnvironment) -> None:
          """Register the batching skills tool and each skill's backing function."""
          registry.register(SkillsTool())
          for skill in SKILLS:
              environment.functions.register(skill.core)
          environment.functions.register(get_skills)
  reason: skills-Modul auf ein einzelnes Batch-Tool zum Laden mehrerer Skills umstellen
```
/answer 95d8-5d allow
Control Request:
```yaml
id: 95d8-5e
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file_stats/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_line/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_lines/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/skills/__init__.py
  reason: Syntaxprüfung aller neu geschriebenen Batch-Tool-Dateien
```
/answer 95d8-5e allow
Control Request:
```yaml
id: 95d8-5f
toolName: grep
arguments:
  directory:
  - /home/user/xyan/xy.ai.workbench/mcpc/src
  pattern: "(edit_block|edit_marks|edit_line|edit_lines|write|read_file|file_stats|grep)\\(path=|register_skills\\(|SKILLS\\[|MarkdownFormatSkill\\("
  include:
  - '*.py'
  exclude:
  - '*.min.py'
  - '**/tools/grep/**'
  - '**/tools/list/**'
  - '**/tools/file_stats/**'
  - '**/tools/edit_block/**'
  - '**/tools/edit_marks/**'
  - '**/tools/edit_line/**'
  - '**/tools/edit_lines/**'
  - '**/tools/read/**'
  - '**/tools/write/**'
  - '**/tools/skills/**'
  reason: Prüfen ob andere Module diese Funktionen mit alter Einzel-Item-Signatur aufrufen
```
/answer 95d8-5f allow
Control Request:
```yaml
id: 95d8-60
result:
  structuredContent:
    matches:
    - path: xy/ai/mcpc/tools/skills/__init__.py
      lineno: 53
      match: "class MarkdownFormatSkill(Skill):"
    - path: xy/ai/mcpc/tools/skills/__init__.py
      lineno: 60
      match: "SKILLS: list[Skill] = [MarkdownFormatSkill()]"
    - path: xy/ai/mcpc/tools/skills/__init__.py
      lineno: 188
      match: "def register_skills(registry: ToolRegistry, environment: AppEnvironment) -> None:"
    - path: xy/ai/mcpc/tools/__init__.py
      lineno: 52
      match: "    register_skills(registry, environment)"
```
/answer 95d8-60 allow
ReasoningToken: 20
Text:
 Jetzt edit_line, edit_lines, read, write und skills.
Text:
 Jetzt read, write, skills.
Text:
 Alle 9 Dateien geschrieben. Jetzt validieren.
ReasoningToken: 156
Result Stats:  id=749919e9-f0f6-4244-aedc-06153908e154, total: 0, in: 167, out: 88201, reason: 0, read: 1223003, write: 146456
Keine weiteren externen Aufrufer mit alter Einzel-Item-Signatur gefunden. Die Umstellung auf Batch-Support ist für alle neun Tools abgeschlossen und validiert.
