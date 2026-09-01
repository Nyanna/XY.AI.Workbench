Die AST Tools in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast` create/create_file und delete/delete_file, sollen jeweils zusammengelegt werden.
"create" erstellt eine Datei auf Basis von source und soll ein noch nicht existierendes Zielverzeichnis rekursiv anlegen. Ein Einzelknoten Create wird bereits von Insert abgebildet.
"delete" soll sowohl knoten aus einem Baum löschen als auch eine Datei löschen, wenn der Root knoten gewählt wird. Ein danach potenziell leeres Verzeichnis soll gelöscht werden.
SystemInit:  id=c59e3957-2ae9-4ef5-99b6-9bf28c45839a, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 0e22-1
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
  reason: "Struktur der AST Tools ansehen, um create/create_file und delete/delete_file zu finden"
```
/answer 0e22-1 allow
Control Request:
```yaml
id: 0e22-2
result:
  structuredContent:
    entries:
    - ".:"
    - __init__.py
    - base.py
    - common.py
    - core.py
    - create.py
    - create_file.py
    - delete.py
    - delete_file.py
    - edit.py
    - find.py
    - insert.py
    - list.py
    - outline.py
    - read.py
    - replace.py
    - script.py
    - validate.py
    - "./generic:"
    - __init__.py
    - "./python:"
    - __init__.py
```
/answer 0e22-2 allow
ReasoningToken: 13
Control Request:
```yaml
id: 0e22-3
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
  reason: create.py Inhalt lesen
```
/answer 0e22-3 allow
Control Request:
```yaml
id: 0e22-4
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create_file.py
  reason: create_file.py Inhalt lesen
```
/answer 0e22-4 allow
Control Request:
```yaml
id: 0e22-5
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
  reason: delete.py Inhalt lesen
```
/answer 0e22-5 allow
Control Request:
```yaml
id: 0e22-6
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete_file.py
  reason: delete_file.py Inhalt lesen
```
/answer 0e22-6 allow
Control Request:
```yaml
id: 0e22-7
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  reason: "core.py Inhalt lesen für require_path, delete_node, append_nodes, CACHE etc."
```
/answer 0e22-7 allow
Control Request:
```yaml
id: 0e22-8
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
  reason: "common.py Inhalt lesen für select_one, SELECTOR_PROPS"
```
/answer 0e22-8 allow
Control Request:
```yaml
id: 0e22-9
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
  reason: __init__.py Inhalt lesen für Registrierung der Tools
```
/answer 0e22-9 allow
Control Request:
```yaml
id: 0e22-a
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
  reason: insert.py Inhalt lesen für Vergleich mit create (Einzelknoten Create)
```
/answer 0e22-a allow
ReasoningToken: 127
Control Request:
```yaml
id: 0e22-b
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  reason: base.py Inhalt lesen für Located/Tree/find/root Definition
```
/answer 0e22-b allow
ReasoningToken: 83
Control Request:
```yaml
id: 0e22-c
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python
  reason: Python engine Dateien auflisten
```
/answer 0e22-c allow
Control Request:
```yaml
id: 0e22-d
result:
  structuredContent:
    entries:
    - ".:"
    - __init__.py
```
/answer 0e22-d allow
Control Request:
```yaml
id: 0e22-e
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic
  reason: Generic engine Dateien auflisten
```
/answer 0e22-e allow
Control Request:
```yaml
id: 0e22-f
result:
  structuredContent:
    entries:
    - ".:"
    - __init__.py
```
/answer 0e22-f allow
ReasoningToken: 104
Control Request:
```yaml
id: 0e22-10
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/__init__.py
  reason: "Python engine Implementierung lesen (locate_all, delete, node_id root)"
```
/answer 0e22-10 allow
Control Request:
```yaml
id: 0e22-11
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
  reason: "Generic engine Implementierung lesen (locate_all, delete, root Handling)"
```
/answer 0e22-11 allow
ReasoningToken: 285
Control Request:
```yaml
id: 0e22-12
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -rn "ast_create_file\|ast_delete_file\|create_file\|delete_file" --include="*.py" mcpc/src | grep -v "/tools/ast/create_file.py\|/tools/ast/delete_file.py"
  reason: Weitere Referenzen auf ast_create_file/ast_delete_file im Code finden
```
/answer 0e22-12 deny nicht relevant
Control Request:
```yaml
id: 0e22-13
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -rln "ast_create_file\|ast_delete_file" mcpc/ --include="*.md" --include="*.txt" --include="*.rst" 2>/dev/null; grep -rn "ast_create\b\|ast_delete\b" mcpc/src --include="*.py" | grep -v "/tools/ast/create.py\|/tools/ast/delete.py\|/tools/ast/__init__.py"
  reason: Weitere Doku-Referenzen und Nutzungen von ast_create/ast_delete prüfen
```
/answer 0e22-13 deny nicht relevant
ReasoningToken: 2191
Control Request:
```yaml
id: 0e22-14
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
  pattern: script|find
  reason: Prüfen ob script.py oder find.py ast_create_file/ast_delete_file referenzieren
```
/answer 0e22-14 deny tun sie nicht
ReasoningToken: 110
Control Request:
```yaml
id: 0e22-15
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc/src
  script: grep -rn "create_file\|delete_file" .
  reason: "Alle Vorkommen von create_file/delete_file im src-Verzeichnis finden, um Referenzen zu identifizieren, die nach dem Zusammenlegen aktualisiert werden müssen"
```
/answer 0e22-15 deny nicht relevant
ReasoningToken: 67
Control Request:
```yaml
id: 0e22-16
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src
  pattern: .
  reason: "Gesamte Verzeichnisstruktur des src-Ordners auflisten, um alle Python-Dateien zu sehen die evtl. ast_create_file/ast_delete_file referenzieren"
```
/answer 0e22-16 deny nicht relevant
ReasoningToken: 89
Control Request:
```yaml
id: 0e22-17
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
  mode: replace
  reason: create.py und create_file.py zu einem ast_create Tool zusammenlegen
  content: |
    """``ast_create`` tool: create a file from source, creating missing directories."""


    from dataclasses import dataclass
    from typing import Any

    from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import ToolContext
    from xy.ai.mcpc.tools.ast import core
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry

    __all__ = ["CreateFileResult", "ast_create", "CreateFileTool", "register"]


    @dataclass(frozen=True)
    class CreateFileResult:
        """Result of :func:`ast_create`.

        Attributes:
            result: Always ``"success"``.
        """

        result: str


    def ast_create(path: str, source: str, overwrite: bool = False) -> CreateFileResult:
        """Create a new file at ``path`` from ``source`` (validated by parsing it).

        Missing parent directories are created recursively. Creating a single node
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
        if file_path.exists() and not overwrite:
            raise core.AstError("File already exists.")
        tree = core.parse_for(path, source)
        file_path.parent.mkdir(parents=True, exist_ok=True)
        core.CACHE.save(file_path, tree)
        return CreateFileResult(result="success")


    class CreateFileTool(ToolDefinition):
        name = "ast_create"
        title = "Create a file"
        description = "Create a file from source (validated by parsing it), creating missing directories."
        input_schema = {
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path of the file to create."},
                "source": {"type": "string", "description": "Source for the new file."},
                "overwrite": {
                    "type": "boolean",
                    "description": "Allow replacing an existing file.",
                    "default": False,
                },
            },
            "required": ["path", "source"],
        }
        output_schema = {
            "type": "object",
            "properties": {"result": {"type": "string"}},
            "required": ["result"],
        }
        annotations = {"readOnlyHint": False, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`ast_create`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = ast_create(
                    path=args["path"], source=args["source"], overwrite=args.get("overwrite", False)
                )
            except core.AstError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content={"result": result.result}, auto_approve=True)


    def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
        registry.register(CreateFileTool())
        functions.register(ast_create)
```
/answer 0e22-17 allow
ReasoningToken: 222
Control Request:
```yaml
id: 0e22-18
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
  mode: replace
  reason: delete.py und delete_file.py zu einem ast_delete Tool zusammenlegen
  content: |
    """``ast_delete`` tool: delete a selected node, or the whole file if none is selected."""


    from dataclasses import dataclass
    from typing import Any

    from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import ToolContext
    from xy.ai.mcpc.tools.ast import core
    from xy.ai.mcpc.tools.ast.common import SELECTOR_PROPS, select_one
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry

    __all__ = ["DeleteResult", "ast_delete", "DeleteTool", "register"]


    @dataclass(frozen=True)
    class DeleteResult:
        """Result of :func:`ast_delete`.

        Attributes:
            result: Always ``"success"``.
        """

        result: str


    def ast_delete(
        path: str,
        *,
        id: str | None = None,
        qualified_name: str | None = None,
        name: str | None = None,
        node_type: str | None = None,
        lineno: int | None = None,
        end_lineno: int | None = None,
        parent_type: str | None = None,
    ) -> DeleteResult:
        """Delete the single selected node, or the whole file if the root is selected.

        The root node is selected by omitting every selector – there is no other way
        to address it, since it is never itself an addressable child. Deleting the
        file also removes it from the AST cache and, if its parent directory becomes
        empty as a result, removes that directory too.

        Args:
            path: Absolute path to the file to modify.
            id: Selector – engine-independent node id.
            qualified_name: Selector – exact qualified name of the target node.
            name: Selector – exact simple name of the target node.
            node_type: Selector – node type name of the target node.
            lineno: Selector – exact start line of the target node.
            end_lineno: Selector – exact end line of the target node.
            parent_type: Selector – node type name of the target node's container.

        Returns:
            DeleteResult: Success status.

        Raises:
            core.AstError: If ``path`` is invalid, or a selector is given but matches
                zero or more than one node.
        """
        file_path = core.require_path(path)
        selectors = dict(
            id=id,
            qualified_name=qualified_name,
            name=name,
            node_type=node_type,
            lineno=lineno,
            end_lineno=end_lineno,
            parent_type=parent_type,
        )
        if all(value is None for value in selectors.values()):
            try:
                file_path.unlink()
            except OSError as exc:
                raise core.AstError("Delete failed.") from exc
            core.CACHE.invalidate(file_path)
            parent = file_path.parent
            if not any(parent.iterdir()):
                parent.rmdir()
            return DeleteResult(result="success")

        tree = core.CACHE.get_tree(file_path)
        target = select_one(tree, **selectors)
        core.delete_node(target)
        core.CACHE.save(file_path, tree)
        return DeleteResult(result="success")


    class DeleteTool(ToolDefinition):
        name = "ast_delete"
        title = "Delete AST node or file"
        description = (
            "Delete the single selected node from a file, or the whole file – and its "
            "directory if it becomes empty – if no selector is given."
        )
        input_schema = {
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path to the file."},
                **SELECTOR_PROPS,
            },
            "required": ["path"],
        }
        output_schema = {
            "type": "object",
            "properties": {"result": {"type": "string"}},
            "required": ["result"],
        }
        annotations = {"readOnlyHint": False, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`ast_delete`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = ast_delete(
                    args["path"],
                    id=args.get("id"),
                    qualified_name=args.get("qualified_name"),
                    name=args.get("name"),
                    node_type=args.get("node_type"),
                    lineno=args.get("lineno"),
                    end_lineno=args.get("end_lineno"),
                    parent_type=args.get("parent_type"),
                )
            except core.AstError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content={"result": result.result}, auto_approve=True)


    def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
        registry.register(DeleteTool())
        functions.register(ast_delete)
```
/answer 0e22-18 allow
Control Request:
```yaml
id: 0e22-19
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
  script: rm -f create_file.py delete_file.py
  reason: "Alte, nun zusammengelegte Dateien create_file.py und delete_file.py entfernen"
```
/answer 0e22-19 allow
Control Request:
```yaml
id: 0e22-1a
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
  mode: replace
  reason: Registrierung nach Zusammenlegen von create/create_file und delete/delete_file anpassen
  content: |
    """``ast_*`` tool family built on the standard-library ``ast`` module.

    A content-hash validated cache (:mod:`.core`) holds parsed modules; comments are
    converted to standalone string-literal annotations on import so they survive the
    ``parse``/``unparse`` round-trip. The tools cover a structural ``outline``,
    node-level CRUD (each tool in its own ``*`` module, ``ast_create``/``ast_delete``
    covering the whole-file case too), the imports/classes/functions convenience
    layers, a node-scoped ``replace_block``, a restricted ``script`` and a
    ``validate`` compile check.
    """


    from xy.ai.mcpc.tools.tool_registry import ToolRegistry
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry
    from xy.ai.mcpc.tools.ast import delete, edit, validate, read, create, insert, script, replace, outline, find, list

    __all__ = ["register_ast_tools", "ALIAS"]
    #: Alias name that activates the whole family in one go.
    ALIAS = "ast"
    _ALIAS_MEMBERS = (
        "ast_outline",
        "ast_list",
        "ast_find",
        "ast_read",
        "ast_insert",
        "ast_edit",
        "ast_replace",
        "ast_delete",
        "ast_create",
        "ast_imports",
        "ast_classes",
        "ast_functions",
        "ast_replace_block",
        "ast_validate",
    )


    def register_ast_tools(registry: ToolRegistry, functions: FunctionRegistry) -> None:
        """Register every ``ast_*`` tool and the ``ast`` alias."""

        outline.register(registry, functions)
        list.register(registry, functions)
        find.register(registry, functions)
        read.register(registry, functions)
        insert.register(registry, functions)
        edit.register(registry, functions)
        replace.register(registry, functions)
        delete.register(registry, functions)
        create.register(registry, functions)
        script.register(registry, functions)
        validate.register(registry, functions)

        registry.register_alias(ALIAS, _ALIAS_MEMBERS)
```
/answer 0e22-1a allow
Control Request:
```yaml
id: 0e22-1b
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit.py
  reason: Prüfen ob edit.py auf create_file/delete_file verweist
```
/answer 0e22-1b deny tut es nicht
Control Request:
```yaml
id: 0e22-1c
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
  reason: Prüfen ob replace.py auf create_file/delete_file verweist
```
/answer 0e22-1c deny tut es nicht
Control Request:
```yaml
id: 0e22-1d
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  reason: Prüfen ob script.py auf create_file/delete_file verweist
```
/answer 0e22-1d deny tut es nicht
ReasoningToken: 54
Text:
 Jetzt die alten `create_file.py`/`delete_file.py` entfernen und `__init__.py` aktualisieren.
ReasoningToken: 133
ReasoningToken: 75
Result Stats:  id=c59e3957-2ae9-4ef5-99b6-9bf28c45839a, total: 0, in: 167, out: 13214, reason: 0, read: 471157, write: 42251
Die Zusammenlegung ist abgeschlossen:

- **`create.py`**: `ast_create` erstellt jetzt eine Datei direkt aus `source`, legt fehlende Zielverzeichnisse rekursiv an (`mkdir(parents=True)`) und validiert per Parsen. Die alte Append-Semantik (Knoten an bestehende Datei anhängen) entfällt, da das bereits `ast_insert` abdeckt. `create_file.py` wurde entfernt.
- **`delete.py`**: `ast_delete` löscht bei angegebenem Selektor weiterhin einzelne Knoten. Wird kein Selektor übergeben (Root ausgewählt, da die Wurzel nie selbst adressierbarer Knoten ist), wird die Datei gelöscht, aus dem Cache entfernt und ihr Elternverzeichnis entfernt, falls es dadurch leer wird. `delete_file.py` wurde entfernt.
- **`__init__.py`**: Imports/Registrierung sowie `_ALIAS_MEMBERS` um `ast_create_file`/`ast_delete_file` bereinigt.
