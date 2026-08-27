"""Per-call tool context and the process-wide environment tools run in.
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
    """Context handed to a tool handler on invocation.
    """
    session: 'Session'
    arguments: dict[str, Any]

@dataclass(slots=True)
class AppEnvironment:
    """Process-wide services available while a tool is being registered.
    """
    config: 'ServerConfig'
    registry: 'ToolRegistry'
    sessions: 'SessionStore'
    cli_manager: 'CliSessionManager'
    profiles: 'ProfileRegistry'
    control_manager: 'ToolControlManager | None' = None
