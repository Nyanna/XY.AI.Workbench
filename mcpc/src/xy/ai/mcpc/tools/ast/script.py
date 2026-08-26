"""``python-ast-script`` – run restricted Python against the parsed AST.

For complex reorganisation/optimisation the model can operate on the tree
directly. The script runs with an empty ``__builtins__`` plus a small, curated
set of safe names; the only capability handed in is the AST itself (``tree``)
and the standard-library ``ast`` module. Any change to ``tree`` is persisted.
"""

from __future__ import annotations

import ast
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult, text_content
from . import core

#: Curated, side-effect-free builtins needed for realistic AST manipulation.
_SAFE_BUILTINS = {
    name: getattr(__builtins__, name, None) if not isinstance(__builtins__, dict)
    else __builtins__.get(name)
    for name in (
        "isinstance", "issubclass", "getattr", "setattr", "hasattr", "delattr",
        "len", "list", "dict", "set", "tuple", "str", "int", "float", "bool",
        "enumerate", "range", "sorted", "reversed", "zip", "map", "filter",
        "any", "all", "min", "max", "sum", "type", "repr",
    )
}


def register(registry: ToolRegistry) -> None:
    @registry.tool(
        "python-ast-script",
        title="Run AST script",
        description=(
            "Run restricted Python against a file's AST for complex/incremental "
            "transforms. Globals expose 'tree' (ast.Module) and 'ast'; assign "
            "'result' to return data. Changes to 'tree' are saved."
        ),
        input_schema={
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path to the Python file."},
                "code": {"type": "string", "description": "Python script operating on 'tree'."},
            },
            "required": ["path", "code"],
        },
        output_schema={
            "type": "object",
            "properties": {
                "result": {"type": "string"},
                "value": {"description": "Repr of the script's 'result' variable, if set."},
            },
            "required": ["result"],
        },
        annotations={"readOnlyHint": False, "openWorldHint": False},
    )
    def run_script(ctx: ToolContext) -> ToolResult:
        args: dict[str, Any] = ctx.arguments
        try:
            path = core.require_path(args["path"])
            tree = core.CACHE.get_tree(path)
            env: dict[str, Any] = {"tree": tree, "ast": ast}
            sandbox_globals = {"__builtins__": _SAFE_BUILTINS}
            exec(compile(args["code"], "<ast-script>", "exec"), sandbox_globals, env)  # noqa: S102
            core.CACHE.save(path, tree)
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        except SyntaxError as exc:
            return ToolResult(content=[text_content(f"Script syntax error: {exc.msg}")], is_error=True)
        except Exception as exc:  # noqa: BLE001 - surface script failures compactly
            return ToolResult(content=[text_content(f"Script failed: {type(exc).__name__}: {exc}")], is_error=True)

        structured: dict[str, Any] = {"result": "success"}
        if "result" in env:
            structured["value"] = repr(env["result"])
        return ToolResult(structured_content=structured)
