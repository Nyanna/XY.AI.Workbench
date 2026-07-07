"""File-system and shell tools for the MCPC server.

Available tools
---------------
* ``read``          – read a file (with session-level content-hash caching)
* ``write``         – overwrite or append to a file
* ``insert``        – insert text at a character offset
* ``replace-chars`` – replace a character range with new text
* ``replace-lines`` – replace a line range with new text
* ``bash``          – run a Bash script in a given working directory
* ``python``        – run a Python script directly from context
* ``markdown``      – AST-based Markdown editing via a remark (Node.js) script

Skills (on-demand hint tools) are registered from the ``skills`` sub-package.
Bridges to external MCP servers (e.g. Exa) live in the ``mcp`` sub-package.

Call :func:`register_tools` to register all tools onto a
:class:`~xy.ai.mcpc.registry.ToolRegistry` instance.
"""

from __future__ import annotations

from ..registry import ToolRegistry
from .bash import register_bash_tool
from .insert import register_insert_tool
from .markdown import register_markdown_tool
from .mcp import register_exa_tools
from .python import register_python_tool
from .read import register_read_tool
from xy.ai.mcpc.tools.replace_chars import register_replace_chars_tool
from .replace_lines import register_replace_lines_tool
from .skills import register_skills
from .write import register_write_tool


def register_tools(registry: ToolRegistry) -> None:
    """Register all built-in file-system and shell tools onto *registry*."""
    register_read_tool(registry)
    register_write_tool(registry)
    register_insert_tool(registry)
    register_replace_chars_tool(registry)
    register_replace_lines_tool(registry)
    register_bash_tool(registry)
    register_python_tool(registry)
    register_markdown_tool(registry)
    register_skills(registry)
    register_exa_tools(registry)


# Keep the old name available so existing call sites don't break.
register_builtin_tools = register_tools

__all__ = [
    "register_tools",
    "register_builtin_tools",
]
