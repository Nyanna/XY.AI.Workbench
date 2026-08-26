"""Replace-chars tool – replaces a character range inside an existing file."""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content

__all__ = [
    "ReplaceCharsError",
    "ReplaceCharsResult",
    "replace_chars",
    "ReplaceCharsTool",
    "register_replace_chars_tool",
]


class ReplaceCharsError(Exception):
    """Raised when a replace-chars operation cannot be performed."""


@dataclass(frozen=True)
class ReplaceCharsResult:
    result: str


def replace_chars(path: str, offset: int, length: int, content: str) -> ReplaceCharsResult:
    """Replace ``length`` characters starting at ``offset`` in the file at ``path`` with ``content``."""
    file_path = Path(path)
    if not file_path.is_absolute():
        raise ReplaceCharsError("Path must be absolute.")
    if not file_path.exists():
        raise ReplaceCharsError("File not found.")
    if not file_path.is_file():
        raise ReplaceCharsError("Not a regular file.")

    try:
        text = file_path.read_text(encoding="utf-8")
        file_len = len(text)
        if offset > file_len:
            raise ReplaceCharsError(
                f"Offset {offset} is beyond end of file (file length: {file_len} characters)."
            )
        end = min(offset + length, file_len)
        result_text = text[:offset] + content + text[end:]
        file_path.write_text(result_text, encoding="utf-8")
    except OSError as exc:
        raise ReplaceCharsError(f"Replace failed: {exc}") from exc

    return ReplaceCharsResult(result="success")


class ReplaceCharsTool(ToolDefinition):
    name = "replace-chars"
    title = "Replace characters in file"
    description = (
        "Replace a range of characters inside an existing file with new content. "
        "The range is defined by a zero-based character ``offset`` and a ``length`` "
        "(number of characters to remove starting at the offset). "
        "The supplied ``content`` is written in place of the removed range. "
        "To replace whole lines instead, use ``replace-lines``."
    )
    input_schema = {
        "type": "object",
        "properties": {
            "path": {
                "type": "string",
                "description": "Absolute path to the file to modify.",
            },
            "offset": {
                "type": "integer",
                "description": "Zero-based character offset of the first character to replace.",
                "minimum": 0,
            },
            "length": {
                "type": "integer",
                "description": "Number of characters to remove starting at ``offset``.",
                "minimum": 0,
            },
            "content": {
                "type": "string",
                "description": "Replacement text (may be empty to perform a pure deletion).",
            },
        },
        "required": ["path", "offset", "length", "content"],
    }
    output_schema = {
        "type": "object",
        "properties": {
            "result": {
                "type": "string",
                "description": "``success`` on success.",
            },
        },
        "required": ["result"],
    }
    annotations = {"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`replace_chars`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = replace_chars(
                path=args["path"],
                offset=args["offset"],
                length=args["length"],
                content=args["content"],
            )
        except ReplaceCharsError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)

        return ToolResult(structured_content={"result": result.result}, auto_approve=True)


def register_replace_chars_tool(registry: ToolRegistry) -> None:
    registry.register(ReplaceCharsTool())
