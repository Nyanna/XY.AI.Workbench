"""Bridges that expose external MCP servers as MCPC tools."""
from xy.ai.mcpc.tools.mcp.bridge import McpBridge, McpBridgeError, compact
from xy.ai.mcpc.tools.mcp.client import DEFAULT_PROTOCOL_VERSION, McpClient, McpClientError
from xy.ai.mcpc.tools.mcp.context7 import Context7Bridge, register_context7_tools
from xy.ai.mcpc.tools.mcp.exa import ExaBridge, register_exa_tools
from xy.ai.mcpc.tools.mcp.github import GitHubBridge, register_github_tools
__all__ = [
    'Context7Bridge',
    'DEFAULT_PROTOCOL_VERSION',
    'ExaBridge',
    'GitHubBridge',
    'McpBridge',
    'McpBridgeError',
    'McpClient',
    'McpClientError',
    'compact',
    'register_context7_tools',
    'register_exa_tools',
    'register_github_tools']