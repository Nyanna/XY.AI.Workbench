"""File-system and shell tools for the MCPC server.

Available tools
---------------
* ``read_file``          – read a file (with session-level content-hash caching, line/char slicing)
* ``file_stats``    – get file metrics for access and processing planning
* ``list``          – recursively list files below a directory
* ``write``         – overwrite or append to a file
* ``insert``        – insert text at a character offset
* ``replace_chars`` – replace a character range with new text
* ``replace_lines`` – replace a line range with new text
* ``replace_block`` – replace an exact block of text (old text -> new text)
* ``change``        – replace a delimited block identified by start/end markers
* ``bash``          – run a Bash script in a given working directory
* ``python``        – run a Python script directly from context
* ``markdown``      – AST-based Markdown editing via a remark (Node.js) script
* ``python_ast_*``  – ``ast``-based Python editing (outline, node CRUD, imports/
  classes/functions, node-scoped replace-block, script, validate); jointly
  enabled via the ``python-ast`` tool-set alias
* ``ask_user``      – ask the user a clarifying question (back-channel)
* ``colgrep``       – search a pre-built colgrep index (search-only; never initializes an index)
* ``tool_search``, ``tool_usage``, ``tool_call`` – discover, introspect and run
  plain Python functions/methods registered in the ``FunctionRegistry``
  instance held by :class:`~xy.ai.mcpc.tools.tool_context.AppEnvironment`
  (see :mod:`xy.ai.mcpc.tools.function_registry`); jointly enabled via the
  ``tools`` tool-set alias

Skills (on-demand hint tools) are registered from the ``skills`` sub-package.
Bridges to external MCP servers (e.g. Exa) live in the ``mcp`` sub-package.
OpenAlex scholarly-search tools live in the ``openalex`` sub-package.

Call :func:`register_tools` to register all tools onto a
:class:`~xy.ai.mcpc.registry.ToolRegistry` instance.
"""


import importlib

from xy.ai.mcpc.tools.registry import ToolRegistry
from xy.ai.mcpc.tools.tool_context import AppEnvironment
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.ast import register_ast_tools
from xy.ai.mcpc.tools.bash import register_bash_tool
from xy.ai.mcpc.tools.replace import register_replace_tool
from xy.ai.mcpc.tools.colgrep import register_colgrep_tool
from xy.ai.mcpc.tools.insert import register_insert_tool
from xy.ai.mcpc.tools.list import register_list_tool
from xy.ai.mcpc.tools.markdown import register_markdown_tool
from xy.ai.mcpc.tools.mcp import register_context7_tools, register_exa_tools, register_github_tools
from xy.ai.mcpc.tools.openalex import register_openalex_tools
from xy.ai.mcpc.tools.python import register_python_tool
from xy.ai.mcpc.tools.read import register_read_tool
from xy.ai.mcpc.tools.replace_chars import register_replace_chars_tool
from xy.ai.mcpc.tools.replace_lines import register_replace_lines_tool
from xy.ai.mcpc.tools.replace_block import register_replace_block_tool
from xy.ai.mcpc.tools.skills import register_skills
from xy.ai.mcpc.tools.write import register_write_tool
from xy.ai.mcpc.tools.agent import register_agent_tools
from xy.ai.mcpc.tools.grep import register_grep_tool
from xy.ai.mcpc.tools.tool_search import register as register_tool_search_tool
from xy.ai.mcpc.tools.tool_usage import register as register_tool_usage_tool
from xy.ai.mcpc.tools.tool_call import register as register_tool_call_tool
from xy.ai.mcpc.tools.ask_user import register_ask_user_tool
from xy.ai.mcpc.tools.file_stats import register_file_stats_tool

#: Tool-set alias grouping the function-registry discovery/usage/exec tools.
TOOLS_ALIAS = "tools"
_TOOLS_ALIAS_MEMBERS = ("tool_search", "tool_usage", "tool_call")

def register_tools(registry: ToolRegistry, environment: AppEnvironment) -> None:
    """Register all built-in file-system and shell tools onto *registry*.
    """
    functions = environment.functions
    register_read_tool(registry, functions)
    register_file_stats_tool(registry, functions)
    register_list_tool(registry, functions)
    register_write_tool(registry, functions)
    register_insert_tool(registry, functions)
    register_replace_tool(registry, functions)
    register_replace_chars_tool(registry, functions)
    register_replace_lines_tool(registry, functions)
    register_replace_block_tool(registry, functions)
    register_bash_tool(registry, functions)
    register_python_tool(registry, functions)
    register_markdown_tool(registry, environment)
    register_ast_tools(registry, functions)
    register_ask_user_tool(registry)
    register_colgrep_tool(registry, functions)
    register_skills(registry, environment)
    register_exa_tools(registry, environment)
    register_github_tools(registry, environment)
    register_context7_tools(registry, environment)
    register_openalex_tools(registry, environment)
    register_agent_tools(registry, environment)
    register_grep_tool(registry, functions)
    register_tool_search_tool(registry, functions)
    register_tool_usage_tool(registry, functions)
    register_tool_call_tool(registry, functions)
    registry.register_alias(TOOLS_ALIAS, _TOOLS_ALIAS_MEMBERS)


# Keep the old name available so existing call sites don't break.
register_builtin_tools = register_tools

__all__ = [
    "register_tools",
    "register_builtin_tools",
]
