"""Bridges that expose external MCP servers as MCPC tools."""

from __future__ import annotations

from .bridge import ArgTransform, McpBridge
from .client import DEFAULT_PROTOCOL_VERSION, McpClient, McpClientError
from .context7 import Context7Bridge, register_context7_tools
from .exa import ExaBridge, register_exa_tools
from .github import GitHubBridge, register_github_tools

__all__ = [
    "ArgTransform",
    "Context7Bridge",
    "DEFAULT_PROTOCOL_VERSION",
    "ExaBridge",
    "GitHubBridge",
    "McpBridge",
    "McpClient",
    "McpClientError",
    "register_context7_tools",
    "register_exa_tools",
    "register_github_tools",
]
