"""``ast_validate`` – compile a list of files and report results."""
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['ValidateError', 'FileCheck', 'ValidateResult', 'ast_validate', 'ValidateTool', 'register']

class ValidateError(Exception):
    """Raised when the validate operation cannot be performed at all."""

@dataclass(frozen=True)
class FileCheck:
    """Compile-check result for a single file, as returned by :func:`ast_validate`.

    Attributes:
        path: The path exactly as given in the input.
        ok: Whether the file compiled successfully.
        error: Error message (with line number) if ``ok`` is ``False``, else ``None``.
    """
    path: str
    ok: bool
    error: str | None

@dataclass(frozen=True)
class ValidateResult:
    """Result of :func:`ast_validate`.

    Attributes:
        all_ok: Whether every file in ``files`` compiled successfully.
        files: One :class:`FileCheck` per input path, in the given order.
    """
    all_ok: bool
    files: list[FileCheck] = field(default_factory=list)

def _check(path_str: str) -> FileCheck:
    path = Path(path_str)
    if not path.is_absolute():
        return FileCheck(path=path_str, ok=False, error='Path must be absolute.')
    try:
        source = path.read_text(encoding='utf-8')
    except OSError:
        return FileCheck(path=path_str, ok=False, error='File not readable.')
    try:
        error = core.validate_source(path, source)
    except core.AstError as exc:
        return FileCheck(path=path_str, ok=False, error=str(exc))
    return FileCheck(path=path_str, ok=error is None, error=error)

def ast_validate(paths: list[str]) -> ValidateResult:
    """Compile each of ``paths`` and report success/error per file.

    Per-file failures (non-absolute path, unreadable file, syntax error) are
    reported inside the corresponding :class:`FileCheck` rather than raised; only
    a malformed call (empty ``paths``) raises.

    Args:
        paths: Absolute paths of files to validate. Must be non-empty.

    Returns:
        ValidateResult: One :class:`FileCheck` per path, in order, plus an overall
        ``all_ok`` flag.

    Raises:
        ValidateError: If ``paths`` is empty.
    """
    if not paths:
        raise ValidateError("'paths' must be a non-empty list.")
    files = [_check(p) for p in paths]
    return ValidateResult(all_ok=all((f.ok for f in files)), files=files)

class ValidateTool(ToolDefinition):
    name = 'ast_validate'
    title = 'Validate files'
    description = 'Check that each of a list of files compiles; report success/error per file.'
    input_schema = {
        'type': 'object',
        'properties': {
            'paths': {
                'type': 'array',
                'items': {
                    'type': 'string'},
                'description': 'Absolute paths of files to validate.'}},
        'required': ['paths']}
    output_schema = {
        'type': 'object', 'properties': {
            'all_ok': {
                'type': 'boolean'}, 'files': {
                    'type': 'array', 'items': {
                        'type': 'object', 'properties': {
                            'path': {
                                'type': 'string'}, 'ok': {
                                    'type': 'boolean'}, 'error': {
                                        'type': [
                                            'string', 'null']}}, 'required': [
                                                'path', 'ok', 'error']}}}, 'required': [
                                                    'all_ok', 'files']}
    annotations = {'readOnlyHint': True, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_validate`, translating the MCP schema to/from the AST API."""
        paths = ctx.arguments['paths']
        if not isinstance(paths, list):
            return ToolResult(content=[text_content("'paths' must be a non-empty list.")], is_error=True)
        try:
            result = ast_validate(paths)
        except ValidateError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(
            structured_content={
                'all_ok': result.all_ok,
                'files': [
                    f.__dict__ for f in result.files]},
            auto_approve=result.all_ok)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ValidateTool())
    functions.register(ast_validate)