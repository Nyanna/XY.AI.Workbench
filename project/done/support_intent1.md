Verbessere den Intent Support für `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py`.
Agenten rufen das Tool häufig mit root, _module_, module, (leer) als ID auf. Änderungen der Toolbeschreibungen brachten keinen Erfolg denn im Endeffekt bestimmt die erste Entscheidung für das Tool die folgende Inferenz. Daher estimieren wir den Intent und unterstützen ihn.

- Besteht die ID Liste lediglich aus den genannten Keywords und es gibt keinen passenden Knoten oder die Liste ist leer, soll auf `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/list.py` zurückgefallen werden. Das heißt, es wird kein Code ausgegeben, sondern Allein die Kontenübersicht. Dies soll nicht im MCP Outputschema abgebildet werden da es ein Spezialfall ist. "errors" enthält stattdessen den Hinweis das die Anfrage umgeleitet und mit `ast_list` beantwortet wurde.
SystemInit:  id=fd5d4d87-4c36-4c29-9d68-0b9613fdeefd, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: ec17-1
toolName: ast_read
arguments:
  ids:
  - __module__
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  reason: Read read.py to add intent fallback
```
/answer ec17-1 allow
Control Request:
```yaml
id: ec17-2
result:
  structuredContent:
    nodes: []
    errors:
    - No node matched '__module__'.
```
/answer ec17-2 allow
Control Request:
```yaml
id: ec17-3
toolName: ast_read
arguments:
  ids:
  - __module__
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/list.py
  reason: Read list.py to understand fallback function to call
```
/answer ec17-3 allow
Control Request:
```yaml
id: ec17-4
result:
  structuredContent:
    nodes: []
    errors:
    - No node matched '__module__'.
```
/answer ec17-4 allow
ReasoningToken: 15
Control Request:
```yaml
id: ec17-5
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  reason: Overview of read.py
```
/answer ec17-5 allow
Control Request:
```yaml
id: ec17-6
result:
  structuredContent:
    nodes:
    - id: YEXBkx
      type: statements
    - id: ReKSW3
      type: imports
    - id: Exk4XO
      type: statements
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
        docstring: "Delegate to :func:`ast_read`, translating the MCP schema to/from the AST API."
    - id: register
      type: FunctionDef
      signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer ec17-6 allow
Control Request:
```yaml
id: ec17-7
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/list.py
  reason: Overview of list.py
```
/answer ec17-7 allow
Control Request:
```yaml
id: ec17-8
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
```
/answer ec17-8 allow
Control Request:
```yaml
id: ec17-9
toolName: ast_read
arguments:
  ids:
  - ast_read
  - ReadNodeTool
  - ReadNodeResult
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  reason: Read full source of ast_read function and ReadNodeTool class
```
/answer ec17-9 allow
Control Request:
```yaml
id: ec17-a
result:
  structuredContent:
    nodes:
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
              """Delegate to :func:`ast_read`, translating the MCP schema to/from the AST API."""
              args: dict[str, Any] = ctx.arguments
              with_lines = bool({'tools', 'edit-lines'} & ctx.session.enabled_tools)
              try:
                  result = ast_read(ids=args.get('ids') or [], path=args.get('path'), with_lines=with_lines)
              except core.AstError as exc:
                  return ToolResult(content=[text_content(str(exc))], is_error=True)
              structured_content: dict[str, Any] = {'nodes': [core.to_dict(n) for n in result.nodes]}
              if result.errors:
                  structured_content['errors'] = result.errors
              return ToolResult(structured_content=structured_content)
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
```
/answer ec17-a allow
Control Request:
```yaml
id: ec17-b
toolName: ast_read
arguments:
  ids:
  - ast_list
  - ListNodesTool
  - ListNodesResult
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/list.py
  reason: Read full source of ast_list function and ListNodesTool class for fallback integration
```
/answer ec17-b allow
Control Request:
```yaml
id: ec17-c
result:
  structuredContent:
    nodes:
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
```
/answer ec17-c allow
Control Request:
```yaml
id: ec17-d
toolName: ast_read
arguments:
  ids:
  - ReKSW3
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  reason: Check imports in read.py to add ast_list import
```
/answer ec17-d allow
Control Request:
```yaml
id: ec17-e
result:
  structuredContent:
    nodes:
    - id: ReKSW3
      type: imports
      code: |-
        from dataclasses import dataclass
        from typing import Any
        from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
        from xy.ai.mcpc.tools.tool_context import ToolContext
        from xy.ai.mcpc.tools.ast import core
        from xy.ai.mcpc.tools.function_registry import FunctionRegistry
```
/answer ec17-e allow
ReasoningToken: 2281
Text:
 Jetzt füge ich Import und Fallback-Logik hinzu.
Control Request:
```yaml
id: ec17-f
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  id: ReKSW3
  old_text: from xy.ai.mcpc.tools.function_registry import FunctionRegistry
  new_text: |-
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry
    from xy.ai.mcpc.tools.ast.list import ast_list
  reason: Import ast_list for the intent fallback
```
/answer ec17-f allow
Control Request:
```yaml
id: ec17-10
toolName: grep
arguments:
  directory:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
  pattern: import read|from .* read
  include:
  - list.py
  reason: Check for circular import between list.py and read.py
```
/answer ec17-10 allow
Control Request:
```yaml
id: ec17-11
result:
  structuredContent:
    matches: []
```
/answer ec17-11 allow
ReasoningToken: 32
Text:
 Kein zirkulärer Import. Jetzt die Fallback-Logik einbauen.
Control Request:
```yaml
id: ec17-12
toolName: ast_replace
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  id: ReadNodeTool.handle
  source: |-
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
            return ToolResult(structured_content={
                'nodes': [core.to_dict(n) for n in list_result.nodes],
                'errors': [f'ids {ids!r} resolved to no node; redirected to ast_list, returning the full outline instead.']})
        structured_content: dict[str, Any] = {'nodes': [core.to_dict(n) for n in result.nodes]}
        if result.errors:
            structured_content['errors'] = result.errors
        return ToolResult(structured_content=structured_content)
  reason: Add intent-based fallback to ast_list when ids are root-like keywords with no matches
```
/answer ec17-12 allow
Control Request:
```yaml
id: ec17-13
toolName: ast_insert
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  id: ReadNodeResult
  position: before
  source: |+
    _ROOT_INTENT_IDS = {'root', '_module_', 'module', ''}


    def _looks_like_root_intent(ids: list[str]) -> bool:
        """Whether ``ids`` is empty or consists solely of common root-id guesses."""
        return not ids or all(i.strip().lower() in _ROOT_INTENT_IDS for i in ids)


  reason: Add helper to detect root-intent id lists
```
/answer ec17-13 allow füge auch "__module__" hinzu - deinen eigenen Fehler
Control Request:
```yaml
id: ec17-14
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  id: FpQjhL
  old_text: "_ROOT_INTENT_IDS = {'root', '_module_', 'module', ''}"
  new_text: "_ROOT_INTENT_IDS = {'root', '_module_', '__module__', 'module', ''}"
  reason: Also recognize __module__ as a root-intent guess
```
/answer ec17-14 allow
Control Request:
```yaml
id: ec17-15
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  reason: Validate syntax of modified file
```
/answer ec17-15 allow
Text:
 Jetzt die Helper-Funktion `_looks_like_root_intent` vor `ReadNodeResult` einfügen.
ReasoningToken: 35
Result Stats:  id=fd5d4d87-4c36-4c29-9d68-0b9613fdeefd, total: 0, in: 153, out: 5772, reason: 0, read: 112829, write: 15547
Fallback implementiert: `ast_read` erkennt reine Root-Intent-ID-Listen (`root`, `_module_`, `__module__`, `module`, leer) ohne Treffer und liefert stattdessen die Knotenübersicht von `ast_list`, mit Hinweis auf die Umleitung in `errors`.
Ich habe den MCP neu gestartet, teste deine Änderung
SystemInit:  id=fd5d4d87-4c36-4c29-9d68-0b9613fdeefd, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 379b-1
toolName: ast_read
arguments:
  ids:
  - root
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  reason: Test intent fallback with keyword id 'root'
```
/answer 379b-1 allow
Control Request:
```yaml
id: 379b-2
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
    - "ids ['root'] resolved to no node; redirected to ast_list, returning the full outline instead."
```
/answer 379b-2 allow
Control Request:
```yaml
id: 379b-3
toolName: ast_read
arguments:
  ids:
  - __module__
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  reason: Test intent fallback with keyword id '__module__'
```
/answer 379b-3 allow
Control Request:
```yaml
id: 379b-4
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
/answer 379b-4 allow
Control Request:
```yaml
id: 379b-5
toolName: ast_read
arguments:
  ids: []
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  reason: Test intent fallback with empty ids list
```
/answer 379b-5 allow
Control Request:
```yaml
id: 379b-6
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
    - "ids [] resolved to no node; redirected to ast_list, returning the full outline instead."
```
/answer 379b-6 allow
ReasoningToken: 301
Control Request:
```yaml
id: 379b-7
toolName: ast_read
arguments:
  ids:
  - ast_read
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/read.py
  reason: Verify normal read path still works unaffected
```
/answer 379b-7 allow
Text:
 Alle drei Fälle (`root`, `__module__`, leere Liste) fallen korrekt auf die `ast_list`-Übersicht zurück, mit Hinweis in `errors`. Test mit gültiger ID zur Kontrolle:
Control Request:
```yaml
id: 379b-8
result:
  structuredContent:
    nodes:
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
```
/answer 379b-8 allow
Result Stats:  id=fd5d4d87-4c36-4c29-9d68-0b9613fdeefd, total: 0, in: 159, out: 6865, reason: 0, read: 162635, write: 19436
Normaler Pfad mit gültiger ID liefert weiterhin echten Quellcode (`code`), kein Fallback. Funktioniert wie erwartet.
