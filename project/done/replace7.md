Das Tool `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py` soll ein zusätzliches Flag "replaceAll" bekommen um multiple vorkommen zu ersetzen.

Erstelle außerdem ein neues Tool "edit_line" das intern auf "edit_block" delegiert aber nur eine einzelne Zeile als Input erlaubt. Eine Zeile soll gegen eine andere Ersetzt werden können oder gegen eine Menge von Zeilen. "replaceAll" wird weitergereicht.
SystemInit:  id=f2e328d1-3fc6-4505-9607-5d670e35c4b0, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: c28d-1
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
  reason: "Verstehen der aktuellen Implementierung von edit_block, um replaceAll Flag hinzuzufügen und edit_line Tool zu erstellen"
```
/answer c28d-1 allow
ReasoningToken: 16
Control Request:
```yaml
id: c28d-2
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_text_match.py
  reason: "Verstehen der find-Funktion, um replaceAll korrekt zu implementieren"
```
/answer c28d-2 allow
ReasoningToken: 54
Control Request:
```yaml
id: c28d-3
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_text_match.py
  start: "def find(haystack: str, needle: str, *, exact: bool) -> MatchResult:"
  end: |2-
        match = matches[0]
        return MatchResult(count=1, start=match.start(), end=match.end())
  content: |-
    def find_all(haystack: str, needle: str, *, exact: bool) -> list[MatchResult]:
        """Return all non-overlapping occurrences of ``needle`` in ``haystack``."""
        if exact:
            results: list[MatchResult] = []
            start = 0
            while True:
                idx = haystack.find(needle, start)
                if idx == -1:
                    break
                results.append(MatchResult(count=1, start=idx, end=idx + len(needle)))
                start = idx + len(needle)
            return results

        pattern = _fuzzy_pattern(needle)
        return [MatchResult(count=1, start=m.start(), end=m.end()) for m in pattern.finditer(haystack)]


    def find(haystack: str, needle: str, *, exact: bool) -> MatchResult:
        if exact:
            count = haystack.count(needle)
            if count != 1:
                return MatchResult(count=count)
            start = haystack.index(needle)
            return MatchResult(count=1, start=start, end=start + len(needle))

        pattern = _fuzzy_pattern(needle)
        matches = list(pattern.finditer(haystack))
        if len(matches) != 1:
            return MatchResult(count=len(matches))
        match = matches[0]
        return MatchResult(count=1, start=match.start(), end=match.end())
  reason: "find_all Funktion hinzufügen, um mehrere Vorkommen für das replaceAll-Flag zu unterstützen"
```
/answer c28d-3 allow
Control Request:
```yaml
id: c28d-4
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
  start: from xy.ai.mcpc.tools._text_match import find as find_text
  end: from xy.ai.mcpc.tools.function_registry import FunctionRegistry
  content: |-
    from xy.ai.mcpc.tools._text_match import find as find_text, find_all as find_all_text
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry
  reason: find_all importieren für replaceAll-Unterstützung
```
/answer c28d-4 allow
ReasoningToken: 359
Text:
 Jetzt `edit_block` mit `replaceAll` erweitern:
Control Request:
```yaml
id: c28d-5
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
  start: "def edit_block(path: str, old_text: str, new_text: str, exact: bool=False) -> EditBlockResult:"
  end: "    result_text = text[:match.start] + new_text + text[match.end:]"
  content: |-
    def edit_block(path: str, old_text: str, new_text: str, exact: bool=False, replace_all: bool=False) -> EditBlockResult:
        \"\"\"Replace occurrence(s) of ``old_text`` in the file at ``path`` with ``new_text``.

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
        \"\"\"
        file_path = Path(path)
        if not file_path.is_absolute():
            raise EditBlockError('Path must be absolute.')
        if not file_path.exists():
            raise EditBlockError('File not found.')
        if not file_path.is_file():
            raise EditBlockError('Not a regular file.')
        text = file_path.read_text(encoding='utf-8')
        if replace_all:
            matches = find_all_text(text, old_text, exact=exact)
            if not matches:
                raise EditBlockError('Text not found in file.')
            result_text = text
            for match in sorted(matches, key=lambda m: m.start, reverse=True):
                result_text = result_text[:match.start] + new_text + result_text[match.end:]
        else:
            match = find_text(text, old_text, exact=exact)
            if match.count == 0:
                raise EditBlockError('Text not found in file.')
            if match.count > 1:
                raise EditBlockError(f'Text is ambiguous – found {match.count} occurrences in file.')
            result_text = text[:match.start] + new_text + text[match.end:]
  reason: "edit_block um replace_all Flag erweitern, das alle Vorkommen ersetzt statt nur ein eindeutiges"
```
/answer c28d-5 allow
Control Request:
```yaml
id: c28d-6
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
  start: '    description = "Replace a complete block of text'
  end: |2-
        def handle(self, ctx: ToolContext) -> ToolResult:
            \"\"\"Delegate to :func:`Edit_block`, translating the MCP schema to/from the Python API.\"\"\"
            args: dict[str, Any] = ctx.arguments
            try:
                result = edit_block(path=args['path'], old_text=args['old_text'], new_text=args['new_text'], exact=args.get('exact', False))
            except EditBlockError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content={'result': result.result}, auto_approve=True)
  content: |2-
        description = "Replace a complete block of text inside an existing file. 'old_text' must occur exactly once, unless 'replaceAll' is set. By default whitespace (spaces, tabs, newlines) is matched tolerantly; set 'exact' to require exact whitespace matching."
        input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the target file.'}, 'old_text': {'type': 'string', 'description': 'Text to find and replace. Must occur exactly once, unless replaceAll is set.'}, 'new_text': {'type': 'string', 'description': "Text that replace 'old_text (may be empty to perform a pure deletion)'."}, 'exact': {'type': 'boolean', 'description': "If true, 'old_text' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.", 'default': False}, 'replaceAll': {'type': 'boolean', 'description': "If true, replace every occurrence of 'old_text' instead of requiring a single unique match. Defaults to false.", 'default': False}}, 'required': ['path', 'old_text', 'new_text']}
        output_schema = {'type': 'object', 'properties': {'result': {'type': 'string'}}, 'required': []}
        annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`edit_block`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = edit_block(path=args['path'], old_text=args['old_text'], new_text=args['new_text'], exact=args.get('exact', False), replace_all=args.get('replaceAll', False))
            except EditBlockError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content={'result': result.result}, auto_approve=True)
  reason: "Schema und handle() um replaceAll erweitern (nur der description-teil und handle-Block, input_schema/output_schema/annotations dazwischen bleiben ansonsten gleich, werden hier neu geschrieben um korrekt zu matchen)"
```
/answer c28d-6 allow
Control Request:
```yaml
id: c28d-7
result:
  content:
  - type: text
    text: End marker not found in file.
  isError: true
```
/answer c28d-7 allow
Control Request:
```yaml
id: c28d-8
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
  reason: "Aktuellen Stand der Datei prüfen, um korrekte Marker für die nächste Bearbeitung zu bestimmen"
```
/answer c28d-8 allow
ReasoningToken: 53
Text:
 Jetzt Tool-Klasse (Schema, handle) anpassen:
Control Request:
```yaml
id: c28d-9
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
  start: "    description = \"Replace a complete block of text inside an existing file. 'old_text' must occur exactly once. By default whitespace (spaces, tabs, newlines) is matched tolerantly; set 'exact' to require exact whitespace matching.\""
  end: "'exact': {'type': 'boolean', 'description': \"If true, 'old_text' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.\", 'default': False}}, 'required': ['path', 'old_text', 'new_text']}"
  content: |2-
        description = "Replace a complete block of text inside an existing file. 'old_text' must occur exactly once, unless 'replaceAll' is set. By default whitespace (spaces, tabs, newlines) is matched tolerantly; set 'exact' to require exact whitespace matching."
        input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the target file.'}, 'old_text': {'type': 'string', 'description': 'Text to find and replace. Must occur exactly once, unless replaceAll is set.'}, 'new_text': {'type': 'string', 'description': "Text that replace 'old_text (may be empty to perform a pure deletion)'."}, 'exact': {'type': 'boolean', 'description': "If true, 'old_text' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.", 'default': False}, 'replaceAll': {'type': 'boolean', 'description': "If true, replace every occurrence of 'old_text' instead of requiring a single unique match. Defaults to false.", 'default': False}}, 'required': ['path', 'old_text', 'new_text']}
  reason: description und input_schema um replaceAll erweitern
```
/answer c28d-9 allow
Control Request:
```yaml
id: c28d-a
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
  start: "    def handle(self, ctx: ToolContext) -> ToolResult:"
  end: "            result = edit_block(path=args['path'], old_text=args['old_text'], new_text=args['new_text'], exact=args.get('exact', False))"
  content: |2-
        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`edit_block`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = edit_block(path=args['path'], old_text=args['old_text'], new_text=args['new_text'], exact=args.get('exact', False), replace_all=args.get('replaceAll', False))
  reason: handle() um replaceAll-Weiterreichung erweitern
```
/answer c28d-a allow
Control Request:
```yaml
id: c28d-b
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_block/__init__.py
  reason: Endkontrolle der edit_block-Datei nach den Änderungen
```
/answer c28d-b allow
Control Request:
```yaml
id: c28d-c
toolName: ask_user
arguments:
  question: "Ich möchte kurz die Registrierungsstelle finden. Gibt es eine zentrale Datei (z.B. `tools/__init__.py` oder Server-Setup), in der `register_edit_block_tool` aufgerufen wird, die ich anpassen soll, um auch `register_edit_line_tool` zu registrieren?"
  reason: "Um edit_line korrekt in den Server einzubinden, muss ich wissen, wo die Registrierungsfunktionen für Tools zentral aufgerufen werden."
```
/answer c28d-c deny nicht notwendig, ich registriere es
ReasoningToken: 89
ReasoningToken: 334
Control Request:
```yaml
id: c28d-d
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/edit_line/__init__.py
  mode: replace
  content: |
    """Edit-line tool – Replaces a single line with one or more lines in a file.

    Thin wrapper around :func:`edit_block` that restricts ``old_text`` to a single line.
    """
    from dataclasses import dataclass
    from pathlib import Path
    from typing import Any
    from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
    from xy.ai.mcpc.tools.tool_context import ToolContext
    from xy.ai.mcpc.tools.edit_block import EditBlockError, edit_block
    from xy.ai.mcpc.tools.function_registry import FunctionRegistry
    __all__ = ['EditLineError', 'EditLineResult', 'edit_line', 'EditLineTool', 'register_edit_line_tool']

    class EditLineError(Exception):
        """Raised when an edit-line operation cannot be performed."""

    @dataclass(frozen=True)
    class EditLineResult:
        result: str

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

    class EditLineTool(ToolDefinition):
        name = 'edit_line'
        title = 'Edit single line in file'
        description = "Replace a single line inside an existing file with one line or multiple lines. 'old_line' must be a single line (no newline character) and must occur exactly once, unless 'replaceAll' is set. By default whitespace is matched tolerantly; set 'exact' to require exact whitespace matching."
        input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the target file.'}, 'old_line': {'type': 'string', 'description': 'The single line to find and replace. Must not contain a newline character. Must occur exactly once, unless replaceAll is set.'}, 'new_lines': {'type': 'string', 'description': "Replacement content for 'old_line'. Either a single line, or multiple lines joined with '\\n' (may be empty to delete the line)."}, 'exact': {'type': 'boolean', 'description': "If true, 'old_line' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.", 'default': False}, 'replaceAll': {'type': 'boolean', 'description': "If true, replace every occurrence of 'old_line' instead of requiring a single unique match. Defaults to false.", 'default': False}}, 'required': ['path', 'old_line', 'new_lines']}
        output_schema = {'type': 'object', 'properties': {'result': {'type': 'string'}}, 'required': []}
        annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`edit_line`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = edit_line(path=args['path'], old_line=args['old_line'], new_lines=args['new_lines'], exact=args.get('exact', False), replace_all=args.get('replaceAll', False))
            except EditLineError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content={'result': result.result}, auto_approve=True)

    def register_edit_line_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
        registry.register(EditLineTool())
        functions.register(edit_line)
  reason: "Neues edit_line-Tool erstellen, das intern auf edit_block delegiert und nur eine einzelne Zeile als old_line zulässt, replaceAll wird weitergereicht."
```
/answer c28d-d allow
Text:
 Jetzt Datei nochmal prüfen, um sicherzustellen, dass alles konsistent ist, bevor ich `edit_line` erstelle:
Text:
 Sieht gut aus. Jetzt schaue ich mir an, wie das Registry/Modul-System registriert wird (z.B. wo `register_edit_block_tool` aufgerufen wird), um `edit_line` gleich einzubinden.
Text:
 Gut, dann konzentriere ich mich nur auf das Tool selbst. Ich schaue mir kurz ein anderes einfaches Tool an, das intern auf ein anderes delegiert, um den üblichen Stil zu treffen – bzw. ich schreibe direkt `edit_line` analog zu `edit_block`.
Beide Änderungen sind fertig:

1. **`edit_block`**: neues `replaceAll`-Flag (intern `replace_all`), das statt eines eindeutigen Treffers alle Vorkommen von `old_text` ersetzt (mittels neuer Funktion `find_all` in `_text_match.py`, Ersetzung erfolgt rückwärts nach Position, um Verschiebungen zu vermeiden).
2. **Neues Tool `edit_line`** (`tools/edit_line/__init__.py`): nimmt `old_line` (muss einzeilig sein, sonst `EditLineError`) und `new_lines` (ein- oder mehrzeilig via `\n`) entgegen, delegiert an `edit_block` und reicht `exact`/`replaceAll` durch.

Die Registrierung übernimmst du selbst, wie gewünscht.
