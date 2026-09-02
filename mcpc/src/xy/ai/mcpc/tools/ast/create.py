"""``ast_create`` tool: create a file from source, creating missing directories."""
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['CreateFileResult', 'ast_create', 'CreateFileTool', 'register']

@dataclass(frozen=True)
class CreateFileResult:
    """Result of :func:`ast_create`.

    Attributes:
        result: Always ``"success"``.
    """
    result: str

def ast_create(path: str, source: str, overwrite: bool=False) -> CreateFileResult:
    """Create a new file at ``path`` from ``source`` (validated by parsing it).

    Creating a single node
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
    if file_path.exists() and (not overwrite):
        raise core.AstError('File already exists.')
    tree = core.parse_for(path, source)
    file_path.parent.mkdir(parents=True, exist_ok=True)
    core.CACHE.save(file_path, tree)
    return CreateFileResult(result='success')

class CreateFileTool(ToolDefinition):
    name = 'ast_create'
    title = 'Create a file'
    description = 'Create a file from source (validated by parsing it).'
    input_schema = {
        'type': 'object', 'properties': {
            'path': {
                'type': 'string', 'description': 'Absolute path of the file to create.'}, 'source': {
                    'type': 'string', 'description': 'Source for the new file.'}, 'overwrite': {
                        'type': 'boolean', 'description': 'Allow replacing an existing file.', 'default': False}}, 'required': [
                            'path', 'source']}
    output_schema = {'type': 'object', 'properties': {'result': {'type': 'string'}}, 'required': ['result']}
    annotations = {'readOnlyHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_create`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = ast_create(path=args['path'], source=args['source'], overwrite=args.get('overwrite', False))
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'result': result.result}, auto_approve=True)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(CreateFileTool())
    functions.register(ast_create)