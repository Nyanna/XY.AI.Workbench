"""``exa`` tool family - Exa web search/fetch bridged as two-stage MCPC tools.

Each stage-1 tool (``web_search_exa`` / ``web_fetch_exa``) normalizes and
caches Exa's raw results by id, returning only a compact overview; the
matching stage-2 tool (``web_search_exa_results`` / ``web_fetch_exa_results``)
resolves those ids back to url and full text.
"""
from xy.ai.mcpc.tools.tool_context import AppEnvironment
from xy.ai.mcpc.tools.tool_registry import ToolRegistry
from xy.ai.mcpc.tools.mcp.exa import fetch, fetch_results, search, search_results
from xy.ai.mcpc.tools.mcp.exa.bridge import ExaBridge, init_bridge
__all__ = ['ExaBridge', 'register_exa_tools', 'ALIAS']
'#: Alias name that activates the whole family in one go.'
ALIAS = 'exa'
_ALIAS_MEMBERS = ('web_search_exa', 'web_search_exa_results', 'web_fetch_exa', 'web_fetch_exa_results')

def register_exa_tools(registry: ToolRegistry, environment: AppEnvironment) -> None:
    """Register every Exa tool and the ``exa`` alias."""
    init_bridge(environment.config)
    search.register(registry, environment.functions)
    search_results.register(registry, environment.functions)
    fetch.register(registry, environment.functions)
    fetch_results.register(registry, environment.functions)
    registry.register_alias(ALIAS, _ALIAS_MEMBERS)