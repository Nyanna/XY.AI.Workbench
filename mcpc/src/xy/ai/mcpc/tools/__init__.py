"""File-system and shell tools for the MCPC server.

Available tools
---------------
* ``read``    – read a file (with session-level content-hash caching)
* ``write``   – overwrite or append to a file
* ``insert``  – insert text at a character offset
* ``replace`` – replace a character range with new text
* ``bash``    – run a Bash script in a given working directory

Call :func:`register_tools` to register all tools onto a
:class:`~xy.ai.mcpc.registry.ToolRegistry` instance.
"""

from __future__ import annotations

from ..registry import ToolRegistry
from .bash import register_bash_tool
from .insert import register_insert_tool
from .read import register_read_tool
from .replace import register_replace_tool
from .write import register_write_tool


def register_tools(registry: ToolRegistry) -> None:
    """Register all built-in file-system and shell tools onto *registry*."""
    register_read_tool(registry)
    register_write_tool(registry)
    register_insert_tool(registry)
    register_replace_tool(registry)
    register_bash_tool(registry)


# Keep the old name available so existing call sites don't break.
register_builtin_tools = register_tools

__all__ = [
    "register_tools",
    "register_builtin_tools",
]
