"""``ast_outline`` – compact structural overview of files."""


from dataclasses import asdict, dataclass, field

from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.core import OutlineNode
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.file_stats import  compute_file_stats, FileStatsResult

__all__ = [
    "OutlineError",
    "OutlineNode",
    "FileOutline",
    "OutlineFailure",
    "OutlineResult",
    "ast_outline",
    "OutlineTool",
    "register",
]

class OutlineError(Exception):
    """Raised when the outline operation cannot be performed at all."""


@dataclass(frozen=True)
class FileOutline:
    """Structural outline of one successfully parsed file.

    Attributes:
        path: The path exactly as given in the input.
        stats: File-metrics block (see the ``file-stats`` tool).
        nodes: Direct children of the module, with nested classes expanded.
    """

    path: str
    stats: FileStatsResult
    nodes: list[OutlineNode] = field(default_factory=list)


@dataclass(frozen=True)
class OutlineFailure:
    """A path that could not be outlined.

    Attributes:
        path: The path exactly as given in the input.
        error: Human-readable reason.
    """

    path: str
    error: str


@dataclass(frozen=True)
class OutlineResult:
    """Result of :func:`ast_outline`.

    Attributes:
        files: Successfully outlined files, in the given order.
        failed: Paths that could not be outlined, in the given order.
    """

    files: list[FileOutline] = field(default_factory=list)
    failed: list[OutlineFailure] = field(default_factory=list)


def _outline_one(path_str: str) -> FileOutline | OutlineFailure:
    try:
        path, tree = core.load(path_str)
    except core.AstError as exc:
        return OutlineFailure(path=path_str, error=str(exc))
    return FileOutline(
        path=path_str,
        stats=compute_file_stats(path),
        nodes=core.outline_nodes(tree),
    )


def ast_outline(paths: list[str]) -> OutlineResult:
    """Build a structural outline (module-level nodes, nested classes, stats) for each of ``paths``.

    Per-file failures (e.g. a non-existent or unparsable file) are reported in
    ``failed`` rather than raised; only a malformed call (empty ``paths``) raises.

    Args:
        paths: Absolute paths of files to outline. Must be non-empty.

    Returns:
        OutlineResult: Successfully outlined files in ``files``, everything else in
        ``failed``, both in the given order.

    Raises:
        OutlineError: If ``paths`` is empty.
    """
    if not paths:
        raise OutlineError("'paths' must be a non-empty list.")
    files: list[FileOutline] = []
    failed: list[OutlineFailure] = []
    for p in paths:
        result = _outline_one(p)
        if isinstance(result, FileOutline):
            files.append(result)
        else:
            failed.append(result)
    return OutlineResult(files=files, failed=failed)


_FILE_OUTLINE_SCHEMA = {
    "type": "object",
    "properties": {
        "path": {"type": "string"},
        "stats": {"type": "object", "description": "File-metrics block."},
        "nodes": {"type": "array", "items": {"$ref": "#/$defs/outline_node"}},
    },
    "required": ["path", "stats", "nodes"],
}

_OUTLINE_FAILURE_SCHEMA = {
    "type": "object",
    "properties": {
        "path": {"type": "string"},
        "error": {"type": "string"},
    },
    "required": ["path", "error"],
}


class OutlineTool(ToolDefinition):
    name = "ast_outline"
    title = "File Outline"
    description = (
        "Token-efficient structural overview of files: file metrics plus "
        "every module-level statement (type, qualified name, line range, one-line "
        "signature, short docstring), with nested classes recursively expanded. "
        "Accepts one or several files at once."
    )
    input_schema = {
        "type": "object",
        "properties": {
            "paths": {
                "type": "array",
                "items": {"type": "string"},
                "description": "Absolute paths of files to outline.",
            }
        },
        "required": ["paths"],
    }
    output_schema = {
        "$defs": {"outline_node": core.OUTLINE_NODE_SCHEMA},
        "type": "object",
        "properties": {
            "files": {"type": "array", "items": _FILE_OUTLINE_SCHEMA},
            "failed": {"type": "array", "items": _OUTLINE_FAILURE_SCHEMA},
        },
        "required": ["files", "failed"],
    }
    annotations = {"readOnlyHint": True, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_outline`, translating the MCP schema to/from the AST API."""
        paths = ctx.arguments["paths"]
        if not isinstance(paths, list):
            return ToolResult(content=[text_content("'paths' must be a non-empty list.")], is_error=True)
        try:
            result = ast_outline(paths)
        except OutlineError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)

        return ToolResult(
            structured_content={
                "files": [asdict(f) for f in result.files],
                "failed": [asdict(f) for f in result.failed],
            },
        )


def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(OutlineTool())
    functions.register(ast_outline)
