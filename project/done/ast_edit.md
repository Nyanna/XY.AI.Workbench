Ich möchte, das du ein neues AST Tool "ast_edit" erstellst das Batch Operationen für alle AST-Edit Tools anbietet.
Der Agent soll alle modifizierenden AST Tools in einem einzigen großen Tool Call verwenden können.
Jedes Edit Tool bekommt seine eigene typisierte Liste in den Eingabeparametern so das ohne OneOf/AnyOf gearbeitet werden kann.
Das Inputschema soll dabei aus den Edit Tools referenziert werden.

`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py`
SystemInit:  id=4f6aaf16-dce4-444d-9cb3-c6d387895bd3, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Control Request:
```yaml
id: 4f11-1-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
  reason: Understand structure of the six edit tools
```
/answer 4f11-1-0 allow
Control Request:
```yaml
id: 4f11-2-1
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
      nodes:
      - id: ZAIFPk|rsb8Xe
        excert: "'``ast_create`` tool: create one or more files from source, creating missing di…"
      - id: 8zfkq5|b8BrXU
        excert: from dataclasses import dataclass
      - id: wtzT47|EP6uG9
        excert: "__all__ = ["
      - id: CreateFileItem
        excert: "@dataclass(frozen=True) class CreateFileItem:"
        docstring: "One file to create. Attributes: path: Absolute path of the file to create. sour…"
      - id: CreateFileResult
        excert: "@dataclass(frozen=True) class CreateFileResult(ItemResult):"
        docstring: "Result of creating a single file. Attributes: path: The path exactly as given i…"
        children:
        - id: CreateFileResult.ck54as|jvxqvL
          excert: "'Result of creating a single file.\\n\\n    Attributes:\\n        path: The path e…"
        - id: CreateFileResult.key
          excert: "def key(self) -> dict[str, Any]:"
      - id: CreateFileError
        excert: "@dataclass(frozen=True) class CreateFileError(ItemError):"
        docstring: "Error creating a single file. Attributes: path: The path exactly as given in th…"
        children:
        - id: CreateFileError.z9L1ur|oLYcZT
          excert: "'Error creating a single file.\\n\\n    Attributes:\\n        path: The path exact…"
        - id: CreateFileError.key
          excert: "def key(self) -> dict[str, Any]:"
      - id: CreateFilesResult
        excert: "class CreateFilesResult(BatchResult[CreateFileResult, CreateFileError]):"
        docstring: "Result of :func:`ast_create`. Attributes: results: One :class:`CreateFileResult…"
      - id: create_one
        excert: "def _create_one(item: CreateFileItem) -> CreateFileResult:"
      - id: ast_create
        excert: "def ast_create(items: list[CreateFileItem]) -> CreateFilesResult:"
        docstring: Create one or more files from source (each validated by parsing it). Creating a…
      - id: CreateFileTool
        excert: "class CreateFileTool(ToolDefinition):"
        children:
        - id: CreateFileTool.omImBZ|s2PVZe
          excert: name = 'ast_create'
        - id: CreateFileTool.ei5nIo|Lu9IT0
          excert: "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTIO…"
        - id: CreateFileTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_create`, translating the MCP schema to/from the Python A…"
      - id: register
        excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      nodes:
      - id: CpezhJ|nJnt4G
        excert: "'``ast_delete`` tool: delete selected nodes, or whole files if none selected.'"
      - id: 8Wm4pY|RoeKEJ
        excert: from dataclasses import dataclass
      - id: qzaxoR|HxGFbQ
        excert: "from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH…"
      - id: zTddKa|ZtwRn0
        excert: "__all__ = ['DeleteItem', 'DeleteResult', 'DeleteError', 'DeleteBatchResult', 'a…"
      - id: DeleteItem
        excert: "@dataclass(frozen=True) class DeleteItem:"
        docstring: "One node (or file) to delete. Attributes: path: Absolute path to the file to mo…"
      - id: DeleteResult
        excert: "@dataclass(frozen=True) class DeleteResult(ItemResult):"
        docstring: "Result of deleting a single node or file. Attributes: path: The path exactly as…"
        children:
        - id: DeleteResult.brBMR9|fakDck
          excert: "'Result of deleting a single node or file.\\n\\n    Attributes:\\n        path: Th…"
        - id: DeleteResult.key
          excert: "def key(self) -> dict[str, Any]:"
        - id: DeleteResult.payload
          excert: "def payload(self) -> dict[str, Any]:"
      - id: DeleteError
        excert: "class DeleteError(NodeError):"
        docstring: "Error deleting a single node or file. Attributes: path: The path exactly as giv…"
      - id: DeleteBatchResult
        excert: "class DeleteBatchResult(ImportCheckedBatchResult[DeleteResult, DeleteError]):"
        docstring: "Result of :func:`ast_delete`. Attributes: results: One :class:`DeleteResult` pe…"
      - id: delete_one
        excert: "def _delete_one(item: DeleteItem) -> DeleteResult:"
      - id: ast_delete
        excert: "def ast_delete(items: list[DeleteItem]) -> DeleteBatchResult:"
        docstring: "Delete one or more selected nodes, or whole files if their ``id`` is omitted. A…"
      - id: DeleteTool
        excert: "class DeleteTool(ToolDefinition):"
        children:
        - id: DeleteTool.gLGwBi|JNohfS
          excert: name = 'ast_delete'
        - id: DeleteTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_delete`, translating the MCP schema to/from the Python A…"
      - id: register
        excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
      nodes:
      - id: mQ7CrW|aoOqEy
        excert: "'``ast_edit_block`` tool: exact-block (old_text -> new_text) edits within selec…"
      - id: S7ZTCd|76ji8H
        excert: from dataclasses import dataclass
      - id: qzaxoR|MrmMMo
        excert: "from xy.ai.mcpc.tools.ast.outcomes import NodeError, edit_payload, node_key"
      - id: B1sD7y|298I9E
        excert: "__all__ = ["
      - id: EditBlockItem
        excert: "@dataclass(frozen=True) class EditBlockItem:"
        docstring: "One block edit to apply. Attributes: path: Absolute path to the file to modify.…"
      - id: EditBlockResult
        excert: "@dataclass(frozen=True) class EditBlockResult(ItemResult):"
        docstring: "Result of applying a single block edit. Attributes: path: The path exactly as g…"
        children:
        - id: EditBlockResult.syA5Nf|eIE3hg
          excert: "\"Result of applying a single block edit.\\n\\n    Attributes:\\n        path: The …"
        - id: EditBlockResult.key
          excert: "def key(self) -> dict[str, Any]:"
        - id: EditBlockResult.payload
          excert: "def payload(self) -> dict[str, Any]:"
      - id: EditBlockError
        excert: "@dataclass(frozen=True) class EditBlockError(NodeError):"
        docstring: "Error applying a single block edit. Attributes: path: The path exactly as given…"
        children:
        - id: EditBlockError.4W3yhx|RU9DUu
          excert: "\"Error applying a single block edit.\\n\\n    Attributes:\\n        path: The path…"
        - id: EditBlockError.0rjxOi|HUGkD6
          excert: "candidates: list[str] | None = None"
        - id: EditBlockError.payload
          excert: "def payload(self) -> dict[str, Any]:"
      - id: EditBlockBatchResult
        excert: "class EditBlockBatchResult(ImportCheckedBatchResult[EditBlockResult, EditBlockError]):"
        docstring: "Result of :func:`ast_edit_block`. Attributes: results: One :class:`EditBlockRes…"
      - id: node_guard
        excert: "def _node_guard(engine, reference):"
        docstring: Guard for tolerant node edits. Engines that flag malformed edits on replace nee…
      - id: edit_block_one
        excert: "def _edit_block_one(item: EditBlockItem) -> EditBlockResult:"
      - id: ast_edit_block
        excert: "def ast_edit_block(items: list[EditBlockItem]) -> EditBlockBatchResult:"
        docstring: Replace occurrence(s) of ``old_text`` with ``new_text`` inside one or more node…
      - id: EditBlockNodeTool
        excert: "class EditBlockNodeTool(ToolDefinition):"
        children:
        - id: EditBlockNodeTool.aKEDSe|mSaRry
          excert: name = 'ast_edit_block'
        - id: EditBlockNodeTool.Z2OjHo|ftL6Rn
          excert: "_ITEM_PROPERTIES = {"
        - id: EditBlockNodeTool.ZDSH0j|Jr7UgN
          excert: "_ITEM_REQUIRED = ['path', 'old_text', 'new_text', 'id']"
        - id: EditBlockNodeTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_edit_block`, translating the MCP schema to/from the AST …"
      - id: register
        excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
      nodes:
      - id: zKe9Du|5qARzC
        excert: "'``ast_edit_marks`` tool: mark-based edits within the source of selected nodes.'"
      - id: S7ZTCd|76ji8H
        excert: from dataclasses import dataclass
      - id: qzaxoR|nDmbXU
        excert: "from xy.ai.mcpc.tools.ast.outcomes import NodeError, edit_payload, node_key"
      - id: IklaUl|hqD6GH
        excert: "__all__ = ["
      - id: EditMarksItem
        excert: "@dataclass(frozen=True) class EditMarksItem:"
        docstring: "One marker-based edit to apply. Attributes: path: Absolute path to the file to …"
      - id: EditMarksResult
        excert: "@dataclass(frozen=True) class EditMarksResult(ItemResult):"
        docstring: "Result of applying a single marker-based edit. Attributes: path: The path exact…"
        children:
        - id: EditMarksResult.syA5Nf|bgha0p
          excert: '"Result of applying a single marker-based edit.\n\n    Attributes:\n        pat…'
        - id: EditMarksResult.key
          excert: "def key(self) -> dict[str, Any]:"
        - id: EditMarksResult.payload
          excert: "def payload(self) -> dict[str, Any]:"
      - id: EditMarksError
        excert: "@dataclass(frozen=True) class EditMarksError(NodeError):"
        docstring: "Error applying a single marker-based edit. Attributes: path: The path exactly a…"
        children:
        - id: EditMarksError.4W3yhx|Vs4AGM
          excert: "\"Error applying a single marker-based edit.\\n\\n    Attributes:\\n        path: T…"
        - id: EditMarksError.0rjxOi|vs8MZ5
          excert: "candidates: list[str] | None = None"
        - id: EditMarksError.payload
          excert: "def payload(self) -> dict[str, Any]:"
      - id: EditMarksBatchResult
        excert: "class EditMarksBatchResult(ImportCheckedBatchResult[EditMarksResult, EditMarksError]):"
        docstring: "Result of :func:`ast_edit_marks`. Attributes: results: One :class:`EditMarksRes…"
      - id: node_marks_guard
        excert: "def _node_marks_guard(engine, begin_marker, end_marker):"
        docstring: Guard for tolerant marker edits (see ``ast.edit_block._node_guard``).
      - id: edit_marks_one
        excert: "def _edit_marks_one(item: EditMarksItem) -> EditMarksResult:"
      - id: ast_edit_marks
        excert: "def ast_edit_marks(items: list[EditMarksItem]) -> EditMarksBatchResult:"
        docstring: Replace everything between the start/end markers inside one or more nodes. Each…
      - id: EditMarksNodeTool
        excert: "class EditMarksNodeTool(ToolDefinition):"
        children:
        - id: EditMarksNodeTool.tqcnP7|JTGu41
          excert: name = 'ast_edit_marks'
        - id: EditMarksNodeTool.Z2OjHo|jMiCNo
          excert: "_ITEM_PROPERTIES = {"
        - id: EditMarksNodeTool.ZDSH0j|PEqhnG
          excert: "_ITEM_REQUIRED = ['path', 'start_marker', 'end_marker', 'content']"
        - id: EditMarksNodeTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_edit_marks`, translating the MCP schema to/from the AST …"
      - id: register
        excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
      nodes:
      - id: 9CmN9x|nHobTc
        excert: "'``ast_insert`` tool: insert statement(s) relative to selected nodes.'"
      - id: 8Wm4pY|RoeKEJ
        excert: from dataclasses import dataclass
      - id: qzaxoR|HxGFbQ
        excert: "from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH…"
      - id: oTjM4y|Jut0OL
        excert: "__all__ = ['InsertItem', 'InsertResult', 'InsertError', 'InsertBatchResult', 'a…"
      - id: InsertItem
        excert: "@dataclass(frozen=True) class InsertItem:"
        docstring: "One insert operation. Attributes: path: Absolute path to the file to modify. so…"
      - id: InsertResult
        excert: "@dataclass(frozen=True) class InsertResult(ItemResult):"
        docstring: "Result of a single insert operation. Attributes: path: The path exactly as give…"
        children:
        - id: InsertResult.aUv9Tr|y2n1TA
          excert: "\"Result of a single insert operation.\\n\\n    Attributes:\\n        path: The pat…"
        - id: InsertResult.key
          excert: "def key(self) -> dict[str, Any]:"
        - id: InsertResult.payload
          excert: "def payload(self) -> dict[str, Any]:"
      - id: InsertError
        excert: "class InsertError(NodeError):"
        docstring: "Error applying a single insert operation. Attributes: path: The path exactly as…"
      - id: InsertBatchResult
        excert: "class InsertBatchResult(ImportCheckedBatchResult[InsertResult, InsertError]):"
        docstring: "Result of :func:`ast_insert`. Attributes: results: One :class:`InsertResult` pe…"
      - id: insert_one
        excert: "def _insert_one(item: InsertItem) -> InsertResult:"
      - id: ast_insert
        excert: "def ast_insert(items: list[InsertItem]) -> InsertBatchResult:"
        docstring: Insert statement(s) parsed from ``source`` relative to one or more selected nod…
      - id: InsertNodeTool
        excert: "class InsertNodeTool(ToolDefinition):"
        children:
        - id: InsertNodeTool.3a1iRP|qj85Mb
          excert: name = 'ast_insert'
        - id: InsertNodeTool.c0C4qn|Ha6JTT
          excert: "_ITEM_PROPERTIES = {"
        - id: InsertNodeTool.ei5nIo|Lu9IT0
          excert: "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTIO…"
        - id: InsertNodeTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_insert`, translating the MCP schema to/from the AST API."
      - id: register
        excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
      nodes:
      - id: G65kv0|hrDhpa
        excert: "'``ast_replace`` tool: replace selected nodes with new source.'"
      - id: 8Wm4pY|RoeKEJ
        excert: from dataclasses import dataclass
      - id: qzaxoR|HxGFbQ
        excert: "from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH…"
      - id: xj2nBU|s4L0vm
        excert: "__all__ = ["
      - id: ReplaceItem
        excert: "@dataclass(frozen=True) class ReplaceItem:"
        docstring: "One node replacement to apply. Attributes: path: Absolute path to the file to m…"
      - id: ReplaceResult
        excert: "@dataclass(frozen=True) class ReplaceResult(ItemResult):"
        docstring: "Result of a single node replacement. Attributes: path: The path exactly as give…"
        children:
        - id: ReplaceResult.xDAyo0|mkL48U
          excert: "\"Result of a single node replacement.\\n\\n    Attributes:\\n        path: The pat…"
        - id: ReplaceResult.key
          excert: "def key(self) -> dict[str, Any]:"
        - id: ReplaceResult.payload
          excert: "def payload(self) -> dict[str, Any]:"
      - id: ReplaceError
        excert: "class ReplaceError(NodeError):"
        docstring: "Error applying a single node replacement. Attributes: path: The path exactly as…"
      - id: ReplaceBatchResult
        excert: "class ReplaceBatchResult(ImportCheckedBatchResult[ReplaceResult, ReplaceError]):"
        docstring: "Result of :func:`ast_replace`. Attributes: results: One :class:`ReplaceResult` …"
      - id: replace_one
        excert: "def _replace_one(item: ReplaceItem) -> ReplaceResult:"
      - id: ast_replace
        excert: "def ast_replace(items: list[ReplaceItem]) -> ReplaceBatchResult:"
        docstring: Replace one or more selected nodes with new source. ``id`` and replacement ``so…
      - id: ReplaceNodeTool
        excert: "class ReplaceNodeTool(ToolDefinition):"
        children:
        - id: ReplaceNodeTool.jKVWKu|Cre3Kp
          excert: name = 'ast_replace'
        - id: ReplaceNodeTool.hKJ2Rq|SrCCxe
          excert: _ITEMS_DESCRIPTION = 'Node replacements to apply.'
        - id: ReplaceNodeTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_replace`, translating the MCP schema to/from the AST API."
      - id: register
        excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 4f11-2-1 allow
Control Request:
```yaml
id: 4f11-3-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
    ids:
    - CreateFileTool
    - register
    - ast_create
    - CreateFilesResult
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    ids:
    - DeleteTool
    - register
    - ast_delete
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    ids:
    - EditBlockNodeTool
    - register
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    ids:
    - ReplaceNodeTool
  reason: Read tool classes to see schema and handle patterns
```
/answer 4f11-3-0 allow
Control Request:
```yaml
id: 4f11-4-3
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
      nodes:
      - id: CreateFileTool
        type: ClassDef
        excert: "class CreateFileTool(ToolDefinition):"
        children:
        - id: CreateFileTool.omImBZ|s2PVZe
          type: statements
          code: |-
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
        - id: CreateFileTool.ei5nIo|Lu9IT0
          type: statements
          code: "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)"
        - id: CreateFileTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_create`, translating the MCP schema to/from the Python API."""

                def item_factory(it: dict[str, Any]) -> CreateFileItem:
                    return CreateFileItem(path=it['path'], source=it['source'], overwrite=it.get('overwrite', False))
                return handle_batch_tool(ctx, item_factory, ast_create, core.AstError, auto_approve=True)
      - id: register
        type: FunctionDef
        code: |-
          def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
              registry.register(CreateFileTool())
              functions.register(ast_create)
      - id: ast_create
        type: FunctionDef
        code: |-
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
      - id: CreateFilesResult
        type: ClassDef
        code: |-
          class CreateFilesResult(BatchResult[CreateFileResult, CreateFileError]):
              """Result of :func:`ast_create`.

              Attributes:
                  results: One :class:`CreateFileResult` per file created successfully.
                  errors: One :class:`CreateFileError` per file that failed.
              """
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      nodes:
      - id: DeleteTool
        type: ClassDef
        excert: "class DeleteTool(ToolDefinition):"
        children:
        - id: DeleteTool.gLGwBi|JNohfS
          type: statements
          code: |-
            name = 'ast_delete'
            title = 'Delete AST nodes or files'
            description = 'Delete selected nodes from files, or whole files if no selector is given, for a batch of items.'
            _ITEM_PROPERTIES = {'path': PATH_PROP, **PATH_SELECTOR_PROPS, **IMPORT_PROPS}
            _ITEM_REQUIRED = ['path']
            _ITEMS_DESCRIPTION = 'Nodes/files to delete.'
            input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)
        - id: DeleteTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_delete`, translating the MCP schema to/from the Python API."""

                def item_factory(it: dict[str, Any]) -> DeleteItem:
                    add_import, remove_import = parse_import_args(it)
                    return DeleteItem(path=it['path'], id=it.get('id'), add_import=add_import, remove_import=remove_import)
                return handle_batch_tool(ctx, item_factory, ast_delete, core.AstError, auto_approve=True)
      - id: register
        type: FunctionDef
        code: |-
          def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
              registry.register(DeleteTool())
              functions.register(ast_delete)
      - id: ast_delete
        type: FunctionDef
        code: |-
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
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
      nodes:
      - id: EditBlockNodeTool
        type: ClassDef
        excert: "class EditBlockNodeTool(ToolDefinition):"
        children:
        - id: EditBlockNodeTool.aKEDSe|mSaRry
          type: statements
          code: |-
            name = 'ast_edit_block'
            title = 'Replace short text within AST nodes'
            description = "Replace occurrences of short 'old_text' with 'new_text', within nodes addressed by prior known id, for a batch of items. Don't use for large edits, use ast_edit_marks instead."
        - id: EditBlockNodeTool.Z2OjHo|ftL6Rn
          type: statements
          code: |-
            _ITEM_PROPERTIES = {
                'path': PATH_PROP,
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
                **IMPORT_PROPS}
        - id: EditBlockNodeTool.ZDSH0j|Jr7UgN
          type: statements
          code: |-
            _ITEM_REQUIRED = ['path', 'old_text', 'new_text', 'id']
            _ITEMS_DESCRIPTION = 'Block edits to apply.'
            input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)
        - id: EditBlockNodeTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_edit_block`, translating the MCP schema to/from the AST API."""

                def item_factory(it: dict[str, Any]) -> EditBlockItem:
                    add_import, remove_import = parse_import_args(it)
                    return EditBlockItem(
                        path=it['path'],
                        old_text=it['old_text'],
                        new_text=it['new_text'],
                        exact=it.get(
                            'exact',
                            False),
                        replace_all=it.get(
                            'replaceAll',
                            False),
                        id=it.get('id'),
                        add_import=add_import,
                        remove_import=remove_import)
                return handle_batch_tool(ctx, item_factory, ast_edit_block, core.AstError, auto_approve=True)
      - id: register
        type: FunctionDef
        code: |-
          def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
              registry.register(EditBlockNodeTool())
              functions.register(ast_edit_block)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
      nodes:
      - id: ReplaceNodeTool
        type: ClassDef
        excert: "class ReplaceNodeTool(ToolDefinition):"
        children:
        - id: ReplaceNodeTool.jKVWKu|Cre3Kp
          type: statements
          code: |-
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
        - id: ReplaceNodeTool.hKJ2Rq|SrCCxe
          type: statements
          code: |-
            _ITEMS_DESCRIPTION = 'Node replacements to apply.'
            input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)
        - id: ReplaceNodeTool.handle
          type: FunctionDef
          code: |-
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
                return handle_batch_tool(ctx, item_factory, ast_replace, core.AstError, auto_approve=True)
```
/answer 4f11-4-3 allow
ReasoningToken: 54
Control Request:
```yaml
id: 4f11-5-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: def batch_schema|class BatchResult|class ImportCheckedBatchResult|ast\.(create|delete|replace)\b|import .*ast\.(create|replace)
    include:
    - '*.py'
    exclude:
    - tests/**
  reason: Locate batch_schema and registration of tools
```
/answer 4f11-5-0 allow
Control Request:
```yaml
id: 4f11-6-5
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
      matches:
      - path: ast/imports.py
        matches:
        - "class ImportCheckedBatchResult(BatchResult[R, E]):"
      - path: results.py
        matches:
        - "class BatchResult(ToolPayload, Generic[R, E]):"
      - path: _tool_helpers.py
        matches:
        - "def batch_schema(item_properties: dict[str, Any], required: list[str], description: str) -> dict[str, Any]:"
```
/answer 4f11-6-5 allow
Control Request:
```yaml
id: 4f11-7-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
  reason: "Understand handle_batch_tool, batch_schema and result payloads"
```
/answer 4f11-7-0 allow
Control Request:
```yaml
id: 4f11-8-7
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
      nodes:
      - id: j2F2xB|e5OyD1
        excert: '''Common helpers for tool definition and request handling.'''
      - id: pXkVK0|aIkesy
        excert: "from typing import Any, Callable, TypeVar"
      - id: xQ71by|MKAujh
        excert: "__all__ = ['require_items', 'handle_batch_tool', 'batch_tool_result', 'serializ…"
      - id: batch_schema
        excert: "def batch_schema(item_properties: dict[str, Any], required: list[str], description: str) -> dict[str, Any]:"
        docstring: "Build the standard ``{items: [...]}`` input schema shared by all batch tools. N…"
      - id: require_items
        excert: "def require_items(ctx: ToolContext, key: str='items') -> 'tuple[list[Any], ToolResult | None]':"
        docstring: "Return the non-empty list at *key* in ``ctx.arguments``, or an error result. Re…"
      - id: serialize_batch_result
        excert: "def serialize_batch_result(batch_result: BatchResult[Any, Any], result_payload: ResultPayload | None=None) -> dict[str, Any]:"
        docstring: Serialize the populated ``results``/``errors`` lists of a batch result. Warning…
      - id: batch_tool_result
        excert: "def batch_tool_result(batch_result: BatchResult[Any, Any], result_payload: ResultPayload | None=None, auto_approve: bool=False) -> ToolResult:"
        docstring: Wrap a batch result into a :class:`ToolResult` carrying its call-wide warnings.
      - id: handle_batch_tool
        excert: "def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], BatchResult[Any, Any]], error_class: type, result_payload: ResultPayload | None=None, auto_approve: bool=False) -> ToolResult:"
        docstring: "Common handler for batch-processing tools. Args: ctx: Tool context with argumen…"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
      nodes:
      - id: QD5cZ5|k9JYH8
        excert: "\"Common result model: items, batches and the single place that owns warnings.\\n…"
      - id: vlNV3F|ayn8b0
        excert: from __future__ import annotations
      - id: noZ1HO|JeywZP
        excert: "__all__ = ["
      - id: f9NIN4|pZYmYI
        excert: "WARNINGS_SCHEMA: dict[str, Any] = {'type': 'array', 'items': {'type': 'string'},"
      - id: compact
        excert: "def compact(value: Any) -> Any:"
        docstring: Recursively drop ``None`` values and empty lists from a dataclass-derived struc…
      - id: compact_asdict
        excert: "def compact_asdict(obj: Any, exclude: Iterable[str]=()) -> dict[str, Any]:"
      - id: guard_fields
        excert: "def _guard_fields(cls: type) -> None:"
      - id: checked_payload
        excert: "def _checked_payload(owner: object, payload: dict[str, Any]) -> dict[str, Any]:"
      - id: unknown_ids_warnings
        excert: "def unknown_ids_warnings(missing: Iterable[str]) -> list[str]:"
      - id: ToolPayload
        excert: "@dataclass(frozen=True, kw_only=True) class ToolPayload(ABC):"
        docstring: Typed root payload of a tool result; the only way to fill ``structured_content`…
        children:
        - id: ToolPayload.Jdu7i7|diAzsp
          excert: '''Typed root payload of a tool result; the only way to fill ``structured_content…'
        - id: ToolPayload.to_dict
          excert: "@abstractmethod def to_dict(self) -> dict[str, Any]:"
          docstring: Wire form of the payload (without root warnings).
        - id: ToolPayload.all_warnings
          excert: "def all_warnings(self) -> list[str]:"
          docstring: Call-wide warnings that travel at the root of the tool result.
        - id: ToolPayload.failed
          excert: "@property def failed(self) -> bool:"
          docstring: "Whether part of the call failed, which withholds automatic approval."
      - id: SinglePayload
        excert: "@dataclass(frozen=True, kw_only=True) class SinglePayload(ToolPayload):"
        docstring: Payload of a tool that returns one result instead of a batch; fields are serial…
        children:
        - id: SinglePayload.Xbkavl|uJ4Nzk
          excert: '''Payload of a tool that returns one result instead of a batch; fields are seria…'
        - id: SinglePayload.init_subclass
          excert: "def __init_subclass__(cls, **kwargs: Any) -> None:"
        - id: SinglePayload.payload
          excert: "def payload(self) -> dict[str, Any]:"
        - id: SinglePayload.to_dict
          excert: "@final def to_dict(self) -> dict[str, Any]:"
        - id: SinglePayload.all_warnings
          excert: "@final def all_warnings(self) -> list[str]:"
      - id: RawPayload
        excert: "@dataclass(frozen=True) class RawPayload(ToolPayload):"
        docstring: Untyped payload whose shape is fixed by someone else; *reason* documents why.
        children:
        - id: RawPayload.gqXWS3|wMnazL
          excert: '''Untyped payload whose shape is fixed by someone else; *reason* documents why.'''
        - id: RawPayload.post_init
          excert: "def __post_init__(self) -> None:"
        - id: RawPayload.to_dict
          excert: "def to_dict(self) -> dict[str, Any]:"
        - id: RawPayload.all_warnings
          excert: "def all_warnings(self) -> list[str]:"
      - id: ItemPayload
        excert: "@dataclass(frozen=True) class ItemPayload(ToolPayload):"
        docstring: A single :class:`ItemResult` used as the whole payload; its warnings move to th…
        children:
        - id: ItemPayload.ObWWN3|N67XK0
          excert: '''A single :class:`ItemResult` used as the whole payload; its warnings move to t…'
        - id: ItemPayload.to_dict
          excert: "def to_dict(self) -> dict[str, Any]:"
        - id: ItemPayload.all_warnings
          excert: "def all_warnings(self) -> list[str]:"
      - id: Done
        excert: "@dataclass(frozen=True, kw_only=True) class Done(SinglePayload):"
        docstring: Success of a call that returns no data.
      - id: ItemOutcome
        excert: "@dataclass(frozen=True, kw_only=True) class ItemOutcome(ABC):"
        children:
        - id: ItemOutcome.key
          excert: "@abstractmethod def key(self) -> dict[str, Any]:"
          docstring: Input fields mirrored for result association.
        - id: ItemOutcome.payload
          excert: "@abstractmethod def payload(self) -> dict[str, Any]:"
          docstring: Outcome-specific fields; must not use reserved names.
        - id: ItemOutcome.to_dict
          excert: "@abstractmethod def to_dict(self, *, payload: dict[str, Any] | None=None) -> dict[str, Any]:"
          docstring: Wire form; *payload* replaces :meth:`payload` for context-dependent rendering.
      - id: ItemResult
        excert: "@dataclass(frozen=True, kw_only=True) class ItemResult(ItemOutcome):"
        docstring: "An item that produced a result, possibly with restrictions (``warnings``)."
        children:
        - id: ItemResult.tihVbt|PBPiGn
          excert: "'An item that produced a result, possibly with restrictions (``warnings``).'"
        - id: ItemResult.init_subclass
          excert: "def __init_subclass__(cls, **kwargs: Any) -> None:"
        - id: ItemResult.payload
          excert: "def payload(self) -> dict[str, Any]:"
        - id: ItemResult.to_dict
          excert: "@final def to_dict(self, *, payload: dict[str, Any] | None=None) -> dict[str, Any]:"
        - id: ItemResult.warn
          excert: "def warn(self, *messages: str) -> Self:"
      - id: ItemError
        excert: "@dataclass(frozen=True, kw_only=True) class ItemError(ItemOutcome):"
        docstring: An item that produced no result.
        children:
        - id: ItemError.cNSh7Q|Ds7nZB
          excert: '''An item that produced no result.'''
        - id: ItemError.init_subclass
          excert: "def __init_subclass__(cls, **kwargs: Any) -> None:"
        - id: ItemError.payload
          excert: "def payload(self) -> dict[str, Any]:"
        - id: ItemError.to_dict
          excert: "@final def to_dict(self, *, payload: dict[str, Any] | None=None) -> dict[str, Any]:"
      - id: wk4AdA|Z3oVk9
        excert: "R = TypeVar('R', bound=ItemResult)"
      - id: BatchResult
        excert: "@dataclass(frozen=True, kw_only=True) class BatchResult(ToolPayload, Generic[R, E]):"
        children:
        - id: BatchResult.jpRjN8|8tyMAT
          excert: "results: list[R] = field(default_factory=list)"
        - id: BatchResult.init_subclass
          excert: "def __init_subclass__(cls, **kwargs: Any) -> None:"
        - id: BatchResult.post_init
          excert: "def __post_init__(self) -> None:"
        - id: BatchResult.to_dict
          excert: "@final def to_dict(self, result_payload: Callable[[Any], dict[str, Any]] | None=None) -> dict[str, Any]:"
        - id: BatchResult.failed
          excert: "@property def failed(self) -> bool:"
        - id: BatchResult.derived_warnings
          excert: "def derived_warnings(self) -> tuple[str, ...]:"
          docstring: Call-wide warnings that follow from the items and so cannot be forgotten.
        - id: BatchResult.all_warnings
          excert: "@final def all_warnings(self) -> list[str]:"
```
/answer 4f11-8-7 allow
ReasoningToken: 58
Control Request:
```yaml
id: 4f11-9-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    pattern: ast\.(create|delete|replace|insert|edit_block|edit_marks)|ast import|create\.register|replace\.register
    include:
    - '*.py'
  reason: Find where the AST tools are registered
```
/answer 4f11-9-0 allow
Control Request:
```yaml
id: 4f11-a-9
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      matches:
      - path: xy/ai/mcpc/tools/ast/find.py
        matches:
        - from xy.ai.mcpc.tools.ast import core
      - path: xy/ai/mcpc/tools/ast/script.py
        matches:
        - from xy.ai.mcpc.tools.ast import core
      - path: xy/ai/mcpc/tools/ast/replace.py
        matches:
        - from xy.ai.mcpc.tools.ast import core
      - path: xy/ai/mcpc/tools/ast/delete.py
        matches:
        - from xy.ai.mcpc.tools.ast import core
      - path: xy/ai/mcpc/tools/ast/outline.py
        matches:
        - from xy.ai.mcpc.tools.ast import core
      - path: xy/ai/mcpc/tools/ast/common.py
        matches:
        - from xy.ai.mcpc.tools.ast import core
      - path: xy/ai/mcpc/tools/ast/__init__.py
        matches:
        - "from xy.ai.mcpc.tools.ast import create, delete, edit_block, edit_marks, find, insert, read, replace, script, validate"
        - from xy.ai.mcpc.tools.ast import outline
        - "replace.register(registry, functions)"
        - "create.register(registry, functions)"
      - path: xy/ai/mcpc/tools/ast/edit_marks.py
        matches:
        - from xy.ai.mcpc.tools.ast import core
        - '"""Guard for tolerant marker edits (see ``ast.edit_block._node_guard``)."""'
      - path: xy/ai/mcpc/tools/ast/read.py
        matches:
        - from xy.ai.mcpc.tools.ast import core
      - path: xy/ai/mcpc/tools/ast/validate.py
        matches:
        - from xy.ai.mcpc.tools.ast import core
      - path: xy/ai/mcpc/tools/ast/edit_block.py
        matches:
        - from xy.ai.mcpc.tools.ast import core
      warnings:
      - "Limit of 15 matches reached (total 23); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
```
/answer 4f11-a-9 allow
Control Request:
```yaml
id: 4f11-b-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
    ids:
    - batch_schema
    - require_items
    - serialize_batch_result
    - batch_tool_result
    - handle_batch_tool
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
    ids:
    - BatchResult
    - ToolPayload
    - RawPayload
  reason: Read helper implementations in full
```
/answer 4f11-b-0 allow
Control Request:
```yaml
id: 4f11-c-b
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
      nodes:
      - id: batch_schema
        type: FunctionDef
        code: |-
          def batch_schema(item_properties: dict[str, Any], required: list[str], description: str) -> dict[str, Any]:
              """Build the standard ``{items: [...]}`` input schema shared by all batch tools.

              Neither the top level nor the items accept properties beyond the declared ones.

              Args:
                  item_properties: The ``properties`` of a single item.
                  required: The ``required`` keys of a single item.
                  description: Description of the ``items`` array.
              """
              item_schema: dict[str, Any] = {'type': 'object', 'properties': item_properties,
                                             'required': required, 'additionalProperties': False}
              return {
                  'type': 'object',
                  'properties': {
                      'items': {
                          'type': 'array',
                          'minItems': 1,
                          'items': item_schema,
                          'description': description}},
                  'required': ['items'],
                  'additionalProperties': False}
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
                  return ([], ToolResult.failure(f"'{key}' must be a non-empty list."))
              return (values, None)
      - id: serialize_batch_result
        type: FunctionDef
        code: |-
          def serialize_batch_result(batch_result: BatchResult[Any, Any], result_payload: ResultPayload | None=None) -> dict[str, Any]:
              """Serialize the populated ``results``/``errors`` lists of a batch result.

              Warnings are not part of this dict; they travel via :attr:`ToolResult.warnings`.
              """
              if not isinstance(batch_result, BatchResult):
                  raise TypeError(f'Expected a BatchResult, got {type(batch_result).__name__}')
              return batch_result.to_dict(result_payload)
      - id: batch_tool_result
        type: FunctionDef
        code: |-
          def batch_tool_result(batch_result: BatchResult[Any, Any], result_payload: ResultPayload | None=None, auto_approve: bool=False) -> ToolResult:
              """Wrap a batch result into a :class:`ToolResult` carrying its call-wide warnings."""
              return ToolResult.ok(
                  batch_result,
                  auto_approve=auto_approve,
                  structured=serialize_batch_result(
                      batch_result,
                      result_payload))
      - id: handle_batch_tool
        type: FunctionDef
        code: |-
          def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], BatchResult[Any, Any]], error_class: type, result_payload: ResultPayload | None=None, auto_approve: bool=False) -> ToolResult:
              """Common handler for batch-processing tools.

              Args:
                  ctx: Tool context with arguments.
                  item_factory: Function to convert raw dict to typed item.
                  batch_fn: Function to process list of items, returning batch result.
                  error_class: Exception class raised by `batch_fn` for whole-batch failures.
                  result_payload: Optional context-dependent payload for result items (default: ``payload()``).
                  auto_approve: Approve the call automatically if no item failed.

              Returns:
                  ToolResult with serialized batch result and its warnings.
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
                      return ToolResult.failure(f'Invalid item: missing required property {key!r}')
                  except (ValueError, TypeError) as exc:
                      return ToolResult.failure(f'Invalid item: {exc}')
              try:
                  batch_result = batch_fn(items)
              except error_class as exc:
                  return ToolResult.failure(str(exc))
              return batch_tool_result(batch_result, result_payload, auto_approve)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
      nodes:
      - id: BatchResult
        type: ClassDef
        excert: "@dataclass(frozen=True, kw_only=True) class BatchResult(ToolPayload, Generic[R, E]):"
        children:
        - id: BatchResult.jpRjN8|8tyMAT
          type: statements
          code: |-
            results: list[R] = field(default_factory=list)
            errors: list[E] = field(default_factory=list)
            warnings: tuple[str, ...] = ()
        - id: BatchResult.init_subclass
          type: FunctionDef
          code: |-
            def __init_subclass__(cls, **kwargs: Any) -> None:
                super().__init_subclass__(**kwargs)
                _guard_fields(cls)
        - id: BatchResult.post_init
          type: FunctionDef
          code: |-
            def __post_init__(self) -> None:
                if not all((isinstance(r, ItemResult) for r in self.results)):
                    raise TypeError('results must be ItemResult instances')
                if not all((isinstance(e, ItemError) for e in self.errors)):
                    raise TypeError('errors must be ItemError instances')
        - id: BatchResult.to_dict
          type: FunctionDef
          code: |-
            @final
            def to_dict(self, result_payload: Callable[[Any], dict[str, Any]] | None=None) -> dict[str, Any]:
                content: dict[str, Any] = {}
                if self.results:
                    content['results'] = [r.to_dict() if result_payload is None else r.to_dict(payload=result_payload(r))
                                          for r in self.results]
                if self.errors:
                    content['errors'] = [e.to_dict() for e in self.errors]
                return content
        - id: BatchResult.failed
          type: FunctionDef
          code: |-
            @property
            def failed(self) -> bool:
                return bool(self.errors)
        - id: BatchResult.derived_warnings
          type: FunctionDef
          code: |-
            def derived_warnings(self) -> tuple[str, ...]:
                """Call-wide warnings that follow from the items and so cannot be forgotten."""
                return ()
        - id: BatchResult.all_warnings
          type: FunctionDef
          code: |-
            @final
            def all_warnings(self) -> list[str]:
                return list(dict.fromkeys((*self.warnings, *self.derived_warnings())))
      - id: ToolPayload
        type: ClassDef
        excert: "@dataclass(frozen=True, kw_only=True) class ToolPayload(ABC):"
        docstring: Typed root payload of a tool result; the only way to fill ``structured_content`…
        children:
        - id: ToolPayload.Jdu7i7|diAzsp
          type: statements
          code: '''Typed root payload of a tool result; the only way to fill ``structured_content``.'''
        - id: ToolPayload.to_dict
          type: FunctionDef
          code: |-
            @abstractmethod
            def to_dict(self) -> dict[str, Any]:
                """Wire form of the payload (without root warnings)."""
        - id: ToolPayload.all_warnings
          type: FunctionDef
          code: |-
            def all_warnings(self) -> list[str]:
                """Call-wide warnings that travel at the root of the tool result."""
                return []
        - id: ToolPayload.failed
          type: FunctionDef
          code: |-
            @property
            def failed(self) -> bool:
                """Whether part of the call failed, which withholds automatic approval."""
                return False
      - id: RawPayload
        type: ClassDef
        excert: "@dataclass(frozen=True) class RawPayload(ToolPayload):"
        docstring: Untyped payload whose shape is fixed by someone else; *reason* documents why.
        children:
        - id: RawPayload.gqXWS3|wMnazL
          type: statements
          code: |-
            'Untyped payload whose shape is fixed by someone else; *reason* documents why.'
            data: dict[str, Any]
            reason: str
            warnings: tuple[str, ...] = ()
        - id: RawPayload.post_init
          type: FunctionDef
          code: |-
            def __post_init__(self) -> None:
                if not self.reason:
                    raise ValueError('RawPayload requires a reason for the deviation')
        - id: RawPayload.to_dict
          type: FunctionDef
          code: |-
            def to_dict(self) -> dict[str, Any]:
                return dict(self.data)
        - id: RawPayload.all_warnings
          type: FunctionDef
          code: |-
            def all_warnings(self) -> list[str]:
                return list(dict.fromkeys(self.warnings))
```
/answer 4f11-c-b allow
Control Request:
```yaml
id: 4f11-d-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
  regexp: .
  reason: Read the package __init__ for registration
```
/answer 4f11-d-0 allow
Control Request:
```yaml
id: 4f11-e-d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
      nodes:
      - id: 12csGL|fKey3N
        code: "'``ast_*`` tool family built on the standard-library ``ast`` module.\\n\\nA content-hash validated cache (:mod:`.core`) holds parsed modules; comments are\\nconverted to standalone string-literal annotations on import so they survive the\\n``parse``/``unparse`` round-trip. Retrieval is layered on a\\nsingle ``outline`` tree (``ast_outline`` structure, ``ast_find`` property/text/regexp\\nfiltering with source, ``ast_read`` reads subtrees by id); mutation is\\nnode-level CRUD, each tool in its own ``*`` module (``ast_create``/``ast_delete``\\ncover the whole-file case too), with two in-node editors ``ast_edit_marks``\\n(marker-delimited) and ``ast_edit_block`` (exact block), a restricted ``script``\\nand a ``validate`` compile check.\\n'"
      - id: ukjPdF|1qq8iY
        code: |-
          from xy.ai.mcpc.tools.tool_registry import ToolRegistry
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
          from xy.ai.mcpc.tools.ast import create, delete, edit_block, edit_marks, find, insert, read, replace, script, validate
          from xy.ai.mcpc.tools.ast import outline
      - id: mADxqb|1vgpZd
        code: |-
          __all__ = ['register_ast_tools', 'ALIAS']
          '#: Alias name that activates the whole family in one go.'
          ALIAS = 'ast'
          _ALIAS_MEMBERS = (
              'ast_outline',
              'ast_find',
              'ast_read',
              'ast_insert',
              'ast_edit_marks',
              'ast_edit_block',
              'ast_replace',
              'ast_delete',
              'ast_create',
              'ast_validate')
          ALIAS_READ = 'ast-read'
          _ALIAS_READ_MEMBERS = ('ast_outline', 'ast_find', 'ast_read')
      - id: register_ast_tools
        code: |-
          def register_ast_tools(registry: ToolRegistry, functions: FunctionRegistry) -> None:
              """Register every ``ast_*`` tool and the ``ast`` alias."""
              outline.register(registry, functions)
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
              registry.register_alias(ALIAS_READ, _ALIAS_READ_MEMBERS)
```
/answer 4f11-e-d allow
ReasoningToken: 397
Control Request:
```yaml
id: 4f11-f-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: ^class (ToolResult|ToolContext|ToolDefinition)\b
    include:
    - '*.py'
  reason: Locate ToolResult/ToolContext definitions
```
/answer 4f11-f-0 allow
Control Request:
```yaml
id: 4f11-10-f
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
      matches:
      - path: tool_context.py
        matches:
        - "class ToolContext:"
      - path: tool_registry.py
        matches:
        - "class ToolResult:"
        - "class ToolDefinition(ABC):"
```
/answer 4f11-10-f allow
Control Request:
```yaml
id: 4f11-11-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_context.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
  node_type: ClassDef
  regexp: class (ToolResult|ToolContext|ToolDefinition)\b
  reason: "Read ToolResult, ToolContext and ToolDefinition to design the merged handler"
```
/answer 4f11-11-0 allow
Control Request:
```yaml
id: 4f11-12-11
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_context.py
      nodes:
      - id: ToolContext
        type: ClassDef
        code: |-
          @dataclass(slots=True)
          class ToolContext:
              """Context handed to a tool handler on invocation.
              """
              session: 'Session'
              arguments: dict[str, Any]
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
              message: str | None = None
              warnings: list[str] = field(default_factory=list)

              def __post_init__(self) -> None:
                  if self.is_error and (self.structured_content or self.warnings):
                      raise ValueError('ToolResult: error results carry neither structured_content nor warnings.')
                  if self.structured_content and RESERVED_ROOT_KEYS & self.structured_content.keys():
                      raise ValueError(
                          'ToolResult: reserved key in structured_content; use ToolResult.warnings / ToolResult.message.')

              @classmethod
              def ok(cls, payload: ToolPayload, *, auto_approve: bool=False, content: list[dict[str, Any]] | None=None, structured: dict[str, Any] | None=None) -> 'ToolResult':
                  """Wrap a typed payload; *structured* replaces its wire form for context-dependent rendering."""
                  return cls(
                      content=list(
                          content or []),
                      structured_content=payload.to_dict() if structured is None else structured,
                      warnings=payload.all_warnings(),
                      auto_approve=auto_approve and (
                          not payload.failed))

              @classmethod
              def failure(cls, message: str) -> 'ToolResult':
                  return cls(content=[text_content(message)], is_error=True)

              def to_dict(self) -> dict[str, Any]:
                  result: dict[str, Any] = {}
                  if self.content:
                      result['content'] = list(self.content)
                  if self.structured_content or self.warnings:
                      structured = dict(self.structured_content or {})
                      if self.warnings:
                          structured[WARNINGS_KEY] = list(self.warnings)
                      result['structuredContent'] = structured
                  if self.message:
                      attach_message(result, self.message)
                  if self.is_error:
                      result['isError'] = True
                  return result
      - id: ToolDefinition
        type: ClassDef
        code: |-
          class ToolDefinition(ABC):
              """Base class for registering a tool as an object.

              Subclasses declare the MCP metadata as class attributes and implement
              :meth:`handle`; an instance is itself a callable :data:`ToolHandler` and
              can be passed directly to :meth:`ToolRegistry.register`::

                  class MyTool(ToolDefinition):
                      name = "my_tool"
                      description = "..."
                      input_schema = {...}

                      def handle(self, ctx: ToolContext) -> ToolResult:
                          ...

                  registry.register(MyTool())
              """
              name: str
              description: str
              input_schema: dict[str, Any]
              title: str | None = None
              output_schema: dict[str, Any] | None = {'type': 'object'}
              annotations: dict[str, Any] | None = None

              @abstractmethod
              def handle(self, ctx: ToolContext) -> 'ToolResult':
                  ...

              def __call__(self, ctx: ToolContext) -> 'ToolResult':
                  return self.handle(ctx)

              def to_tool(self) -> Tool:
                  return Tool(
                      name=self.name,
                      description=self.description,
                      input_schema=self.input_schema,
                      handler=self,
                      title=self.title,
                      output_schema=self.output_schema,
                      annotations=self.annotations)
```
/answer 4f11-12-11 allow
ReasoningToken: 584
Control Request:
```yaml
id: 4f11-13-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit.py
    source: |
      """``ast_edit`` tool: all modifying AST tools in a single call."""
      from __future__ import annotations

      from typing import Any

      from xy.ai.mcpc.tools.ast.create import CreateFileTool
      from xy.ai.mcpc.tools.ast.delete import DeleteTool
      from xy.ai.mcpc.tools.ast.edit_block import EditBlockNodeTool
      from xy.ai.mcpc.tools.ast.edit_marks import EditMarksNodeTool
      from xy.ai.mcpc.tools.ast.insert import InsertNodeTool
      from xy.ai.mcpc.tools.ast.replace import ReplaceNodeTool
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult

      __all__ = ['EditTool', 'register']

      # Execution order: files must exist before edits, deletions come last.
      _STEPS: tuple[tuple[str, ToolDefinition], ...] = (
          ('create', CreateFileTool()),
          ('insert', InsertNodeTool()),
          ('replace', ReplaceNodeTool()),
          ('edit_block', EditBlockNodeTool()),
          ('edit_marks', EditMarksNodeTool()),
          ('delete', DeleteTool()),
      )


      def _error_text(result: ToolResult) -> str:
          return ' '.join(str(c.get('text', '')) for c in result.content) or 'Unknown error'


      class EditTool(ToolDefinition):
          name = 'ast_edit'
          title = 'Batch of AST modifications'
          description = (
              'Run create, insert, replace, edit_block, edit_marks and delete operations in one call. '
              'Each list has the item format of the tool ast_<list name>. '
              'Lists are processed in the order: create, insert, replace, edit_block, edit_marks, delete; '
              'the result contains one section per used list.')
          input_schema = {
              'type': 'object',
              'properties': {key: tool.input_schema['properties']['items'] for key, tool in _STEPS},
              'additionalProperties': False,
          }

          def handle(self, ctx: ToolContext) -> ToolResult:
              sections: dict[str, Any] = {}
              warnings: list[str] = []
              approve = True
              for key, tool in _STEPS:
                  items = ctx.arguments.get(key)
                  if not items:
                      continue
                  sub = tool.handle(ToolContext(session=ctx.session, arguments={'items': items}))
                  if sub.is_error:
                      sections[key] = {'error': _error_text(sub)}
                      approve = False
                      continue
                  sections[key] = sub.structured_content or {}
                  warnings.extend(sub.warnings)
                  approve = approve and sub.auto_approve
              if not sections:
                  return ToolResult.failure('At least one non-empty operation list is required.')
              return ToolResult(
                  structured_content=sections,
                  warnings=list(dict.fromkeys(warnings)),
                  auto_approve=approve)


      def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
          registry.register(EditTool())
  reason: Create the new ast_edit tool module
```
/answer 4f11-13-0 allow
ReasoningToken: 19
Control Request:
```yaml
id: 4f11-14-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
    id: ukjPdF|1qq8iY
    old_text: "from xy.ai.mcpc.tools.ast import create, delete, edit_block,"
    new_text: "from xy.ai.mcpc.tools.ast import create, delete, edit, edit_block,"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
    id: mADxqb|1vgpZd
    old_text: |2-
          'ast_create',
          'ast_validate')
      ALIAS_READ
    new_text: |2-
          'ast_create',
          'ast_edit',
          'ast_validate')
      ALIAS_READ
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
    id: register_ast_tools
    old_text: |2
          create.register(registry, functions)
    new_text: |2
          create.register(registry, functions)
          edit.register(registry, functions)
  reason: Register ast_edit in the ast package and alias
```
/answer 4f11-14-0 allow
Control Request:
```yaml
id: 4f11-15-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
  reason: Verify both files compile
```
/answer 4f11-15-0 allow
Control Request:
```yaml
id: 4f11-16-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: input_schema =
    include:
    - edit_marks.py
    - insert.py
  reason: Confirm input_schema exists on edit_marks and insert tools
```
/answer 4f11-16-0 allow
Control Request:
```yaml
id: 4f11-17-16
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches:
      - path: edit_marks.py
        matches:
        - "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)"
      - path: insert.py
        matches:
        - "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)"
```
/answer 4f11-17-16 allow
Result Stats: id=4f6aaf16-dce4-444d-9cb3-c6d387895bd3, total: 46764, in: 20, out: 7840, reason: 0, read: 239742, write: 38904, start: 1794954546707, end: 1794954552129, time: 5380, total: 55860, io: 0, control: 42, sent: 0, recv: 142
Ich habe das neue Tool `ast_edit` in `tools/ast/edit.py` angelegt und registriert. Beide Dateien lassen sich kompilieren. Ausgeführt oder getestet habe ich es nicht.

**Eingabe:** Es gibt sechs optionale, typisierte Listen: `create`, `insert`, `replace`, `edit_block`, `edit_marks` und `delete`. Das Schema jeder Liste wird zur Importzeit aus `<Tool>.input_schema['properties']['items']` des jeweiligen Edit-Tools übernommen. Es gibt kein OneOf/AnyOf, und Schemaänderungen in den Einzeltools wirken automatisch im Batch-Tool. Die sechs Edit-Tools selbst habe ich nicht angefasst.

**Ausführung:** Die Listen laufen in fester Reihenfolge: create, insert, replace, edit_block, edit_marks, delete. Leere oder fehlende Listen werden übersprungen. Jede Liste läuft über `handle()` des bestehenden Tools, die Logik wird also nicht doppelt gepflegt.

**Ergebnis:** Die Antwort hat einen Abschnitt pro verwendeter Liste, zum Beispiel `{"create": {...}, "replace": {...}}`. Die Warnings aller Abschnitte werden zusammengeführt. Die automatische Freigabe gilt nur, wenn alle Teilaufrufe freigegeben sind und keiner fehlgeschlagen ist. Schlägt ein ganzer Teilaufruf fehl, steht in seinem Abschnitt `{"error": ...}`, und die übrigen Listen laufen trotzdem weiter. Sind alle Listen leer, ist das ein Fehler.

**Registrierung:** In `tools/ast/__init__.py` ist `edit` importiert und registriert, und `ast_edit` gehört zu den Alias-Mitgliedern von `ast`. Es gibt keine Funktions-API (`functions.register`), weil es nur an die bestehenden Tools delegiert.
