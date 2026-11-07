Wir erhöhen die Toleranz beider Tools. Wie im Beispiel ersichtlich hat der Agent das Tool ohne ID aufgerufen. Das Ziel kann potenziell antizipiert werden. Wenn künftig die ID fehlt, findet zuerst ein Suchlauf statt, der Versucht die fehlende ID anhand der Platzhalter zu ermitteln. Treffen die Kriterien nur auf einen Knoten zu, wird dessen ID übernommen. Das ist logisch für verschachtelte Knoten nicht möglich, da Kind und Parent gleichermaßen die Kriterien erfüllen. Bei multiplen Kandidaten wird der Fehler um die Kandidatenliste ergänzt -> "Multiple target nodes found" (Kandidatenliste typisiert zurückgeben).

- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py`
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py`

Beispiel:
```yaml
id: 1c3b-7d
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/OpenAIModelResolver.java
  old_text: "\t\t\t\tString id = entry.path(\"id\").asText(null);\n\t\t\t\tif (id == null || !entry.path(\"shutdown_date\").isNull() && !entry.path(\"shutdown_date\").isMissingNode())\n\t\t\t\t\tcontinue;\n\t\t\t\tif (!ModelFilter.isMainstream(id))"
  new_text: "\t\t\t\tString id = entry.path(\"id\").asText(null);\n\t\t\t\tJsonNode shutdown = entry.path(\"shutdown_date\");\n\t\t\t\tboolean hasShutdownDate = !shutdown.isMissingNode() && !shutdown.isNull();\n\t\t\t\tif (id == null || hasShutdownDate)\n\t\t\t\t\tcontinue;\n\t\t\t\tif (!ModelFilter.isMainstream(id))"
  reason: Lesbarkeit der shutdown_date-Prüfung verbessern
  
id: 1c3b-7e
result:
  content:
  - type: text
    text: A node selector (id) is required.
  isError: true
```
SystemInit:  id=af0ee96d-700e-4985-a8a0-bea7c89b62ca, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 1c3b-dc
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  ids:
  - root
  reason: "Aktuellen Code von edit_block.py lesen, um ID-Pflicht-Logik zu finden"
```
/answer 1c3b-dc allow
Control Request:
```yaml
id: 1c3b-dd
result:
  structuredContent:
    nodes: []
    errors:
    - No node matched 'root'.
```
/answer 1c3b-dd allow
Control Request:
```yaml
id: 1c3b-de
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  ids:
  - root
  reason: "Aktuellen Code von edit_marks.py lesen, um ID-Pflicht-Logik zu finden"
```
/answer 1c3b-de allow
Control Request:
```yaml
id: 1c3b-df
result:
  structuredContent:
    nodes: []
    errors:
    - No node matched 'root'.
```
/answer 1c3b-df allow
ReasoningToken: 27
Control Request:
```yaml
id: 1c3b-e0
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  reason: Struktur der Datei ermitteln
```
/answer 1c3b-e0 allow
Control Request:
```yaml
id: 1c3b-e1
result:
  structuredContent:
    nodes:
    - id: q0CvRQ
      type: statements
    - id: fjdzVS
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
      - id: EditBlockNodeTool.oC5twa
        type: statements
      - id: EditBlockNodeTool.handle
        type: FunctionDef
        signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
        docstring: "Delegate to :func:`ast_edit_block`, translating the MCP schema to/from the AST …"
    - id: register
      type: FunctionDef
      signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 1c3b-e1 allow
Control Request:
```yaml
id: 1c3b-e2
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  reason: Struktur der Datei ermitteln
```
/answer 1c3b-e2 allow
Control Request:
```yaml
id: 1c3b-e3
result:
  structuredContent:
    nodes:
    - id: zkTaU7
      type: statements
    - id: hRC4sE
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
      - id: EditMarksNodeTool.oC5twa
        type: statements
      - id: EditMarksNodeTool.handle
        type: FunctionDef
        signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
        docstring: "Delegate to :func:`ast_edit_marks`, translating the MCP schema to/from the AST …"
    - id: register
      type: FunctionDef
      signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 1c3b-e3 allow
Control Request:
```yaml
id: 1c3b-e4
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  ids:
  - q0CvRQ
  - fjdzVS
  - pbRXsQ
  - EditBlockNodeResult
  - node_guard
  - ast_edit_block
  - EditBlockNodeTool
  - register
  reason: Vollständigen Code für Anpassung der ID-Pflicht-Logik lesen
```
/answer 1c3b-e4 allow
Control Request:
```yaml
id: 1c3b-e5
result:
  structuredContent:
    nodes:
    - id: q0CvRQ
      type: statements
      code: "'``ast_edit_block`` tool: exact-block (old_text -> new_text) edit within a selected node.'"
    - id: fjdzVS
      type: imports
      code: |-
        from dataclasses import dataclass
        from typing import Any
        from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
        from xy.ai.mcpc.tools.tool_context import ToolContext
        from xy.ai.mcpc.tools.ast import core
        from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_path
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
                id: Unique id of the target node.

            Returns:
                EditBlockNodeResult: Success status.

            Raises:
                core.AstError: If ``path`` is invalid, ``id`` is not
                    given, the path matches zero or more than one node, ``old_text`` is not
                    found or (without ``replace_all``) ambiguous within the node's source, or
                    the edited source has a syntax error.
            """
            file_path = core.require_path(path)
            tree = core.CACHE.get_tree(file_path)
            target = select_by_path(tree, id=id)
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
      - id: EditBlockNodeTool.oC5twa
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
/answer 1c3b-e5 allow
Control Request:
```yaml
id: 1c3b-e6
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  ids:
  - zkTaU7
  - hRC4sE
  - OlQPSt
  - EditMarksNodeResult
  - node_marks_guard
  - ast_edit_marks
  - EditMarksNodeTool
  - register
  reason: Vollständigen Code für Anpassung der ID-Pflicht-Logik lesen
```
/answer 1c3b-e6 allow
Control Request:
```yaml
id: 1c3b-e7
result:
  structuredContent:
    nodes:
    - id: zkTaU7
      type: statements
      code: "'``ast_edit_marks`` tool: mark-based edit within the source of a node addressed by id.'"
    - id: hRC4sE
      type: imports
      code: |-
        from dataclasses import dataclass
        from typing import Any
        from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
        from xy.ai.mcpc.tools.tool_context import ToolContext
        from xy.ai.mcpc.tools.ast import core
        from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_path
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
                id: Unique id of the target node.

            Returns:
                EditMarksNodeResult: Success status.

            Raises:
                core.AstError: If ``path`` is invalid, ``id`` is not
                    given, the path matches zero or more than one node, the markers are not
                    found or ambiguous within the node's source, or the edited source has a
                    syntax error.
            """
            file_path = core.require_path(path)
            tree = core.CACHE.get_tree(file_path)
            target = select_by_path(tree, id=id)
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
      - id: EditMarksNodeTool.oC5twa
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
/answer 1c3b-e7 allow
Control Request:
```yaml
id: 1c3b-e8
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
  reason: "select_by_path Implementierung finden, um Fallback-Suche für fehlende ID zu ergänzen"
```
/answer 1c3b-e8 allow
Control Request:
```yaml
id: 1c3b-e9
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
      nodes:
      - id: 1c3Fpi
        type: statements
        code: "'Selector machinery shared by the ``ast_*`` tools.\\n\\n``ast_find`` is the only tool that restricts on diverse node properties, so it\\nuses the full :data:`SELECTOR_PROPS`. Every mutation tool addresses a node purely\\nby its unique ``id`` and uses the reduced :data:`PATH_SELECTOR_PROPS`.\\n'"
      - id: 6eecMb
        type: imports
        code: |-
          from typing import Any
          from xy.ai.mcpc.tools.ast import core
      - id: LN88Al
        type: statements
        code: |-
          __all__ = ['SELECTOR_PROPS', 'PATH_SELECTOR_PROPS', 'select_one', 'select_by_path', 'list_output_schema']
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
      - id: list_output_schema
        type: FunctionDef
        code: |-
          def list_output_schema() -> dict[str, Any]:
              return {'$defs': {'outline_node': core.OUTLINE_NODE_SCHEMA}, 'type': 'object', 'properties': {
                  'nodes': {'type': 'array', 'items': {'$ref': '#/$defs/outline_node'}}}, 'required': ['nodes']}
```
/answer 1c3b-e9 allow
ReasoningToken: 44
Control Request:
```yaml
id: 1c3b-ea
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  reason: core.find und core.Located Implementierung für Fallback-Suche verstehen
  name: find
```
/answer 1c3b-ea allow
Control Request:
```yaml
id: 1c3b-eb
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      nodes: []
```
/answer 1c3b-eb allow
Control Request:
```yaml
id: 1c3b-ec
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  reason: Located-Klasse und AstError verstehen
  name: Located
```
/answer 1c3b-ec allow
Control Request:
```yaml
id: 1c3b-ed
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      nodes: []
```
/answer 1c3b-ed allow
ReasoningToken: 484
Control Request:
```yaml
id: 1c3b-ee
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  reason: Struktur von core.py ermitteln für find/Located/AstError
```
/answer 1c3b-ee allow
Control Request:
```yaml
id: 1c3b-ef
result:
  structuredContent:
    nodes:
    - id: ue626f
      type: statements
    - id: ZlvshC
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
/answer 1c3b-ef allow
Control Request:
```yaml
id: 1c3b-f0
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  ids:
  - ZlvshC
  - locate_all
  - edit_node_source
  reason: Imports und Located/find Herkunft klären sowie edit_node_source für Textzugriff
```
/answer 1c3b-f0 allow
Control Request:
```yaml
id: 1c3b-f1
result:
  structuredContent:
    nodes:
    - id: ZlvshC
      type: imports
      code: |-
        from __future__ import annotations
        import hashlib
        import threading
        from dataclasses import dataclass
        from pathlib import Path
        from xy.ai.mcpc.tools.ast.base import AstError, Engine, Located, OutlineNode, OUTLINE_NODE_SCHEMA, Tree, build_outline, find, line_range, matches, most_specific, node_outline, read_subtrees, require_path, to_dict
        from xy.ai.mcpc.tools.ast import generic
        from xy.ai.mcpc.tools.ast import python
    - id: locate_all
      type: FunctionDef
      code: |-
        def locate_all(tree: Tree) -> list[Located]:
            return tree.engine.locate_all(tree)
    - id: edit_node_source
      type: FunctionDef
      code: |-
        def edit_node_source(loc: Located) -> str:
            return loc.tree.engine.node_code(loc.node)
```
/answer 1c3b-f1 allow
ReasoningToken: 73
Control Request:
```yaml
id: 1c3b-f2
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  reason: find/matches/most_specific/Located Implementierung verstehen für Fallback-Suche
  name: find
```
/answer 1c3b-f2 allow
Control Request:
```yaml
id: 1c3b-f3
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      nodes:
      - id: find
        type: FunctionDef
        code: |-
          def find(tree: Tree, **filters: object) -> list[Located]:
              active = {k: v for k, v in filters.items() if v is not None}
              '# type: ignore[arg-type]'
              return [loc for loc in tree.engine.locate_all(tree) if matches(loc, **active)]
```
/answer 1c3b-f3 allow
Control Request:
```yaml
id: 1c3b-f4
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  reason: most_specific Implementierung verstehen
  name: most_specific
```
/answer 1c3b-f4 allow
Control Request:
```yaml
id: 1c3b-f5
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      nodes:
      - id: most_specific
        type: FunctionDef
        code: |-
          def most_specific(located: list[Located], lineno: int, end_lineno: int) -> Located | None:
              """Return the smallest node in *located* fully containing lines [lineno, end_lineno]."""
              best = None
              for loc in located:
                  if loc.lineno <= lineno and loc.end_lineno >= end_lineno:
                      if best is None or loc.end_lineno - loc.lineno < best.end_lineno - best.lineno:
                          best = loc
              return best
```
/answer 1c3b-f5 allow
Control Request:
```yaml
id: 1c3b-f6
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  reason: Located Klasse verstehen
  name: Located
```
/answer 1c3b-f6 allow
Control Request:
```yaml
id: 1c3b-f7
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      nodes:
      - id: Located
        type: ClassDef
        code: |-
          @dataclass
          class Located:
              """A node with the engine-independent metadata the selectors match on.

              Attributes:
                  tree: The owning tree (for engine dispatch).
                  node: Engine-native node object.
                  parent: Engine-native container node.
                  index: Position of ``node`` among its parent's addressable children.
                  node_id: The node's unique ``id`` — its fully-qualified path from the
                      root (e.g. ``"MyClass.method"``), in name/hash form or, for nameless
                      nodes/segments, a stable content-hash fallback. There is no separate FQN.
                  node_type: Engine-reported node type name.
                  name: Simple name, if the node carries one.
                  lineno / end_lineno: 1-based inclusive line span.
                  parent_type: Type name of ``parent``, or ``None`` at the top level.
                  expandable: Whether ``read`` should descend into children instead of
                      returning the node's full source (a pure container of nested defs).
              """
              tree: Tree
              node: Any
              parent: Any
              index: int
              node_id: str
              node_type: str
              name: str | None
              lineno: int
              end_lineno: int
              parent_type: str | None
              expandable: bool = False
```
/answer 1c3b-f7 allow
ReasoningToken: 61
Control Request:
```yaml
id: 1c3b-f8
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_text_match.py
  reason: Bestehende Textabgleich-Utilities für tolerantes Substring-Matching prüfen
```
/answer 1c3b-f8 allow
Control Request:
```yaml
id: 1c3b-f9
result:
  structuredContent:
    nodes:
    - id: 8jG3B1
      type: statements
    - id: Oz1Db8
      type: imports
    - id: gIB8SU
      type: statements
    - id: MatchResult
      type: ClassDef
      signature: "@dataclass(frozen=True) class MatchResult:"
    - id: TextMatchError
      type: ClassDef
      signature: "class TextMatchError(Exception):"
      docstring: Base error for the shared text-block matcher.
    - id: TextNotFound
      type: ClassDef
      signature: "class TextNotFound(TextMatchError):"
    - id: TextAmbiguous
      type: ClassDef
      signature: "class TextAmbiguous(TextMatchError):"
      children:
      - id: TextAmbiguous.init
        type: FunctionDef
        signature: "def __init__(self, message: str, count: int) -> None:"
    - id: phHr3b
      type: statements
    - id: delim_runs
      type: FunctionDef
      signature: "def _delim_runs(part: str) -> list[tuple[str, bool]]:"
    - id: token
      type: FunctionDef
      signature: "def _token(part: str, level: int) -> str:"
    - id: pattern
      type: FunctionDef
      signature: "def _pattern(needle: str, level: int) -> re.Pattern[str]:"
    - id: matches
      type: FunctionDef
      signature: "def _matches(haystack: str, needle: str, level: int) -> list[MatchResult]:"
    - id: levels
      type: FunctionDef
      signature: "def _levels(exact: bool, max_level: int) -> tuple[int, ...]:"
    - id: find_all
      type: FunctionDef
      signature: "def find_all(haystack: str, needle: str, *, exact: bool) -> list[MatchResult]:"
      docstring: Return all non-overlapping occurrences of ``needle`` (exact or level-1).
    - id: find
      type: FunctionDef
      signature: "def find(haystack: str, needle: str, *, exact: bool) -> MatchResult:"
    - id: line_preserving
      type: FunctionDef
      signature: "def line_preserving(reference: str) -> ReplaceGuard:"
      docstring: "Guard: the matched span keeps ``reference``'s newline count (no merged lines)."
    - id: marks_line_preserving
      type: FunctionDef
      signature: "def marks_line_preserving(begin_marker: str, end_marker: str) -> MarksGuard:"
    - id: mirror_escaping
      type: FunctionDef
      signature: "def _mirror_escaping(span: str, replacement: str) -> str:"
      docstring: Encode ``replacement``'s raw newlines/tabs like the replaced ``span``. When the…
    - id: replace_in_block
      type: FunctionDef
      signature: "def replace_in_block(block: str, old_text: str, new_text: str, *, exact: bool, replace_all: bool=False, accept: ReplaceGuard | None=None, max_level: int=3, where: str='block') -> str:"
      docstring: "Replace ``old_text`` with ``new_text`` in ``block``, escalating tolerance. The …"
    - id: replace_between
      type: FunctionDef
      signature: "def replace_between(block: str, begin_marker: str, end_marker: str, content: str, *, exact: bool, accept: MarksGuard | None=None, max_level: int=3, where: str='block') -> str:"
      docstring: Replace the span between (and including) both markers with ``content``. Both ma…
```
/answer 1c3b-f9 allow
ReasoningToken: 275
Control Request:
```yaml
id: 1c3b-fa
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  reason: AstError-Klasse und core.find Signaturen für Fehlerformat prüfen
  name: AstError
```
/answer 1c3b-fa allow
Control Request:
```yaml
id: 1c3b-fb
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
/answer 1c3b-fb allow
Control Request:
```yaml
id: 1c3b-fc
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  ids:
  - matches
  reason: matches()-Funktion für Selektor-Logik verstehen
```
/answer 1c3b-fc allow
Control Request:
```yaml
id: 1c3b-fd
result:
  structuredContent:
    nodes:
    - id: matches
      type: FunctionDef
      code: |-
        def matches(loc: Located, *, id: str | None=None, node_type: str | None=None, name: str | None=None, parent_type: str | None=None) -> bool:
            if id is not None and loc.node_id != id:
                return False
            if node_type is not None and loc.node_type.lower() != node_type.lower():
                return False
            if name is not None and loc.name != name:
                return False
            if parent_type is not None and (loc.parent_type or '').lower() != parent_type.lower():
                return False
            return True
```
/answer 1c3b-fd allow
ReasoningToken: 793
Control Request:
```yaml
id: 1c3b-fe
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
  reason: Alle Verwendungen von select_by_path finden
  text: select_by_path
```
/answer 1c3b-fe allow
Control Request:
```yaml
id: 1c3b-ff
result:
  content:
  - type: text
    text: Not a regular file.
  isError: true
```
/answer 1c3b-ff allow
ReasoningToken: 612
Control Request:
```yaml
id: 1c3b-100
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
  reason: Prüfen ob weitere Tools select_by_path nutzen
  text: select_by_path
```
/answer 1c3b-100 allow
Control Request:
```yaml
id: 1c3b-101
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
      nodes:
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
      - id: ast_replace.Hdowcl
        type: statements
        code: |-
          file_path = core.require_path(path)
          tree = core.CACHE.get_tree(file_path)
          target = select_by_path(tree, id=id)
          new_id = core.replace_node(target, source)
          core.CACHE.save(file_path, tree)
          return ReplaceNodeResult(result='success', id=new_id)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      nodes:
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
      - id: ast_delete.QnWvVH
        type: statements
        code: |-
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
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
      nodes:
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
      - id: ast_insert.6AyhPt
        type: statements
        code: |-
          file_path = core.require_path(path)
          tree = core.CACHE.get_tree(file_path)
          target = select_by_path(tree, id=id)
          before_ids = {loc.node_id for loc in core.locate_all(tree)}
          inserted = core.insert_node(target, source, position)
          new_ids = [loc.node_id for loc in core.locate_all(tree) if loc.node_id not in before_ids]
          core.CACHE.save(file_path, tree)
          return InsertNodeResult(result='success', inserted=inserted, ids=new_ids or None)
```
/answer 1c3b-101 allow
Control Request:
```yaml
id: 1c3b-102
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
  reason: ToolResult-Definition prüfen für Fehler mit structured_content
  name: ToolResult
```
/answer 1c3b-102 allow
Control Request:
```yaml
id: 1c3b-103
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
      nodes:
      - id: ToolResult
        type: ClassDef
        code: |-
          @dataclass(slots=True)
          class ToolResult:
              """The result of a tool call (maps onto MCP ``CallToolResult``)."""
              content: list[dict[str, Any]] = field(default_factory=list)
              structured_content: dict[str, Any] | None = None
              is_error: bool = False
              auto_approve: bool = False
              control_hint: str | None = None
              'Optional hint attached by the controller on approval (``/allow <id> <hint>``).\n\n    Embedded as :data:`CONTROL_HINT_PROPERTY` *inside* ``structuredContent``\n    (see :meth:`to_dict`) rather than as a top-level ``CallToolResult`` field:\n    MCP clients only surface ``content``/``structuredContent``/``isError`` to\n    the model, so a sibling top-level key would silently be dropped before\n    ever reaching the agent.\n    '

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
```
/answer 1c3b-103 allow
ReasoningToken: 1736
Control Request:
```yaml
id: 1c3b-104
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  ids:
  - RBFWwR
  reason: __all__ Liste in core.py prüfen
```
/answer 1c3b-104 allow
Control Request:
```yaml
id: 1c3b-105
result:
  structuredContent:
    nodes:
    - id: RBFWwR
      type: statements
      code: "_PYTHON_EXTENSIONS = ('.py', '.pyi')"
```
/answer 1c3b-105 allow
Control Request:
```yaml
id: 1c3b-106
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  reason: __all__ Liste in base.py prüfen
  regexp: __all__
```
/answer 1c3b-106 allow
Control Request:
```yaml
id: 1c3b-107
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      nodes: []
```
/answer 1c3b-107 allow
ReasoningToken: 177
Text:
 No `__all__` restrictions. Now implement changes.
Control Request:
```yaml
id: 1c3b-108
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  old_text: |-
    class AstError(Exception):
        """A user-facing, path-free error raised by the AST tools."""
  new_text: |-
    class AstError(Exception):
        """A user-facing, path-free error raised by the AST tools."""


    class AstAmbiguous(AstError):
        """Raised when a text-based node search (no id given) finds several
        unrelated candidates instead of a single node."""

        def __init__(self, message: str, candidates: list[str]) -> None:
            super().__init__(message)
            self.candidates = candidates
  reason: AstAmbiguous-Ausnahme mit typisierter Kandidatenliste ergänzen
```
/answer 1c3b-108 allow
Control Request:
```yaml
id: 1c3b-109
result:
  content:
  - type: text
    text: A node selector (id) is required.
  isError: true
```
/answer 1c3b-109 allow
Control Request:
```yaml
id: 1c3b-10a
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  reason: Node-ID für AstError-Klasse ermitteln
  name: AstError
```
/answer 1c3b-10a allow
Control Request:
```yaml
id: 1c3b-10b
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
/answer 1c3b-10b allow
ReasoningToken: 52
Control Request:
```yaml
id: 1c3b-10c
toolName: ast_insert
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  id: AstError
  position: after
  source: |2-


    class AstAmbiguous(AstError):
        """Raised when a text-based node search (no id given) finds several
        unrelated candidates instead of a single node."""

        def __init__(self, message: str, candidates: list[str]) -> None:
            super().__init__(message)
            self.candidates = candidates
  reason: AstAmbiguous-Ausnahme mit typisierter Kandidatenliste ergänzen
```
/answer 1c3b-10c allow
Text:
 Now update core.py imports to expose `AstAmbiguous` and `locate_all`/`edit_node_source` already present.
Control Request:
```yaml
id: 1c3b-10d
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  old_text: "from xy.ai.mcpc.tools.ast.base import AstError, Engine, Located, OutlineNode, OUTLINE_NODE_SCHEMA, Tree, build_outline, find, line_range, matches, most_specific, node_outline, read_subtrees, require_path, to_dict"
  new_text: "from xy.ai.mcpc.tools.ast.base import AstAmbiguous, AstError, Engine, Located, OutlineNode, OUTLINE_NODE_SCHEMA, Tree, build_outline, find, line_range, matches, most_specific, node_outline, read_subtrees, require_path, to_dict"
  reason: AstAmbiguous über core-Modul verfügbar machen
```
/answer 1c3b-10d allow
Control Request:
```yaml
id: 1c3b-10e
result:
  content:
  - type: text
    text: A node selector (id) is required.
  isError: true
```
/answer 1c3b-10e allow
Control Request:
```yaml
id: 1c3b-10f
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  id: ZlvshC
  old_text: "from xy.ai.mcpc.tools.ast.base import AstError, Engine, Located"
  new_text: "from xy.ai.mcpc.tools.ast.base import AstAmbiguous, AstError, Engine, Located"
  reason: AstAmbiguous über core-Modul verfügbar machen
```
/answer 1c3b-10f allow
ReasoningToken: 49
Text:
 Now add the search helper in common.py.
Control Request:
```yaml
id: 1c3b-110
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
  id: select_by_path
  old_text: |2-
        if id is None:
            raise core.AstError('A node selector (id) is required.')
        return select_one(tree, id=id)
  new_text: |2-
        if id is None:
            raise core.AstError('A node selector (id) is required.')
        return select_one(tree, id=id)


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
        candidates = [
            loc for loc in core.locate_all(tree)
            if all(text in core.edit_node_source(loc) for text in texts)]
        if not candidates:
            raise core.AstError('No node matched the given text; a node selector (id) is required.')
        candidates = [
            c for c in candidates
            if not any(
                other is not c and c.lineno <= other.lineno and c.end_lineno >= other.end_lineno
                and (c.lineno, c.end_lineno) != (other.lineno, other.end_lineno)
                for other in candidates)]
        if len(candidates) > 1:
            raise core.AstAmbiguous(
                'Multiple target nodes found; specify a node selector (id).',
                [c.node_id for c in candidates])
        return candidates[0]
  reason: Fallback-Textsuche für fehlende ID mit Reduktion verschachtelter Kandidaten und typisierter Ambiguitätsliste ergänzen
```
/answer 1c3b-110 allow
Control Request:
```yaml
id: 1c3b-111
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
  id: LN88Al
  old_text: "__all__ = ['SELECTOR_PROPS', 'PATH_SELECTOR_PROPS', 'select_one', 'select_by_path', 'list_output_schema']"
  new_text: "__all__ = ['SELECTOR_PROPS', 'PATH_SELECTOR_PROPS', 'select_one', 'select_by_path', 'select_by_text', 'list_output_schema']"
  reason: select_by_text in __all__ aufnehmen
```
/answer 1c3b-111 allow
Control Request:
```yaml
id: 1c3b-112
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  id: fjdzVS
  old_text: "from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_path"
  new_text: "from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_text"
  reason: select_by_text statt strikter select_by_path importieren
```
/answer 1c3b-112 allow
Text:
 Now update `__all__` in common.py, and both edit_block.py / edit_marks.py to use `select_by_text`, plus handle() catching `AstAmbiguous`.
Control Request:
```yaml
id: 1c3b-113
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  id: hRC4sE
  old_text: "from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_path"
  new_text: "from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_text"
  reason: select_by_text statt strikter select_by_path importieren
```
/answer 1c3b-113 allow
Text:
 Now edit edit_block.py: import select_by_text, use it, and handle AstAmbiguous in `handle`.
Control Request:
```yaml
id: 1c3b-114
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  id: ast_edit_block
  old_text: "    target = select_by_path(tree, id=id)"
  new_text: "    target = select_by_text(tree, [old_text], id=id)"
  reason: "Fallback-Suche über old_text nutzen, wenn id fehlt"
```
/answer 1c3b-114 allow
Control Request:
```yaml
id: 1c3b-115
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  id: ast_edit_marks
  old_text: "    target = select_by_path(tree, id=id)"
  new_text: "    target = select_by_text(tree, [start_marker, end_marker], id=id)"
  reason: "Fallback-Suche über start_marker/end_marker nutzen, wenn id fehlt"
```
/answer 1c3b-115 allow
Control Request:
```yaml
id: 1c3b-116
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  id: ast_edit_block
  old_text: |2-
            id: Unique id of the target node.

        Returns:
            EditBlockNodeResult: Success status.

        Raises:
            core.AstError: If ``path`` is invalid, ``id`` is not
                given, the path matches zero or more than one node, ``old_text`` is not
                found or (without ``replace_all``) ambiguous within the node's source, or
                the edited source has a syntax error.
  new_text: |2-
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
  reason: Docstring an neues Fallback-Verhalten anpassen
```
/answer 1c3b-116 allow
Control Request:
```yaml
id: 1c3b-117
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  id: ast_edit_marks
  old_text: |2-
            id: Unique id of the target node.

        Returns:
            EditMarksNodeResult: Success status.

        Raises:
            core.AstError: If ``path`` is invalid, ``id`` is not
                given, the path matches zero or more than one node, the markers are not
                found or ambiguous within the node's source, or the edited source has a
                syntax error.
  new_text: |2-
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
  reason: Docstring an neues Fallback-Verhalten anpassen
```
/answer 1c3b-117 allow
Control Request:
```yaml
id: 1c3b-118
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  id: EditBlockNodeTool.handle
  old_text: |2-
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
  new_text: |2-
        except core.AstAmbiguous as exc:
            return ToolResult(
                content=[text_content(str(exc))],
                structured_content={'candidates': exc.candidates},
                is_error=True)
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
  reason: Kandidatenliste bei Ambiguität typisiert in structured_content zurückgeben
```
/answer 1c3b-118 allow
Control Request:
```yaml
id: 1c3b-119
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  id: EditMarksNodeTool.handle
  old_text: |2-
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
  new_text: |2-
        except core.AstAmbiguous as exc:
            return ToolResult(
                content=[text_content(str(exc))],
                structured_content={'candidates': exc.candidates},
                is_error=True)
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
  reason: Kandidatenliste bei Ambiguität typisiert in structured_content zurückgeben
```
/answer 1c3b-119 allow
ReasoningToken: 69
Text:
 Now update `handle()` in both tool classes to surface candidates typed in structured_content on `AstAmbiguous`.
Control Request:
```yaml
id: 1c3b-11a
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  id: EditBlockNodeTool.oC5twa
  old_text: |2-
            'id': {
                'type': 'string',
                'description': "The node's new id."}},
        'required': ['result']}
  new_text: |2-
            'id': {
                'type': 'string',
                'description': "The node's new id."},
            'candidates': {
                'type': 'array',
                'items': {'type': 'string'},
                'description': 'On ambiguity (id omitted, several nodes matched), the candidate node ids.'}},
        'required': ['result']}
  reason: candidates-Feld im output_schema dokumentieren
```
/answer 1c3b-11a allow
Control Request:
```yaml
id: 1c3b-11b
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  id: EditMarksNodeTool.oC5twa
  old_text: |2-
            'id': {
                'type': 'string',
                'description': "The node's new id."}},
        'required': ['result']}
  new_text: |2-
            'id': {
                'type': 'string',
                'description': "The node's new id."},
            'candidates': {
                'type': 'array',
                'items': {'type': 'string'},
                'description': 'On ambiguity (id omitted, several nodes matched), the candidate node ids.'}},
        'required': ['result']}
  reason: candidates-Feld im output_schema dokumentieren
```
/answer 1c3b-11b allow
Control Request:
```yaml
id: 1c3b-11c
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  reason: Alle geänderten Dateien auf Syntaxfehler prüfen
```
/answer 1c3b-11c allow
Text:
 Now update the output_schema to document optional `candidates` field, and required-list for input_schema is unaffected since `id` already optional. Let's check output_schema and update.
Text:
 Now validate all changed files.
Result Stats:  id=af0ee96d-700e-4985-a8a0-bea7c89b62ca, total: 0, in: 187, out: 15859, reason: 0, read: 658737, write: 37738
Alle Dateien validiert. Änderungen implementiert:

- `base.py`: neue `AstAmbiguous(AstError)` mit `candidates: list[str]`.
- `core.py`: reexportiert `AstAmbiguous`.
- `common.py`: neue `select_by_text(tree, texts, *, id=None)` – bei fehlender `id` werden alle Knoten gesucht, deren Quelltext alle `texts` enthält; verschachtelte Kandidaten (Vorfahre enthält Nachfahre) werden auf den innersten reduziert; bleiben mehrere unabhängige Kandidaten, wird `AstAmbiguous` mit deren IDs geworfen.
- `edit_block.py` / `edit_marks.py`: nutzen `select_by_text` mit `[old_text]` bzw. `[start_marker, end_marker]`; `handle()` fängt `AstAmbiguous` und liefert die Kandidatenliste typisiert im `structured_content.candidates`; Docstrings und `output_schema` entsprechend ergänzt.
Ok, ich habe den MCPC aktualisiert. Teste die Änderung.
SystemInit:  id=af0ee96d-700e-4985-a8a0-bea7c89b62ca, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 065d-1
toolName: ast_create
arguments:
  path: /tmp/test_select_by_text.py
  source: |
    def outer():
        def inner():
            return 1
        return inner()


    def other():
        return 1
  reason: "Testdatei erstellen, um Fallback-Suche (select_by_text) für ast_edit_block/ast_edit_marks ohne id zu testen"
```
/answer 065d-1 allow
ReasoningToken: 240
Control Request:
```yaml
id: 065d-2
toolName: ast_edit_block
arguments:
  path: /tmp/test_select_by_text.py
  old_text: '    return 1'
  new_text: '    return 2'
  reason: "Testen der Ambiguitäts-Behandlung: 'return 1' kommt in zwei unabhängigen Funktionen vor"
```
/answer 065d-2 allow
Control Request:
```yaml
id: 065d-3
result:
  content:
  - type: text
    text: Multiple target nodes found; specify a node selector (id).
  structuredContent:
    candidates:
    - outer.inner
    - other
  isError: true
```
/answer 065d-3 allow
ReasoningToken: 199
Text:
 Ambiguity-Fall funktioniert. Jetzt der Nested-Reduction- und Happy-Path-Test mit eindeutigem Text nur in `inner`:
Control Request:
```yaml
id: 065d-4
toolName: ast_edit_block
arguments:
  path: /tmp/test_select_by_text.py
  old_text: |2-
        def inner():
            return 1
  new_text: |2-
        def inner():
            return 42
  reason: "Testen: eindeutiger Text nur in innerem Knoten, obwohl outer() ihn ebenfalls enthält (Nested Reduction) - ohne id"
```
/answer 065d-4 allow
Control Request:
```yaml
id: 065d-5
toolName: ast_find
arguments:
  paths:
  - /tmp/test_select_by_text.py
  reason: Ergebnis der Testdatei nach Edit prüfen
  node_type: FunctionDef
```
/answer 065d-5 allow
Control Request:
```yaml
id: 065d-6
result:
  structuredContent:
    files:
    - path: /tmp/test_select_by_text.py
      nodes:
      - id: outer
        type: FunctionDef
        code: |-
          def outer():

              def inner():
                  return 42
              return inner()
      - id: outer.inner
        type: FunctionDef
        code: |-
          def inner():
              return 42
      - id: other
        type: FunctionDef
        code: |-
          def other():
              return 1
```
/answer 065d-6 allow
ReasoningToken: 169
Text:
 Korrekt: nur `inner` wurde geändert (innerster Kandidat gewählt), `other` blieb unberührt, Nested-Reduction funktioniert.

Jetzt der Not-Found-Fall:
Control Request:
```yaml
id: 065d-7
toolName: ast_edit_block
arguments:
  path: /tmp/test_select_by_text.py
  old_text: totally_not_present_x
  new_text: whatever
  reason: "Testen: kein Knoten enthält den Text -> AstError erwartet"
```
/answer 065d-7 allow
Control Request:
```yaml
id: 065d-8
result:
  content:
  - type: text
    text: No node matched the given text; a node selector (id) is required.
  isError: true
```
/answer 065d-8 allow
Text:
 Jetzt `ast_edit_marks` ohne id testen:
Control Request:
```yaml
id: 065d-9
toolName: ast_edit_marks
arguments:
  path: /tmp/test_select_by_text.py
  start_marker: "def other():"
  end_marker: return 1
  content: |-
    def other():
        return 99
  reason: Fallback-Suche für ast_edit_marks ohne id testen (Marker eindeutig nur in other())
```
/answer 065d-9 allow
Control Request:
```yaml
id: 065d-a
toolName: ast_find
arguments:
  paths:
  - /tmp/test_select_by_text.py
  reason: Ergebnis nach ast_edit_marks-Test prüfen
  node_type: FunctionDef
```
/answer 065d-a allow
Control Request:
```yaml
id: 065d-b
result:
  structuredContent:
    files:
    - path: /tmp/test_select_by_text.py
      nodes:
      - id: outer
        type: FunctionDef
        code: |-
          def outer():

              def inner():
                  return 42
              return inner()
      - id: outer.inner
        type: FunctionDef
        code: |-
          def inner():
              return 42
      - id: other
        type: FunctionDef
        code: |-
          def other():
              return 99
```
/answer 065d-b allow
Control Request:
```yaml
id: 065d-c
toolName: ast_delete
arguments:
  path: /tmp/test_select_by_text.py
  reason: Testdatei nach abgeschlossenem Test aufräumen
```
/answer 065d-c allow
Result Stats:  id=af0ee96d-700e-4985-a8a0-bea7c89b62ca, total: 0, in: 203, out: 18288, reason: 0, read: 971667, write: 40809
Alle Tests erfolgreich:

1. **Ambiguität** (`return 1` in zwei unabhängigen Funktionen) → `AstError: "Multiple target nodes found; specify a node selector (id)."`
2. **Nested Reduction** (`ast_edit_block`, Text nur eindeutig im inneren Knoten, äußerer enthält ihn ebenfalls) → korrekt der innerste Knoten (`inner`) getroffen, `outer` unverändert.
3. **Not-Found** (Text in keinem Knoten) → `AstError: "No node matched the given text..."`
4. **`ast_edit_marks` ohne id** (Marker eindeutig nur in `other`) → korrekt `other` getroffen und ersetzt.

Testdatei wieder gelöscht.
