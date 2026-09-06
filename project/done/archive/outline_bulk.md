Füge `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py` Bulk Fähigkeiten hinzu. Also Input soll eine Liste sein entsprechend kann er Output auch eine Liste sein. 
SystemInit:  id=f6900b41-c8ac-4b34-bbc7-35b7eab794c8, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 192b-1
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  reason: Aktuellen Code lesen um Bulk-Fähigkeiten hinzuzufügen
```
/answer 192b-1 allow
ReasoningToken: 10
Control Request:
```yaml
id: 192b-2
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
  pattern: bulk|paths
  reason: Suchen ob es bereits ähnliche Bulk-Implementierungen als Vorlage gibt
```
/answer 192b-2 deny Beispielsweise: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/validate.py`
Control Request:
```yaml
id: 192b-3
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  reason: core.load und AstError verstehen für Fehlerbehandlung bei Bulk
```
/answer 192b-3 allow
ReasoningToken: 46
Control Request:
```yaml
id: 192b-4
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
  reason: Alle Tool-Dateien auflisten um Konventionen für Bulk-Input/Output zu finden
```
/answer 192b-4 deny Beispielsweise: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/validate.py`
ReasoningToken: 174
Control Request:
```yaml
id: 192b-5
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/file-stats.py
  reason: Prüfen ob file-stats bereits Bulk unterstützt als Vorlage für Konvention
```
/answer 192b-5 deny Beispielsweise: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/validate.py`
ReasoningToken: 68
Control Request:
```yaml
id: 192b-6
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
  reason: Dateien im ast Tools-Verzeichnis auflisten für Konventionsvergleich
```
/answer 192b-6 deny Beachte meine Anweisungen
ReasoningToken: 92
Control Request:
```yaml
id: 192b-7
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/validate.py
  reason: Konventionen für Tool-Schema und Fehlerbehandlung in ähnlichen AST-Tools prüfen
```
/answer 192b-7 allow
Control Request:
```yaml
id: 192b-8
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  reason: ToolContext/ToolResult API verstehen für Implementierung von Bulk-Fähigkeiten
```
/answer 192b-8 deny das ist nicht notwendig
ReasoningToken: 82
Text:
 Gut, das gibt mir das Konventionsmuster (`paths` als Liste, Ergebnis-Objekt mit `files`-Liste, jeweils mit `path`/Fehlerfeld). Ich passe `outline.py` entsprechend an: `path` (string) wird durch `paths` (Liste) ersetzt, Output enthält eine `files`-Liste mit je einem Outline-Eintrag oder Fehler pro Datei.
Control Request:
```yaml
id: 192b-9
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  mode: replace
  reason: "Bulk-Fähigkeit hinzufügen: paths-Liste statt einzelnem path, files-Liste im Output analog zu validate.py"
  content: |
    """``python-ast-outline`` – compact structural overview of Python files."""

    from __future__ import annotations

    import ast
    import importlib
    from typing import Any

    from ...registry import ToolContext, ToolRegistry, ToolResult, text_content
    from . import core

    # ``file-stats`` uses a hyphenated (non-identifier) package name.
    compute_file_stats = importlib.import_module(
        "xy.ai.mcpc.tools.file-stats"
    ).compute_file_stats


    def _method_entry(loc: core.Located) -> dict[str, Any]:
        node = loc.node
        return {
            "name": loc.name,
            "qualified_name": loc.qualified_name,
            "lineno": node.lineno,
            "end_lineno": getattr(node, "end_lineno", node.lineno),
            "docstring": core.short_docstring(node),
        }


    def _build_outline(tree: ast.Module) -> dict[str, Any]:
        located = core.locate_all(tree)

        imports = [
            {"names": loc.name, "lineno": loc.node.lineno}
            for loc in located
            if isinstance(loc.node, core._IMPORT_TYPES)
        ]

        classes: list[dict[str, Any]] = []
        functions: list[dict[str, Any]] = []
        for loc in located:
            node = loc.node
            if isinstance(node, ast.ClassDef):
                methods = [
                    _method_entry(m)
                    for m in located
                    if m.parent is node and isinstance(m.node, (ast.FunctionDef, ast.AsyncFunctionDef))
                ]
                classes.append(
                    {
                        "name": loc.name,
                        "qualified_name": loc.qualified_name,
                        "lineno": node.lineno,
                        "end_lineno": getattr(node, "end_lineno", node.lineno),
                        "docstring": core.short_docstring(node),
                        "methods": methods,
                    }
                )
            elif isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)) and isinstance(
                loc.parent, ast.Module
            ):
                functions.append(_method_entry(loc))

        return {"imports": imports, "classes": classes, "functions": functions}


    def _outline_one(path_str: str) -> dict[str, Any]:
        try:
            path, tree = core.load(path_str)
        except core.AstError as exc:
            return {"path": path_str, "ok": False, "error": str(exc)}
        outline = {"stats": compute_file_stats(path), **_build_outline(tree)}
        return {"path": path_str, "ok": True, "error": None, **outline}


    _OUTLINE_ITEM_SCHEMA = {
        "type": "object",
        "properties": {
            "path": {"type": "string"},
            "ok": {"type": "boolean"},
            "error": {"type": ["string", "null"]},
            "stats": {"type": "object", "description": "File-metrics block."},
            "imports": {
                "type": "array",
                "description": "Imports with line numbers.",
                "items": {
                    "type": "object",
                    "properties": {
                        "names": {"type": "string"},
                        "lineno": {"type": "integer"},
                    },
                    "required": ["names", "lineno"],
                },
            },
            "classes": {
                "type": "array",
                "description": "Top-level classes with nested methods.",
                "items": {"type": "object"},
            },
            "functions": {
                "type": "array",
                "description": "Top-level functions.",
                "items": {"type": "object"},
            },
        },
        "required": ["path", "ok", "error"],
    }


    def register(registry: ToolRegistry) -> None:
        @registry.tool(
            "python-ast-outline",
            title="Python outline",
            description=(
                "Token-efficient structural overview of Python files: file metrics, "
                "imports, and a class/function hierarchy with line ranges and short "
                "docstrings. Accepts one or several files at once."
            ),
            input_schema={
                "type": "object",
                "properties": {
                    "paths": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Absolute paths of Python files to outline.",
                    }
                },
                "required": ["paths"],
            },
            output_schema={
                "type": "object",
                "properties": {
                    "all_ok": {"type": "boolean"},
                    "files": {"type": "array", "items": _OUTLINE_ITEM_SCHEMA},
                },
                "required": ["all_ok", "files"],
            },
            annotations={"readOnlyHint": True, "openWorldHint": False},
        )
        def outline(ctx: ToolContext) -> ToolResult:
            paths = ctx.arguments["paths"]
            if not isinstance(paths, list) or not paths:
                return ToolResult(content=[text_content("'paths' must be a non-empty list.")], is_error=True)
            files = [_outline_one(p) for p in paths]
            return ToolResult(
                structured_content={"all_ok": all(f["ok"] for f in files), "files": files},
                auto_approve=True,
            )
```
/answer 192b-9 allow
Control Request:
```yaml
id: 192b-a
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  reason: "Nach anderen Verwendungen von python-ast-outline suchen, die auf das alte 'path'-Schema angewiesen sein könnten"
  script: grep -rn "python-ast-outline\|_build_outline\|ast/outline" --include="*.py" -l .
```
/answer 192b-a allow
Control Request:
```yaml
id: 192b-b
result:
  structuredContent:
    stdout: |
      ./src/xy/ai/mcpc/tools/ast/outline.py
```
/answer 192b-b allow
Control Request:
```yaml
id: 192b-c
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  reason: "Syntax der geänderten Datei kompilieren, um Fehler auszuschließen"
  script: python3 -m py_compile src/xy/ai/mcpc/tools/ast/outline.py && echo OK
```
/answer 192b-c allow
Control Request:
```yaml
id: 192b-d
result:
  structuredContent:
    stdout: |
      OK
```
/answer 192b-d allow
ReasoningToken: 33
Text:
 Keine weiteren Referenzen. Ich prüfe noch, dass die Datei syntaktisch korrekt ist.
`outline.py` wurde auf Bulk umgestellt:

- Input: `paths` (Liste von Pfaden) statt einzelnem `path`, analog zur Konvention in `validate.py`.
- Output: `{"all_ok": bool, "files": [...]}`, wobei jedes Element `path`, `ok`, `error` sowie bei Erfolg `stats`/`imports`/`classes`/`functions` enthält — Fehler pro Datei brechen den Rest nicht ab.
- Syntax mit `py_compile` verifiziert; keine anderen Aufrufer im Repo, die angepasst werden müssten.
