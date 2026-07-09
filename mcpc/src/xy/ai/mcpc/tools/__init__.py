"""File-system and shell tools for the MCPC server.

Available tools
---------------
* ``read``          – read a file (with session-level content-hash caching)
* ``write``         – overwrite or append to a file
* ``insert``        – insert text at a character offset
* ``replace-chars`` – replace a character range with new text
* ``replace-lines`` – replace a line range with new text
* ``change``        – replace a delimited block identified by start/end markers
* ``bash``          – run a Bash script in a given working directory
* ``python``        – run a Python script directly from context
* ``markdown``      – AST-based Markdown editing via a remark (Node.js) script
* ``ask-user``      – ask the user a clarifying question (back-channel)

Skills (on-demand hint tools) are registered from the ``skills`` sub-package.
Bridges to external MCP servers (e.g. Exa) live in the ``mcp`` sub-package.
OpenAlex scholarly-search tools live in the ``openalex`` sub-package.

Call :func:`register_tools` to register all tools onto a
:class:`~xy.ai.mcpc.registry.ToolRegistry` instance.
"""

from __future__ import annotations

import importlib

from ..registry import ToolRegistry
from .bash import register_bash_tool
from .change import register_change_tool
from .insert import register_insert_tool
from .markdown import register_markdown_tool
from .mcp import register_context7_tools, register_exa_tools, register_github_tools
from .openalex import register_openalex_tools
from .python import register_python_tool
from .read import register_read_tool
from xy.ai.mcpc.tools.replace_chars import register_replace_chars_tool
from .replace_lines import register_replace_lines_tool
from .skills import register_skills
from .write import register_write_tool

# ``ask-user`` uses a hyphenated directory name, which is not a valid Python
# identifier, so it cannot be imported with a regular ``from .ask-user import``
# statement. Use ``importlib`` instead.
register_ask_user_tool = importlib.import_module(
    "xy.ai.mcpc.tools.ask-user"
).register_ask_user_tool


def register_tools(registry: ToolRegistry) -> None:
    """Register all built-in file-system and shell tools onto *registry*."""
    register_read_tool(registry)
    register_write_tool(registry)
    register_insert_tool(registry)
    register_change_tool(registry)
    register_replace_chars_tool(registry)
    register_replace_lines_tool(registry)
    register_bash_tool(registry)
    register_python_tool(registry)
    register_markdown_tool(registry)
    register_ask_user_tool(registry)
    register_skills(registry)
    register_exa_tools(registry)
    register_github_tools(registry)
    register_context7_tools(registry)
    register_openalex_tools(registry)


# Keep the old name available so existing call sites don't break.
register_builtin_tools = register_tools

__all__ = [
    "register_tools",
    "register_builtin_tools",
]
