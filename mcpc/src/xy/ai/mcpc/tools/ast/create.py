"""``ast_create`` tool: create one or more files from source, creating missing directories."""
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
__all__ = [
    'CreateFileItem',
    'CreateFileResult',
    'CreateFileError',
    'CreateFilesResult',
    'ast_create',
    'CreateFileTool',
    'register']

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

@dataclass(frozen=True)
class CreateFileResult:
    """Result of creating a single file.

    Attributes:
        path: The path exactly as given in the input, for result association.
        result: Always ``"success"``.
    """
    path: str
    result: str

@dataclass(frozen=True)
class CreateFileError:
    """Error creating a single file.

    Attributes:
        path: The path exactly as given in the input, for result association.
        error: The error message.
    """
    path: str
    error: str

@dataclass(frozen=True)
class CreateFilesResult:
    """Result of :func:`ast_create`.

    Attributes:
        results: One :class:`CreateFileResult` per file created successfully.
        errors: One :class:`CreateFileError` per file that failed.
    """
    results: list[CreateFileResult]
    errors: list[CreateFileError]

def _create_one(item: CreateFileItem) -> CreateFileResult:
    file_path = core.require_path(item.path, must_exist=False)
    if file_path.exists() and (not item.overwrite):
        raise core.AstError('File already exists.')
    tree = core.parse_for(item.path, item.source)
    file_path.parent.mkdir(parents=True, exist_ok=True)
    core.CACHE.save(file_path, tree)
    return CreateFileResult(path=item.path, result='success')

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

class CreateFileTool(ToolDefinition):
    name = 'ast_create'
    title = 'Create files'
    description = 'Create one or more files from source.'
    input_schema = {
        'type': 'object', 'properties': {
            'items': {
                'type': 'array', 'minItems': 1, 'items': {
                    'type': 'object', 'properties': {
                        'path': {
                            'type': 'string', 'description': 'Absolute path of the file to create.'}, 'source': {
                                'type': 'string', 'description': 'Source for the new file.'}, 'overwrite': {
                                    'type': 'boolean', 'description': 'Allow replacing an existing file.', 'default': False}}, 'required': [
                                        'path', 'source']}, 'description': 'Files to create.'}}, 'required': ['items']}
    output_schema = {
        'type': 'object', 'properties': {
            'results': {
                'type': 'array', 'items': {
                    'type': 'object', 'properties': {
                        'path': {
                            'type': 'string'}, 'result': {
                                'type': 'string'}}, 'required': [
                                    'path', 'result']}}, 'errors': {
                                        'type': 'array', 'items': {
                                            'type': 'object', 'properties': {
                                                'path': {
                                                    'type': 'string'}, 'error': {
                                                        'type': 'string'}}, 'required': [
                                                            'path', 'error']}}}}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_create`, translating the MCP schema to/from the Python API."""

        def item_factory(it: dict[str, Any]) -> CreateFileItem:
            return CreateFileItem(path=it['path'], source=it['source'], overwrite=it.get('overwrite', False))
        return handle_batch_tool(ctx, item_factory, ast_create, core.AstError, auto_approve=True)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(CreateFileTool())
    functions.register(ast_create)