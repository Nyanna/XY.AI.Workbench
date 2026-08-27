"""Process-wide services exposed to tool handlers.
"""
from __future__ import annotations
from dataclasses import dataclass
from typing import TYPE_CHECKING, Any
if TYPE_CHECKING:
    from xy.ai.mcpc.cli import CliSessionManager
    from xy.ai.mcpc.config import ServerConfig
    from xy.ai.mcpc.control import ToolControlManager
    from xy.ai.mcpc.tools.registry import ToolRegistry
    from xy.ai.mcpc.server.session import SessionStore, Session
    from xy.ai.mcpc.tools.agent.profiles import ProfileRegistry

@dataclass(slots=True)
class ToolContext:
    """Context handed to a tool handler on invocation."""
    session: 'Session'
    arguments: dict[str, Any]
    services: 'AppServices | None' = None

@dataclass(slots=True)
class AppServices:
    """Container for the shared components a tool handler may need."""
    config: 'ServerConfig'
    registry: 'ToolRegistry'
    sessions: 'SessionStore'
    cli_manager: 'CliSessionManager'
    profiles: 'ProfileRegistry'
    control_manager: 'ToolControlManager | None' = None