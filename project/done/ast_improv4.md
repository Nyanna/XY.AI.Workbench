Implementiere Analog zu `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud_replace.py` und `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud_insert.py` ein Tool "python_ast_edit".
Dieses soll auf Basis von `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py` ein Editieren innerhalb eines Ziel AST Knotens erlauben. "edit_marks" muss dafür den inneren Funktionsblock exponieren, erhält Code statt einer Datei und liefert den veränderten Code zurück damit er per "python_ast_replace" den Zielknoden mit den neuen Änderungen ersetzt.
Registrierung hier: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/__init__.py`
SystemInit:  id=50465f43-bc4e-4b26-9b64-59d954bee586, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 1683-1
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud_replace.py
  reason: Analyze pattern for new tool implementation
```
/answer 1683-1 allow
Control Request:
```yaml
id: 1683-2
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud_insert.py
  reason: Analyze pattern for new tool implementation
```
/answer 1683-2 allow
Control Request:
```yaml
id: 1683-3
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py
  reason: Understand edit_marks implementation to expose inner function
```
/answer 1683-3 allow
Control Request:
```yaml
id: 1683-4
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/__init__.py
  reason: Understand tool registration pattern
```
/answer 1683-4 allow
ReasoningToken: 12
Control Request:
```yaml
id: 1683-5
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
  reason: Understand ast tools registration module
```
/answer 1683-5 allow
Control Request:
```yaml
id: 1683-6
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  reason: Understand core helpers for AST node source extraction
```
/answer 1683-6 allow
Control Request:
```yaml
id: 1683-7
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud_common.py
  reason: Understand selector helpers
```
/answer 1683-7 allow
Control Request:
```yaml
id: 1683-8
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_text_match.py
  reason: Understand find_text helper used by edit_marks to reuse in code-based edit
```
/answer 1683-8 allow
ReasoningToken: 112
Control Request:
```yaml
id: 1683-9
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py
  start: "def edit_marks(path: str, start: str, end: str, content: str, exact: bool=False) -> EditMarksResult:"
  end: '    return EditMarksResult(result=''success'')'
  content: |-
    def edit_marks_text(text: str, start: str, end: str, content: str, exact: bool = False) -> str:
        """Replace everything between and including 'start' and 'end' with content, in *text*.

        Both markers are included in the replacement.

        Args:
            text: Source text to edit.
            start: Unique substring marking the beginning of the block.
            end: Unique substring marking the end of the block.
            content: Replacement text.
            exact: If False (default), whitespace in start/end is matched tolerantly. If True, whitespace must match exactly.

        Returns:
            The edited text.

        Raises:
            EditMarksError: If start or end markers are not found or appear more than once.
            EditMarksError: If end marker does not start after start marker ends.
        """
        start_match = find_text(text, start, exact=exact)
        if start_match.count == 0:
            raise EditMarksError('Start marker not found in file.')
        if start_match.count > 1:
            raise EditMarksError(
                f'Start marker is ambiguous – found {start_match.count} occurrences in file.'
            )

        end_match = find_text(text, end, exact=exact)
        if end_match.count == 0:
            raise EditMarksError('End marker not found in file.')
        if end_match.count > 1:
            raise EditMarksError(
                f'End marker is ambiguous – found {end_match.count} occurrences in file.'
            )

        if end_match.start < start_match.end:
            raise EditMarksError('End marker must start after start marker ends.')

        return text[:start_match.start] + content + text[end_match.end:]


    def edit_marks(path: str, start: str, end: str, content: str, exact: bool=False) -> EditMarksResult:
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
        result_text = edit_marks_text(text, start, end, content, exact=exact)

        try:
            file_path.write_text(result_text, encoding='utf-8')
        except OSError as exc:
            raise EditMarksError(f'Write failed: {exc}') from exc

        return EditMarksResult(result='success')
  reason: Expose reusable text-based edit_marks function for python_ast_edit tool
```
/answer 1683-9 allow
Control Request:
```yaml
id: 1683-a
toolName: edit_line
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_marks/__init__.py
  old_line: "__all__ = ['EditMarksError', 'EditMarksResult', 'edit_marks', 'EditMarksTool', 'register_edit_mark_tool']"
  new_lines: "__all__ = ['EditMarksError', 'EditMarksResult', 'edit_marks', 'edit_marks_text', 'EditMarksTool', 'register_edit_mark_tool']"
  reason: Export new text-based function
```
/answer 1683-a allow
ReasoningToken: 769
Text:
 Now create the new `python_ast_edit` tool file.
Control Request:
```yaml
id: 1683-b
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud_edit.py
  mode: replace
  content: |
    """``python_ast_edit`` tool: mark-based edit within the source of a selected node."""


    from dataclasses import dataclass
    from typing import Any

    from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import ToolContext
    from xy.ai.mcpc.tools.ast import core
    from xy.ai.mcpc.tools.ast.crud_common import SELECTOR_PROPS, select_one
    from xy.ai.mcpc.tools.edit_marks import EditMarksError, edit_marks_text
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry

    __all__ = ["EditNodeResult", "python_ast_edit", "EditNodeTool", "register"]


    @dataclass(frozen=True)
    class EditNodeResult:
        """Result of :func:`python_ast_edit`.

        Attributes:
            result: Always ``"success"``.
        """

        result: str


    def python_ast_edit(
        path: str,
        start: str,
        end: str,
        content: str,
        *,
        exact: bool = False,
        qualified_name: str | None = None,
        name: str | None = None,
        node_type: str | None = None,
        lineno: int | None = None,
        end_lineno: int | None = None,
        parent_type: str | None = None,
    ) -> EditNodeResult:
        """Replace everything between the 'start' and 'end' markers inside a selected node's source.

        The selected node's source is unparsed, edited between the two markers (both
        included) as with ``edit_marks``, re-parsed, and used to replace the node.

        Args:
            path: Absolute path to the Python file to modify.
            start: Unique substring marking the beginning of the block, within the selected node's source.
            end: Unique substring marking the end of the block, within the selected node's source.
            content: Replacement Python source for the marked block.
            exact: If False (default), whitespace in start/end is matched tolerantly. If True, whitespace must match exactly.
            qualified_name: Selector – exact Python-style FQN of the target node.
            name: Selector – exact simple name of the target node.
            node_type: Selector – AST node class name of the target node.
            lineno: Selector – exact start line of the target node.
            end_lineno: Selector – exact end line of the target node.
            parent_type: Selector – AST class name of the target node's container.

        Returns:
            EditNodeResult: Success status.

        Raises:
            core.AstError: If ``path`` is invalid, the selector matches zero or more
                than one node, the markers are not found or ambiguous within the
                node's source, or the edited source has a syntax error.
        """
        file_path = core.require_path(path)
        tree = core.CACHE.get_tree(file_path)
        target = select_one(
            tree,
            qualified_name=qualified_name,
            name=name,
            node_type=node_type,
            lineno=lineno,
            end_lineno=end_lineno,
            parent_type=parent_type,
        )
        node_source = core.unparse(target.node)
        try:
            new_source = edit_marks_text(node_source, start, end, content, exact=exact)
        except EditMarksError as exc:
            raise core.AstError(str(exc)) from exc
        new_nodes = core.parse_snippet(new_source)
        core.replace_in_body(target, new_nodes)
        core.CACHE.save(file_path, tree)
        return EditNodeResult(result="success")


    class EditNodeTool(ToolDefinition):
        name = "python_ast_edit"
        title = "Edit AST node"
        description = (
            "Replace everything strictly between and including the unique 'start' and 'end' "
            "markers, found within the source of the selected node, with 'content'."
        )
        input_schema = {
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path to the Python file."},
                "start": {
                    "type": "string",
                    "description": "Unique substring marking the beginning of the block, within the selected node's source.",
                },
                "end": {
                    "type": "string",
                    "description": "Unique substring marking the end of the block, within the selected node's source.",
                },
                "content": {"type": "string", "description": "Replacement Python source for the marked block."},
                "exact": {
                    "type": "boolean",
                    "description": "If true, 'start'/'end' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                    "default": False,
                },
                **SELECTOR_PROPS,
            },
            "required": ["path", "start", "end", "content"],
        }
        output_schema = {
            "type": "object",
            "properties": {"result": {"type": "string"}},
            "required": ["result"],
        }
        annotations = {"readOnlyHint": False, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`python_ast_edit`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = python_ast_edit(
                    args["path"],
                    args["start"],
                    args["end"],
                    args["content"],
                    exact=args.get("exact", False),
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
        registry.register(EditNodeTool())
        functions.register(python_ast_edit)
  reason: Create python_ast_edit tool implementing mark-based edit within a selected AST node's source
```
/answer 1683-b allow
Control Request:
```yaml
id: 1683-c
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
  start: |-
    from xy.ai.mcpc.tools.ast import (
        crud_create,
  end: "    crud_insert,"
  content: |-
    from xy.ai.mcpc.tools.ast import (
        crud_create,
        crud_delete,
        crud_edit,
        crud_find,
        crud_insert,
  reason: Import new crud_edit module
```
/answer 1683-c allow
Control Request:
```yaml
id: 1683-d
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
  reason: Verify import block correctness after edit
```
/answer 1683-d deny ist korrekt
Control Request:
```yaml
id: 1683-e
toolName: edit_line
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
  old_line: "    \"python_ast_insert\","
  new_lines: |2-
        "python_ast_insert",
        "python_ast_edit",
  reason: Add python_ast_edit to the alias members list
```
/answer 1683-e allow
Control Request:
```yaml
id: 1683-f
toolName: edit_line
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
  old_line: "    crud_insert.register(registry, functions)"
  new_lines: |2-
        crud_insert.register(registry, functions)
        crud_edit.register(registry, functions)
  reason: Register python_ast_edit tool
```
/answer 1683-f allow
Text:
 Now register the new tool in `ast/__init__.py`.
ReasoningToken: 226
ReasoningToken: 55
Fertig. Zusammenfassung der Änderungen:

- `edit_marks/__init__.py`: Kernlogik in `edit_marks_text(text, start, end, content, exact)` extrahiert (arbeitet auf Text statt Datei), `edit_marks` nutzt diese Funktion nun intern; Export erweitert.
- `ast/crud_edit.py` (neu): Tool `python_ast_edit` – selektiert einen Knoten (wie `crud_replace`/`crud_insert`), unparst dessen Quelltext, wendet `edit_marks_text` darauf an, parst das Ergebnis neu und ersetzt den Knoten via `core.replace_in_body`.
- `ast/__init__.py`: `crud_edit` importiert, registriert und zum `python-ast`-Alias hinzugefügt.
Ein Missverständnis python_ast_edit darf nicht nur einen Pfad bekommen sondern zwingend auch einen Knoten Selektor. Das Ziel ist nicht die Datei sondern der Inhalt eines Knotens. 
SystemInit:  id=50465f43-bc4e-4b26-9b64-59d954bee586, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 95ec-1
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud_edit.py
  start: |2-
        Args:
            path: Absolute path to the Python file to modify.
            start: Unique substring marking the beginning of the block, within the selected node's source.
            end: Unique substring marking the end of the block, within the selected node's source.
            content: Replacement Python source for the marked block.
            exact: If False (default), whitespace in start/end is matched tolerantly. If True, whitespace must match exactly.
            qualified_name: Selector – exact Python-style FQN of the target node.
            name: Selector – exact simple name of the target node.
            node_type: Selector – AST node class name of the target node.
            lineno: Selector – exact start line of the target node.
            end_lineno: Selector – exact end line of the target node.
            parent_type: Selector – AST class name of the target node's container.

        Returns:
            EditNodeResult: Success status.

        Raises:
            core.AstError: If ``path`` is invalid, the selector matches zero or more
                than one node, the markers are not found or ambiguous within the
                node's source, or the edited source has a syntax error.
        """
        file_path = core.require_path(path)
        tree = core.CACHE.get_tree(file_path)
        target = select_one(
  end: '    node_source = core.unparse(target.node)'
  content: |2-
        Args:
            path: Absolute path to the Python file to modify.
            start: Unique substring marking the beginning of the block, within the selected node's source.
            end: Unique substring marking the end of the block, within the selected node's source.
            content: Replacement Python source for the marked block.
            exact: If False (default), whitespace in start/end is matched tolerantly. If True, whitespace must match exactly.
            qualified_name: Selector – exact Python-style FQN of the target node.
            name: Selector – exact simple name of the target node.
            node_type: Selector – AST node class name of the target node.
            lineno: Selector – exact start line of the target node.
            end_lineno: Selector – exact end line of the target node.
            parent_type: Selector – AST class name of the target node's container.

        Returns:
            EditNodeResult: Success status.

        Raises:
            core.AstError: If ``path`` is invalid, no selector is given, the selector
                matches zero or more than one node, the markers are not found or
                ambiguous within the node's source, or the edited source has a
                syntax error.
        """
        if not any((qualified_name, name, node_type, lineno, end_lineno, parent_type)):
            raise core.AstError("A node selector is required; python_ast_edit targets a node's content, not the whole file.")
        file_path = core.require_path(path)
        tree = core.CACHE.get_tree(file_path)
        target = select_one(
  reason: "Enforce mandatory node selector since the tool targets a node's content, not the file"
```
/answer 95ec-1 allow
Control Request:
```yaml
id: 95ec-2
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud_edit.py
  start: '        node_source = core.unparse(target.node)'
  reason: Check remaining part of function after edit to ensure correctness
```
/answer 95ec-2 allow
Control Request:
```yaml
id: 95ec-3
result:
  content:
  - type: text
    text: Start marker not found in file.
  isError: true
```
/answer 95ec-3 allow
Control Request:
```yaml
id: 95ec-4
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud_edit.py
  reason: Verify full file after edit_marks operation to ensure node_source line still present and correct
```
/answer 95ec-4 allow
Control Request:
```yaml
id: 95ec-5
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud_edit.py
  start: |2-
        target = select_one(
        try:
  end: "    try:"
  content: |2-
        target = select_one(
            tree,
            qualified_name=qualified_name,
            name=name,
            node_type=node_type,
            lineno=lineno,
            end_lineno=end_lineno,
            parent_type=parent_type,
        )
        node_source = core.unparse(target.node)
        try:
  reason: Restore selector call arguments and node_source line accidentally removed by prior edit
```
/answer 95ec-5 allow
Control Request:
```yaml
id: 95ec-6
result:
  content:
  - type: text
    text: End marker is ambiguous – found 2 occurrences in file.
  isError: true
```
/answer 95ec-6 allow
Control Request:
```yaml
id: 95ec-7
toolName: edit_line
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud_edit.py
  old_line: |2-
        target = select_one(
        try:
            new_source = edit_marks_text(node_source, start, end, content, exact=exact)
  new_lines: |2-
        target = select_one(
            tree,
            qualified_name=qualified_name,
            name=name,
            node_type=node_type,
            lineno=lineno,
            end_lineno=end_lineno,
            parent_type=parent_type,
        )
        node_source = core.unparse(target.node)
        try:
            new_source = edit_marks_text(node_source, start, end, content, exact=exact)
  reason: Restore selector call arguments and node_source line accidentally removed by prior edit
```
/answer 95ec-7 allow
Control Request:
```yaml
id: 95ec-8
result:
  content:
  - type: text
    text: old_line must be a single line without newline characters.
  isError: true
```
/answer 95ec-8 allow
Control Request:
```yaml
id: 95ec-9
toolName: edit_line
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud_edit.py
  old_line: '    target = select_one('
  new_lines: |2-
        target = select_one(
            tree,
            qualified_name=qualified_name,
            name=name,
            node_type=node_type,
            lineno=lineno,
            end_lineno=end_lineno,
            parent_type=parent_type,
        )
        node_source = core.unparse(target.node)
  reason: Restore selector call arguments and node_source line accidentally removed by prior edit
```
/answer 95ec-9 allow
Control Request:
```yaml
id: 95ec-a
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud_edit.py
  reason: Verify file is now syntactically correct and complete
```
/answer 95ec-a allow
ReasoningToken: 414
ReasoningToken: 49
Text:
 I need to restore the missing lines that got removed by my end marker being too broad.
Datei ist wieder korrekt und vollständig. `python_ast_edit` verlangt nun zwingend mindestens einen Knoten-Selektor und wirft andernfalls einen `core.AstError`.
