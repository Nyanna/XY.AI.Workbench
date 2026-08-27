"""``python-ast-script`` – run restricted Python against the parsed AST.

For complex reorganisation/optimisation the model can operate on the tree
directly. The script runs with an empty ``__builtins__`` plus a small, curated
set of safe names; the only capability handed in is the AST itself (``tree``)
and the standard-library ``ast`` module. Any change to ``tree`` is persisted.
"""

from __future__ import annotations

import ast
from dataclasses import dataclass
from typing import Any

from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core

__all__ = ["ScriptError", "AstScriptResult", "run_ast_script", "ScriptTool", "register"]

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


class ScriptError(Exception):
    """Raised when an AST script cannot be run to completion."""


@dataclass(frozen=True)
class AstScriptResult:
    """Result of :func:`run_ast_script`.

    Attributes:
        result: Always ``"success"``.
        value: ``repr()`` of the script's ``result`` variable, if the script set one;
            otherwise ``None``.
    """

    result: str
    value: str | None = None


def run_ast_script(path: str, code: str) -> AstScriptResult:
    """Execute ``code`` in a restricted sandbox exposing the AST of ``path`` as ``tree``.

    ``code`` runs with an empty ``__builtins__`` plus a small, curated set of safe
    names (see ``_SAFE_BUILTINS``); the only capabilities handed in are the parsed
    tree (``tree``, an ``ast.Module``) and the standard-library ``ast`` module
    itself. Any mutation of ``tree`` is unparsed and persisted to ``path`` on
    success.

    Args:
        path: Absolute path to the Python file whose AST is exposed as ``tree``.
        code: Python script to execute against ``tree``. May assign a module-level
            name ``result`` to return an arbitrary value (reported as its ``repr()``).

    Returns:
        AstScriptResult: Success status and, if the script set one, the ``repr()``
        of its ``result`` variable.

    Raises:
        core.AstError: If ``path`` is invalid.
        ScriptError: If ``code`` has a syntax error, or raises during execution.
    """
    file_path = core.require_path(path)
    tree = core.CACHE.get_tree(file_path)
    env: dict[str, Any] = {"tree": tree, "ast": ast}
    sandbox_globals = {"__builtins__": _SAFE_BUILTINS}
    try:
        exec(compile(code, "<ast-script>", "exec"), sandbox_globals, env)  # noqa: S102
    except SyntaxError as exc:
        raise ScriptError(f"Script syntax error: {exc.msg}") from exc
    except Exception as exc:  # noqa: BLE001
        raise ScriptError(f"Script failed: {type(exc).__name__}: {exc}") from exc
    core.CACHE.save(file_path, tree)

    if "result" in env:
        return AstScriptResult(result="success", value=repr(env["result"]))
    return AstScriptResult(result="success")


class ScriptTool(ToolDefinition):
    name = "python-ast-script"
    title = "Run AST script"
    description = (
        "Run restricted Python against a file's AST for complex/incremental "
        "transforms. Globals expose 'tree' (ast.Module) and 'ast'; assign "
        "'result' to return data. Changes to 'tree' are saved."
    )
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path to the Python file."},
            "code": {"type": "string", "description": "Python script operating on 'tree';"
                     "Environment is restricted; Don't use imports;"},
        },
        "required": ["path", "code"],
    }
    output_schema = {
        "type": "object",
        "properties": {
            "result": {"type": "string"},
            "value": {"description": "Repr of the script's 'result' variable, if set."},
        },
        "required": ["result"],
    }
    annotations = {"readOnlyHint": False, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`run_ast_script`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = run_ast_script(args["path"], args["code"])
        except (core.AstError, ScriptError) as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)

        structured: dict[str, Any] = {"result": result.result}
        if result.value is not None:
            structured["value"] = result.value
        return ToolResult(structured_content=structured)


def register(registry: ToolRegistry) -> None:
    registry.register(ScriptTool())
