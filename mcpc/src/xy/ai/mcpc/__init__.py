"""MCP Controller — a stateful MCP server (Streamable HTTP).

Public API::

    from xy.ai.mcpc import ServerConfig, ToolRegistry, build_server, run
"""

from __future__ import annotations

from .config import ServerConfig
from .registry import Tool, ToolContext, ToolRegistry, ToolResult, text_content
from .server import McpHTTPServer, build_server, run
from .session import Session, SessionStore

__version__ = "0.1.0"

__all__ = [
    "__version__",
    "ServerConfig",
    "Tool",
    "ToolContext",
    "ToolRegistry",
    "ToolResult",
    "text_content",
    "Session",
    "SessionStore",
    "McpHTTPServer",
    "build_server",
    "run",
]
