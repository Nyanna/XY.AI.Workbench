"""Shared bridge to the Exa remote MCP server, used by all ``exa`` tools."""
from xy.ai.mcpc.config import ServerConfig
from xy.ai.mcpc.tools.mcp.bridge import McpBridge, McpBridgeError
from xy.ai.mcpc.tools.mcp.client import McpClient, McpClientError
__all__ = ['ExaBridge', 'init_bridge', 'get_bridge']

class ExaBridge(McpBridge):
    """Bridge to the Exa remote MCP server."""

    def build_client(self, config: ServerConfig) -> McpClient:
        api_key = config.exa_api_key
        if not api_key:
            raise McpClientError('Exa API key is not configured (set MCPC_EXA_API_KEY / EXA_API_KEY).')
        return McpClient(config.exa_mcp_url, headers={'x-api-key': api_key})
'#: Module-level bridge, built by :func:`~xy.ai.mcpc.tools.mcp.exa.register_exa_tools`.'
_bridge: ExaBridge | None = None

def init_bridge(config: ServerConfig) -> None:
    global _bridge
    _bridge = ExaBridge(config)

def get_bridge() -> ExaBridge:
    """Return the module-level Exa bridge configured by ``register_exa_tools``."""
    if _bridge is None:
        raise McpBridgeError('Exa tools used before register_exa_tools() was called.')
    return _bridge