"""File-system and shell tools for the MCPC server
"""
from xy.ai.mcpc.tools.tool_registry import ToolRegistry
from xy.ai.mcpc.tools.tool_context import AppEnvironment
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.ast import register_ast_tools
from xy.ai.mcpc.tools.bash import register_bash_tool
from xy.ai.mcpc.tools.colgrep import register_colgrep_tool
from xy.ai.mcpc.tools.list import register_list_tool
from xy.ai.mcpc.tools.markdown import register_markdown_tool
from xy.ai.mcpc.tools.mcp import register_context7_tools, register_exa_tools, register_github_tools
from xy.ai.mcpc.tools.openalex import register_openalex_tools
from xy.ai.mcpc.tools.python import register_python_tool
from xy.ai.mcpc.tools.read import register_read_tool
from xy.ai.mcpc.tools.edit_marks import register_edit_marks_tool
from xy.ai.mcpc.tools.edit_chars import register_edit_chars_tool
from xy.ai.mcpc.tools.edit_lines import register_edit_lines_tool
from xy.ai.mcpc.tools.edit_block import register_edit_block_tool
from xy.ai.mcpc.tools.edit_line import register_edit_line_tool
from xy.ai.mcpc.tools.skills import register_skills
from xy.ai.mcpc.tools.write import register_write_tool
from xy.ai.mcpc.tools.agent import register_agent_tools
from xy.ai.mcpc.tools.grep import register_grep_tool
from xy.ai.mcpc.tools.tool_search import register as register_tool_search_tool
from xy.ai.mcpc.tools.tool_usage import register as register_tool_usage_tool
from xy.ai.mcpc.tools.tool_call import register as register_tool_call_tool
from xy.ai.mcpc.tools.ask_user import register_ask_user_tool
from xy.ai.mcpc.tools.file_stats import register_file_stats_tool
'#: Tool-set alias grouping the function-registry discovery/usage/exec tools.'
TOOLS_ALIAS = 'tools'
_TOOLS_ALIAS_MEMBERS = ('tool_search', 'tool_usage', 'tool_call')

def register_tools(registry: ToolRegistry, environment: AppEnvironment) -> None:
    """Register all built-in file-system and shell tools onto *registry*.
    """
    functions = environment.functions
    register_read_tool(registry, functions)
    register_file_stats_tool(registry, functions)
    register_list_tool(registry, functions)
    register_write_tool(registry, functions)
    register_edit_marks_tool(registry, functions)
    register_edit_chars_tool(registry, functions)
    register_edit_lines_tool(registry, functions)
    register_edit_block_tool(registry, functions)
    register_edit_line_tool(registry, functions)
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
__all__ = ['register_tools']