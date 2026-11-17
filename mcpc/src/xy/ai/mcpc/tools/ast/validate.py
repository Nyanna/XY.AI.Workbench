"""``ast_validate`` – compile a list of files and report results."""
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools._tool_helpers import require_items
from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult
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

@dataclass(frozen=True)
class ValidFile(ItemResult):
    """A file that compiled."""
    path: str

    def key(self) -> dict[str, Any]:
        return {'path': self.path}

@dataclass(frozen=True)
class InvalidFile(ItemError):
    """A file that did not compile; ``error`` carries the message with line number."""
    path: str

    def key(self) -> dict[str, Any]:
        return {'path': self.path}

class ValidateBatchResult(BatchResult[ValidFile, InvalidFile]):
    """Compile check per input path."""

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

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_validate`, translating the MCP schema to/from the AST API."""
        paths, error = require_items(ctx, key='paths')
        if error is not None:
            return error
        try:
            result = ast_validate(paths)
        except ValidateError as exc:
            return ToolResult.failure(str(exc))
        return ToolResult.ok(
            ValidateBatchResult(
                results=[
                    ValidFile(
                        path=f.path) for f in result.files if f.ok],
                errors=[
                    InvalidFile(
                        path=f.path,
                        error=f.error or 'invalid') for f in result.files if not f.ok]),
            auto_approve=True)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ValidateTool())
    functions.register(ast_validate)