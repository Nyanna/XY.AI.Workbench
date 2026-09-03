"""``ast_script`` – run restricted Python against the parsed AST.

For complex reorganisation/optimisation the model can operate on the tree
directly. The script runs with an empty ``__builtins__`` plus a small, curated
set of safe names; the only capabilities handed in are ``tree`` (a
:class:`ScriptTree` wrapping the parsed file) and the standard-library
``ast`` module. Any change made through ``tree`` is persisted.

``tree`` exposes the same locate/replace/insert/delete/append primitives the
other ``ast_*`` tools use, so scripts work the same way for Python and
tree-sitter files alike. ``tree.raw`` gives direct access to the engine-native
tree (``ast.Module`` for Python, ``tree_sitter.Tree`` otherwise); only the
Python ``ast.Module`` is safely mutable in place – tree-sitter's parse tree is
read-only and must be edited through ``tree``'s methods instead.
"""
import ast
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['ScriptTree', 'ScriptError', 'AstScriptResult', 'ast_script', 'ScriptTool', 'register']
_SAFE_BUILTINS = {
    name: getattr(
        __builtins__,
        name,
        None) if not isinstance(
            __builtins__,
            dict) else __builtins__.get(name) for name in (
                'isinstance',
                'issubclass',
                'getattr',
                'setattr',
                'hasattr',
                'delattr',
                'len',
                'list',
                'dict',
                'set',
                'tuple',
                'str',
                'int',
                'float',
                'bool',
                'enumerate',
                'range',
                'sorted',
                'reversed',
                'zip',
                'map',
                'filter',
                'any',
                'all',
                'min',
                'max',
                'sum',
                'type',
        'repr')}

class ScriptTree:
    """Engine-agnostic ``tree`` handle exposed to sandboxed scripts.

    Wraps a :class:`core.Tree`, exposing the locate/replace/insert/delete/append
    primitives the other ``ast_*`` tools use, so scripts behave identically
    regardless of which engine parsed the file. ``raw`` gives direct access to
    the engine-native tree; only the Python engine's ``ast.Module`` is safely
    mutable in place, the tree-sitter engine's parse tree is read-only and
    must be edited through the methods below.
    """

    def __init__(self, tree: core.Tree) -> None:
        self._tree = tree

    @property
    def raw(self) -> Any:
        return self._tree.raw

    @property
    def source(self) -> str:
        return self._tree.source

    @property
    def path(self) -> Any:
        return self._tree.path

    def find(self, *, id: str | None=None, node_type: str | None=None, name: str | None=None, parent_type: str | None=None) -> list[core.Located]:
        return core.find(self._tree, id=id, node_type=node_type, name=name, parent_type=parent_type)

    def locate_all(self) -> list[core.Located]:
        return core.locate_all(self._tree)

    def node_code(self, loc: core.Located) -> str:
        return core.edit_node_source(loc)

    def replace(self, loc: core.Located, code: str) -> str | None:
        return core.replace_node(loc, code)

    def insert(self, loc: core.Located, code: str, position: str='after') -> int:
        return core.insert_node(loc, code, position)

    def delete(self, loc: core.Located) -> None:
        core.delete_node(loc)

    def append(self, code: str) -> int:
        return core.append_nodes(self._tree, code)

class ScriptError(Exception):
    """Raised when an AST script cannot be run to completion."""

@dataclass(frozen=True)
class AstScriptResult:
    """Result of :func:`ast_script`.

    Attributes:
        result: Always ``"success"``.
        value: ``repr()`` of the script's ``result`` variable, if the script set one;
            otherwise ``None``.
    """
    result: str
    value: str | None = None

def ast_script(path: str, code: str) -> AstScriptResult:
    """Execute ``code`` in a restricted sandbox exposing the file's tree as ``tree``.

    ``code`` runs with an empty ``__builtins__`` plus a small, curated set of safe
    names (see ``_SAFE_BUILTINS``); the only capabilities handed in are ``tree``
    (a :class:`ScriptTree`) and the standard-library ``ast`` module itself. Any
    mutation made through ``tree`` is persisted to ``path`` on success, for any
    file type the AST tools support (Python or tree-sitter).

    Args:
        path: Absolute path to the file whose AST is exposed as ``tree``.
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
    env: dict[str, Any] = {'tree': ScriptTree(tree), 'ast': ast}
    sandbox_globals = {'__builtins__': _SAFE_BUILTINS}
    try:
        '# noqa: S102'
        exec(compile(code, '<ast-script>', 'exec'), sandbox_globals, env)
    except SyntaxError as exc:
        raise ScriptError(f'Script syntax error: {exc.msg}') from exc
    except Exception as exc:
        '# noqa: BLE001'
        raise ScriptError(f'Script failed: {type(exc).__name__}: {exc}') from exc
    core.CACHE.save(file_path, tree)
    if 'result' in env:
        return AstScriptResult(result='success', value=repr(env['result']))
    return AstScriptResult(result='success')

class ScriptTool(ToolDefinition):
    name = 'ast_script'
    title = 'Run AST script'
    description = "Run a restricted Python script code against a file's AST for complex/incremental transforms. Globals expose 'tree' (a ScriptTree with find/replace/insert/delete/append, plus 'tree.raw' for the engine-native ast.Module/tree_sitter.Tree) and 'ast'; assign 'result' to return data. Changes made through 'tree' are saved. Imports are not allowed."
    input_schema = {
        'type': 'object',
        'properties': {
            'path': {
                'type': 'string',
                'description': 'Absolute path to the file.'},
            'code': {
                'type': 'string',
                'description': "Python script operating on 'tree' (find/replace/insert/delete/append); Environment is restricted; Don't use imports;"}},
        'required': [
            'path',
            'code']}
    output_schema = {'type': 'object', 'properties': {'result': {'type': 'string'}, 'value': {
        'description': "Repr of the script's 'result' variable, if set."}}, 'required': ['result']}
    annotations = {'readOnlyHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_script`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = ast_script(args['path'], args['code'])
        except (core.AstError, ScriptError) as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        structured: dict[str, Any] = {'result': result.result}
        if result.value is not None:
            structured['value'] = result.value
        return ToolResult(structured_content=structured)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ScriptTool())
    functions.register(ast_script)