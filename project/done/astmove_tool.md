Implementiere analog zu den anderen AST Tool in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast` ein Tool "ast_move".

Dieses Move-Tool soll Knoten zwischen Dateien (insert & delete) oder ganze Dateien verschieben (create & delete).

- Eingabe sind ein absoluter Quell und ein absoluter Ziel Dateipfad
	- Ziel und Quelle können identisch sein dann wird innerhalb der Datei bewegt
	- Eine nicht existierende Zieldatei wird wie bei Create erzeugt
- sowie eine Liste von Quell ID's; ist diese Leer ist die ganze Datei die Quelle
	- Eine einzelne nicht gefundene ID erzeugt einen Fehler aber verhindert nicht das Bewegen gefundener Knoten
	- Beispiel: Um eine innere Klasse zu bewegen muss ihre ID verwendet werden. Werden nur alle Kinder genannt bleibt die Klasse selbst erhalten.
- sowie eine Ziel-ID und ein Insertion mode (Optional After/append am Ende der Zieldatei)
	- Bei einer nicht vorhandenen Ziel-ID wird das Item abgebrochen
	
- Wird eine Quelldatei durch das Verändern Leer so wird sie entfernt
- Import handling wird unterstützt
- Es soll ebenfalls einen Batch Support bekommen
- Es soll ebenfalls in`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit.py` verwendbar sein
- Zurückgegeben wird der Name der Zieldatei und eine Liste der verschobenen Node ID's (Alte ID  zu neue ID, diese können nun geändert sein), jedoch keine vollständige Outline.

`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py`
SystemInit:  id=71fc1cac-d49d-4728-bb65-73d1d1769780, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-opus-5-5
Text:
 Ich schaue mir zunächst die Struktur und die bestehenden AST-Tools an.
ReasoningToken: 16
Control Request:
```yaml
id: 1f37-1ea-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
  reason: "Überblick über die vorhandenen AST-Tool-Dateien, um ast_move analog zu implementieren"
```
/answer 1f37-1ea-0 allow
Control Request:
```yaml
id: 1f37-1eb-1ea
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      entries:
      - ".:"
      - __init__.py
      - base.py
      - common.py
      - core.py
      - create.py
      - delete.py
      - edit.py
      - edit_block.py
      - edit_marks.py
      - find.py
      - imports.py
      - insert.py
      - outcomes.py
      - outline.py
      - read.py
      - replace.py
      - retrieve.py
      - script.py
      - validate.py
      - "./generic:"
      - __init__.py
      - _css.py
      - _engine.py
      - _html.py
      - _java.py
      - _javascript.py
      - _markdown.py
      - _text.py
      - _xml.py
      - _yaml.py
      - "./python:"
      - __init__.py
      - _comments.py
      - _engine.py
      - _imports.py
      - _nodes.py
      - reformat_cli.py
      - "./rpc:"
      - __init__.py
      - _engine.py
      - _java.py
      - _markdown.py
      - _process.py
      - _rust.py
```
/answer 1f37-1eb-1ea allow
Control Request:
```yaml
id: 1f37-1ec-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit.py
  reason: "Vollständigen Quelltext von create/delete/insert/edit lesen, um Struktur und Konventionen für ast_move zu übernehmen"
```
/answer 1f37-1ec-0 allow
Control Request:
```yaml
id: 1f37-1ed-1ec
result:
  structuredContent:
    outline:
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
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
        nodes:
        - id: 9CmN9x|nHobTc
          excert: "'``ast_insert`` tool: insert statement(s) relative to selected nodes.'"
        - id: 8Wm4pY|RoeKEJ
          excert: from dataclasses import dataclass
        - id: indDlk|VBU4wK
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
          - id: InsertNodeTool.098XR3|mb1vJU
            excert: name = 'ast_insert'
          - id: InsertNodeTool.c0C4qn|KsRfPw
            excert: "_ITEM_PROPERTIES = {"
          - id: InsertNodeTool.ei5nIo|Lu9IT0
            excert: "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTIO…"
          - id: InsertNodeTool.handle
            excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
            docstring: "Delegate to :func:`ast_insert`, translating the MCP schema to/from the AST API."
        - id: register
          excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
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
```
/answer 1f37-1ed-1ec allow
ReasoningToken: 57
Control Request:
```yaml
id: 1f37-1ee-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit.py
  reason: Vollständigen Quelltext der vier Referenzdateien lesen
```
/answer 1f37-1ee-0 allow
Control Request:
```yaml
id: 1f37-1ef-1ee
result:
  structuredContent:
    read:
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
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py instead."
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
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py instead."
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
        nodes:
        - id: 9CmN9x|nHobTc
          excert: "'``ast_insert`` tool: insert statement(s) relative to selected nodes.'"
        - id: 8Wm4pY|RoeKEJ
          excert: from dataclasses import dataclass
        - id: indDlk|VBU4wK
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
          - id: InsertNodeTool.098XR3|mb1vJU
            excert: name = 'ast_insert'
          - id: InsertNodeTool.c0C4qn|KsRfPw
            excert: "_ITEM_PROPERTIES = {"
          - id: InsertNodeTool.ei5nIo|Lu9IT0
            excert: "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTIO…"
          - id: InsertNodeTool.handle
            excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
            docstring: "Delegate to :func:`ast_insert`, translating the MCP schema to/from the AST API."
        - id: register
          excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py instead."
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
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit.py instead."
```
/answer 1f37-1ef-1ee allow
ReasoningToken: 82
Control Request:
```yaml
id: 1f37-1f0-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    ids:
    - CpezhJ|nJnt4G
    - 8Wm4pY|RoeKEJ
    - qzaxoR|HxGFbQ
    - zTddKa|ZtwRn0
    - DeleteItem
    - DeleteResult
    - DeleteError
    - DeleteBatchResult
    - delete_one
    - ast_delete
    - DeleteTool
    - register
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    ids:
    - 9CmN9x|nHobTc
    - 8Wm4pY|RoeKEJ
    - indDlk|VBU4wK
    - oTjM4y|Jut0OL
    - InsertItem
    - InsertResult
    - InsertError
    - InsertBatchResult
    - insert_one
    - ast_insert
    - InsertNodeTool
    - register
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
    ids:
    - ZAIFPk|rsb8Xe
    - 8zfkq5|b8BrXU
    - wtzT47|EP6uG9
    - CreateFileItem
    - CreateFileResult
    - CreateFileError
    - CreateFilesResult
    - create_one
    - ast_create
    - CreateFileTool
    - register
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit.py
    ids:
    - 5ZtN89|XTSQEh
    - neazUj|4NSldY
    - indDlk|k9bRxt
    - SectionError
    - 14h53u|AeVQqA
    - error_text
    - EditTool
    - register
  reason: Vollständigen Quelltext aller Top-Level-Knoten der Referenzdateien lesen
```
/answer 1f37-1f0-0 allow
Control Request:
```yaml
id: 1f37-1f1-1f0
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
        nodes:
        - id: CpezhJ|nJnt4G
          type: statements
          code: "'``ast_delete`` tool: delete selected nodes, or whole files if none selected.'"
        - id: 8Wm4pY|RoeKEJ
          type: imports
          code: |-
            from dataclasses import dataclass
            from xy.ai.mcpc.tools.ast.imports import ImportCheckedBatchResult, ImportOutcome, parse_import_args
            from xy.ai.mcpc.tools.ast.outcomes import NodeError, edit_payload, node_key
            from xy.ai.mcpc.tools.results import ItemResult
            from typing import Any
            from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
            from xy.ai.mcpc.tools.tool_context import ToolContext
            from xy.ai.mcpc.tools.ast import core
        - id: qzaxoR|HxGFbQ
          type: imports
          code: |-
            from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_path
            from xy.ai.mcpc.tools._tool_helpers import batch_schema
            from xy.ai.mcpc.tools.function_registry import FunctionRegistry
            from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
        - id: zTddKa|ZtwRn0
          type: statements
          code: "__all__ = ['DeleteItem', 'DeleteResult', 'DeleteError', 'DeleteBatchResult', 'ast_delete', 'DeleteTool', 'register']"
        - id: DeleteItem
          type: ClassDef
          code: |-
            @dataclass(frozen=True)
            class DeleteItem:
                """One node (or file) to delete.

                Attributes:
                    path: Absolute path to the file to modify.
                    id: Unique id of the target node. Omit to delete the whole file.
                    add_import: Import statements to add if missing, applied after the delete.
                    remove_import: Import statements to remove if present, applied after the delete.
                        Both are ignored (and flagged in the batch warnings) when a whole file is deleted.
                """
                path: str
                id: str | None = None
                add_import: list[str] | None = None
                remove_import: list[str] | None = None
        - id: DeleteResult
          type: ClassDef
          excert: "@dataclass(frozen=True) class DeleteResult(ItemResult):"
          docstring: "Result of deleting a single node or file. Attributes: path: The path exactly as…"
          children:
          - id: DeleteResult.brBMR9|fakDck
            type: statements
            code: |-
              'Result of deleting a single node or file.\n\n    Attributes:\n        path: The path exactly as given in the input, for result association.\n        id: The id exactly as given in the input, for result association.\n    '
              path: str
              id: str | None
              imports: ImportOutcome | None = None
          - id: DeleteResult.key
            type: FunctionDef
            code: |-
              def key(self) -> dict[str, Any]:
                  return node_key(self.path, self.id)
          - id: DeleteResult.payload
            type: FunctionDef
            code: |-
              def payload(self) -> dict[str, Any]:
                  return edit_payload(imports=self.imports)
        - id: DeleteError
          type: ClassDef
          code: |-
            class DeleteError(NodeError):
                """Error deleting a single node or file.

                Attributes:
                    path: The path exactly as given in the input, for result association.
                    id: The id exactly as given in the input, for result association.
                    error: The error message.
                """
        - id: DeleteBatchResult
          type: ClassDef
          code: |-
            class DeleteBatchResult(ImportCheckedBatchResult[DeleteResult, DeleteError]):
                """Result of :func:`ast_delete`.

                Attributes:
                    results: One :class:`DeleteResult` per successfully deleted item.
                    errors: One :class:`DeleteError` per item that failed.
                """
        - id: delete_one
          type: FunctionDef
          code: |-
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
                    unchecked = ImportOutcome(unchecked=True) if item.add_import or item.remove_import else None
                    return DeleteResult(path=item.path, id=None, imports=unchecked)
                tree = core.CACHE.get_tree(file_path)
                target = select_by_path(tree, id=item.id)
                core.delete_node(target)
                imports = core.apply_imports(tree, item.add_import, item.remove_import)
                core.CACHE.save(file_path, tree)
                return DeleteResult(path=item.path, id=item.id, imports=imports)
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
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
        nodes:
        - id: 9CmN9x|nHobTc
          type: statements
          code: "'``ast_insert`` tool: insert statement(s) relative to selected nodes.'"
        - id: 8Wm4pY|RoeKEJ
          type: imports
          code: |-
            from dataclasses import dataclass
            from xy.ai.mcpc.tools.ast.imports import ImportCheckedBatchResult, ImportOutcome, parse_import_args
            from xy.ai.mcpc.tools.ast.outcomes import NodeError, edit_payload, node_key
            from xy.ai.mcpc.tools.results import ItemResult
            from typing import Any
            from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
            from xy.ai.mcpc.tools.tool_context import ToolContext
            from xy.ai.mcpc.tools.ast import core
        - id: indDlk|VBU4wK
          type: imports
          code: |-
            from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_path
            from xy.ai.mcpc.tools._tool_helpers import batch_schema
            from xy.ai.mcpc.tools.function_registry import FunctionRegistry
            from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
            from xy.ai.mcpc.tools.tool_registry import ToolFeature
        - id: oTjM4y|Jut0OL
          type: statements
          code: "__all__ = ['InsertItem', 'InsertResult', 'InsertError', 'InsertBatchResult', 'ast_insert', 'InsertNodeTool', 'register']"
        - id: InsertItem
          type: ClassDef
          code: |-
            @dataclass(frozen=True)
            class InsertItem:
                """One insert operation.

                Attributes:
                    path: Absolute path to the file to modify.
                    source: Source of the statement(s) to insert.
                    position: ``"before"`` or ``"after"`` the selected node. Defaults to ``"after"``.
                    id: Unique id of the target node.
                    add_import: Import statements to add if missing, applied after the insert.
                    remove_import: Import statements to remove if present, applied after the insert.
                """
                path: str
                source: str
                id: str
                position: str = 'after'
                add_import: list[str] | None = None
                remove_import: list[str] | None = None
        - id: InsertResult
          type: ClassDef
          excert: "@dataclass(frozen=True) class InsertResult(ItemResult):"
          docstring: "Result of a single insert operation. Attributes: path: The path exactly as give…"
          children:
          - id: InsertResult.aUv9Tr|y2n1TA
            type: statements
            code: |-
              "Result of a single insert operation.\n\n    Attributes:\n        path: The path exactly as given in the input, for result association.\n        id: The id exactly as given in the input, for result association.\n        inserted: Number of top-level statements parsed from ``source`` and inserted.\n        ids: The newly inserted top-level node(s)' ids.\n    "
              path: str
              id: str | None
              inserted: int
              ids: list[str] | None = None
              imports: ImportOutcome | None = None
          - id: InsertResult.key
            type: FunctionDef
            code: |-
              def key(self) -> dict[str, Any]:
                  return node_key(self.path, self.id)
          - id: InsertResult.payload
            type: FunctionDef
            code: |-
              def payload(self) -> dict[str, Any]:
                  return edit_payload(imports=self.imports, inserted=self.inserted, ids=self.ids)
        - id: InsertError
          type: ClassDef
          code: |-
            class InsertError(NodeError):
                """Error applying a single insert operation.

                Attributes:
                    path: The path exactly as given in the input, for result association.
                    id: The id exactly as given in the input, for result association.
                    error: The error message.
                """
        - id: InsertBatchResult
          type: ClassDef
          code: |-
            class InsertBatchResult(ImportCheckedBatchResult[InsertResult, InsertError]):
                """Result of :func:`ast_insert`.

                Attributes:
                    results: One :class:`InsertResult` per successful insert.
                    errors: One :class:`InsertError` per insert that failed.
                """
        - id: insert_one
          type: FunctionDef
          code: |-
            def _insert_one(item: InsertItem) -> InsertResult:
                file_path = core.require_path(item.path)
                tree = core.CACHE.get_tree(file_path)
                target = select_by_path(tree, id=item.id)
                before_ids = {loc.node_id for loc in core.locate_all(tree)}
                inserted = core.insert_node(target, item.source, item.position)
                new_ids = [loc.node_id for loc in core.locate_all(tree) if loc.node_id not in before_ids]
                imports = core.apply_imports(tree, item.add_import, item.remove_import)
                core.CACHE.save(file_path, tree)
                return InsertResult(path=item.path, id=item.id, inserted=inserted, ids=new_ids or None, imports=imports)
        - id: ast_insert
          type: FunctionDef
          code: |-
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
        - id: InsertNodeTool
          type: ClassDef
          excert: "class InsertNodeTool(ToolDefinition):"
          children:
          - id: InsertNodeTool.098XR3|mb1vJU
            type: statements
            code: |-
              name = 'ast_insert'
              features = frozenset({ToolFeature.INPUT_ASTID})
              title = 'Insert AST nodes'
              description = "Insert source relative to selected nodes ('before' or 'after'), for a batch of items; several operations may target the same or different files."
          - id: InsertNodeTool.c0C4qn|KsRfPw
            type: statements
            code: |-
              _ITEM_PROPERTIES = {
                  'path': PATH_PROP,
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
                  **PATH_SELECTOR_PROPS,
                  **IMPORT_PROPS}
              _ITEM_REQUIRED = ['path', 'source', 'id']
              _ITEMS_DESCRIPTION = 'Insert operations to apply.'
          - id: InsertNodeTool.ei5nIo|Lu9IT0
            type: statements
            code: "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)"
          - id: InsertNodeTool.handle
            type: FunctionDef
            code: |-
              def handle(self, ctx: ToolContext) -> ToolResult:
                  """Delegate to :func:`ast_insert`, translating the MCP schema to/from the AST API."""

                  def item_factory(it: dict[str, Any]) -> InsertItem:
                      add_import, remove_import = parse_import_args(it)
                      return InsertItem(
                          path=it['path'],
                          source=it['source'],
                          position=it.get(
                              'position',
                              'after'),
                          id=it['id'],
                          add_import=add_import,
                          remove_import=remove_import)
                  return handle_batch_tool(ctx, item_factory, ast_insert, core.AstError, auto_approve=True)
        - id: register
          type: FunctionDef
          code: |-
            def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
                registry.register(InsertNodeTool())
                functions.register(ast_insert)
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
        nodes:
        - id: ZAIFPk|rsb8Xe
          type: statements
          code: "'``ast_create`` tool: create one or more files from source, creating missing directories.'"
        - id: 8zfkq5|b8BrXU
          type: imports
          code: |-
            from dataclasses import dataclass
            from typing import Any
            from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
            from xy.ai.mcpc.tools.tool_context import ToolContext
            from xy.ai.mcpc.tools.ast import core
            from xy.ai.mcpc.tools._tool_helpers import batch_schema
            from xy.ai.mcpc.tools.function_registry import FunctionRegistry
            from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
            from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult
        - id: wtzT47|EP6uG9
          type: statements
          code: |-
            __all__ = [
                'CreateFileItem',
                'CreateFileResult',
                'CreateFileError',
                'CreateFilesResult',
                'ast_create',
                'CreateFileTool',
                'register']
        - id: CreateFileItem
          type: ClassDef
          code: |-
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
        - id: CreateFileResult
          type: ClassDef
          excert: "@dataclass(frozen=True) class CreateFileResult(ItemResult):"
          docstring: "Result of creating a single file. Attributes: path: The path exactly as given i…"
          children:
          - id: CreateFileResult.ck54as|jvxqvL
            type: statements
            code: |-
              'Result of creating a single file.\n\n    Attributes:\n        path: The path exactly as given in the input, for result association.\n    '
              path: str
          - id: CreateFileResult.key
            type: FunctionDef
            code: |-
              def key(self) -> dict[str, Any]:
                  return {'path': self.path}
        - id: CreateFileError
          type: ClassDef
          excert: "@dataclass(frozen=True) class CreateFileError(ItemError):"
          docstring: "Error creating a single file. Attributes: path: The path exactly as given in th…"
          children:
          - id: CreateFileError.z9L1ur|oLYcZT
            type: statements
            code: |-
              'Error creating a single file.\n\n    Attributes:\n        path: The path exactly as given in the input, for result association.\n        error: The error message.\n    '
              path: str
          - id: CreateFileError.key
            type: FunctionDef
            code: |-
              def key(self) -> dict[str, Any]:
                  return {'path': self.path}
        - id: CreateFilesResult
          type: ClassDef
          code: |-
            class CreateFilesResult(BatchResult[CreateFileResult, CreateFileError]):
                """Result of :func:`ast_create`.

                Attributes:
                    results: One :class:`CreateFileResult` per file created successfully.
                    errors: One :class:`CreateFileError` per file that failed.
                """
        - id: create_one
          type: FunctionDef
          code: |-
            def _create_one(item: CreateFileItem) -> CreateFileResult:
                file_path = core.require_path(item.path, must_exist=False)
                if file_path.exists() and (not item.overwrite):
                    raise core.AstError('File already exists.')
                tree = core.parse_for(item.path, item.source)
                file_path.parent.mkdir(parents=True, exist_ok=True)
                core.CACHE.save(file_path, tree)
                return CreateFileResult(path=item.path)
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
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit.py
        nodes:
        - id: 5ZtN89|XTSQEh
          type: statements
          code: "'``ast_edit`` tool: all modifying AST tools in a single call.'"
        - id: neazUj|4NSldY
          type: imports
          code: |-
            from __future__ import annotations
            from dataclasses import dataclass
            from typing import Any
            from xy.ai.mcpc.tools.ast.create import CreateFileTool
            from xy.ai.mcpc.tools.ast.delete import DeleteTool
            from xy.ai.mcpc.tools.ast.edit_block import EditBlockNodeTool
            from xy.ai.mcpc.tools.ast.edit_marks import EditMarksNodeTool
            from xy.ai.mcpc.tools.ast.insert import InsertNodeTool
            from xy.ai.mcpc.tools.ast.replace import ReplaceNodeTool
            from xy.ai.mcpc.tools.function_registry import FunctionRegistry
        - id: indDlk|k9bRxt
          type: imports
          code: |-
            from xy.ai.mcpc.tools.tool_context import ToolContext
            from xy.ai.mcpc.tools.results import ItemError, RawPayload
            from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
            from xy.ai.mcpc.tools.tool_registry import ToolFeature
        - id: SectionError
          type: ClassDef
          excert: "@dataclass(frozen=True, kw_only=True) class _SectionError(ItemError):"
          children:
          - id: SectionError.key
            type: FunctionDef
            code: |-
              def key(self) -> dict[str, Any]:
                  return {}
        - id: 14h53u|AeVQqA
          type: statements
          code: |-
            __all__ = ['EditTool', 'register']
            '# Execution order: files must exist before edits, deletions come last.'
            _STEPS: tuple[tuple[str, ToolDefinition], ...] = (('create', CreateFileTool()), ('insert', InsertNodeTool(
            )), ('replace', ReplaceNodeTool()), ('edit_block', EditBlockNodeTool()), ('edit_marks', EditMarksNodeTool()), ('delete', DeleteTool()))
        - id: error_text
          type: FunctionDef
          code: |-
            def _error_text(result: ToolResult) -> str:
                return ' '.join((str(c.get('text', '')) for c in result.content)) or 'Unknown error'
        - id: EditTool
          type: ClassDef
          excert: "class EditTool(ToolDefinition):"
          children:
          - id: EditTool.oy2a69|JHgl3f
            type: statements
            code: |-
              name = 'ast_edit'
              features = frozenset({ToolFeature.INPUT_ASTID})
              title = 'Batch of AST modifications'
              description = 'Run create, insert, replace, edit_block, edit_marks and delete operations in one call. Each list has the item format of the tool ast_<list name>.'
              input_schema = {
                  'type': 'object',
                  'properties': {
                      key: tool.input_schema['properties']['items'] for key,
                      tool in _STEPS},
                  'additionalProperties': False}
          - id: EditTool.handle
            type: FunctionDef
            code: |-
              def handle(self, ctx: ToolContext) -> ToolResult:
                  sections: dict[str, Any] = {}
                  warnings: list[str] = []
                  approve = True
                  for key, tool in _STEPS:
                      items = ctx.arguments.get(key)
                      if not items:
                          continue
                      sub = tool.handle(ToolContext(session=ctx.session, arguments={'items': items}, registry=ctx.registry))
                      if sub.is_error:
                          sections[key] = _SectionError(error=_error_text(sub)).to_dict()
                          approve = False
                          continue
                      sections[key] = sub.structured_content or {}
                      warnings.extend(sub.warnings)
                      approve = approve and sub.auto_approve
                  if not sections:
                      return ToolResult.failure('At least one non-empty operation list is required.')
                  payload = RawPayload(
                      sections,
                      reason='composes the payloads of the individual edit tools',
                      warnings=tuple(warnings))
                  return ToolResult.ok(payload, auto_approve=approve)
        - id: register
          type: FunctionDef
          code: |-
            def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
                registry.register(EditTool())
```
/answer 1f37-1f1-1f0 allow
ReasoningToken: 113
Control Request:
```yaml
id: 1f37-1f2-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outcomes.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  reason: "API von core/common/imports/outcomes/base verstehen (delete_node, insert_node, locate_all, CACHE, apply_imports, select_by_path)"
```
/answer 1f37-1f2-0 allow
Control Request:
```yaml
id: 1f37-1f3-1f2
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
        nodes:
        - id: V6fdSR|z5Q7Tq
          excert: "'Facade the ``ast_*`` tools call, dispatching to a per-file engine.\\n\\nThe engi…"
        - id: OsZYRc|1ZfHhi
          excert: from __future__ import annotations
        - id: 64y9R1|MAcfZT
          excert: from xy.ai.mcpc.tools.ast import python
        - id: fr6KXO|uhDdPM
          excert: logger = logging.getLogger(__name__)
        - id: engine_for_path
          excert: "def engine_for_path(path: Path) -> Engine:"
          docstring: Return the engine responsible for ``path`` based on its extension.
        - id: CacheEntry
          excert: "@dataclass class _CacheEntry:"
        - id: AstCache
          excert: "class AstCache:"
          docstring: Content-hash validated cache of parsed trees keyed by absolute path.
          children:
          - id: AstCache.AzRmHX|ayni5v
            excert: '''Content-hash validated cache of parsed trees keyed by absolute path.'''
          - id: AstCache.init
            excert: "def __init__(self) -> None:"
          - id: AstCache.get_tree
            excert: "def get_tree(self, path: Path) -> Tree:"
          - id: AstCache.save
            excert: "def save(self, path: Path, tree: Tree) -> str:"
            docstring: "Serialise *tree*, write it to *path* and refresh the cache entry."
          - id: AstCache.invalidate
            excert: "def invalidate(self, path: Path) -> None:"
        - id: dRq1FJ|5EG9eJ
          excert: "'#: Process-wide shared cache instance.'"
        - id: load
          excert: "def load(path_str: str) -> tuple[Path, Tree]:"
          docstring: Resolve *path_str* and return it together with its cached tree.
        - id: parse_source
          excert: "def parse_source(source: str, engine: Engine | None=None) -> Tree:"
          docstring: Parse *source* with *engine* (Python by default for path-less snippets).
        - id: parse_for
          excert: "def parse_for(path_str: str, code: str) -> Tree:"
          docstring: Parse *code* with the engine selected for *path_str*'s extension.
        - id: locate_all
          excert: "def locate_all(tree: Tree) -> list[Located]:"
        - id: edit_node_source
          excert: "def edit_node_source(loc: Located) -> str:"
        - id: relocate
          excert: "def relocate(tree: Tree, old: Located) -> Located | None:"
          docstring: Find the node now occupying ``old``'s former slot after an in-place edit. Match…
        - id: replace_node
          excert: "def replace_node(loc: Located, code: str) -> str | None:"
          docstring: Replace ``loc``'s node with ``code``; return its new id.
        - id: insert_node
          excert: "def insert_node(loc: Located, code: str, position: str) -> int:"
        - id: delete_node
          excert: "def delete_node(loc: Located) -> None:"
        - id: append_nodes
          excert: "def append_nodes(tree: Tree, code: str) -> int:"
        - id: apply_imports
          excert: "def apply_imports(tree: Tree, add: list[str] | None, remove: list[str] | None) -> ImportOutcome | None:"
          docstring: Apply an item's ``addImport``/``removeImport`` to the already edited ``tree``. …
        - id: empty_tree
          excert: "def empty_tree(path: Path) -> Tree:"
        - id: validate_source
          excert: "def validate_source(path: Path, source: str) -> str | None:"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
        nodes:
        - id: Q9NkRl|1c3Fpi
          excert: '''Selector machinery shared by the ``ast_*`` tools.\n\n``ast_find`` is the only …'
        - id: QUDAPO|6eecMb
          excert: from typing import Any
        - id: w0xyc1|24HH6w
          excert: "__all__ = ["
        - id: clnjIr|Jz72IC
          excert: "SELECTOR_PROPS = {"
        - id: RwEctf|qsWZ3v
          excert: "'#: Path-only selectors used by every mutation tool (replace/insert/delete/edit…"
        - id: YMszCp|e3vwZI
          excert: "IMPORT_PROPS = {"
        - id: select_one
          excert: "def select_one(tree, **selectors: Any) -> core.Located:"
          docstring: "Return the single node in *tree* matching *selectors*. Raises: core.AstError: I…"
        - id: select_by_path
          excert: "def select_by_path(tree, *, id: str | None=None) -> core.Located:"
          docstring: "Return the single node in *tree* addressed by its unique ``id``. Raises: core.A…"
        - id: select_by_text
          excert: "def select_by_text(tree, texts: list[str], *, id: str | None=None) -> core.Located:"
          docstring: "Return the node addressed by ``id``; if ``id`` is omitted, blank or unusable (s…"
        - id: select_by_contained_text
          excert: "def _select_by_contained_text(tree, texts: list[str]) -> core.Located:"
          docstring: Return the single innermost node whose source contains every string in *texts*.…
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
        nodes:
        - id: foxd99|7aiScK
          excert: '"Import/include/use handling shared by the ``ast_*`` edit tools.\n\nEvery edit …'
        - id: 1EBKpT|N6PPdc
          excert: from __future__ import annotations
        - id: AiyIX3|Ohci6H
          excert: "__all__ = ["
        - id: ImportOutcome
          excert: "@dataclass class ImportOutcome:"
          docstring: "What an engine did for one item's ``addImport``/``removeImport``. Attributes: a…"
        - id: string_list
          excert: "def _string_list(item: dict[str, Any], key: str) -> list[str] | None:"
        - id: parse_import_args
          excert: "def parse_import_args(item: dict[str, Any]) -> tuple[list[str] | None, list[str] | None]:"
          docstring: "``(addImport, removeImport)`` of a raw MCP item; ``ValueError`` on malformed va…"
        - id: ImportCheckedBatchResult
          excert: "class ImportCheckedBatchResult(BatchResult[R, E]):"
          docstring: Batch result whose call-wide warning is derived from its items' unchecked impor…
          children:
          - id: ImportCheckedBatchResult.AtNmpo|lNEn03
            excert: '"Batch result whose call-wide warning is derived from its items'' unchecked impo…'
          - id: ImportCheckedBatchResult.derived_warnings
            excert: "def derived_warnings(self) -> tuple[str, ...]:"
        - id: import_entry
          excert: "def import_entry(outcome: ImportOutcome | None) -> dict[str, list[str]] | None:"
          docstring: "Serialized ``imports`` field of a result, or ``None`` if nothing changed."
        - id: S64wGg|AyaH5Z
          excert: _SPACE = re.compile('\\s+')
        - id: normalize_import
          excert: "def normalize_import(text: str) -> str:"
          docstring: "Whitespace-, quote- and trailing-``;``-insensitive form used to compare stateme…"
        - id: checked
          excert: "def _checked(statement: str) -> str:"
        - id: unique
          excert: "def _unique(statements: list[str]) -> list[str]:"
        - id: statement_spans
          excert: "def statement_spans(data: bytes) -> list[tuple[int, int]]:"
          docstring: Byte spans of the ``;``-terminated top-level statements in ``data`` (comments s…
        - id: line_extent
          excert: "def line_extent(data: bytes, start: int, end: int) -> tuple[int, int]:"
          docstring: "``(start, end)`` widened to its whole line(s) if nothing else stands on them."
        - id: delete_span
          excert: "def delete_span(data: bytes, start: int, end: int) -> bytes:"
          docstring: "Remove ``data[start:end]``, including its line if nothing else stands on it."
        - id: next_line_start
          excert: "def _next_line_start(data: bytes, offset: int) -> int:"
        - id: line_starts
          excert: "def _line_starts(data: bytes) -> list[int]:"
        - id: store
          excert: "def _store(engine: Engine, tree: Tree, data: bytes) -> None:"
        - id: apply_generic
          excert: "def apply_generic(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:"
          docstring: "Engine-agnostic fallback. ``add`` is only checked: a statement missing as a lin…"
        - id: prologue_end
          excert: "def _prologue_end(root: Any, data: bytes, is_prologue: Callable[[Any], bool] | None) -> int:"
          docstring: Offset after leading shebang/directives and header comments (comments followed …
        - id: snippet
          excert: "def _snippet(engine: Engine, statement: str, import_types: set[str]) -> str:"
        - id: apply_treesitter
          excert: "def apply_treesitter(engine: Engine, tree: Tree, add: list[str], remove: list[str], import_types: set[str], is_prologue: Callable[[Any], bool] | None=None) -> ImportOutcome:"
          docstring: Structure-aware handling for tree-sitter engines whose top-level import nodes a…
        - id: fs86A7|AnrSrF
          excert: "'#: Node types under which the RPC processes report a run of imports.'"
        - id: rpc_statement
          excert: "def _rpc_statement(statement: str) -> str:"
        - id: rpc_present
          excert: "def _rpc_present(engine: Engine, tree: Tree, source: str | None=None) -> tuple[dict[str, tuple[int, int]], list[Any]]:"
          docstring: Every top-level import statement's byte span (keyed by normalized text) and the…
        - id: g7FpPW|tkBF2z
          excert: "'#: Line prefixes that stay attached to the declaration below them (docs, annot…"
        - id: leading_start
          excert: "def _leading_start(data: bytes, starts: list[int], index: int) -> int:"
          docstring: "Start offset of line ``index``, moved up over directly attached comment/annotat…"
        - id: rpc_insertion
          excert: "def _rpc_insertion(data: bytes, present: dict[str, tuple[int, int]], top: list[Any], code: str) -> tuple[int, int, bytes]:"
          docstring: "Edit that places ``code``: behind the last import, else behind ``package``/modu…"
        - id: apply_rpc
          excert: "def apply_rpc(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:"
          docstring: Structure-aware handling for RPC engines that report import groups (Java ``impo…
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outcomes.py
        nodes:
        - id: Of4wjT|mYi9p8
          excert: '''Item outcome building blocks shared by the node-addressed ``ast_*`` mutation t…'
        - id: DInuTd|tFeX7t
          excert: from __future__ import annotations
        - id: 9tJIob|ZKi3m2
          excert: "__all__ = ['node_key', 'edit_payload', 'NodeError']"
        - id: node_key
          excert: "def node_key(path: str, id: str | None) -> dict[str, Any]:"
        - id: edit_payload
          excert: "def edit_payload(*, new_id: str | None=None, imports: ImportOutcome | None=None, **extra: Any) -> dict[str, Any]:"
        - id: NodeError
          excert: "@dataclass(frozen=True) class NodeError(ItemError):"
          docstring: Error for one node-addressed item; ``path`` and ``id`` are mirrored exactly as …
          children:
          - id: NodeError.cqewr3|ktqFVv
            excert: '''Error for one node-addressed item; ``path`` and ``id`` are mirrored exactly as…'
          - id: NodeError.key
            excert: "def key(self) -> dict[str, Any]:"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
        nodes:
        - id: ZjoNfB|uJIfID
          excert: '''Engine-agnostic model shared by every ``ast_*`` tool.\n\nThe tools address nod…'
        - id: oRSlHi|kUrRiw
          excert: from __future__ import annotations
        - id: K1Mt8u|UQj1Lh
          excert: "if TYPE_CHECKING:"
        - id: AstError
          excert: "class AstError(Exception):"
          docstring: "A user-facing, path-free error raised by the AST tools."
        - id: AstAmbiguous
          excert: "class AstAmbiguous(AstError):"
          docstring: Raised when a text-based node search (no id given) finds several unrelated cand…
          children:
          - id: AstAmbiguous.FSy0jO|jchvnz
            excert: '''Raised when a text-based node search (no id given) finds several\n    unrelate…'
          - id: AstAmbiguous.init
            excert: "def __init__(self, message: str, candidates: list[str]) -> None:"
        - id: AstTextError
          excert: "class AstTextError(AstError):"
          docstring: Raised when a text/marker-based edit's search text could not be applied. Carrie…
          children:
          - id: AstTextError.xHQ70j|YI3gjQ
            excert: '"Raised when a text/marker-based edit''s search text could not be applied.\n\n  …'
          - id: AstTextError.init
            excert: "def __init__(self, message: str, *, reason: str | None=None, position: str | None=None, corrected_text: str | None=None, guess: str | None=None, next_step: str | None=None) -> None:"
        - id: Tree
          excert: "@dataclass class Tree:"
          docstring: "A parsed file/snippet plus the engine that owns it. Attributes: engine: The eng…"
        - id: Located
          excert: "@dataclass class Located:"
          docstring: A node with the engine-independent metadata the selectors match on. Attributes:…
        - id: OutlineNode
          excert: "@dataclass(frozen=True) class OutlineNode:"
          docstring: "One node in a structural (list/find/read) result. ``id`` is the node's unique, …"
        - id: line_range
          excert: "def line_range(loc: Located) -> str:"
          docstring: "Return ``loc``'s start line, or a ``\"start-end\"`` range if it spans several."
        - id: 1vtTBH|EXC8hj
          excert: _ID_CLEAN_RE = re.compile('\\W+')
        - id: hash
          excert: "def _hash(name: str, length: int) -> str:"
        - id: ThMOZm|vomnxc
          excert: _ID_HASH_ALPHABET = string.digits + string.ascii_letters
        - id: base62_hash
          excert: "def _base62_hash(text: str, length: int) -> str:"
          docstring: Base62 (0-9a-zA-Z) digest of ``text``.
        - id: content_hash
          excert: "def _content_hash(content: str, length: int=6) -> str:"
          docstring: "Base62 digest of ``content``, stable across unrelated tree edits."
        - id: content_prefix_hash
          excert: "def _content_prefix_hash(content: str, length: int=6) -> str:"
          docstring: Base62 digest of ``content``'s whitespace-stripped first/last 20 chars. Forms a…
        - id: mFoC5C|Xl4C8a
          excert: _ID_SUFFIX_RE = re.compile('_\\d+$')
        - id: id_prefix
          excert: "def _id_prefix(segment: str) -> str | None:"
          docstring: "An anonymous id segment's stable prefix (before ``'|'``), if any."
        - id: resolve_by_prefix
          excert: "def resolve_by_prefix(located: list['Located'], target_id: str) -> 'Located | None':"
          docstring: Fallback selector for a ``target_id`` that matches no node exactly. Used when a…
        - id: id_segment
          excert: "def id_segment(name: str | None, index: int, used: dict[str, int], *, hash_only: bool=False, content: str | None=None) -> str:"
          docstring: "Return a unique-within-siblings id segment, name-based when feasible. A clean, …"
        - id: node_outline
          excert: "def node_outline(loc: Located, *, with_code: bool=False, with_lines: bool=True, with_type: bool=True, children: list[OutlineNode] | None=None) -> OutlineNode:"
          docstring: "Build an :class:`OutlineNode` describing ``loc`` (source only if ``with_code``,…"
        - id: compact
          excert: "def _compact(value: Any) -> Any:"
          docstring: Recursively drop ``None`` values and empty lists from a dataclass-derived struc…
        - id: to_dict
          excert: "def to_dict(node: OutlineNode) -> dict:"
          docstring: "Serialize an :class:`OutlineNode` to MCP output, omitting empty fields."
        - id: TreeNode
          excert: "@dataclass class _TreeNode:"
        - id: build_forest
          excert: "def _build_forest(located: list[Located]) -> list[_TreeNode]:"
          docstring: Nest a pre-order list of ``Located`` into a forest via ``node_id`` prefixes.
        - id: build_outline
          excert: "def build_outline(located: list[Located], *, with_code: bool=False, with_lines: bool=True, with_type: bool=True) -> list[OutlineNode]:"
          docstring: "Build the nested outline of ``located`` (source only if ``with_code``, lines on…"
        - id: outline_nodes
          excert: "def _outline_nodes(nodes: list['_TreeNode'], *, with_code: bool, with_lines: bool=True, with_type: bool=True) -> list[OutlineNode]:"
          docstring: "Convert a forest into OutlineNodes, collapsing non-expandable nodes to full sou…"
        - id: resolve_by_name
          excert: "def _resolve_by_name(key: str, by_name: dict[str, list['_TreeNode']]) -> tuple['_TreeNode | None', str | None]:"
          docstring: Resolve ``key`` against node names when it doesn't match an id directly. Tries …
        - id: read_subtrees
          excert: "def read_subtrees(located: list[Located], keys: list[str], *, with_lines: bool=True) -> tuple[list[OutlineNode], list[str]]:"
          docstring: "Return one read subtree per resolvable ``keys`` entry. Each key is matched, in …"
        - id: matches
          excert: "def matches(loc: Located, *, id: str | None=None, node_type: str | None=None, name: str | None=None, parent_type: str | None=None) -> bool:"
        - id: find
          excert: "def find(tree: Tree, **filters: object) -> list[Located]:"
        - id: most_specific
          excert: "def most_specific(located: list[Located], lineno: int, end_lineno: int) -> Located | None:"
          docstring: "Return the smallest node in *located* fully containing lines [lineno, end_linen…"
        - id: Engine
          excert: "class Engine(ABC):"
          docstring: "A parser back-end turning source into an addressable, mutable tree. Structural …"
          children:
          - id: Engine.yeTUw1|1zTXCW
            excert: "'A parser back-end turning source into an addressable, mutable tree.\\n\\n    Str…"
          - id: Engine.dqC7gX|bz1VNl
            excert: "'#: Whether ``validate``/``replace`` reliably reject malformed edits. Only then'"
          - id: Engine.parse
            excert: "@abstractmethod def parse(self, source: str, path: Path | None=None) -> Tree:"
            docstring: "Parse ``source`` into a :class:`Tree`, raising :class:`AstError` on error."
          - id: Engine.empty_tree
            excert: "@abstractmethod def empty_tree(self, path: Path | None=None) -> Tree:"
            docstring: "Return an empty tree, used when appending to a not-yet-existing file."
          - id: Engine.serialize
            excert: "@abstractmethod def serialize(self, tree: Tree) -> str:"
            docstring: Render ``tree`` back to source text for writing to disk.
          - id: Engine.validate
            excert: "@abstractmethod def validate(self, source: str) -> str | None:"
            docstring: "Return an error message if ``source`` is malformed, else ``None``."
          - id: Engine.locate_all
            excert: "@abstractmethod def locate_all(self, tree: Tree) -> list[Located]:"
            docstring: "Flatten ``tree`` into every addressable node, in document order."
          - id: Engine.is_definition
            excert: "def is_definition(self, node_type: str) -> bool:"
            docstring: Whether ``node_type`` is "def-like" enough for a ``excert`` to make sense. Defa…
          - id: Engine.signature
            excert: "@abstractmethod def signature(self, node: Any) -> str:"
            docstring: One-line rendering of ``node``'s header (or the node itself).
          - id: Engine.default_signature
            excert: "def default_signature(self, node: Any, limit: int=80) -> str:"
            docstring: "Fallback used when ``is_definition`` is False: the first non-blank line of the …"
          - id: Engine.docstring
            excert: "@abstractmethod def docstring(self, node: Any) -> str | None:"
            docstring: "Short docstring of ``node``, if the format has such a concept."
          - id: Engine.node_code
            excert: "@abstractmethod def node_code(self, node: Any) -> str:"
            docstring: Full source of a single ``node``.
          - id: Engine.replace
            excert: "@abstractmethod def replace(self, loc: Located, code: str) -> None:"
            docstring: Replace ``loc``'s node with ``code``.
          - id: Engine.insert
            excert: "@abstractmethod def insert(self, loc: Located, code: str, position: str) -> int:"
            docstring: Insert ``code`` ``"before"``/``"after"`` ``loc``; return units inserted.
          - id: Engine.delete
            excert: "@abstractmethod def delete(self, loc: Located) -> None:"
            docstring: Delete ``loc``'s node from its container.
          - id: Engine.append
            excert: "@abstractmethod def append(self, tree: Tree, code: str) -> int:"
            docstring: Append ``code`` at ``tree``'s top level; return units appended.
          - id: Engine.apply_imports
            excert: "def apply_imports(self, tree: Tree, add: list[str], remove: list[str]) -> 'ImportOutcome':"
            docstring: Add missing / remove present import statements (``addImport``/``removeImport``)…
        - id: require_path
          excert: "def require_path(path_str: str, *, must_exist: bool=True) -> Path:"
          docstring: "Validate a mandatory absolute path, raising :class:`AstError` on failure."
        - id: GaNJOk|lfM39j
          excert: "_CONTROL_CHAR_RE = re.compile('[\\x00-\\x08\\x0b\\x0c\\x0e-\\x1f\\x7f]')"
        - id: check_no_control_chars
          excert: "def check_no_control_chars(code: str) -> None:"
          docstring: Raise :class:`AstError` if ``code`` contains abnormal control characters (e.g. …
```
/answer 1f37-1f3-1f2 allow
Control Request:
```yaml
id: 1f37-1f4-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
    ids:
    - OsZYRc|1ZfHhi
    - 64y9R1|MAcfZT
    - AstCache
    - dRq1FJ|5EG9eJ
    - load
    - parse_for
    - locate_all
    - edit_node_source
    - relocate
    - replace_node
    - insert_node
    - delete_node
    - append_nodes
    - apply_imports
    - empty_tree
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
    ids:
    - RwEctf|qsWZ3v
    - YMszCp|e3vwZI
    - select_by_path
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outcomes.py
    ids:
    - node_key
    - edit_payload
    - NodeError
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
    ids:
    - Tree
    - Located
    - Engine.node_code
    - Engine.delete
    - Engine.append
    - Engine.empty_tree
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    ids:
    - ImportOutcome
    - ImportCheckedBatchResult
    - import_entry
  reason: "Implementierungen der core-Funktionen sowie Located/Tree, Selektor- und Payload-Helfer lesen"
```
/answer 1f37-1f4-0 allow
Control Request:
```yaml
id: 1f37-1f5-1f4
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
        nodes:
        - id: OsZYRc|1ZfHhi
          type: imports
          code: |-
            from __future__ import annotations
            import hashlib
            import logging
            import threading
            from dataclasses import dataclass
            from pathlib import Path
            from xy.ai.mcpc.tools.ast.base import AstAmbiguous, AstError, AstTextError, Engine, Located, OutlineNode, Tree, build_outline, find, line_range, matches, most_specific, node_outline, read_subtrees, require_path, resolve_by_prefix, to_dict
            from xy.ai.mcpc.tools.ast.imports import ImportOutcome, apply_generic
            from xy.ai.mcpc.tools.ast import generic
        - id: 64y9R1|MAcfZT
          type: imports
          code: from xy.ai.mcpc.tools.ast import python
        - id: AstCache
          type: ClassDef
          excert: "class AstCache:"
          docstring: Content-hash validated cache of parsed trees keyed by absolute path.
          children:
          - id: AstCache.AzRmHX|ayni5v
            type: statements
            code: '''Content-hash validated cache of parsed trees keyed by absolute path.'''
          - id: AstCache.init
            type: FunctionDef
            code: |-
              def __init__(self) -> None:
                  self._entries: dict[str, _CacheEntry] = {}
                  self._lock = threading.RLock()
          - id: AstCache.get_tree
            type: FunctionDef
            code: |-
              def get_tree(self, path: Path) -> Tree:
                  key = str(path)
                  engine = engine_for_path(path)
                  with self._lock:
                      entry = self._entries.get(key)
                      mtime_ns = path.stat().st_mtime_ns
                      if entry is not None and entry.mtime_ns == mtime_ns:
                          return entry.tree
                      source = path.read_text(encoding='utf-8')
                      digest = hashlib.sha256(source.encode('utf-8')).hexdigest()
                      if entry is not None and entry.content_hash == digest:
                          entry.mtime_ns = mtime_ns
                          return entry.tree
                      tree = engine.parse(source, path)
                      self._entries[key] = _CacheEntry(mtime_ns, digest, tree)
                      return tree
          - id: AstCache.save
            type: FunctionDef
            code: |-
              def save(self, path: Path, tree: Tree) -> str:
                  """Serialise *tree*, write it to *path* and refresh the cache entry."""
                  source = tree.engine.serialize(tree)
                  path.write_text(source, encoding='utf-8')
                  '# Re-parse so cached positions match the file exactly.'
                  normalized = tree.engine.parse(source, path)
                  digest = hashlib.sha256(source.encode('utf-8')).hexdigest()
                  with self._lock:
                      self._entries[str(path)] = _CacheEntry(path.stat().st_mtime_ns, digest, normalized)
                  return source
          - id: AstCache.invalidate
            type: FunctionDef
            code: |-
              def invalidate(self, path: Path) -> None:
                  with self._lock:
                      self._entries.pop(str(path), None)
        - id: dRq1FJ|5EG9eJ
          type: statements
          code: |-
            '#: Process-wide shared cache instance.'
            CACHE = AstCache()
        - id: load
          type: FunctionDef
          code: |-
            def load(path_str: str) -> tuple[Path, Tree]:
                """Resolve *path_str* and return it together with its cached tree."""
                path = require_path(path_str)
                return (path, CACHE.get_tree(path))
        - id: parse_for
          type: FunctionDef
          code: |-
            def parse_for(path_str: str, code: str) -> Tree:
                """Parse *code* with the engine selected for *path_str*'s extension."""
                path = require_path(path_str, must_exist=False)
                return engine_for_path(path).parse(code, path)
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
        - id: relocate
          type: FunctionDef
          code: |-
            def relocate(tree: Tree, old: Located) -> Located | None:
                """Find the node now occupying ``old``'s former slot after an in-place edit.

                Matched by start line and parent type (unaffected by the edit itself), then
                disambiguated by sibling index if several candidates share that line.
                """
                candidates = [loc for loc in locate_all(tree) if loc.lineno == old.lineno and loc.parent_type == old.parent_type]
                for loc in candidates:
                    if loc.index == old.index:
                        return loc
                return candidates[0] if candidates else None
        - id: replace_node
          type: FunctionDef
          code: |-
            def replace_node(loc: Located, code: str) -> str | None:
                """Replace ``loc``'s node with ``code``; return its new id."""
                loc.tree.engine.replace(loc, code)
                new_loc = relocate(loc.tree, loc)
                return new_loc.node_id if new_loc and new_loc.node_id != loc.node_id else loc.node_id
        - id: insert_node
          type: FunctionDef
          code: |-
            def insert_node(loc: Located, code: str, position: str) -> int:
                return loc.tree.engine.insert(loc, code, position)
        - id: delete_node
          type: FunctionDef
          code: |-
            def delete_node(loc: Located) -> None:
                loc.tree.engine.delete(loc)
        - id: append_nodes
          type: FunctionDef
          code: |-
            def append_nodes(tree: Tree, code: str) -> int:
                return tree.engine.append(tree, code)
        - id: apply_imports
          type: FunctionDef
          code: |-
            def apply_imports(tree: Tree, add: list[str] | None, remove: list[str] | None) -> ImportOutcome | None:
                """Apply an item's ``addImport``/``removeImport`` to the already edited ``tree``.

                Independent of the node edit: engines compute all import changes first and write them
                once, so a failure leaves the tree as the node edit left it. It is then reported as
                ``unchecked``, which adds a warning to the batch. Engines without dedicated support
                get the generic check-only fallback.
                """
                if not add and (not remove):
                    return None
                try:
                    try:
                        return tree.engine.apply_imports(tree, add or [], remove or [])
                    except NotImplementedError:
                        return apply_generic(tree.engine, tree, add or [], remove or [])
                except Exception:
                    '# noqa: BLE001 - never fail the edit because of its imports'
                    logger.warning('Import handling failed for %s', tree.path, exc_info=True)
                    return ImportOutcome(unchecked=True)
        - id: empty_tree
          type: FunctionDef
          code: |-
            def empty_tree(path: Path) -> Tree:
                return engine_for_path(path).empty_tree(path)
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
        nodes:
        - id: RwEctf|qsWZ3v
          type: statements
          code: |-
            '#: Path-only selectors used by every mutation tool (replace/insert/delete/edit_*).'
            PATH_SELECTOR_PROPS = {'id': SELECTOR_PROPS['id']}
            "#: Absolute-path property shared by every batch tool's item schema."
            PATH_PROP = {'type': 'string', 'description': 'Absolute path to the file (never directories).'}
            '#: Import handling shared by every node-editing tool (replace/insert/delete/edit_*).'
        - id: YMszCp|e3vwZI
          type: statements
          code: |-
            IMPORT_PROPS = {
                'addImport': {
                    'type': 'array',
                    'items': {
                        'type': 'string',
                        'minLength': 1},
                    'description': "Import/include/use statements this edit needs, one complete statement per entry in the file's own syntax (e.g. 'import os', 'import a.b.C;', 'use std::fmt;')."},
                'removeImport': {
                    'type': 'array',
                            'items': {
                                'type': 'string',
                                'minLength': 1},
                    'description': 'Import/include/use statements that are no longer needed. Removed if present, ignored otherwise.'}}
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
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outcomes.py
        nodes:
        - id: node_key
          type: FunctionDef
          code: |-
            def node_key(path: str, id: str | None) -> dict[str, Any]:
                return {'path': path} if id is None else {'path': path, 'id': id}
        - id: edit_payload
          type: FunctionDef
          code: |-
            def edit_payload(*, new_id: str | None=None, imports: ImportOutcome | None=None, **extra: Any) -> dict[str, Any]:
                out: dict[str, Any] = {k: v for k, v in extra.items() if v is not None}
                if new_id is not None:
                    out['new_id'] = new_id
                if (entry := import_entry(imports)) is not None:
                    out['imports'] = entry
                return out
        - id: NodeError
          type: ClassDef
          excert: "@dataclass(frozen=True) class NodeError(ItemError):"
          docstring: Error for one node-addressed item; ``path`` and ``id`` are mirrored exactly as …
          children:
          - id: NodeError.cqewr3|ktqFVv
            type: statements
            code: |-
              'Error for one node-addressed item; ``path`` and ``id`` are mirrored exactly as given.'
              path: str
              id: str | None
          - id: NodeError.key
            type: FunctionDef
            code: |-
              def key(self) -> dict[str, Any]:
                  return node_key(self.path, self.id)
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
        nodes:
        - id: Tree
          type: ClassDef
          code: |-
            @dataclass
            class Tree:
                """A parsed file/snippet plus the engine that owns it.

                Attributes:
                    engine: The engine that parsed ``raw`` and knows how to mutate it.
                    raw: Engine-native tree (``ast.Module`` or ``tree_sitter.Tree``).
                    source: Current source text; the single source of truth for tree-sitter
                        engines, refreshed by the Python engine only on save.
                    path: Absolute path the tree was loaded from, or ``None`` for snippets.
                """
                engine: 'Engine'
                raw: Any
                source: str
                path: Path | None = None
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
        - id: Engine.node_code
          type: FunctionDef
          code: |-
            @abstractmethod
            def node_code(self, node: Any) -> str:
                """Full source of a single ``node``."""
        - id: Engine.delete
          type: FunctionDef
          code: |-
            @abstractmethod
            def delete(self, loc: Located) -> None:
                """Delete ``loc``'s node from its container."""
        - id: Engine.append
          type: FunctionDef
          code: |-
            @abstractmethod
            def append(self, tree: Tree, code: str) -> int:
                """Append ``code`` at ``tree``'s top level; return units appended."""
        - id: Engine.empty_tree
          type: FunctionDef
          code: |-
            @abstractmethod
            def empty_tree(self, path: Path | None=None) -> Tree:
                """Return an empty tree, used when appending to a not-yet-existing file."""
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
        - id: ImportCheckedBatchResult
          type: ClassDef
          excert: "class ImportCheckedBatchResult(BatchResult[R, E]):"
          docstring: Batch result whose call-wide warning is derived from its items' unchecked impor…
          children:
          - id: ImportCheckedBatchResult.AtNmpo|lNEn03
            type: statements
            code: '"Batch result whose call-wide warning is derived from its items'' unchecked imports."'
          - id: ImportCheckedBatchResult.derived_warnings
            type: FunctionDef
            code: |-
              def derived_warnings(self) -> tuple[str, ...]:
                  unchecked = any((r.imports is not None and r.imports.unchecked for r in self.results))
                  return (IMPORT_NOTICE,) if unchecked else ()
        - id: import_entry
          type: FunctionDef
          code: |-
            def import_entry(outcome: ImportOutcome | None) -> dict[str, list[str]] | None:
                """Serialized ``imports`` field of a result, or ``None`` if nothing changed."""
                if outcome is None:
                    return None
                entry = {key: value for key, value in (('added', outcome.added), ('removed', outcome.removed)) if value}
                return entry or None
```
/answer 1f37-1f5-1f4 allow
Thinking:
Ich habe die `core`-API und die Struktur der Tools gelesen; als Nächstes prüfe ich `results.py`, die Tool-Registrierung und den Umgang der Engines mit Einrückung beim Einfügen.


ReasoningToken: 3342
Control Request:
```yaml
id: 1f37-1f6-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    - /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: ast\.(insert|delete|edit)\b|from xy.ai.mcpc.tools.ast import .*insert|ast_delete|tools\.ast\.delete
    include:
    - '*.py'
    - '*.toml'
    - '*.md'
    - '*.json'
    exclude:
    - '**/__pycache__/**'
    limit: 40
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: ^class (BatchResult|ItemResult|ItemError)|def (to_dict|payload|key|derived_warnings)
    include:
    - results.py
  reason: "Registrierung der AST-Tools, BatchResult/ItemResult-API und vorhandene Tests finden"
```
/answer 1f37-1f6-0 allow
Control Request:
```yaml
id: 1f37-1f7-1f6
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc/src
        - /home/user/xyan/xy.ai.workbench/mcpc/tests
        matches:
        - path: xy/ai/mcpc/tools/ast/script.py
          matches:
          - "'description': \"Python source (or an absolute/relative path to a script file, if the value is nothing but a path to an existing file) executed with 't…ne header / short doc, only set for definition-like nodes when code is omitted) have no direct Located field; derive via node_code(loc) if needed.\"}},"
        - path: xy/ai/mcpc/tools/ast/edit.py
          matches:
          - from xy.ai.mcpc.tools.ast.delete import DeleteTool
          - from xy.ai.mcpc.tools.ast.insert import InsertNodeTool
        - path: xy/ai/mcpc/tools/ast/delete.py
          matches:
          - "\"\"\"``ast_delete`` tool: delete selected nodes, or whole files if none selected.\"\"\""
          - "__all__ = ['DeleteItem', 'DeleteResult', 'DeleteError', 'DeleteBatchResult', 'ast_delete', 'DeleteTool', 'register']"
          - '"""Result of :func:`ast_delete`.'
          - "def ast_delete(items: list[DeleteItem]) -> DeleteBatchResult:"
          - name = 'ast_delete'
          - "\"\"\"Delegate to :func:`ast_delete`, translating the MCP schema to/from the Python API.\"\"\""
          - "return handle_batch_tool(ctx, item_factory, ast_delete, core.AstError, auto_approve=True)"
          - functions.register(ast_delete)
        - path: xy/ai/mcpc/tools/ast/__init__.py
          matches:
          - "node-level CRUD, each tool in its own ``*`` module (``ast_create``/``ast_delete``"
          - "from xy.ai.mcpc.tools.ast import create, delete, edit, edit_block, edit_marks, find, insert, read, replace, retrieve, script, validate"
          - "'ast_delete',"
        - path: test_ast_engines.py
          matches:
          - "from xy.ai.mcpc.tools.ast.delete import DeleteItem, ast_delete"
          - "from xy.ai.mcpc.tools.ast.insert import InsertItem, ast_insert"
          - "ast_delete(items=[DeleteItem(path=py_file, id=import_id)])"
        - path: test_ast_markdown_bridge.py
          matches:
          - "from xy.ai.mcpc.tools.ast.delete import DeleteItem, ast_delete"
          - "from xy.ai.mcpc.tools.ast.insert import InsertItem, ast_insert"
          - "result = ast_delete(items=[DeleteItem(path=md_file, id=_file_id(md_file, 'Doc', 'Part 2'))])"
        - path: test_ast_imports.py
          matches:
          - "from xy.ai.mcpc.tools.ast.delete import DeleteItem, ast_delete"
          - "from xy.ai.mcpc.tools.ast.insert import InsertItem, ast_insert"
          - "result = ast_delete(items=[DeleteItem(path=path, id='A', remove_import=['import os'])])"
          - "result = ast_delete(items=[DeleteItem(path=path, add_import=['import sys'])])"
        warnings:
        - 1 match line(s) exceeded 300 characters and were shortened in the middle.
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
        matches:
        - path: results.py
          matches:
          - "def to_dict(self) -> dict[str, Any]:"
          - "def payload(self) -> dict[str, Any]:"
          - "def key(self) -> dict[str, Any]:"
          - "def to_dict(self, *, payload: dict[str, Any] | None=None) -> dict[str, Any]:"
          - "class ItemResult(ItemOutcome):"
          - "class ItemError(ItemOutcome):"
          - "class BatchResult(ToolPayload, Generic[R, E]):"
        warnings:
        - "Limit of 15 matches reached (total 17); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
```
/answer 1f37-1f7-1f6 allow
Control Request:
```yaml
id: 1f37-1f8-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
  reason: results.py (Payload/Key-Semantik) und __init__.py (Tool-Registrierung) vollständig verstehen
```
/answer 1f37-1f8-0 allow
Control Request:
```yaml
id: 1f37-1f9-1f8
result:
  structuredContent:
    outline:
      results:
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
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
        nodes:
        - id: 12csGL|fKey3N
          excert: '''``ast_*`` tool family built on the standard-library ``ast`` module.\n\nA conte…'
        - id: ukjPdF|CoBg9W
          excert: from xy.ai.mcpc.tools.tool_registry import ToolRegistry
        - id: mADxqb|yRFYn0
          excert: "__all__ = ['register_ast_tools', 'ALIAS']"
        - id: register_ast_tools
          excert: "def register_ast_tools(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
          docstring: Register every ``ast_*`` tool and the ``ast`` alias.
```
/answer 1f37-1f9-1f8 allow
Control Request:
```yaml
id: 1f37-1fa-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: ast
    recursive: false
  reason: "Testdateien für AST-Tools finden, um analog Tests für ast_move anzulegen"
```
/answer 1f37-1fa-0 allow
Control Request:
```yaml
id: 1f37-1fb-1fa
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/tests
      entries:
      - ".:"
      - test_ast_engines.py
      - test_ast_imports.py
      - test_ast_imports_rpc.py
      - test_ast_markdown_bridge.py
      - test_ast_retrieve.py
      - test_ast_rust_bridge.py
      - test_grep_with_ast.py
```
/answer 1f37-1fb-1fa allow
Control Request:
```yaml
id: 1f37-1fc-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
    ids:
    - 12csGL|fKey3N
    - ukjPdF|CoBg9W
    - mADxqb|yRFYn0
    - register_ast_tools
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
    ids:
    - guard_fields
    - checked_payload
    - ItemResult
    - BatchResult
  reason: "Registrierung, Reserved-Field-Regeln von ItemResult und BatchResult-Serialisierung lesen"
```
/answer 1f37-1fc-0 allow
Control Request:
```yaml
id: 1f37-1fd-1fc
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
        nodes:
        - id: 12csGL|fKey3N
          type: statements
          code: "'``ast_*`` tool family built on the standard-library ``ast`` module.\\n\\nA content-hash validated cache (:mod:`.core`) holds parsed modules; comments are\\nconverted to standalone string-literal annotations on import so they survive the\\n``parse``/``unparse`` round-trip. Retrieval is layered on a\\nsingle ``outline`` tree (``ast_outline`` structure, ``ast_find`` property/text/regexp\\nfiltering with source, ``ast_read`` reads subtrees by id); mutation is\\nnode-level CRUD, each tool in its own ``*`` module (``ast_create``/``ast_delete``\\ncover the whole-file case too), with two in-node editors ``ast_edit_marks``\\n(marker-delimited) and ``ast_edit_block`` (exact block), a restricted ``script``\\nand a ``validate`` compile check.\\n'"
        - id: ukjPdF|CoBg9W
          type: imports
          code: |-
            from xy.ai.mcpc.tools.tool_registry import ToolRegistry
            from xy.ai.mcpc.tools.function_registry import FunctionRegistry
            from xy.ai.mcpc.tools.ast import create, delete, edit, edit_block, edit_marks, find, insert, read, replace, retrieve, script, validate
            from xy.ai.mcpc.tools.ast import outline
        - id: mADxqb|yRFYn0
          type: statements
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
                'ast_edit',
                'ast_validate')
            ALIAS_READ = 'ast-read'
            _ALIAS_READ_MEMBERS = ('ast_outline', 'ast_find', 'ast_read')
        - id: register_ast_tools
          type: FunctionDef
          code: |-
            def register_ast_tools(registry: ToolRegistry, functions: FunctionRegistry) -> None:
                """Register every ``ast_*`` tool and the ``ast`` alias."""
                outline.register(registry, functions)
                find.register(registry, functions)
                read.register(registry, functions)
                retrieve.register(registry, functions)
                insert.register(registry, functions)
                edit_marks.register(registry, functions)
                edit_block.register(registry, functions)
                replace.register(registry, functions)
                delete.register(registry, functions)
                create.register(registry, functions)
                edit.register(registry, functions)
                script.register(registry, functions)
                validate.register(registry, functions)
                registry.register_alias(ALIAS, _ALIAS_MEMBERS)
                registry.register_alias(ALIAS_READ, _ALIAS_READ_MEMBERS)
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/results.py
        nodes:
        - id: guard_fields
          type: FunctionDef
          code: |-
            def _guard_fields(cls: type) -> None:
                clash = set(get_annotations(cls)) & RESERVED_NAMES
                if clash:
                    raise TypeError(f'{cls.__name__}: reserved field name(s) {sorted(clash)}; use the base class API')
                if 'to_dict' in cls.__dict__:
                    raise TypeError(f'{cls.__name__}: to_dict is final; override payload()')
        - id: checked_payload
          type: FunctionDef
          code: |-
            def _checked_payload(owner: object, payload: dict[str, Any]) -> dict[str, Any]:
                clash = RESERVED_NAMES & payload.keys()
                if clash:
                    raise ValueError(f'{type(owner).__name__}: payload uses reserved key(s) {sorted(clash)}')
                return payload
        - id: ItemResult
          type: ClassDef
          excert: "@dataclass(frozen=True, kw_only=True) class ItemResult(ItemOutcome):"
          docstring: "An item that produced a result, possibly with restrictions (``warnings``)."
          children:
          - id: ItemResult.tihVbt|PBPiGn
            type: statements
            code: |-
              'An item that produced a result, possibly with restrictions (``warnings``).'
              warnings: tuple[str, ...] = ()
          - id: ItemResult.init_subclass
            type: FunctionDef
            code: |-
              def __init_subclass__(cls, **kwargs: Any) -> None:
                  super().__init_subclass__(**kwargs)
                  _guard_fields(cls)
          - id: ItemResult.payload
            type: FunctionDef
            code: |-
              def payload(self) -> dict[str, Any]:
                  return compact_asdict(self, exclude={WARNINGS_KEY})
          - id: ItemResult.to_dict
            type: FunctionDef
            code: |-
              @final
              def to_dict(self, *, payload: dict[str, Any] | None=None) -> dict[str, Any]:
                  body = self.payload() if payload is None else payload
                  out = {**self.key(), **_checked_payload(self, body)}
                  if self.warnings:
                      out[WARNINGS_KEY] = list(self.warnings)
                  return out
          - id: ItemResult.warn
            type: FunctionDef
            code: |-
              def warn(self, *messages: str) -> Self:
                  return replace(self, warnings=(*self.warnings, *messages))
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
```
/answer 1f37-1fd-1fc allow
ReasoningToken: 76
Control Request:
```yaml
id: 1f37-1fe-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: RESERVED_NAMES\s*=|WARNINGS_KEY\s*=
    include:
    - '*.py'
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: def (node_code|insert|append|delete|empty_tree|serialize)\(
    include:
    - '*.py'
    limit: 50
  reason: RESERVED_NAMES prüfen sowie node_code/insert/append/delete der Python- und Generic-Engine (Einrückung beim Verschieben) finden
```
/answer 1f37-1fe-0 allow
Control Request:
```yaml
id: 1f37-1ff-1fe
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
        matches:
        - path: results.py
          matches:
          - WARNINGS_KEY = 'warnings'
          - "RESERVED_NAMES = frozenset({'warning', 'warnings', 'notice', 'notices', 'hint', 'error', 'errors', 'message'})"
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
        matches:
        - path: base.py
          matches:
          - "def empty_tree(self, path: Path | None=None) -> Tree:"
          - "def serialize(self, tree: Tree) -> str:"
          - "def node_code(self, node: Any) -> str:"
          - "def insert(self, loc: Located, code: str, position: str) -> int:"
          - "def delete(self, loc: Located) -> None:"
          - "def append(self, tree: Tree, code: str) -> int:"
        - path: script.py
          matches:
          - "def node_code(self, loc: core.Located) -> str:"
          - "def insert(self, loc: core.Located, code: str, position: str='after') -> int:"
          - "def delete(self, loc: core.Located) -> None:"
          - "def append(self, code: str) -> int:"
        - path: rpc/_engine.py
          matches:
          - "def empty_tree(self, path: Path | None=None) -> Tree:"
          - "def serialize(self, tree: Tree) -> str:"
          - "def node_code(self, node: _RpcNode) -> str:"
          - "def insert(self, loc: Located, code: str, position: str) -> int:"
          - "def delete(self, loc: Located) -> None:"
          - "def append(self, tree: Tree, code: str) -> int:"
        - path: generic/_text.py
          matches:
          - "def empty_tree(self, path: Path | None=None) -> Tree:"
          - "def serialize(self, tree: Tree) -> str:"
          - "def node_code(self, node: Any) -> str:"
          - "def insert(self, loc: Located, code: str, position: str) -> int:"
          - "def delete(self, loc: Located) -> None:"
          - "def append(self, tree: Tree, code: str) -> int:"
        - path: generic/_engine.py
          matches:
          - "def empty_tree(self, path: Path | None=None) -> Tree:"
          - "def serialize(self, tree: Tree) -> str:"
          - "def node_code(self, node: Any) -> str:"
          - "def insert(self, loc: Located, code: str, position: str) -> int:"
          - "def delete(self, loc: Located) -> None:"
          - "def append(self, tree: Tree, code: str) -> int:"
        - path: python/_engine.py
          matches:
          - "def empty_tree(self, path: Path | None=None) -> Tree:"
          - "def serialize(self, tree: Tree) -> str:"
          - "def node_code(self, node: Any) -> str:"
          - "def insert(self, loc: Located, code: str, position: str) -> int:"
          - "def delete(self, loc: Located) -> None:"
          - "def append(self, tree: Tree, code: str) -> int:"
        - path: core.py
          matches:
          - "def empty_tree(path: Path) -> Tree:"
```
/answer 1f37-1ff-1fe allow
Control Request:
```yaml
id: 1f37-200-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
    node_type: FunctionDef
    regexp: ^def (node_code|insert|append|delete|empty_tree)\(
  reason: "Implementierungen von node_code/insert/append/delete in Python-, Generic- und RPC-Engine lesen (Einrückung, Separatoren, Gültigkeit von Located nach Löschung)"
```
/answer 1f37-200-0 allow
Control Request:
```yaml
id: 1f37-201-200
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
        nodes: []
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
        nodes: []
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
        nodes: []
```
/answer 1f37-201-200 allow
Control Request:
```yaml
id: 1f37-202-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
    name: insert
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
    name: node_code
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
    name: delete
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
    name: append
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
    name: insert
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
    name: node_code
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
    name: delete
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
    name: append
  reason: "Engine-Methoden per Text-Filter finden, da der Regex-Filter keine Treffer lieferte"
```
/answer 1f37-202-0 allow
Control Request:
```yaml
id: 1f37-203-202
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
        nodes:
        - id: PythonEngine.insert
          code: |-
            def insert(self, loc: Located, code: str, position: str) -> int:
                check_no_control_chars(code)
                stmts = self._parse_fragment(code)
                node = loc.node
                if isinstance(node, _StatementGroup):
                    body = node.parent.body
                    index = node.stop if position == 'after' else node.start
                else:
                    body = loc.parent.body
                    offset = 1 if position == 'after' else 0
                    index = body.index(loc.node) + offset
                body[index:index] = stmts
                return len(stmts)
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
        nodes:
        - id: PythonEngine.node_code
          code: |-
            def node_code(self, node: Any) -> str:
                if isinstance(node, _StatementGroup):
                    return '\n'.join((_unparse(ast.fix_missing_locations(s)) for s in node.stmts))
                return _unparse(ast.fix_missing_locations(node))
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
        nodes:
        - id: PythonEngine.delete
          code: |-
            def delete(self, loc: Located) -> None:
                node = loc.node
                if isinstance(node, _StatementGroup):
                    del node.parent.body[node.start:node.stop]
                else:
                    del loc.parent.body[loc.index]
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
        nodes:
        - id: PythonEngine.append
          code: |-
            def append(self, tree: Tree, code: str) -> int:
                check_no_control_chars(code)
                stmts = self._parse_fragment(code)
                tree.raw.body.extend(stmts)
                return len(stmts)
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
        nodes:
        - id: TreeSitterEngine.insert
          code: |-
            def insert(self, loc: Located, code: str, position: str) -> int:
                check_no_control_chars(code)
                if position == 'before':
                    self._splice(loc.tree, loc.node.start_byte, loc.node.start_byte, code + '\n')
                else:
                    self._splice(loc.tree, loc.node.end_byte, loc.node.end_byte, '\n' + code)
                return 1
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
        nodes:
        - id: TreeSitterEngine.node_code
          code: |-
            def node_code(self, node: Any) -> str:
                return node.text.decode('utf-8', 'replace')
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
        nodes:
        - id: TreeSitterEngine.delete
          code: |-
            def delete(self, loc: Located) -> None:
                self._splice(loc.tree, loc.node.start_byte, loc.node.end_byte, '')
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
        nodes:
        - id: TreeSitterEngine.append
          code: |-
            def append(self, tree: Tree, code: str) -> int:
                check_no_control_chars(code)
                sep = '' if not tree.source or tree.source.endswith('\n') else '\n'
                self._splice(tree, len(tree.source.encode('utf-8')), len(tree.source.encode('utf-8')), sep + code)
                return 1
```
/answer 1f37-203-202 allow
Thinking:
Die Python-Engine braucht wegen `unparse` keine Rücksicht auf Einrückung, Tree-sitter und RPC fügen aber Rohtext ein, also achte ich beim Verschieben verschachtelter Knoten darauf. Als Nächstes prüfe ich die RPC-Engine und die Python-Exporte.


ReasoningToken: 3024
Control Request:
```yaml
id: 1f37-204-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
    name: insert
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
    name: node_code
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
    name: delete
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
    name: append
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
    ids:
    - Engine.dqC7gX|bz1VNl
  reason: "RPC-Engine insert/node_code/delete und Engine-Klassenattribute prüfen, um Re-Indentation korrekt zu entscheiden"
```
/answer 1f37-204-0 allow
Control Request:
```yaml
id: 1f37-205-204
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
        nodes:
        - id: RpcEngine.insert
          code: |-
            def insert(self, loc: Located, code: str, position: str) -> int:
                check_no_control_chars(code)
                request = InsertRequest()
                request.set_Source(loc.tree.source)
                if loc.tree.path is not None:
                    request.set_Path(str(loc.tree.path))
                request.set_Code(code)
                request.set_Position(PositionEnum.BEFORE if position == 'before' else PositionEnum.AFTER)
                payload = self._unwrap(self._rpc().insertRelativeToNode(loc.node_id, request))
                loc.tree.source = payload.get_Source()
                return payload.get_UnitsInserted()
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
        nodes:
        - id: RpcEngine.node_code
          code: |-
            def node_code(self, node: _RpcNode) -> str:
                if node.code is not None:
                    return node.code
                lines = node.tree.source.splitlines(keepends=True)
                return ''.join(lines[node.lineno - 1:node.end_lineno])
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
        nodes:
        - id: RpcEngine.delete
          code: |-
            def delete(self, loc: Located) -> None:
                request = SourceRequest()
                request.set_Source(loc.tree.source)
                if loc.tree.path is not None:
                    request.set_Path(str(loc.tree.path))
                payload = self._unwrap(self._rpc().deleteNode(loc.node_id, request))
                loc.tree.source = payload.get_Source()
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
        nodes:
        - id: RpcEngine.append
          code: |-
            def append(self, tree: Tree, code: str) -> int:
                check_no_control_chars(code)
                request = CodeRequest()
                request.set_Source(tree.source)
                if tree.path is not None:
                    request.set_Path(str(tree.path))
                request.set_Code(code)
                payload = self._unwrap(self._rpc().appendTopLevel(request))
                tree.source = payload.get_Source()
                return payload.get_UnitsAppended()
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
        nodes:
        - id: Engine.dqC7gX|bz1VNl
          type: statements
          code: |-
            '#: Whether ``validate``/``replace`` reliably reject malformed edits. Only then'
            '#: may callers rely on re-parse to catch corruption (false for markup grammars'
            '#: whose parser accepts almost any text without reporting errors).'
            validates_syntax: bool = False
```
/answer 1f37-205-204 allow
ReasoningToken: 395
Control Request:
```yaml
id: 1f37-206-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
  reason: "Teststil (Fixtures, Imports, Assertions) in test_ast_imports.py ansehen, um Tests für ast_move analog zu schreiben"
```
/answer 1f37-206-0 allow
Control Request:
```yaml
id: 1f37-207-206
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
        nodes:
        - id: Qqmdyv|s8N7jy
          excert: '''Tests for ``addImport``/``removeImport`` handling of the node-editing ``ast_*`…'
        - id: 0ekC7u|9IXYUc
          excert: from __future__ import annotations
        - id: EDFjeC|StWP8k
          excert: "_SRC = Path(__file__).resolve().parents[1] / 'src'"
        - id: 8Pl1k7|GddZCa
          excert: from xy.ai.mcpc.tools.ast import core
        - id: 3Mpw8b|t0uVQE
          excert: "'# noqa: E402'"
        - id: NREYuD|nugLQv
          excert: "from xy.ai.mcpc.tools.ast.delete import DeleteItem, ast_delete"
        - id: 3Mpw8b|t0uVQE_1
          excert: "'# noqa: E402'"
        - id: LgnMR1|79MOp3
          excert: from xy.ai.mcpc.tools.ast.find import ast_find
        - id: 3Mpw8b|t0uVQE_2
          excert: "'# noqa: E402'"
        - id: xJMLX3|tYgCCM
          excert: "from xy.ai.mcpc.tools.ast.imports import IMPORT_NOTICE, normalize_import, state…"
        - id: 3Mpw8b|t0uVQE_3
          excert: "'# noqa: E402'"
        - id: cNhh2C|88ZZ6p
          excert: "from xy.ai.mcpc.tools.ast.insert import InsertItem, ast_insert"
        - id: 3Mpw8b|t0uVQE_4
          excert: "'# noqa: E402'"
        - id: mO2aS8|5v2aZQ
          excert: "from xy.ai.mcpc.tools.ast.replace import ReplaceItem, ast_replace"
        - id: R9Ge4p|9nD8LQ
          excert: PY_SOURCE = '"""Doc."""\nimport os\n\n\nclass A:\n    pass\n'
        - id: write
          excert: "def _write(tmp_path: Path, name: str, source: str) -> str:"
        - id: first_id
          excert: "def _first_id(path: str, node_type: str) -> str:"
        - id: 3mF7m9|fcmzbE
          excert: '''# --------------------------------------------------------------------------- …'
        - id: h5732ce4e
          excert: "def test_normalize_import_ignores_whitespace_quotes_and_semicolon():"
        - id: h4a06df6b
          excert: "def test_statement_spans_skip_comments_and_braces():"
        - id: cdVHjw|Z8dp7j
          excert: '''# --------------------------------------------------------------------------- …'
        - id: ha9d8358c
          excert: "def test_python_adds_missing_imports_after_existing_ones(tmp_path: Path):"
        - id: test_python_add_is_idempotent
          excert: "def test_python_add_is_idempotent(tmp_path: Path):"
        - id: h185e7a1a
          excert: "def test_python_remove_import_and_single_alias(tmp_path: Path):"
        - id: test_python_future_import_goes_first
          excert: "def test_python_future_import_goes_first(tmp_path: Path):"
        - id: h243cd0c1
          excert: "def test_python_invalid_import_keeps_edit_and_flags_notice(tmp_path: Path):"
        - id: h5c97a8cd
          excert: "def test_delete_with_imports_removes_unused_import(tmp_path: Path):"
        - id: he8f51ccc
          excert: "def test_delete_file_ignores_imports_but_flags_notice(tmp_path: Path):"
        - id: N3p4qW|KgIdG9
          excert: '''# --------------------------------------------------------------------------- …'
        - id: test_javascript_add_and_remove
          excert: "def test_javascript_add_and_remove(tmp_path: Path):"
        - id: he1b0d6c3
          excert: "def test_javascript_without_imports_inserts_after_directive(tmp_path: Path):"
        - id: h1e0bf190
          excert: "def test_javascript_non_import_keeps_edit_and_flags_notice(tmp_path: Path):"
        - id: h02cf6cf9
          excert: "def test_failing_engine_keeps_edit_and_flags_notice(tmp_path: Path, monkeypatch):"
        - id: vTqL0d|hNFSuK
          excert: '''# --------------------------------------------------------------------------- …'
        - id: h273f84a2
          excert: "def test_generic_present_import_needs_no_notice(tmp_path: Path):"
        - id: hc2f3f704
          excert: "def test_generic_missing_import_is_not_written_but_flagged(tmp_path: Path):"
        - id: test_generic_removes_whole_line_only
          excert: "def test_generic_removes_whole_line_only(tmp_path: Path):"
        - id: test_generic_unsafe_remove_is_flagged
          excert: "def test_generic_unsafe_remove_is_flagged(tmp_path: Path):"
        - id: h739bcb8e
          excert: "@pytest.mark.parametrize('bad', [[''], 'import os', [1]]) def test_schema_level_validation_rejects_malformed_values(bad):"
```
/answer 1f37-207-206 allow
Control Request:
```yaml
id: 1f37-208-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
    ids:
    - 0ekC7u|9IXYUc
    - EDFjeC|StWP8k
    - 8Pl1k7|GddZCa
    - write
    - first_id
    - h5c97a8cd
    - h739bcb8e
  reason: "Test-Header (sys.path-Setup, Hilfsfunktionen) und ein Beispieltest lesen"
```
/answer 1f37-208-0 allow
Control Request:
```yaml
id: 1f37-209-208
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
        nodes:
        - id: 0ekC7u|9IXYUc
          type: imports
          code: |-
            from __future__ import annotations
            import sys
            from pathlib import Path
            import pytest
        - id: EDFjeC|StWP8k
          type: statements
          code: |-
            _SRC = Path(__file__).resolve().parents[1] / 'src'
            if _SRC.is_dir() and str(_SRC) not in sys.path:
                sys.path.insert(0, str(_SRC))
            '# noqa: E402'
        - id: 8Pl1k7|GddZCa
          type: imports
          code: from xy.ai.mcpc.tools.ast import core
        - id: write
          type: FunctionDef
          code: |-
            def _write(tmp_path: Path, name: str, source: str) -> str:
                path = tmp_path / name
                path.write_text(source, encoding='utf-8')
                return str(path)
        - id: first_id
          type: FunctionDef
          code: |-
            def _first_id(path: str, node_type: str) -> str:
                return ast_find(paths=[path], node_type=node_type).files[0].nodes[0].id
        - id: h5c97a8cd
          type: FunctionDef
          code: |-
            def test_delete_with_imports_removes_unused_import(tmp_path: Path):
                path = _write(tmp_path, 'm.py', PY_SOURCE)
                result = ast_delete(items=[DeleteItem(path=path, id='A', remove_import=['import os'])])
                assert not result.errors
                assert 'import os' not in Path(path).read_text()
        - id: h739bcb8e
          type: FunctionDef
          code: |-
            @pytest.mark.parametrize('bad', [[''], 'import os', [1]])
            def test_schema_level_validation_rejects_malformed_values(bad):
                from xy.ai.mcpc.tools.ast.imports import parse_import_args
                with pytest.raises(ValueError):
                    parse_import_args({'addImport': bad})
```
/answer 1f37-209-208 allow
ReasoningToken: 450
Control Request:
```yaml
id: 1f37-20a-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    ids:
    - string_list
    - parse_import_args
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: def (handle_batch_tool|batch_schema)
    include:
    - _tool_helpers.py
  reason: parse_import_args/_string_list (Umgang mit None) und handle_batch_tool-Signatur prüfen
```
/answer 1f37-20a-0 allow
Control Request:
```yaml
id: 1f37-20b-20a
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
        matches:
        - path: _tool_helpers.py
          matches:
          - "def batch_schema(item_properties: dict[str, Any], required: list[str], description: str) -> dict[str, Any]:"
          - "def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], BatchResult[Any, Any]], error_class: type, result_payload: ResultPayload | None=None, auto_approve: bool=False) -> ToolResult:"
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
        nodes:
        - id: string_list
          type: FunctionDef
          code: |-
            def _string_list(item: dict[str, Any], key: str) -> list[str] | None:
                value = item.get(key)
                if value is None:
                    return None
                if not isinstance(value, list) or not all((isinstance(v, str) and v.strip() for v in value)):
                    raise ValueError(f"'{key}' must be a list of non-empty strings")
                return value or None
        - id: parse_import_args
          type: FunctionDef
          code: |-
            def parse_import_args(item: dict[str, Any]) -> tuple[list[str] | None, list[str] | None]:
                """``(addImport, removeImport)`` of a raw MCP item; ``ValueError`` on malformed values."""
                return (_string_list(item, 'addImport'), _string_list(item, 'removeImport'))
```
/answer 1f37-20b-20a allow
Thinking:
Ich lege jetzt das neue Modul `move.py` an, das sich an insert und delete orientiert; bei Fehlern wird der Cache verworfen.


ReasoningToken: 7334
Control Request:
```yaml
id: 1f37-20c-0
toolName: ast_edit
arguments:
  create:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    source: |
      """``ast_move`` tool: move nodes between/within files, or whole files (insert/create & delete)."""
      from __future__ import annotations

      import re
      from dataclasses import dataclass
      from pathlib import Path
      from typing import Any

      from xy.ai.mcpc.tools._tool_helpers import batch_schema, handle_batch_tool
      from xy.ai.mcpc.tools.ast import core
      from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, select_by_path
      from xy.ai.mcpc.tools.ast.imports import IMPORT_NOTICE, ImportCheckedBatchResult, ImportOutcome, import_entry, parse_import_args
      from xy.ai.mcpc.tools.ast.outcomes import edit_payload
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      from xy.ai.mcpc.tools.results import ItemError, ItemResult
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolFeature, ToolRegistry, ToolResult

      __all__ = ['MoveItem', 'MoveResult', 'MoveError', 'MoveBatchResult', 'ast_move', 'MoveTool', 'register']


      @dataclass(frozen=True)
      class MoveItem:
          """One move operation.

          Attributes:
              path: Absolute path of the source file.
              target: Absolute path of the target file. May equal ``path`` (move within the
                  file); a missing target file is created like ``ast_create`` does.
              ids: Ids of the source nodes to move, inserted in the given order. Empty/``None``
                  moves the whole file. A node listed together with one of its ancestors is
                  moved along with that ancestor.
              target_id: Id of the target node in the target file. ``None`` appends at the
                  end of the target file. An unknown ``target_id`` aborts the item.
              position: ``"before"`` or ``"after"`` the target node. Defaults to ``"after"``.
              add_import / remove_import: Import statements to add/remove in the target file.
              source_add_import / source_remove_import: Import statements to add/remove in the
                  source file (ignored and flagged if the source file is deleted).
          """
          path: str
          target: str
          ids: list[str] | None = None
          target_id: str | None = None
          position: str = 'after'
          add_import: list[str] | None = None
          remove_import: list[str] | None = None
          source_add_import: list[str] | None = None
          source_remove_import: list[str] | None = None


      @dataclass(frozen=True)
      class MoveResult(ItemResult):
          """Result of a single move operation.

          Attributes:
              path: The source path exactly as given in the input, for result association.
              target: The target path exactly as given in the input.
              moved: ``(old id, new id)`` per moved node; the new id is ``None`` if it could
                  not be attributed unambiguously.
              unassigned: New top-level ids in the target that could not be attributed.
              imports: Import outcome in the target file.
              source_imports: Import outcome in the source file.
              source_deleted: Whether the source file was removed (moved whole or left empty).
          """
          path: str
          target: str
          moved: list[tuple[str, str | None]]
          unassigned: list[str] | None = None
          imports: ImportOutcome | None = None
          source_imports: ImportOutcome | None = None
          source_deleted: bool = False

          def key(self) -> dict[str, Any]:
              return {'path': self.path, 'target': self.target}

          def payload(self) -> dict[str, Any]:
              moved = [{'old': old} if new is None else {'old': old, 'new': new} for old, new in self.moved]
              return edit_payload(
                  imports=self.imports,
                  moved=moved,
                  unassigned=self.unassigned or None,
                  sourceImports=import_entry(self.source_imports),
                  sourceDeleted=True if self.source_deleted else None)


      @dataclass(frozen=True)
      class MoveError(ItemError):
          """Error of a move operation, or of a single source id within it.

          Attributes:
              path: The source path exactly as given in the input.
              target: The target path exactly as given in the input.
              id: The source id that could not be resolved, or ``None`` if the whole item failed.
              error: The error message.
          """
          path: str
          target: str
          id: str | None = None

          def key(self) -> dict[str, Any]:
              out: dict[str, Any] = {'path': self.path, 'target': self.target}
              if self.id is not None:
                  out['id'] = self.id
              return out


      class MoveBatchResult(ImportCheckedBatchResult[MoveResult, MoveError]):
          """Result of :func:`ast_move`.

          Attributes:
              results: One :class:`MoveResult` per item that moved at least one node or file.
              errors: One :class:`MoveError` per failed item and per unresolvable source id.
          """

          def derived_warnings(self) -> tuple[str, ...]:
              unchecked = any(
                  outcome is not None and outcome.unchecked
                  for r in self.results
                  for outcome in (r.imports, r.source_imports))
              return (IMPORT_NOTICE,) if unchecked else ()


      _SUFFIX_RE = re.compile(r'_\d+$')


      def _is_inside(node_id: str, ancestor_id: str) -> bool:
          return node_id.startswith(ancestor_id + '.')


      def _top_level(ids: list[str]) -> list[str]:
          """Those ``ids`` that have no ancestor among ``ids`` (document order kept)."""
          return [i for i in ids if not any(_is_inside(i, other) for other in ids if other != i)]


      def _same_file(a: Path, b: Path) -> bool:
          return a.resolve() == b.resolve()


      def _remove_file(path: Path) -> None:
          """Delete ``path`` like ``ast_delete`` does: drop it from the cache and remove an emptied directory."""
          try:
              path.unlink()
          except OSError as exc:
              raise core.AstError('Deleting the source file failed.') from exc
          core.CACHE.invalidate(path)
          parent = path.parent
          if not any(parent.iterdir()):
              parent.rmdir()


      def _save(path: Path, tree: core.Tree) -> None:
          try:
              path.parent.mkdir(parents=True, exist_ok=True)
              core.CACHE.save(path, tree)
          except OSError as exc:
              raise core.AstError('Writing a file failed.') from exc


      def _is_empty(tree: core.Tree) -> bool:
          return not tree.engine.serialize(tree).strip()


      def _place(dst_tree: core.Tree, target: core.Located | None, code: str, position: str) -> list[str]:
          """Insert/append ``code`` and return the top-level ids that are new in ``dst_tree``."""
          before = {loc.node_id for loc in core.locate_all(dst_tree)}
          if target is None:
              core.append_nodes(dst_tree, code)
          else:
              core.insert_node(target, code, position)
          return _top_level([loc.node_id for loc in core.locate_all(dst_tree) if loc.node_id not in before])


      def _attribute(old: list[tuple[str, str | None]], new_ids: list[str]) -> tuple[list[str | None], list[str]]:
          """Pair each moved node ``(old id, simple name)`` with its new top-level id.

          Paired by order if the counts match, otherwise by name. Returns the new id per
          moved node (``None`` if unattributable) and the unattributed new ids.
          """
          if len(old) == len(new_ids):
              return list(new_ids), []
          free = list(new_ids)
          mapped: list[str | None] = []
          for _old_id, name in old:
              hit = None
              if name:
                  hit = next((n for n in free if _SUFFIX_RE.sub('', n.rsplit('.', 1)[-1]) == name), None)
              if hit is not None:
                  free.remove(hit)
              mapped.append(hit)
          return mapped, free


      def _parse_position(position: str) -> str:
          if position not in ('before', 'after'):
              raise core.AstError("'position' must be 'before' or 'after'.")
          return position


      def _move_file(item: MoveItem, src_path: Path, dst_path: Path) -> MoveResult:
          if _same_file(src_path, dst_path):
              raise core.AstError('Moving a whole file onto itself is not possible; give source ids.')
          src_tree = core.CACHE.get_tree(src_path)
          old_top = [loc for loc in core.locate_all(src_tree) if loc.parent_type is None]
          text = src_path.read_text(encoding='utf-8')
          if dst_path.exists():
              dst_tree = core.CACHE.get_tree(dst_path)
              target = select_by_path(dst_tree, id=item.target_id) if item.target_id else None
              try:
                  new_top = _place(dst_tree, target, text, item.position)
              except BaseException:
                  core.CACHE.invalidate(dst_path)
                  raise
          else:
              if item.target_id:
                  raise core.AstError('Target file does not exist; omit targetId to create it.')
              dst_tree = core.parse_for(item.target, text)
              new_top = [loc.node_id for loc in core.locate_all(dst_tree) if loc.parent_type is None]
          try:
              imports = core.apply_imports(dst_tree, item.add_import, item.remove_import)
              _save(dst_path, dst_tree)
          except BaseException:
              core.CACHE.invalidate(dst_path)
              raise
          _remove_file(src_path)
          mapped, unassigned = _attribute([(loc.node_id, loc.name) for loc in old_top], new_top)
          source_imports = ImportOutcome(unchecked=True) if item.source_add_import or item.source_remove_import else None
          return MoveResult(
              path=item.path,
              target=item.target,
              moved=[(loc.node_id, new) for loc, new in zip(old_top, mapped)],
              unassigned=unassigned or None,
              imports=imports,
              source_imports=source_imports,
              source_deleted=True)


      def _select_sources(item: MoveItem, src_tree: core.Tree, errors: list[MoveError]) -> list[tuple[str, core.Located]]:
          """Resolve every source id; unknown ids become errors without stopping the item."""
          selected: list[tuple[str, core.Located]] = []
          seen: set[str] = set()
          for node_id in dict.fromkeys(item.ids or []):
              try:
                  loc = select_by_path(src_tree, id=node_id)
              except core.AstError as exc:
                  errors.append(MoveError(path=item.path, target=item.target, id=node_id, error=str(exc)))
                  continue
              if loc.node_id not in seen:
                  seen.add(loc.node_id)
                  selected.append((node_id, loc))
          return selected


      def _move_nodes(item: MoveItem, src_path: Path, dst_path: Path, errors: list[MoveError]) -> MoveResult | None:
          src_tree = core.CACHE.get_tree(src_path)
          selected = _select_sources(item, src_tree, errors)
          if not selected:
              return None
          roots = [(given, loc) for given, loc in selected
                   if not any(_is_inside(loc.node_id, other.node_id) for _, other in selected)]
          nested = [(given, loc) for given, loc in selected if (given, loc) not in roots]
          code = '\n'.join(core.edit_node_source(loc).rstrip('\n') for _, loc in roots)
          all_src = core.locate_all(src_tree)
          order = {loc.node_id: i for i, loc in enumerate(all_src)}
          removal = sorted((loc for _, loc in roots), key=lambda loc: order.get(loc.node_id, -1), reverse=True)
          same = _same_file(src_path, dst_path)
          try:
              if same:
                  dst_tree = src_tree
                  target = _move_within(item, src_tree, all_src, roots, removal)
              else:
                  if dst_path.exists():
                      dst_tree = core.CACHE.get_tree(dst_path)
                      target = select_by_path(dst_tree, id=item.target_id) if item.target_id else None
                  else:
                      if item.target_id:
                          raise core.AstError('Target file does not exist; omit targetId to create it.')
                      dst_tree = core.empty_tree(dst_path)
                      target = None
                  '# Delete from the source in reverse document order so the remaining nodes stay addressed.'
                  for loc in removal:
                      core.delete_node(loc)
              new_top = _place(dst_tree, target, code, item.position)
              new_all = {loc.node_id for loc in core.locate_all(dst_tree)}
              if same:
                  imports = core.apply_imports(
                      dst_tree,
                      [*(item.add_import or []), *(item.source_add_import or [])] or None,
                      [*(item.remove_import or []), *(item.source_remove_import or [])] or None)
                  source_imports = None
                  _save(dst_path, dst_tree)
                  source_deleted = False
              else:
                  imports = core.apply_imports(dst_tree, item.add_import, item.remove_import)
                  source_imports = core.apply_imports(src_tree, item.source_add_import, item.source_remove_import)
                  _save(dst_path, dst_tree)
                  source_deleted = _is_empty(src_tree)
                  if source_deleted:
                      _remove_file(src_path)
                  else:
                      _save(src_path, src_tree)
          except BaseException:
              core.CACHE.invalidate(src_path)
              core.CACHE.invalidate(dst_path)
              raise
          mapped, unassigned = _attribute([(loc.node_id, loc.name) for _, loc in roots], new_top)
          new_by_root = {loc.node_id: new for (_, loc), new in zip(roots, mapped)}
          moved: list[tuple[str, str | None]] = [(given, new) for (given, _), new in zip(roots, mapped)]
          for given, loc in nested:
              anc = next(other for _, other in roots if _is_inside(loc.node_id, other.node_id))
              anc_new = new_by_root.get(anc.node_id)
              new_id = anc_new + loc.node_id[len(anc.node_id):] if anc_new else None
              moved.append((given, new_id if new_id in new_all else None))
          return MoveResult(
              path=item.path,
              target=item.target,
              moved=moved,
              unassigned=unassigned or None,
              imports=imports,
              source_imports=source_imports,
              source_deleted=source_deleted)


      def _move_within(item: MoveItem, tree: core.Tree, all_src: list[core.Located],
                       roots: list[tuple[str, core.Located]], removal: list[core.Located]) -> core.Located | None:
          """Delete ``removal`` from ``tree`` and return the target node re-resolved afterwards."""
          target = select_by_path(tree, id=item.target_id) if item.target_id else None
          root_ids = [loc.node_id for _, loc in roots]
          if target is not None and any(target.node_id == r or _is_inside(target.node_id, r) for r in root_ids):
              raise core.AstError('The target node is (inside) a moved node.')
          remaining = [loc for loc in all_src if not any(loc.node_id == r or _is_inside(loc.node_id, r) for r in root_ids)]
          position = next((i for i, loc in enumerate(remaining) if target and loc.node_id == target.node_id), None)
          for loc in removal:
              core.delete_node(loc)
          if target is None:
              return None
          after = core.locate_all(tree)
          '# Deleting keeps the document order, so the target keeps its index among the remaining nodes.'
          if position is not None and len(after) == len(remaining):
              return after[position]
          return select_by_path(tree, id=target.node_id)


      def _move_one(item: MoveItem, errors: list[MoveError]) -> MoveResult | None:
          src_path = core.require_path(item.path)
          dst_path = core.require_path(item.target, must_exist=False)
          _parse_position(item.position)
          if not item.ids:
              return _move_file(item, src_path, dst_path)
          return _move_nodes(item, src_path, dst_path, errors)


      def ast_move(items: list[MoveItem]) -> MoveBatchResult:
          """Move nodes between or within files, or whole files.

          Nodes are inserted into the target (relative to ``target_id`` or appended at its
          end) and deleted from the source; without ids the whole source file is moved.
          A missing target file is created, a source file left empty is deleted. An
          unknown source id is reported as an error but the other ids are still moved;
          an unknown ``target_id`` aborts the item. Nothing is written for an aborted item.

          Args:
              items: Move operations to apply in order. Must be non-empty.

          Returns:
              MoveBatchResult: One result per item that moved something, one error per
              failed item or unresolvable source id.

          Raises:
              core.AstError: If ``items`` is empty.
          """
          if not items:
              raise core.AstError("'items' must be a non-empty list.")
          results: list[MoveResult] = []
          errors: list[MoveError] = []
          for item in items:
              item_errors: list[MoveError] = []
              try:
                  result = _move_one(item, item_errors)
                  if result is not None:
                      results.append(result)
              except core.AstError as exc:
                  item_errors.append(MoveError(path=item.path, target=item.target, error=str(exc)))
              errors.extend(item_errors)
          return MoveBatchResult(results=results, errors=errors)


      def _string_list(it: dict[str, Any], key: str) -> list[str] | None:
          value = it.get(key)
          if value is None:
              return None
          if not isinstance(value, list) or not all(isinstance(v, str) and v.strip() for v in value):
              raise ValueError(f"'{key}' must be a list of non-empty strings")
          return value or None


      class MoveTool(ToolDefinition):
          name = 'ast_move'
          features = frozenset({ToolFeature.INPUT_ASTID})
          title = 'Move AST nodes or files'
          description = (
              'Move nodes between or within files (insert & delete), or whole files if no ids are given '
              '(create/insert & delete), for a batch of items. A missing target file is created, an emptied '
              'source file is deleted. Returns the old and new id of every moved node.')
          _ITEM_PROPERTIES = {
              'path': {'type': 'string', 'description': 'Absolute path of the source file (never directories).'},
              'target': {
                  'type': 'string',
                  'description': 'Absolute path of the target file; may equal path to move within the file. Created if missing.'},
              'ids': {
                  'type': 'array',
                  'items': {'type': 'string', 'minLength': 1},
                  'description': ('Ids of the source nodes to move, inserted in this order. Omit or leave empty to move the '
                                  'whole file. To move a container (e.g. an inner class) give its own id; listing only its '
                                  'children leaves the container in place.')},
              'targetId': {
                  'type': 'string',
                  'description': 'Id of the node in the target file to insert relative to. Omit to append at the end of the target file.'},
              'position': {
                  'type': 'string',
                  'enum': ['before', 'after'],
                  'description': 'Placement relative to targetId.',
                  'default': 'after'},
              'addImport': {**IMPORT_PROPS['addImport'], 'description': IMPORT_PROPS['addImport']['description'] + ' Applied to the target file.'},
              'removeImport': {**IMPORT_PROPS['removeImport'], 'description': IMPORT_PROPS['removeImport']['description'] + ' Applied to the target file.'},
              'sourceAddImport': {**IMPORT_PROPS['addImport'], 'description': 'Import statements to add to the source file if missing.'},
              'sourceRemoveImport': {**IMPORT_PROPS['removeImport'], 'description': 'Import statements the source file no longer needs.'}}
          _ITEM_REQUIRED = ['path', 'target']
          _ITEMS_DESCRIPTION = 'Move operations to apply.'
          input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)

          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`ast_move`, translating the MCP schema to/from the Python API."""

              def item_factory(it: dict[str, Any]) -> MoveItem:
                  add_import, remove_import = parse_import_args(it)
                  target_id = it.get('targetId')
                  if target_id is not None and not isinstance(target_id, str):
                      raise ValueError("'targetId' must be a string")
                  return MoveItem(
                      path=it['path'],
                      target=it['target'],
                      ids=_string_list(it, 'ids'),
                      target_id=target_id or None,
                      position=it.get('position', 'after'),
                      add_import=add_import,
                      remove_import=remove_import,
                      source_add_import=_string_list(it, 'sourceAddImport'),
                      source_remove_import=_string_list(it, 'sourceRemoveImport'))
              return handle_batch_tool(ctx, item_factory, ast_move, core.AstError, auto_approve=True)


      def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
          registry.register(MoveTool())
          functions.register(ast_move)
  reason: Neues Tool ast_move als Modul move.py anlegen
```
/answer 1f37-20c-0 allow
Thinking:
Das Modul steht. Als Nächstes stelle ich den Vergleich der `nested`-Liste von tiefem `==` auf Identitätsvergleich um.


ReasoningToken: 409
Control Request:
```yaml
id: 1f37-20d-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    text: nested =
  reason: "Aktuellen Stand von _move_nodes nach dem Speichern (ggf. reformatiert) lesen, um die nested-Berechnung gezielt zu ändern"
```
/answer 1f37-20d-0 allow
Control Request:
```yaml
id: 1f37-20e-20d
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
        nodes:
        - id: move_nodes.UOhMcX|Q3wVBx
          code: |-
            src_tree = core.CACHE.get_tree(src_path)
            selected = _select_sources(item, src_tree, errors)
            if not selected:
                return None
            roots = [(given, loc) for given, loc in selected if not any(
                (_is_inside(loc.node_id, other.node_id) for _, other in selected))]
            nested = [(given, loc) for given, loc in selected if (given, loc) not in roots]
            code = '\n'.join((core.edit_node_source(loc).rstrip('\n') for _, loc in roots))
            all_src = core.locate_all(src_tree)
```
/answer 1f37-20e-20d allow
Control Request:
```yaml
id: 1f37-20f-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: move_nodes.UOhMcX|Q3wVBx
    old_text: "nested = [(given, loc) for given, loc in selected if (given, loc) not in roots]"
    new_text: |-
      root_ids = {loc.node_id for _, loc in roots}
      nested = [(given, loc) for given, loc in selected if loc.node_id not in root_ids]
  reason: nested-Berechnung über node_id statt tiefem Dataclass-Vergleich
```
/answer 1f37-20f-0 allow
Text:
 Jetzt binde ich das Tool in `edit.py` und `__init__.py` ein.
Control Request:
```yaml
id: 1f37-210-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit.py
    id: 14h53u|AeVQqA
    old_text: "('edit_marks', EditMarksNodeTool()), ('delete', DeleteTool()))"
    new_text: "('edit_marks', EditMarksNodeTool()), ('move', MoveTool()), ('delete', DeleteTool()))"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit.py
    id: neazUj|4NSldY
    old_text: from xy.ai.mcpc.tools.ast.insert import InsertNodeTool
    new_text: |-
      from xy.ai.mcpc.tools.ast.insert import InsertNodeTool
      from xy.ai.mcpc.tools.ast.move import MoveTool
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit.py
    id: EditTool.oy2a69|JHgl3f
    old_text: "Run create, insert, replace, edit_block, edit_marks and delete operations"
    new_text: "Run create, insert, replace, edit_block, edit_marks, move and delete operations"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
    id: ukjPdF|CoBg9W
    old_text: "find, insert, read, replace, retrieve, script, validate"
    new_text: "find, insert, move, read, replace, retrieve, script, validate"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
    id: mADxqb|yRFYn0
    old_text: |-
      'ast_delete',
          'ast_create',
    new_text: |-
      'ast_delete',
          'ast_create',
          'ast_move',
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
    id: register_ast_tools
    old_text: |2
          create.register(registry, functions)
    new_text: |2
          create.register(registry, functions)
          move.register(registry, functions)
  reason: ast_move in ast_edit (als Schritt vor delete) und in die Registrierung/Alias der AST-Tools einbinden
```
/answer 1f37-210-0 allow
Text:
 Jetzt lege ich Tests an, die zur bestehenden Test-Suite passen.
ReasoningToken: 318
Control Request:
```yaml
id: 1f37-211-0
toolName: ast_edit
arguments:
  create:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_move.py
    source: |
      """Tests for the ``ast_move`` tool."""
      from __future__ import annotations

      import sys
      from pathlib import Path

      _SRC = Path(__file__).resolve().parents[1] / 'src'
      if _SRC.is_dir() and str(_SRC) not in sys.path:
          sys.path.insert(0, str(_SRC))

      from xy.ai.mcpc.tools.ast import core  # noqa: E402
      from xy.ai.mcpc.tools.ast.edit import EditTool  # noqa: E402
      from xy.ai.mcpc.tools.ast.move import MoveItem, MoveTool, ast_move  # noqa: E402

      SRC = 'import os\n\n\ndef foo():\n    return os.sep\n\n\ndef bar():\n    return 1\n'
      CLASSES = (
          'class A:\n\n    def m(self):\n        return 1\n\n    def n(self):\n        return 2\n\n'
          '    class Inner:\n\n        def x(self):\n            return 3\n\n\n'
          'class B:\n\n    def k(self):\n        return 4\n')


      def _write(tmp_path: Path, name: str, source: str) -> str:
          path = tmp_path / name
          path.parent.mkdir(parents=True, exist_ok=True)
          path.write_text(source, encoding='utf-8')
          return str(path)


      def _ids(path: str) -> list[str]:
          return [loc.node_id for loc in core.locate_all(core.CACHE.get_tree(Path(path)))]


      def test_move_to_new_file_creates_it_and_handles_imports(tmp_path: Path):
          src = _write(tmp_path, 'a.py', SRC)
          dst = str(tmp_path / 'sub' / 'b.py')
          result = ast_move([MoveItem(path=src, target=dst, ids=['foo'], add_import=['import os'])])
          assert not result.errors
          (res,) = result.results
          assert res.moved == [('foo', 'foo')]
          text = Path(dst).read_text()
          assert 'def foo' in text and 'import os' in text
          assert 'def foo' not in Path(src).read_text()
          assert res.to_dict()['moved'] == [{'old': 'foo', 'new': 'foo'}]
          assert res.to_dict()['target'] == dst


      def test_emptied_source_file_is_deleted(tmp_path: Path):
          src = _write(tmp_path, 'a.py', SRC)
          dst = _write(tmp_path, 'b.py', 'X = 1\n')
          result = ast_move([MoveItem(path=src, target=dst, ids=['foo', 'bar'], source_remove_import=['import os'])])
          assert not result.errors
          assert result.results[0].source_deleted
          assert not Path(src).exists()
          text = Path(dst).read_text()
          assert text.index('def foo') < text.index('def bar')


      def test_unknown_source_id_is_error_but_others_move(tmp_path: Path):
          src = _write(tmp_path, 'a.py', SRC)
          dst = _write(tmp_path, 'b.py', 'X = 1\n')
          result = ast_move([MoveItem(path=src, target=dst, ids=['nope', 'bar'])])
          assert [e.id for e in result.errors] == ['nope']
          assert result.results[0].moved == [('bar', 'bar')]
          assert 'def bar' in Path(dst).read_text()


      def test_unknown_target_id_aborts_without_writing(tmp_path: Path):
          src = _write(tmp_path, 'a.py', SRC)
          dst = _write(tmp_path, 'b.py', 'X = 1\n')
          result = ast_move([MoveItem(path=src, target=dst, ids=['foo'], target_id='missing')])
          assert not result.results
          assert len(result.errors) == 1 and result.errors[0].id is None
          assert Path(src).read_text() == SRC
          assert Path(dst).read_text() == 'X = 1\n'
          assert 'foo' in _ids(src)


      def test_move_method_between_classes_in_same_file(tmp_path: Path):
          path = _write(tmp_path, 'c.py', CLASSES)
          result = ast_move([MoveItem(path=path, target=path, ids=['A.m'], target_id='B.k', position='before')])
          assert not result.errors
          assert result.results[0].moved == [('A.m', 'B.m')]
          ids = _ids(path)
          assert 'A.m' not in ids
          assert ids.index('B.m') < ids.index('B.k')


      def test_target_inside_moved_node_is_rejected(tmp_path: Path):
          path = _write(tmp_path, 'c.py', CLASSES)
          result = ast_move([MoveItem(path=path, target=path, ids=['A'], target_id='A.n')])
          assert result.errors and not result.results
          assert Path(path).read_text() == CLASSES


      def test_inner_class_moves_by_its_own_id(tmp_path: Path):
          path = _write(tmp_path, 'c.py', CLASSES)
          dst = str(tmp_path / 'd.py')
          result = ast_move([MoveItem(path=path, target=dst, ids=['A.Inner'])])
          assert not result.errors
          assert result.results[0].moved == [('A.Inner', 'Inner')]
          assert 'A.Inner' not in _ids(path)
          assert 'Inner.x' in _ids(dst)


      def test_moving_only_children_keeps_container(tmp_path: Path):
          path = _write(tmp_path, 'c.py', CLASSES)
          dst = str(tmp_path / 'd.py')
          result = ast_move([MoveItem(path=path, target=dst, ids=['A.Inner.x'])])
          assert not result.errors
          assert 'A.Inner' in _ids(path)
          assert 'x' in _ids(dst)


      def test_nested_id_moves_along_with_ancestor(tmp_path: Path):
          path = _write(tmp_path, 'c.py', CLASSES)
          dst = str(tmp_path / 'd.py')
          result = ast_move([MoveItem(path=path, target=dst, ids=['A.Inner.x', 'A.Inner'])])
          assert not result.errors
          assert dict(result.results[0].moved) == {'A.Inner': 'Inner', 'A.Inner.x': 'Inner.x'}


      def test_whole_file_to_new_target(tmp_path: Path):
          src = _write(tmp_path, 'pkg/a.py', SRC)
          dst = str(tmp_path / 'other' / 'b.py')
          result = ast_move([MoveItem(path=src, target=dst)])
          assert not result.errors
          res = result.results[0]
          assert res.source_deleted
          assert not Path(src).exists() and not Path(src).parent.exists()
          assert 'def foo' in Path(dst).read_text()
          assert ('foo', 'foo') in res.moved


      def test_whole_file_into_existing_target_after_node(tmp_path: Path):
          src = _write(tmp_path, 'a.py', 'def foo():\n    return 1\n')
          dst = _write(tmp_path, 'b.py', 'def first():\n    pass\n\n\ndef last():\n    pass\n')
          result = ast_move([MoveItem(path=src, target=dst, target_id='first')])
          assert not result.errors
          assert result.results[0].moved == [('foo', 'foo')]
          assert _ids(dst) == ['first', 'foo', 'last']
          assert not Path(src).exists()


      def test_whole_file_onto_itself_fails(tmp_path: Path):
          src = _write(tmp_path, 'a.py', SRC)
          result = ast_move([MoveItem(path=src, target=src)])
          assert result.errors and Path(src).read_text() == SRC


      def test_javascript_cross_file_move(tmp_path: Path):
          src = _write(tmp_path, 'a.js', 'function foo() {\n  return 1;\n}\n\nfunction bar() {\n  return 2;\n}\n')
          dst = _write(tmp_path, 'b.js', 'function baz() {\n  return 3;\n}\n')
          result = ast_move([MoveItem(path=src, target=dst, ids=[i for i in _ids(src) if 'bar' in i][:1])])
          assert not result.errors
          assert 'function bar' in Path(dst).read_text()
          assert 'function bar' not in Path(src).read_text()
          assert result.results[0].moved[0][1] is not None


      def test_batch_and_edit_integration(tmp_path: Path):
          src = _write(tmp_path, 'a.py', SRC)
          dst = str(tmp_path / 'b.py')
          assert 'move' in EditTool.input_schema['properties']
          assert EditTool.input_schema['properties']['move'] == MoveTool.input_schema['properties']['items']
          result = ast_move([
              MoveItem(path=src, target=dst, ids=['foo']),
              MoveItem(path=src, target=dst, ids=['bar'], target_id='foo', position='before'),
          ])
          assert not result.errors
          assert _ids(dst)[:2] == ['bar', 'foo']
  reason: "Tests für ast_move anlegen (Datei-übergreifend, innerhalb einer Datei, ganze Datei, Fehlerfälle, Imports, ast_edit-Integration)"
```
/answer 1f37-211-0 allow
Text:
 Jetzt führe ich die Tests aus.
ReasoningToken: 104
Control Request:
```yaml
id: 1f37-212-0
toolName: run_command
arguments:
  items:
  - id: pytest
    cwd: /home/user/xyan/xy.ai.workbench/mcpc
    values:
    - name: args
      value: tests/test_ast_move.py -q
  reason: Neue Tests für ast_move ausführen
```
/answer 1f37-212-0 allow
Control Request:
```yaml
id: 1f37-213-212
result:
  structuredContent:
    errors:
    - index: 0
      error: Unknown command id 'pytest'.
```
/answer 1f37-213-212 allow
Control Request:
```yaml
id: 1f37-214-0
toolName: ask_user
arguments:
  question: Ich habe ast_move implementiert und Tests in tests/test_ast_move.py angelegt. Welche Command-ID im MCPC run_command darf ich zum Ausführen der Tests verwenden (z. B. für pytest) und wie heißen ihre Platzhalter? 'pytest' ist unbekannt.
  reason: "Zum Verifizieren der Implementierung muss ich die Tests ausführen, kenne aber die vordefinierten Command-IDs nicht"
```
/answer 1f37-214-0 allow "runTests" ohne Platzhalter 
Control Request:
```yaml
id: 1f37-215-0
toolName: run_command
arguments:
  items:
  - id: runTests
    cwd: /home/user/xyan/xy.ai.workbench/mcpc
  reason: Testsuite inkl. neuer ast_move-Tests ausführen
```
/answer 1f37-215-0 allow
Control Request:
```yaml
id: 1f37-216-215
result:
  structuredContent:
    results:
    - index: 0
      exit_code: 1
      stdout: |
        ============================= test session starts ==============================
        platform linux -- Python 3.12.3, pytest-8.3.5, pluggy-1.6.0
        rootdir: /home/user/xyan/xy.ai.workbench/mcpc
        configfile: pyproject.toml
        plugins: anyio-4.8.0, langsmith-0.4.18, docker-3.1.2
        collected 500 items

        tests/test_ast_engines.py ..................                             [  3%]
        tests/test_ast_imports.py ....................                           [  7%]
        tests/test_ast_imports_rpc.py ...........                                [  9%]
        tests/test_ast_markdown_bridge.py ...................................... [ 17%]
        ..................................................F                      [ 27%]
        tests/test_ast_move.py .......F.FF..F                                    [ 30%]
        tests/test_ast_retrieve.py ......                                        [ 31%]
        tests/test_ast_rust_bridge.py ....                                       [ 32%]
        tests/test_cli_session.py .................                              [ 35%]
        tests/test_edit_diagnostics.py .......................                   [ 40%]
        tests/test_edit_tolerance.py ...........................                 [ 45%]
        tests/test_embedded.py .............                                     [ 48%]
        tests/test_embedded_batch.py .                                           [ 48%]
        tests/test_grep_with_ast.py ...                                          [ 49%]
        tests/test_http_human_in_the_loop.py ..                                  [ 49%]
        tests/test_http_tool_calls.py ...                                        [ 50%]
        tests/test_import_all.py ............................................... [ 59%]
        ........................................................................ [ 74%]
        .......                                                                  [ 75%]
        tests/test_mcpcignore.py ........................                        [ 80%]
        tests/test_persist.py ..........                                         [ 82%]
        tests/test_result_contract.py ..................................         [ 89%]
        tests/test_rpc_java_engine.py ..........                                 [ 91%]
        tests/test_script_file_source.py ..............................          [ 97%]
        tests/test_smoke.py ..                                                   [ 97%]
        tests/test_tools_family_smoke.py ...                                     [ 98%]
        tests/test_validate_arguments.py ..........                              [100%]

        =================================== FAILURES ===================================
        __________ test_existing_engine_instance_recovers_after_process_crash __________
        [XPASS(strict)] RpcEngine caches its client and only starts the process when the client is created, so an already used engine instance (e.g. from get_engine) never recovers from a crashed process.
        __________________ test_moving_only_children_keeps_container ___________________

        tmp_path = PosixPath('/tmp/pytest-of-user/pytest-0/test_moving_only_children_keep0')

            def test_moving_only_children_keeps_container(tmp_path: Path):
                path = _write(tmp_path, 'c.py', CLASSES)
                dst = str(tmp_path / 'd.py')
                result = ast_move([MoveItem(path=path, target=dst, ids=['A.Inner.x'])])
        >       assert not result.errors
        E       AssertionError: assert not [MoveError(error='Syntax error: expected an indented block after class definition on line 9 (line 11)', path='/tmp/pyt...moving_only_children_keep0/c.py', target='/tmp/pytest-of-user/pytest-0/test_moving_only_children_keep0/d.py', id=None)]
        E        +  where [MoveError(error='Syntax error: expected an indented block after class definition on line 9 (line 11)', path='/tmp/pyt...moving_only_children_keep0/c.py', target='/tmp/pytest-of-user/pytest-0/test_moving_only_children_keep0/d.py', id=None)] = MoveBatchResult(results=[], errors=[MoveError(error='Syntax error: expected an indented block after class definition o...ildren_keep0/c.py', target='/tmp/pytest-of-user/pytest-0/test_moving_only_children_keep0/d.py', id=None)], warnings=()).errors

        tests/test_ast_move.py:95: AssertionError
        ________________________ test_whole_file_to_new_target _________________________

        tmp_path = PosixPath('/tmp/pytest-of-user/pytest-0/test_whole_file_to_new_target0')

            def test_whole_file_to_new_target(tmp_path: Path):
                src = _write(tmp_path, 'pkg/a.py', SRC)
                dst = str(tmp_path / 'other' / 'b.py')
                result = ast_move([MoveItem(path=src, target=dst)])
                assert not result.errors
                res = result.results[0]
                assert res.source_deleted
                assert not Path(src).exists() and (not Path(src).parent.exists())
                assert 'def foo' in Path(dst).read_text()
        >       assert ('foo', 'foo') in res.moved
        E       AssertionError: assert ('foo', 'foo') in []
        E        +  where [] = MoveResult(warnings=(), path='/tmp/pytest-of-user/pytest-0/test_whole_file_to_new_target0/pkg/a.py', target='/tmp/pyte...ole_file_to_new_target0/other/b.py', moved=[], unassigned=None, imports=None, source_imports=None, source_deleted=True).moved

        tests/test_ast_move.py:115: AssertionError
        _______________ test_whole_file_into_existing_target_after_node ________________

        tmp_path = PosixPath('/tmp/pytest-of-user/pytest-0/test_whole_file_into_existing_0')

            def test_whole_file_into_existing_target_after_node(tmp_path: Path):
                src = _write(tmp_path, 'a.py', 'def foo():\n    return 1\n')
                dst = _write(tmp_path, 'b.py', 'def first():\n    pass\n\n\ndef last():\n    pass\n')
                result = ast_move([MoveItem(path=src, target=dst, target_id='first')])
                assert not result.errors
        >       assert result.results[0].moved == [('foo', 'foo')]
        E       AssertionError: assert [] == [('foo', 'foo')]
        E
        E         Right contains one more item: ('foo', 'foo')
        E         Use -v to get more diff

        tests/test_ast_move.py:122: AssertionError
        _______________________ test_batch_and_edit_integration ________________________

        tmp_path = PosixPath('/tmp/pytest-of-user/pytest-0/test_batch_and_edit_integratio0')

            def test_batch_and_edit_integration(tmp_path: Path):
                src = _write(tmp_path, 'a.py', SRC)
                dst = str(tmp_path / 'b.py')
                assert 'move' in EditTool.input_schema['properties']
                assert EditTool.input_schema['properties']['move'] == MoveTool.input_schema['properties']['items']
                result = ast_move([MoveItem(path=src, target=dst, ids=['foo']), MoveItem(
                    path=src, target=dst, ids=['bar'], target_id='foo', position='before')])
                assert not result.errors
        >       assert _ids(dst)[:2] == ['bar', 'foo']
        E       AssertionError: assert ['bar', 'bar.j8A4NW|NfE1bm'] == ['bar', 'foo']
        E
        E         At index 1 diff: 'bar.j8A4NW|NfE1bm' != 'foo'
        E         Use -v to get more diff

        tests/test_ast_move.py:148: AssertionError
        =========================== short test summary info ============================
        FAILED tests/test_ast_markdown_bridge.py::test_existing_engine_instance_recovers_after_process_crash
        FAILED tests/test_ast_move.py::test_moving_only_children_keeps_container - As...
        FAILED tests/test_ast_move.py::test_whole_file_to_new_target - AssertionError...
        FAILED tests/test_ast_move.py::test_whole_file_into_existing_target_after_node
        FAILED tests/test_ast_move.py::test_batch_and_edit_integration - AssertionErr...
        ======================== 5 failed, 495 passed in 14.31s ========================
```
/answer 1f37-216-215 allow
Thinking:
495 Tests bestanden, 5 fehlgeschlagen; ich prüfe jetzt den Fehler im XPASS-Strict-Test und passe die Behandlung von Top-Level-Knoten, das `pass`-Einfügen in PythonEngine.delete und die Filterung der ID-Vergleiche an.


ReasoningToken: 804
Control Request:
```yaml
id: 1f37-217-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    text: parent_type is None
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
    regexp: ^import ast|^from __future__
  reason: Aktuelle IDs der zu ändernden Stellen in move.py und Python-Engine-Imports ermitteln
```
/answer 1f37-217-0 allow
Control Request:
```yaml
id: 1f37-218-217
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
        nodes:
        - id: move_file.G09txE|L19gPb
          code: |-
            if _same_file(src_path, dst_path):
                raise core.AstError('Moving a whole file onto itself is not possible; give source ids.')
            src_tree = core.CACHE.get_tree(src_path)
            old_top = [loc for loc in core.locate_all(src_tree) if loc.parent_type is None]
            text = src_path.read_text(encoding='utf-8')
        - id: move_file.A9WCiK|2R5207
          code: |-
            if dst_path.exists():
                dst_tree = core.CACHE.get_tree(dst_path)
                target = select_by_path(dst_tree, id=item.target_id) if item.target_id else None
                try:
                    new_top = _place(dst_tree, target, text, item.position)
                except BaseException:
                    core.CACHE.invalidate(dst_path)
                    raise
            else:
                if item.target_id:
                    raise core.AstError('Target file does not exist; omit targetId to create it.')
                dst_tree = core.parse_for(item.target, text)
                new_top = [loc.node_id for loc in core.locate_all(dst_tree) if loc.parent_type is None]
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
        nodes: []
```
/answer 1f37-218-217 allow
Control Request:
```yaml
id: 1f37-219-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: move_file.G09txE|L19gPb
    old_text: "old_top = [loc for loc in core.locate_all(src_tree) if loc.parent_type is None]"
    new_text: |-
      src_all = core.locate_all(src_tree)
      top_ids = set(_top_level([loc.node_id for loc in src_all]))
      old_top = [loc for loc in src_all if loc.node_id in top_ids]
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: move_file.A9WCiK|2R5207
    old_text: "new_top = [loc.node_id for loc in core.locate_all(dst_tree) if loc.parent_type is None]"
    new_text: "new_top = _top_level([loc.node_id for loc in core.locate_all(dst_tree)])"
  reason: Top-Level-Erkennung beim Verschieben ganzer Dateien über die ID-Hierarchie statt parent_type
```
/answer 1f37-219-0 allow
Control Request:
```yaml
id: 1f37-21a-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python
    pattern: ^import |^from |ast\.Pass
    include:
    - _engine.py
    limit: 30
  reason: Imports der Python-Engine prüfen (ast-Modul verfügbar?) für die pass-Ergänzung in delete
```
/answer 1f37-21a-0 allow
Control Request:
```yaml
id: 1f37-21b-21a
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python
        matches:
        - path: _engine.py
          matches:
          - from __future__ import annotations
          - import ast
          - import autopep8
          - import logging
          - from pathlib import Path
          - from typing import Any
          - "from xy.ai.mcpc.tools.ast.base import AstError, Engine, Located, SEGMENT_MAX_CHARS, Tree, check_no_control_chars, id_segment"
          - from xy.ai.mcpc.tools.ast.imports import ImportOutcome
          - from xy.ai.mcpc.tools.ast.python._comments import comments_to_annotations
          - from xy.ai.mcpc.tools.ast.python._imports import apply_python_imports
          - "from xy.ai.mcpc.tools.ast.python._nodes import _DEF_TYPES, _IMPORT_TYPES, _StatementGroup, _decorators, _is_expandable"
```
/answer 1f37-21b-21a allow
ReasoningToken: 116
Control Request:
```yaml
id: 1f37-21c-0
toolName: ast_edit
arguments:
  replace:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
    id: PythonEngine.delete
    source: |-
      def delete(self, loc: Located) -> None:
          node = loc.node
          if isinstance(node, _StatementGroup):
              parent = node.parent
              del parent.body[node.start:node.stop]
          else:
              parent = loc.parent
              del parent.body[loc.index]
          # A compound statement (class/def/...) must keep a non-empty body.
          if not parent.body and not isinstance(parent, ast.Module):
              parent.body.append(ast.Pass())
  reason: "PythonEngine.delete: geleerten Block-Body (Klasse/Funktion) mit pass gültig halten, damit beim Verschieben aller Kinder der Container erhalten bleibt"
```
/answer 1f37-21c-0 allow
Text:
 Jetzt passe ich die Tests an, sodass sie nur Top-Level-IDs vergleichen.
Control Request:
```yaml
id: 1f37-21d-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_move.py
    regexp: "_ids\\(dst\\) ==|_ids\\(dst\\)\\[:2\\]|^def _ids"
  reason: IDs der zu ändernden Testfunktionen ermitteln
```
/answer 1f37-21d-0 allow
Control Request:
```yaml
id: 1f37-21e-21d
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_move.py
        nodes:
        - id: h6d2ef6fa.j3LbN6|tFY4un
          code: |-
            src = _write(tmp_path, 'a.py', 'def foo():\n    return 1\n')
            dst = _write(tmp_path, 'b.py', 'def first():\n    pass\n\n\ndef last():\n    pass\n')
            result = ast_move([MoveItem(path=src, target=dst, target_id='first')])
            assert not result.errors
            assert result.results[0].moved == [('foo', 'foo')]
            assert _ids(dst) == ['first', 'foo', 'last']
            assert not Path(src).exists()
        - id: test_batch_and_edit_integration.P6gP7N|XpKtcg
          code: |-
            src = _write(tmp_path, 'a.py', SRC)
            dst = str(tmp_path / 'b.py')
            assert 'move' in EditTool.input_schema['properties']
            assert EditTool.input_schema['properties']['move'] == MoveTool.input_schema['properties']['items']
            result = ast_move([MoveItem(path=src, target=dst, ids=['foo']), MoveItem(
                path=src, target=dst, ids=['bar'], target_id='foo', position='before')])
            assert not result.errors
            assert _ids(dst)[:2] == ['bar', 'foo']
```
/answer 1f37-21e-21d allow
Control Request:
```yaml
id: 1f37-21f-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_move.py
    id: h6d2ef6fa.j3LbN6|tFY4un
    old_text: "assert _ids(dst) == ['first', 'foo', 'last']"
    new_text: "assert _top_ids(dst) == ['first', 'foo', 'last']"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_move.py
    id: test_batch_and_edit_integration.P6gP7N|XpKtcg
    old_text: "assert _ids(dst)[:2] == ['bar', 'foo']"
    new_text: "assert _top_ids(dst) == ['bar', 'foo']"
  insert:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_move.py
    id: ids
    position: after
    source: |-
      def _top_ids(path: str) -> list[str]:
          return [i for i in _ids(path) if '.' not in i]
  reason: Tests auf Top-Level-IDs umstellen (Kindknoten werden von locate_all mitgeliefert)
```
/answer 1f37-21f-0 allow
ReasoningToken: 86
Control Request:
```yaml
id: 1f37-220-0
toolName: run_command
arguments:
  items:
  - id: runTests
    cwd: /home/user/xyan/xy.ai.workbench/mcpc
  reason: Testsuite nach den Korrekturen erneut ausführen
```
/answer 1f37-220-0 allow
Control Request:
```yaml
id: 1f37-221-220
result:
  structuredContent:
    results:
    - index: 0
      exit_code: 1
      stdout: |
        ============================= test session starts ==============================
        platform linux -- Python 3.12.3, pytest-8.3.5, pluggy-1.6.0
        rootdir: /home/user/xyan/xy.ai.workbench/mcpc
        configfile: pyproject.toml
        plugins: anyio-4.8.0, langsmith-0.4.18, docker-3.1.2
        collected 500 items

        tests/test_ast_engines.py ..................                             [  3%]
        tests/test_ast_imports.py ....................                           [  7%]
        tests/test_ast_imports_rpc.py ...........                                [  9%]
        tests/test_ast_markdown_bridge.py ...................................... [ 17%]
        ..................................................F                      [ 27%]
        tests/test_ast_move.py ..............                                    [ 30%]
        tests/test_ast_retrieve.py ......                                        [ 31%]
        tests/test_ast_rust_bridge.py ....                                       [ 32%]
        tests/test_cli_session.py .................                              [ 35%]
        tests/test_edit_diagnostics.py .......................                   [ 40%]
        tests/test_edit_tolerance.py ...........................                 [ 45%]
        tests/test_embedded.py .............                                     [ 48%]
        tests/test_embedded_batch.py .                                           [ 48%]
        tests/test_grep_with_ast.py ...                                          [ 49%]
        tests/test_http_human_in_the_loop.py ..                                  [ 49%]
        tests/test_http_tool_calls.py ...                                        [ 50%]
        tests/test_import_all.py ............................................... [ 59%]
        ........................................................................ [ 74%]
        .......                                                                  [ 75%]
        tests/test_mcpcignore.py ........................                        [ 80%]
        tests/test_persist.py ..........                                         [ 82%]
        tests/test_result_contract.py ..................................         [ 89%]
        tests/test_rpc_java_engine.py ..........                                 [ 91%]
        tests/test_script_file_source.py ..............................          [ 97%]
        tests/test_smoke.py ..                                                   [ 97%]
        tests/test_tools_family_smoke.py ...                                     [ 98%]
        tests/test_validate_arguments.py ..........                              [100%]

        =================================== FAILURES ===================================
        __________ test_existing_engine_instance_recovers_after_process_crash __________
        [XPASS(strict)] RpcEngine caches its client and only starts the process when the client is created, so an already used engine instance (e.g. from get_engine) never recovers from a crashed process.
        =========================== short test summary info ============================
        FAILED tests/test_ast_markdown_bridge.py::test_existing_engine_instance_recovers_after_process_crash
        ======================== 1 failed, 499 passed in 16.12s ========================
```
/answer 1f37-221-220 allow
Result Stats: id=71fc1cac-d49d-4728-bb65-73d1d1769780, total: 146871, in: 62, out: 42211, reason: 97, read: 1999303, write: 104598, start: 1795553938130, end: 1795553953167, time: 14987, total: 475250, io: 0, control: 50, sent: 0, recv: 256
ReasoningToken: 97
`ast_move` ist fertig und funktioniert auch innerhalb von `ast_edit`. Die neuen Tests in `tests/test_ast_move.py` laufen alle durch, insgesamt bestehen 499 von 500 Tests. Der eine Fehlschlag ist `test_ast_markdown_bridge.py::test_existing_engine_instance_recovers_after_process_crash`: Ein mit strict markierter Erwartungs-Fehlschlag läuft plötzlich durch (XPASS). Er betrifft das Neustarten der RPC-Engine, und daran habe ich nichts geändert.

**Eingabe pro Item:**
- `path` (Quelle) und `target` (Ziel). Beide dürfen gleich sein, dann wird innerhalb der Datei verschoben.
- `ids`: Leer oder weggelassen heißt, die ganze Datei wird verschoben.
- `targetId` und `position` (`before`/`after`, Standard `after`). Ohne `targetId` wird ans Dateiende angehängt.
- `addImport`/`removeImport` gelten für die Zieldatei, `sourceAddImport`/`sourceRemoveImport` für die Quelldatei.

**Verhalten:**
- Eine nicht gefundene Quell-ID erzeugt einen eigenen Fehler mit dieser `id`, die übrigen Knoten werden trotzdem verschoben.
- Eine nicht gefundene `targetId` bricht das Item ab, und es wird nichts geschrieben. Ebenso abgebrochen wird, wenn das Ziel in einem der verschobenen Knoten liegt.
- Eine fehlende Zieldatei wird wie bei Create angelegt, samt fehlender Verzeichnisse.
- Eine dadurch leere Quelldatei wird wie bei Delete entfernt, ein leeres Verzeichnis gleich mit.
- Eine innere Klasse verschiebst du über ihre eigene ID. Nennst du nur Kinder, bleibt die Klasse stehen. Nennst du Eltern und Kind zusammen, wandert das Kind mit dem Elternknoten.
- Mehrere Knoten werden in der angegebenen Reihenfolge eingefügt.
- Rückgabe: Quelle, Ziel und `moved: [{old, new}]`, keine Outline. Dazu kommen bei Bedarf `imports`, `sourceImports` und `sourceDeleted`. Lässt sich eine neue ID nicht eindeutig zuordnen, fehlt `new`, und die übrigen neuen IDs stehen unter `unassigned`.

**Geänderte Dateien:**
- **`move.py` (neu):** das Tool selbst.
- **`edit.py`:** neuer Abschnitt `move`, der nach `edit_marks` und vor `delete` läuft.
- **`__init__.py`:** Registrierung und Aufnahme in den Alias `ast`.
- **`python/_engine.py`:** Wird ein Klassen- oder Funktions-Body durch Löschen leer, setzt `PythonEngine.delete` jetzt `pass` ein. Ohne das entstand ein Syntaxfehler, wenn man nur die Kinder verschiebt. Diese Änderung betrifft auch `ast_delete`.

**Einschränkungen:**
- **Einrückung:** Bei Tree-sitter- und RPC-Sprachen (z. B. JS, Java) wird der Knotentext unverändert eingefügt, wie bei `ast_insert`. Wechselt ein Knoten dabei die Verschachtelungstiefe, wird die Einrückung nicht angepasst. Bei Python gibt es das Problem nicht, weil der Code dort neu erzeugt wird.
- **Ganze Datei in eine bestehende Datei:** Der komplette Dateitext wird an der Zielstelle eingefügt, die Imports der Quelle landen also mitten im Ziel. Mit `addImport`/`removeImport` lässt sich das nachziehen.
Die Imports landen mitten im Ziel nur wenn es eine Quelldatei ist aber ein Zielknoten?
SystemInit:  id=71fc1cac-d49d-4728-bb65-73d1d1769780, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-opus-5-5
ReasoningToken: 519
Control Request:
```yaml
id: 1f37-222-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python
    pattern: autopep8\.|E402|select|ignore
    include:
    - '*.py'
    limit: 20
  reason: "Prüfen, ob die Python-Engine beim Serialisieren (autopep8) Imports an den Dateianfang verschiebt – relevant für die Antwort, wo Imports beim Verschieben ganzer Dateien landen"
```
/answer 1f37-222-0 allow
Control Request:
```yaml
id: 1f37-223-222
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python
        matches:
        - path: _engine.py
          matches:
          - whole via autopep8. Compound statements (``if``/``def``/``class``/...)
          - "return autopep8.fix_code(code, options={**options, 'aggressive': aggressive}).rstrip('\\n')"
          - "return Tree(self, ast.Module(body=[], type_ignores=[]), '', path)"
```
/answer 1f37-223-222 allow
Result Stats: id=71fc1cac-d49d-4728-bb65-73d1d1769780, total: 151086, in: 66, out: 43858, reason: 153, read: 2210072, write: 107162, start: 1795554153694, end: 1795554164258, time: 10517, total: 498745, io: 0, control: 47, sent: 0, recv: 153
ReasoningToken: 153
Nein, nicht nur dann. Die Imports landen auch ohne Zielknoten an der falschen Stelle. Beim Verschieben einer ganzen Datei hängt es nur davon ab, ob die Zieldatei schon existiert:

| Fall | Was passiert | Wo landen die Imports der Quelle |
|---|---|---|
| Zieldatei existiert nicht | Ziel wird aus dem Quelltext erzeugt, wie bei Create | Oben, genau wie in der Quelle ✔ |
| Zieldatei existiert, mit `targetId` | ganzer Quelltext wird vor oder hinter dem Zielknoten eingefügt | Mitten im Ziel, an der Einfügestelle |
| Zieldatei existiert, ohne `targetId` | ganzer Quelltext wird ans Ende angehängt | Am Ende des Ziels |

In beiden Fällen mit bestehender Zieldatei fügt `_move_file` den Quelltext unverändert als einen Block ein. Imports werden dabei nicht gesondert behandelt.

Beim Verschieben einzelner Knoten (`ids` gesetzt) gibt es dieses Problem nicht. Es werden nur die Knoten selbst übertragen und keine Imports der Quelle. Fehlende Imports müssen dann über `addImport` angegeben werden.

Eine Einschränkung zu Python: Die Engine schickt den Code beim Speichern durch autopep8. Ob autopep8 mit den dort gesetzten Optionen Imports nach oben zieht (Regel E402), habe ich nicht geprüft. Darauf sollte man sich nicht verlassen.

Wenn du willst, kann ich das beheben. Beim Verschieben einer ganzen Datei in eine bestehende Datei würden dann die Imports der Quelle abgetrennt und über die vorhandene Import-Logik oben im Ziel eingefügt, also wie bei `addImport` und ohne Duplikate. Nur der Rest würde an der Zielstelle eingefügt oder angehängt.
Nein, das sind zu viele Annahmen. Der Agent kennt die ID des Quell import Blocks und könnte diesen selbst hinter einen existierenden Zielimportblock bewegen.
Result Stats: id=71fc1cac-d49d-4728-bb65-73d1d1769780, total: 152764, in: 68, out: 44565, reason: 295, read: 2317234, write: 108131, start: 1795554277804, end: 1795554289723, time: 11871, total: 510616, io: 0, control: 48, sent: 1, recv: 98
SystemInit:  id=71fc1cac-d49d-4728-bb65-73d1d1769780, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-opus-5-5
ReasoningToken: 295
Einverstanden, dann bleibt das Verhalten so und ich ändere nichts. Das Werkzeug dafür ist schon vorhanden, und das passt zum restlichen Werkzeugsatz: Das Tool verschiebt genau das, was adressiert wird, und der Agent entscheidet, wo es hingehört.

Der Ablauf geht in einem einzigen Aufruf, weil die Items der Reihe nach ausgeführt werden:

1. **Erstes Item:** `ids=[<ID des Quell-Importblocks>]`, `targetId=<ID des Ziel-Importblocks>`, `position="after"`.
2. **Zweites Item:** gleiche Quelle und gleiches Ziel, ohne `ids`. Der Rest der Datei wird angehängt oder per `targetId` eingefügt, danach wird die Quelldatei gelöscht.

Doppelte Imports werden dabei nicht zusammengeführt. Steht ein Import schon im Ziel, ist er danach zweimal vorhanden. Dann muss der Agent nachträglich aufräumen oder statt des Verschiebens `addImport` verwenden, das Duplikate vermeidet.
`imports`, `sourceImports` und `sourceDeleted` brauchen nicht erneut Bestandteil der Resultrückgabe sein.
SystemInit:  id=71fc1cac-d49d-4728-bb65-73d1d1769780, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-opus-5-5
ReasoningToken: 292
Control Request:
```yaml
id: 1f37-224-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    name: payload
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    parent_type: ClassDef
    text: "source_deleted: Whether"
  reason: "Aktuelle payload()-Methode und Docstring von MoveResult lesen, um imports/sourceImports/sourceDeleted aus der Rückgabe zu entfernen"
```
/answer 1f37-224-0 allow
Control Request:
```yaml
id: 1f37-225-224
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
        nodes:
        - id: MoveResult.payload
          code: |-
            def payload(self) -> dict[str, Any]:
                moved = [{'old': old} if new is None else {'old': old, 'new': new} for old, new in self.moved]
                return edit_payload(
                    imports=self.imports,
                    moved=moved,
                    unassigned=self.unassigned or None,
                    sourceImports=import_entry(
                        self.source_imports),
                    sourceDeleted=True if self.source_deleted else None)
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
        nodes:
        - id: MoveResult.AKKya9|zruM6c
          code: "'Result of a single move operation.\\n\\n    Attributes:\\n        path: The source path exactly as given in the input, for result association.\\n        target: The target path exactly as given in the input.\\n        moved: ``(old id, new id)`` per moved node; the new id is ``None`` if it could\\n            not be attributed unambiguously.\\n        unassigned: New top-level ids in the target that could not be attributed.\\n        imports: Import outcome in the target file.\\n        source_imports: Import outcome in the source file.\\n        source_deleted: Whether the source file was removed (moved whole or left empty).\\n    '"
```
/answer 1f37-225-224 allow
Thinking:
Ich entferne die drei Felder aus `payload()`, behalte aber die Dataclass-Felder intern, da `derived_warnings` sie für den Hinweis bei ungeprüften Imports und die Python-API weiterhin braucht.


ReasoningToken: 93
Control Request:
```yaml
id: 1f37-226-0
toolName: ast_edit
arguments:
  replace:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: MoveResult.payload
    source: |-
      def payload(self) -> dict[str, Any]:
          moved = [{'old': old} if new is None else {'old': old, 'new': new} for old, new in self.moved]
          return edit_payload(moved=moved, unassigned=self.unassigned or None)
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: MoveResult.AKKya9|zruM6c
    old_text: "        imports: Import outcome in the target file."
    new_text: |2-
              imports: Import outcome in the target file (not serialized; only drives the
                  batch-wide import notice, like ``source_imports``).
    removeImport:
    - "from xy.ai.mcpc.tools.ast.imports import IMPORT_NOTICE, ImportCheckedBatchResult, ImportOutcome, import_entry, parse_import_args"
    addImport:
    - "from xy.ai.mcpc.tools.ast.imports import IMPORT_NOTICE, ImportCheckedBatchResult, ImportOutcome, parse_import_args"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: MoveResult.AKKya9|zruM6c
    old_text: "        source_deleted: Whether the source file was removed (moved whole or left empty)."
    new_text: |2-
              source_deleted: Whether the source file was removed (moved whole or left empty);
                  Python API only, not serialized.
  reason: "imports, sourceImports und sourceDeleted aus der serialisierten Rückgabe von ast_move entfernen"
```
/answer 1f37-226-0 allow
Control Request:
```yaml
id: 1f37-227-226
result:
  structuredContent:
    replace:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
        id: MoveResult.payload
        new_id: MoveResult.payload
    edit_block:
      errors:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
        id: MoveResult.AKKya9|zruM6c
        error: "Syntax error: unterminated string literal (detected at line 1) (line 1)"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
        id: MoveResult.AKKya9|zruM6c
        error: "Syntax error: unterminated string literal (detected at line 1) (line 1)"
```
/answer 1f37-227-226 allow
Text:
 Die Docstring-Änderung ist gescheitert, weil der Docstring-Knoten als einzeiliges Literal mit `\n`-Escapes vorliegt. Ich passe das an.
ReasoningToken: 116
Control Request:
```yaml
id: 1f37-228-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: MoveResult.AKKya9|zruM6c
    old_text: "imports: Import outcome in the target file.\\n"
    new_text: "imports: Import outcome in the target file (not serialized; only drives the\\n            batch-wide import notice, like ``source_imports``).\\n"
    exact: true
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: MoveResult.AKKya9|zruM6c
    old_text: (moved whole or left empty).\n
    new_text: "(moved whole or left empty);\\n            Python API only, not serialized.\\n"
    exact: true
    removeImport:
    - "from xy.ai.mcpc.tools.ast.imports import IMPORT_NOTICE, ImportCheckedBatchResult, ImportOutcome, import_entry, parse_import_args"
    addImport:
    - "from xy.ai.mcpc.tools.ast.imports import IMPORT_NOTICE, ImportCheckedBatchResult, ImportOutcome, parse_import_args"
  reason: Docstring von MoveResult (escaped Form) anpassen und den nicht mehr benötigten import_entry-Import entfernen
```
/answer 1f37-228-0 allow
Text:
 Ich ergänze einen Test, der die Form der Rückgabe festhält, und führe danach die Tests aus.
ReasoningToken: 57
Control Request:
```yaml
id: 1f37-229-0
toolName: ast_edit
arguments:
  insert:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_move.py
    id: test_emptied_source_file_is_deleted
    position: after
    source: |-
      def test_result_contains_only_paths_and_moved_ids(tmp_path: Path):
          src = _write(tmp_path, 'a.py', SRC)
          dst = str(tmp_path / 'b.py')
          result = ast_move([MoveItem(path=src, target=dst, ids=['foo', 'bar'], add_import=['import os'],
                                      source_remove_import=['import os'])])
          assert not result.errors
          assert result.results[0].source_deleted
          assert result.results[0].to_dict() == {
              'path': src, 'target': dst, 'moved': [{'old': 'foo', 'new': 'foo'}, {'old': 'bar', 'new': 'bar'}]}
  reason: "Test ergänzen, der sicherstellt, dass die Rückgabe nur path, target und moved enthält"
```
/answer 1f37-229-0 allow
Control Request:
```yaml
id: 1f37-22a-0
toolName: run_command
arguments:
  items:
  - id: runTests
    cwd: /home/user/xyan/xy.ai.workbench/mcpc
  reason: Tests nach dem Entfernen der Felder aus der Rückgabe ausführen
```
/answer 1f37-22a-0 allow
Control Request:
```yaml
id: 1f37-22b-22a
result:
  structuredContent:
    results:
    - index: 0
      exit_code: 1
      stdout: |
        ============================= test session starts ==============================
        platform linux -- Python 3.12.3, pytest-8.3.5, pluggy-1.6.0
        rootdir: /home/user/xyan/xy.ai.workbench/mcpc
        configfile: pyproject.toml
        plugins: anyio-4.8.0, langsmith-0.4.18, docker-3.1.2
        collected 501 items

        tests/test_ast_engines.py ..................                             [  3%]
        tests/test_ast_imports.py ....................                           [  7%]
        tests/test_ast_imports_rpc.py ...........                                [  9%]
        tests/test_ast_markdown_bridge.py ...................................... [ 17%]
        ..................................................F                      [ 27%]
        tests/test_ast_move.py ...............                                   [ 30%]
        tests/test_ast_retrieve.py ......                                        [ 31%]
        tests/test_ast_rust_bridge.py ....                                       [ 32%]
        tests/test_cli_session.py .................                              [ 35%]
        tests/test_edit_diagnostics.py .......................                   [ 40%]
        tests/test_edit_tolerance.py ...........................                 [ 45%]
        tests/test_embedded.py .............                                     [ 48%]
        tests/test_embedded_batch.py .                                           [ 48%]
        tests/test_grep_with_ast.py ...                                          [ 49%]
        tests/test_http_human_in_the_loop.py ..                                  [ 49%]
        tests/test_http_tool_calls.py ...                                        [ 50%]
        tests/test_import_all.py ............................................... [ 59%]
        ........................................................................ [ 74%]
        .......                                                                  [ 75%]
        tests/test_mcpcignore.py ........................                        [ 80%]
        tests/test_persist.py ..........                                         [ 82%]
        tests/test_result_contract.py ..................................         [ 89%]
        tests/test_rpc_java_engine.py ..........                                 [ 91%]
        tests/test_script_file_source.py ..............................          [ 97%]
        tests/test_smoke.py ..                                                   [ 97%]
        tests/test_tools_family_smoke.py ...                                     [ 98%]
        tests/test_validate_arguments.py ..........                              [100%]

        =================================== FAILURES ===================================
        __________ test_existing_engine_instance_recovers_after_process_crash __________
        [XPASS(strict)] RpcEngine caches its client and only starts the process when the client is created, so an already used engine instance (e.g. from get_engine) never recovers from a crashed process.
        =========================== short test summary info ============================
        FAILED tests/test_ast_markdown_bridge.py::test_existing_engine_instance_recovers_after_process_crash
        ======================== 1 failed, 500 passed in 15.76s ========================
```
/answer 1f37-22b-22a allow
Result Stats: id=71fc1cac-d49d-4728-bb65-73d1d1769780, total: 162485, in: 80, out: 48035, reason: 0, read: 2981347, write: 114370, start: 1795554451761, end: 1795554458697, time: 6888, total: 576633, io: 0, control: 48, sent: 0, recv: 98
`ast_move` gibt pro Item jetzt nur noch `path`, `target` und `moved` zurück. `unassigned` kommt nur hinzu, wenn sich neue IDs nicht zuordnen lassen. `imports`, `sourceImports` und `sourceDeleted` fehlen in der Rückgabe.

- **Änderung in `move.py`:** Nur `MoveResult.payload()` ist geändert, und der Import von `import_entry`, der nicht mehr gebraucht wird, ist entfernt. Die Felder selbst bleiben intern erhalten:
  - `imports`/`source_imports` braucht der Batch weiterhin. Konnten Imports nicht verarbeitet werden, erscheint daraus der übliche Hinweis.
  - `source_deleted` gibt es nur noch in der Python-API.
- **Neuer Test** `test_result_contains_only_paths_and_moved_ids`: Er prüft, dass die Rückgabe auch mit Import-Angaben und gelöschter Quelldatei genau diese Form hat.
- **Testlauf:** 500 von 501 Tests bestehen. Weiterhin fehlschlägt nur `test_existing_engine_instance_recovers_after_process_crash` in `test_ast_markdown_bridge.py`, wie schon zuvor. Ein als strict markierter erwarteter Fehlschlag läuft dort inzwischen durch, und mit `ast_move` hat das nichts zu tun.
Erklär mir `unassigned`
Result Stats: id=71fc1cac-d49d-4728-bb65-73d1d1769780, total: 165172, in: 82, out: 50170, reason: 781, read: 3095717, write: 114920, start: 1795554484836, end: 1795554507482, time: 22601, total: 599234, io: 0, control: 45, sent: 1, recv: 261
SystemInit:  id=71fc1cac-d49d-4728-bb65-73d1d1769780, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-opus-5-5
ReasoningToken: 781
`unassigned` ist ein Rückfallfeld. Es listet neue Top-Level-IDs im Ziel, die `ast_move` keinem verschobenen Knoten sicher zuordnen konnte.

**Wie die neuen IDs ermittelt werden** (`_place` und `_attribute` in `move.py`):

1. Vor dem Einfügen merkt sich das Tool alle IDs der Zieldatei.
2. Nach dem Einfügen gelten alle IDs, die vorher nicht da waren, als neu. Davon bleiben nur die obersten übrig, Kinder wie `foo.x` fallen weg.
3. Diese neuen IDs werden den verschobenen Knoten zugeordnet:
   - **Gleiche Anzahl:** Die Zuordnung erfolgt einfach nach Reihenfolge. `unassigned` bleibt leer.
   - **Unterschiedliche Anzahl:** Die Zuordnung erfolgt über den Namen. Das letzte ID-Segment wird ohne Zähl-Suffix wie `_1` mit dem Namen des Knotens verglichen. Knoten ohne Treffer bekommen kein `new`. Neue IDs, die übrig bleiben, landen in `unassigned`.

**Wann die Anzahl nicht passt:** Wenn die Engine den eingefügten Code nicht als genau einen Knoten pro verschobenem Knoten abbildet. Beispiele:
- **Python-Anweisungsgruppen:** Aufeinanderfolgende einfache Anweisungen bilden einen namenlosen Knoten `statements` mit Hash-ID. Landet so eine Gruppe neben einer anderen, können beide zu einer verschmelzen. Die bestehende Gruppe im Ziel bekommt dann ebenfalls eine neue ID.
- **Ganze Datei:** Die Top-Level-Knoten der Quelle können sich im Ziel anders gruppieren als vorher.
- **Text- oder Markup-Engines:** Eingefügter Text kann mit Nachbarn verschmelzen oder sich anders aufteilen.

Verschachtelt angegebene IDs wie `A.Inner.x` zusammen mit `A.Inner` bekommen ihre neue ID vom Vorfahren abgeleitet. Hat der Vorfahre kein `new`, fehlt es auch beim Kind.

**Was der Agent damit tun soll:** Sobald `unassigned` gesetzt ist oder bei einem Eintrag `new` fehlt, schaut er sich die Zieldatei mit `ast_outline` oder `ast_find` selbst an.

**Eine Schwäche des aktuellen Stands:** Die Zuordnung nach Reihenfolge prüft nicht, ob die IDs zusammenpassen. Stimmt die Anzahl zufällig, kann die Zuordnung trotzdem falsch sein, und es gibt keinen Hinweis darauf. Zwei Beispiele:
- **Namensgleicher Knoten:** Das Ziel hat schon `foo`. Je nach Vergabe der Suffixe kann statt des eingefügten `foo` das bestehende zu `foo_1` werden, und dann wird das falsche zugeordnet.
- **Verschmolzene Gruppe:** Eine verschobene Gruppe verschmilzt mit einer bestehenden im Ziel und ergibt genau eine neue ID. Diese wird dann ohne Hinweis zugeordnet, obwohl sie auch den bestehenden Inhalt umfasst.

Das ließe sich ohne zusätzliche Annahmen strenger machen: Nach Reihenfolge nur zuordnen, wenn auch die Namen übereinstimmen. Wo das nicht klappt, fehlt `new` und die ID kommt nach `unassigned`. Soll ich das umsetzen?
Das ist unsinn, ein insert gibt doch die neue ID zurück, darüber ist eine eindeutige zuordnung möglich.
SystemInit:  id=71fc1cac-d49d-4728-bb65-73d1d1769780, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-opus-5-5
Text:
 Du hast recht, die Zuordnung nach Anzahl und Namen ist überflüssig. Das Problem entsteht nur, weil `_move_nodes` alle Knoten als einen einzigen Code-Block einfügt. Ein kleines Detail: Auch `ast_insert` ermittelt seine `ids` durch einen Vergleich der IDs vor und nach dem Einfügen, denn `core.insert_node` liefert nur eine Anzahl zurück. Entscheidend ist aber: Wird **ein Knoten pro Einfügung** eingesetzt, gehört das Ergebnis eindeutig zu genau diesem Knoten.

So baue ich es um:
- **Einzeln einfügen:** Jeder Knoten wird einzeln eingefügt. Bei `after` geschieht das in umgekehrter Reihenfolge direkt hinter dem Ziel, bei `before` und beim Anhängen in der angegebenen Reihenfolge. So bleibt die Reihenfolge erhalten, und das Ziel wird vor jeder Einfügung über seine ID neu aufgelöst.
- **`new` pro Knoten:** `new` ist das Ergebnis genau dieser einen Einfügung. Liefert eine Einfügung mehrere IDs, wie die `ids` bei `ast_insert`, wird die Liste zurückgegeben. `unassigned` und die Zuordnung nach Namen fallen weg.
- **Ganze Datei:** Bei einer neuen Zieldatei mit gleicher Endung bleibt es beim Kopieren. Gleicher Text und gleiche Engine ergeben identische IDs. In allen anderen Fällen wird die Datei Knoten für Knoten über alle Top-Level-IDs verschoben, also mit derselben eindeutigen Zuordnung.
ReasoningToken: 5910
Control Request:
```yaml
id: 1f37-22c-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    ids:
    - MoveResult
    - move_nodes
    - move_file
  outline:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
  reason: Aktuellen Stand der zu ersetzenden Funktionen in move.py lesen (nach den bisherigen Edits)
```
/answer 1f37-22c-0 allow
Control Request:
```yaml
id: 1f37-22d-22c
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
        nodes:
        - id: BcUf2h|UyT2Wv
          excert: "'``ast_move`` tool: move nodes between/within files, or whole files (insert/cre…"
        - id: mElsbe|hGOXBg
          excert: from __future__ import annotations
        - id: 54Uy07|KOy5MI
          excert: from xy.ai.mcpc.tools.tool_context import ToolContext
        - id: Xow2dX|YxioES
          excert: "__all__ = ['MoveItem', 'MoveResult', 'MoveError', 'MoveBatchResult', 'ast_move'…"
        - id: MoveItem
          excert: "@dataclass(frozen=True) class MoveItem:"
          docstring: "One move operation. Attributes: path: Absolute path of the source file. target:…"
        - id: MoveResult
          excert: "@dataclass(frozen=True) class MoveResult(ItemResult):"
          docstring: "Result of a single move operation. Attributes: path: The source path exactly as…"
          children:
          - id: MoveResult.qPFX90|V4Vur1
            excert: "'Result of a single move operation.\\n\\n    Attributes:\\n        path: The sourc…"
          - id: MoveResult.FFtpky|hyGPJj
            excert: "path: str"
          - id: MoveResult.key
            excert: "def key(self) -> dict[str, Any]:"
          - id: MoveResult.payload
            excert: "def payload(self) -> dict[str, Any]:"
        - id: MoveError
          excert: "@dataclass(frozen=True) class MoveError(ItemError):"
          docstring: "Error of a move operation, or of a single source id within it. Attributes: path…"
          children:
          - id: MoveError.XiWhCv|sW8PME
            excert: "'Error of a move operation, or of a single source id within it.\\n\\n    Attribut…"
          - id: MoveError.key
            excert: "def key(self) -> dict[str, Any]:"
        - id: MoveBatchResult
          excert: "class MoveBatchResult(ImportCheckedBatchResult[MoveResult, MoveError]):"
          docstring: "Result of :func:`ast_move`. Attributes: results: One :class:`MoveResult` per it…"
          children:
          - id: MoveBatchResult.QB5rlo|fkrhmk
            excert: "'Result of :func:`ast_move`.\\n\\n    Attributes:\\n        results: One :class:`M…"
          - id: MoveBatchResult.derived_warnings
            excert: "def derived_warnings(self) -> tuple[str, ...]:"
        - id: DK9oll|BC5PAC
          excert: _SUFFIX_RE = re.compile('_\\d+$')
        - id: is_inside
          excert: "def _is_inside(node_id: str, ancestor_id: str) -> bool:"
        - id: top_level
          excert: "def _top_level(ids: list[str]) -> list[str]:"
          docstring: Those ``ids`` that have no ancestor among ``ids`` (document order kept).
        - id: same_file
          excert: "def _same_file(a: Path, b: Path) -> bool:"
        - id: remove_file
          excert: "def _remove_file(path: Path) -> None:"
          docstring: "Delete ``path`` like ``ast_delete`` does: drop it from the cache and remove an …"
        - id: save
          excert: "def _save(path: Path, tree: core.Tree) -> None:"
        - id: is_empty
          excert: "def _is_empty(tree: core.Tree) -> bool:"
        - id: place
          excert: "def _place(dst_tree: core.Tree, target: core.Located | None, code: str, position: str) -> list[str]:"
          docstring: Insert/append ``code`` and return the top-level ids that are new in ``dst_tree`…
        - id: attribute
          excert: "def _attribute(old: list[tuple[str, str | None]], new_ids: list[str]) -> tuple[list[str | None], list[str]]:"
          docstring: "Pair each moved node ``(old id, simple name)`` with its new top-level id. Paire…"
        - id: parse_position
          excert: "def _parse_position(position: str) -> str:"
        - id: move_file
          excert: "def _move_file(item: MoveItem, src_path: Path, dst_path: Path) -> MoveResult:"
        - id: select_sources
          excert: "def _select_sources(item: MoveItem, src_tree: core.Tree, errors: list[MoveError]) -> list[tuple[str, core.Located]]:"
          docstring: Resolve every source id; unknown ids become errors without stopping the item.
        - id: move_nodes
          excert: "def _move_nodes(item: MoveItem, src_path: Path, dst_path: Path, errors: list[MoveError]) -> MoveResult | None:"
        - id: move_within
          excert: "def _move_within(item: MoveItem, tree: core.Tree, all_src: list[core.Located], roots: list[tuple[str, core.Located]], removal: list[core.Located]) -> core.Located | None:"
          docstring: Delete ``removal`` from ``tree`` and return the target node re-resolved afterwa…
        - id: move_one
          excert: "def _move_one(item: MoveItem, errors: list[MoveError]) -> MoveResult | None:"
        - id: ast_move
          excert: "def ast_move(items: list[MoveItem]) -> MoveBatchResult:"
          docstring: "Move nodes between or within files, or whole files. Nodes are inserted into the…"
        - id: string_list
          excert: "def _string_list(it: dict[str, Any], key: str) -> list[str] | None:"
        - id: MoveTool
          excert: "class MoveTool(ToolDefinition):"
          children:
          - id: MoveTool.rsicXK|eyBEDD
            excert: name = 'ast_move'
          - id: MoveTool.ANYI2U|uVkAYi
            excert: "_ITEM_PROPERTIES = {"
          - id: MoveTool.ZDSH0j|8W388v
            excert: "_ITEM_REQUIRED = ['path', 'target']"
          - id: MoveTool.handle
            excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
            docstring: "Delegate to :func:`ast_move`, translating the MCP schema to/from the Python API."
        - id: register
          excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
        nodes:
        - id: MoveResult
          type: ClassDef
          excert: "@dataclass(frozen=True) class MoveResult(ItemResult):"
          docstring: "Result of a single move operation. Attributes: path: The source path exactly as…"
          children:
          - id: MoveResult.qPFX90|V4Vur1
            type: statements
            code: "'Result of a single move operation.\\n\\n    Attributes:\\n        path: The source path exactly as given in the input, for result association.\\n        target: The target path exactly as given in the input.\\n        moved: ``(old id, new id)`` per moved node; the new id is ``None`` if it could\\n            not be attributed unambiguously.\\n        unassigned: New top-level ids in the target that could not be attributed.\\n        imports: Import outcome in the target file (not serialized; only drives the\\n            batch-wide import notice, like ``source_imports``).\\n        source_imports: Import outcome in the source file.\\n        source_deleted: Whether the source file was removed (moved whole or left empty);\\n            Python API only, not serialized.\\n    '"
          - id: MoveResult.FFtpky|hyGPJj
            type: statements
            code: |-
              path: str
              target: str
              moved: list[tuple[str, str | None]]
              unassigned: list[str] | None = None
              imports: ImportOutcome | None = None
              source_imports: ImportOutcome | None = None
              source_deleted: bool = False
          - id: MoveResult.key
            type: FunctionDef
            code: |-
              def key(self) -> dict[str, Any]:
                  return {'path': self.path, 'target': self.target}
          - id: MoveResult.payload
            type: FunctionDef
            code: |-
              def payload(self) -> dict[str, Any]:
                  moved = [{'old': old} if new is None else {'old': old, 'new': new} for old, new in self.moved]
                  return edit_payload(moved=moved, unassigned=self.unassigned or None)
        - id: move_nodes
          type: FunctionDef
          code: |-
            def _move_nodes(item: MoveItem, src_path: Path, dst_path: Path, errors: list[MoveError]) -> MoveResult | None:
                src_tree = core.CACHE.get_tree(src_path)
                selected = _select_sources(item, src_tree, errors)
                if not selected:
                    return None
                roots = [(given, loc) for given, loc in selected if not any(
                    (_is_inside(loc.node_id, other.node_id) for _, other in selected))]
                root_ids = {loc.node_id for _, loc in roots}
                nested = [(given, loc) for given, loc in selected if loc.node_id not in root_ids]
                code = '\n'.join((core.edit_node_source(loc).rstrip('\n') for _, loc in roots))
                all_src = core.locate_all(src_tree)
                order = {loc.node_id: i for i, loc in enumerate(all_src)}
                removal = sorted((loc for _, loc in roots), key=lambda loc: order.get(loc.node_id, -1), reverse=True)
                same = _same_file(src_path, dst_path)
                try:
                    if same:
                        dst_tree = src_tree
                        target = _move_within(item, src_tree, all_src, roots, removal)
                    else:
                        if dst_path.exists():
                            dst_tree = core.CACHE.get_tree(dst_path)
                            target = select_by_path(dst_tree, id=item.target_id) if item.target_id else None
                        else:
                            if item.target_id:
                                raise core.AstError('Target file does not exist; omit targetId to create it.')
                            dst_tree = core.empty_tree(dst_path)
                            target = None
                        '# Delete from the source in reverse document order so the remaining nodes stay addressed.'
                        for loc in removal:
                            core.delete_node(loc)
                    new_top = _place(dst_tree, target, code, item.position)
                    new_all = {loc.node_id for loc in core.locate_all(dst_tree)}
                    if same:
                        imports = core.apply_imports(dst_tree, [*
                                                                (item.add_import or []), *
                                                                (item.source_add_import or [])] or None, [*
                                                                                                          (item.remove_import or []), *
                                                                                                          (item.source_remove_import or [])] or None)
                        source_imports = None
                        _save(dst_path, dst_tree)
                        source_deleted = False
                    else:
                        imports = core.apply_imports(dst_tree, item.add_import, item.remove_import)
                        source_imports = core.apply_imports(src_tree, item.source_add_import, item.source_remove_import)
                        _save(dst_path, dst_tree)
                        source_deleted = _is_empty(src_tree)
                        if source_deleted:
                            _remove_file(src_path)
                        else:
                            _save(src_path, src_tree)
                except BaseException:
                    core.CACHE.invalidate(src_path)
                    core.CACHE.invalidate(dst_path)
                    raise
                mapped, unassigned = _attribute([(loc.node_id, loc.name) for _, loc in roots], new_top)
                new_by_root = {loc.node_id: new for (_, loc), new in zip(roots, mapped)}
                moved: list[tuple[str, str | None]] = [(given, new) for (given, _), new in zip(roots, mapped)]
                for given, loc in nested:
                    anc = next((other for _, other in roots if _is_inside(loc.node_id, other.node_id)))
                    anc_new = new_by_root.get(anc.node_id)
                    new_id = anc_new + loc.node_id[len(anc.node_id):] if anc_new else None
                    moved.append((given, new_id if new_id in new_all else None))
                return MoveResult(path=item.path, target=item.target, moved=moved, unassigned=unassigned or None,
                                  imports=imports, source_imports=source_imports, source_deleted=source_deleted)
        - id: move_file
          type: FunctionDef
          code: |-
            def _move_file(item: MoveItem, src_path: Path, dst_path: Path) -> MoveResult:
                if _same_file(src_path, dst_path):
                    raise core.AstError('Moving a whole file onto itself is not possible; give source ids.')
                src_tree = core.CACHE.get_tree(src_path)
                src_all = core.locate_all(src_tree)
                top_ids = set(_top_level([loc.node_id for loc in src_all]))
                old_top = [loc for loc in src_all if loc.node_id in top_ids]
                text = src_path.read_text(encoding='utf-8')
                if dst_path.exists():
                    dst_tree = core.CACHE.get_tree(dst_path)
                    target = select_by_path(dst_tree, id=item.target_id) if item.target_id else None
                    try:
                        new_top = _place(dst_tree, target, text, item.position)
                    except BaseException:
                        core.CACHE.invalidate(dst_path)
                        raise
                else:
                    if item.target_id:
                        raise core.AstError('Target file does not exist; omit targetId to create it.')
                    dst_tree = core.parse_for(item.target, text)
                    new_top = _top_level([loc.node_id for loc in core.locate_all(dst_tree)])
                try:
                    imports = core.apply_imports(dst_tree, item.add_import, item.remove_import)
                    _save(dst_path, dst_tree)
                except BaseException:
                    core.CACHE.invalidate(dst_path)
                    raise
                _remove_file(src_path)
                mapped, unassigned = _attribute([(loc.node_id, loc.name) for loc in old_top], new_top)
                source_imports = ImportOutcome(unchecked=True) if item.source_add_import or item.source_remove_import else None
                return MoveResult(
                    path=item.path,
                    target=item.target,
                    moved=[
                        (loc.node_id,
                         new) for loc,
                        new in zip(
                            old_top,
                            mapped)],
                    unassigned=unassigned or None,
                    imports=imports,
                    source_imports=source_imports,
                    source_deleted=True)
```
/answer 1f37-22d-22c allow
ReasoningToken: 1423
Control Request:
```yaml
id: 1f37-22e-0
toolName: ast_edit
arguments:
  replace:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: MoveResult.qPFX90|V4Vur1
    source: |-
      """Result of a single move operation.

          Attributes:
              path: The source path exactly as given in the input, for result association.
              target: The target path exactly as given in the input.
              moved: ``(old id, new id)`` per moved node. Every node is inserted on its own, so
                  the new id is what that single insert created: one id, a list if the insert
                  created several top-level ids (as ``ast_insert``'s ``ids``), or ``None``.
              imports: Import outcome in the target file (not serialized; only drives the
                  batch-wide import notice, like ``source_imports``).
              source_imports: Import outcome in the source file.
              source_deleted: Whether the source file was removed (moved whole or left empty);
                  Python API only, not serialized.
          """
      path: str
      target: str
      moved: list[tuple[str, str | list[str] | None]]
      imports: ImportOutcome | None = None
      source_imports: ImportOutcome | None = None
      source_deleted: bool = False
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: MoveResult.payload
    source: |-
      def payload(self) -> dict[str, Any]:
          moved = [{'old': old} if new is None else {'old': old, 'new': new} for old, new in self.moved]
          return edit_payload(moved=moved)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: place
    source: |-
      def _place_each(dst_tree: core.Tree, target_id: str | None, codes: list[str], position: str) -> list[list[str]]:
          """Insert every code on its own; return the top-level ids each single insert created.

          The target is re-resolved by id before each insert. To keep the given order, nodes
          placed ``after`` the target are inserted last-first (each lands directly behind it).
          """
          created: list[list[str]] = [[] for _ in codes]
          order = list(range(len(codes)))
          if target_id is not None and position == 'after':
              order.reverse()
          for i in order:
              before = {loc.node_id for loc in core.locate_all(dst_tree)}
              if target_id is None:
                  core.append_nodes(dst_tree, codes[i])
              else:
                  core.insert_node(select_by_path(dst_tree, id=target_id), codes[i], position)
              created[i] = _top_level([loc.node_id for loc in core.locate_all(dst_tree) if loc.node_id not in before])
          final = {loc.node_id for loc in core.locate_all(dst_tree)}
          return [[node_id for node_id in ids if node_id in final] for ids in created]


      def _new_id(ids: list[str]) -> str | list[str] | None:
          if not ids:
              return None
          return ids[0] if len(ids) == 1 else ids
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: move_file
    source: |-
      def _move_file(item: MoveItem, src_path: Path, dst_path: Path, errors: list[MoveError]) -> MoveResult | None:
          if _same_file(src_path, dst_path):
              raise core.AstError('Moving a whole file onto itself is not possible; give source ids.')
          src_tree = core.CACHE.get_tree(src_path)
          top_ids = _top_level([loc.node_id for loc in core.locate_all(src_tree)])
          source_imports = ImportOutcome(unchecked=True) if item.source_add_import or item.source_remove_import else None
          if not dst_path.exists() and src_path.suffix.lower() == dst_path.suffix.lower():
              if item.target_id:
                  raise core.AstError('Target file does not exist; omit targetId to create it.')
              '# Plain copy like ast_create: same text and engine give the same outline and ids.'
              dst_tree = core.parse_for(item.target, src_path.read_text(encoding='utf-8'))
              new_top = _top_level([loc.node_id for loc in core.locate_all(dst_tree)])
              try:
                  imports = core.apply_imports(dst_tree, item.add_import, item.remove_import)
                  _save(dst_path, dst_tree)
              except BaseException:
                  core.CACHE.invalidate(dst_path)
                  raise
              _remove_file(src_path)
              moved = list(zip(top_ids, new_top)) if len(top_ids) == len(new_top) else [(i, None) for i in top_ids]
              return MoveResult(path=item.path, target=item.target, moved=moved, imports=imports,
                                source_imports=source_imports, source_deleted=True)
          if not top_ids:
              _remove_file(src_path)
              return MoveResult(path=item.path, target=item.target, moved=[], source_imports=source_imports,
                                source_deleted=True)
          '# Otherwise move every top-level node on its own, which keeps the old->new ids exact.'
          result = _move_nodes(replace(item, ids=top_ids), src_path, dst_path, errors)
          if result is not None and not result.source_deleted:
              result = result.warn('The source file was kept: content outside its top-level nodes remained.')
          return result
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: move_nodes
    source: |-
      def _move_nodes(item: MoveItem, src_path: Path, dst_path: Path, errors: list[MoveError]) -> MoveResult | None:
          src_tree = core.CACHE.get_tree(src_path)
          selected = _select_sources(item, src_tree, errors)
          if not selected:
              return None
          roots = [(given, loc) for given, loc in selected
                   if not any(_is_inside(loc.node_id, other.node_id) for _, other in selected)]
          root_ids = {loc.node_id for _, loc in roots}
          nested = [(given, loc) for given, loc in selected if loc.node_id not in root_ids]
          codes = [core.edit_node_source(loc).rstrip('\n') for _, loc in roots]
          all_src = core.locate_all(src_tree)
          order = {loc.node_id: i for i, loc in enumerate(all_src)}
          removal = sorted((loc for _, loc in roots), key=lambda loc: order.get(loc.node_id, -1), reverse=True)
          same = _same_file(src_path, dst_path)
          try:
              if same:
                  dst_tree = src_tree
                  target = _move_within(item, src_tree, all_src, roots, removal)
              else:
                  if dst_path.exists():
                      dst_tree = core.CACHE.get_tree(dst_path)
                      target = select_by_path(dst_tree, id=item.target_id) if item.target_id else None
                  else:
                      if item.target_id:
                          raise core.AstError('Target file does not exist; omit targetId to create it.')
                      dst_tree = core.empty_tree(dst_path)
                      target = None
                  '# Delete from the source in reverse document order so the remaining nodes stay addressed.'
                  for loc in removal:
                      core.delete_node(loc)
              created = _place_each(dst_tree, target.node_id if target else None, codes, item.position)
              new_all = {loc.node_id for loc in core.locate_all(dst_tree)}
              if same:
                  imports = core.apply_imports(
                      dst_tree,
                      [*(item.add_import or []), *(item.source_add_import or [])] or None,
                      [*(item.remove_import or []), *(item.source_remove_import or [])] or None)
                  source_imports = None
                  _save(dst_path, dst_tree)
                  source_deleted = False
              else:
                  imports = core.apply_imports(dst_tree, item.add_import, item.remove_import)
                  source_imports = core.apply_imports(src_tree, item.source_add_import, item.source_remove_import)
                  _save(dst_path, dst_tree)
                  source_deleted = _is_empty(src_tree)
                  if source_deleted:
                      _remove_file(src_path)
                  else:
                      _save(src_path, src_tree)
          except BaseException:
              core.CACHE.invalidate(src_path)
              core.CACHE.invalidate(dst_path)
              raise
          moved: list[tuple[str, str | list[str] | None]] = [(given, _new_id(ids)) for (given, _), ids in zip(roots, created)]
          new_by_root = {loc.node_id: ids[0] for (_, loc), ids in zip(roots, created) if len(ids) == 1}
          for given, loc in nested:
              anc = next(other for _, other in roots if _is_inside(loc.node_id, other.node_id))
              anc_new = new_by_root.get(anc.node_id)
              new_id = anc_new + loc.node_id[len(anc.node_id):] if anc_new else None
              moved.append((given, new_id if new_id in new_all else None))
          return MoveResult(path=item.path, target=item.target, moved=moved, imports=imports,
                            source_imports=source_imports, source_deleted=source_deleted)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: move_one
    source: |-
      def _move_one(item: MoveItem, errors: list[MoveError]) -> MoveResult | None:
          src_path = core.require_path(item.path)
          dst_path = core.require_path(item.target, must_exist=False)
          _parse_position(item.position)
          if not item.ids:
              return _move_file(item, src_path, dst_path, errors)
          return _move_nodes(item, src_path, dst_path, errors)
  delete:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: attribute
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: DK9oll|BC5PAC
    removeImport:
    - import re
    - from dataclasses import dataclass
    addImport:
    - "from dataclasses import dataclass, replace"
  reason: "ast_move auf Einzel-Einfügung pro Knoten umbauen: eindeutige Alt→Neu-Zuordnung, unassigned und Namens-Heuristik entfernen; ganze Dateien knotenweise verschieben, außer beim exakten Kopieren in eine neue Datei gleicher Endung"
```
/answer 1f37-22e-0 allow
Text:
 Beim Ersetzen des Docstrings habe ich die Felder mitgeschrieben, sie standen aber in einem eigenen Knoten. Ich prüfe, ob sie jetzt doppelt vorhanden sind.
ReasoningToken: 115
Control Request:
```yaml
id: 1f37-22f-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    ids:
    - MoveResult
  reason: "Prüfen, ob MoveResult nach dem Ersetzen doppelte Felddeklarationen enthält"
```
/answer 1f37-22f-0 allow
Control Request:
```yaml
id: 1f37-230-22f
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
        nodes:
        - id: MoveResult
          type: ClassDef
          excert: "@dataclass(frozen=True) class MoveResult(ItemResult):"
          docstring: "Result of a single move operation. Attributes: path: The source path exactly as…"
          children:
          - id: MoveResult.9IgNv3|bveNVr
            type: statements
            code: "\"Result of a single move operation.\\n\\n    Attributes:\\n        path: The source path exactly as given in the input, for result association.\\n        target: The target path exactly as given in the input.\\n        moved: ``(old id, new id)`` per moved node. Every node is inserted on its own, so\\n            the new id is what that single insert created: one id, a list if the insert\\n            created several top-level ids (as ``ast_insert``'s ``ids``), or ``None``.\\n        imports: Import outcome in the target file (not serialized; only drives the\\n            batch-wide import notice, like ``source_imports``).\\n        source_imports: Import outcome in the source file.\\n        source_deleted: Whether the source file was removed (moved whole or left empty);\\n            Python API only, not serialized.\\n    \""
          - id: MoveResult.FFtpky|1q905T
            type: statements
            code: |-
              path: str
              target: str
              moved: list[tuple[str, str | list[str] | None]]
              imports: ImportOutcome | None = None
              source_imports: ImportOutcome | None = None
              source_deleted: bool = False
              path: str
              target: str
              moved: list[tuple[str, str | None]]
              unassigned: list[str] | None = None
              imports: ImportOutcome | None = None
              source_imports: ImportOutcome | None = None
              source_deleted: bool = False
          - id: MoveResult.key
            type: FunctionDef
            code: |-
              def key(self) -> dict[str, Any]:
                  return {'path': self.path, 'target': self.target}
          - id: MoveResult.payload
            type: FunctionDef
            code: |-
              def payload(self) -> dict[str, Any]:
                  moved = [{'old': old} if new is None else {'old': old, 'new': new} for old, new in self.moved]
                  return edit_payload(moved=moved)
```
/answer 1f37-230-22f allow
Text:
 Die Felder sind tatsächlich doppelt vorhanden. Ich entferne die alte Deklaration, in der noch `unassigned` steht.
Control Request:
```yaml
id: 1f37-231-0
toolName: ast_edit
arguments:
  replace:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: MoveResult.FFtpky|1q905T
    source: |-
      path: str
      target: str
      moved: list[tuple[str, str | list[str] | None]]
      imports: ImportOutcome | None = None
      source_imports: ImportOutcome | None = None
      source_deleted: bool = False
  reason: "Doppelte, veraltete Felddeklarationen (inkl. unassigned) in MoveResult entfernen"
```
/answer 1f37-231-0 allow
ReasoningToken: 26
Control Request:
```yaml
id: 1f37-232-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    - /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: unassigned|_attribute|_place\(|_SUFFIX_RE|\bre\.
    include:
    - move.py
    - test_ast_move.py
  reason: "Sicherstellen, dass keine Verweise auf entfernte Elemente (unassigned, _attribute, _place, _SUFFIX_RE, re) übrig sind"
```
/answer 1f37-232-0 allow
Control Request:
```yaml
id: 1f37-233-232
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
        - /home/user/xyan/xy.ai.workbench/mcpc/tests
```
/answer 1f37-233-232 allow
Text:
 Jetzt ergänze ich Tests für die eindeutige Zuordnung: mehrere Knoten hinter einem Ziel (Reihenfolge und Zuordnung) sowie einen namensgleichen Knoten, der im Ziel schon existiert.
ReasoningToken: 144
Control Request:
```yaml
id: 1f37-234-0
toolName: ast_edit
arguments:
  insert:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_move.py
    id: test_unknown_source_id_is_error_but_others_move
    position: after
    source: |-
      def _code(path: str, node_id: str) -> str:
          tree = core.CACHE.get_tree(Path(path))
          return next(core.edit_node_source(loc) for loc in core.locate_all(tree) if loc.node_id == node_id)


      def test_several_nodes_after_target_keep_order_and_exact_ids(tmp_path: Path):
          src = _write(tmp_path, 'a.py', SRC)
          dst = _write(tmp_path, 'b.py', 'def first():\n    pass\n\n\ndef last():\n    pass\n')
          result = ast_move([MoveItem(path=src, target=dst, ids=['bar', 'foo'], target_id='first')])
          assert not result.errors
          assert result.results[0].moved == [('bar', 'bar'), ('foo', 'foo')]
          assert _top_ids(dst) == ['first', 'bar', 'foo', 'last']


      def test_name_clash_in_target_is_attributed_to_the_inserted_node(tmp_path: Path):
          src = _write(tmp_path, 'a.py', 'def foo():\n    return 42\n')
          dst = _write(tmp_path, 'b.py', 'def foo():\n    return 1\n')
          result = ast_move([MoveItem(path=src, target=dst, ids=['foo'], target_id='foo', position='before')])
          assert not result.errors
          ((old, new),) = result.results[0].moved
          assert old == 'foo' and isinstance(new, str)
          assert 'return 42' in _code(dst, new)
  reason: Tests für eindeutige Alt→Neu-Zuordnung durch Einzel-Einfügung ergänzen
```
/answer 1f37-234-0 allow
Control Request:
```yaml
id: 1f37-235-234
result:
  structuredContent:
    insert:
      errors:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_move.py
        id: test_unknown_source_id_is_error_but_others_move
        error: No node matched the selector. verify the node with `search` or `ast_find`
```
/answer 1f37-235-234 allow
ReasoningToken: 51
Control Request:
```yaml
id: 1f37-236-0
toolName: ast_edit
arguments:
  insert:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_move.py
    id: hb8e9c18a
    position: after
    source: |-
      def _code(path: str, node_id: str) -> str:
          tree = core.CACHE.get_tree(Path(path))
          return next(core.edit_node_source(loc) for loc in core.locate_all(tree) if loc.node_id == node_id)


      def test_several_nodes_after_target_keep_order_and_exact_ids(tmp_path: Path):
          src = _write(tmp_path, 'a.py', SRC)
          dst = _write(tmp_path, 'b.py', 'def first():\n    pass\n\n\ndef last():\n    pass\n')
          result = ast_move([MoveItem(path=src, target=dst, ids=['bar', 'foo'], target_id='first')])
          assert not result.errors
          assert result.results[0].moved == [('bar', 'bar'), ('foo', 'foo')]
          assert _top_ids(dst) == ['first', 'bar', 'foo', 'last']


      def test_name_clash_in_target_is_attributed_to_the_inserted_node(tmp_path: Path):
          src = _write(tmp_path, 'a.py', 'def foo():\n    return 42\n')
          dst = _write(tmp_path, 'b.py', 'def foo():\n    return 1\n')
          result = ast_move([MoveItem(path=src, target=dst, ids=['foo'], target_id='foo', position='before')])
          assert not result.errors
          ((old, new),) = result.results[0].moved
          assert old == 'foo' and isinstance(new, str)
          assert 'return 42' in _code(dst, new)
  reason: Tests hinter dem Result-Contract-Test einfügen (lange Testnamen haben Hash-IDs)
```
/answer 1f37-236-0 allow
Control Request:
```yaml
id: 1f37-237-0
toolName: run_command
arguments:
  items:
  - id: runTests
    cwd: /home/user/xyan/xy.ai.workbench/mcpc
  reason: Tests nach dem Umbau auf Einzel-Einfügung ausführen
```
/answer 1f37-237-0 allow
Control Request:
```yaml
id: 1f37-238-237
result:
  structuredContent:
    results:
    - index: 0
      exit_code: 1
      stdout: |
        ============================= test session starts ==============================
        platform linux -- Python 3.12.3, pytest-8.3.5, pluggy-1.6.0
        rootdir: /home/user/xyan/xy.ai.workbench/mcpc
        configfile: pyproject.toml
        plugins: anyio-4.8.0, langsmith-0.4.18, docker-3.1.2
        collected 503 items

        tests/test_ast_engines.py ..................                             [  3%]
        tests/test_ast_imports.py ....................                           [  7%]
        tests/test_ast_imports_rpc.py ...........                                [  9%]
        tests/test_ast_markdown_bridge.py ...................................... [ 17%]
        ..................................................F                      [ 27%]
        tests/test_ast_move.py ....F............                                 [ 30%]
        tests/test_ast_retrieve.py ......                                        [ 32%]
        tests/test_ast_rust_bridge.py ....                                       [ 32%]
        tests/test_cli_session.py .................                              [ 36%]
        tests/test_edit_diagnostics.py .......................                   [ 40%]
        tests/test_edit_tolerance.py ...........................                 [ 46%]
        tests/test_embedded.py .............                                     [ 48%]
        tests/test_embedded_batch.py .                                           [ 48%]
        tests/test_grep_with_ast.py ...                                          [ 49%]
        tests/test_http_human_in_the_loop.py ..                                  [ 49%]
        tests/test_http_tool_calls.py ...                                        [ 50%]
        tests/test_import_all.py ............................................... [ 59%]
        ........................................................................ [ 74%]
        .......                                                                  [ 75%]
        tests/test_mcpcignore.py ........................                        [ 80%]
        tests/test_persist.py ..........                                         [ 82%]
        tests/test_result_contract.py ..................................         [ 89%]
        tests/test_rpc_java_engine.py ..........                                 [ 91%]
        tests/test_script_file_source.py ..............................          [ 97%]
        tests/test_smoke.py ..                                                   [ 97%]
        tests/test_tools_family_smoke.py ...                                     [ 98%]
        tests/test_validate_arguments.py ..........                              [100%]

        =================================== FAILURES ===================================
        __________ test_existing_engine_instance_recovers_after_process_crash __________
        [XPASS(strict)] RpcEngine caches its client and only starts the process when the client is created, so an already used engine instance (e.g. from get_engine) never recovers from a crashed process.
        _________ test_name_clash_in_target_is_attributed_to_the_inserted_node _________

        tmp_path = PosixPath('/tmp/pytest-of-user/pytest-3/test_name_clash_in_target_is_a0')

            def test_name_clash_in_target_is_attributed_to_the_inserted_node(tmp_path: Path):
                src = _write(tmp_path, 'a.py', 'def foo():\n    return 42\n')
                dst = _write(tmp_path, 'b.py', 'def foo():\n    return 1\n')
                result = ast_move([MoveItem(path=src, target=dst, ids=['foo'], target_id='foo', position='before')])
                assert not result.errors
                (old, new), = result.results[0].moved
        >       assert old == 'foo' and isinstance(new, str)
        E       AssertionError: assert ('foo' == 'foo'
        E
        E           foo and False)
        E        +  where False = isinstance(['foo.gEQro1|svuKcz', 'foo_1'], str)

        tests/test_ast_move.py:80: AssertionError
        =========================== short test summary info ============================
        FAILED tests/test_ast_markdown_bridge.py::test_existing_engine_instance_recovers_after_process_crash
        FAILED tests/test_ast_move.py::test_name_clash_in_target_is_attributed_to_the_inserted_node
        ======================== 2 failed, 501 passed in 15.58s ========================
```
/answer 1f37-238-237 allow
Thinking:
Der neue Test zeigt, dass eine eingefügte `foo`-Knoten beim Vergleich per ID falsch zugeordnet wird; ich bestimme die Einfügung deshalb über ihre Position und prüfe die Struktur außerhalb davon.


ReasoningToken: 3357
Control Request:
```yaml
id: 1f37-239-0
toolName: ast_edit
arguments:
  replace:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: place
    source: |-
      def _place_each(dst_tree: core.Tree, target_id: str | None, codes: list[str], position: str) -> list[list[str] | None]:
          """Insert every code on its own; return the top-level ids each single insert created.

          Inserted nodes are found by position, not by an id diff (an insert may rename
          existing nodes, e.g. ``foo`` -> ``foo_1``): in document order they start at the
          target's index (``before``), right behind the target's subtree (``after``) or at the
          end (append). The target is tracked the same way. Each insert is checked to leave
          the structure outside the inserted nodes unchanged; if not (e.g. merged statement
          groups), positions are no longer trustworthy and all results become ``None``.
          To keep the given order, nodes placed ``after`` the target are inserted last-first.
          """
          current = core.locate_all(dst_tree)
          t = None if target_id is None else next(i for i, loc in enumerate(current) if loc.node_id == target_id)
          ranges: list[tuple[int, int] | None] = [None] * len(codes)
          exact = True
          order = list(range(len(codes)))
          if t is not None and position == 'after':
              order.reverse()
          for i in order:
              before = current
              if t is None or t >= len(before):
                  p = len(before)
                  core.append_nodes(dst_tree, codes[i])
              else:
                  anchor = before[t]
                  size = 1 + sum(1 for loc in before if _is_inside(loc.node_id, anchor.node_id))
                  p = t if position == 'before' else t + size
                  core.insert_node(anchor, codes[i], position)
              current = core.locate_all(dst_tree)
              k = len(current) - len(before)
              rest = current[:p] + current[p + k:]
              if k <= 0 or [loc.node_type for loc in rest] != [loc.node_type for loc in before]:
                  exact = False
                  continue
              ranges = [r if r is None or r[0] < p else (r[0] + k, r[1]) for r in ranges]
              ranges[i] = (p, k)
              if t is not None and position == 'before':
                  t += k
          if not exact:
              return [None] * len(codes)
          return [_top_level([loc.node_id for loc in current[r[0]:r[0] + r[1]]]) if r else None for r in ranges]
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: new_id
    source: |-
      def _new_id(ids: list[str] | None) -> str | list[str] | None:
          if not ids:
              return None
          return ids[0] if len(ids) == 1 else ids
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: move_nodes
    old_text: "new_by_root = {loc.node_id: ids[0] for (_, loc), ids in zip(roots, created) if len(ids) == 1}"
    new_text: "new_by_root = {loc.node_id: ids[0] for (_, loc), ids in zip(roots, created) if ids and len(ids) == 1}"
  reason: _place_each auf positionsbasierte Ermittlung der eingefügten Knoten umstellen (ID-Diff ist bei Namenskollisionen falsch) und Aufrufer anpassen
```
/answer 1f37-239-0 allow
Control Request:
```yaml
id: 1f37-23a-239
result:
  structuredContent:
    replace:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
        id: new_id
        new_id: new_id
      errors:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
        id: place
        error: No node matched the selector. verify the node with `search` or `ast_find`
    edit_block:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
        id: move_nodes
        new_id: move_nodes
```
/answer 1f37-23a-239 allow
Text:
 Die ID des Knotens hat sich geändert, er heißt jetzt `place_each`. Ich ersetze ihn darüber.
Control Request:
```yaml
id: 1f37-23b-0
toolName: ast_edit
arguments:
  replace:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: place_each
    source: |-
      def _place_each(dst_tree: core.Tree, target_id: str | None, codes: list[str], position: str) -> list[list[str] | None]:
          """Insert every code on its own; return the top-level ids each single insert created.

          Inserted nodes are found by position, not by an id diff (an insert may rename
          existing nodes, e.g. ``foo`` -> ``foo_1``): in document order they start at the
          target's index (``before``), right behind the target's subtree (``after``) or at the
          end (append). The target is tracked the same way. Each insert is checked to leave
          the structure outside the inserted nodes unchanged; if not (e.g. merged statement
          groups), positions are no longer trustworthy and all results become ``None``.
          To keep the given order, nodes placed ``after`` the target are inserted last-first.
          """
          current = core.locate_all(dst_tree)
          t = None if target_id is None else next(i for i, loc in enumerate(current) if loc.node_id == target_id)
          ranges: list[tuple[int, int] | None] = [None] * len(codes)
          exact = True
          order = list(range(len(codes)))
          if t is not None and position == 'after':
              order.reverse()
          for i in order:
              before = current
              if t is None or t >= len(before):
                  p = len(before)
                  core.append_nodes(dst_tree, codes[i])
              else:
                  anchor = before[t]
                  size = 1 + sum(1 for loc in before if _is_inside(loc.node_id, anchor.node_id))
                  p = t if position == 'before' else t + size
                  core.insert_node(anchor, codes[i], position)
              current = core.locate_all(dst_tree)
              k = len(current) - len(before)
              rest = current[:p] + current[p + k:]
              if k <= 0 or [loc.node_type for loc in rest] != [loc.node_type for loc in before]:
                  exact = False
                  continue
              ranges = [r if r is None or r[0] < p else (r[0] + k, r[1]) for r in ranges]
              ranges[i] = (p, k)
              if t is not None and position == 'before':
                  t += k
          if not exact:
              return [None] * len(codes)
          return [_top_level([loc.node_id for loc in current[r[0]:r[0] + r[1]]]) if r else None for r in ranges]
  reason: _place_each (neue ID place_each) durch positionsbasierte Variante ersetzen
```
/answer 1f37-23b-0 allow
Text:
 Noch ein Randfall: Schlägt die Strukturprüfung fehl, muss das Ziel für die weiteren Einfügungen trotzdem sinnvoll nachgeführt werden.
Control Request:
```yaml
id: 1f37-23c-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: place_each
    old_text: |2-
                  exact = False
                  continue
    new_text: |2-
                  exact = False
                  if t is not None and position == 'before':
                      '# Best effort to keep inserting in front of the target.'
                      t = next((j for j, loc in enumerate(current) if loc.node_id == target_id), t + max(k, 0))
                  continue
  reason: Bei fehlgeschlagener Strukturprüfung das Ziel für weitere Einfügungen bestmöglich nachführen
```
/answer 1f37-23c-0 allow
ReasoningToken: 316
Control Request:
```yaml
id: 1f37-23d-0
toolName: run_command
arguments:
  items:
  - id: runTests
    cwd: /home/user/xyan/xy.ai.workbench/mcpc
  reason: Tests nach positionsbasierter Zuordnung ausführen
```
/answer 1f37-23d-0 allow
Control Request:
```yaml
id: 1f37-23e-23d
result:
  structuredContent:
    results:
    - index: 0
      exit_code: 1
      stdout: |
        ============================= test session starts ==============================
        platform linux -- Python 3.12.3, pytest-8.3.5, pluggy-1.6.0
        rootdir: /home/user/xyan/xy.ai.workbench/mcpc
        configfile: pyproject.toml
        plugins: anyio-4.8.0, langsmith-0.4.18, docker-3.1.2
        collected 503 items

        tests/test_ast_engines.py ..................                             [  3%]
        tests/test_ast_imports.py ....................                           [  7%]
        tests/test_ast_imports_rpc.py ...........                                [  9%]
        tests/test_ast_markdown_bridge.py ...................................... [ 17%]
        ..................................................F                      [ 27%]
        tests/test_ast_move.py .................                                 [ 30%]
        tests/test_ast_retrieve.py ......                                        [ 32%]
        tests/test_ast_rust_bridge.py ....                                       [ 32%]
        tests/test_cli_session.py .................                              [ 36%]
        tests/test_edit_diagnostics.py .......................                   [ 40%]
        tests/test_edit_tolerance.py ...........................                 [ 46%]
        tests/test_embedded.py .............                                     [ 48%]
        tests/test_embedded_batch.py .                                           [ 48%]
        tests/test_grep_with_ast.py ...                                          [ 49%]
        tests/test_http_human_in_the_loop.py ..                                  [ 49%]
        tests/test_http_tool_calls.py ...                                        [ 50%]
        tests/test_import_all.py ............................................... [ 59%]
        ........................................................................ [ 74%]
        .......                                                                  [ 75%]
        tests/test_mcpcignore.py ........................                        [ 80%]
        tests/test_persist.py ..........                                         [ 82%]
        tests/test_result_contract.py ..................................         [ 89%]
        tests/test_rpc_java_engine.py ..........                                 [ 91%]
        tests/test_script_file_source.py ..............................          [ 97%]
        tests/test_smoke.py ..                                                   [ 97%]
        tests/test_tools_family_smoke.py ...                                     [ 98%]
        tests/test_validate_arguments.py ..........                              [100%]

        =================================== FAILURES ===================================
        __________ test_existing_engine_instance_recovers_after_process_crash __________
        [XPASS(strict)] RpcEngine caches its client and only starts the process when the client is created, so an already used engine instance (e.g. from get_engine) never recovers from a crashed process.
        =========================== short test summary info ============================
        FAILED tests/test_ast_markdown_bridge.py::test_existing_engine_instance_recovers_after_process_crash
        ======================== 1 failed, 502 passed in 16.10s ========================
```
/answer 1f37-23e-23d allow
Result Stats: id=71fc1cac-d49d-4728-bb65-73d1d1769780, total: 223750, in: 106, out: 73666, reason: 49, read: 4717922, write: 149978, start: 1795554946896, end: 1795554959321, time: 12378, total: 894297, io: 0, control: 47, sent: 0, recv: 203
ReasoningToken: 49
Du hattest recht: Jeder Knoten wird jetzt einzeln eingefügt, damit lässt sich die neue ID eindeutig zuordnen. `unassigned` und die Zuordnung über Anzahl und Namen sind entfernt. Alle 17 Tests in `test_ast_move.py` laufen durch. Von der ganzen Suite schlägt nur noch der XPASS-Test in `test_ast_markdown_bridge` fehl, wie schon vorher.

Beim Testen kam allerdings heraus, dass die neue ID nicht einfach aus dem Vergleich der IDs vor und nach dem Einfügen kommen kann. Genau so ermittelt `ast_insert` seine `ids`, denn `core.insert_node` gibt nur die Anzahl eingefügter Knoten zurück. Wird `foo` vor ein bestehendes `foo` eingefügt, heißt der eingefügte Knoten danach `foo` und der alte `foo_1`. Der Vergleich meldet dann `foo_1` plus eine Kind-ID, beides falsch.

**Wie `_place_each` die neue ID jetzt bestimmt:**
- **Über die Position:** In Dokumentreihenfolge liegt der eingefügte Knoten bei `before` am Index des Ziels, bei `after` direkt hinter dem Teilbaum des Ziels und beim Anhängen am Ende.
- **Ziel ebenfalls über die Position:** Das Ziel kann durch das Einfügen seine ID ändern. Deshalb wird es nicht über die ID neu gesucht, sondern über seinen Index nachgeführt.
- **Reihenfolge:** Bei `after` wird von hinten nach vorn eingefügt, bei `before` und beim Anhängen von vorn nach hinten. So bleibt die angegebene Reihenfolge erhalten.
- **Absicherung:** Nach jeder Einfügung wird geprüft, dass sich außerhalb des eingefügten Bereichs die Folge der Knotentypen nicht geändert hat. Ändert sie sich doch, etwa weil zwei Python-Anweisungsgruppen verschmelzen, fehlt `new`, statt dass geraten wird.
- **`new` als Liste:** Ergibt der Code eines Knotens mehrere Top-Level-Knoten, ist `new` eine Liste.

**Ganze Datei:** Bei einer neuen Zieldatei mit gleicher Endung wird sie weiterhin einfach kopiert. Gleicher Text und gleiche Engine ergeben dieselben IDs. Sonst werden alle Top-Level-Knoten einzeln verschoben. Bleibt danach in der Quelle Inhalt außerhalb der Knoten übrig, bleibt die Datei bestehen und das Ergebnis enthält eine Warnung.

**Neue Tests:** Mehrere Knoten hinter einem Ziel behalten Reihenfolge und IDs. Bei der Namenskollision wird `new` dem tatsächlich eingefügten Knoten zugeordnet; geprüft wird das über dessen Code.

`ast_insert` gibt bei einer Namenskollision weiterhin falsche `ids` zurück. Soll ich dort dieselbe positionsbasierte Ermittlung einbauen?
Warum? Wenn etwas vor Foo eingeführt wird und Foo heißt. Ist seine neue ID "Foo" oder wie soll das deiner Meinung nach laufen? 
Result Stats: id=71fc1cac-d49d-4728-bb65-73d1d1769780, total: 226321, in: 108, out: 74937, reason: 562, read: 4867900, write: 151276, start: 1795555081256, end: 1795555099719, time: 18415, total: 912712, io: 0, control: 48, sent: 1, recv: 157
SystemInit:  id=71fc1cac-d49d-4728-bb65-73d1d1769780, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-opus-5-5
ReasoningToken: 562
Ja, der eingefügte Knoten bekommt die ID `foo`. Die ID-Vergabe ist in Ordnung, das erste `foo` im Dokument heißt `foo`, das zweite `foo_1`. Falsch ist nur, wie `ast_insert` die neue ID *ermittelt*. Mein Satz "gibt falsche `ids` zurück" war missverständlich.

`_insert_one` vergleicht die ID-Menge vor und nach dem Einfügen und meldet alles, was vorher nicht da war. Ein Beispiel:

```
vorher:  foo        (return 1)
         foo.<hashA>

nachher: foo        (return 42)   ← eingefügt
         foo.<hashB>
         foo_1      (return 1)    ← alter Knoten, umbenannt
         foo_1.<hashA>
```

Der Vergleich ergibt `['foo.<hashB>', 'foo_1']`, genau das stand im fehlgeschlagenen Test (`['foo.gEQro1|svuKcz', 'foo_1']`). Daran sind zwei Dinge falsch:
- **`foo` fehlt**, obwohl es die neue ID ist. Es gab `foo` schon vorher, also gilt es für den Vergleich nicht als neu.
- **`foo_1` wird gemeldet**, obwohl das der alte Knoten unter neuem Namen ist.

Die Position umgeht das: Der eingefügte Knoten steht an der Stelle, an der vorher das Ziel stand. `ast_move` liest dort die ID ab und meldet korrekt `foo`. Das prüft der neue Test über den Code (`return 42`).

Unabhängig davon hat der bestehende Knoten jetzt die ID `foo_1` statt `foo`. Das ist eine Folge der ID-Vergabe und kein Fehler. Weder `ast_insert` noch `ast_move` melden diese Umbenennung aber, eine früher gemerkte ID `foo` zeigt danach also auf den neuen Knoten.
Dann ist das aber ein Fehler. Insert sollte dann zur ermittelung eher eine Conten Hash verwenden oder ähnlich.
SystemInit:  id=71fc1cac-d49d-4728-bb65-73d1d1769780, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-opus-5-5
Text:
 Ja, das ist ein Fehler in `ast_insert`. Ein reiner Content-Hash reicht allerdings nicht, um ihn zu beheben:

- **Was er löst:** `foo_1` hätte denselben Inhalt wie vorher `foo` und würde nicht mehr als neu gemeldet. Der eingefügte Knoten hat neuen Inhalt und würde erkannt.
- **Was er nicht löst:** Wird in eine Klasse eingefügt, ändert sich auch der Inhalt der Klasse und aller Vorfahren. Sie würden alle als neu erkannt, und der Filter auf Top-Level-Knoten würde dann ausgerechnet die Klasse statt des eingefügten Knotens liefern.

Verlässlich ist die Position, und das ist das „oder ähnlich“: Der Ort einer Einfügung ist durch Ziel und `before`/`after` genau bekannt. Ich lagere die Ermittlung, die schon in `ast_move` steckt, als gemeinsame Funktion nach `core.py` aus. Dann nutzen `ast_insert` und `ast_move` dieselbe Logik.
ReasoningToken: 3460
Control Request:
```yaml
id: 1f37-23f-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: _top_level\(|_place_each\(
    include:
    - move.py
  reason: "Aktuellen _place_each-Code und Verwendungen von _top_level in move.py ermitteln, um auf den gemeinsamen core-Helfer umzustellen"
```
/answer 1f37-23f-0 allow
Control Request:
```yaml
id: 1f37-240-23f
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
        matches:
        - path: move.py
          matches:
          - "def _top_level(ids: list[str]) -> list[str]:"
          - "def _place_each(dst_tree: core.Tree, target_id: str | None, codes: list[str], position: str) -> list[list[str] | None]:"
          - "return [_top_level([loc.node_id for loc in current[r[0]:r[0] + r[1]]]) if r else None for r in ranges]"
          - "top_ids = _top_level([loc.node_id for loc in core.locate_all(src_tree)])"
          - "new_top = _top_level([loc.node_id for loc in core.locate_all(dst_tree)])"
          - "created = _place_each(dst_tree, target.node_id if target else None, codes, item.position)"
```
/answer 1f37-240-23f allow
ReasoningToken: 288
Control Request:
```yaml
id: 1f37-241-0
toolName: ast_edit
arguments:
  insert:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
    id: append_nodes
    position: after
    source: |-
      @dataclass(frozen=True)
      class Placement:
          """Where an insert/append landed, in :func:`locate_all` order.

          Attributes:
              start: Index of the first inserted node after the edit.
              count: Number of addressable nodes the edit added.
              units: Units the engine reported as inserted/appended.
              exact: Whether the nodes outside ``[start, start + count)`` kept their structure
                  (node types). If not (e.g. merged statement groups), ``start``/``count``
                  cannot be trusted.
          """
          start: int
          count: int
          units: int
          exact: bool


      def top_level_ids(ids: list[str]) -> list[str]:
          """Those ``ids`` without an ancestor among ``ids``, in the given order."""
          return [i for i in ids if not any(i.startswith(other + '.') for other in ids if other != i)]


      def place_node(tree: Tree, loc: Located | None, code: str, position: str) -> Placement:
          """Insert ``code`` ``position`` ``loc`` (append to ``tree`` if ``loc`` is ``None``) and track where it landed.

          The inserted nodes are found by position, not by an id diff: inserting may rename
          existing nodes (a second ``foo`` turns the old one into ``foo_1``), so neither a
          new id nor a changed id proves a node new. In document order the inserted nodes
          start at the target's index (``before``), right behind its subtree (``after``) or
          at the end (append).
          """
          before = locate_all(tree)
          if loc is None:
              start = len(before)
              units = append_nodes(tree, code)
          else:
              index = next((i for i, other in enumerate(before) if other.node_id == loc.node_id), None)
              if index is None:
                  raise AstError('The target node is no longer present.')
              start = index
              if position != 'before':
                  start += 1 + sum(1 for other in before if other.node_id.startswith(loc.node_id + '.'))
              units = insert_node(loc, code, position)
          after = locate_all(tree)
          count = len(after) - len(before)
          rest = after[:start] + after[start + count:]
          exact = count > 0 and [other.node_type for other in rest] == [other.node_type for other in before]
          return Placement(start=start, count=count, units=units, exact=exact)


      def placed_ids(tree: Tree, placement: Placement) -> list[str] | None:
          """Top-level ids of the nodes ``placement`` inserted, or ``None`` if it is not exact."""
          if not placement.exact:
              return None
          located = locate_all(tree)[placement.start:placement.start + placement.count]
          return top_level_ids([loc.node_id for loc in located])
  replace:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: place_each
    source: |-
      def _place_each(dst_tree: core.Tree, target_id: str | None, codes: list[str], position: str) -> list[list[str] | None]:
          """Insert every code on its own; return the top-level ids each single insert created.

          Inserted nodes are located by position (:func:`core.place_node`); the target is
          tracked the same way, since an insert may rename it. If one insert changes the
          structure elsewhere, positions are no longer trustworthy and all results become
          ``None``. To keep the given order, nodes placed ``after`` the target are inserted
          last-first (each lands directly behind it).
          """
          located = core.locate_all(dst_tree)
          t = None if target_id is None else next(i for i, loc in enumerate(located) if loc.node_id == target_id)
          ranges: list[tuple[int, int] | None] = [None] * len(codes)
          exact = True
          order = list(range(len(codes)))
          if t is not None and position == 'after':
              order.reverse()
          for i in order:
              anchor = None if t is None else core.locate_all(dst_tree)[t]
              placement = core.place_node(dst_tree, anchor, codes[i], position)
              if not placement.exact:
                  exact = False
                  if t is not None and position == 'before':
                      '# Best effort to keep inserting in front of the target.'
                      t = next((j for j, loc in enumerate(core.locate_all(dst_tree)) if loc.node_id == target_id),
                               t + max(placement.count, 0))
                  continue
              p, k = placement.start, placement.count
              ranges = [r if r is None or r[0] < p else (r[0] + k, r[1]) for r in ranges]
              ranges[i] = (p, k)
              if t is not None and position == 'before':
                  t += k
          if not exact:
              return [None] * len(codes)
          current = core.locate_all(dst_tree)
          return [core.top_level_ids([loc.node_id for loc in current[r[0]:r[0] + r[1]]]) if r else None for r in ranges]
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: move_file
    old_text: top_ids = _top_level(
    new_text: top_ids = core.top_level_ids(
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: move_file
    old_text: new_top = _top_level(
    new_text: new_top = core.top_level_ids(
  delete:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: top_level
  reason: Positionsbasierte Einfüge-Ermittlung als gemeinsamen Helfer in core.py anlegen und ast_move darauf umstellen
```
/answer 1f37-241-0 allow
Text:
 Jetzt stelle ich `ast_insert` auf den Helfer um.
ReasoningToken: 95
Control Request:
```yaml
id: 1f37-242-0
toolName: ast_edit
arguments:
  replace:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: insert_one
    source: |-
      def _insert_one(item: InsertItem) -> InsertResult:
          file_path = core.require_path(item.path)
          tree = core.CACHE.get_tree(file_path)
          target = select_by_path(tree, id=item.id)
          placement = core.place_node(tree, target, item.source, item.position)
          new_ids = core.placed_ids(tree, placement)
          imports = core.apply_imports(tree, item.add_import, item.remove_import)
          core.CACHE.save(file_path, tree)
          return InsertResult(path=item.path, id=item.id, inserted=placement.units, ids=new_ids or None, imports=imports)
  reason: ast_insert ermittelt neue IDs positionsbasiert über core.place_node statt per ID-Diff
```
/answer 1f37-242-0 allow
Text:
 Ich ergänze Tests für `ast_insert`: bei Namenskollision, beim Einfügen in eine Klasse und mit mehreren Anweisungen.
ReasoningToken: 263
Control Request:
```yaml
id: 1f37-243-0
toolName: ast_edit
arguments:
  insert:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_move.py
    id: h2ee731a7
    position: after
    source: |-
      def test_insert_reports_inserted_id_on_name_clash(tmp_path: Path):
          from xy.ai.mcpc.tools.ast.insert import InsertItem, ast_insert
          path = _write(tmp_path, 'b.py', 'def foo():\n    return 1\n')
          result = ast_insert([InsertItem(path=path, id='foo', position='before', source='def foo():\n    return 42\n')])
          assert not result.errors
          assert result.results[0].ids == ['foo']
          assert 'return 42' in _code(path, 'foo')


      def test_insert_into_class_reports_only_the_new_nodes(tmp_path: Path):
          from xy.ai.mcpc.tools.ast.insert import InsertItem, ast_insert
          path = _write(tmp_path, 'c.py', CLASSES)
          source = 'def p(self):\n    return 5\n\ndef q(self):\n    return 6\n'
          result = ast_insert([InsertItem(path=path, id='B.k', source=source)])
          assert not result.errors
          assert result.results[0].ids == ['B.p', 'B.q']
  reason: Regressionstests für die positionsbasierte ID-Ermittlung von ast_insert ergänzen
```
/answer 1f37-243-0 allow
Control Request:
```yaml
id: 1f37-244-0
toolName: run_command
arguments:
  items:
  - id: runTests
    cwd: /home/user/xyan/xy.ai.workbench/mcpc
  reason: Gesamte Testsuite nach Umstellung von ast_insert und ast_move auf core.place_node ausführen
```
/answer 1f37-244-0 allow
Control Request:
```yaml
id: 1f37-245-244
result:
  structuredContent:
    results:
    - index: 0
      exit_code: 1
      stdout: |
        ============================= test session starts ==============================
        platform linux -- Python 3.12.3, pytest-8.3.5, pluggy-1.6.0
        rootdir: /home/user/xyan/xy.ai.workbench/mcpc
        configfile: pyproject.toml
        plugins: anyio-4.8.0, langsmith-0.4.18, docker-3.1.2
        collected 505 items

        tests/test_ast_engines.py ..................                             [  3%]
        tests/test_ast_imports.py ....................                           [  7%]
        tests/test_ast_imports_rpc.py ...........                                [  9%]
        tests/test_ast_markdown_bridge.py ...................................... [ 17%]
        ..................................................F                      [ 27%]
        tests/test_ast_move.py ...................                               [ 31%]
        tests/test_ast_retrieve.py ......                                        [ 32%]
        tests/test_ast_rust_bridge.py ....                                       [ 33%]
        tests/test_cli_session.py .................                              [ 36%]
        tests/test_edit_diagnostics.py .......................                   [ 40%]
        tests/test_edit_tolerance.py ...........................                 [ 46%]
        tests/test_embedded.py .............                                     [ 48%]
        tests/test_embedded_batch.py .                                           [ 49%]
        tests/test_grep_with_ast.py ...                                          [ 49%]
        tests/test_http_human_in_the_loop.py ..                                  [ 50%]
        tests/test_http_tool_calls.py ...                                        [ 50%]
        tests/test_import_all.py ............................................... [ 60%]
        ........................................................................ [ 74%]
        .......                                                                  [ 75%]
        tests/test_mcpcignore.py ........................                        [ 80%]
        tests/test_persist.py ..........                                         [ 82%]
        tests/test_result_contract.py ..................................         [ 89%]
        tests/test_rpc_java_engine.py ..........                                 [ 91%]
        tests/test_script_file_source.py ..............................          [ 97%]
        tests/test_smoke.py ..                                                   [ 97%]
        tests/test_tools_family_smoke.py ...                                     [ 98%]
        tests/test_validate_arguments.py ..........                              [100%]

        =================================== FAILURES ===================================
        __________ test_existing_engine_instance_recovers_after_process_crash __________
        [XPASS(strict)] RpcEngine caches its client and only starts the process when the client is created, so an already used engine instance (e.g. from get_engine) never recovers from a crashed process.
        =========================== short test summary info ============================
        FAILED tests/test_ast_markdown_bridge.py::test_existing_engine_instance_recovers_after_process_crash
        ======================== 1 failed, 504 passed in 15.78s ========================
```
/answer 1f37-245-244 allow
Result Stats: id=71fc1cac-d49d-4728-bb65-73d1d1769780, total: 247534, in: 120, out: 84185, reason: 82, read: 5812006, write: 163229, start: 1795555380078, end: 1795555390031, time: 9907, total: 1033032, io: 0, control: 46, sent: 0, recv: 152
ReasoningToken: 82
`ast_insert` ermittelt die neuen IDs jetzt über die Position statt über den Vergleich der IDs vor und nach dem Einfügen. Ein reiner Content-Hash hätte nicht gereicht, weil beim Einfügen in eine Klasse auch die Klasse selbst ihren Inhalt ändert und als neu erkannt worden wäre. Alle 19 Tests in `test_ast_move.py` laufen durch. Von der ganzen Suite schlägt weiterhin nur der XPASS-Test in `test_ast_markdown_bridge` fehl.

**Neuer gemeinsamer Helfer in `core.py`:** `place_node(tree, loc, code, position)` fügt ein bzw. hängt an und merkt sich, wo der Code gelandet ist. In Dokumentreihenfolge stehen die eingefügten Knoten bei `before` am Index des Ziels, bei `after` direkt hinter dessen Teilbaum und beim Anhängen am Ende. `placed_ids()` liefert daraus die Top-Level-IDs.

Zusätzlich wird geprüft, dass sich außerhalb des eingefügten Bereichs die Folge der Knotentypen nicht geändert hat. Ändert sie sich doch, etwa weil zwei Python-Anweisungsgruppen verschmelzen, werden keine IDs gemeldet, statt falsche.

**Wer den Helfer nutzt:**
- **`ast_insert`:** Bei einer Namenskollision liefert es jetzt korrekt `ids == ['foo']`.
- **`ast_move`:** Es verwendet denselben Helfer, die eigene Kopie der Logik ist entfernt.

**Geänderte Rückgabe von `ast_insert`:**
- **Nur noch Top-Level-IDs:** Bisher kamen auch Kind-IDs mit, obwohl der Docstring nur die obersten neuen Knoten verspricht. Beim Einfügen von `p` und `q` in Klasse `B` kommt jetzt genau `['B.p', 'B.q']`. Das hat ein neuer Test bestätigt.
- **`ids` kann fehlen:** Das passiert, wenn die Strukturprüfung fehlschlägt.

Wird ein bestehender Knoten durch das Einfügen umbenannt (`foo` → `foo_1`), meldet das weiterhin keines der beiden Tools.
Warte
Result Stats: id=71fc1cac-d49d-4728-bb65-73d1d1769780, total: 248607, in: 122, out: 84300, reason: 35, read: 5975235, write: 164185, start: 1795555679784, end: 1795555686818, time: 6989, total: 1040021, io: 0, control: 45, sent: 1, recv: 31
SystemInit:  id=71fc1cac-d49d-4728-bb65-73d1d1769780, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-opus-5-5
ReasoningToken: 35
Ich warte und ändere nichts weiter. Den aktuellen Stand habe ich in der letzten Nachricht zusammengefasst, alle Änderungen liegen ungespeichert im Arbeitsverzeichnis.
Lies `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py`. Einen import in einer Datei hinzuzufügen aus der ein Knoten entfernt wird macht keinen Sinn, genausowenig wie das entfernen aus der Zieldatei. Entferne die beiden Parameter. 
SystemInit:  id=71fc1cac-d49d-4728-bb65-73d1d1769780, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-opus-5-5
ReasoningToken: 51
Control Request:
```yaml
id: 1f37-246-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    regexp: remove_import|source_add_import|removeImport|sourceAddImport|parse_import_args
  reason: "move.py lesen, um removeImport (Ziel) und sourceAddImport (Quelle) überall zu entfernen"
```
/answer 1f37-246-0 allow
Control Request:
```yaml
id: 1f37-247-246
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
        nodes:
        - id: ohrFT4|fn3nGt
          code: |-
            from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolFeature, ToolRegistry, ToolResult
            from xy.ai.mcpc.tools.ast.imports import IMPORT_NOTICE, ImportCheckedBatchResult, ImportOutcome, parse_import_args
            from dataclasses import dataclass, replace
        - id: MoveItem.vS3oji|XYZSfk
          code: "'One move operation.\\n\\n    Attributes:\\n        path: Absolute path of the source file.\\n        target: Absolute path of the target file. May equal ``path`` (move within the\\n            file); a missing target file is created like ``ast_create`` does.\\n        ids: Ids of the source nodes to move, inserted in the given order. Empty/``None``\\n            moves the whole file. A node listed together with one of its ancestors is\\n            moved along with that ancestor.\\n        target_id: Id of the target node in the target file. ``None`` appends at the\\n            end of the target file. An unknown ``target_id`` aborts the item.\\n        position: ``\"before\"`` or ``\"after\"`` the target node. Defaults to ``\"after\"``.\\n        add_import / remove_import: Import statements to add/remove in the target file.\\n        source_add_import / source_remove_import: Import statements to add/remove in the\\n            source file (ignored and flagged if the source file is deleted).\\n    '"
        - id: MoveItem.ihDxNz|wvoLhG
          code: |-
            path: str
            target: str
            ids: list[str] | None = None
            target_id: str | None = None
            position: str = 'after'
            add_import: list[str] | None = None
            remove_import: list[str] | None = None
            source_add_import: list[str] | None = None
            source_remove_import: list[str] | None = None
        - id: move_file.h5Znf8|AuTcds
          code: |-
            if _same_file(src_path, dst_path):
                raise core.AstError('Moving a whole file onto itself is not possible; give source ids.')
            src_tree = core.CACHE.get_tree(src_path)
            top_ids = core.top_level_ids([loc.node_id for loc in core.locate_all(src_tree)])
            source_imports = ImportOutcome(unchecked=True) if item.source_add_import or item.source_remove_import else None
        - id: move_file.DDbXMz|TgMGiH
          code: |-
            if not dst_path.exists() and src_path.suffix.lower() == dst_path.suffix.lower():
                if item.target_id:
                    raise core.AstError('Target file does not exist; omit targetId to create it.')
                '# Plain copy like ast_create: same text and engine give the same outline and ids.'
                dst_tree = core.parse_for(item.target, src_path.read_text(encoding='utf-8'))
                new_top = core.top_level_ids([loc.node_id for loc in core.locate_all(dst_tree)])
                try:
                    imports = core.apply_imports(dst_tree, item.add_import, item.remove_import)
                    _save(dst_path, dst_tree)
                except BaseException:
                    core.CACHE.invalidate(dst_path)
                    raise
                _remove_file(src_path)
                moved = list(zip(top_ids, new_top)) if len(top_ids) == len(new_top) else [(i, None) for i in top_ids]
                return MoveResult(
                    path=item.path,
                    target=item.target,
                    moved=moved,
                    imports=imports,
                    source_imports=source_imports,
                    source_deleted=True)
        - id: move_nodes.bnB3Pc|DWteW6
          code: |-
            try:
                if same:
                    dst_tree = src_tree
                    target = _move_within(item, src_tree, all_src, roots, removal)
                else:
                    if dst_path.exists():
                        dst_tree = core.CACHE.get_tree(dst_path)
                        target = select_by_path(dst_tree, id=item.target_id) if item.target_id else None
                    else:
                        if item.target_id:
                            raise core.AstError('Target file does not exist; omit targetId to create it.')
                        dst_tree = core.empty_tree(dst_path)
                        target = None
                    '# Delete from the source in reverse document order so the remaining nodes stay addressed.'
                    for loc in removal:
                        core.delete_node(loc)
                created = _place_each(dst_tree, target.node_id if target else None, codes, item.position)
                new_all = {loc.node_id for loc in core.locate_all(dst_tree)}
                if same:
                    imports = core.apply_imports(dst_tree, [*
                                                            (item.add_import or []), *
                                                            (item.source_add_import or [])] or None, [*
                                                                                                      (item.remove_import or []), *
                                                                                                      (item.source_remove_import or [])] or None)
                    source_imports = None
                    _save(dst_path, dst_tree)
                    source_deleted = False
                else:
                    imports = core.apply_imports(dst_tree, item.add_import, item.remove_import)
                    source_imports = core.apply_imports(src_tree, item.source_add_import, item.source_remove_import)
                    _save(dst_path, dst_tree)
                    source_deleted = _is_empty(src_tree)
                    if source_deleted:
                        _remove_file(src_path)
                    else:
                        _save(src_path, src_tree)
            except BaseException:
                core.CACHE.invalidate(src_path)
                core.CACHE.invalidate(dst_path)
                raise
        - id: MoveTool.ANYI2U|8EDlZ2
          code: |-
            _ITEM_PROPERTIES = {
                'path': {
                    'type': 'string',
                    'description': 'Absolute path of the source file.'},
                'target': {
                    'type': 'string',
                    'description': 'Absolute path of the target file; may equal path to move within the file. Created if missing.'},
                'ids': {
                    'type': 'array',
                            'items': {
                                'type': 'string',
                                'minLength': 1},
                    'description': 'Ids of the source nodes to move. Omit or leave empty to move the whole file. To move a container (e.g. an inner class) give its own id; listing only its children leaves the container in place.'},
                'targetId': {
                    'type': 'string',
                    'description': 'Id of the node in the target file to insert relative to.'},
                'position': {
                    'type': 'string',
                    'enum': [
                        'before',
                        'after'],
                    'description': 'Placement relative to targetId.',
                    'default': 'after'},
                'addImport': {
                    **IMPORT_PROPS['addImport'],
                    'description': IMPORT_PROPS['addImport']['description'] + ' Applied to the target file.'},
                'removeImport': {
                    **IMPORT_PROPS['removeImport'],
                    'description': IMPORT_PROPS['removeImport']['description'] + ' Applied to the target file.'},
                'sourceAddImport': {
                    **IMPORT_PROPS['addImport'],
                    'description': 'Import statements to add to the source file if missing.'},
                'sourceRemoveImport': {
                    **IMPORT_PROPS['removeImport'],
                    'description': 'Import statements the source file no longer needs.'}}
        - id: MoveTool.handle.item_factory.R8fGaY|skCovq
          code: |-
            add_import, remove_import = parse_import_args(it)
            target_id = it.get('targetId')
            if target_id is not None and (not isinstance(target_id, str)):
                raise ValueError("'targetId' must be a string")
        - id: MoveTool.handle.item_factory.ua2Nxh|QdkV6u
          code: |-
            return MoveItem(
                path=it['path'], target=it['target'], ids=_string_list(
                    it, 'ids'), target_id=target_id or None, position=it.get(
                        'position', 'after'), add_import=add_import, remove_import=remove_import, source_add_import=_string_list(
                            it, 'sourceAddImport'), source_remove_import=_string_list(
                                it, 'sourceRemoveImport'))
```
/answer 1f37-247-246 allow
Thinking:
Ich entferne die beiden doppelten Import-Methoden, `removeImport` und `sourceAddImport`, und behalte `addImport` sowie `sourceRemoveImport`. Deine Änderungen an den Beschreibungen bleiben dabei unberührt.


ReasoningToken: 416
Control Request:
```yaml
id: 1f37-248-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: MoveItem.vS3oji|XYZSfk
    old_text: "add_import / remove_import: Import statements to add/remove in the target file.\\n        source_add_import / source_remove_import: Import statements to add/remove in the\\n            source file (ignored and flagged if the source file is deleted).\\n"
    new_text: "add_import: Import statements to add to the target file if missing.\\n        source_remove_import: Import statements to remove from the source file if present\\n            (ignored and flagged if the source file is deleted).\\n"
    exact: true
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: MoveItem.ihDxNz|wvoLhG
    old_text: |
      add_import: list[str] | None = None
      remove_import: list[str] | None = None
      source_add_import: list[str] | None = None
    new_text: |
      add_import: list[str] | None = None
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: move_file.h5Znf8|AuTcds
    old_text: if item.source_add_import or item.source_remove_import else None
    new_text: if item.source_remove_import else None
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: move_file.DDbXMz|TgMGiH
    old_text: "core.apply_imports(dst_tree, item.add_import, item.remove_import)"
    new_text: "core.apply_imports(dst_tree, item.add_import, None)"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: MoveTool.ANYI2U|8EDlZ2
    old_text: |2
          'removeImport': {
              **IMPORT_PROPS['removeImport'],
              'description': IMPORT_PROPS['removeImport']['description'] + ' Applied to the target file.'},
          'sourceAddImport': {
              **IMPORT_PROPS['addImport'],
              'description': 'Import statements to add to the source file if missing.'},
    new_text: ""
    exact: true
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: MoveTool.handle.item_factory.R8fGaY|skCovq
    old_text: |
      add_import, remove_import = parse_import_args(it)
    new_text: |
      add_import = _string_list(it, 'addImport')
    removeImport:
    - "from xy.ai.mcpc.tools.ast.imports import IMPORT_NOTICE, ImportCheckedBatchResult, ImportOutcome, parse_import_args"
    addImport:
    - "from xy.ai.mcpc.tools.ast.imports import IMPORT_NOTICE, ImportCheckedBatchResult, ImportOutcome"
  replace:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: MoveTool.handle.item_factory.ua2Nxh|QdkV6u
    source: |-
      return MoveItem(
          path=it['path'],
          target=it['target'],
          ids=_string_list(it, 'ids'),
          target_id=target_id or None,
          position=it.get('position', 'after'),
          add_import=add_import,
          source_remove_import=_string_list(it, 'sourceRemoveImport'))
  reason: "Parameter removeImport (Ziel) und sourceAddImport (Quelle) aus ast_move entfernen: Item-Dataclass, Logik, Schema und Item-Factory"
```
/answer 1f37-248-0 allow
Text:
 Jetzt noch die Import-Behandlung in `_move_nodes`:
Control Request:
```yaml
id: 1f37-249-0
toolName: ast_edit
arguments:
  edit_marks:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: move_nodes.bnB3Pc|DWteW6
    start_marker: "imports = core.apply_imports(dst_tree, [*"
    end_marker: "source_remove_import or [])] or None)"
    content: "imports = core.apply_imports(dst_tree, item.add_import, item.source_remove_import)"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: move_nodes.bnB3Pc|DWteW6
    start_marker: "imports = core.apply_imports(dst_tree, item.add_import, item.remove_import)"
    end_marker: |
      item.source_remove_import)
    content: |
      imports = core.apply_imports(dst_tree, item.add_import, None)
              source_imports = core.apply_imports(src_tree, None, item.source_remove_import)
  reason: Import-Behandlung in _move_nodes auf addImport (Ziel) und sourceRemoveImport (Quelle) reduzieren
```
/answer 1f37-249-0 allow
Control Request:
```yaml
id: 1f37-24a-249
result:
  structuredContent:
    edit_marks:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
        id: move_nodes.bnB3Pc|DWteW6
        new_id: move_nodes.bnB3Pc|DWteW6
      errors:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
        id: move_nodes.bnB3Pc|DWteW6
        error: End marker is ambiguous – found 2 occurrences in node.
        reason: ambiguous
```
/answer 1f37-24a-249 allow
Control Request:
```yaml
id: 1f37-24b-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: move_nodes.bnB3Pc|DWteW6
    old_text: "imports = core.apply_imports(dst_tree, item.add_import, item.remove_import)"
    new_text: "imports = core.apply_imports(dst_tree, item.add_import, None)"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    id: move_nodes.bnB3Pc|DWteW6
    old_text: "core.apply_imports(src_tree, item.source_add_import, item.source_remove_import)"
    new_text: "core.apply_imports(src_tree, None, item.source_remove_import)"
  reason: Verbleibende Import-Zeilen im Cross-File-Zweig von _move_nodes anpassen
```
/answer 1f37-24b-0 allow
Control Request:
```yaml
id: 1f37-24c-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    - /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: "[^_]remove_import|source_add_import|'removeImport'|sourceAddImport|item\\.remove_import"
    include:
    - move.py
    - test_ast_move.py
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
    text: apply_imports
  reason: "Prüfen, dass keine Verweise auf remove_import/source_add_import/removeImport/sourceAddImport in move.py und Tests übrig sind"
```
/answer 1f37-24c-0 allow
Control Request:
```yaml
id: 1f37-24d-24c
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
        - /home/user/xyan/xy.ai.workbench/mcpc/tests
        matches:
        - path: move.py
          matches:
          - "**IMPORT_PROPS['removeImport'],"
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/move.py
        nodes:
        - id: move_file.DDbXMz|HEZsmy
          code: |-
            if not dst_path.exists() and src_path.suffix.lower() == dst_path.suffix.lower():
                if item.target_id:
                    raise core.AstError('Target file does not exist; omit targetId to create it.')
                '# Plain copy like ast_create: same text and engine give the same outline and ids.'
                dst_tree = core.parse_for(item.target, src_path.read_text(encoding='utf-8'))
                new_top = core.top_level_ids([loc.node_id for loc in core.locate_all(dst_tree)])
                try:
                    imports = core.apply_imports(dst_tree, item.add_import, None)
                    _save(dst_path, dst_tree)
                except BaseException:
                    core.CACHE.invalidate(dst_path)
                    raise
                _remove_file(src_path)
                moved = list(zip(top_ids, new_top)) if len(top_ids) == len(new_top) else [(i, None) for i in top_ids]
                return MoveResult(
                    path=item.path,
                    target=item.target,
                    moved=moved,
                    imports=imports,
                    source_imports=source_imports,
                    source_deleted=True)
        - id: move_nodes.bnB3Pc|RRHdIf
          code: |-
            try:
                if same:
                    dst_tree = src_tree
                    target = _move_within(item, src_tree, all_src, roots, removal)
                else:
                    if dst_path.exists():
                        dst_tree = core.CACHE.get_tree(dst_path)
                        target = select_by_path(dst_tree, id=item.target_id) if item.target_id else None
                    else:
                        if item.target_id:
                            raise core.AstError('Target file does not exist; omit targetId to create it.')
                        dst_tree = core.empty_tree(dst_path)
                        target = None
                    '# Delete from the source in reverse document order so the remaining nodes stay addressed.'
                    for loc in removal:
                        core.delete_node(loc)
                created = _place_each(dst_tree, target.node_id if target else None, codes, item.position)
                new_all = {loc.node_id for loc in core.locate_all(dst_tree)}
                if same:
                    imports = core.apply_imports(dst_tree, item.add_import, item.source_remove_import)
                    source_imports = None
                    _save(dst_path, dst_tree)
                    source_deleted = False
                else:
                    imports = core.apply_imports(dst_tree, item.add_import, None)
                    source_imports = core.apply_imports(src_tree, None, item.source_remove_import)
                    _save(dst_path, dst_tree)
                    source_deleted = _is_empty(src_tree)
                    if source_deleted:
                        _remove_file(src_path)
                    else:
                        _save(src_path, src_tree)
            except BaseException:
                core.CACHE.invalidate(src_path)
                core.CACHE.invalidate(dst_path)
                raise
```
/answer 1f37-24d-24c allow
Text:
 Übrig ist nur noch `IMPORT_PROPS['removeImport']` als Basis für `sourceRemoveImport`, das ist so gewollt. Jetzt die Tests:
ReasoningToken: 43
Control Request:
```yaml
id: 1f37-24e-0
toolName: run_command
arguments:
  items:
  - id: runTests
    cwd: /home/user/xyan/xy.ai.workbench/mcpc
  reason: Tests nach Entfernen der Parameter removeImport/sourceAddImport ausführen
```
/answer 1f37-24e-0 allow
Control Request:
```yaml
id: 1f37-24f-24e
result:
  structuredContent:
    results:
    - index: 0
      exit_code: 1
      stdout: |
        ============================= test session starts ==============================
        platform linux -- Python 3.12.3, pytest-8.3.5, pluggy-1.6.0
        rootdir: /home/user/xyan/xy.ai.workbench/mcpc
        configfile: pyproject.toml
        plugins: anyio-4.8.0, langsmith-0.4.18, docker-3.1.2
        collected 505 items

        tests/test_ast_engines.py ..................                             [  3%]
        tests/test_ast_imports.py ....................                           [  7%]
        tests/test_ast_imports_rpc.py ...........                                [  9%]
        tests/test_ast_markdown_bridge.py ...................................... [ 17%]
        ..................................................F                      [ 27%]
        tests/test_ast_move.py ...................                               [ 31%]
        tests/test_ast_retrieve.py ......                                        [ 32%]
        tests/test_ast_rust_bridge.py ....                                       [ 33%]
        tests/test_cli_session.py .................                              [ 36%]
        tests/test_edit_diagnostics.py .......................                   [ 40%]
        tests/test_edit_tolerance.py ...........................                 [ 46%]
        tests/test_embedded.py .............                                     [ 48%]
        tests/test_embedded_batch.py .                                           [ 49%]
        tests/test_grep_with_ast.py ...                                          [ 49%]
        tests/test_http_human_in_the_loop.py ..                                  [ 50%]
        tests/test_http_tool_calls.py ...                                        [ 50%]
        tests/test_import_all.py ............................................... [ 60%]
        ........................................................................ [ 74%]
        .......                                                                  [ 75%]
        tests/test_mcpcignore.py ........................                        [ 80%]
        tests/test_persist.py ..........                                         [ 82%]
        tests/test_result_contract.py ..................................         [ 89%]
        tests/test_rpc_java_engine.py ..........                                 [ 91%]
        tests/test_script_file_source.py ..............................          [ 97%]
        tests/test_smoke.py ..                                                   [ 97%]
        tests/test_tools_family_smoke.py ...                                     [ 98%]
        tests/test_validate_arguments.py ..........                              [100%]

        =================================== FAILURES ===================================
        __________ test_existing_engine_instance_recovers_after_process_crash __________
        [XPASS(strict)] RpcEngine caches its client and only starts the process when the client is created, so an already used engine instance (e.g. from get_engine) never recovers from a crashed process.
        =========================== short test summary info ============================
        FAILED tests/test_ast_markdown_bridge.py::test_existing_engine_instance_recovers_after_process_crash
        ======================== 1 failed, 504 passed in 15.83s ========================
```
/answer 1f37-24f-24e allow
Result Stats: id=71fc1cac-d49d-4728-bb65-73d1d1769780, total: 263945, in: 136, out: 88723, reason: 0, read: 7159643, write: 175086, start: 1795555896862, end: 1795555904698, time: 7792, total: 1122685, io: 0, control: 44, sent: 0, recv: 97
`removeImport` (Zieldatei) und `sourceAddImport` (Quelldatei) sind aus `move.py` entfernt. Übrig sind `addImport` für die Zieldatei und `sourceRemoveImport` für die Quelldatei. Alle 19 Tests in `test_ast_move.py` laufen durch; in der ganzen Suite schlägt nur noch der XPASS-Test in `test_ast_markdown_bridge` fehl.

Entfernt habe ich die beiden Parameter an diesen Stellen:
- **`MoveItem`:** Felder `remove_import` und `source_add_import` samt Docstring.
- **Schema:** Properties `removeImport` und `sourceAddImport`.
- **`item_factory`:** liest nur noch `addImport` und `sourceRemoveImport`. `parse_import_args` brauche ich dafür nicht mehr, der Import ist entfernt.
- **Import-Logik:**
  - **Anderes Ziel:** `addImport` gilt nur für das Ziel, `sourceRemoveImport` nur für die Quelle.
  - **Ganze Datei:** `addImport` gilt für das Ziel. Wird `sourceRemoveImport` angegeben, obwohl die Quelldatei danach gelöscht ist, gibt es wie bisher einen Hinweis.
  - **Gleiche Datei:** Beide gelten für dieselbe Datei.

Deine Änderungen an den Beschreibungen habe ich nicht angefasst. Die Tests nutzen nur die verbliebenen Parameter und mussten nicht angepasst werden.
