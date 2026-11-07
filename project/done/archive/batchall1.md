Alle genannten Tools sollen um einen Batch Support erweitert werden. Das bedeutet das alle Input-Parameter gemeinsam von einer Liste umschlossen werden wie auch die Outputs von einer Liste umschlossen werden. Dadurch muss der Ouput in einigen Tools ergänzt werden um den gespiegelten Input von Path und IDs. Die ist notwendig für die Ergebnisassoziation. Das Ziel ist die Minimierung von Tool Zyklen und dem Agenten größere Flexibilität in seiner Arbeitsweise zu erlauben.
Bei Fehlern enthält der Output dann eine Liste von Ergebnissen und eine getrennte Liste von Fehlern.

Tools:
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py`
	- mehrere Dateien gleichzeitig anlegen können
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py`
	- In mehreren Dateien zeitgleich Knoten löschen oder Dateien löschen oder eine beliebige Kombination
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py`
	- Gleichzeitige Editieroperationen erlauben. ID und neuer Inhalt bleiben auf der gleichen Ebene. Mehrfache Editierung im selben Path benötigt die mehrfache Angabe von Ziel ID und Inhalt 
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py`
	- Gleichzeitige Editieroperationen erlauben. ID und neuer Inhalt bleiben auf der gleichen Ebene. Mehrfache Editierung im selben Path benötigt die mehrfache Angabe von Ziel ID und Inhalt 
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py`
	- Gleichzeitige Einfügeoperationen. Es soll auch für einen Pfad möglich seine mehrere Operationen anzugeben sowie auch mehrere Pfade mit mehreren Operationen.
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/list.py`
	- Die Übersicht mehrerer Dateien gleichzeitig auslesen.
	- Hier soll es ein Preprocessing von Verzeichnissen geben, die im Input eigentlich nicht erlaubt sind. Wird festgestellt das im Input dennoch ein Verzeichnis angegeben wurde, wird geprüft, ob das Verzeichnis zu Dateien expandiert werden kann. Das darf nur erfolgreich sein, wenn das Verzeichnis keine Unterverzeichnisse enthält und auch nicht mehr als 5 Dateien. Kann das nicht antizipiert werden, wird es nicht korrigiert und führt in Folge zu einem normalen Fehler. 
		- Ein Beispiel findest du in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py` Function `_load_tolerant`
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py`
	- Soll auch erlauben, mehrere Dateien mit mehreren IDs zu lesen
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py`
	- Soll erlauben, mehrere Dateien mit mehreren Replace Operationen auszuführen. ID und Content bleiben dabei zusammen da es keinen Sinn macht einen Knoten mehrfach zu ersetzen. 
SystemInit:  id=122e5683-70f2-4025-b666-f4d8b85918d4, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 379b-9
toolName: ast_read
arguments:
  ids:
  - __module__
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
  reason: "Aktuellen Code lesen, um Batch-Support zu implementieren"
```
/answer 379b-9 allow
Control Request:
```yaml
id: 379b-a
result:
  structuredContent:
    nodes:
    - id: kLgDpr
      type: statements
    - id: ReKSW3
      type: imports
    - id: LIvIaB
      type: statements
    - id: CreateFileResult
      type: ClassDef
      signature: "@dataclass(frozen=True) class CreateFileResult:"
      docstring: "Result of :func:`ast_create`. Attributes: result: Always ``\"success\"``."
    - id: ast_create
      type: FunctionDef
      signature: "def ast_create(path: str, source: str, overwrite: bool=False) -> CreateFileResult:"
      docstring: Create a new file at ``path`` from ``source`` (validated by parsing it). Creati…
    - id: CreateFileTool
      type: ClassDef
      signature: "class CreateFileTool(ToolDefinition):"
      children:
      - id: CreateFileTool.Vsx9J1
        type: statements
      - id: CreateFileTool.Ej7a7u
        type: statements
      - id: CreateFileTool.bYnD7y
        type: statements
      - id: CreateFileTool.handle
        type: FunctionDef
        signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
        docstring: "Delegate to :func:`ast_create`, translating the MCP schema to/from the Python A…"
    - id: register
      type: FunctionDef
      signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    errors:
    - "ids ['__module__'] resolved to no node; redirected to ast_list, returning the full outline instead."
```
/answer 379b-a allow
Control Request:
```yaml
id: 379b-b
toolName: ast_read
arguments:
  ids:
  - __module__
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
  reason: "Aktuellen Code lesen, um Batch-Support zu implementieren"
```
/answer 379b-b allow
Control Request:
```yaml
id: 379b-c
result:
  structuredContent:
    nodes:
    - id: G9BBNn
      type: statements
    - id: 3ctrsF
      type: imports
    - id: w2zsO4
      type: statements
    - id: DeleteResult
      type: ClassDef
      signature: "@dataclass(frozen=True) class DeleteResult:"
      docstring: "Result of :func:`ast_delete`. Attributes: result: Always ``\"success\"``."
    - id: ast_delete
      type: FunctionDef
      signature: "def ast_delete(path: str, *, id: str | None=None) -> DeleteResult:"
      docstring: "Delete the single selected node, or the whole file if the root is selected. The…"
    - id: DeleteTool
      type: ClassDef
      signature: "class DeleteTool(ToolDefinition):"
      children:
      - id: DeleteTool.FhGUEZ
        type: statements
      - id: DeleteTool.sfOEmu
        type: statements
      - id: DeleteTool.handle
        type: FunctionDef
        signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
        docstring: "Delegate to :func:`ast_delete`, translating the MCP schema to/from the Python A…"
    - id: register
      type: FunctionDef
      signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    errors:
    - "ids ['__module__'] resolved to no node; redirected to ast_list, returning the full outline instead."
```
/answer 379b-c allow
Control Request:
```yaml
id: 379b-d
toolName: ast_read
arguments:
  ids:
  - __module__
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  reason: "Aktuellen Code lesen, um Batch-Support zu implementieren"
```
/answer 379b-d allow
Control Request:
```yaml
id: 379b-e
result:
  structuredContent:
    nodes:
    - id: q0CvRQ
      type: statements
    - id: IwT4eM
      type: imports
    - id: pbRXsQ
      type: statements
    - id: EditBlockNodeResult
      type: ClassDef
      signature: "@dataclass(frozen=True) class EditBlockNodeResult:"
      docstring: "Result of :func:`ast_edit_block`. Attributes: result: Always ``\"success\"``. id:…"
    - id: node_guard
      type: FunctionDef
      signature: "def _node_guard(engine, reference):"
      docstring: Guard for tolerant node edits. Engines that flag malformed edits on replace nee…
    - id: ast_edit_block
      type: FunctionDef
      signature: "def ast_edit_block(path: str, old_text: str, new_text: str, *, exact: bool=False, replace_all: bool=False, id: str | None=None) -> EditBlockNodeResult:"
      docstring: Replace occurrence(s) of ``old_text`` with ``new_text`` inside a node addressed…
    - id: EditBlockNodeTool
      type: ClassDef
      signature: "class EditBlockNodeTool(ToolDefinition):"
      children:
      - id: EditBlockNodeTool.mBzLqL
        type: statements
      - id: EditBlockNodeTool.IL491U
        type: statements
      - id: EditBlockNodeTool.UzEL6H
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
    - "ids ['__module__'] resolved to no node; redirected to ast_list, returning the full outline instead."
```
/answer 379b-e allow
Control Request:
```yaml
id: 379b-f
toolName: ast_read
arguments:
  ids:
  - __module__
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  reason: "Aktuellen Code lesen, um Batch-Support zu implementieren"
```
/answer 379b-f allow
Control Request:
```yaml
id: 379b-10
result:
  structuredContent:
    nodes:
    - id: zkTaU7
      type: statements
    - id: itEMHk
      type: imports
    - id: OlQPSt
      type: statements
    - id: EditMarksNodeResult
      type: ClassDef
      signature: "@dataclass(frozen=True) class EditMarksNodeResult:"
      docstring: "Result of :func:`ast_edit_marks`. Attributes: result: Always ``\"success\"``. id:…"
    - id: node_marks_guard
      type: FunctionDef
      signature: "def _node_marks_guard(engine, begin_marker, end_marker):"
      docstring: Guard for tolerant marker edits (see ``ast.edit_block._node_guard``).
    - id: ast_edit_marks
      type: FunctionDef
      signature: "def ast_edit_marks(path: str, start_marker: str, end_marker: str, content: str, *, exact: bool=False, id: str | None=None) -> EditMarksNodeResult:"
      docstring: Replace everything between the 'start_marker' and 'end_marker' markers inside a…
    - id: EditMarksNodeTool
      type: ClassDef
      signature: "class EditMarksNodeTool(ToolDefinition):"
      children:
      - id: EditMarksNodeTool.BEUMmb
        type: statements
      - id: EditMarksNodeTool.1HNIqt
        type: statements
      - id: EditMarksNodeTool.oQSnse
        type: statements
      - id: EditMarksNodeTool.sfOEmu
        type: statements
      - id: EditMarksNodeTool.handle
        type: FunctionDef
        signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
        docstring: "Delegate to :func:`ast_edit_marks`, translating the MCP schema to/from the AST …"
    - id: register
      type: FunctionDef
      signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    errors:
    - "ids ['__module__'] resolved to no node; redirected to ast_list, returning the full outline instead."
```
/answer 379b-10 allow
Control Request:
```yaml
id: 379b-11
toolName: ast_read
arguments:
  ids:
  - __module__
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
  reason: "Aktuellen Code lesen, um Batch-Support zu implementieren"
```
/answer 379b-11 allow
Control Request:
```yaml
id: 379b-12
result:
  structuredContent:
    nodes:
    - id: 0RkB6G
      type: statements
    - id: 3ctrsF
      type: imports
    - id: 8HTJQS
      type: statements
    - id: InsertNodeResult
      type: ClassDef
      signature: "@dataclass(frozen=True) class InsertNodeResult:"
      docstring: "Result of :func:`ast_insert`. Attributes: result: Always ``\"success\"``. inserte…"
    - id: ast_insert
      type: FunctionDef
      signature: "def ast_insert(path: str, source: str, *, position: str='after', id: str | None=None) -> InsertNodeResult:"
      docstring: "Insert statement(s) parsed from ``source`` relative to a selected node. Args: p…"
    - id: InsertNodeTool
      type: ClassDef
      signature: "class InsertNodeTool(ToolDefinition):"
      children:
      - id: InsertNodeTool.WkLcI4
        type: statements
      - id: InsertNodeTool.p2bYjY
        type: statements
      - id: InsertNodeTool.T9St9u
        type: statements
      - id: InsertNodeTool.handle
        type: FunctionDef
        signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
        docstring: "Delegate to :func:`ast_insert`, translating the MCP schema to/from the AST API."
    - id: register
      type: FunctionDef
      signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    errors:
    - "ids ['__module__'] resolved to no node; redirected to ast_list, returning the full outline instead."
```
/answer 379b-12 allow
Control Request:
```yaml
id: 379b-13
toolName: ast_read
arguments:
  ids:
  - __module__
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/list.py
  reason: "Aktuellen Code lesen, um Batch-Support zu implementieren"
```
/answer 379b-13 allow
Control Request:
```yaml
id: 379b-14
result:
  structuredContent:
    nodes:
    - id: ATBfxv
      type: statements
    - id: DbGQUa
      type: imports
    - id: K1UFFp
      type: statements
    - id: ListNodesResult
      type: ClassDef
      signature: "@dataclass(frozen=True) class ListNodesResult:"
      docstring: "Result of :func:`ast_list`. Attributes: nodes: Outline-style node descriptions …"
    - id: ast_list
      type: FunctionDef
      signature: "def ast_list(path: str, *, with_lines: bool=True) -> ListNodesResult:"
      docstring: List the hierarchical AST-node tree of a file. The tree is the foundation every…
    - id: ListNodesTool
      type: ClassDef
      signature: "class ListNodesTool(ToolDefinition):"
      children:
      - id: ListNodesTool.JpxHvN
        type: statements
      - id: ListNodesTool.handle
        type: FunctionDef
        signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
        docstring: "Delegate to :func:`ast_list`, translating the MCP schema to/from the AST API."
    - id: register
      type: FunctionDef
      signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    errors:
    - "ids ['__module__'] resolved to no node; redirected to ast_list, returning the full outline instead."
```
/answer 379b-14 allow
Control Request:
```yaml
id: 379b-15
toolName: ast_read
arguments:
  ids:
  - __module__
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  reason: "Aktuellen Code lesen, um Batch-Support zu implementieren"
```
/answer 379b-15 allow
Control Request:
```yaml
id: 379b-16
result:
  structuredContent:
    nodes:
    - id: YEXBkx
      type: statements
    - id: QGKJvK
      type: imports
    - id: ObuTMo
      type: statements
    - id: looks_like_root_intent
      type: FunctionDef
      signature: "def _looks_like_root_intent(ids: list[str]) -> bool:"
      docstring: Whether ``ids`` is empty or consists solely of common root-id guesses.
    - id: ReadNodeResult
      type: ClassDef
      signature: "@dataclass(frozen=True) class ReadNodeResult:"
      docstring: "Result of :func:`ast_read`. Attributes: nodes: One expanded subtree per resolve…"
    - id: ast_read
      type: FunctionDef
      signature: "def ast_read(ids: list[str], path: str, *, with_lines: bool=True) -> ReadNodeResult:"
      docstring: Recursively read the subtree of each addressed node for block-wise edit/replace…
    - id: ReadNodeTool
      type: ClassDef
      signature: "class ReadNodeTool(ToolDefinition):"
      children:
      - id: ReadNodeTool.ADVV8W
        type: statements
      - id: ReadNodeTool.B0qnog
        type: statements
      - id: ReadNodeTool.vxU2fb
        type: statements
      - id: ReadNodeTool.handle
        type: FunctionDef
        signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
        docstring: "Delegate to :func:`ast_read`, translating the MCP schema to/from the AST API. F…"
    - id: register
      type: FunctionDef
      signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    errors:
    - "ids ['__module__'] resolved to no node; redirected to ast_list, returning the full outline instead."
```
/answer 379b-16 allow
Control Request:
```yaml
id: 379b-17
toolName: ast_read
arguments:
  ids:
  - __module__
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
  reason: "Aktuellen Code lesen, um Batch-Support zu implementieren"
```
/answer 379b-17 allow
Control Request:
```yaml
id: 379b-18
result:
  structuredContent:
    nodes:
    - id: mZlKhS
      type: statements
    - id: 3ctrsF
      type: imports
    - id: pMvmCp
      type: statements
    - id: ReplaceNodeResult
      type: ClassDef
      signature: "@dataclass(frozen=True) class ReplaceNodeResult:"
      docstring: "Result of :func:`ast_replace`. Attributes: result: Always ``\"success\"``. id: Th…"
    - id: ast_replace
      type: FunctionDef
      signature: "def ast_replace(path: str, source: str, *, id: str | None=None) -> ReplaceNodeResult:"
      docstring: "Replace the single selected node with ``source``. Args: path: Absolute path to …"
    - id: ReplaceNodeTool
      type: ClassDef
      signature: "class ReplaceNodeTool(ToolDefinition):"
      children:
      - id: ReplaceNodeTool.6LfsTB
        type: statements
      - id: ReplaceNodeTool.oC5twa
        type: statements
      - id: ReplaceNodeTool.handle
        type: FunctionDef
        signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
        docstring: "Delegate to :func:`ast_replace`, translating the MCP schema to/from the AST API."
    - id: register
      type: FunctionDef
      signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    errors:
    - "ids ['__module__'] resolved to no node; redirected to ast_list, returning the full outline instead."
```
/answer 379b-18 allow
Control Request:
```yaml
id: 379b-19
toolName: ast_read
arguments:
  ids:
  - __module__
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
  reason: Beispiel für _load_tolerant Verzeichnis-Expansion ansehen
```
/answer 379b-19 allow
Control Request:
```yaml
id: 379b-1a
result:
  structuredContent:
    nodes:
    - id: jlFZZg
      type: statements
    - id: SMAply
      type: imports
    - id: mmIbkP
      type: statements
    - id: FileNodesResult
      type: ClassDef
      signature: "@dataclass(frozen=True) class FileNodesResult:"
      docstring: "Nodes matching the given selectors within a single file. Attributes: path: The …"
    - id: FindNodesResult
      type: ClassDef
      signature: "@dataclass(frozen=True) class FindNodesResult:"
      docstring: "Result of :func:`ast_find`. Attributes: files: One :class:`FileNodesResult` per…"
    - id: load_tolerant
      type: FunctionDef
      signature: "def _load_tolerant(path: str) -> core.Tree:"
      docstring: "Load *path*, tolerating a forgotten filename extension. If *path* does not exis…"
    - id: find_in_file
      type: FunctionDef
      signature: "def _find_in_file(path: str, *, exact: dict[str, Any], lineno: int | None, end_lineno: int | None, no_selector: bool, pattern: re.Pattern[str] | None, with_lines: bool) -> FileNodesResult:"
    - id: ast_find
      type: FunctionDef
      signature: "def ast_find(paths: list[str], *, id: str | None=None, name: str | None=None, node_type: str | None=None, lineno: int | None=None, end_lineno: int | None=None, parent_type: str | None=None, text: str | None=None, regexp: str | None=None, with_lines: bool=True) -> FindNodesResult:"
      docstring: "Find nodes by id, type, name, line range, parent type, text or regexp. ``ast_fi…"
    - id: FindNodesTool
      type: ClassDef
      signature: "class FindNodesTool(ToolDefinition):"
      children:
      - id: FindNodesTool.vMKkuI
        type: statements
      - id: FindNodesTool.nplwMP
        type: statements
      - id: FindNodesTool.ZaIxOu
        type: statements
      - id: FindNodesTool.e5owbr
        type: statements
      - id: FindNodesTool.handle
        type: FunctionDef
        signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
        docstring: "Delegate to :func:`ast_find`, translating the MCP schema to/from the AST API."
    - id: register
      type: FunctionDef
      signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    errors:
    - "ids ['__module__'] resolved to no node; redirected to ast_list, returning the full outline instead."
```
/answer 379b-1a allow
Control Request:
```yaml
id: 379b-1b
toolName: ast_read
arguments:
  ids:
  - kLgDpr
  - ReKSW3
  - LIvIaB
  - CreateFileResult
  - ast_create
  - CreateFileTool
  - register
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
  reason: Vollständigen Quellcode lesen für Batch-Umbau
```
/answer 379b-1b allow
Control Request:
```yaml
id: 379b-1c
result:
  structuredContent:
    nodes:
    - id: kLgDpr
      type: statements
      code: "'``ast_create`` tool: create a file from source, creating missing directories.'"
    - id: ReKSW3
      type: imports
      code: |-
        from dataclasses import dataclass
        from typing import Any
        from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
        from xy.ai.mcpc.tools.tool_context import ToolContext
        from xy.ai.mcpc.tools.ast import core
        from xy.ai.mcpc.tools.function_registry import FunctionRegistry
    - id: LIvIaB
      type: statements
      code: "__all__ = ['CreateFileResult', 'ast_create', 'CreateFileTool', 'register']"
    - id: CreateFileResult
      type: ClassDef
      code: |-
        @dataclass(frozen=True)
        class CreateFileResult:
            """Result of :func:`ast_create`.

            Attributes:
                result: Always ``"success"``.
            """
            result: str
    - id: ast_create
      type: FunctionDef
      code: |-
        def ast_create(path: str, source: str, overwrite: bool=False) -> CreateFileResult:
            """Create a new file at ``path`` from ``source`` (validated by parsing it).

            Creating a single node
            in an existing file is covered by ``ast_insert``, not this tool.

            Args:
                path: Absolute path of the file to create.
                source: Source for the new file.
                overwrite: Allow replacing an existing file. Defaults to ``False``.

            Returns:
                CreateFileResult: Success status.

            Raises:
                core.AstError: If ``path`` is not absolute, if the file already exists and
                    ``overwrite`` is ``False``, or if ``source`` has a syntax error.
            """
            file_path = core.require_path(path, must_exist=False)
            if file_path.exists() and (not overwrite):
                raise core.AstError('File already exists.')
            tree = core.parse_for(path, source)
            file_path.parent.mkdir(parents=True, exist_ok=True)
            core.CACHE.save(file_path, tree)
            return CreateFileResult(result='success')
    - id: CreateFileTool
      type: ClassDef
      signature: "class CreateFileTool(ToolDefinition):"
      children:
      - id: CreateFileTool.Vsx9J1
        type: statements
        code: |-
          name = 'ast_create'
          title = 'Create a file'
          description = 'Create a file from source.'
      - id: CreateFileTool.Ej7a7u
        type: statements
        code: |-
          input_schema = {
              'type': 'object', 'properties': {
                  'path': {
                      'type': 'string', 'description': 'Absolute path of the file to create.'}, 'source': {
                          'type': 'string', 'description': 'Source for the new file.'}, 'overwrite': {
                              'type': 'boolean', 'description': 'Allow replacing an existing file.', 'default': False}}, 'required': [
                                  'path', 'source']}
      - id: CreateFileTool.bYnD7y
        type: statements
        code: |-
          output_schema = {'type': 'object', 'properties': {'result': {'type': 'string'}}, 'required': ['result']}
          annotations = {'readOnlyHint': False, 'openWorldHint': False}
      - id: CreateFileTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`ast_create`, translating the MCP schema to/from the Python API."""
              args: dict[str, Any] = ctx.arguments
              try:
                  result = ast_create(path=args['path'], source=args['source'], overwrite=args.get('overwrite', False))
              except core.AstError as exc:
                  return ToolResult(content=[text_content(str(exc))], is_error=True)
              return ToolResult(structured_content={'result': result.result}, auto_approve=True)
    - id: register
      type: FunctionDef
      code: |-
        def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
            registry.register(CreateFileTool())
            functions.register(ast_create)
```
/answer 379b-1c allow
Control Request:
```yaml
id: 379b-1d
toolName: ast_read
arguments:
  ids:
  - G9BBNn
  - 3ctrsF
  - w2zsO4
  - DeleteResult
  - ast_delete
  - DeleteTool
  - register
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
  reason: Vollständigen Quellcode lesen für Batch-Umbau
```
/answer 379b-1d allow
Control Request:
```yaml
id: 379b-1e
result:
  structuredContent:
    nodes:
    - id: G9BBNn
      type: statements
      code: "'``ast_delete`` tool: delete a selected node, or the whole file if none is selected.'"
    - id: 3ctrsF
      type: imports
      code: |-
        from dataclasses import dataclass
        from typing import Any
        from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
        from xy.ai.mcpc.tools.tool_context import ToolContext
        from xy.ai.mcpc.tools.ast import core
        from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_path
        from xy.ai.mcpc.tools.function_registry import FunctionRegistry
    - id: w2zsO4
      type: statements
      code: "__all__ = ['DeleteResult', 'ast_delete', 'DeleteTool', 'register']"
    - id: DeleteResult
      type: ClassDef
      code: |-
        @dataclass(frozen=True)
        class DeleteResult:
            """Result of :func:`ast_delete`.

            Attributes:
                result: Always ``"success"``.
            """
            result: str
    - id: ast_delete
      type: FunctionDef
      code: |-
        def ast_delete(path: str, *, id: str | None=None) -> DeleteResult:
            """Delete the single selected node, or the whole file if the root is selected.

            The whole file is deleted by omitting the ``id`` selector – there is no other way
            to address the root, since it is never itself an addressable child. Deleting the
            file also removes it from the AST cache and, if its parent directory becomes
            empty as a result, removes that directory too.

            Args:
                path: Absolute path to the file to modify.
                id: Unique id of the target node.

            Returns:
                DeleteResult: Success status.

            Raises:
                core.AstError: If ``path`` is invalid, or a selector is given but matches
                    zero or more than one node.
            """
            file_path = core.require_path(path)
            if id is None:
                try:
                    file_path.unlink()
                except OSError as exc:
                    raise core.AstError('Delete failed.') from exc
                core.CACHE.invalidate(file_path)
                parent = file_path.parent
                if not any(parent.iterdir()):
                    parent.rmdir()
                return DeleteResult(result='success')
            tree = core.CACHE.get_tree(file_path)
            target = select_by_path(tree, id=id)
            core.delete_node(target)
            core.CACHE.save(file_path, tree)
            return DeleteResult(result='success')
    - id: DeleteTool
      type: ClassDef
      signature: "class DeleteTool(ToolDefinition):"
      children:
      - id: DeleteTool.FhGUEZ
        type: statements
        code: |-
          name = 'ast_delete'
          title = 'Delete AST node or file'
          description = 'Delete the single selected node from a file, or the whole file if no selector is given.'
          input_schema = {
              'type': 'object',
              'properties': {
                  'path': {
                      'type': 'string',
                      'description': 'Absolute path to the file.'},
                  **PATH_SELECTOR_PROPS},
              'required': ['path']}
          output_schema = {'type': 'object', 'properties': {'result': {'type': 'string'}}, 'required': ['result']}
      - id: DeleteTool.sfOEmu
        type: statements
        code: "annotations = {'readOnlyHint': False, 'openWorldHint': False}"
      - id: DeleteTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`ast_delete`, translating the MCP schema to/from the Python API."""
              args: dict[str, Any] = ctx.arguments
              try:
                  result = ast_delete(args['path'], id=args.get('id'))
              except core.AstError as exc:
                  return ToolResult(content=[text_content(str(exc))], is_error=True)
              return ToolResult(structured_content={'result': result.result}, auto_approve=True)
    - id: register
      type: FunctionDef
      code: |-
        def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
            registry.register(DeleteTool())
            functions.register(ast_delete)
```
/answer 379b-1e allow
Control Request:
```yaml
id: 379b-1f
toolName: ast_read
arguments:
  ids:
  - q0CvRQ
  - IwT4eM
  - pbRXsQ
  - EditBlockNodeResult
  - node_guard
  - ast_edit_block
  - EditBlockNodeTool
  - register
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  reason: Vollständigen Quellcode lesen für Batch-Umbau
```
/answer 379b-1f allow
Control Request:
```yaml
id: 379b-20
result:
  structuredContent:
    nodes:
    - id: q0CvRQ
      type: statements
      code: "'``ast_edit_block`` tool: exact-block (old_text -> new_text) edit within a selected node.'"
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
    - id: pbRXsQ
      type: statements
      code: "__all__ = ['EditBlockNodeResult', 'ast_edit_block', 'EditBlockNodeTool', 'register']"
    - id: EditBlockNodeResult
      type: ClassDef
      code: |-
        @dataclass(frozen=True)
        class EditBlockNodeResult:
            """Result of :func:`ast_edit_block`.

            Attributes:
                result: Always ``"success"``.
                id: The node's new id, only set if the edit changed it.
            """
            result: str
            id: str | None = None
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
    - id: ast_edit_block
      type: FunctionDef
      code: |-
        def ast_edit_block(path: str, old_text: str, new_text: str, *, exact: bool=False, replace_all: bool=False, id: str | None=None) -> EditBlockNodeResult:
            """Replace occurrence(s) of ``old_text`` with ``new_text`` inside a node addressed by id.

            The addressed node's source is unparsed, its ``old_text`` block replaced (as with
            ``edit_block``), re-parsed, and used to replace the node.

            Args:
                path: Absolute path to the file to modify.
                old_text: Unique 10-100 char block to find within the node's source (unless ``replace_all``).
                new_text: Replacement text (may be empty to delete the block).
                exact: If False (default), whitespace in ``old_text`` is matched tolerantly.
                replace_all: If True, replace every occurrence instead of requiring a single match.
                id: Unique id of the target node. If omitted, the node is searched for by
                    ``old_text`` instead.

            Returns:
                EditBlockNodeResult: Success status.

            Raises:
                core.AstError: If ``path`` is invalid, ``id`` matches zero or more than
                    one node, no node contains ``old_text`` (when ``id`` is omitted),
                    ``old_text`` is not found or (without ``replace_all``) ambiguous
                    within the node's source, or the edited source has a syntax error.
                core.AstAmbiguous: If ``id`` is omitted and several unrelated nodes
                    contain ``old_text``.
            """
            file_path = core.require_path(path)
            tree = core.CACHE.get_tree(file_path)
            target = select_by_text(tree, [old_text], id=id)
            node_source = core.edit_node_source(target)
            try:
                new_source = replace_in_block(
                    node_source,
                    old_text,
                    new_text,
                    exact=exact,
                    replace_all=replace_all,
                    accept=_node_guard(
                        tree.engine,
                        old_text),
                    max_level=3 if tree.engine.validates_syntax else 2,
                    where='node')
            except TextMatchError as exc:
                raise core.AstError(str(exc)) from exc
            new_id = core.replace_node(target, new_source)
            core.CACHE.save(file_path, tree)
            return EditBlockNodeResult(result='success', id=new_id)
    - id: EditBlockNodeTool
      type: ClassDef
      signature: "class EditBlockNodeTool(ToolDefinition):"
      children:
      - id: EditBlockNodeTool.mBzLqL
        type: statements
        code: |-
          name = 'ast_edit_block'
          title = 'Replace short text within AST node'
          description = "Replace occurrence of short 'old_text' with 'new_text', within the node addressed by id. Don't use for large edits, use ast_edit_marks instead."
      - id: EditBlockNodeTool.IL491U
        type: statements
        code: |-
          input_schema = {
              'type': 'object',
              'strict': True,
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
                  'new_text']}
      - id: EditBlockNodeTool.UzEL6H
        type: statements
        code: |-
          output_schema = {
              'type': 'object',
              'properties': {
                  'result': {
                      'type': 'string',
                      'description': 'Result status'},
                  'id': {
                      'type': 'string',
                      'description': "The node's new id. Preffer 'ast_find' over 'ast_list'."},
                  'candidates': {
                      'type': 'array',
                              'items': {
                                  'type': 'string'},
                      'description': 'On ambiguity (id omitted, several nodes matched), the candidate node ids.'}},
              'required': ['result']}
      - id: EditBlockNodeTool.sfOEmu
        type: statements
        code: "annotations = {'readOnlyHint': False, 'openWorldHint': False}"
      - id: EditBlockNodeTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`ast_edit_block`, translating the MCP schema to/from the AST API."""
              args: dict[str, Any] = ctx.arguments
              try:
                  result = ast_edit_block(
                      args['path'], args['old_text'], args['new_text'], exact=args.get(
                          'exact', False), replace_all=args.get(
                              'replaceAll', False), id=args.get('id'))
              except core.AstAmbiguous as exc:
                  return ToolResult(
                      content=[
                          text_content(
                              str(exc))],
                      structured_content={
                          'candidates': exc.candidates},
                      is_error=True)
              except core.AstError as exc:
                  return ToolResult(content=[text_content(str(exc))], is_error=True)
              if result.id is not None:
                  message = f'Node {args.get('id')} was replaced with {result.id}'
              else:
                  message = f'Node ID {args.get('id')} unchanged'
              content = {'result': message}
              if result.id is not None:
                  content['id'] = result.id
              return ToolResult(content=[text_content(message)], structured_content=content, auto_approve=True)
    - id: register
      type: FunctionDef
      code: |-
        def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
            registry.register(EditBlockNodeTool())
            functions.register(ast_edit_block)
```
/answer 379b-20 allow
Control Request:
```yaml
id: 379b-21
toolName: ast_read
arguments:
  ids:
  - zkTaU7
  - itEMHk
  - OlQPSt
  - EditMarksNodeResult
  - node_marks_guard
  - ast_edit_marks
  - EditMarksNodeTool
  - register
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  reason: Vollständigen Quellcode lesen für Batch-Umbau
```
/answer 379b-21 allow
Control Request:
```yaml
id: 379b-22
result:
  structuredContent:
    nodes:
    - id: zkTaU7
      type: statements
      code: "'``ast_edit_marks`` tool: mark-based edit within the source of a node addressed by id.'"
    - id: itEMHk
      type: imports
      code: |-
        from dataclasses import dataclass
        from typing import Any
        from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
        from xy.ai.mcpc.tools.tool_context import ToolContext
        from xy.ai.mcpc.tools.ast import core
        from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_text
        from xy.ai.mcpc.tools._text_match import replace_between, marks_line_preserving, TextMatchError
        from xy.ai.mcpc.tools.function_registry import FunctionRegistry
    - id: OlQPSt
      type: statements
      code: "__all__ = ['EditMarksNodeResult', 'ast_edit_marks', 'EditMarksNodeTool', 'register']"
    - id: EditMarksNodeResult
      type: ClassDef
      code: |-
        @dataclass(frozen=True)
        class EditMarksNodeResult:
            """Result of :func:`ast_edit_marks`.

            Attributes:
                result: Always ``"success"``.
                id: The node's new id, only set if the edit changed it.
            """
            result: str
            id: str | None = None
    - id: node_marks_guard
      type: FunctionDef
      code: |-
        def _node_marks_guard(engine, begin_marker, end_marker):
            """Guard for tolerant marker edits (see ``ast.edit_block._node_guard``)."""
            if engine.validates_syntax:
                return None
            keep_lines = marks_line_preserving(begin_marker, end_marker)
            return lambda begin, end, result: keep_lines(begin, end, result) and engine.validate(result) is None
    - id: ast_edit_marks
      type: FunctionDef
      code: |-
        def ast_edit_marks(path: str, start_marker: str, end_marker: str, content: str, *, exact: bool=False, id: str | None=None) -> EditMarksNodeResult:
            """Replace everything between the 'start_marker' and 'end_marker' markers inside a node addressed by id.

            The addressed node's source is unparsed, edited between the two markers (both
            included) with escalating whitespace/escape/quote tolerance, re-parsed, and
            used to replace the node.

            Args:
                path: Absolute path to the file to modify.
                start_marker: Unique 10-30 char substring marking the beginning of the block, within the node's source.
                end_marker: Unique 10-30 char substring marking the end of the block, within the node's source.
                content: Replacement source for the marked block.
                exact: If False (default), whitespace in start/end is matched tolerantly. If True, whitespace must match exactly.
                id: Unique id of the target node. If omitted, the node is searched for by
                    ``start_marker``/``end_marker`` instead.

            Returns:
                EditMarksNodeResult: Success status.

            Raises:
                core.AstError: If ``path`` is invalid, ``id`` matches zero or more than
                    one node, no node contains both markers (when ``id`` is omitted),
                    the markers are not found or ambiguous within the node's source, or
                    the edited source has a syntax error.
                core.AstAmbiguous: If ``id`` is omitted and several unrelated nodes
                    contain both markers.
            """
            file_path = core.require_path(path)
            tree = core.CACHE.get_tree(file_path)
            target = select_by_text(tree, [start_marker, end_marker], id=id)
            node_source = core.edit_node_source(target)
            begin, end = (start_marker, end_marker) if exact else (start_marker.strip(), end_marker.strip())
            try:
                new_source = replace_between(node_source, begin, end, content, exact=exact, accept=_node_marks_guard(
                    tree.engine, begin, end), max_level=3 if tree.engine.validates_syntax else 2, where='node')
            except TextMatchError as exc:
                raise core.AstError(str(exc)) from exc
            new_id = core.replace_node(target, new_source)
            core.CACHE.save(file_path, tree)
            return EditMarksNodeResult(result='success', id=new_id)
    - id: EditMarksNodeTool
      type: ClassDef
      signature: "class EditMarksNodeTool(ToolDefinition):"
      children:
      - id: EditMarksNodeTool.BEUMmb
        type: statements
        code: |-
          name = 'ast_edit_marks'
          title = 'Replace large text regions within a AST node between markers'
          description = "Replace everything between and including the unique 'start_marker' and 'end_marker' markers, found within the node addressed by id, with new 'content'."
      - id: EditMarksNodeTool.1HNIqt
        type: statements
        code: |-
          input_schema = {
              'type': 'object',
              'strict': True,
              'additionalProperties': False,
              'properties': {
                  'path': {
                      'type': 'string',
                      'description': 'Absolute path to the file.'},
                  'start_marker': {
                      'type': 'string',
                      'minLength': 10,
                      'maxLength': 30,
                      'description': "Unique 10-30 char substring marking the beginning of the text to replace, within the node's source."},
                  'end_marker': {
                      'type': 'string',
                              'minLength': 10,
                              'maxLength': 30,
                              'description': "Unique 10-30 char substring marking the end of the text to replace, within the node's source."},
                  'content': {
                      'type': 'string',
                      'description': 'Replacement source for the marked text.'},
                  'exact': {
                      'type': 'boolean',
                      'description': "If true, 'start_marker'/'end_marker' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                      'default': False},
                  **PATH_SELECTOR_PROPS},
              'required': [
                  'path',
                  'start_marker',
                  'end_marker',
                  'content']}
      - id: EditMarksNodeTool.oQSnse
        type: statements
        code: |-
          output_schema = {
              'type': 'object',
              'properties': {
                  'result': {
                      'type': 'string',
                      'description': 'Result status'},
                  'id': {
                      'type': 'string',
                      'description': "The node's new id.  Preffer 'ast_find' over 'ast_list'."},
                  'candidates': {
                      'type': 'array',
                              'items': {
                                  'type': 'string'},
                      'description': 'On ambiguity (id omitted, several nodes matched), the candidate node ids.'}},
              'required': ['result']}
      - id: EditMarksNodeTool.sfOEmu
        type: statements
        code: "annotations = {'readOnlyHint': False, 'openWorldHint': False}"
      - id: EditMarksNodeTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`ast_edit_marks`, translating the MCP schema to/from the AST API."""
              args: dict[str, Any] = ctx.arguments
              try:
                  result = ast_edit_marks(
                      args['path'],
                      args['start_marker'],
                      args['end_marker'],
                      args['content'],
                      exact=args.get(
                          'exact',
                          False),
                      id=args.get('id'))
              except core.AstAmbiguous as exc:
                  return ToolResult(
                      content=[
                          text_content(
                              str(exc))],
                      structured_content={
                          'candidates': exc.candidates},
                      is_error=True)
              except core.AstError as exc:
                  return ToolResult(content=[text_content(str(exc))], is_error=True)
              if result.id is not None:
                  message = f'Node {args.get('id')} was replaced with {result.id}'
              else:
                  message = f'Node ID {args.get('id')} unchanged'
              content = {'result': message}
              if result.id is not None:
                  content['id'] = result.id
              return ToolResult(content=[text_content(message)], structured_content=content, auto_approve=True)
    - id: register
      type: FunctionDef
      code: |-
        def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
            registry.register(EditMarksNodeTool())
            functions.register(ast_edit_marks)
```
/answer 379b-22 allow
Control Request:
```yaml
id: 379b-23
toolName: ast_read
arguments:
  ids:
  - 0RkB6G
  - 3ctrsF
  - 8HTJQS
  - InsertNodeResult
  - ast_insert
  - InsertNodeTool
  - register
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
  reason: Vollständigen Quellcode lesen für Batch-Umbau
```
/answer 379b-23 allow
Control Request:
```yaml
id: 379b-24
result:
  structuredContent:
    nodes:
    - id: 0RkB6G
      type: statements
      code: "'``ast_insert`` tool: insert statement(s) relative to a selected node.'"
    - id: 3ctrsF
      type: imports
      code: |-
        from dataclasses import dataclass
        from typing import Any
        from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
        from xy.ai.mcpc.tools.tool_context import ToolContext
        from xy.ai.mcpc.tools.ast import core
        from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_path
        from xy.ai.mcpc.tools.function_registry import FunctionRegistry
    - id: 8HTJQS
      type: statements
      code: "__all__ = ['InsertNodeResult', 'ast_insert', 'InsertNodeTool', 'register']"
    - id: InsertNodeResult
      type: ClassDef
      code: |-
        @dataclass(frozen=True)
        class InsertNodeResult:
            """Result of :func:`ast_insert`.

            Attributes:
                result: Always ``"success"``.
                inserted: Number of top-level statements parsed from ``source`` and inserted.
                ids: The newly inserted top-level node(s)' ids.
            """
            result: str
            inserted: int
            ids: list[str] | None = None
    - id: ast_insert
      type: FunctionDef
      code: |-
        def ast_insert(path: str, source: str, *, position: str='after', id: str | None=None) -> InsertNodeResult:
            """Insert statement(s) parsed from ``source`` relative to a selected node.

            Args:
                path: Absolute path to the file to modify.
                source: Source of the statement(s) to insert.
                position: ``"before"`` or ``"after"`` the selected node. Defaults to ``"after"``.
                id: Unique id of the target node.


            Returns:
                InsertNodeResult: Success status, the number of statements inserted, and
                    their new ids.

            Raises:
                core.AstError: If ``path`` is invalid, ``source`` has a syntax error, ``id`` is
                    not given, or it matches zero or more than one node.
            """
            file_path = core.require_path(path)
            tree = core.CACHE.get_tree(file_path)
            target = select_by_path(tree, id=id)
            before_ids = {loc.node_id for loc in core.locate_all(tree)}
            inserted = core.insert_node(target, source, position)
            new_ids = [loc.node_id for loc in core.locate_all(tree) if loc.node_id not in before_ids]
            core.CACHE.save(file_path, tree)
            return InsertNodeResult(result='success', inserted=inserted, ids=new_ids or None)
    - id: InsertNodeTool
      type: ClassDef
      signature: "class InsertNodeTool(ToolDefinition):"
      children:
      - id: InsertNodeTool.WkLcI4
        type: statements
        code: |-
          name = 'ast_insert'
          title = 'Insert AST node'
          description = "Insert source relative to a selected node ('before' or 'after')."
      - id: InsertNodeTool.p2bYjY
        type: statements
        code: |-
          input_schema = {
              'type': 'object',
              'properties': {
                  'path': {
                      'type': 'string',
                      'description': 'Absolute path to the file.'},
                  'source': {
                      'type': 'string',
                      'description': 'Source to insert.'},
                  'position': {
                      'type': 'string',
                              'enum': [
                                  'before',
                                  'after'],
                      'description': 'Placement relative to the selected node.',
                      'default': 'after'},
                  **PATH_SELECTOR_PROPS},
              'required': [
                  'path',
                  'source']}
      - id: InsertNodeTool.T9St9u
        type: statements
        code: |-
          output_schema = {
              'type': 'object', 'properties': {
                  'result': {
                      'type': 'string'}, 'inserted': {
                          'type': 'integer'}, 'ids': {
                              'type': 'array', 'items': {
                                  'type': 'string'}, 'description': 'The newly inserted node(s) ids.'}}, 'required': [
                                      'result', 'inserted']}
          annotations = {'readOnlyHint': False, 'openWorldHint': False}
      - id: InsertNodeTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`ast_insert`, translating the MCP schema to/from the AST API."""
              args: dict[str, Any] = ctx.arguments
              try:
                  result = ast_insert(args['path'], args['source'], position=args.get('position', 'after'), id=args.get('id'))
              except core.AstError as exc:
                  return ToolResult(content=[text_content(str(exc))], is_error=True)
              content = {'result': result.result, 'inserted': result.inserted}
              if result.ids is not None:
                  content['ids'] = result.ids
              return ToolResult(structured_content=content, auto_approve=True)
    - id: register
      type: FunctionDef
      code: |-
        def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
            registry.register(InsertNodeTool())
            functions.register(ast_insert)
```
/answer 379b-24 allow
Control Request:
```yaml
id: 379b-25
toolName: ast_read
arguments:
  ids:
  - ATBfxv
  - DbGQUa
  - K1UFFp
  - ListNodesResult
  - ast_list
  - ListNodesTool
  - register
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/list.py
  reason: Vollständigen Quellcode lesen für Batch-Umbau
```
/answer 379b-25 allow
Control Request:
```yaml
id: 379b-26
result:
  structuredContent:
    nodes:
    - id: ATBfxv
      type: statements
      code: "'``ast_list`` tool: list AST nodes of a file or source snippet.'"
    - id: DbGQUa
      type: imports
      code: |-
        from dataclasses import dataclass
        from typing import Any
        from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
        from xy.ai.mcpc.tools.tool_context import ToolContext
        from xy.ai.mcpc.tools.ast import core
        from xy.ai.mcpc.tools.ast.common import list_output_schema
        from xy.ai.mcpc.tools.function_registry import FunctionRegistry
    - id: K1UFFp
      type: statements
      code: "__all__ = ['ListNodesResult', 'ast_list', 'ListNodesTool', 'register']"
    - id: ListNodesResult
      type: ClassDef
      code: |-
        @dataclass(frozen=True)
        class ListNodesResult:
            """Result of :func:`ast_list`.

            Attributes:
                nodes: Outline-style node descriptions (see :class:`core.OutlineNode`), in
                    document order, suited for retrieval and navigation.
            """
            nodes: list[core.OutlineNode]
    - id: ast_list
      type: FunctionDef
      code: |-
        def ast_list(path: str, *, with_lines: bool=True) -> ListNodesResult:
            """List the hierarchical AST-node tree of a file.

            The tree is the foundation every other tool builds on: each node carries its
            unique, primarily name-based ``id`` and line range, but never its source –
            use ``ast_find`` (property/text filtering) or ``ast_read`` (by id) to
            retrieve source.

            Args:
                path: Absolute path to the file to read.
                with_lines: Whether to populate each node's line range.

            Returns:
                ListNodesResult: The nested node tree and the number of top-level nodes.

            Raises:
                core.AstError: If ``path`` is not absolute or does not point to an existing
                    regular file, or if the source has a syntax error.
            """
            tree = core.load(path)[1]
            nodes = core.build_outline(core.locate_all(tree), with_lines=with_lines)
            return ListNodesResult(nodes=nodes)
    - id: ListNodesTool
      type: ClassDef
      signature: "class ListNodesTool(ToolDefinition):"
      children:
      - id: ListNodesTool.JpxHvN
        type: statements
        code: |-
          name = 'ast_list'
          title = 'List AST nodes'
          description = "Hierarchical tree of a file's AST nodes (import/statement segments, classes, functions, sections) with id and optional line range – no source."
          input_schema = {
              'type': 'object',
              'properties': {
                  'path': {
                      'type': 'string',
                      'description': 'Absolute path to the file.'}},
              'required': ['path']}
          output_schema = list_output_schema()
          annotations = {'readOnlyHint': True, 'openWorldHint': False}
      - id: ListNodesTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`ast_list`, translating the MCP schema to/from the AST API."""
              args: dict[str, Any] = ctx.arguments
              with_lines = bool({'tools', 'edit-lines'} & ctx.session.enabled_tools)
              try:
                  result = ast_list(path=args.get('path'), with_lines=with_lines)
              except core.AstError as exc:
                  return ToolResult(content=[text_content(str(exc))], is_error=True)
              return ToolResult(structured_content={'nodes': [core.to_dict(n) for n in result.nodes]})
    - id: register
      type: FunctionDef
      code: |-
        def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
            registry.register(ListNodesTool())
            functions.register(ast_list)
```
/answer 379b-26 allow
Control Request:
```yaml
id: 379b-27
toolName: ast_read
arguments:
  ids:
  - YEXBkx
  - QGKJvK
  - ObuTMo
  - looks_like_root_intent
  - ReadNodeResult
  - ast_read
  - ReadNodeTool
  - register
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  reason: Vollständigen Quellcode lesen für Batch-Umbau
```
/answer 379b-27 allow
Control Request:
```yaml
id: 379b-28
result:
  structuredContent:
    nodes:
    - id: YEXBkx
      type: statements
      code: "'``ast_read`` tool: read one or more node subtrees (with source) by id.'"
    - id: QGKJvK
      type: imports
      code: |-
        from dataclasses import dataclass
        from typing import Any
        from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
        from xy.ai.mcpc.tools.tool_context import ToolContext
        from xy.ai.mcpc.tools.ast import core
        from xy.ai.mcpc.tools.function_registry import FunctionRegistry
        from xy.ai.mcpc.tools.ast.list import ast_list
    - id: ObuTMo
      type: statements
      code: |-
        __all__ = ['ReadNodeResult', 'ast_read', 'ReadNodeTool', 'register']
        _ROOT_INTENT_IDS = {'root', '_module_', '__module__', 'module', ''}
    - id: looks_like_root_intent
      type: FunctionDef
      code: |-
        def _looks_like_root_intent(ids: list[str]) -> bool:
            """Whether ``ids`` is empty or consists solely of common root-id guesses."""
            return not ids or all((i.strip().lower() in _ROOT_INTENT_IDS for i in ids))
    - id: ReadNodeResult
      type: ClassDef
      code: |-
        @dataclass(frozen=True)
        class ReadNodeResult:
            """Result of :func:`ast_read`.

            Attributes:
                nodes: One expanded subtree per resolved id, in the given order; same
                    shape as :func:`ast_find`'s results (see :class:`core.OutlineNode`).
                errors: One message per requested id that could not be resolved (id
                    unknown/ambiguous, and no unambiguous name/fuzzy match found).
            """
            nodes: list[core.OutlineNode]
            errors: list[str]
    - id: ast_read
      type: FunctionDef
      code: |-
        def ast_read(ids: list[str], path: str, *, with_lines: bool=True) -> ReadNodeResult:
            """Recursively read the subtree of each addressed node for block-wise edit/replace.

            Each id resolves to a subtree: a node whose body consists solely of nested
            classes/functions is expanded into ``children`` instead of source, so the agent
            can descend to the innermost editable block; any other node is returned whole,
            as ``code`` ready to hand back to ``ast_replace`` via its ``id``.

            An id that doesn't match any node is retried as a node *name* (exact, then a
            conservative fuzzy match) instead of failing the whole call; ids that still
            can't be resolved are reported in ``errors``, not raised.

            Args:
                ids: Node ids to read. Must be non-empty.
                path: Absolute path to the file to read.
                with_lines: Whether to populate each node's line range.

            Returns:
                ReadNodeResult: One subtree per resolved entry in ``ids``, plus errors for
                the rest.

            Raises:
                core.AstError: If ``ids`` is empty, ``path`` is not absolute or not an existing
                    regular file, or the source has a syntax error.
            """
            if not ids:
                raise core.AstError("'ids' must be a non-empty list of node ids.")
            tree = core.load(path)[1]
            nodes, errors = core.read_subtrees(core.locate_all(tree), ids, with_lines=with_lines)
            return ReadNodeResult(nodes=nodes, errors=errors)
    - id: ReadNodeTool
      type: ClassDef
      signature: "class ReadNodeTool(ToolDefinition):"
      children:
      - id: ReadNodeTool.ADVV8W
        type: statements
        code: |-
          name = 'ast_read'
          title = 'After using `ast_list` or `ast_find`, read AST subtrees for known node IDs'
          description = "After  using `ast_list` or `ast_find`, recursively read the subtree of each ID addressed AST node, surfacing each node's children, and source."
      - id: ReadNodeTool.B0qnog
        type: statements
        code: |-
          input_schema = {
              'type': 'object',
              'properties': {
                  'path': {
                      'type': 'string',
                      'description': 'Absolute path to the file.'},
                  'ids': {
                      'type': 'array',
                      'items': {
                              'type': 'string'},
                      'description': 'List of AST node ids to read.'}},
              'required': [
                  'ids',
                  'path']}
      - id: ReadNodeTool.vxU2fb
        type: statements
        code: |-
          output_schema = {
              '$defs': {
                  'outline_node': core.OUTLINE_NODE_SCHEMA},
              'type': 'object',
              'properties': {
                      'nodes': {
                          'type': 'array',
                          'items': {
                              '$ref': '#/$defs/outline_node'}},
                  'errors': {
                          'type': 'array',
                                  'items': {
                                      'type': 'string'}}},
              'required': ['nodes']}
          annotations = {'readOnlyHint': True, 'openWorldHint': False}
      - id: ReadNodeTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`ast_read`, translating the MCP schema to/from the AST API.

              Falls back to :func:`ast_list` when ``ids`` looks like agents habitually mis-guessing
              a root id (``root``/``_module_``/``module``/empty) and none of them resolve to a
              node, or ``ids`` is empty: returns the full outline instead of an error, and notes
              the redirect in ``errors``.
              """
              args: dict[str, Any] = ctx.arguments
              with_lines = bool({'tools', 'edit-lines'} & ctx.session.enabled_tools)
              ids: list[str] = args.get('ids') or []
              path = args.get('path')
              result: ReadNodeResult | None = None
              if ids:
                  try:
                      result = ast_read(ids=ids, path=path, with_lines=with_lines)
                  except core.AstError as exc:
                      return ToolResult(content=[text_content(str(exc))], is_error=True)
              if _looks_like_root_intent(ids) and (result is None or not result.nodes):
                  try:
                      list_result = ast_list(path=path, with_lines=with_lines)
                  except core.AstError as exc:
                      return ToolResult(content=[text_content(str(exc))], is_error=True)
                  return ToolResult(structured_content={'nodes': [core.to_dict(n) for n in list_result.nodes], 'errors': [
                                    f'ids {ids!r} resolved to no node; redirected to ast_list, returning the full outline instead.']})
              structured_content: dict[str, Any] = {'nodes': [core.to_dict(n) for n in result.nodes]}
              if result.errors:
                  structured_content['errors'] = result.errors
              return ToolResult(structured_content=structured_content)
    - id: register
      type: FunctionDef
      code: |-
        def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
            registry.register(ReadNodeTool())
            functions.register(ast_read)
```
/answer 379b-28 allow
Control Request:
```yaml
id: 379b-29
toolName: ast_read
arguments:
  ids:
  - mZlKhS
  - 3ctrsF
  - pMvmCp
  - ReplaceNodeResult
  - ast_replace
  - ReplaceNodeTool
  - register
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
  reason: Vollständigen Quellcode lesen für Batch-Umbau
```
/answer 379b-29 allow
Control Request:
```yaml
id: 379b-2a
result:
  structuredContent:
    nodes:
    - id: mZlKhS
      type: statements
      code: "'``ast_replace`` tool: replace the single selected node with new source.'"
    - id: 3ctrsF
      type: imports
      code: |-
        from dataclasses import dataclass
        from typing import Any
        from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
        from xy.ai.mcpc.tools.tool_context import ToolContext
        from xy.ai.mcpc.tools.ast import core
        from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_path
        from xy.ai.mcpc.tools.function_registry import FunctionRegistry
    - id: pMvmCp
      type: statements
      code: "__all__ = ['ReplaceNodeResult', 'ast_replace', 'ReplaceNodeTool', 'register']"
    - id: ReplaceNodeResult
      type: ClassDef
      code: |-
        @dataclass(frozen=True)
        class ReplaceNodeResult:
            """Result of :func:`ast_replace`.

            Attributes:
                result: Always ``"success"``.
                id: The node's new id, only set if the replacement changed it.
            """
            result: str
            id: str | None = None
    - id: ast_replace
      type: FunctionDef
      code: |-
        def ast_replace(path: str, source: str, *, id: str | None=None) -> ReplaceNodeResult:
            """Replace the single selected node with ``source``.

            Args:
                path: Absolute path to the file to modify.
                source: Replacement source.
                id: Unique id of the target node.

            Returns:
                ReplaceNodeResult: Success status and the node's new id, if changed.

            Raises:
                core.AstError: If ``path`` is invalid, ``source`` has a syntax error, ``id`` is
                    not given, or it matches zero or more than one node.
            """
            file_path = core.require_path(path)
            tree = core.CACHE.get_tree(file_path)
            target = select_by_path(tree, id=id)
            new_id = core.replace_node(target, source)
            core.CACHE.save(file_path, tree)
            return ReplaceNodeResult(result='success', id=new_id)
    - id: ReplaceNodeTool
      type: ClassDef
      signature: "class ReplaceNodeTool(ToolDefinition):"
      children:
      - id: ReplaceNodeTool.6LfsTB
        type: statements
        code: |-
          name = 'ast_replace'
          title = 'Replace AST node'
          description = 'Replace the single selected node with source or text.'
          input_schema = {
              'type': 'object',
              'properties': {
                  'path': {
                      'type': 'string',
                      'description': 'Absolute path to the file.'},
                  'source': {
                      'type': 'string',
                      'description': 'Replacement source.'},
                  **PATH_SELECTOR_PROPS},
              'required': [
                  'path',
                  'source']}
      - id: ReplaceNodeTool.oC5twa
        type: statements
        code: |-
          output_schema = {
              'type': 'object',
              'properties': {
                  'result': {
                      'type': 'string',
                      'description': 'Result status'},
                  'id': {
                      'type': 'string',
                      'description': "The node's new id."}},
              'required': ['result']}
          annotations = {'readOnlyHint': False, 'openWorldHint': False}
      - id: ReplaceNodeTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`ast_replace`, translating the MCP schema to/from the AST API."""
              args: dict[str, Any] = ctx.arguments
              try:
                  result = ast_replace(args['path'], args['source'], id=args.get('id'))
              except core.AstError as exc:
                  return ToolResult(content=[text_content(str(exc))], is_error=True)
              if result.id is not None:
                  message = f'Node {args.get('id')} was replaced with {result.id}'
              else:
                  message = f'Node ID {args.get('id')} unchanged'
              content = {'result': message}
              if result.id is not None:
                  content['id'] = result.id
              return ToolResult(content=[text_content(message)], structured_content=content, auto_approve=True)
    - id: register
      type: FunctionDef
      code: |-
        def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
            registry.register(ReplaceNodeTool())
            functions.register(ast_replace)
```
/answer 379b-2a allow
Control Request:
```yaml
id: 379b-2b
toolName: ast_read
arguments:
  ids:
  - jlFZZg
  - SMAply
  - mmIbkP
  - FileNodesResult
  - FindNodesResult
  - load_tolerant
  - find_in_file
  - ast_find
  - FindNodesTool
  - register
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
  reason: Vollständigen Quellcode lesen für Batch-Umbau und Verzeichnis-Expansion-Beispiel
```
/answer 379b-2b allow
Control Request:
```yaml
id: 379b-2c
result:
  structuredContent:
    nodes:
    - id: jlFZZg
      type: statements
      code: "'``ast_find`` tool: find AST nodes by type, name, id, line range or parent type.'"
    - id: SMAply
      type: imports
      code: |-
        import re
        from dataclasses import dataclass
        from pathlib import Path
        from typing import Any
        from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
        from xy.ai.mcpc.tools.tool_context import ToolContext
        from xy.ai.mcpc.tools.ast import core
        from xy.ai.mcpc.tools.ast.common import SELECTOR_PROPS
        from xy.ai.mcpc.tools.function_registry import FunctionRegistry
    - id: mmIbkP
      type: statements
      code: "__all__ = ['FileNodesResult', 'FindNodesResult', 'ast_find', 'FindNodesTool', 'register']"
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
    - id: load_tolerant
      type: FunctionDef
      code: |-
        def _load_tolerant(path: str) -> core.Tree:
            """Load *path*, tolerating a forgotten filename extension.

            If *path* does not exist, its parent directory is searched for a single
            file whose name (without extension) matches the given basename; if
            exactly one such file exists, it is loaded instead (the reported result
            path stays unchanged). Ambiguous or empty matches fall through to the
            original 'File not found.' error.
            """
            try:
                return core.load(path)[1]
            except core.AstError as exc:
                if str(exc) == 'File not found.':
                    parent = Path(path).parent
                    stem = Path(path).name
                    if parent.is_dir():
                        matches = [p for p in parent.iterdir() if p.is_file() and p.stem == stem]
                        if len(matches) == 1:
                            return core.load(str(matches[0]))[1]
                raise
    - id: find_in_file
      type: FunctionDef
      code: |-
        def _find_in_file(path: str, *, exact: dict[str, Any], lineno: int | None, end_lineno: int | None, no_selector: bool, pattern: re.Pattern[str] | None, with_lines: bool) -> FileNodesResult:
            tree = _load_tolerant(path)
            if no_selector:
                nodes = core.build_outline(core.locate_all(tree), with_code=True, with_lines=with_lines)
                return FileNodesResult(path=path, nodes=nodes)
            candidates = core.find(tree, **exact)
            if lineno is not None or end_lineno is not None:
                start = lineno if lineno is not None else end_lineno
                end = end_lineno if end_lineno is not None else lineno
                hit = core.most_specific(candidates, start, end)
                candidates = [hit] if hit is not None else []
            if pattern is None:
                nodes = core.build_outline(candidates, with_code=True, with_lines=with_lines)
                return FileNodesResult(path=path, nodes=nodes)
            source = tree.source
            seen: set[str] = set()
            ordered: list[core.Located] = []
            for m in pattern.finditer(source):
                start_line = source.count('\n', 0, m.start()) + 1
                end_line = source.count('\n', 0, max(m.end() - 1, m.start())) + 1
                loc = core.most_specific(candidates, start_line, end_line)
                if loc is not None and loc.node_id not in seen:
                    seen.add(loc.node_id)
                    ordered.append(loc)
            nodes = core.build_outline(ordered, with_code=True, with_lines=with_lines)
            return FileNodesResult(path=path, nodes=nodes)
    - id: ast_find
      type: FunctionDef
      code: |-
        def ast_find(paths: list[str], *, id: str | None=None, name: str | None=None, node_type: str | None=None, lineno: int | None=None, end_lineno: int | None=None, parent_type: str | None=None, text: str | None=None, regexp: str | None=None, with_lines: bool=True) -> FindNodesResult:
            """Find nodes by id, type, name, line range, parent type, text or regexp.

            ``ast_find`` is the single retrieval point that restricts on node properties;
            every other tool addresses nodes purely by ``id``. ``text``/``regexp`` are
            matched against each file's whole source, and each match is attributed to the
            most specific (smallest) enclosing node rather than to every ancestor whose
            source happens to contain it. Matches are returned with their full source.
            Called with no selector at all, ``ast_find`` returns the whole node tree per
            file, nested like ``ast_list`` but including source.

            Args:
                paths: Absolute paths of the files to search. Must be non-empty.
                id: Engine-independent unique node id (primarily name-based path).
                name: Exact simple name a node's ``name`` must equal.
                node_type: Node type name a node must match (case-insensitive).
                lineno: Line the target node must contain; selects the most specific
                    (smallest) matching node. Combined with ``end_lineno``, selects the
                    most specific node fully covering ``[lineno, end_lineno]``.
                end_lineno: End line of the target range; see ``lineno``. May be given
                    alone to select the most specific node containing that single line.
                parent_type: Node type name of the enclosing container (case-insensitive).
                text: Case-insensitive substring to search for in each file.
                regexp: Regular expression to search for in each file (``re.finditer``).
                with_lines: Whether to populate each match's line range.

            Returns:
                FindNodesResult: One :class:`FileNodesResult` per path, in order. Any
                number of matches per file (including zero) is a normal, successful result.

            Raises:
                core.AstError: If ``paths`` is empty, if any path is not absolute or does
                    not point to an existing regular file, if a file has a syntax error, or
                    if ``regexp`` is not a valid regular expression.
            """
            if not paths:
                raise core.AstError("'paths' must be a non-empty list.")
            exact = dict(id=id, name=name, node_type=node_type, parent_type=parent_type)
            structural = dict(exact, lineno=lineno, end_lineno=end_lineno)
            no_selector = not any(structural.values()) and text is None and (regexp is None)
            pattern: re.Pattern[str] | None = None
            if not no_selector and (text is not None or regexp is not None):
                if regexp is not None:
                    try:
                        pattern = re.compile(regexp)
                    except re.error as exc:
                        raise core.AstError(f'Invalid regexp: {exc}') from exc
                else:
                    pattern = re.compile(re.escape(text), re.IGNORECASE)
            files = [
                _find_in_file(
                    p,
                    exact=exact,
                    lineno=lineno,
                    end_lineno=end_lineno,
                    no_selector=no_selector,
                    pattern=pattern,
                    with_lines=with_lines) for p in paths]
            return FindNodesResult(files=files)
    - id: FindNodesTool
      type: ClassDef
      signature: "class FindNodesTool(ToolDefinition):"
      children:
      - id: FindNodesTool.vMKkuI
        type: statements
        code: |-
          name = 'ast_find'
          title = 'Find AST nodes'
          description = 'Filter the AST-node tree of a list of files by type, name, id, line range, parent type, text substring or regexp. Returns matches per file with their full source.'
      - id: FindNodesTool.nplwMP
        type: statements
        code: |-
          input_schema = {
              'type': 'object',
              'properties': {
                  'paths': {
                      'type': 'array',
                      'items': {
                          'type': 'string'},
                      'description': 'Absolute paths of the files to search.'},
                  **SELECTOR_PROPS,
                  'text': {
                      'type': 'string',
                              'description': "Case-insensitive substring the node's source must contain."},
                  'regexp': {
                      'type': 'string',
                      'description': "Regular expression the node's source must match (re.search)."}},
              'required': ['paths']}
      - id: FindNodesTool.ZaIxOu
        type: statements
        code: |-
          output_schema = {
              '$defs': {
                  'outline_node': core.OUTLINE_NODE_SCHEMA},
              'type': 'object',
              'properties': {
                      'files': {
                          'type': 'array',
                          'items': {
                              'type': 'object',
                              'properties': {
                                  'path': {
                                      'type': 'string'},
                                  'nodes': {
                                      'type': 'array',
                                      'items': {
                                              '$ref': '#/$defs/outline_node'}}},
                              'required': [
                                  'path',
                                  'nodes']}}},
              'required': ['files']}
      - id: FindNodesTool.e5owbr
        type: statements
        code: "annotations = {'readOnlyHint': True, 'openWorldHint': False}"
      - id: FindNodesTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`ast_find`, translating the MCP schema to/from the AST API."""
              args: dict[str, Any] = ctx.arguments
              paths = args.get('paths')
              if not isinstance(paths, list):
                  return ToolResult(content=[text_content("'paths' must be a non-empty list.")], is_error=True)
              with_lines = bool({'tools', 'edit-lines'} & ctx.session.enabled_tools)
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
    - id: register
      type: FunctionDef
      code: |-
        def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
            registry.register(FindNodesTool())
            functions.register(ast_find)
```
/answer 379b-2c allow
ReasoningToken: 25
Control Request:
```yaml
id: 379b-2d
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
  reason: Struktur von common.py verstehen für gemeinsame Batch-Hilfsfunktionen
```
/answer 379b-2d allow
Control Request:
```yaml
id: 379b-2e
result:
  structuredContent:
    nodes:
    - id: 1c3Fpi
      type: statements
    - id: 6eecMb
      type: imports
    - id: Qo4wec
      type: statements
    - id: CbzUdU
      type: statements
    - id: HPIj6Z
      type: statements
    - id: select_one
      type: FunctionDef
      signature: "def select_one(tree, **selectors: Any) -> core.Located:"
      docstring: "Return the single node in *tree* matching *selectors*. Raises: core.AstError: I…"
    - id: select_by_path
      type: FunctionDef
      signature: "def select_by_path(tree, *, id: str | None=None) -> core.Located:"
      docstring: "Return the single node in *tree* addressed by its unique ``id``. Raises: core.A…"
    - id: select_by_text
      type: FunctionDef
      signature: "def select_by_text(tree, texts: list[str], *, id: str | None=None) -> core.Located:"
      docstring: "Return the node addressed by ``id``, or, if omitted, the single node whose sour…"
    - id: list_output_schema
      type: FunctionDef
      signature: "def list_output_schema() -> dict[str, Any]:"
```
/answer 379b-2e allow
Control Request:
```yaml
id: 379b-2f
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  reason: "Core-API verstehen (AstError, AstAmbiguous, require_path, CACHE, etc.) für Batch-Umbau"
```
/answer 379b-2f allow
Control Request:
```yaml
id: 379b-30
result:
  structuredContent:
    nodes:
    - id: ue626f
      type: statements
    - id: aZH0Oy
      type: imports
    - id: RBFWwR
      type: statements
    - id: engine_for_path
      type: FunctionDef
      signature: "def engine_for_path(path: Path) -> Engine:"
      docstring: Return the engine responsible for ``path`` based on its extension.
    - id: CacheEntry
      type: ClassDef
      signature: "@dataclass class _CacheEntry:"
    - id: AstCache
      type: ClassDef
      signature: "class AstCache:"
      docstring: Content-hash validated cache of parsed trees keyed by absolute path.
      children:
      - id: AstCache.ayni5v
        type: statements
      - id: AstCache.init
        type: FunctionDef
        signature: "def __init__(self) -> None:"
      - id: AstCache.get_tree
        type: FunctionDef
        signature: "def get_tree(self, path: Path) -> Tree:"
      - id: AstCache.save
        type: FunctionDef
        signature: "def save(self, path: Path, tree: Tree) -> str:"
        docstring: "Serialise *tree*, write it to *path* and refresh the cache entry."
      - id: AstCache.invalidate
        type: FunctionDef
        signature: "def invalidate(self, path: Path) -> None:"
    - id: 5EG9eJ
      type: statements
    - id: load
      type: FunctionDef
      signature: "def load(path_str: str) -> tuple[Path, Tree]:"
      docstring: Resolve *path_str* and return it together with its cached tree.
    - id: parse_source
      type: FunctionDef
      signature: "def parse_source(source: str, engine: Engine | None=None) -> Tree:"
      docstring: Parse *source* with *engine* (Python by default for path-less snippets).
    - id: parse_for
      type: FunctionDef
      signature: "def parse_for(path_str: str, code: str) -> Tree:"
      docstring: Parse *code* with the engine selected for *path_str*'s extension.
    - id: locate_all
      type: FunctionDef
      signature: "def locate_all(tree: Tree) -> list[Located]:"
    - id: edit_node_source
      type: FunctionDef
      signature: "def edit_node_source(loc: Located) -> str:"
    - id: relocate
      type: FunctionDef
      signature: "def relocate(tree: Tree, old: Located) -> Located | None:"
      docstring: Find the node now occupying ``old``'s former slot after an in-place edit. Match…
    - id: replace_node
      type: FunctionDef
      signature: "def replace_node(loc: Located, code: str) -> str | None:"
      docstring: Replace ``loc``'s node with ``code``; return its new id.
    - id: insert_node
      type: FunctionDef
      signature: "def insert_node(loc: Located, code: str, position: str) -> int:"
    - id: delete_node
      type: FunctionDef
      signature: "def delete_node(loc: Located) -> None:"
    - id: append_nodes
      type: FunctionDef
      signature: "def append_nodes(tree: Tree, code: str) -> int:"
    - id: empty_tree
      type: FunctionDef
      signature: "def empty_tree(path: Path) -> Tree:"
    - id: validate_source
      type: FunctionDef
      signature: "def validate_source(path: Path, source: str) -> str | None:"
```
/answer 379b-30 allow
Control Request:
```yaml
id: 379b-31
toolName: ast_read
arguments:
  ids:
  - 1c3Fpi
  - 6eecMb
  - Qo4wec
  - CbzUdU
  - HPIj6Z
  - select_one
  - select_by_path
  - select_by_text
  - list_output_schema
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
  reason: Vollständigen Quellcode von common.py lesen
```
/answer 379b-31 allow
Control Request:
```yaml
id: 379b-32
result:
  structuredContent:
    nodes:
    - id: 1c3Fpi
      type: statements
      code: "'Selector machinery shared by the ``ast_*`` tools.\\n\\n``ast_find`` is the only tool that restricts on diverse node properties, so it\\nuses the full :data:`SELECTOR_PROPS`. Every mutation tool addresses a node purely\\nby its unique ``id`` and uses the reduced :data:`PATH_SELECTOR_PROPS`.\\n'"
    - id: 6eecMb
      type: imports
      code: |-
        from typing import Any
        from xy.ai.mcpc.tools.ast import core
    - id: Qo4wec
      type: statements
      code: |-
        __all__ = [
            'SELECTOR_PROPS',
            'PATH_SELECTOR_PROPS',
            'select_one',
            'select_by_path',
            'select_by_text',
            'list_output_schema']
        '#: Full node selectors – only ``ast_find`` may restrict on node properties.'
    - id: CbzUdU
      type: statements
      code: |-
        SELECTOR_PROPS = {
            'id': {
                'type': 'string', 'description': 'Unique node id'}, 'name': {
                    'type': 'string', 'description': 'Simple node name.'}, 'node_type': {
                        'type': 'string', 'description': "Node type name, e.g. 'FunctionDef' or 'pair'."}, 'lineno': {
                            'type': 'integer', 'description': 'Line in the target node.'}, 'end_lineno': {
                                'type': 'integer', 'description': 'End line of a range to get all nodes touching the lines.'}, 'parent_type': {
                                    'type': 'string', 'description': 'Node type name of the container.'}}
    - id: HPIj6Z
      type: statements
      code: |-
        '#: Path-only selectors used by every mutation tool (replace/insert/delete/edit_*).'
        PATH_SELECTOR_PROPS = {'id': SELECTOR_PROPS['id']}
    - id: select_one
      type: FunctionDef
      code: |-
        def select_one(tree, **selectors: Any) -> core.Located:
            """Return the single node in *tree* matching *selectors*.

            Raises:
                core.AstError: If no node matches, or more than one node matches.
            """
            hits = core.find(tree, **selectors)
            if not hits:
                raise core.AstError('No node matched the selector.')
            if len(hits) > 1:
                raise core.AstError(f'Selector is ambiguous – {len(hits)} nodes matched.')
            return hits[0]
    - id: select_by_path
      type: FunctionDef
      code: |-
        def select_by_path(tree, *, id: str | None=None) -> core.Located:
            """Return the single node in *tree* addressed by its unique ``id``.

            Raises:
                core.AstError: If ``id`` is missing, or it matches zero/many nodes.
            """
            if id is None:
                raise core.AstError('A node selector (id) is required.')
            return select_one(tree, id=id)
    - id: select_by_text
      type: FunctionDef
      code: |-
        def select_by_text(tree, texts: list[str], *, id: str | None=None) -> core.Located:
            """Return the node addressed by ``id``, or, if omitted, the single node whose
            source contains every string in *texts*.

            Nodes fully containing another matching node (an ancestor picked up merely
            because it contains the descendant) are dropped in favour of the innermost
            match; nesting alone is therefore not treated as ambiguity.

            Raises:
                core.AstError: If ``id`` is given but matches zero/many nodes, or no
                    node contains all of *texts*.
                core.AstAmbiguous: If several unrelated candidates remain after
                    dropping nested matches; carries their ids as ``candidates``.
            """
            if id is not None:
                return select_one(tree, id=id)
            candidates = [loc for loc in core.locate_all(tree) if all((text in core.edit_node_source(loc) for text in texts))]
            if not candidates:
                raise core.AstError('No node matched the given text; a node selector (id) is required.')
            candidates = [
                c for c in candidates if not any(
                    (other is not c and c.lineno <= other.lineno and (
                        c.end_lineno >= other.end_lineno) and (
                            (c.lineno,
                             c.end_lineno) != (
                                other.lineno,
                                other.end_lineno)) for other in candidates))]
            if len(candidates) > 1:
                raise core.AstAmbiguous(
                    'Multiple target nodes found; specify a node selector (id).', [
                        c.node_id for c in candidates])
            return candidates[0]
    - id: list_output_schema
      type: FunctionDef
      code: |-
        def list_output_schema() -> dict[str, Any]:
            return {'$defs': {'outline_node': core.OUTLINE_NODE_SCHEMA}, 'type': 'object', 'properties': {
                'nodes': {'type': 'array', 'items': {'$ref': '#/$defs/outline_node'}}}, 'required': ['nodes']}
```
/answer 379b-32 allow
Control Request:
```yaml
id: 379b-33
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  name: AstError
  reason: AstError-Klasse und verwandte Definitionen lesen
```
/answer 379b-33 allow
Control Request:
```yaml
id: 379b-34
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      nodes: []
```
/answer 379b-34 allow
Control Request:
```yaml
id: 379b-35
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  name: require_path
  reason: require_path Funktion lesen
```
/answer 379b-35 allow
Control Request:
```yaml
id: 379b-36
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      nodes: []
```
/answer 379b-36 allow
Control Request:
```yaml
id: 379b-37
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  name: AstAmbiguous
  reason: AstAmbiguous-Klasse lesen
```
/answer 379b-37 allow
Control Request:
```yaml
id: 379b-38
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      nodes: []
```
/answer 379b-38 allow
Control Request:
```yaml
id: 379b-39
toolName: grep
arguments:
  directory:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
  pattern: class AstError|class AstAmbiguous|def require_path
  include:
  - '*.py'
  exclude: []
  reason: "Fundstelle von AstError, AstAmbiguous, require_path finden"
```
/answer 379b-39 allow
Control Request:
```yaml
id: 379b-3a
result:
  structuredContent:
    matches:
    - path: base.py
      lineno: 23
      match: "class AstError(Exception):"
    - path: base.py
      lineno: 26
      match: "class AstAmbiguous(AstError):"
    - path: base.py
      lineno: 387
      match: "def require_path(path_str: str, *, must_exist: bool=True) -> Path:"
```
/answer 379b-3a allow
Control Request:
```yaml
id: 379b-3b
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  name: AstError
  reason: AstError-Klasse Details lesen
```
/answer 379b-3b allow
Control Request:
```yaml
id: 379b-3c
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      nodes:
      - id: AstError
        type: ClassDef
        code: |-
          class AstError(Exception):
              """A user-facing, path-free error raised by the AST tools."""
```
/answer 379b-3c allow
Control Request:
```yaml
id: 379b-3d
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  name: AstAmbiguous
  reason: AstAmbiguous-Klasse Details lesen
```
/answer 379b-3d allow
Control Request:
```yaml
id: 379b-3e
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      nodes:
      - id: AstAmbiguous
        type: ClassDef
        code: |-
          class AstAmbiguous(AstError):
              """Raised when a text-based node search (no id given) finds several
              unrelated candidates instead of a single node."""

              def __init__(self, message: str, candidates: list[str]) -> None:
                  super().__init__(message)
                  self.candidates = candidates
```
/answer 379b-3e allow
Control Request:
```yaml
id: 379b-3f
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  name: require_path
  reason: require_path Implementierung lesen
```
/answer 379b-3f allow
Control Request:
```yaml
id: 379b-40
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      nodes:
      - id: require_path
        type: FunctionDef
        code: |-
          def require_path(path_str: str, *, must_exist: bool=True) -> Path:
              """Validate a mandatory absolute path, raising :class:`AstError` on failure."""
              path = Path(path_str)
              if not path.is_absolute():
                  raise AstError('Path must be absolute.')
              if must_exist:
                  if not path.exists():
                      raise AstError('File not found.')
                  if not path.is_file():
                      raise AstError('Not a regular file.')
              return path
```
/answer 379b-40 allow
ReasoningToken: 129
ReasoningToken: 125
ReasoningToken: 39
ReasoningToken: 65
Control Request:
```yaml
id: 379b-41
toolName: grep
arguments:
  directory:
  - /home/user/xyan/xy.ai.workbench/mcpc/src
  pattern: from xy\.ai\.mcpc\.tools\.ast\.(create|delete|edit_block|edit_marks|insert|list|read|replace) import
  include:
  - '*.py'
  exclude: []
  reason: "Prüfen, ob Klassen/Funktionen der Tools an anderer Stelle importiert werden, um Kompatibilität zu wahren"
```
/answer 379b-41 allow
Control Request:
```yaml
id: 379b-42
result:
  structuredContent:
    matches:
    - path: xy/ai/mcpc/tools/ast/read.py
      lineno: 8
      match: from xy.ai.mcpc.tools.ast.list import ast_list
```
/answer 379b-42 allow
Control Request:
```yaml
id: 379b-43
toolName: grep
arguments:
  directory:
  - /home/user/xyan/xy.ai.workbench/mcpc/src
  pattern: ast_create\(|ast_delete\(|ast_edit_block\(|ast_edit_marks\(|ast_insert\(|ast_list\(|ast_read\(|ast_replace\(
  include:
  - '*.py'
  exclude:
  - '*/tools/ast/*'
  reason: "Prüfen, ob die Tool-Funktionen außerhalb ihres eigenen Moduls direkt aufgerufen werden"
```
/answer 379b-43 allow
Control Request:
```yaml
id: 379b-44
result:
  content:
  - type: text
    text: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
  structuredContent:
    matches:
    - path: xy/ai/mcpc/tools/ast/replace.py
      lineno: 22
      match: "def ast_replace(path: str, source: str, *, id: str | None=None) -> ReplaceNodeResult:"
    - path: xy/ai/mcpc/tools/ast/replace.py
      lineno: 77
      match: "            result = ast_replace(args['path'], args['source'], id=args.get('id'))"
    - path: xy/ai/mcpc/tools/ast/list.py
      lineno: 21
      match: "def ast_list(path: str, *, with_lines: bool=True) -> ListNodesResult:"
    - path: xy/ai/mcpc/tools/ast/list.py
      lineno: 63
      match: "            result = ast_list(path=args.get('path'), with_lines=with_lines)"
    - path: xy/ai/mcpc/tools/ast/delete.py
      lineno: 20
      match: "def ast_delete(path: str, *, id: str | None=None) -> DeleteResult:"
    - path: xy/ai/mcpc/tools/ast/delete.py
      lineno: 75
      match: "            result = ast_delete(args['path'], id=args.get('id'))"
    - path: xy/ai/mcpc/tools/ast/edit_marks.py
      lineno: 30
      match: "def ast_edit_marks(path: str, start_marker: str, end_marker: str, content: str, *, exact: bool=False, id: str | None=None) -> EditMarksNodeResult:"
    - path: xy/ai/mcpc/tools/ast/edit_marks.py
      lineno: 127
      match: '            result = ast_edit_marks('
    - path: xy/ai/mcpc/tools/ast/read.py
      lineno: 29
      match: "def ast_read(ids: list[str], path: str, *, with_lines: bool=True) -> ReadNodeResult:"
    - path: xy/ai/mcpc/tools/ast/read.py
      lineno: 109
      match: "                result = ast_read(ids=ids, path=path, with_lines=with_lines)"
    - path: xy/ai/mcpc/tools/ast/read.py
      lineno: 114
      match: "                list_result = ast_list(path=path, with_lines=with_lines)"
    - path: xy/ai/mcpc/tools/ast/edit_block.py
      lineno: 34
      match: "def ast_edit_block(path: str, old_text: str, new_text: str, *, exact: bool=False, replace_all: bool=False, id: str | None=None) -> EditBlockNodeResult:"
    - path: xy/ai/mcpc/tools/ast/edit_block.py
      lineno: 136
      match: '            result = ast_edit_block('
    - path: xy/ai/mcpc/tools/ast/create.py
      lineno: 19
      match: "def ast_create(path: str, source: str, overwrite: bool=False) -> CreateFileResult:"
    - path: xy/ai/mcpc/tools/ast/create.py
      lineno: 63
      match: "            result = ast_create(path=args['path'], source=args['source'], overwrite=args.get('overwrite', False))"
    warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 379b-44 allow
ReasoningToken: 21473
Control Request:
```yaml
id: 379b-45
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/function_registry.py
  reason: "Verstehen, wie FunctionRegistry.register die Python-Funktionen nutzt (Signatur-Constraints)"
```
/answer 379b-45 allow
Control Request:
```yaml
id: 379b-46
result:
  structuredContent:
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
```
/answer 379b-46 allow
ReasoningToken: 219
Control Request:
```yaml
id: 379b-47
toolName: ast_create
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
  overwrite: true
  reason: ast_create Tool um Batch-Support erweitern
  source: |
    """``ast_create`` tool: create one or more files from source, creating missing directories."""
    from dataclasses import dataclass
    from typing import Any
    from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import ToolContext
    from xy.ai.mcpc.tools.ast import core
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry

    __all__ = ['CreateFileItem', 'CreateFileResult', 'CreateFileError', 'CreateFilesResult', 'ast_create', 'CreateFileTool', 'register']


    @dataclass(frozen=True)
    class CreateFileItem:
        """One file to create.

        Attributes:
            path: Absolute path of the file to create.
            source: Source for the new file.
            overwrite: Allow replacing an existing file. Defaults to ``False``.
        """
        path: str
        source: str
        overwrite: bool = False


    @dataclass(frozen=True)
    class CreateFileResult:
        """Result of creating a single file.

        Attributes:
            path: The path exactly as given in the input, for result association.
            result: Always ``"success"``.
        """
        path: str
        result: str


    @dataclass(frozen=True)
    class CreateFileError:
        """Error creating a single file.

        Attributes:
            path: The path exactly as given in the input, for result association.
            error: The error message.
        """
        path: str
        error: str


    @dataclass(frozen=True)
    class CreateFilesResult:
        """Result of :func:`ast_create`.

        Attributes:
            results: One :class:`CreateFileResult` per file created successfully.
            errors: One :class:`CreateFileError` per file that failed.
        """
        results: list[CreateFileResult]
        errors: list[CreateFileError]


    def _create_one(item: CreateFileItem) -> CreateFileResult:
        file_path = core.require_path(item.path, must_exist=False)
        if file_path.exists() and (not item.overwrite):
            raise core.AstError('File already exists.')
        tree = core.parse_for(item.path, item.source)
        file_path.parent.mkdir(parents=True, exist_ok=True)
        core.CACHE.save(file_path, tree)
        return CreateFileResult(path=item.path, result='success')


    def ast_create(items: list[CreateFileItem]) -> CreateFilesResult:
        """Create one or more files from source (each validated by parsing it).

        Creating a single node in an existing file is covered by ``ast_insert``,
        not this tool.

        Args:
            items: Files to create. Must be non-empty.

        Returns:
            CreateFilesResult: One result per created file, one error per failed file.

        Raises:
            core.AstError: If ``items`` is empty.
        """
        if not items:
            raise core.AstError("'items' must be a non-empty list.")
        results: list[CreateFileResult] = []
        errors: list[CreateFileError] = []
        for item in items:
            try:
                results.append(_create_one(item))
            except core.AstError as exc:
                errors.append(CreateFileError(path=item.path, error=str(exc)))
        return CreateFilesResult(results=results, errors=errors)


    class CreateFileTool(ToolDefinition):
        name = 'ast_create'
        title = 'Create files'
        description = 'Create one or more files from source.'
        input_schema = {
            'type': 'object',
            'properties': {
                'items': {
                    'type': 'array',
                    'minItems': 1,
                    'items': {
                        'type': 'object',
                        'properties': {
                            'path': {'type': 'string', 'description': 'Absolute path of the file to create.'},
                            'source': {'type': 'string', 'description': 'Source for the new file.'},
                            'overwrite': {'type': 'boolean', 'description': 'Allow replacing an existing file.', 'default': False},
                        },
                        'required': ['path', 'source'],
                    },
                    'description': 'Files to create.',
                },
            },
            'required': ['items'],
        }
        output_schema = {
            'type': 'object',
            'properties': {
                'results': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {'path': {'type': 'string'}, 'result': {'type': 'string'}},
                        'required': ['path', 'result'],
                    },
                },
                'errors': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {'path': {'type': 'string'}, 'error': {'type': 'string'}},
                        'required': ['path', 'error'],
                    },
                },
            },
            'required': ['results', 'errors'],
        }
        annotations = {'readOnlyHint': False, 'openWorldHint': False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`ast_create`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            raw_items = args.get('items') or []
            if not raw_items:
                return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
            items = [CreateFileItem(path=it['path'], source=it['source'], overwrite=it.get('overwrite', False)) for it in raw_items]
            batch = ast_create(items)
            content = {
                'results': [{'path': r.path, 'result': r.result} for r in batch.results],
                'errors': [{'path': e.path, 'error': e.error} for e in batch.errors],
            }
            is_error = bool(batch.errors) and (not batch.results)
            return ToolResult(structured_content=content, is_error=is_error, auto_approve=not is_error)


    def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
        registry.register(CreateFileTool())
        functions.register(ast_create)
```
/answer 379b-47 allow
Control Request:
```yaml
id: 379b-48
toolName: ast_create
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
  overwrite: true
  reason: ast_delete Tool um Batch-Support erweitern
  source: |
    """``ast_delete`` tool: delete selected nodes, or whole files if none selected."""
    from dataclasses import dataclass
    from typing import Any
    from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import ToolContext
    from xy.ai.mcpc.tools.ast import core
    from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_path
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry

    __all__ = ['DeleteItem', 'DeleteResult', 'DeleteError', 'DeleteBatchResult', 'ast_delete', 'DeleteTool', 'register']


    @dataclass(frozen=True)
    class DeleteItem:
        """One node (or file) to delete.

        Attributes:
            path: Absolute path to the file to modify.
            id: Unique id of the target node. Omit to delete the whole file.
        """
        path: str
        id: str | None = None


    @dataclass(frozen=True)
    class DeleteResult:
        """Result of deleting a single node or file.

        Attributes:
            path: The path exactly as given in the input, for result association.
            id: The id exactly as given in the input, for result association.
            result: Always ``"success"``.
        """
        path: str
        id: str | None
        result: str


    @dataclass(frozen=True)
    class DeleteError:
        """Error deleting a single node or file.

        Attributes:
            path: The path exactly as given in the input, for result association.
            id: The id exactly as given in the input, for result association.
            error: The error message.
        """
        path: str
        id: str | None
        error: str


    @dataclass(frozen=True)
    class DeleteBatchResult:
        """Result of :func:`ast_delete`.

        Attributes:
            results: One :class:`DeleteResult` per successfully deleted item.
            errors: One :class:`DeleteError` per item that failed.
        """
        results: list[DeleteResult]
        errors: list[DeleteError]


    def _delete_one(item: DeleteItem) -> DeleteResult:
        file_path = core.require_path(item.path)
        if item.id is None:
            try:
                file_path.unlink()
            except OSError as exc:
                raise core.AstError('Delete failed.') from exc
            core.CACHE.invalidate(file_path)
            parent = file_path.parent
            if not any(parent.iterdir()):
                parent.rmdir()
            return DeleteResult(path=item.path, id=None, result='success')
        tree = core.CACHE.get_tree(file_path)
        target = select_by_path(tree, id=item.id)
        core.delete_node(target)
        core.CACHE.save(file_path, tree)
        return DeleteResult(path=item.path, id=item.id, result='success')


    def ast_delete(items: list[DeleteItem]) -> DeleteBatchResult:
        """Delete one or more selected nodes, or whole files if their ``id`` is omitted.

        A whole file is deleted by omitting its ``id`` selector – there is no other
        way to address the root, since it is never itself an addressable child.
        Deleting a file also removes it from the AST cache and, if its parent
        directory becomes empty as a result, removes that directory too.

        Args:
            items: Nodes/files to delete. Must be non-empty.

        Returns:
            DeleteBatchResult: One result per deleted item, one error per failed item.

        Raises:
            core.AstError: If ``items`` is empty.
        """
        if not items:
            raise core.AstError("'items' must be a non-empty list.")
        results: list[DeleteResult] = []
        errors: list[DeleteError] = []
        for item in items:
            try:
                results.append(_delete_one(item))
            except core.AstError as exc:
                errors.append(DeleteError(path=item.path, id=item.id, error=str(exc)))
        return DeleteBatchResult(results=results, errors=errors)


    class DeleteTool(ToolDefinition):
        name = 'ast_delete'
        title = 'Delete AST nodes or files'
        description = 'Delete selected nodes from files, or whole files if no selector is given, for a batch of items.'
        input_schema = {
            'type': 'object',
            'properties': {
                'items': {
                    'type': 'array',
                    'minItems': 1,
                    'items': {
                        'type': 'object',
                        'properties': {
                            'path': {'type': 'string', 'description': 'Absolute path to the file.'},
                            **PATH_SELECTOR_PROPS,
                        },
                        'required': ['path'],
                    },
                    'description': 'Nodes/files to delete.',
                },
            },
            'required': ['items'],
        }
        output_schema = {
            'type': 'object',
            'properties': {
                'results': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'path': {'type': 'string'},
                            'id': {'type': 'string'},
                            'result': {'type': 'string'},
                        },
                        'required': ['path', 'result'],
                    },
                },
                'errors': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'path': {'type': 'string'},
                            'id': {'type': 'string'},
                            'error': {'type': 'string'},
                        },
                        'required': ['path', 'error'],
                    },
                },
            },
            'required': ['results', 'errors'],
        }
        annotations = {'readOnlyHint': False, 'openWorldHint': False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`ast_delete`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            raw_items = args.get('items') or []
            if not raw_items:
                return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
            items = [DeleteItem(path=it['path'], id=it.get('id')) for it in raw_items]
            batch = ast_delete(items)
            content = {
                'results': [{'path': r.path, 'id': r.id, 'result': r.result} for r in batch.results],
                'errors': [{'path': e.path, 'id': e.id, 'error': e.error} for e in batch.errors],
            }
            is_error = bool(batch.errors) and (not batch.results)
            return ToolResult(structured_content=content, is_error=is_error, auto_approve=not is_error)


    def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
        registry.register(DeleteTool())
        functions.register(ast_delete)
```
/answer 379b-48 allow
Control Request:
```yaml
id: 379b-49
toolName: ast_create
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  overwrite: true
  reason: ast_edit_block Tool um Batch-Support erweitern
  source: |
    """``ast_edit_block`` tool: exact-block (old_text -> new_text) edits within selected nodes."""
    from dataclasses import dataclass
    from typing import Any
    from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import ToolContext
    from xy.ai.mcpc.tools.ast import core
    from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_text
    from xy.ai.mcpc.tools._text_match import replace_in_block, line_preserving, TextMatchError
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry

    __all__ = ['EditBlockItem', 'EditBlockResult', 'EditBlockError', 'EditBlockBatchResult', 'ast_edit_block', 'EditBlockNodeTool', 'register']


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


    @dataclass(frozen=True)
    class EditBlockBatchResult:
        """Result of :func:`ast_edit_block`.

        Attributes:
            results: One :class:`EditBlockResult` per successfully applied edit.
            errors: One :class:`EditBlockError` per edit that failed.
        """
        results: list[EditBlockResult]
        errors: list[EditBlockError]


    def _node_guard(engine, reference):
        """Guard for tolerant node edits.

        Engines that flag malformed edits on replace need no extra check. Others
        (e.g. markup grammars) must not merge lines and must still re-parse cleanly.
        """
        if engine.validates_syntax:
            return None
        keep_lines = line_preserving(reference)
        return lambda span, result: keep_lines(span, result) and engine.validate(result) is None


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
                accept=_node_guard(tree.engine, item.old_text),
                max_level=3 if tree.engine.validates_syntax else 2,
                where='node')
        except TextMatchError as exc:
            raise core.AstError(str(exc)) from exc
        new_id = core.replace_node(target, new_source)
        core.CACHE.save(file_path, tree)
        return EditBlockResult(path=item.path, id=item.id, result='success', new_id=new_id)


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


    class EditBlockNodeTool(ToolDefinition):
        name = 'ast_edit_block'
        title = 'Replace short text within AST nodes'
        description = "Replace occurrence(s) of short 'old_text' with 'new_text', within nodes addressed by id, for a batch of items. Don't use for large edits, use ast_edit_marks instead."
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
                            'path': {'type': 'string', 'description': 'Absolute path to the file.'},
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
                            **PATH_SELECTOR_PROPS,
                        },
                        'required': ['path', 'old_text', 'new_text'],
                    },
                    'description': 'Block edits to apply.',
                },
            },
            'required': ['items'],
        }
        output_schema = {
            'type': 'object',
            'properties': {
                'results': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'path': {'type': 'string'},
                            'id': {'type': 'string'},
                            'result': {'type': 'string', 'description': 'Result status'},
                            'new_id': {'type': 'string', 'description': "The node's new id. Prefer 'ast_find' over 'ast_list'."},
                        },
                        'required': ['path', 'result'],
                    },
                },
                'errors': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'path': {'type': 'string'},
                            'id': {'type': 'string'},
                            'error': {'type': 'string'},
                            'candidates': {
                                'type': 'array',
                                'items': {'type': 'string'},
                                'description': 'On ambiguity (id omitted, several nodes matched), the candidate node ids.'},
                        },
                        'required': ['path', 'error'],
                    },
                },
            },
            'required': ['results', 'errors'],
        }
        annotations = {'readOnlyHint': False, 'openWorldHint': False}

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
                    exact=it.get('exact', False),
                    replace_all=it.get('replaceAll', False),
                    id=it.get('id'))
                for it in raw_items]
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
            return ToolResult(structured_content={'results': results, 'errors': errors}, is_error=is_error, auto_approve=not is_error)


    def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
        registry.register(EditBlockNodeTool())
        functions.register(ast_edit_block)
```
/answer 379b-49 allow
Control Request:
```yaml
id: 379b-4a
toolName: ast_create
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  overwrite: true
  reason: ast_edit_marks Tool um Batch-Support erweitern
  source: |
    """``ast_edit_marks`` tool: mark-based edits within the source of selected nodes."""
    from dataclasses import dataclass
    from typing import Any
    from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import ToolContext
    from xy.ai.mcpc.tools.ast import core
    from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_text
    from xy.ai.mcpc.tools._text_match import replace_between, marks_line_preserving, TextMatchError
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry

    __all__ = ['EditMarksItem', 'EditMarksResult', 'EditMarksError', 'EditMarksBatchResult', 'ast_edit_marks', 'EditMarksNodeTool', 'register']


    @dataclass(frozen=True)
    class EditMarksItem:
        """One marker-based edit to apply.

        Attributes:
            path: Absolute path to the file to modify.
            start_marker: Unique 10-30 char substring marking the beginning of the block, within the node's source.
            end_marker: Unique 10-30 char substring marking the end of the block, within the node's source.
            content: Replacement source for the marked block.
            exact: If False (default), whitespace in start/end is matched tolerantly.
            id: Unique id of the target node. If omitted, the node is searched for by the markers.
        """
        path: str
        start_marker: str
        end_marker: str
        content: str
        exact: bool = False
        id: str | None = None


    @dataclass(frozen=True)
    class EditMarksResult:
        """Result of applying a single marker-based edit.

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


    @dataclass(frozen=True)
    class EditMarksError:
        """Error applying a single marker-based edit.

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


    @dataclass(frozen=True)
    class EditMarksBatchResult:
        """Result of :func:`ast_edit_marks`.

        Attributes:
            results: One :class:`EditMarksResult` per successfully applied edit.
            errors: One :class:`EditMarksError` per edit that failed.
        """
        results: list[EditMarksResult]
        errors: list[EditMarksError]


    def _node_marks_guard(engine, begin_marker, end_marker):
        """Guard for tolerant marker edits (see ``ast.edit_block._node_guard``)."""
        if engine.validates_syntax:
            return None
        keep_lines = marks_line_preserving(begin_marker, end_marker)
        return lambda begin, end, result: keep_lines(begin, end, result) and engine.validate(result) is None


    def _edit_marks_one(item: EditMarksItem) -> EditMarksResult:
        file_path = core.require_path(item.path)
        tree = core.CACHE.get_tree(file_path)
        target = select_by_text(tree, [item.start_marker, item.end_marker], id=item.id)
        node_source = core.edit_node_source(target)
        begin, end = (item.start_marker, item.end_marker) if item.exact else (item.start_marker.strip(), item.end_marker.strip())
        try:
            new_source = replace_between(
                node_source, begin, end, item.content,
                exact=item.exact,
                accept=_node_marks_guard(tree.engine, begin, end),
                max_level=3 if tree.engine.validates_syntax else 2,
                where='node')
        except TextMatchError as exc:
            raise core.AstError(str(exc)) from exc
        new_id = core.replace_node(target, new_source)
        core.CACHE.save(file_path, tree)
        return EditMarksResult(path=item.path, id=item.id, result='success', new_id=new_id)


    def ast_edit_marks(items: list[EditMarksItem]) -> EditMarksBatchResult:
        """Replace everything between the start/end markers inside one or more nodes.

        Each addressed node's source is unparsed, edited between the two markers
        (both included) with escalating whitespace/escape/quote tolerance,
        re-parsed, and used to replace the node.

        Args:
            items: Marker-based edits to apply. Must be non-empty.

        Returns:
            EditMarksBatchResult: One result per successful edit, one error per failed edit.

        Raises:
            core.AstError: If ``items`` is empty.
        """
        if not items:
            raise core.AstError("'items' must be a non-empty list.")
        results: list[EditMarksResult] = []
        errors: list[EditMarksError] = []
        for item in items:
            try:
                results.append(_edit_marks_one(item))
            except core.AstAmbiguous as exc:
                errors.append(EditMarksError(path=item.path, id=item.id, error=str(exc), candidates=exc.candidates))
            except core.AstError as exc:
                errors.append(EditMarksError(path=item.path, id=item.id, error=str(exc)))
        return EditMarksBatchResult(results=results, errors=errors)


    class EditMarksNodeTool(ToolDefinition):
        name = 'ast_edit_marks'
        title = 'Replace large text regions within AST nodes between markers'
        description = "Replace everything between and including the unique 'start_marker' and 'end_marker' markers, found within nodes addressed by id, with new 'content', for a batch of items."
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
                            'path': {'type': 'string', 'description': 'Absolute path to the file.'},
                            'start_marker': {
                                'type': 'string',
                                'minLength': 10,
                                'maxLength': 30,
                                'description': "Unique 10-30 char substring marking the beginning of the text to replace, within the node's source."},
                            'end_marker': {
                                'type': 'string',
                                'minLength': 10,
                                'maxLength': 30,
                                'description': "Unique 10-30 char substring marking the end of the text to replace, within the node's source."},
                            'content': {'type': 'string', 'description': 'Replacement source for the marked text.'},
                            'exact': {
                                'type': 'boolean',
                                'description': "If true, 'start_marker'/'end_marker' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                                'default': False},
                            **PATH_SELECTOR_PROPS,
                        },
                        'required': ['path', 'start_marker', 'end_marker', 'content'],
                    },
                    'description': 'Marker-based edits to apply.',
                },
            },
            'required': ['items'],
        }
        output_schema = {
            'type': 'object',
            'properties': {
                'results': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'path': {'type': 'string'},
                            'id': {'type': 'string'},
                            'result': {'type': 'string', 'description': 'Result status'},
                            'new_id': {'type': 'string', 'description': "The node's new id. Prefer 'ast_find' over 'ast_list'."},
                        },
                        'required': ['path', 'result'],
                    },
                },
                'errors': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'path': {'type': 'string'},
                            'id': {'type': 'string'},
                            'error': {'type': 'string'},
                            'candidates': {
                                'type': 'array',
                                'items': {'type': 'string'},
                                'description': 'On ambiguity (id omitted, several nodes matched), the candidate node ids.'},
                        },
                        'required': ['path', 'error'],
                    },
                },
            },
            'required': ['results', 'errors'],
        }
        annotations = {'readOnlyHint': False, 'openWorldHint': False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`ast_edit_marks`, translating the MCP schema to/from the AST API."""
            args: dict[str, Any] = ctx.arguments
            raw_items = args.get('items') or []
            if not raw_items:
                return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
            items = [
                EditMarksItem(
                    path=it['path'],
                    start_marker=it['start_marker'],
                    end_marker=it['end_marker'],
                    content=it['content'],
                    exact=it.get('exact', False),
                    id=it.get('id'))
                for it in raw_items]
            batch = ast_edit_marks(items)
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
            return ToolResult(structured_content={'results': results, 'errors': errors}, is_error=is_error, auto_approve=not is_error)


    def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
        registry.register(EditMarksNodeTool())
        functions.register(ast_edit_marks)
```
/answer 379b-4a allow
Control Request:
```yaml
id: 379b-4b
toolName: ast_create
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
  overwrite: true
  reason: ast_insert Tool um Batch-Support erweitern
  source: |
    """``ast_insert`` tool: insert statement(s) relative to selected nodes."""
    from dataclasses import dataclass
    from typing import Any
    from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import ToolContext
    from xy.ai.mcpc.tools.ast import core
    from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_path
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry

    __all__ = ['InsertItem', 'InsertResult', 'InsertError', 'InsertBatchResult', 'ast_insert', 'InsertNodeTool', 'register']


    @dataclass(frozen=True)
    class InsertItem:
        """One insert operation.

        Attributes:
            path: Absolute path to the file to modify.
            source: Source of the statement(s) to insert.
            position: ``"before"`` or ``"after"`` the selected node. Defaults to ``"after"``.
            id: Unique id of the target node.
        """
        path: str
        source: str
        position: str = 'after'
        id: str | None = None


    @dataclass(frozen=True)
    class InsertResult:
        """Result of a single insert operation.

        Attributes:
            path: The path exactly as given in the input, for result association.
            id: The id exactly as given in the input, for result association.
            result: Always ``"success"``.
            inserted: Number of top-level statements parsed from ``source`` and inserted.
            ids: The newly inserted top-level node(s)' ids.
        """
        path: str
        id: str | None
        result: str
        inserted: int
        ids: list[str] | None = None


    @dataclass(frozen=True)
    class InsertError:
        """Error applying a single insert operation.

        Attributes:
            path: The path exactly as given in the input, for result association.
            id: The id exactly as given in the input, for result association.
            error: The error message.
        """
        path: str
        id: str | None
        error: str


    @dataclass(frozen=True)
    class InsertBatchResult:
        """Result of :func:`ast_insert`.

        Attributes:
            results: One :class:`InsertResult` per successful insert.
            errors: One :class:`InsertError` per insert that failed.
        """
        results: list[InsertResult]
        errors: list[InsertError]


    def _insert_one(item: InsertItem) -> InsertResult:
        file_path = core.require_path(item.path)
        tree = core.CACHE.get_tree(file_path)
        target = select_by_path(tree, id=item.id)
        before_ids = {loc.node_id for loc in core.locate_all(tree)}
        inserted = core.insert_node(target, item.source, item.position)
        new_ids = [loc.node_id for loc in core.locate_all(tree) if loc.node_id not in before_ids]
        core.CACHE.save(file_path, tree)
        return InsertResult(path=item.path, id=item.id, result='success', inserted=inserted, ids=new_ids or None)


    def ast_insert(items: list[InsertItem]) -> InsertBatchResult:
        """Insert statement(s) parsed from ``source`` relative to one or more selected nodes.

        Several operations may target the same path (also across several paths);
        each item is applied in order.

        Args:
            items: Insert operations to apply. Must be non-empty.

        Returns:
            InsertBatchResult: One result per successful insert, one error per failed insert.

        Raises:
            core.AstError: If ``items`` is empty.
        """
        if not items:
            raise core.AstError("'items' must be a non-empty list.")
        results: list[InsertResult] = []
        errors: list[InsertError] = []
        for item in items:
            try:
                results.append(_insert_one(item))
            except core.AstError as exc:
                errors.append(InsertError(path=item.path, id=item.id, error=str(exc)))
        return InsertBatchResult(results=results, errors=errors)


    class InsertNodeTool(ToolDefinition):
        name = 'ast_insert'
        title = 'Insert AST nodes'
        description = "Insert source relative to selected nodes ('before' or 'after'), for a batch of items; several operations may target the same or different files."
        input_schema = {
            'type': 'object',
            'properties': {
                'items': {
                    'type': 'array',
                    'minItems': 1,
                    'items': {
                        'type': 'object',
                        'properties': {
                            'path': {'type': 'string', 'description': 'Absolute path to the file.'},
                            'source': {'type': 'string', 'description': 'Source to insert.'},
                            'position': {
                                'type': 'string',
                                'enum': ['before', 'after'],
                                'description': 'Placement relative to the selected node.',
                                'default': 'after'},
                            **PATH_SELECTOR_PROPS,
                        },
                        'required': ['path', 'source'],
                    },
                    'description': 'Insert operations to apply.',
                },
            },
            'required': ['items'],
        }
        output_schema = {
            'type': 'object',
            'properties': {
                'results': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'path': {'type': 'string'},
                            'id': {'type': 'string'},
                            'result': {'type': 'string'},
                            'inserted': {'type': 'integer'},
                            'ids': {
                                'type': 'array',
                                'items': {'type': 'string'},
                                'description': 'The newly inserted node(s) ids.'},
                        },
                        'required': ['path', 'result', 'inserted'],
                    },
                },
                'errors': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'path': {'type': 'string'},
                            'id': {'type': 'string'},
                            'error': {'type': 'string'},
                        },
                        'required': ['path', 'error'],
                    },
                },
            },
            'required': ['results', 'errors'],
        }
        annotations = {'readOnlyHint': False, 'openWorldHint': False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`ast_insert`, translating the MCP schema to/from the AST API."""
            args: dict[str, Any] = ctx.arguments
            raw_items = args.get('items') or []
            if not raw_items:
                return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
            items = [
                InsertItem(path=it['path'], source=it['source'], position=it.get('position', 'after'), id=it.get('id'))
                for it in raw_items]
            batch = ast_insert(items)
            results = []
            for r in batch.results:
                entry = {'path': r.path, 'id': r.id, 'result': r.result, 'inserted': r.inserted}
                if r.ids is not None:
                    entry['ids'] = r.ids
                results.append(entry)
            errors = [{'path': e.path, 'id': e.id, 'error': e.error} for e in batch.errors]
            is_error = bool(batch.errors) and (not batch.results)
            return ToolResult(structured_content={'results': results, 'errors': errors}, is_error=is_error, auto_approve=not is_error)


    def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
        registry.register(InsertNodeTool())
        functions.register(ast_insert)
```
/answer 379b-4b allow
Control Request:
```yaml
id: 379b-4c
toolName: ast_create
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
  overwrite: true
  reason: ast_replace Tool um Batch-Support erweitern
  source: |
    """``ast_replace`` tool: replace selected nodes with new source."""
    from dataclasses import dataclass
    from typing import Any
    from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import ToolContext
    from xy.ai.mcpc.tools.ast import core
    from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_path
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry

    __all__ = ['ReplaceItem', 'ReplaceResult', 'ReplaceError', 'ReplaceBatchResult', 'ast_replace', 'ReplaceNodeTool', 'register']


    @dataclass(frozen=True)
    class ReplaceItem:
        """One node replacement to apply.

        Attributes:
            path: Absolute path to the file to modify.
            source: Replacement source.
            id: Unique id of the target node.
        """
        path: str
        source: str
        id: str | None = None


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


    @dataclass(frozen=True)
    class ReplaceBatchResult:
        """Result of :func:`ast_replace`.

        Attributes:
            results: One :class:`ReplaceResult` per successful replacement.
            errors: One :class:`ReplaceError` per replacement that failed.
        """
        results: list[ReplaceResult]
        errors: list[ReplaceError]


    def _replace_one(item: ReplaceItem) -> ReplaceResult:
        file_path = core.require_path(item.path)
        tree = core.CACHE.get_tree(file_path)
        target = select_by_path(tree, id=item.id)
        new_id = core.replace_node(target, item.source)
        core.CACHE.save(file_path, tree)
        return ReplaceResult(path=item.path, id=item.id, result='success', new_id=new_id)


    def ast_replace(items: list[ReplaceItem]) -> ReplaceBatchResult:
        """Replace one or more selected nodes with new source.

        ``id`` and replacement ``source`` stay together per item, since replacing
        the same node twice makes no sense; several items may target the same or
        different files.

        Args:
            items: Node replacements to apply. Must be non-empty.

        Returns:
            ReplaceBatchResult: One result per successful replacement, one error per
            failed replacement.

        Raises:
            core.AstError: If ``items`` is empty.
        """
        if not items:
            raise core.AstError("'items' must be a non-empty list.")
        results: list[ReplaceResult] = []
        errors: list[ReplaceError] = []
        for item in items:
            try:
                results.append(_replace_one(item))
            except core.AstError as exc:
                errors.append(ReplaceError(path=item.path, id=item.id, error=str(exc)))
        return ReplaceBatchResult(results=results, errors=errors)


    class ReplaceNodeTool(ToolDefinition):
        name = 'ast_replace'
        title = 'Replace AST nodes'
        description = 'Replace selected nodes with source or text, for a batch of items; several operations may target the same or different files.'
        input_schema = {
            'type': 'object',
            'properties': {
                'items': {
                    'type': 'array',
                    'minItems': 1,
                    'items': {
                        'type': 'object',
                        'properties': {
                            'path': {'type': 'string', 'description': 'Absolute path to the file.'},
                            'source': {'type': 'string', 'description': 'Replacement source.'},
                            **PATH_SELECTOR_PROPS,
                        },
                        'required': ['path', 'source'],
                    },
                    'description': 'Node replacements to apply.',
                },
            },
            'required': ['items'],
        }
        output_schema = {
            'type': 'object',
            'properties': {
                'results': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'path': {'type': 'string'},
                            'id': {'type': 'string'},
                            'result': {'type': 'string', 'description': 'Result status'},
                            'new_id': {'type': 'string', 'description': "The node's new id."},
                        },
                        'required': ['path', 'result'],
                    },
                },
                'errors': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'path': {'type': 'string'},
                            'id': {'type': 'string'},
                            'error': {'type': 'string'},
                        },
                        'required': ['path', 'error'],
                    },
                },
            },
            'required': ['results', 'errors'],
        }
        annotations = {'readOnlyHint': False, 'openWorldHint': False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`ast_replace`, translating the MCP schema to/from the AST API."""
            args: dict[str, Any] = ctx.arguments
            raw_items = args.get('items') or []
            if not raw_items:
                return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
            items = [ReplaceItem(path=it['path'], source=it['source'], id=it.get('id')) for it in raw_items]
            batch = ast_replace(items)
            results = []
            for r in batch.results:
                entry = {'path': r.path, 'id': r.id, 'result': r.result}
                if r.new_id is not None:
                    entry['new_id'] = r.new_id
                results.append(entry)
            errors = [{'path': e.path, 'id': e.id, 'error': e.error} for e in batch.errors]
            is_error = bool(batch.errors) and (not batch.results)
            return ToolResult(structured_content={'results': results, 'errors': errors}, is_error=is_error, auto_approve=not is_error)


    def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
        registry.register(ReplaceNodeTool())
        functions.register(ast_replace)
```
/answer 379b-4c allow
Control Request:
```yaml
id: 379b-4d
toolName: ast_create
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/list.py
  overwrite: true
  reason: ast_list Tool um Batch-Support und Verzeichnis-Expansion erweitern
  source: |
    """``ast_list`` tool: list AST nodes of one or more files."""
    from dataclasses import dataclass
    from pathlib import Path
    from typing import Any
    from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import ToolContext
    from xy.ai.mcpc.tools.ast import core
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry

    __all__ = ['ListNodesResult', 'ListNodesError', 'ListNodesBatchResult', 'ast_list', 'ListNodesTool', 'register']

    _MAX_DIR_EXPANSION = 5


    @dataclass(frozen=True)
    class ListNodesResult:
        """Nodes of a single file.

        Attributes:
            path: The resolved path of the listed file (see ``ast_list``'s
                directory-expansion note), for result association.
            nodes: Outline-style node descriptions (see :class:`core.OutlineNode`), in
                document order, suited for retrieval and navigation.
        """
        path: str
        nodes: list[core.OutlineNode]


    @dataclass(frozen=True)
    class ListNodesError:
        """Error listing a single file.

        Attributes:
            path: The resolved path that failed, for result association.
            error: The error message.
        """
        path: str
        error: str


    @dataclass(frozen=True)
    class ListNodesBatchResult:
        """Result of :func:`ast_list`.

        Attributes:
            results: One :class:`ListNodesResult` per successfully listed file.
            errors: One :class:`ListNodesError` per file that failed.
        """
        results: list[ListNodesResult]
        errors: list[ListNodesError]


    def _expand_path(path_str: str) -> list[str]:
        """Expand *path_str* to its own single-element list, unless it names a
        directory that can be unambiguously expanded to its files.

        Expansion only succeeds if the directory has no subdirectories and holds
        at most :data:`_MAX_DIR_EXPANSION` files; otherwise *path_str* is returned
        unchanged and left to fail normally.
        """
        p = Path(path_str)
        if not p.is_dir():
            return [path_str]
        entries = list(p.iterdir())
        if any(e.is_dir() for e in entries):
            return [path_str]
        files = sorted((e for e in entries if e.is_file()), key=lambda e: e.name)
        if not files or len(files) > _MAX_DIR_EXPANSION:
            return [path_str]
        return [str(f) for f in files]


    def _list_one(path: str, *, with_lines: bool) -> ListNodesResult:
        tree = core.load(path)[1]
        nodes = core.build_outline(core.locate_all(tree), with_lines=with_lines)
        return ListNodesResult(path=path, nodes=nodes)


    def ast_list(paths: list[str], *, with_lines: bool=True) -> ListNodesBatchResult:
        """List the hierarchical AST-node tree of one or more files.

        The tree is the foundation every other tool builds on: each node carries its
        unique, primarily name-based ``id`` and line range, but never its source –
        use ``ast_find`` (property/text filtering) or ``ast_read`` (by id) to
        retrieve source.

        A path naming a directory without subdirectories and holding at most
        5 files is transparently expanded to those files; any other directory
        fails normally.

        Args:
            paths: Absolute paths of the files to list. Must be non-empty.
            with_lines: Whether to populate each node's line range.

        Returns:
            ListNodesBatchResult: One result per listed file, one error per failed file.

        Raises:
            core.AstError: If ``paths`` is empty.
        """
        if not paths:
            raise core.AstError("'paths' must be a non-empty list.")
        results: list[ListNodesResult] = []
        errors: list[ListNodesError] = []
        for path in paths:
            for real_path in _expand_path(path):
                try:
                    results.append(_list_one(real_path, with_lines=with_lines))
                except core.AstError as exc:
                    errors.append(ListNodesError(path=real_path, error=str(exc)))
        return ListNodesBatchResult(results=results, errors=errors)


    class ListNodesTool(ToolDefinition):
        name = 'ast_list'
        title = 'List AST nodes'
        description = "Hierarchical tree of one or more files' AST nodes (import/statement segments, classes, functions, sections) with id and optional line range – no source. A directory without subdirectories and with at most 5 files is transparently expanded to those files."
        input_schema = {
            'type': 'object',
            'properties': {
                'paths': {
                    'type': 'array',
                    'minItems': 1,
                    'items': {'type': 'string'},
                    'description': 'Absolute paths of the files to list.'},
            },
            'required': ['paths'],
        }
        output_schema = {
            '$defs': {'outline_node': core.OUTLINE_NODE_SCHEMA},
            'type': 'object',
            'properties': {
                'results': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'path': {'type': 'string'},
                            'nodes': {'type': 'array', 'items': {'$ref': '#/$defs/outline_node'}},
                        },
                        'required': ['path', 'nodes'],
                    },
                },
                'errors': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {'path': {'type': 'string'}, 'error': {'type': 'string'}},
                        'required': ['path', 'error'],
                    },
                },
            },
            'required': ['results', 'errors'],
        }
        annotations = {'readOnlyHint': True, 'openWorldHint': False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`ast_list`, translating the MCP schema to/from the AST API."""
            args: dict[str, Any] = ctx.arguments
            with_lines = bool({'tools', 'edit-lines'} & ctx.session.enabled_tools)
            paths = args.get('paths') or []
            if not paths:
                return ToolResult(content=[text_content("'paths' must be a non-empty list.")], is_error=True)
            batch = ast_list(paths=paths, with_lines=with_lines)
            content = {
                'results': [{'path': r.path, 'nodes': [core.to_dict(n) for n in r.nodes]} for r in batch.results],
                'errors': [{'path': e.path, 'error': e.error} for e in batch.errors],
            }
            is_error = bool(batch.errors) and (not batch.results)
            return ToolResult(structured_content=content, is_error=is_error)


    def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
        registry.register(ListNodesTool())
        functions.register(ast_list)
```
/answer 379b-4d allow
Control Request:
```yaml
id: 379b-4e
toolName: ast_create
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  overwrite: true
  reason: ast_read Tool um Batch-Support über mehrere Dateien und IDs erweitern
  source: |
    """``ast_read`` tool: read one or more node subtrees (with source) by id, across files."""
    from dataclasses import dataclass
    from typing import Any
    from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import ToolContext
    from xy.ai.mcpc.tools.ast import core
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry
    from xy.ai.mcpc.tools.ast.list import ast_list

    __all__ = ['ReadItem', 'ReadResult', 'ReadError', 'ReadBatchResult', 'ast_read', 'ReadNodeTool', 'register']
    _ROOT_INTENT_IDS = {'root', '_module_', '__module__', 'module', ''}


    def _looks_like_root_intent(ids: list[str]) -> bool:
        """Whether ``ids`` is empty or consists solely of common root-id guesses."""
        return not ids or all((i.strip().lower() in _ROOT_INTENT_IDS for i in ids))


    @dataclass(frozen=True)
    class ReadItem:
        """One file's node ids to read.

        Attributes:
            path: Absolute path to the file to read.
            ids: Node ids to read. Must be non-empty.
        """
        path: str
        ids: list[str]


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


    @dataclass(frozen=True)
    class ReadError:
        """Error reading a whole item (e.g. bad path or empty ``ids``).

        Attributes:
            path: The path exactly as given in the input, for result association.
            error: The error message.
        """
        path: str
        error: str


    @dataclass(frozen=True)
    class ReadBatchResult:
        """Result of :func:`ast_read`.

        Attributes:
            results: One :class:`ReadResult` per successfully read file.
            errors: One :class:`ReadError` per file that failed entirely.
        """
        results: list[ReadResult]
        errors: list[ReadError]


    def _read_one(item: ReadItem, *, with_lines: bool) -> ReadResult:
        if not item.ids:
            raise core.AstError("'ids' must be a non-empty list of node ids.")
        tree = core.load(item.path)[1]
        nodes, errs = core.read_subtrees(core.locate_all(tree), item.ids, with_lines=with_lines)
        return ReadResult(path=item.path, nodes=nodes, errors=errs)


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


    class ReadNodeTool(ToolDefinition):
        name = 'ast_read'
        title = 'After using `ast_list` or `ast_find`, read AST subtrees for known node IDs'
        description = "After using `ast_list` or `ast_find`, recursively read the subtree of each ID-addressed AST node across one or more files, surfacing each node's children and source."
        input_schema = {
            'type': 'object',
            'properties': {
                'items': {
                    'type': 'array',
                    'minItems': 1,
                    'items': {
                        'type': 'object',
                        'properties': {
                            'path': {'type': 'string', 'description': 'Absolute path to the file.'},
                            'ids': {
                                'type': 'array',
                                'items': {'type': 'string'},
                                'description': 'List of AST node ids to read.'},
                        },
                        'required': ['path', 'ids'],
                    },
                    'description': 'Per-file node ids to read.',
                },
            },
            'required': ['items'],
        }
        output_schema = {
            '$defs': {'outline_node': core.OUTLINE_NODE_SCHEMA},
            'type': 'object',
            'properties': {
                'results': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'path': {'type': 'string'},
                            'nodes': {'type': 'array', 'items': {'$ref': '#/$defs/outline_node'}},
                            'errors': {'type': 'array', 'items': {'type': 'string'}},
                        },
                        'required': ['path', 'nodes'],
                    },
                },
                'errors': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {'path': {'type': 'string'}, 'error': {'type': 'string'}},
                        'required': ['path', 'error'],
                    },
                },
            },
            'required': ['results', 'errors'],
        }
        annotations = {'readOnlyHint': True, 'openWorldHint': False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`ast_read`, translating the MCP schema to/from the AST API.

            Per item, falls back to :func:`ast_list` when ``ids`` looks like agents
            habitually mis-guessing a root id (``root``/``_module_``/``module``/empty)
            and none of them resolve to a node: returns that file's full outline
            instead of an error, and notes the redirect in the item's ``errors``.
            """
            args: dict[str, Any] = ctx.arguments
            with_lines = bool({'tools', 'edit-lines'} & ctx.session.enabled_tools)
            raw_items = args.get('items') or []
            if not raw_items:
                return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
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
                    list_batch = ast_list([path], with_lines=with_lines)
                    for lr in list_batch.results:
                        results.append({
                            'path': lr.path,
                            'nodes': [core.to_dict(n) for n in lr.nodes],
                            'errors': [f'ids {ids!r} resolved to no node; redirected to ast_list, returning the full outline instead.']})
                    for le in list_batch.errors:
                        errors.append({'path': le.path, 'error': le.error})
                    continue
                results.append({'path': item_result.path, 'nodes': [core.to_dict(n) for n in item_result.nodes], 'errors': item_result.errors})
            is_error = bool(errors) and (not results)
            return ToolResult(structured_content={'results': results, 'errors': errors}, is_error=is_error)


    def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
        registry.register(ReadNodeTool())
        functions.register(ast_read)
```
/answer 379b-4e allow
Control Request:
```yaml
id: 379b-4f
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/list.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
  reason: Alle acht umgebauten Tool-Dateien auf Syntaxfehler prüfen
```
/answer 379b-4f allow
ReasoningToken: 763
ReasoningToken: 136
ReasoningToken: 3707
Text:
 Nun validiere ich alle acht Dateien.
Result Stats:  id=122e5683-70f2-4025-b666-f4d8b85918d4, total: 0, in: 157, out: 56839, reason: 0, read: 643326, write: 98651
Alle acht Dateien wurden auf Batch-Support umgestellt und validieren fehlerfrei.
